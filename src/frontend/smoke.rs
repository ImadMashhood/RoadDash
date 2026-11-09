//! Reproducible native UI flow/screenshots. Never writes a player profile.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
struct Smoke {
    stage: usize,
    portraits_checked: usize,
    frames: u32,
    wait: u32,
    viewports: Vec<[u32; 2]>,
    layouts: Vec<serde_json::Value>,
}
pub(crate) fn install(app: &mut App) {
    app.init_resource::<Smoke>()
        .add_systems(Update, run.after(super::resize))
        .add_systems(PostUpdate, layout.after(bevy::ui::UiSystems::PostLayout));
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn run(
    mut s: ResMut<Smoke>,
    mut front: ResMut<Frontend>,
    mut race: ResMut<Race>,
    mut panel: ResMut<SettingsPanel>,
    mut controls: ResMut<Controls>,
    server: Res<AssetServer>,
    art: Res<Art>,
    state: Res<State<AppScreen>>,
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    mut exit: MessageWriter<AppExit>,
    profile: Res<Profile>,
    clock: Res<crate::runtime::SimulationClock>,
    grid: Res<crate::weapons::GridWeapons>,
    riders: Query<&crate::game::Rider>,
    actors: (
        Query<(&crate::weapons::Held, &Visibility)>,
        Query<(&MenuButton, &ComputedNode)>,
        Query<(&Text, &ComputedNode, &UiGlobalTransform)>,
    ),
) {
    let (held, cards, labels) = actors;
    s.frames += 1;
    assert!(s.frames < 20000, "frontend smoke timed out");
    if !ready(Some(&state), Some(&server), Some(&art)) {
        return;
    }
    s.wait += 1;
    if s.wait < 8 {
        return;
    }
    let expected = match s.stage {
        0 => Page::Splash,
        1 => Page::Main,
        2 => Page::Rider,
        3 | 5 | 7 | 8 | 9 => Page::Street,
        4 => Page::Garage,
        6 => Page::Bar,
        10 => Page::Main,
        11..=15 => Page::Courses,
        16..=18 => Page::Starting,
        19 => Page::Results,
        20 => Page::Main,
        21 => Page::Quit,
        _ => unreachable!(),
    };
    if s.stage == 6 {
        assert!(matches!(front.flow.page, Page::Bar | Page::Portrait));
    } else {
        assert_eq!(front.flow.page, expected, "stage {}", s.stage);
    }
    let name = match s.stage {
        0 => Some("01-splash"),
        1 => Some("02-main"),
        2 => Some("03-rider"),
        3 => Some("04-street"),
        4 => Some("05-shop"),
        6 => Some("06-bar"),
        8 => Some("07-settings"),
        11 => Some("08-course-1440x900"),
        12 => Some("09-course-1280x720"),
        13 => Some("10-course-2560x1440"),
        14 => Some("11-course-ultrawide"),
        15 => Some("12-course-4k"),
        16 => Some("13-loading"),
        17 => Some("14-pause"),
        19 => Some("15-results"),
        21 => Some("16-quit"),
        _ => None,
    };
    if s.wait == 8
        && let Some(name) = name
    {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/frontend-{name}.png")));
    }
    if s.wait == 8 && (11..=15).contains(&s.stage) {
        let window = windows.single().unwrap();
        s.viewports.push([
            window.resolution.physical_width(),
            window.resolution.physical_height(),
        ]);
    }
    if s.wait < 20 {
        return;
    }
    if s.stage == 6 && s.portraits_checked < ROSTER.len() {
        if front.flow.page == Page::Bar {
            let items: Vec<_> = cards
                .iter()
                .filter(|(b, _)| b.index < ROSTER.len())
                .collect();
            assert_eq!(items.len(), 16);
            let window = windows.single().unwrap();
            for (_, node) in items {
                assert!(
                    node.size().x > 0.0
                        && node.size().x < window.resolution.physical_width() as f32 / 3.0
                );
                assert!(
                    node.size().y > 0.0
                        && node.size().y < window.resolution.physical_height() as f32 / 3.0
                );
            }
            front.pending = Some(Action::SelectPortrait(s.portraits_checked));
        } else {
            assert_eq!(front.flow.portrait, s.portraits_checked);
            let window = windows.single().unwrap();
            let (_, node, transform) = labels
                .iter()
                .find(|(label, _, _)| label.0 == ROSTER[s.portraits_checked].bio)
                .expect("full character biography must be rendered");
            let last_line = transform.translation.y + node.size().y * 0.5;
            assert!(
                node.size().y > 0.0
                    && last_line < window.resolution.physical_height() as f32 * 0.93,
                "biography clipped or overlaps bottom controls"
            );
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!(
                    "screenshots/frontend-portrait-{:02}.png",
                    s.portraits_checked + 1
                )));
            s.portraits_checked += 1;
            front.pending = Some(Action::Back);
        }
        s.wait = 0;
        return;
    }
    if s.stage == 6 {
        assert_eq!(front.flow.page, Page::Bar);
        info!("Approved portrait picker: all 16 cards opened and returned to grid");
    }
    match s.stage {
        0 => {
            assert_eq!(clock.active_ticks, 0);
            front.pending = Some(Action::Continue);
        }
        1 => front.pending = Some(Action::Street),
        2 => front.pending = Some(Action::Continue),
        3 => front.pending = Some(Action::Garage),
        4 => front.pending = Some(Action::Back),
        6 => {
            windows.single_mut().unwrap().resolution.set(1440., 900.);
            front.pending = Some(Action::Back);
        }
        5 => {
            windows.single_mut().unwrap().resolution.set(1280., 720.);
            front.pending = Some(Action::Bar);
        }
        7 => front.pending = Some(Action::Settings),
        8 => {
            assert!(panel.open);
            panel.open = false;
        }
        9 => front.pending = Some(Action::Back),
        10 => front.pending = Some(Action::Thrash),
        11 => windows.single_mut().unwrap().resolution.set(1280., 720.),
        12 => windows.single_mut().unwrap().resolution.set(2560., 1440.),
        13 => windows.single_mut().unwrap().resolution.set(3440., 1440.),
        14 => windows.single_mut().unwrap().resolution.set(3840., 2160.),
        15 => {
            assert_eq!(clock.active_ticks, 0);
            windows.single_mut().unwrap().resolution.set(1440., 900.);
            front.pending = Some(Action::Ride);
        }
        16 => {
            if race.phase == Phase::Garage {
                return;
            }
            assert!(matches!(race.phase, Phase::Countdown | Phase::Racing));
            let equipped: Vec<_> = riders.iter().filter(|r| r.weapon.is_some()).collect();
            assert_eq!(equipped.len(), 7, "default 14-rival grid must arm half");
            assert!(
                equipped
                    .iter()
                    .all(|r| r.id != 0 && !r.police && grid.armed.contains(&r.id))
            );
            assert_eq!(
                held.iter()
                    .filter(|(_, v)| **v == Visibility::Inherited)
                    .count(),
                7,
                "all starting clubs must be rendered"
            );
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("screenshots/frontend-13b-race.png"));
            controls.pause = true;
            race.phase = Phase::Racing;
        }
        17 => {
            assert_eq!(race.phase, Phase::Paused);
            controls.pause = true;
        }
        18 => {
            assert_eq!(race.phase, Phase::Racing);
            race.phase = Phase::Finished;
        }
        19 => front.pending = Some(Action::Back),
        20 => {
            assert_eq!(race.phase, Phase::Garage);
            front.pending = Some(Action::Quit);
        }
        21 => {
            let report = serde_json::json!({"stages":22,"images":23,"portraits_checked":s.portraits_checked,"portrait_grid":[4,4],"active_ticks":clock.active_ticks,"profile_credits":profile.credits,"ended_in_garage":race.phase==Phase::Garage,"actual_viewports":s.viewports,"layouts":s.layouts,"race_seed":grid.seed,"armed_rivals":grid.armed,"limitations":"Scripted native flow and rendered screenshots; not human controller/art approval or performance certification"});
            std::fs::create_dir_all("screenshots").unwrap();
            std::fs::write(
                "screenshots/frontend-report.json",
                serde_json::to_string_pretty(&report).unwrap(),
            )
            .unwrap();
            info!(
                "Frontend smoke passed: 22 stages, native race, pause, results and five viewports"
            );
            exit.write(AppExit::Success);
        }
        _ => unreachable!(),
    }
    s.stage += 1;
    s.wait = 0;
}

