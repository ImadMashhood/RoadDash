//! Native art-led offline entry screens. Simulation and profile contracts stay in game.rs.
use crate::{
    controls::Controls,
    game::{Phase, Profile, Race},
    runtime::AppScreen,
    settings::SettingsPanel,
    track::Track,
};
use bevy::{asset::LoadState, prelude::*};
use rr_core::frontend::{Action, Effect, Flow, Page, ROSTER};
mod gallery;
mod smoke;
pub(crate) use smoke::install as install_smoke;

#[derive(Resource, Default)]
pub struct Frontend {
    pub flow: Flow,
    pub selected: usize,
    pub pending: Option<Action>,
    launch_wait: f32,
    pub(crate) released: bool,
    race_seen: bool,
}
impl Frontend {
    pub fn owns(&self, phase: Phase) -> bool {
        matches!(phase, Phase::Garage | Phase::Finished) && !self.released
    }
}
const ART: [&str; 7] = [
    "ui/splash.png",
    "ui/main.png",
    "ui/rider.png",
    "ui/street.png",
    "ui/course.png",
    "ui/garage.png",
    "ui/bar.png",
];
#[derive(Resource)]
struct Art {
    images: Vec<Handle<Image>>,
    heading_font: Handle<Font>,
    body_font: Handle<Font>,
}
#[derive(Component)]
struct Root;
#[derive(Component)]
struct Canvas;
#[derive(Component)]
struct UiCamera;
#[derive(Component)]
struct MenuButton {
    index: usize,
    action: Action,
}
#[derive(Component)]
struct FontSize(f32);
#[derive(Resource, Default)]
struct View {
    key: Option<(Page, usize, bool, bool, bool)>,
}
#[derive(Clone)]
struct Entry {
    text: String,
    action: Option<Action>,
}
fn entry(text: impl Into<String>, action: Action) -> Entry {
    Entry {
        text: text.into(),
        action: Some(action),
    }
}
fn entries(page: Page) -> Vec<Entry> {
    use Action as A;
    match page {
        Page::Splash => vec![entry("HIT THE ROAD", A::Continue)],
        Page::Main => vec![
            entry("THRASH", A::Thrash),
            entry("BIG GAME / STREET", A::Street),
            Entry {
                text: "MANO A MANO\nOFFLINE ONLY".into(),
                action: None,
            },
            entry("OPTIONS", A::Settings),
            entry("QUIT", A::Quit),
        ],
        Page::Rider => vec![entry("RIDE AS YOU", A::Continue), entry("BACK", A::Back)],
        Page::Street => vec![
            entry("RACE", A::Courses),
            entry("BIKE SHOP", A::Garage),
            entry("BIKER BAR", A::Bar),
            entry("OPTIONS", A::Settings),
            entry("MAIN MENU", A::Back),
        ],
        Page::Courses => vec![entry("RIDE THIS COURSE", A::Ride), entry("BACK", A::Back)],
        Page::Garage => vec![entry("BACK TO THE STREET", A::Back)],
        Page::Bar => ROSTER
            .iter()
            .enumerate()
            .map(|(i, rider)| entry(rider.name, A::SelectPortrait(i)))
            .chain(std::iter::once(entry("BACK TO THE STREET", A::Back)))
            .collect(),
        Page::Portrait => vec![entry("BACK TO RIDERS", A::Back)],
        Page::Results => vec![entry("RACE AGAIN", A::Ride), entry("MAIN MENU", A::Back)],
        Page::Quit => vec![
            entry("KEEP RIDING", A::Back),
            entry("QUIT GAME", A::ConfirmQuit),
        ],
        Page::Starting => vec![],
    }
}
fn ready(
    state: Option<&State<AppScreen>>,
    server: Option<&AssetServer>,
    art: Option<&Art>,
) -> bool {
    state.is_none_or(|s| !matches!(s.get(), AppScreen::Boot | AppScreen::Loading))
        && art.is_none_or(|a| {
            server.is_some_and(|s| {
                a.images
                    .iter()
                    .all(|h| s.is_loaded_with_dependencies(h.id()))
                    && s.is_loaded_with_dependencies(a.heading_font.id())
                    && s.is_loaded_with_dependencies(a.body_font.id())
            })
        })
}

