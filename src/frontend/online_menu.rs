//! Presentation-only adapter for the authoritative online session.
use super::*;
use crate::online::{Online, OnlineAction, OnlineMap, OnlinePhase};
#[derive(Default)]
pub(super) struct Menu {
    pub public: bool,
    pub map: OnlineMap,
    pub code: String,
    pub cursor: usize,
    pub editing: bool,
    pub error: String,
    pub leaving: bool,
    pub private_code: bool,
}
fn is_online(page: Page) -> bool {
    matches!(page, Page::Online | Page::Host | Page::Join | Page::Lobby)
}
pub(super) fn sync(front: &mut Frontend, online: &Online) {
    if !front.online_menu.leaving
        && matches!(
            online.phase,
            OnlinePhase::Connecting
                | OnlinePhase::Lobby
                | OnlinePhase::Countdown
                | OnlinePhase::Error
        )
        && front.flow.page != Page::Lobby
    {
        front.flow.page = Page::Lobby;
        front.selected = 0;
        front.online_menu.editing = false;
    }
}
fn disabled(text: impl Into<String>) -> Entry {
    Entry {
        text: text.into(),
        action: None,
    }
}
pub(super) fn entries(front: &Frontend, online: Option<&Online>) -> Option<Vec<Entry>> {
    use Action as A;
    let menu = &front.online_menu;
    Some(match front.flow.page {
        Page::Results if online.is_some_and(|o| o.phase == OnlinePhase::Results) => {
            vec![entry("LEAVE ONLINE RACE", A::Back)]
        }
        Page::Online => vec![
            entry("QUICK PLAY", A::QuickPlay),
            entry("HOST CUSTOM RACE", A::Host),
            entry("JOIN WITH CODE", A::Join),
            entry("BACK", A::Back),
        ],
        Page::Host => vec![
            entry(
                format!(
                    "VISIBILITY  /  {}",
                    if menu.public { "PUBLIC" } else { "PRIVATE" }
                ),
                A::TogglePublic,
            ),
            entry(format!("LEVEL  /  {}", menu.map.label()), A::NextMap),
            entry("CREATE LOBBY", A::CreateLobby),
            entry("BACK", A::Back),
        ],
        Page::Join => vec![
            entry(
                format!(
                    "CODE  /  {}{}",
                    grouped_code(&menu.code),
                    if menu.editing { " _" } else { "" }
                ),
                A::EditCode,
            ),
            entry(
                if menu.private_code {
                    "PRIVATE CODE / 36"
                } else {
                    "PUBLIC CODE / 10"
                },
                A::ToggleCodeType,
            ),
            entry("JOIN RACE", A::JoinLobby),
            entry("BACK", A::Back),
        ],
        Page::Lobby => {
            if let Some(lobby) = online
                .filter(|o| matches!(o.phase, OnlinePhase::Lobby | OnlinePhase::Countdown))
                .and_then(|o| o.lobby.as_ref())
            {
                vec![
                    if lobby.host && lobby.countdown.is_none() {
                        entry(format!("LEVEL  /  {}", lobby.map.label()), A::NextMap)
                    } else {
                        disabled(format!("LEVEL  /  {}", lobby.map.label()))
                    },
                    if lobby.host && lobby.countdown.is_none() {
                        entry("START RACE", A::StartOnline)
                    } else {
                        disabled(if lobby.countdown.is_some() {
                            "STARTING AUTOMATICALLY"
                        } else {
                            "WAITING FOR HOST"
                        })
                    },
                    entry("LEAVE LOBBY", A::Back),
                ]
            } else {
                vec![entry("CANCEL / BACK", A::Back)]
            }
        }
        _ => return None,
    })
}
fn grouped_code(code: &str) -> String {
    code.chars()
        .collect::<Vec<_>>()
        .chunks(6)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}