fn layout(
    mut s: ResMut<Smoke>,
    window: Query<&Window>,
    canvas: Query<(&ComputedNode, &UiGlobalTransform), With<Canvas>>,
    roots: Query<&ComputedNode, With<Root>>,
    buttons: Query<(&ComputedNode, &UiGlobalTransform), With<MenuButton>>,
    cameras: Query<&Camera, With<UiCamera>>,
) {
    if s.wait != 8 || !(11..=15).contains(&s.stage) {
        return;
    }
    let w = window.single().unwrap();
    let (node, transform) = canvas.single().unwrap();
    let size = w.resolution.physical_size().as_vec2();
    let row = serde_json::json!({"stage":s.stage,"window":size.to_array(),"root":roots.single().unwrap().size().to_array(),"canvas":node.size().to_array(),"center":transform.translation.to_array(),"camera":cameras.single().unwrap().physical_viewport_size().map(|v|v.to_array())});
    s.layouts.push(row.clone());
    for (b, t) in &buttons {
        let min = t.translation - b.size() * 0.5;
        let max = t.translation + b.size() * 0.5;
        assert!(
            min.x >= -1. && min.y >= -1. && max.x <= size.x + 1. && max.y <= size.y + 1.,
            "button clipped: {row} min {min:?} max {max:?}"
        );
    }
}