/// After input capture, before game::input. Each button edge has exactly one owner.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn input(
    mut front: ResMut<Frontend>,
    mut controls: ResMut<Controls>,
    mut race: ResMut<Race>,
    mut panel: ResMut<SettingsPanel>,
    state: Option<Res<State<AppScreen>>>,
    keys: Res<ButtonInput<KeyCode>>,
    pads: Query<(Entity, &Gamepad)>,
    clicks: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    time: Res<Time<Real>>,
    mut menu: ResMut<crate::ui::MenuAction>,
    mut exit: MessageWriter<AppExit>,
    server: Option<Res<AssetServer>>,
    art: Option<Res<Art>>,
) {
    if !matches!(race.phase, Phase::Garage | Phase::Finished) {
        front.released = false;
        front.race_seen = true;
        return;
    }
    if race.phase == Phase::Finished
        && (front.race_seen || !matches!(front.flow.page, Page::Results | Page::Starting))
    {
        front.flow.page = Page::Results;
        front.selected = 0;
        front.race_seen = false;
    }
    let start = std::mem::take(&mut controls.start);
    let back = std::mem::take(&mut controls.pause);
    controls.restart = false;
    controls.camera = false;
    controls.buffer.clear_edges();
    controls.pickup = false;
    controls.drop = false;
    menu.0 = false;
    if panel.open || controls.suppress_frame {
        front.pending = None;
        return;
    }
    if !ready(state.as_deref(), server.as_deref(), art.as_deref()) {
        front.pending = None;
        return;
    }
    let pad = pads.iter().min_by_key(|(e, _)| e.to_bits()).map(|(_, p)| p);
    let edge = |b| pad.is_some_and(|p| p.just_pressed(b));
    if front.flow.page == Page::Starting {
        front.launch_wait += time.delta_secs();
        // One rendered loading page and a fresh input boundary, not arbitrary physics work.
        if front.launch_wait >= 0.2 {
            front.released = true;
            menu.0 = true;
        }
        return;
    }
    let rows = entries(front.flow.page);
    let direction = if keys.just_pressed(KeyCode::ArrowRight)
        || keys.just_pressed(KeyCode::ArrowDown)
        || edge(GamepadButton::DPadRight)
        || edge(GamepadButton::DPadDown)
    {
        1
    } else if keys.just_pressed(KeyCode::ArrowLeft)
        || keys.just_pressed(KeyCode::ArrowUp)
        || edge(GamepadButton::DPadLeft)
        || edge(GamepadButton::DPadUp)
    {
        -1
    } else {
        0
    };
    let direction: isize = if front.flow.page == Page::Bar
        && direction != 0
        && (keys.just_pressed(KeyCode::ArrowUp)
            || keys.just_pressed(KeyCode::ArrowDown)
            || edge(GamepadButton::DPadUp)
            || edge(GamepadButton::DPadDown))
    {
        direction * 4
    } else {
        direction
    };
    if front.flow.page == Page::Bar && direction.abs() == 4 {
        front.selected = match (front.selected, direction > 0) {
            (16, true) => 0,
            (16, false) => 12,
            (i, true) if i >= 12 => 16,
            (i, false) if i < 4 => i + 12,
            (i, _) => (i as isize + direction) as usize,
        };
    } else if direction != 0 {
        for _ in 0..rows.len() {
            front.selected =
                (front.selected as isize + direction).rem_euclid(rows.len() as isize) as usize;
            if rows[front.selected].action.is_some() {
                break;
            }
        }
    }
    let click = clicks
        .iter()
        .find_map(|(i, b)| (*i == Interaction::Pressed).then_some(b.action));
    let action = front.pending.take().or(click).or_else(|| {
        if back || edge(GamepadButton::East) {
            Some(Action::Back)
        } else if start {
            rows.get(front.selected).and_then(|e| e.action)
        } else {
            None
        }
    });
    if let Some(action) = action {
        let was = front.flow.page;
        match front.flow.dispatch(action) {
            Effect::Settings => panel.open = true,
            Effect::Start => front.launch_wait = 0.0,
            Effect::Exit => {
                exit.write(AppExit::Success);
            }
            Effect::None => {}
        }
        if was != front.flow.page {
            front.selected = if front.flow.page == Page::Bar {
                front.flow.portrait
            } else {
                0
            };
        }
        if was == Page::Results && front.flow.page == Page::Main {
            race.phase = Phase::Garage;
        }
    }
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        UiCamera,
        IsDefaultUiCamera,
        Camera {
            order: 100,
            clear_color: bevy::camera::ClearColorConfig::None,
            ..default()
        },
    ));
    commands.insert_resource(Art {
        heading_font: server.load("ui/fonts/OpenSans-ExtraBold.ttf"),
        body_font: server.load("ui/fonts/OpenSans-Regular.ttf"),
        images: ART
            .iter()
            .map(|p| server.load(*p))
            .chain(
                ROSTER
                    .iter()
                    .map(|p| server.load(format!("ui/portraits/{}.png", p.id))),
            )
            .collect(),
    });
    commands
        .spawn((
            Root,
            Node {
                width: percent(100),
                height: percent(100),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            GlobalZIndex(20),
        ))
        .with_children(|p| {
            p.spawn((
                Canvas,
                Node {
                    width: px(1440),
                    height: px(810),
                    overflow: Overflow::clip(),
                    ..default()
                },
            ));
        });
}
fn text(
    commands: &mut Commands,
    value: impl Into<String>,
    size: f32,
    node: Node,
    color: Color,
) -> Entity {
    commands
        .spawn((
            Text::new(value),
            TextFont {
                font_size: size,
                ..default()
            },
            TextColor(color),
            FontSize(size),
            TextShadow {
                offset: Vec2::splat(2.0),
                color: Color::BLACK,
            },
            node,
        ))
        .id()
}
fn title(page: Page) -> &'static str {
    match page {
        Page::Splash | Page::Main => "ROAD / DASH",
        Page::Rider => "MEET YOUR RIDER",
        Page::Street => "THE STREET",
        Page::Courses => "PICK YOUR ROAD",
        Page::Garage => "THE BIKE SHOP",
        Page::Bar => "PICK A RIDER",
        Page::Portrait => "MEET THE RIDER",
        Page::Starting => "GET READY TO RIDE",
        Page::Results => "END OF THE ROAD",
        Page::Quit => "LEAVING ALREADY?",
    }
}
fn picture(page: Page) -> usize {
    match page {
        Page::Splash => 0,
        Page::Main | Page::Quit => 1,
        Page::Rider => 2,
        Page::Street => 3,
        Page::Courses | Page::Starting | Page::Results => 4,
        Page::Garage => 5,
        Page::Bar | Page::Portrait => 6,
    }
}
fn body(page: Page, profile: &Profile, track: &Track, race: &Race) -> String {
    let course_name = track.definition.name.replace('—', "/").replace('–', "-");
    match page {
        Page::Splash => "Fifteen riders. One road.\nNo polite company.".into(),
        Page::Main => format!("THRASH: straight into a race.\nBIG GAME: visit the street first.\n\n{} CREDITS  /  {} WINS\nOffline single races. Campaign coming later.", profile.credits, profile.wins),
        Page::Rider => "YOU\nBlue helmet. Bad intentions.\n\nYour current rider and motorcycle.\nOther playable profiles coming later.".into(),
        Page::Street => format!("{}\n{} CREDITS\nChoose your road, or look around.", course_name, profile.credits),
        Page::Courses => format!("{}\n\n{:.2} KM  /  {}\n15 RIDERS\n\nFarm roads. City traffic. Rival racers.\nRide the current course.", course_name, track.length/1000.0, if track.definition.closed {"CIRCUIT"} else {"POINT TO POINT"}),
        Page::Garage => "YOUR ROAD BIKE\nReady for another beating.\n\nBike purchases and upgrades coming later.\nYour credits and bike are unchanged.".into(),
        Page::Portrait => "RIDER PORTRAITS".into(),
        Page::Bar => "THE LOCALS DON'T LIKE YOU.\n\nWatch the aggressive riders.\nA clean pass is safer than a fight.\n\nConversations coming later.".into(),
        Page::Starting => format!("{}\n\nW / S  THROTTLE / BRAKE\nA / D  STEER\nMOUSE  PUNCH  /  SPACE  KICK\nESC  PAUSE\n\nRIDE OUT...",course_name),
        Page::Results if race.arrested => "BUSTED\n\nNo payout.\nTry your luck again.".into(),
        Page::Results => format!("P{} / 15\nTIME {:02}:{:05.2}\nTAKEDOWNS {}\n\n+{} CREDITS\nBALANCE {}",race.place,(race.elapsed/60.0) as u32,race.elapsed%60.0,race.takedowns,race.earned,profile.credits),
        Page::Quit => "Quit RoadDash?\nCompleted race progress is saved automatically.".into(),
    }
}

