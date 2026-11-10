//! Bounded nonblocking length-prefixed transport; TCP supplies delivery, not trust.
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::VecDeque,
    io::{self, Read, Write},
    net::TcpStream,
    sync::Arc,
};
pub const FRAME_LIMIT: usize = 512 * 1024;
const COMPRESSED: u32 = 1 << 31;
/// Serialize/compress once per broadcast. All slow-peer queues share these bytes.
pub fn encode<T: Serialize>(value: &T, compress: bool) -> io::Result<Arc<[u8]>> {
    let data = serde_json::to_vec(value)?;
    if data.len() > FRAME_LIMIT {
        return Err(io::Error::other(
            "snapshot exceeds uncompressed frame budget",
        ));
    }
    let mut compressed = Vec::new();
    if compress && data.len() > 1024 {
        let mut encoder =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(&data)?;
        compressed = encoder.finish()?;
    }
    let use_compressed = !compressed.is_empty() && compressed.len() < data.len();
    let payload = if use_compressed { &compressed } else { &data };
    let prefix = payload.len() as u32 | if use_compressed { COMPRESSED } else { 0 };
    let mut frame = prefix.to_be_bytes().to_vec();
    frame.extend_from_slice(payload);
    Ok(Arc::from(frame))
}
fn decode<T: DeserializeOwned>(payload: &[u8], compressed: bool, limit: usize) -> io::Result<T> {
    if !compressed {
        return Ok(serde_json::from_slice(payload)?);
    }
    let mut reader = flate2::read::DeflateDecoder::new(payload).take(limit as u64 + 1);
    let mut decoded = Vec::new();
    reader.read_to_end(&mut decoded)?;
    if decoded.len() > limit {
        return Err(io::Error::other("decompressed frame exceeds budget"));
    }
    Ok(serde_json::from_slice(&decoded)?)
}

pub struct Peer {
    stream: TcpStream,
    incoming: Vec<u8>,
    outgoing: VecDeque<Arc<[u8]>>,
    offset: usize,
    closed: bool,
}
impl Peer {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nonblocking(true)?;
        stream.set_nodelay(true)?;
        Ok(Self {
            stream,
            incoming: Vec::new(),
            outgoing: VecDeque::new(),
            offset: 0,
            closed: false,
        })
    }
    pub fn send<T: Serialize>(&mut self, value: &T) -> io::Result<()> {
        self.send_frame(encode(value, false)?)
    }
    pub fn send_frame(&mut self, frame: Arc<[u8]>) -> io::Result<()> {
        if self.outgoing.len() >= 8 {
            return Err(io::Error::other("peer output backlog"));
        }
        self.outgoing.push_back(frame);
        self.flush()
    }
    pub fn flush(&mut self) -> io::Result<()> {
        while let Some(frame) = self.outgoing.front() {
            match self.stream.write(&frame[self.offset..]) {
                Ok(0) => return Err(io::Error::other("peer closed")),
                Ok(n) => {
                    self.offset += n;
                    if self.offset == frame.len() {
                        self.outgoing.pop_front();
                        self.offset = 0;
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    pub fn receive<T: DeserializeOwned>(&mut self, limit: usize) -> io::Result<Vec<T>> {
        if self.closed && self.incoming.is_empty() {
            return Err(io::Error::other("peer disconnected"));
        }
        self.flush()?;
        let mut data = [0u8; 8192];
        for _ in 0..64 {
            match self.stream.read(&mut data) {
                Ok(0) => {
                    self.closed = true;
                    break;
                }
                Ok(n) => {
                    self.incoming.extend_from_slice(&data[..n]);
                    if self.incoming.len() > FRAME_LIMIT * 2 {
                        return Err(io::Error::other("input backlog"));
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
        let mut messages = Vec::new();
        while self.incoming.len() >= 4 {
            let prefix = u32::from_be_bytes(self.incoming[..4].try_into().unwrap());
            let compressed = prefix & COMPRESSED != 0;
            let len = (prefix & !COMPRESSED) as usize;
            if len == 0 || len > limit {
                return Err(io::Error::other("invalid frame length"));
            }
            if self.incoming.len() < len + 4 {
                break;
            }
            if messages.len() >= 128 {
                return Err(io::Error::other("message rate exceeded"));
            }
            messages.push(decode(&self.incoming[4..4 + len], compressed, limit)?);
            self.incoming.drain(..4 + len);
        }
        if self.closed && messages.is_empty() {
            return Err(io::Error::other("peer closed with an incomplete frame"));
        }
        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_reload_message_is_delivered_before_eof() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let mut client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (stream, _) = listener.accept().unwrap();
        let mut peer = Peer::new(stream).unwrap();
        client
            .write_all(&encode(&"reload", false).unwrap())
            .unwrap();
        client.shutdown(std::net::Shutdown::Write).unwrap();
        let until = std::time::Instant::now() + std::time::Duration::from_secs(1);
        loop {
            let messages = peer.receive::<String>(4096).unwrap();
            if !messages.is_empty() {
                assert_eq!(messages, ["reload"]);
                break;
            }
            assert!(std::time::Instant::now() < until);
        }
        assert!(peer.receive::<String>(4096).is_err());
    }
    #[test]
    fn oversized_and_malformed_frames_disconnect() {
        for bytes in [
            5000u32.to_be_bytes().to_vec(),
            [3u32.to_be_bytes().as_slice(), b"???"].concat(),
        ] {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let mut client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
            let (stream, _) = listener.accept().unwrap();
            let mut peer = Peer::new(stream).unwrap();
            client.write_all(&bytes).unwrap();
            let until = std::time::Instant::now() + std::time::Duration::from_secs(1);
            loop {
                match peer.receive::<serde_json::Value>(4096) {
                    Err(_) => break,
                    Ok(_) => assert!(std::time::Instant::now() < until),
                }
            }
        }
    }
    #[test]
    fn fragment_roundtrip_preserves_order() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let mut client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (stream, _) = listener.accept().unwrap();
        let mut peer = Peer::new(stream).unwrap();
        client.write_all(&[0, 0]).unwrap();
        assert!(peer.receive::<u32>(4096).unwrap().is_empty());
        client.write_all(&[0, 1, b'7', 0, 0, 0, 1, b'8']).unwrap();
        let until = std::time::Instant::now() + std::time::Duration::from_secs(1);
        let mut values = vec![];
        while values.len() < 2 {
            values.extend(peer.receive::<u32>(4096).unwrap());
            assert!(std::time::Instant::now() < until);
        }
        assert_eq!(values, vec![7, 8]);
    }
}

#[cfg(test)]
mod compression_tests {
    use super::*;
    #[test]
    fn compressed_roundtrip_and_bomb_limit() {
        let original = vec!["repeated snapshot fields"; 1000];
        let encoded = encode(&original, true).unwrap();
        let prefix = u32::from_be_bytes(encoded[..4].try_into().unwrap());
        assert_ne!(prefix & COMPRESSED, 0);
        assert!(encoded.len() < 1024);
        let decoded: Vec<String> = decode(&encoded[4..], true, FRAME_LIMIT).unwrap();
        assert_eq!(decoded, original);
        assert!(decode::<Vec<String>>(&encoded[4..], true, 4096).is_err());
        assert!(decode::<serde_json::Value>(b"invalid deflate", true, 4096).is_err());
    }
}