fn next_map(map: OnlineMap) -> OnlineMap {
    match map {
        OnlineMap::Metropolis => OnlineMap::Valley,
        OnlineMap::Valley => OnlineMap::City,
        OnlineMap::City => OnlineMap::Hills,
        OnlineMap::Hills => OnlineMap::Metropolis,
    }
}
pub(super) fn dispatch(front: &mut Frontend, online: Option<&mut Online>, action: Action) -> bool {
    if front.flow.page == Page::Results
        && online
            .as_ref()
            .is_some_and(|o| o.phase == OnlinePhase::Results)
    {
        if let Some(online) = online
            && matches!(action, Action::Back | Action::Ride)
        {
            online.actions.push_back(OnlineAction::Leave);
            front.flow.page = Page::Main;
        }
        return true;
    }
    if !is_online(front.flow.page) {
        return false;
    }
    let menu = &mut front.online_menu;
    if action == Action::Back && menu.editing {
        menu.editing = false;
        return true;
    }
    match action {
        Action::TogglePublic => {
            menu.public = !menu.public;
            return true;
        }
        Action::ToggleCodeType => {
            menu.private_code = !menu.private_code;
            menu.code.clear();
            menu.cursor = 0;
            return true;
        }
        Action::EditCode => {
            front.selected = 0;
            menu.editing = !menu.editing;
            return true;
        }
        Action::CodePrevious | Action::CodeNext | Action::CodeUp | Action::CodeDown => {
            edit_character(menu, action);
            return true;
        }
        _ => {}
    }
    let Some(online) = online else {
        menu.error = "Online service is unavailable in this build.".into();
        return action != Action::Back;
    };
    let command = match action {
        Action::NextMap if front.flow.page == Page::Host => {
            menu.map = next_map(menu.map);
            return true;
        }
        Action::NextMap => online
            .lobby
            .as_ref()
            .filter(|l| l.host && l.countdown.is_none() && online.phase == OnlinePhase::Lobby)
            .map(|l| OnlineAction::SetMap(next_map(l.map))),
        Action::CreateLobby => Some(OnlineAction::Host {
            public: menu.public,
            map: menu.map,
        }),
        Action::QuickPlay => Some(OnlineAction::QuickPlay),
        Action::JoinLobby => {
            if !matches!(menu.code.len(), 10 | 36) {
                menu.error = "Enter the public (10) or private (36) character code.".into();
                return true;
            }
            Some(OnlineAction::Join {
                code: menu.code.clone(),
            })
        }
        Action::StartOnline => online
            .lobby
            .as_ref()
            .filter(|l| l.host && l.countdown.is_none() && online.phase == OnlinePhase::Lobby)
            .map(|_| OnlineAction::Start),
        Action::Back => {
            menu.leaving = true;
            online.actions.push_back(OnlineAction::Leave);
            menu.error.clear();
            return false;
        }
        _ => return false,
    };
    if let Some(command) = command {
        if online.phase == OnlinePhase::Connecting {
            return true;
        }
        menu.leaving = false;
        online.actions.push_back(command);
        menu.error.clear();
        menu.editing = false;
        if matches!(
            action,
            Action::CreateLobby | Action::QuickPlay | Action::JoinLobby
        ) {
            front.flow.page = Page::Lobby;
            front.selected = 0;
        }
    }
    true
}
const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
fn edit_character(menu: &mut Menu, action: Action) {
    while menu.code.len() < if menu.private_code { 36 } else { 10 } {
        menu.code.push('A');
    }
    let length = if menu.private_code { 36 } else { 10 };
    menu.cursor = menu.cursor.min(length - 1);
    match action {
        Action::CodePrevious => menu.cursor = (menu.cursor + length - 1) % length,
        Action::CodeNext => menu.cursor = (menu.cursor + 1) % length,
        Action::CodeUp | Action::CodeDown => {
            let mut bytes = menu.code.as_bytes().to_vec();
            let index = CHARS
                .iter()
                .position(|c| *c == bytes[menu.cursor])
                .unwrap_or(0);
            bytes[menu.cursor] = CHARS[(index
                + if action == Action::CodeUp {
                    1
                } else {
                    CHARS.len() - 1
                })
                % CHARS.len()];
            menu.code = String::from_utf8(bytes).expect("ASCII code");
        }
        _ => {}
    }
}
pub(super) fn type_code(front: &mut Frontend, keys: &ButtonInput<KeyCode>) -> bool {
    if front.flow.page != Page::Join || !front.online_menu.editing {
        return false;
    }
    let menu = &mut front.online_menu;
    for (key, byte) in [
        KeyCode::KeyA,
        KeyCode::KeyB,
        KeyCode::KeyC,
        KeyCode::KeyD,
        KeyCode::KeyE,
        KeyCode::KeyF,
        KeyCode::KeyG,
        KeyCode::KeyH,
        KeyCode::KeyI,
        KeyCode::KeyJ,
        KeyCode::KeyK,
        KeyCode::KeyL,
        KeyCode::KeyM,
        KeyCode::KeyN,
        KeyCode::KeyO,
        KeyCode::KeyP,
        KeyCode::KeyQ,
        KeyCode::KeyR,
        KeyCode::KeyS,
        KeyCode::KeyT,
        KeyCode::KeyU,
        KeyCode::KeyV,
        KeyCode::KeyW,
        KeyCode::KeyX,
        KeyCode::KeyY,
        KeyCode::KeyZ,
        KeyCode::Digit0,
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ]
    .into_iter()
    .zip(CHARS)
    {
        if keys.just_pressed(key) && menu.code.len() < 36 {
            menu.code.push(*byte as char);
            menu.private_code = menu.code.len() > 10 || menu.private_code;
            menu.error.clear();
        }
    }
    if keys.just_pressed(KeyCode::Backspace) {
        menu.code.pop();
    }
    if keys.just_pressed(KeyCode::Delete) {
        menu.code.clear();
    }
    for (key, action) in [
        (KeyCode::ArrowLeft, Action::CodePrevious),
        (KeyCode::ArrowRight, Action::CodeNext),
        (KeyCode::ArrowUp, Action::CodeUp),
        (KeyCode::ArrowDown, Action::CodeDown),
    ] {
        if keys.just_pressed(key) {
            edit_character(menu, action);
        }
    }
    true
}
pub(super) fn signature(front: &Frontend, online: Option<&Online>) -> String {
    if front.online_pause {
        return format!(
            "online-race-menu:{}",
            online.map(|o| o.status.as_str()).unwrap_or_default()
        );
    }
    if !is_online(front.flow.page) && front.flow.page != Page::Results {
        return String::new();
    }
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}",
        front.online_menu.public,
        front.online_menu.private_code,
        front.online_menu.map.label(),
        front.online_menu.code,
        front.online_menu.editing,
        front.online_menu.cursor,
        online.map(|o| o.status.as_str()).unwrap_or_default(),
        details(front, online)
    )
}
fn details(front: &Frontend, online: Option<&Online>) -> String {
    let Some(online) = online else {
        return "Online service unavailable.".into();
    };
    if !front.online_menu.error.is_empty() {
        return front.online_menu.error.clone();
    }
    if let Some(error) = &online.error {
        return format!("CONNECTION FAILED\n{error}\nBack to retry or choose another race.");
    }
    match front.flow.page {
        Page::Results => "RACE COMPLETE\nLeave this session to host or join another race.".into(),
        Page::Online => format!("Public hosts • up to 250 ms.\nNo eligible lobby? Your PC hosts with AI.\n15-second countdown. Random starting grid.\n\nHOST ADDRESS / PORT  {}",online.endpoint),
        Page::Host => "Choose a level and visibility.\nPrivate races are accessible by code only.\nYou decide when your custom race starts.".into(),
        Page::Join => if front.online_menu.editing {format!("Type code • Backspace erases • Enter finishes\nController: D-pad changes letter / position; A finishes\nEditing character {} (public 10 / private 36)",front.online_menu.cursor + 1)} else {"Select CODE to enter the host’s public or private code.\nKeyboard, controller or mouse controls supported.".into()},
        Page::Lobby => {
            if let Some(lobby) = &online.lobby {
                let mut text = format!("CODE  {}     /     {}\n{} / 15 RIDERS",grouped_code(&lobby.code),if lobby.public {"PUBLIC"} else {"PRIVATE"}, lobby.players.len());
                if let Some(seconds) = lobby.countdown {text.push_str(&format!("     •     STARTS IN {}",seconds.ceil().max(0.) as u32));}
                text.push_str(&format!("\n{}  /  {}\n", lobby.map.label(), if lobby.host {"YOU ARE HOST"} else {"GUEST"}));
                for row in lobby.players.chunks(3) {
                    let names: Vec<_> = row.iter().map(|player| {
                        let name: String = player.name.chars().take(10).collect();
                        format!("{}{}{}", name, if player.name.chars().count() > 10 {"…"} else {""}, if player.bot {" AI"} else {""})
                    }).collect();
                    text.push_str(&names.join("  /  "));
                    text.push('\n');
                }
                text
            } else if online.phase == OnlinePhase::Error {"Disconnected. Return and try again.".into()} else {format!("CONNECTING…\n{}\nWaiting for the race service. Cancel returns safely.",online.endpoint)}
        }
        _ => String::new(),
    }
}
pub(super) fn render(
    commands: &mut Commands,
    canvas: Entity,
    front: &Frontend,
    online: Option<&Online>,
    race: &Race,
) -> bool {
    let Some(rows) = entries(front, online) else {
        return false;
    };
    let description = if front.flow.page == Page::Results {
        format!(
            "P{} / 15\nTIME {:02}:{:05.2}\nTAKEDOWNS {}\n\nLeave this session to host or join another race.",
            race.place,
            (race.elapsed / 60.) as u32,
            race.elapsed % 60.,
            race.takedowns
        )
    } else {
        details(front, online)
    };
    layout::render_online(
        commands,
        canvas,
        title(front.flow.page),
        &description,
        &rows,
    );
    render_status(commands, canvas, online);
    if front.flow.page == Page::Join && front.online_menu.editing {
        for (i, (label, action)) in [
            ("<", Action::CodePrevious),
            (">", Action::CodeNext),
            ("+", Action::CodeUp),
            ("-", Action::CodeDown),
        ]
        .into_iter()
        .enumerate()
        {
            layout::button(
                commands,
                canvas,
                90 + i,
                &entry(label, action),
                [6. + i as f32 * 10., 68., 8., 7.],
                false,
            );
        }
    }
    true
}