#[allow(clippy::too_many_arguments)]
fn view(
    mut commands: Commands,
    front: Res<Frontend>,
    race: Res<Race>,
    profile: Res<Profile>,
    track: Res<Track>,
    state: Res<State<AppScreen>>,
    art: Res<Art>,
    server: Res<AssetServer>,
    mut cache: ResMut<View>,
    mut root: Query<&mut Node, With<Root>>,
    canvas: Query<(Entity, Option<&Children>), With<Canvas>>,
) {
    let active = matches!(race.phase, Phase::Garage | Phase::Finished);
    let mut root = root.single_mut().expect("frontend root");
    root.display = if active { Display::Flex } else { Display::None };
    let loading = matches!(state.get(), AppScreen::Boot | AppScreen::Loading)
        || !art
            .images
            .iter()
            .all(|h| server.is_loaded_with_dependencies(h.id()))
        || !server.is_loaded_with_dependencies(art.heading_font.id())
        || !server.is_loaded_with_dependencies(art.body_font.id());
    let failed = art
        .images
        .iter()
        .any(|h| matches!(server.get_load_state(h.id()), Some(LoadState::Failed(_))))
        || matches!(
            server.get_load_state(art.heading_font.id()),
            Some(LoadState::Failed(_))
        )
        || matches!(
            server.get_load_state(art.body_font.id()),
            Some(LoadState::Failed(_))
        );
    let key = (
        front.flow.page,
        front.flow.portrait,
        loading,
        active,
        failed,
    );
    if cache.key == Some(key) {
        return;
    }
    cache.key = Some(key);
    let (canvas, children) = canvas.single().expect("frontend canvas");
    if let Some(children) = children {
        for e in children {
            commands.entity(*e).despawn();
        }
    }
    let page = front.flow.page;
    let image = commands
        .spawn((
            ImageNode::new(
                art.images[if page == Page::Portrait {
                    ART.len() + front.flow.portrait
                } else {
                    picture(page)
                }]
                .clone(),
            ),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                ..default()
            },
        ))
        .id();
    commands.entity(canvas).add_child(image);
    if !loading && !failed {
        if page == Page::Portrait {
            gallery::detail(&mut commands, canvas, front.flow.portrait);
            return;
        }
        if page == Page::Bar {
            gallery::render(&mut commands, canvas, &art.images[ART.len()..]);
            return;
        }
    }
    let heading = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: percent(3),
                top: percent(3),
                padding: UiRect::all(px(10)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.015, 0.025, 0.88)),
        ))
        .id();
    let label = text(
        &mut commands,
        title(page),
        47.0,
        Node::default(),
        Color::srgb(0.96, 0.85, 0.65),
    );
    commands.entity(heading).add_child(label);
    commands.entity(canvas).add_child(heading);
    if page == Page::Bar && !loading && !failed {
        gallery::render(&mut commands, canvas, &art.images[ART.len()..]);
        return;
    }
    let wide = matches!(page, Page::Street | Page::Garage | Page::Splash);
    let panel = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: percent(if wide {
                    3.0
                } else if page == Page::Portrait {
                    56.0
                } else {
                    66.0
                }),
                top: percent(if wide { 56.0 } else { 18.0 }),
                width: percent(if wide {
                    61.0
                } else if page == Page::Portrait {
                    41.0
                } else {
                    31.0
                }),
                max_height: percent(if wide { 24.0 } else { 59.0 }),
                padding: UiRect::all(px(14)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.035, 0.018, 0.055, 0.9)),
        ))
        .id();
    let value = if failed {
        "ARTWORK COULD NOT LOAD.\nCheck assets/ui in your installation.\nClose and relaunch after restoring it.".into()
    } else if loading {
        "PREPARING THE ROAD...\nPlease wait. Your race has not started.".into()
    } else if page == Page::Portrait {
        let rider = &ROSTER[front.flow.portrait];
        format!(
            "{:02} / {}\n\n{}\n\n{}",
            front.flow.portrait + 1,
            ROSTER.len(),
            rider.name,
            rider.bio
        )
    } else {
        body(page, &profile, &track, &race)
    };
    let label = text(
        &mut commands,
        value,
        if wide { 20.0 } else { 22.0 },
        Node::default(),
        Color::srgb(0.96, 0.90, 0.77),
    );
    commands.entity(panel).add_child(label);
    commands.entity(canvas).add_child(panel);
    let strip = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: percent(3),
                left: percent(3),
                width: percent(94),
                height: percent(12),
                column_gap: px(4),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.018, 0.04, 0.95)),
        ))
        .id();
    for (index, entry) in entries(page).into_iter().enumerate() {
        let button = commands
            .spawn((
                Node {
                    flex_grow: 1.0,
                    flex_basis: px(0),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(px(2)),
                    padding: UiRect::all(px(7)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.04, 0.07)),
                BorderColor::all(Color::srgb(0.33, 0.18, 0.15)),
            ))
            .id();
        if let Some(action) = entry.action.filter(|_| !loading) {
            commands
                .entity(button)
                .insert((Button, MenuButton { index, action }));
        }
        let label = text(
            &mut commands,
            entry.text,
            22.0,
            Node::default(),
            if entry.action.is_some() && !loading {
                Color::srgb(0.96, 0.88, 0.7)
            } else {
                Color::srgb(0.47, 0.42, 0.44)
            },
        );
        commands.entity(button).add_child(label);
        commands.entity(strip).add_child(button);
    }
    commands.entity(canvas).add_child(strip);
    let hint = text(
        &mut commands,
        "ARROWS / D-PAD: CHOOSE     ENTER / SOUTH: SELECT     ESC / EAST: BACK     F2: OPTIONS",
        14.0,
        Node {
            position_type: PositionType::Absolute,
            left: percent(3),
            bottom: percent(0.5),
            ..default()
        },
        Color::srgb(0.87, 0.77, 0.67),
    );
    commands.entity(canvas).add_child(hint);
}
fn focus(
    front: Res<Frontend>,
    mut buttons: Query<(
        &MenuButton,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    for (b, i, mut bg, mut border) in &mut buttons {
        let selected = b.index == front.selected || *i == Interaction::Hovered;
        bg.set_if_neq(BackgroundColor(if selected {
            Color::srgb(0.46, 0.035, 0.065)
        } else {
            Color::srgb(0.12, 0.04, 0.07)
        }));
        border.set_if_neq(BorderColor::all(if selected {
            Color::srgb(0.96, 0.66, 0.24)
        } else {
            Color::srgb(0.33, 0.18, 0.15)
        }));
    }
}
fn resize(
    windows: Query<&Window>,
    front: Res<Frontend>,
    mut canvas: Query<&mut Node, With<Canvas>>,
    mut fonts: Query<(&FontSize, &mut TextFont)>,
    mut cameras: Query<(&Camera, &mut Projection), With<UiCamera>>,
) {
    let Ok(window) = windows.single() else { return };
    // Bevy 0.17 refreshes camera targets on native resize messages or projection
    // changes. A programmatic Window resolution change can precede that message.
    // Refresh the projection when sizes disagree, retaining the full automatic
    // viewport so render-surface resize does not leave an oversized scissor rect.
    for (camera, mut projection) in &mut cameras {
        if camera.physical_target_size() != Some(window.resolution.physical_size()) {
            projection.set_changed();
        }
    }
    let aspect = if matches!(front.flow.page, Page::Bar | Page::Portrait) {
        4.0 / 3.0
    } else {
        16.0 / 9.0
    };
    let width = window.width().min(window.height() * aspect);
    let height = width / aspect;
    let Ok(mut node) = canvas.single_mut() else {
        return;
    };
    node.width = px(width);
    node.height = px(height);
    for (base, mut font) in &mut fonts {
        font.font_size = base.0 * (width / 1440.0);
    }
}
fn typography(art: Res<Art>, mut labels: Query<(&FontSize, &mut TextFont)>) {
    for (base, mut font) in &mut labels {
        let face = if base.0 >= 30.0 {
            &art.heading_font
        } else {
            &art.body_font
        };
        if font.font != *face {
            font.font = face.clone();
        }
    }
}
pub struct FrontendPlugin;
impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Frontend>()
            .init_resource::<View>()
            .add_systems(Startup, setup.in_set(crate::runtime::StartupSet::Ui))
            .add_systems(
                PreUpdate,
                input
                    .after(crate::controls::capture)
                    .before(crate::game::input),
            )
            .add_systems(
                Update,
                (view, typography, focus, resize, scene_camera)
                    .chain()
                    .after(crate::ui::update_ui),
            );
    }
}

