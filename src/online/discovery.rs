//! Distributed public endpoint discovery. Candidates are not trusted lobbies.
//! A game-protocol probe must still verify availability and measure RTT.
use bevy::prelude::Resource;
use futures_lite::StreamExt;
use std::{
    collections::BTreeSet,
    future::Future,
    net::{Ipv4Addr, SocketAddr},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, SyncSender},
    },
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

// SHA1("road-dash/public-listen-host/protocol-2"); this is not a content hash.
const NAMESPACE: [u8; 20] = [
    49, 32, 70, 101, 12, 250, 202, 120, 19, 135, 10, 101, 102, 89, 180, 169, 157, 224, 36, 121,
];
const LIMIT: usize = 64;
const LOOKUP_TIME: Duration = Duration::from_secs(12);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DiscoveryStatus {
    #[default]
    Idle,
    Searching,
    Ready,
    Unavailable(String),
}
enum Event {
    ObservedIp(Ipv4Addr),
    Candidate(SocketAddr),
    Status(DiscoveryStatus),
}

#[derive(Resource, Default)]
pub struct Discovery {
    cancel: Option<Arc<AtomicBool>>,
    events: Option<Mutex<Receiver<Event>>>,
    pub status: DiscoveryStatus,
    pub observed_ip: Option<Ipv4Addr>,
}

impl Discovery {
    pub fn begin_search(&mut self) {
        self.start(None);
    }
    /// Announce only a public, joinable host. The DHT observes our WAN address.
    /// This does not establish that its TCP port is reachable through NAT.
    pub fn announce(&mut self, port: u16) {
        if port != 0 {
            self.start(Some(port));
        }
    }
    pub fn stop(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel.store(true, Ordering::Relaxed);
        }
        self.events = None;
        self.status = DiscoveryStatus::Idle;
        self.observed_ip = None;
    }
    pub fn poll_candidates(&mut self) -> Vec<SocketAddr> {
        let mut candidates = Vec::new();
        if let Some(events) = &self.events
            && let Ok(rx) = events.lock()
        {
            for event in rx.try_iter().take(LIMIT + 4) {
                match event {
                    Event::Candidate(address) => candidates.push(address),
                    Event::Status(status) => self.status = status,
                    Event::ObservedIp(ip) => self.observed_ip = Some(ip),
                }
            }
        }
        candidates
    }
    fn start(&mut self, port: Option<u16>) {
        self.stop();
        if std::env::var("ROADDASH_DHT").as_deref() == Ok("0") {
            self.status = DiscoveryStatus::Unavailable(
                "Public discovery disabled; using LAN/known peers only".into(),
            );
            return;
        }
        let cancel = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::sync_channel(LIMIT + 4);
        self.events = Some(Mutex::new(rx));
        self.cancel = Some(cancel.clone());
        self.status = DiscoveryStatus::Searching;
        if let Err(error) = thread::Builder::new()
            .name("public-lobby-discovery".into())
            .spawn(move || worker(port, cancel, tx))
        {
            self.status = DiscoveryStatus::Unavailable(error.to_string());
        }
    }
}
impl Drop for Discovery {
    fn drop(&mut self) {
        self.stop();
    }
}

// Poll on this dedicated worker only. Neither bootstrap DNS, UDP operations nor
// waiting for a DHT response runs on Bevy's render/fixed threads. Cancellation
// also drops pending futures and their node handles instead of leaking searches.
fn bounded<F: Future>(future: F, deadline: Instant, cancel: &AtomicBool) -> Option<F::Output> {
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    while !cancel.load(Ordering::Relaxed) && Instant::now() < deadline {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return Some(value);
        }
        thread::sleep(Duration::from_millis(10));
    }
    None
}

