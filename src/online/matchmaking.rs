//! Bounded discovery candidates are untrusted until a live application handshake.
use super::*;
use std::{
    collections::BTreeSet,
    io::{Read, Write},
    sync::{Mutex, mpsc},
};
struct ResultMessage {
    code: String,
    rtt: Duration,
}
pub(super) struct Search {
    socket: UdpSocket,
    until: Instant,
    seen: BTreeSet<SocketAddr>,
    last_broadcast: Instant,
    sender: mpsc::SyncSender<ResultMessage>,
    receiver: Mutex<mpsc::Receiver<ResultMessage>>,
    matches: Vec<(String, Duration)>,
}
impl Search {
    pub fn new() -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;
        socket.set_nonblocking(true)?;
        let _ = socket.send_to(b"ROAD-DASH-DISCOVER-1", ("255.255.255.255", 47655));
        let _ = socket.send_to(b"ROAD-DASH-DISCOVER-1", ("127.0.0.1", 47655));
        let (sender, receiver) = mpsc::sync_channel(32);
        let mut search = Self {
            socket,
            until: Instant::now() + Duration::from_secs(15),
            seen: BTreeSet::new(),
            sender,
            receiver: Mutex::new(receiver),
            matches: Vec::new(),
            last_broadcast: Instant::now(),
        };
        if let Ok(peers) = std::env::var("ROADDASH_PEERS") {
            for peer in peers.split(',').take(256) {
                search.candidate(peer.trim().into());
            }
        }
        Ok(search)
    }
    pub fn candidate(&mut self, code: String) {
        let Ok(target) = address(&code) else { return };
        if target.port() == 0 || target.ip().is_unspecified() || target.ip().is_multicast() {
            return;
        }
        if self.seen.len() >= 32 || !self.seen.insert(target) {
            return;
        }
        let code = target.to_string();
        let sender = self.sender.clone();
        std::thread::spawn(move || {
            if let Ok((code, rtt)) = probe(&code) {
                let _ = sender.try_send(ResultMessage { code, rtt });
            }
        });
    }
    pub fn poll(&mut self) -> Option<Vec<(String, Duration)>> {
        if self.last_broadcast.elapsed() >= Duration::from_secs(1) {
            self.last_broadcast = Instant::now();
            let _ = self
                .socket
                .send_to(b"ROAD-DASH-DISCOVER-1", ("255.255.255.255", 47655));
            let _ = self
                .socket
                .send_to(b"ROAD-DASH-DISCOVER-1", ("127.0.0.1", 47655));
        }
        let mut data = [0u8; 128];
        for _ in 0..32 {
            match self.socket.recv_from(&mut data) {
                Ok((n, _)) => {
                    if let Ok(code) = std::str::from_utf8(&data[..n]) {
                        self.candidate(code.into());
                    }
                }
                Err(_) => break,
            }
        }
        if let Ok(receiver) = self.receiver.lock() {
            for value in receiver.try_iter() {
                self.matches.push((value.code, value.rtt));
            }
        }
        (Instant::now() >= self.until).then(|| {
            self.matches.sort_by_key(|(_, rtt)| *rtt);
            std::mem::take(&mut self.matches)
        })
    }
}
pub(super) fn probe(code: &str) -> Result<(String, Duration), String> {
    let target = address(code)?;
    let mut stream = TcpStream::connect_timeout(&target, Duration::from_millis(250))
        .map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_millis(250)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_millis(250)))
        .map_err(|e| e.to_string())?;
    stream.set_nodelay(true).map_err(|e| e.to_string())?;
    let mut random = [0u8; 8];
    getrandom::fill(&mut random).map_err(|e| e.to_string())?;
    let nonce = u64::from_ne_bytes(random);
    let bytes = serde_json::to_vec(&ClientMessage::Probe {
        protocol: PROTOCOL,
        nonce,
    })
    .unwrap();
    let start = Instant::now();
    stream
        .write_all(&(bytes.len() as u32).to_be_bytes())
        .and_then(|_| stream.write_all(&bytes))
        .map_err(|e| e.to_string())?;
    let mut prefix = [0u8; 4];
    stream.read_exact(&mut prefix).map_err(|e| e.to_string())?;
    let n = u32::from_be_bytes(prefix) as usize;
    if n > 4096 {
        return Err("invalid probe reply".into());
    }
    let mut data = vec![0u8; n];
    stream.read_exact(&mut data).map_err(|e| e.to_string())?;
    let rtt = start.elapsed();
    match serde_json::from_slice(&data).map_err(|e| e.to_string())? {
        ServerMessage::Offer {
            protocol,
            nonce: echo,
            available: true,
        } if protocol == PROTOCOL && echo == nonce && rr_core::online::eligible_rtt(rtt) => {
            Ok((code.to_string(), rtt))
        }
        _ => Err("host unavailable or RTT exceeds 250 ms".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn peer(
        available: bool,
        wrong_nonce: bool,
        wrong_protocol: bool,
        delay_ms: u64,
    ) -> (String, std::thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        let task = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut prefix = [0u8; 4];
            stream.read_exact(&mut prefix).unwrap();
            let mut data = vec![0; u32::from_be_bytes(prefix) as usize];
            stream.read_exact(&mut data).unwrap();
            let ClientMessage::Probe { nonce, .. } = serde_json::from_slice(&data).unwrap() else {
                panic!("expected probe");
            };
            std::thread::sleep(Duration::from_millis(delay_ms));
            let bytes = serde_json::to_vec(&ServerMessage::Offer {
                protocol: if wrong_protocol {
                    PROTOCOL + 1
                } else {
                    PROTOCOL
                },
                nonce: if wrong_nonce { nonce ^ 1 } else { nonce },
                available,
            })
            .unwrap();
            let _ = stream
                .write_all(&(bytes.len() as u32).to_be_bytes())
                .and_then(|_| stream.write_all(&bytes));
        });
        (address, task)
    }
    #[test]
    fn real_handshake_rejects_wrong_identity_full_and_slow_hosts() {
        for (available, nonce, protocol, delay, expected) in [
            (true, false, false, 0, true),
            (false, false, false, 0, false),
            (true, true, false, 0, false),
            (true, false, true, 0, false),
            (true, false, false, 270, false),
        ] {
            let (address, task) = peer(available, nonce, protocol, delay);
            assert_eq!(probe(&address).is_ok(), expected);
            task.join().unwrap();
        }
    }
    #[test]
    fn invalid_candidates_do_not_spend_probe_budget() {
        let mut search = Search::new().unwrap();
        search.seen.clear();
        for i in 0..64 {
            search.candidate(format!("invalid-{i}"));
        }
        search.candidate("0.0.0.0:1234".into());
        search.candidate("127.0.0.1:0".into());
        assert!(search.seen.is_empty());
        search.candidate("127.0.0.1:9".into());
        let code = rr_core::online::encode_endpoint([127, 0, 0, 1], 9);
        search.candidate(code);
        assert_eq!(search.seen.len(), 1);
    }
    #[test]
    fn unreachable_is_not_a_match() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        drop(listener);
        assert!(probe(&address).is_err());
    }
}
