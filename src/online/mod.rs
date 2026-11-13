//! Online session interface. Lobby state is supplied by the shared service.
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnlineMap {
    #[default]
    Metropolis,
    Valley,
    City,
    Hills,
}
impl OnlineMap {
    pub fn label(self) -> &'static str {
        match self {
            Self::Metropolis => "Metropolis",
            Self::Valley => "Valley Run",
            Self::City => "City Run",
            Self::Hills => "Hills",
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnlinePhase {
    #[default]
    Offline,
    Connecting,
    Lobby,
    Countdown,
    Racing,
    Results,
    Error,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OnlineAction {
    Host { public: bool, map: OnlineMap },
    Join { code: String },
    QuickPlay,
    SetMap(OnlineMap),
    Start,
    Leave,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LobbyPlayer {
    pub slot: usize,
    pub name: String,
    pub bot: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LobbyView {
    pub code: String,
    pub public: bool,
    pub host: bool,
    pub map: OnlineMap,
    pub players: Vec<LobbyPlayer>,
    pub countdown: Option<f32>,
}
impl LobbyView {
    fn valid_for_guest(&self) -> bool {
        let mut slots = std::collections::BTreeSet::new();
        !self.host
            && self.code.len() <= 64
            && self.players.len() == RACERS
            && self.players.iter().all(|p| {
                p.slot < RACERS
                    && slots.insert(p.slot)
                    && p.name.len() <= 64
                    && !p.name.chars().any(char::is_control)
            })
            && self
                .countdown
                .is_none_or(|t| t.is_finite() && (0.0..=15.1).contains(&t))
    }
}
#[derive(Resource)]
pub struct Online {
    pub status: String,
    pub voice_status: String,
    pub(crate) voice_mutes: BTreeMap<usize, String>,
    pub endpoint: String,
    pub name: String,
    pub phase: OnlinePhase,
    pub error: Option<String>,
    pub lobby: Option<LobbyView>,
    pub actions: VecDeque<OnlineAction>,
}
impl Default for Online {
    fn default() -> Self {
        Self {
            status: String::new(),
            voice_status: String::new(),
            voice_mutes: BTreeMap::new(),
            endpoint: std::env::var("ROADDASH_SERVER").unwrap_or_else(|_| "0.0.0.0:47654".into()),
            name: std::env::var("ROADDASH_PLAYER_NAME").unwrap_or_else(|_| "Rider".into()),
            phase: OnlinePhase::Offline,
            error: None,
            lobby: None,
            actions: VecDeque::new(),
        }
    }
}

impl Online {
    pub fn voice_muted(&self, slot: usize) -> bool {
        self.voice_mutes.get(&slot).is_some_and(|name| {
            self.lobby.as_ref().is_some_and(|l| {
                l.players
                    .iter()
                    .any(|p| p.slot == slot && !p.bot && p.name == *name)
            })
        })
    }

    pub fn toggle_voice_mute(&mut self, slot: usize) {
        if self.voice_muted(slot) {
            self.voice_mutes.remove(&slot);
        } else if let Some(player) = self
            .lobby
            .as_ref()
            .and_then(|l| l.players.iter().find(|p| p.slot == slot && !p.bot))
        {
            self.voice_mutes.insert(slot, player.name.clone());
        }
    }
}

mod audio;
pub mod discovery;
mod matchmaking;
mod nat;
pub mod replication;
pub mod verification;
pub(crate) mod voice;
mod wire;
mod world_state;
use rr_core::online::{Input, PROTOCOL, RACERS};
use std::{
    collections::BTreeMap,
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket},
    time::{Duration, Instant},
};
use wire::Peer;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum ClientMessage {
    Hello {
        protocol: u32,
        name: String,
        secret: String,
    },
    Input(Input),
    Voice(voice::Packet),
    Leave,
    Probe {
        protocol: u32,
        nonce: u64,
    },
}
#[derive(Serialize, Deserialize)]
enum ServerMessage {
    Welcome {
        slot: usize,
        lobby: LobbyView,
        phase: OnlinePhase,
    },
    Lobby {
        lobby: LobbyView,
        phase: OnlinePhase,
    },
    Snapshot(replication::Snapshot),
    Voice {
        speaker: usize,
        gain: u8,
        packet: voice::Packet,
    },
    Reload {
        map: OnlineMap,
        code: String,
    },
    Error(String),
    Offer {
        protocol: u32,
        nonce: u64,
        available: bool,
    },
}
#[derive(Serialize)]
enum VoiceRelay<'a> {
    Voice {
        speaker: usize,
        gain: u8,
        packet: &'a voice::Packet,
    },
}

fn voice_gain(positions: &[Option<Vec3>; RACERS], speaker: usize, listener: usize) -> u8 {
    if speaker == listener {
        return 0;
    }
    positions[speaker]
        .zip(positions[listener])
        .map_or(0, |(speaker, listener)| {
            rr_core::online::proximity_voice_gain(speaker.distance(listener))
        })
}

fn relay_voice(
    guests: &mut [Guest],
    positions: &[Option<Vec3>; RACERS],
    speaker: usize,
    packet: &voice::Packet,
) {
    for guest in guests {
        if let Some(slot) = guest.slot {
            let gain = voice_gain(positions, speaker, slot);
            if gain > 0 {
                guest.peer.send_ephemeral(&VoiceRelay::Voice {
                    speaker,
                    gain,
                    packet,
                });
            }
        }
    }
}

struct Guest {
    peer: Peer,
    slot: Option<usize>,
    name: String,
    seen: Instant,
    rate_start: Instant,
    rate: u32,
    voice_rate: u32,
    sequence: u64,
}
struct Host {
    listener: TcpListener,
    discovery: Option<UdpSocket>,
    guests: Vec<Guest>,
    public: bool,
    map: OnlineMap,
    code: String,
    countdown: Option<Instant>,
    last_lobby: Instant,
    started: bool,
    tick: u64,
    elapsed: f32,
    seed: u64,
    secret: String,
    expected_guests: usize,
    ready_deadline: Instant,
}
struct Client {
    peer: Peer,
    sequence: u64,
    seen: Instant,
    last_input: Instant,
    code: String,
}
#[derive(Default)]
enum Connection {
    #[default]
    None,
    Host(Host),
    Client(Client),
    Discover(matchmaking::Search),
}
#[derive(Resource, Default)]
pub struct AuthorityInputs(pub BTreeMap<usize, Input>);
impl AuthorityInputs {
    pub fn get(&self, id: usize) -> Option<Input> {
        self.0.get(&id).copied()
    }
}
#[derive(Resource, Default)]
pub struct NetState {
    connection: Connection,
    pub(crate) local_slot: usize,
    pending: Option<replication::Snapshot>,
    pub(crate) launch: bool,
    retry: Option<(OnlineAction, Instant)>,
    resume_deadline: Option<Instant>,
    last_audio: u64,
    quickplay: bool,
    quickplay_map: OnlineMap,
    fallback_hosts: VecDeque<(String, Duration)>,
}

pub struct OnlinePlugin;
impl Plugin for OnlinePlugin {
    fn build(&self, app: &mut App) {
        let mut online = Online::default();
        if let Ok(value) = std::env::var("ROADDASH_ONLINE_BOOT")
            && let Ok(action) = serde_json::from_str(&value)
        {
            online.actions.push_back(action);
        }
        let state = NetState {
            resume_deadline: std::env::var_os("ROADDASH_ONLINE_BOOT")
                .map(|_| Instant::now() + Duration::from_secs(90)),
            ..default()
        };
        app.insert_resource(state)
            .insert_resource(online)
            .insert_non_send(voice::Voice::default())
            .init_resource::<NetState>()
            .init_resource::<AuthorityInputs>()
            .init_resource::<discovery::Discovery>()
            .init_resource::<nat::NatMapping>()
            .init_resource::<audio::Events>()
            .add_systems(
                PreUpdate,
                pump.after(crate::controls::capture)
                    .after(crate::frontend::FrontendSet::Input)
                    .before(crate::game::input),
            )
            .add_systems(
                FixedPreUpdate,
                replication::receive.before(crate::vehicle::attach),
            )
            .add_systems(
                FixedUpdate,
                (audio::collect, publish)
                    .chain()
                    .after(crate::runtime::SimSet::AfterPhysics),
            );
    }
}
pub fn simulates(state: Option<Res<NetState>>) -> bool {
    state.is_none_or(|s| !matches!(s.connection, Connection::Client(_)))
}
pub fn local_slot(world: &World) -> usize {
    world.get_resource::<NetState>().map_or(0, |s| s.local_slot)
}
pub fn remote_client(online: Option<&Online>) -> bool {
    online.is_some_and(|o| o.lobby.as_ref().is_some_and(|l| !l.host))
}
fn address(code: &str) -> Result<SocketAddr, String> {
    if let Some((ip, port)) =
        rr_core::online::decode_endpoint(code.trim().get(..10).unwrap_or(code.trim()))
    {
        return Ok(SocketAddr::from((ip, port)));
    }
    code.trim()
        .parse()
        .map_err(|_| "Enter the complete host code or IPv4:port".into())
}
fn local_ip() -> Ipv4Addr {
    let socket = UdpSocket::bind("0.0.0.0:0");
    socket
        .and_then(|s| {
            s.connect("192.0.2.1:80")?;
            s.local_addr()
        })
        .ok()
        .and_then(|a| match a.ip() {
            std::net::IpAddr::V4(v) => Some(v),
            _ => None,
        })
        .unwrap_or(Ipv4Addr::LOCALHOST)
}
fn open_host(endpoint: &str, public: bool, map: OnlineMap) -> Result<Host, String> {
    let bind: SocketAddr = endpoint
        .parse()
        .map_err(|_| "Host bind address must be IPv4:port")?;
    let listener =
        TcpListener::bind(bind).map_err(|e| format!("Cannot host on {endpoint}: {e}"))?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    let advertised = std::env::var("ROADDASH_ADVERTISE_ADDR")
        .ok()
        .and_then(|s| s.parse::<SocketAddr>().ok());
    let ip = std::env::var("ROADDASH_ADVERTISE")
        .ok()
        .and_then(|s| s.parse::<Ipv4Addr>().ok())
        .unwrap_or_else(|| match bind.ip() {
            std::net::IpAddr::V4(ip) if !ip.is_unspecified() => ip,
            _ => local_ip(),
        });
    let (ip, port) = advertised
        .and_then(|a| match a.ip() {
            std::net::IpAddr::V4(ip) => Some((ip, a.port())),
            _ => None,
        })
        .unwrap_or((ip, port));
    let discovery = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 47655)).ok();
    if let Some(d) = &discovery {
        d.set_nonblocking(true).map_err(|e| e.to_string())?;
    }
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
        | 1;
    static USED_RESUME: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    let resumed = !USED_RESUME.swap(true, std::sync::atomic::Ordering::Relaxed);
    let resume_secret = if resumed {
        std::env::var("ROADDASH_HOST_SECRET").ok().filter(|s| {
            s.len() == 26
                && s.bytes()
                    .all(|c| b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(&c))
        })
    } else {
        None
    };
    let secret = if public {
        String::new()
    } else if let Some(secret) = resume_secret {
        secret
    } else {
        let mut bytes = [0u8; 16];
        getrandom::fill(&mut bytes).map_err(|e| e.to_string())?;
        let mut bits = u128::from_be_bytes(bytes);
        let mut out = [b'A'; 26];
        for c in out.iter_mut().rev() {
            *c = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"[(bits & 31) as usize];
            bits >>= 5;
        }
        String::from_utf8(out.to_vec()).unwrap()
    };
    Ok(Host {
        listener,
        discovery,
        guests: vec![],
        public,
        map,
        code: format!(
            "{}{}",
            rr_core::online::encode_endpoint(ip.octets(), port),
            secret
        ),
        secret,
        expected_guests: if resumed {
            std::env::var("ROADDASH_EXPECTED_GUESTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0)
                .min(14)
        } else {
            0
        },
        ready_deadline: Instant::now() + Duration::from_secs(90),
        countdown: None,
        last_lobby: Instant::now() - Duration::from_secs(1),
        started: false,
        tick: 0,
        elapsed: 0.,
        seed,
    })
}
fn connect(code: &str, name: &str) -> Result<Client, String> {
    let target = address(code)?;
    let stream = TcpStream::connect_timeout(&target, Duration::from_millis(350))
        .map_err(|e| format!("Host unavailable: {e}"))?;
    let mut peer = Peer::new(stream).map_err(|e| e.to_string())?;
    peer.send(&ClientMessage::Hello {
        protocol: PROTOCOL,
        secret: if code.len() == 36 {
            code[10..].to_string()
        } else {
            String::new()
        },
        name: name.chars().filter(|c| !c.is_control()).take(24).collect(),
    })
    .map_err(|e| e.to_string())?;
    Ok(Client {
        peer,
        sequence: 0,
        seen: Instant::now(),
        last_input: Instant::now() - Duration::from_secs(1),
        code: code.into(),
    })
}
fn map_matches(map: OnlineMap, track: &crate::track::Track) -> bool {
    if std::env::args().any(|a| a == "--online-test-node") {
        return true;
    }
    match map {
        OnlineMap::Metropolis => track.metropolis.is_some(),
        OnlineMap::Valley => track.definition.id.as_str() == "valley-run-01",
        OnlineMap::City => track.city_surface.is_some(),
        OnlineMap::Hills => track.is_hills(),
    }
}
#[allow(clippy::too_many_arguments)]
fn relaunch(
    map: OnlineMap,
    action: OnlineAction,
    endpoint: &str,
    name: &str,
    secret: &str,
    reuse: Option<(u16, String)>,
    expected: usize,
) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    std::process::Command::new(executable)
        .args(crate::frontend::map_launch_arguments(
            std::env::args_os().skip(1),
        ))
        .env_remove("ROADDASH_OFFLINE_BOOT")
        .env_remove("ROADDASH_OFFLINE_STATE")
        .env("ROADDASH_EXPECTED_GUESTS", expected.to_string())
        .env(
            "ROADDASH_TRACK",
            match map {
                OnlineMap::Metropolis => "metro",
                OnlineMap::Valley => "valley",
                OnlineMap::City => "city",
                OnlineMap::Hills => "hills",
            },
        )
        .env("ROADDASH_SERVER", endpoint)
        .env("ROADDASH_PLAYER_NAME", name)
        .env("ROADDASH_HOST_SECRET", secret)
        .env(
            "ROADDASH_NAT_REUSE_PORT",
            reuse.as_ref().map_or(String::new(), |(p, _)| p.to_string()),
        )
        .env(
            "ROADDASH_NAT_TOKEN",
            reuse.as_ref().map_or("", |(_, token)| token.as_str()),
        )
        .env(
            "ROADDASH_ONLINE_BOOT",
            serde_json::to_string(&action).map_err(|e| e.to_string())?,
        )
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
fn lobby(host: &Host, name: &str, host_view: bool) -> LobbyView {
    LobbyView {
        code: host.code.clone(),
        public: host.public,
        host: host_view,
        map: host.map,
        countdown: host
            .countdown
            .map(|d| d.saturating_duration_since(Instant::now()).as_secs_f32()),
        players: (0..RACERS)
            .map(|slot| {
                if slot == 0 {
                    LobbyPlayer {
                        slot,
                        name: name.into(),
                        bot: false,
                    }
                } else if let Some(g) = host.guests.iter().find(|g| g.slot == Some(slot)) {
                    LobbyPlayer {
                        slot,
                        name: g.name.clone(),
                        bot: false,
                    }
                } else {
                    LobbyPlayer {
                        slot,
                        name: format!("{} · BOT", crate::game::NAMES[slot]),
                        bot: true,
                    }
                }
            })
            .collect(),
    }
}
#[derive(bevy::ecs::system::SystemParam)]
struct VoiceIo<'w, 's> {
    poses: Query<'w, 's, &'static Transform>,
    windows: Query<'w, 's, &'static Window, With<bevy::window::PrimaryWindow>>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    prefs: Option<Res<'w, crate::settings::Preferences>>,
    panel: Option<Res<'w, crate::settings::SettingsPanel>>,
    audio: NonSendMut<'w, voice::Voice>,
}

#[allow(clippy::too_many_arguments)]
fn pump(
    mut online: ResMut<Online>,
    mut state: ResMut<NetState>,
    mut inputs: ResMut<AuthorityInputs>,
    mut race: ResMut<crate::game::Race>,
    mut controls: ResMut<crate::controls::Controls>,
    track: Res<crate::track::Track>,
    screen: Res<State<crate::runtime::AppScreen>>,
    mut riders: Query<(
        &mut crate::game::Rider,
        &Transform,
        &crate::recovery::Recovery,
    )>,
    mut voice: VoiceIo,
    mut exit: MessageWriter<AppExit>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    mut commands: Commands,
    mut winit: Option<ResMut<bevy::winit::WinitSettings>>,
    mut discovery: ResMut<discovery::Discovery>,
    mut nat: ResMut<nat::NatMapping>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    if !matches!(state.connection, Connection::None)
        && let Some(winit) = winit.as_deref_mut()
    {
        *winit = bevy::winit::WinitSettings::continuous();
    }
    if state
        .retry
        .as_ref()
        .is_some_and(|(_, at)| Instant::now() >= *at)
    {
        let (action, _) = state.retry.take().unwrap();
        online.actions.push_back(action);
    }
    if online.actions.len() > 32 {
        online.actions.truncate(32);
    }
    while let Some(action) = online.actions.pop_front() {
        online.error = None;
        if matches!(
            action,
            OnlineAction::Host { .. }
                | OnlineAction::Join { .. }
                | OnlineAction::QuickPlay
                | OnlineAction::Leave
        ) {
            *voice.audio = default();
            online.voice_mutes.clear();
            online.voice_status.clear();
        }
        match action {
            OnlineAction::Leave => {
                discovery.stop();
                nat.stop();
                state.retry = None;
                state.quickplay = false;
                state.fallback_hosts.clear();
                state.resume_deadline = None;
                commands.queue(replication::clear_remote);
                state.connection = Connection::None;
                state.pending = None;
                state.launch = false;
                state.last_audio = 0;
                state.local_slot = 0;
                inputs.0.clear();
                online.phase = OnlinePhase::Offline;
                online.lobby = None;
                race.phase = crate::game::Phase::Garage;
                for (mut r, _, _) in &mut riders {
                    r.networked = false;
                    r.remote_human = false;
                }
            }
            OnlineAction::Host { public, map } => {
                prepare_host(&mut state, &mut online);
                if !map_matches(map, &track) {
                    match relaunch(
                        map,
                        OnlineAction::Host { public, map },
                        &online.endpoint,
                        &online.name,
                        "",
                        None,
                        0,
                    ) {
                        Ok(()) => {
                            exit.write(AppExit::Success);
                        }
                        Err(e) => online.error = Some(e),
                    }
                    return;
                }
                match open_host(&online.endpoint, public, map) {
                    Ok(host) => {
                        online.endpoint = host.listener.local_addr().unwrap().to_string();
                        nat.start(host.listener.local_addr().unwrap());
                        state.resume_deadline = None;
                        if public {
                            discovery.announce(host.listener.local_addr().unwrap().port());
                        }
                        online.phase = OnlinePhase::Lobby;
                        online.lobby = Some(lobby(&host, &online.name, true));
                        state.connection = Connection::Host(host);
                        state.local_slot = 0;
                        race.phase = crate::game::Phase::Garage;
                    }
                    Err(e) => {
                        if state.resume_deadline.is_some_and(|d| Instant::now() < d) {
                            state.retry = Some((
                                OnlineAction::Host { public, map },
                                Instant::now() + Duration::from_millis(500),
                            ));
                            online.status = "Waiting for previous host to release its port…".into();
                            online.phase = OnlinePhase::Connecting;
                        } else {
                            online.error = Some(e);
                            online.phase = OnlinePhase::Error;
                        }
                    }
                }
            }
            OnlineAction::Join { code } => match connect(&code, &online.name) {
                Ok(client) => {
                    state.connection = Connection::Client(client);
                    online.phase = OnlinePhase::Connecting;
                    race.phase = crate::game::Phase::Garage;
                }
                Err(e) => {
                    if state.resume_deadline.is_some_and(|d| Instant::now() < d) {
                        state.retry = Some((
                            OnlineAction::Join { code },
                            Instant::now() + Duration::from_millis(500),
                        ));
                        online.status = "Waiting for host readiness…".into();
                        online.phase = OnlinePhase::Connecting;
                    } else if state.quickplay {
                        let action = next_quickplay(&mut state, &mut online);
                        state.retry = Some((action, Instant::now()));
                    } else {
                        online.error = Some(e);
                        online.phase = OnlinePhase::Error;
                    }
                }
            },
            OnlineAction::QuickPlay => {
                state.quickplay = true;
                state.quickplay_map = loaded_map(&track);
                state.fallback_hosts.clear();
                discovery.begin_search();
                match matchmaking::Search::new() {
                    Ok(search) => {
                        state.connection = Connection::Discover(search);
                        online.phase = OnlinePhase::Connecting;
                        online.status =
                            "Discovering public hosts and measuring RTT (maximum 250 ms)…".into();
                    }
                    Err(e) => {
                        online.error = Some(e.to_string());
                        online.phase = OnlinePhase::Error;
                    }
                }
            }
            OnlineAction::Start => {
                if let Connection::Host(h) = &mut state.connection
                    && !h.started
                {
                    if h.guests.iter().filter(|g| g.slot.is_some()).count() < h.expected_guests
                        && Instant::now() < h.ready_deadline
                    {
                        online.error = Some(
                            "Waiting for returning players to finish loading the selected level"
                                .into(),
                        );
                        continue;
                    }
                    h.countdown = Some(Instant::now() + Duration::from_secs(15));
                    online.phase = OnlinePhase::Countdown;
                }
            }
            OnlineAction::SetMap(map) => {
                if let Connection::Host(h) = &mut state.connection
                    && !h.started
                    && h.countdown.is_none()
                    && map != h.map
                {
                    match relaunch(
                        map,
                        OnlineAction::Host {
                            public: h.public,
                            map,
                        },
                        &online.endpoint,
                        &online.name,
                        &h.secret,
                        nat.handoff(),
                        h.guests.iter().filter(|g| g.slot.is_some()).count(),
                    ) {
                        Ok(()) => {
                            for g in &mut h.guests {
                                let _ = g.peer.send(&ServerMessage::Reload {
                                    map,
                                    code: h.code.clone(),
                                });
                            }
                            exit.write(AppExit::Success);
                        }
                        Err(e) => {
                            nat.cancel_handoff();
                            online.error = Some(e);
                        }
                    }
                }
            }
        }
    }
    let mut failure = None;
    let mut next = None;
    let mut pending = None;
    let mut slot = None;
    let mut launch = false;
    let mut welcomed = false;
    let active_voice = online.lobby.is_some()
        && matches!(online.phase, OnlinePhase::Countdown | OnlinePhase::Racing);
    let voice_prefs = voice.prefs.as_ref().map(|p| &p.file.voice);
    let listening = active_voice && voice_prefs.is_some_and(|v| v.enabled);
    let focused = voice.windows.single().is_ok_and(|window| window.focused);
    let transmitting = listening
        && focused
        && !voice.panel.as_ref().is_some_and(|p| p.open)
        && voice_prefs.is_some_and(|v| v.microphone);
    let gain = voice
        .prefs
        .as_ref()
        .filter(|p| !p.file.audio_muted)
        .map_or(0., |p| p.file.voice.volume * p.file.master_volume);
    let push = voice.prefs.as_ref().is_some_and(|p| {
        p.file.voice_key_available(&p.file.voice.push_key)
            && crate::controls::key_code(&p.file.voice.push_key)
                .is_some_and(|key| voice.keys.pressed(key))
    });
    voice.audio.configure(transmitting, listening, gain);
    let outgoing = voice_prefs.map_or_else(Vec::new, |v| voice.audio.outgoing(v, push));
    let mut positions = [None; RACERS];
    if active_voice && matches!(state.connection, Connection::Host(_)) {
        for (rider, transform, recovery) in &riders {
            if let Some(position) = positions.get_mut(rider.id) {
                *position = Some(
                    recovery
                        .runner
                        .or(recovery.torso)
                        .and_then(|e| voice.poses.get(e).ok())
                        .unwrap_or(transform)
                        .translation,
                );
            }
        }
    }
    for slot in 0..RACERS {
        if online.voice_muted(slot) {
            voice.audio.clear_slot(slot);
        }
    }
    let local_slot = state.local_slot;
    match &mut state.connection {
        Connection::None => {}
        Connection::Discover(search) => {
            for address in discovery.poll_candidates() {
                search.candidate(address.to_string());
            }
            if let Some(result) = search.poll() {
                state.fallback_hosts = result.into();
                next = Some(next_quickplay(&mut state, &mut online));
            }
        }
        Connection::Client(client) => {
            match client.peer.receive::<ServerMessage>(wire::FRAME_LIMIT) {
                Ok(messages) => {
                    for message in messages {
                        client.seen = Instant::now();
                        match message {
                            ServerMessage::Welcome {
                                slot: s,
                                lobby: l,
                                phase,
                            } => {
                                if s == 0 || s >= RACERS || !l.valid_for_guest() {
                                    failure = Some("Host sent an invalid lobby".into());
                                    break;
                                }
                                if !map_matches(l.map, &track) {
                                    match relaunch(
                                        l.map,
                                        OnlineAction::Join {
                                            code: client.code.clone(),
                                        },
                                        &online.endpoint,
                                        &online.name,
                                        "",
                                        None,
                                        0,
                                    ) {
                                        Ok(()) => {
                                            exit.write(AppExit::Success);
                                        }
                                        Err(e) => online.error = Some(e),
                                    }
                                    return;
                                }
                                welcomed = true;
                                slot = Some(s);
                                online.lobby = Some(l);
                                online.phase = phase;
                            }
                            ServerMessage::Lobby { lobby: l, phase } => {
                                if !l.valid_for_guest() {
                                    failure = Some("Host sent an invalid lobby".into());
                                    break;
                                }
                                online.lobby = Some(l);
                                if online.phase != OnlinePhase::Results {
                                    online.phase = phase;
                                }
                            }
                            ServerMessage::Snapshot(s) => {
                                if s.valid() {
                                    pending = Some(s);
                                } else {
                                    failure = Some("Host sent an invalid game snapshot".into());
                                }
                            }
                            ServerMessage::Voice {
                                speaker,
                                gain,
                                packet,
                            } => {
                                if listening
                                    && speaker != local_slot
                                    && !online.voice_muted(speaker)
                                {
                                    voice.audio.hear(speaker, gain, &packet);
                                }
                            }
                            ServerMessage::Error(e) => failure = Some(e),
                            ServerMessage::Offer { .. } => {}
                            ServerMessage::Reload { map, code: _ } => {
                                match relaunch(
                                    map,
                                    OnlineAction::Join {
                                        code: client.code.clone(),
                                    },
                                    &online.endpoint,
                                    &online.name,
                                    "",
                                    None,
                                    0,
                                ) {
                                    Ok(()) => {
                                        exit.write(AppExit::Success);
                                    }
                                    Err(e) => online.error = Some(e),
                                }
                                return;
                            }
                        }
                    }
                }
                Err(e) => failure = Some(format!("Connection lost: {e}")),
            }
            if client.seen.elapsed() > Duration::from_secs(10) {
                failure = Some("Host timed out".into());
            }
            if online.lobby.is_some()
                && client.last_input.elapsed() >= Duration::from_secs_f64(1. / 60.)
            {
                client.last_input = Instant::now();
                client.sequence += 1;
                let input = Input {
                    sequence: client.sequence,
                    vehicle: controls.buffer.held,
                    attack: controls.buffer.take_attack(),
                    foot: controls.foot.to_array(),
                    foot_forward: cameras
                        .iter()
                        .next()
                        .map_or(Vec2::Y, |c| c.forward().xz().normalize_or_zero())
                        .to_array(),
                    interact: std::mem::take(&mut controls.interact),
                    pickup: std::mem::take(&mut controls.pickup),
                    drop: std::mem::take(&mut controls.drop),
                };
                if let Err(e) = client.peer.send(&ClientMessage::Input(input)) {
                    failure = Some(e.to_string());
                }
                controls.pause = false;
                controls.restart = false;
                controls.start = false;
            }
            for packet in outgoing {
                client.peer.send_ephemeral(&ClientMessage::Voice(packet));
            }
        }
        Connection::Host(host) => {
            if host.started && race.phase == crate::game::Phase::Paused {
                race.phase = crate::game::Phase::Racing;
            }
            let _ = discovery.poll_candidates();
            nat.poll();
            let explicit = std::env::var_os("ROADDASH_ADVERTISE_ADDR").is_some()
                || std::env::var_os("ROADDASH_ADVERTISE").is_some();
            let advertised = if explicit {
                None
            } else {
                nat.endpoint.or_else(|| {
                    discovery.observed_ip.map(|ip| {
                        SocketAddr::from((ip, host.listener.local_addr().unwrap().port()))
                    })
                })
            };
            if let Some(SocketAddr::V4(endpoint)) = advertised {
                let code = format!(
                    "{}{}",
                    rr_core::online::encode_endpoint(endpoint.ip().octets(), endpoint.port()),
                    host.secret
                );
                if code != host.code {
                    host.code = code;
                    if host.public {
                        discovery.announce(endpoint.port());
                    }
                }
            }
            online.status = nat.status.clone();
            if let discovery::DiscoveryStatus::Unavailable(message) = &discovery.status {
                online.status = format!("{} · {}", online.status, message);
            }
            for _ in 0..16 {
                match host.listener.accept() {
                    Ok((stream, _)) => {
                        if host.guests.len() < 14
                            && let Ok(peer) = Peer::new(stream)
                        {
                            host.guests.push(Guest {
                                peer,
                                slot: None,
                                name: String::new(),
                                seen: Instant::now(),
                                rate_start: Instant::now(),
                                rate: 0,
                                voice_rate: 0,
                                sequence: 0,
                            });
                        }
                    }
                    Err(_) => break,
                }
            }
            if host.public
                && let Some(socket) = &host.discovery
            {
                let mut bytes = [0u8; 64];
                for _ in 0..16 {
                    match socket.recv_from(&mut bytes) {
                        Ok((len, addr))
                            if &bytes[..len] == b"ROAD-DASH-DISCOVER-1"
                                && host.guests.len() < 14 =>
                        {
                            let lan_code = rr_core::online::encode_endpoint(
                                local_ip().octets(),
                                host.listener.local_addr().unwrap().port(),
                            );
                            let _ = socket.send_to(lan_code.as_bytes(), addr);
                        }
                        Ok(_) => {}
                        Err(_) => break,
                    }
                }
            }
            let mut occupied: Vec<usize> = host.guests.iter().filter_map(|g| g.slot).collect();
            let phase = if host.started {
                OnlinePhase::Racing
            } else if host.countdown.is_some() {
                OnlinePhase::Countdown
            } else {
                OnlinePhase::Lobby
            };
            let view = lobby(host, &online.name, false);
            let mut speech: Vec<_> = outgoing.into_iter().map(|packet| (0, packet)).collect();
            host.guests.retain_mut(|guest| {
                if guest.seen.elapsed() > Duration::from_secs(10) {
                    if let Some(s) = guest.slot {
                        inputs.0.remove(&s);
                    }
                    return false;
                }
                if guest.rate_start.elapsed() > Duration::from_secs(1) {
                    guest.rate_start = Instant::now();
                    guest.rate = 0;
                    guest.voice_rate = 0;
                }
                let Ok(messages) = guest.peer.receive::<ClientMessage>(4096) else {
                    if let Some(s) = guest.slot {
                        inputs.0.remove(&s);
                    }
                    return false;
                };
                guest.rate += messages.len() as u32;
                if guest.rate > 300 {
                    if let Some(s) = guest.slot {
                        inputs.0.remove(&s);
                    }
                    return false;
                }
                for message in messages {
                    guest.seen = Instant::now();
                    match message {
                        ClientMessage::Probe { protocol, nonce }
                            if protocol == PROTOCOL && guest.slot.is_none() =>
                        {
                            let _ = guest.peer.send(&ServerMessage::Offer {
                                protocol: PROTOCOL,
                                nonce,
                                available: host.public
                                    && occupied.len() < 14
                                    && riders.iter().any(|(r, _, _)| {
                                        r.id > 0
                                            && r.id < RACERS
                                            && !occupied.contains(&r.id)
                                            && r.finished.is_none()
                                    }),
                            });
                            return false;
                        }
                        ClientMessage::Hello {
                            protocol,
                            name,
                            secret,
                        } if guest.slot.is_none()
                            && protocol == PROTOCOL
                            && secret == host.secret
                            && name.len() <= 96 =>
                        {
                            let available = (1..RACERS).find(|s| {
                                !occupied.contains(s)
                                    && riders
                                        .iter()
                                        .any(|(r, _, _)| r.id == *s && r.finished.is_none())
                            });
                            let Some(s) = available else {
                                let _ = guest.peer.send(&ServerMessage::Error(
                                    "Race has no unfinished bot slots".into(),
                                ));
                                return false;
                            };
                            occupied.push(s);
                            guest.slot = Some(s);
                            guest.name =
                                name.chars().filter(|c| !c.is_control()).take(24).collect();
                            inputs.0.insert(s, Input::default());
                            let _ = guest.peer.send(&ServerMessage::Welcome {
                                slot: s,
                                lobby: view.clone(),
                                phase,
                            });
                        }
                        ClientMessage::Input(input)
                            if input.valid()
                                && guest.slot.is_some()
                                && input.sequence > guest.sequence =>
                        {
                            guest.sequence = input.sequence;
                            let s = guest.slot.unwrap();
                            let previous = inputs.0.entry(s).or_default();
                            // Retain edges through render frames that contain no fixed tick.
                            let attack = previous.attack.or(input.attack);
                            let interact = previous.interact || input.interact;
                            let pickup = previous.pickup || input.pickup;
                            let drop = previous.drop || input.drop;
                            *previous = input;
                            previous.attack = attack;
                            previous.interact = interact;
                            previous.pickup = pickup;
                            previous.drop = drop;
                        }
                        ClientMessage::Voice(packet) if guest.slot.is_some() => {
                            guest.voice_rate += 1;
                            if active_voice
                                && guest.voice_rate <= 32
                                && let Some(slot) = guest.slot
                                && packet.samples().is_some()
                            {
                                speech.push((slot, packet));
                            }
                        }
                        ClientMessage::Leave => {
                            if let Some(s) = guest.slot {
                                inputs.0.remove(&s);
                            }
                            return false;
                        }
                        _ => {
                            if let Some(s) = guest.slot {
                                inputs.0.remove(&s);
                            }
                            return false;
                        }
                    }
                }
                true
            });
            for (speaker, packet) in speech {
                if speaker != 0 && !host.guests.iter().any(|g| g.slot == Some(speaker)) {
                    continue;
                }
                if listening && !online.voice_muted(speaker) {
                    let gain = voice_gain(&positions, speaker, 0);
                    voice.audio.hear(speaker, gain, &packet);
                }
                relay_voice(&mut host.guests, &positions, speaker, &packet);
            }
            inputs
                .0
                .retain(|slot, _| host.guests.iter().any(|g| g.slot == Some(*slot)));
            for (mut rider, _, _) in &mut riders {
                if rider.id < RACERS {
                    rider.networked = true;
                    rider.remote_human = rider.id == 0 || inputs.0.contains_key(&rider.id);
                }
            }
            if host.countdown.is_some_and(|d| Instant::now() >= d) && !host.started {
                host.started = true;
                host.countdown = None;
                launch = true;
            }
            if host.last_lobby.elapsed() >= Duration::from_millis(200) {
                host.last_lobby = Instant::now();
                let phase = if race.phase == crate::game::Phase::Finished {
                    OnlinePhase::Results
                } else if host.started {
                    OnlinePhase::Racing
                } else if host.countdown.is_some() {
                    OnlinePhase::Countdown
                } else {
                    OnlinePhase::Lobby
                };
                let view = lobby(host, &online.name, false);
                for guest in &mut host.guests {
                    let _ = guest.peer.send(&ServerMessage::Lobby {
                        lobby: view.clone(),
                        phase,
                    });
                }
                online.lobby = Some(lobby(host, &online.name, true));
                online.phase = phase;
            }
        }
    }
    if let Some(s) = slot {
        state.local_slot = s;
        state.resume_deadline = None;
    }
    if welcomed {
        state.quickplay = false;
        state.fallback_hosts.clear();
    }
    if pending.is_some() {
        state.pending = pending;
    }
    if launch {
        state.launch = true;
        online.phase = OnlinePhase::Racing;
    }
    if let Some(action) = next {
        state.connection = Connection::None;
        online.actions.push_back(action);
    }
    if let Some(error) = failure {
        *voice.audio = default();
        online.voice_mutes.clear();
        state.connection = Connection::None;
        commands.queue(replication::clear_remote);
        state.local_slot = 0;
        state.pending = None;
        state.launch = false;
        inputs.0.clear();
        online.phase = OnlinePhase::Error;
        online.error = Some(error);
        if state.quickplay {
            online.error = None;
            let action = next_quickplay(&mut state, &mut online);
            state.retry = Some((action, Instant::now()));
            online.phase = OnlinePhase::Connecting;
        }
        online.lobby = None;
        race.phase = crate::game::Phase::Garage;
    }
    online.voice_status = voice.audio.status(transmitting, online.lobby.as_ref());
}
fn publish(world: &mut World) {
    replication::publish(world);
}

#[derive(Component)]
pub struct TrafficId(pub u32);

fn next_quickplay(state: &mut NetState, online: &mut Online) -> OnlineAction {
    if let Some((code, rtt)) = state.fallback_hosts.pop_front() {
        online.status = format!("Joining host · measured RTT {} ms", rtt.as_millis());
        OnlineAction::Join { code }
    } else {
        online.status = "No reachable host at ≤250 ms; hosting here".into();
        OnlineAction::Host {
            public: true,
            map: state.quickplay_map,
        }
    }
}
fn loaded_map(track: &crate::track::Track) -> OnlineMap {
    if track.is_hills() {
        OnlineMap::Hills
    } else if track.metropolis.is_some() {
        OnlineMap::Metropolis
    } else if track.city_surface.is_some() {
        OnlineMap::City
    } else {
        OnlineMap::Valley
    }
}
fn prepare_host(state: &mut NetState, online: &mut Online) {
    if state.quickplay {
        online.actions.push_back(OnlineAction::Start);
    }
    state.quickplay = false;
    state.fallback_hosts.clear();
}

#[cfg(test)]
mod admission_tests {
    use super::*;
    #[test]
    fn proximity_voice_reaches_only_a_nearby_admitted_peer_over_the_race_wire() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let mut guests = Vec::new();
        let mut clients = Vec::new();
        for slot in [Some(1), Some(2), Some(3), None] {
            clients.push(
                Peer::new(TcpStream::connect(listener.local_addr().unwrap()).unwrap()).unwrap(),
            );
            guests.push(Guest {
                peer: Peer::new(listener.accept().unwrap().0).unwrap(),
                slot,
                name: String::new(),
                seen: Instant::now(),
                rate_start: Instant::now(),
                rate: 0,
                voice_rate: 0,
                sequence: 0,
            });
        }
        let mut positions = [None; RACERS];
        positions[1] = Some(Vec3::ZERO);
        positions[2] = Some(Vec3::X * 25.);
        positions[3] = Some(Vec3::X * 75.);
        let packet = voice::Packet::from_samples([0.25; voice::SAMPLES]);
        relay_voice(&mut guests, &positions, 1, &packet);
        let deadline = Instant::now() + Duration::from_secs(1);
        loop {
            let messages = clients[1].receive::<ServerMessage>(4096).unwrap();
            if let [
                ServerMessage::Voice {
                    speaker,
                    gain,
                    packet,
                },
            ] = messages.as_slice()
            {
                assert_eq!(*speaker, 1);
                assert!((0..255).contains(gain));
                assert!((packet.samples().unwrap()[0] - 0.25).abs() < 0.01);
                break;
            }
            assert!(messages.is_empty() && Instant::now() < deadline);
            std::thread::yield_now();
        }
        for index in [0, 2, 3] {
            assert!(
                clients[index]
                    .receive::<ServerMessage>(4096)
                    .unwrap()
                    .is_empty()
            );
        }
    }

    #[test]
    fn failed_quickplay_candidates_fall_back_then_host() {
        let mut state = NetState {
            quickplay: true,
            ..default()
        };
        state
            .fallback_hosts
            .push_back(("127.0.0.1:9".into(), Duration::from_millis(249)));
        let mut online = Online::default();
        assert!(matches!(
            next_quickplay(&mut state, &mut online),
            OnlineAction::Join { .. }
        ));
        assert!(matches!(
            next_quickplay(&mut state, &mut online),
            OnlineAction::Host { public: true, .. }
        ));
        assert!(online.error.is_none());
    }
    #[test]
    fn empty_quickplay_on_supported_maps_preserves_automatic_start_without_relaunch() {
        for (json, expected) in [
            (
                include_str!("../../content/tracks/valley.json"),
                OnlineMap::Valley,
            ),
            (
                include_str!("../../content/tracks/city5300.json"),
                OnlineMap::City,
            ),
            (
                include_str!("../../content/tracks/hills.json"),
                OnlineMap::Hills,
            ),
        ] {
            let track = crate::track::Track::from_json(json).unwrap();
            let mut state = NetState {
                quickplay: true,
                quickplay_map: loaded_map(&track),
                ..default()
            };
            let mut online = Online::default();
            let OnlineAction::Host { public, map } = next_quickplay(&mut state, &mut online) else {
                panic!("empty lookup must host")
            };
            assert!(public);
            assert_eq!(map, expected);
            assert!(
                map_matches(map, &track),
                "fallback must not relaunch and lose queued countdown"
            );
            prepare_host(&mut state, &mut online);
            assert!(matches!(
                online.actions.pop_front(),
                Some(OnlineAction::Start)
            ));
            assert!(online.actions.is_empty());
        }
    }
    #[test]
    fn untrusted_lobby_cannot_enable_local_authority_or_duplicate_slots() {
        let mut lobby = LobbyView {
            code: "TEST".into(),
            public: true,
            host: false,
            map: OnlineMap::Metropolis,
            countdown: Some(15.),
            players: (0..RACERS)
                .map(|slot| LobbyPlayer {
                    slot,
                    name: "Rider".into(),
                    bot: slot > 0,
                })
                .collect(),
        };
        assert!(lobby.valid_for_guest());
        lobby.host = true;
        assert!(!lobby.valid_for_guest());
        lobby.host = false;
        lobby.players[1].slot = 0;
        assert!(!lobby.valid_for_guest());
        lobby.players[1].slot = 1;
        lobby.countdown = Some(f32::NAN);
        assert!(!lobby.valid_for_guest());
    }
}