fn worker(port: Option<u16>, cancel: Arc<AtomicBool>, tx: SyncSender<Event>) {
    loop {
        if cancel.load(Ordering::Relaxed) {
            return;
        }
        let deadline = Instant::now() + LOOKUP_TIME;
        let node = match mainline::Dht::builder()
            .request_timeout(Duration::from_secs(2))
            .build()
        {
            Ok(node) => node,
            Err(error) => {
                let _ = tx.try_send(Event::Status(DiscoveryStatus::Unavailable(
                    error.to_string(),
                )));
                return;
            }
        };
        let dht = node.clone().as_async();
        let ready = bounded(dht.bootstrapped(), deadline, &cancel) == Some(true);
        if ready {
            if let Some(info) = bounded(dht.info(), deadline, &cancel)
                && let Some(address) = info.public_address()
                && public_address(*address.ip())
            {
                let _ = tx.try_send(Event::ObservedIp(*address.ip()));
            }
            if let Some(port) = port {
                let status = match bounded(
                    dht.announce_peer(NAMESPACE.into(), Some(port)),
                    deadline,
                    &cancel,
                ) {
                    Some(Ok(_)) => DiscoveryStatus::Ready,
                    _ => DiscoveryStatus::Unavailable(
                        "Public host announcement did not complete".into(),
                    ),
                };
                let _ = tx.try_send(Event::Status(status));
            } else {
                let _ = tx.try_send(Event::Status(DiscoveryStatus::Ready));
                let mut stream = dht.get_peers(NAMESPACE.into());
                let mut seen = BTreeSet::new();
                while seen.len() < LIMIT {
                    let Some(Some(peers)) = bounded(stream.next(), deadline, &cancel) else {
                        break;
                    };
                    for peer in peers {
                        let address = SocketAddr::V4(peer);
                        if public_address(*peer.ip()) && peer.port() != 0 && seen.insert(address) {
                            let _ = tx.try_send(Event::Candidate(address));
                            if seen.len() == LIMIT {
                                break;
                            }
                        }
                    }
                }
            }
        } else {
            let _ = tx.try_send(Event::Status(DiscoveryStatus::Unavailable(
                "Internet discovery bootstrap timed out; direct codes remain available".into(),
            )));
        }
        // Drop short-lived nodes before the dependency's 15-minute adaptive
        // routing-server threshold. No general-purpose DHT server is enabled.
        drop(dht);
        drop(node);
        if port.is_none() {
            return;
        }
        let refresh = Instant::now() + Duration::from_secs(120);
        while Instant::now() < refresh {
            if cancel.load(Ordering::Relaxed) {
                return;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}

pub(super) fn public_address(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    !(a == 0
        || a == 10
        || a == 127
        || a >= 224
        || (a == 100 && (64..=127).contains(&b))
        || (a == 169 && b == 254)
        || (a == 172 && (16..=31).contains(&b))
        || (a == 192 && b == 168)
        || (a == 192 && b == 0)
        || (a == 192 && b == 88 && c == 99)
        || (a == 198 && (b == 18 || b == 19 || (b == 51 && c == 100)))
        || (a == 203 && b == 0 && c == 113))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn untrusted_dht_cannot_target_local_reserved_or_multicast_networks() {
        for ip in [
            "0.1.2.3",
            "10.4.5.6",
            "127.0.0.1",
            "100.64.1.1",
            "100.127.1.1",
            "169.254.1.1",
            "172.16.0.1",
            "172.31.255.254",
            "192.168.1.1",
            "192.0.2.1",
            "198.18.1.1",
            "198.19.1.1",
            "198.51.100.1",
            "203.0.113.1",
            "224.1.1.1",
            "255.255.255.255",
        ] {
            assert!(!public_address(ip.parse().unwrap()), "{ip}");
        }
        for ip in [
            "1.1.1.1",
            "8.8.8.8",
            "100.63.1.1",
            "100.128.1.1",
            "172.15.0.1",
            "172.32.0.1",
        ] {
            assert!(public_address(ip.parse().unwrap()), "{ip}");
        }
    }
    #[test]
    fn cancellation_does_not_wait_for_unresponsive_discovery() {
        let cancel = AtomicBool::new(true);
        assert_eq!(
            bounded(
                std::future::pending::<()>(),
                Instant::now() + LOOKUP_TIME,
                &cancel
            ),
            None
        );
        assert_eq!(
            bounded(
                std::future::ready(42),
                Instant::now() + LOOKUP_TIME,
                &AtomicBool::new(false)
            ),
            Some(42)
        );
    }
    #[test]
    fn isolated_dht_can_announce_and_discover_game_namespace() {
        // Only loopback nodes: tests never publish fictional hosts publicly.
        let network = mainline::Testnet::builder(3).build().unwrap();
        let make_node = || {
            mainline::Dht::builder()
                .bootstrap(&network.bootstrap)
                .bind_address(Ipv4Addr::LOCALHOST)
                .request_timeout(Duration::from_millis(250))
                .build()
                .unwrap()
        };
        let host = make_node();
        let guest = make_node();
        let cancel = AtomicBool::new(false);
        let deadline = Instant::now() + Duration::from_secs(8);
        let announced = bounded(
            host.as_async().announce_peer(NAMESPACE.into(), Some(47654)),
            deadline,
            &cancel,
        );
        assert!(matches!(announced, Some(Ok(_))), "{announced:?}");
        let dht = guest.as_async();
        let mut peers = dht.get_peers(NAMESPACE.into());
        let mut found = false;
        while let Some(Some(batch)) = bounded(peers.next(), deadline, &cancel) {
            if batch
                .iter()
                .any(|peer| peer.port() == 47654 && peer.ip().is_loopback())
            {
                found = true;
                break;
            }
        }
        assert!(
            found,
            "game endpoint missing from isolated discovery network"
        );
    }
}
