//! Opt-in visual fixture, not transport evidence. Run --frontend-smoke with
//! ROADDASH_VERIFY_ONLINE_UI=1; screenshots may contain external user media.
use super::*;
use crate::online::{LobbyPlayer, LobbyView, Online, OnlinePhase};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
struct Check {
    frame: u32,
    stage: usize,
}
pub(super) fn install(app: &mut App) {
    app.init_resource::<Check>()
        .init_resource::<Online>()
        .add_systems(Update, run.after(super::resize));
}
#[allow(clippy::too_many_arguments)]
fn run(
    mut check: ResMut<Check>,
    mut front: ResMut<Frontend>,
    mut online: ResMut<Online>,
    mut race: ResMut<Race>,
    state: Res<State<AppScreen>>,
    server: Res<AssetServer>,
    art: Res<Art>,
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    mut exit: MessageWriter<AppExit>,
) {
    if !ready(Some(&state), Some(&server), Some(&art)) {
        return;
    }
    check.frame += 1;
    if check.frame == 1 {
        front.flow.page = match check.stage {
            0 => Page::Main,
            1 => Page::Online,
            2 => Page::Host,
            3 => Page::Join,
            _ => Page::Lobby,
        };
        front.selected = 0;
        online.phase = OnlinePhase::Offline;
        online.status =
            "Discovery: public hosts / LAN and known peers. Measured ping: 42 ms.".into();
        online.error = None;
        online.lobby = None;
        if check.stage == 3 {
            front.online_menu.code = "AAAQAAK3CYABCDEFGHIJKLMNOPQRSTUVWXYZ".into();
            front.online_menu.private_code = true;
            front.online_menu.editing = true;
        }
        if check.stage == 4 {
            online.phase = OnlinePhase::Connecting;
        }
        if (5..=7).contains(&check.stage) {
            online.phase = OnlinePhase::Lobby;
            online.lobby = Some(LobbyView {
                code: if check.stage == 5 {
                    "AAAQAAK3CYABCDEFGHIJKLMNOPQRSTUVWXYZ"
                } else {
                    "AAAQAAK3CY"
                }
                .into(),
                public: check.stage == 7,
                host: check.stage != 6,
                map: crate::online::OnlineMap::Metropolis,
                players: (0..15)
                    .map(|slot| LobbyPlayer {
                        slot,
                        name: format!("Rider {}", slot + 1),
                        bot: slot > 1,
                    })
                    .collect(),
                countdown: (check.stage == 7).then_some(15.),
            });
        }
        if check.stage == 8 {
            online.phase = OnlinePhase::Error;
            online.error = Some("Host disconnected. Your saved progress is safe.".into());
        }
        if check.stage == 10 {
            online.phase = OnlinePhase::Racing;
            online.lobby = Some(LobbyView {
                code: "AAAQAAK3CY".into(),
                public: true,
                host: true,
                map: crate::online::OnlineMap::Metropolis,
                players: vec![],
                countdown: None,
            });
            race.phase = Phase::Racing;
            front.online_pause = true;
        }
        if check.stage == 9 {
            windows.single_mut().unwrap().resolution.set(1280., 720.);
            front.flow.page = Page::Join;
            front.online_menu.editing = true;
        }
    }
    if check.frame == 25 {
        let dir = std::env::var("ROADDASH_UI_CAPTURE_DIR")
            .unwrap_or_else(|_| "/tmp/roaddash-online-ui".into());
        std::fs::create_dir_all(&dir).unwrap();
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "{dir}/online-ui-{:02}.png",
                check.stage
            )));
    }
    if check.frame == 35 {
        check.stage += 1;
        check.frame = 0;
        if check.stage == 11 {
            info!("Online UI visual fixture complete (11 states; not transport evidence)");
            exit.write(AppExit::Success);
        }
    }
}