fn scene_camera(race: Res<Race>, mut cameras: Query<&mut Camera, With<Camera3d>>) {
    for mut camera in &mut cameras {
        camera.is_active = !matches!(race.phase, Phase::Garage | Phase::Finished);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn app() -> App {
        let mut app = crate::game::tests::simulation();
        app.init_resource::<Frontend>().add_systems(
            PreUpdate,
            input
                .after(crate::controls::capture)
                .before(crate::game::input),
        );
        for _ in 0..4 {
            app.update();
        }
        app
    }
    fn action(app: &mut App, action: Action) {
        app.world_mut().resource_mut::<Frontend>().pending = Some(action);
        app.update();
    }
    #[test]
    fn browsing_portraits_preserves_profile_and_garage_phase() {
        let mut app = app();
        app.world_mut().resource_mut::<Frontend>().flow.page = Page::Portrait;
        let credits = app.world().resource::<Profile>().credits;
        action(&mut app, Action::PreviousPortrait);
        assert_eq!(app.world().resource::<Frontend>().flow.portrait, 15);
        for _ in 0..16 {
            action(&mut app, Action::NextPortrait);
        }
        assert_eq!(app.world().resource::<Frontend>().flow.portrait, 15);
        assert_eq!(app.world().resource::<Race>().phase, Phase::Garage);
        assert_eq!(app.world().resource::<Profile>().credits, credits);
        action(&mut app, Action::Back);
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Bar);
    }
    #[test]
    fn portrait_grid_uses_four_columns_and_returns_to_the_picked_card() {
        let mut app = app();
        app.world_mut().resource_mut::<Frontend>().flow.page = Page::Bar;
        for (key, expected) in [
            (KeyCode::ArrowDown, 4),
            (KeyCode::ArrowDown, 8),
            (KeyCode::ArrowDown, 12),
            (KeyCode::ArrowDown, 16),
            (KeyCode::ArrowUp, 12),
        ] {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(key);
            app.update();
            assert_eq!(app.world().resource::<Frontend>().selected, expected);
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .release(key);
            app.update();
        }
        action(&mut app, Action::SelectPortrait(12));
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Portrait);
        action(&mut app, Action::Back);
        assert_eq!(app.world().resource::<Frontend>().selected, 12);
    }
    #[test]
    fn keyboard_edges_navigate_without_starting_the_race_or_repeating() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Main);
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Main);
        assert_eq!(app.world().resource::<Race>().phase, Phase::Garage);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::Enter);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Courses);
        assert_eq!(app.world().resource::<Race>().phase, Phase::Garage);
        action(&mut app, Action::Ride);
        for _ in 0..20 {
            app.update();
        }
        assert_eq!(app.world().resource::<Race>().phase, Phase::Countdown);
        assert!(!app.world().resource::<Controls>().start);
        app.world_mut().resource_mut::<Race>().phase = Phase::Finished;
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Results);
        action(&mut app, Action::Ride);
        for _ in 0..20 {
            app.update();
        }
        assert_eq!(app.world().resource::<Race>().phase, Phase::Countdown);
    }
    #[test]
    fn mouse_action_uses_the_same_flow_and_settings_returns_to_hub() {
        let mut app = app();
        action(&mut app, Action::Continue);
        let e = app
            .world_mut()
            .spawn((
                MenuButton {
                    index: 1,
                    action: Action::Street,
                },
                Interaction::Pressed,
            ))
            .id();
        app.update();
        app.world_mut().despawn(e);
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Rider);
        action(&mut app, Action::Continue);
        action(&mut app, Action::Settings);
        assert!(app.world().resource::<SettingsPanel>().open);
        action(&mut app, Action::Courses);
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Street);
        app.world_mut().resource_mut::<SettingsPanel>().open = false;
        action(&mut app, Action::Garage);
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Garage);
        action(&mut app, Action::Back);
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Street);
    }
    #[test]
    fn disabled_network_is_skipped_and_results_menu_preserves_profile() {
        let mut app = app();
        action(&mut app, Action::Continue);
        app.world_mut().resource_mut::<Frontend>().selected = 1;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowRight);
        app.update();
        assert_eq!(app.world().resource::<Frontend>().selected, 3);
        app.world_mut().resource_mut::<Profile>().credits = 1234;
        app.world_mut().resource_mut::<Frontend>().flow.page = Page::Starting;
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        app.update();
        app.world_mut().resource_mut::<Race>().phase = Phase::Finished;
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Results);
        action(&mut app, Action::Back);
        assert_eq!(app.world().resource::<Race>().phase, Phase::Garage);
        assert_eq!(app.world().resource::<Profile>().credits, 1234);
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Main);
    }
    #[test]
    fn controller_confirm_and_back_are_owned_by_the_menu() {
        let mut app = app();
        let pad = app.world_mut().spawn(Gamepad::default()).id();
        app.world_mut()
            .get_mut::<Gamepad>(pad)
            .unwrap()
            .digital_mut()
            .press(GamepadButton::South);
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Main);
        app.world_mut()
            .get_mut::<Gamepad>(pad)
            .unwrap()
            .digital_mut()
            .clear();
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Main);
        app.world_mut()
            .get_mut::<Gamepad>(pad)
            .unwrap()
            .digital_mut()
            .press(GamepadButton::East);
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Quit);
        assert_eq!(app.world().resource::<Race>().phase, Phase::Garage);
    }
}