fn render_status(commands: &mut Commands, canvas: Entity, online: Option<&Online>) {
    if let Some(online) = online.filter(|o| !o.status.is_empty() && o.error.is_none()) {
        let label = text(
            commands,
            &online.status,
            18.,
            Node {
                position_type: PositionType::Absolute,
                left: percent(6.),
                top: percent(79.),
                width: percent(87.),
                height: percent(7.),
                overflow: Overflow::clip(),
                ..default()
            },
            layout::PAPER,
        );
        commands.entity(canvas).add_child(label);
    }
}
pub(super) fn render_pause(commands: &mut Commands, canvas: Entity, online: Option<&Online>) {
    let leaving = if online
        .and_then(|o| o.lobby.as_ref())
        .is_some_and(|l| l.host)
    {
        "Leaving as host ends the session for everyone."
    } else {
        "Leave to return to the main menu."
    };
    layout::render_online(
        commands,
        canvas,
        "RACE MENU",
        &format!(
            "The race continues while this menu is open.\nYour rider receives no driving input.\n\n{leaving}"
        ),
        &[
            entry("RESUME", Action::Continue),
            entry("LEAVE RACE", Action::Back),
        ],
    );
    render_status(commands, canvas, online);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relaunched_online_session_replaces_splash_and_exposes_connection_errors() {
        let mut front = Frontend::default();
        let mut online = Online {
            phase: OnlinePhase::Connecting,
            ..default()
        };
        sync(&mut front, &online);
        assert_eq!(front.flow.page, Page::Lobby);
        front.flow.page = Page::Splash;
        online.phase = OnlinePhase::Error;
        online.error = Some("Host unavailable".into());
        sync(&mut front, &online);
        assert_eq!(front.flow.page, Page::Lobby);
        front.online_menu.leaving = true;
        front.flow.page = Page::Main;
        sync(&mut front, &online);
        assert_eq!(front.flow.page, Page::Main);
    }
    #[test]
    fn create_and_join_queue_transport_without_releasing_local_race() {
        let mut front = Frontend::default();
        let mut online = Online::default();
        front.flow.page = Page::Host;
        dispatch(&mut front, Some(&mut online), Action::TogglePublic);
        dispatch(&mut front, Some(&mut online), Action::NextMap);
        dispatch(&mut front, Some(&mut online), Action::CreateLobby);
        assert!(matches!(
            online.actions.pop_front(),
            Some(OnlineAction::Host {
                public: true,
                map: OnlineMap::Valley
            })
        ));
        assert_eq!(front.flow.page, Page::Lobby);
        assert!(!front.released);
        front.flow.page = Page::Join;
        dispatch(&mut front, Some(&mut online), Action::JoinLobby);
        assert!(online.actions.is_empty());
        front.online_menu.code = "AAAQAAK3CY".into();
        dispatch(&mut front, Some(&mut online), Action::JoinLobby);
        assert!(
            matches!(online.actions.pop_front(),Some(OnlineAction::Join{code}) if code == "AAAQAAK3CY")
        );
        assert!(!front.released);
    }
    #[test]
    fn guest_cannot_start_or_change_host_map() {
        let mut front = Frontend::default();
        front.flow.page = Page::Lobby;
        let mut online = Online {
            phase: OnlinePhase::Lobby,
            lobby: Some(crate::online::LobbyView {
                code: "AAAQAAK3CY".into(),
                public: false,
                host: false,
                map: OnlineMap::City,
                players: vec![],
                countdown: None,
            }),
            ..default()
        };
        dispatch(&mut front, Some(&mut online), Action::StartOnline);
        dispatch(&mut front, Some(&mut online), Action::NextMap);
        assert!(online.actions.is_empty());
        assert!(entries(&front, Some(&online)).unwrap()[0].action.is_none());
        assert!(!dispatch(&mut front, Some(&mut online), Action::Back));
        assert!(matches!(
            online.actions.pop_front(),
            Some(OnlineAction::Leave)
        ));
    }
    #[test]
    fn private_code_controller_covers_all_capability_characters() {
        let mut menu = Menu {
            private_code: true,
            ..default()
        };
        edit_character(&mut menu, Action::CodePrevious);
        assert_eq!(menu.cursor, 35);
        edit_character(&mut menu, Action::CodeUp);
        assert_eq!(menu.code.len(), 36);
        assert!(menu.code.ends_with('B'));
        edit_character(&mut menu, Action::CodeNext);
        assert_eq!(menu.cursor, 0);
    }
    #[test]
    fn code_controller_edits_and_keyboard_edges_are_bounded() {
        let mut front = Frontend::default();
        front.flow.page = Page::Join;
        front.online_menu.editing = true;
        let mut keys = ButtonInput::default();
        keys.press(KeyCode::KeyA);
        type_code(&mut front, &keys);
        assert_eq!(front.online_menu.code, "A");
        keys.clear();
        type_code(&mut front, &keys);
        assert_eq!(front.online_menu.code, "A");
        edit_character(&mut front.online_menu, Action::CodePrevious);
        assert_eq!(front.online_menu.cursor, 9);
        edit_character(&mut front.online_menu, Action::CodeUp);
        assert_eq!(front.online_menu.code, "AAAAAAAAAB");
    }
}
