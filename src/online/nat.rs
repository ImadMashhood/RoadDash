//! Temporary, host-only router mappings. A successful lease is not a WAN probe.
use bevy::prelude::Resource;
use igd_next::{Gateway, PortMappingProtocol, SearchOptions};
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
    },
    thread,
    time::{Duration, Instant},
};

const LEASE_SECONDS: u32 = 120;
const RENEW_AFTER: Duration = Duration::from_secs(50);

enum Event {
    Mapped(SocketAddr),
    Failed(String),
}

#[derive(Resource, Default)]
pub struct NatMapping {
    cancel: Option<Arc<AtomicBool>>,
    events: Option<Mutex<Receiver<Event>>>,
    pub endpoint: Option<SocketAddr>,
    pub status: String,
    retain: Option<Arc<AtomicBool>>,
    token: Option<String>,
    has_started: bool,
}
impl NatMapping {
    pub fn start(&mut self, local: SocketAddr) {
        self.stop();
        if std::env::var("ROADDASH_UPNP").is_ok_and(|v| v == "0")
            || std::env::var_os("ROADDASH_ADVERTISE_ADDR").is_some()
        {
            self.status = "Automatic router mapping disabled".into();
            return;
        }
        if local.port() == 0 || local.ip().is_loopback() || !local.is_ipv4() {
            self.status = "This bind address cannot be mapped for Internet hosting".into();
            return;
        }
        let cancel = Arc::new(AtomicBool::new(false));
        let retain = Arc::new(AtomicBool::new(false));
        let (token, reuse) = match inherited_token(!self.has_started) {
            Ok(value) => value,
            Err(error) => {
                self.status = error;
                return;
            }
        };
        let description = format!("RoadDash {token}");
        self.has_started = true;
        self.token = Some(token);
        self.retain = Some(retain.clone());
        let (tx, rx) = mpsc::sync_channel(4);
        self.cancel = Some(cancel.clone());
        self.events = Some(Mutex::new(rx));
        self.status = "Checking temporary router mapping".into();
        if let Err(error) = thread::Builder::new()
            .name("host-router-mapping".into())
            .spawn(move || {
                let result = open_mapping(local, &cancel, description, reuse, retain);
                match result {
                    Ok(mut lease) => {
                        if !cancel.load(Ordering::Relaxed) {
                            let _ = tx.try_send(Event::Mapped(lease.endpoint));
                        }
                        let mut renew_at = Instant::now() + RENEW_AFTER;
                        while !cancel.load(Ordering::Relaxed) {
                            if Instant::now() >= renew_at {
                                if let Err(error) = lease.renew() {
                                    let _ = tx.try_send(Event::Failed(format!(
                                        "Router lease renewal failed: {error}"
                                    )));
                                    break;
                                }
                                renew_at = Instant::now() + RENEW_AFTER;
                            }
                            thread::sleep(Duration::from_millis(100));
                        }
                        // RAII removes only this session's allocated mapping. If the
                        // process dies first, the finite lease bounds its lifetime.
                    }
                    Err(error) => {
                        let _ = tx.try_send(Event::Failed(error));
                    }
                }
            })
        {
            self.status = error.to_string();
        }
    }
    pub fn poll(&mut self) {
        if let Some(events) = &self.events
            && let Ok(rx) = events.lock()
        {
            for event in rx.try_iter().take(4) {
                match event {
                    Event::Mapped(endpoint) => {
                        self.endpoint = Some(endpoint);
                        self.status =
                            "Temporary router mapping active; external connection not yet verified"
                                .into();
                    }
                    Event::Failed(error) => {
                        self.endpoint = None;
                        self.status = format!(
                            "Automatic hosting unavailable: {error}. Manual forwarding may be needed"
                        );
                    }
                }
            }
        }
    }
    pub fn stop(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel.store(true, Ordering::Relaxed);
        }
        self.events = None;
        self.endpoint = None;
        self.status.clear();
        self.token = None;
        self.retain = None;
    }
    /// Only for an immediately relaunched host; the finite lease still expires
    /// if the child fails. Call cancel_handoff if process creation fails.
    pub fn handoff(&mut self) -> Option<(u16, String)> {
        let port = self.endpoint?.port();
        let token = self.token.clone()?;
        self.retain.as_ref()?.store(true, Ordering::Relaxed);
        Some((port, token))
    }
    pub fn cancel_handoff(&mut self) {
        if let Some(retain) = &self.retain {
            retain.store(false, Ordering::Relaxed);
        }
    }
}
impl Drop for NatMapping {
    fn drop(&mut self) {
        self.stop();
    }
}

trait Router {
    fn renew(&mut self, external: u16, local: SocketAddr, description: &str) -> Result<(), String>;
    fn remove(&mut self, external: u16, local: SocketAddr, description: &str);
}
impl Router for Gateway {
    fn renew(&mut self, external: u16, local: SocketAddr, description: &str) -> Result<(), String> {
        verify_owned_mapping(self, external, local, description, &AtomicBool::new(false))?;
        self.add_port(
            PortMappingProtocol::TCP,
            external,
            local,
            LEASE_SECONDS,
            description,
        )
        .map_err(|e| e.to_string())
    }
    fn remove(&mut self, external: u16, local: SocketAddr, description: &str) {
        if verify_owned_mapping(self, external, local, description, &AtomicBool::new(false)).is_ok()
        {
            let _ = self.remove_port(PortMappingProtocol::TCP, external);
        }
    }
}
struct Lease<R: Router> {
    router: R,
    endpoint: SocketAddr,
    local: SocketAddr,
    description: String,
    retain: Arc<AtomicBool>,
}
impl<R: Router> Lease<R> {
    fn renew(&mut self) -> Result<(), String> {
        self.router
            .renew(self.endpoint.port(), self.local, &self.description)
    }
}
impl<R: Router> Drop for Lease<R> {
    fn drop(&mut self) {
        if !self.retain.load(Ordering::Relaxed) {
            self.router
                .remove(self.endpoint.port(), self.local, &self.description);
        }
    }
}

fn inherited_token(allow_inherited: bool) -> Result<(String, Option<u16>), String> {
    if allow_inherited
        && let (Ok(port), Ok(token)) = (
            std::env::var("ROADDASH_NAT_REUSE_PORT"),
            std::env::var("ROADDASH_NAT_TOKEN"),
        )
    {
        let port = port
            .parse::<u16>()
            .map_err(|_| "Invalid inherited router port")?;
        if port == 0 || token.len() != 32 || !token.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("Invalid inherited router ownership token".into());
        }
        return Ok((token, Some(port)));
    }
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes).map_err(|e| e.to_string())?;
    Ok((bytes.iter().map(|b| format!("{b:02x}")).collect(), None))
}

fn open_mapping(
    mut local: SocketAddr,
    cancel: &AtomicBool,
    description: String,
    reuse: Option<u16>,
    retain: Arc<AtomicBool>,
) -> Result<Lease<Gateway>, String> {
    let gateway = igd_next::search_gateway(SearchOptions {
        timeout: Some(Duration::from_secs(3)),
        single_search_timeout: Some(Duration::from_secs(1)),
        ..Default::default()
    })
    .map_err(|e| e.to_string())?;
    if cancel.load(Ordering::Relaxed) {
        return Err("Hosting canceled".into());
    }
    if local.ip().is_unspecified() {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
        socket.connect(gateway.addr).map_err(|e| e.to_string())?;
        local.set_ip(socket.local_addr().map_err(|e| e.to_string())?.ip());
    }
    let external_ip = gateway.get_external_ip().map_err(|e| e.to_string())?;
    match external_ip {
        std::net::IpAddr::V4(ip) if super::discovery::public_address(ip) => {}
        _ => return Err("Router has no public IPv4 address (possibly carrier NAT)".into()),
    }
    if cancel.load(Ordering::Relaxed) {
        return Err("Hosting canceled".into());
    }
    let port = if let Some(port) = reuse {
        verify_owned_mapping(&gateway, port, local, &description, cancel)?;
        gateway
            .add_port(
                PortMappingProtocol::TCP,
                port,
                local,
                LEASE_SECONDS,
                &description,
            )
            .map_err(|e| e.to_string())?;
        port
    } else {
        gateway
            .add_any_port(PortMappingProtocol::TCP, local, LEASE_SECONDS, &description)
            .map_err(|e| e.to_string())?
    };
    Ok(Lease {
        router: gateway,
        endpoint: SocketAddr::new(external_ip, port),
        local,
        description,
        retain,
    })
}

fn verify_owned_mapping(
    gateway: &Gateway,
    port: u16,
    local: SocketAddr,
    description: &str,
    cancel: &AtomicBool,
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(3);
    for index in 0..64 {
        if cancel.load(Ordering::Relaxed) || Instant::now() >= deadline {
            return Err("Router mapping handoff inspection timed out".into());
        }
        let entry = gateway
            .get_generic_port_mapping_entry(index)
            .map_err(|e| e.to_string())?;
        if entry.external_port == port && entry.protocol == PortMappingProtocol::TCP {
            if entry.internal_port == local.port()
                && entry.internal_client == local.ip().to_string()
                && entry.port_mapping_description == description
            {
                return Ok(());
            }
            return Err("Requested router port belongs to another mapping".into());
        }
    }
    Err("Owned router mapping was not found; refusing to replace another mapping".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone)]
    struct Mock {
        calls: Arc<Mutex<Vec<String>>>,
        fail: bool,
    }
    impl Router for Mock {
        fn renew(&mut self, external: u16, local: SocketAddr, _: &str) -> Result<(), String> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("renew {external} {local}"));
            if self.fail {
                Err("refused".into())
            } else {
                Ok(())
            }
        }
        fn remove(&mut self, external: u16, _: SocketAddr, _: &str) {
            self.calls
                .lock()
                .unwrap()
                .push(format!("remove {external}"));
        }
    }
    #[test]
    fn lease_renews_and_removes_only_its_allocated_port_even_after_failure() {
        for fail in [false, true] {
            let calls = Arc::new(Mutex::new(Vec::new()));
            {
                let mut lease = Lease {
                    router: Mock {
                        calls: calls.clone(),
                        fail,
                    },
                    endpoint: "1.2.3.4:55001".parse().unwrap(),
                    local: "192.168.1.2:47654".parse().unwrap(),
                    description: "RoadDash test".into(),
                    retain: Arc::new(AtomicBool::new(false)),
                };
                assert_eq!(lease.renew().is_err(), fail);
            }
            assert_eq!(
                *calls.lock().unwrap(),
                vec!["renew 55001 192.168.1.2:47654", "remove 55001"]
            );
        }
    }
    #[test]
    fn explicit_handoff_keeps_finite_lease_for_child_only() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let retain = Arc::new(AtomicBool::new(true));
        let lease = Lease {
            router: Mock {
                calls: calls.clone(),
                fail: false,
            },
            endpoint: "1.2.3.4:55001".parse().unwrap(),
            local: "192.168.1.2:47654".parse().unwrap(),
            description: "RoadDash test".into(),
            retain,
        };
        drop(lease);
        assert!(calls.lock().unwrap().is_empty());
        assert!(LEASE_SECONDS > RENEW_AFTER.as_secs() as u32);
    }
}
