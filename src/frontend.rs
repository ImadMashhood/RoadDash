//! Native entry screens and authoritative-session lobby presentation.
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
mod layout;
mod online_menu;
mod online_smoke;
mod original_art;
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
    pub online_pause: bool,
    online_menu: online_menu::Menu,
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
    originals: Vec<Option<Handle<Image>>>,
    heading_font: Handle<Font>,
    body_font: Handle<Font>,
}
#[derive(Component)]
struct Root;
#[derive(Component)]
struct Canvas;
#[derive(Component)]
struct Backdrop;
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
    key: Option<(Page, usize, bool, bool, bool, String)>,
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
            entry("PLAY OFFLINE", A::Thrash),
            entry("STREET HUB", A::Street),
            entry("MULTIPLAYER", A::Online),
            entry("SETTINGS", A::Settings),
        ],
        Page::Online | Page::Host | Page::Join | Page::Lobby => vec![],
        Page::Rider => vec![entry("RIDE AS YOU", A::Continue), entry("BACK", A::Back)],
        Page::Street => vec![
            entry("BIKER BAR", A::Bar),
            entry("BIKE SHOP", A::Garage),
            entry("RACE", A::Courses),
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
    mut online: Option<ResMut<crate::online::Online>>,
) {
    let online_race = online.as_ref().is_some_and(|o| o.lobby.is_some())
        && matches!(race.phase, Phase::Racing | Phase::Countdown);
    if online_race && !panel.open {
        let pad = pads.iter().min_by_key(|(e, _)| e.to_bits()).map(|(_, p)| p);
        let edge = |b| pad.is_some_and(|p| p.just_pressed(b));
        if std::mem::take(&mut controls.pause)
            || keys.just_pressed(KeyCode::Escape)
            || edge(GamepadButton::Start)
        {
            front.online_pause = !front.online_pause;
            front.selected = 0;
        }
        if front.online_pause {
            let start = std::mem::take(&mut controls.start);
            controls.buffer = default();
            controls.foot = Vec2::ZERO;
            controls.look = Vec2::ZERO;
            controls.interact = false;
            controls.pickup = false;
            controls.drop = false;
            controls.camera = false;
            controls.restart = false;
            menu.0 = false;
            if keys.just_pressed(KeyCode::ArrowUp)
                || keys.just_pressed(KeyCode::ArrowDown)
                || edge(GamepadButton::DPadUp)
                || edge(GamepadButton::DPadDown)
            {
                front.selected = 1 - front.selected.min(1);
            }
            let click = clicks
                .iter()
                .find_map(|(i, b)| (*i == Interaction::Pressed).then_some(b.action));
            let action = front.pending.take().or(click).or_else(|| {
                if edge(GamepadButton::East) {
                    Some(Action::Continue)
                } else if start {
                    Some(if front.selected == 0 {
                        Action::Continue
                    } else {
                        Action::Back
                    })
                } else {
                    None
                }
            });
            match action {
                Some(Action::Continue) => front.online_pause = false,
                Some(Action::Back) => {
                    if let Some(online) = online.as_deref_mut() {
                        online.actions.push_back(crate::online::OnlineAction::Leave);
                    }
                    front.online_pause = false;
                    front.online_menu.leaving = true;
                    front.flow.page = Page::Main;
                }
                _ => {}
            }
            return;
        }
    } else if !online_race {
        front.online_pause = false;
    }
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
    let retry = std::mem::take(&mut controls.restart) && race.phase == Phase::Finished;
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
    if let Some(online) = online.as_deref_mut() {
        online_menu::sync(&mut front, online);
    }
    let rows =
        online_menu::entries(&front, online.as_deref()).unwrap_or_else(|| entries(front.flow.page));
    front.selected = front.selected.min(rows.len().saturating_sub(1));
    let editing = online_menu::type_code(&mut front, &keys);
    if editing {
        // Text characters must not trigger remapped gameplay/mute shortcuts.
        controls.mute = false;
        for (button, action) in [
            (GamepadButton::DPadLeft, Action::CodePrevious),
            (GamepadButton::DPadRight, Action::CodeNext),
            (GamepadButton::DPadUp, Action::CodeUp),
            (GamepadButton::DPadDown, Action::CodeDown),
        ] {
            if edge(button) {
                online_menu::dispatch(&mut front, online.as_deref_mut(), action);
            }
        }
    }
    let direction = if editing {
        0
    } else if keys.just_pressed(KeyCode::ArrowRight)
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
        if (if editing {
            keys.just_pressed(KeyCode::Escape)
        } else {
            back
        }) || edge(GamepadButton::East)
        {
            Some(Action::Back)
        } else if retry {
            Some(Action::Ride)
        } else if if editing {
            keys.just_pressed(KeyCode::Enter) || edge(GamepadButton::South)
        } else {
            start
        } {
            rows.get(front.selected).and_then(|e| e.action)
        } else {
            None
        }
    });
    if let Some(action) = action {
        let was = front.flow.page;
        if online_menu::dispatch(&mut front, online.as_deref_mut(), action) {
            return;
        }
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

fn setup(mut commands: Commands, server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
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
        originals: original_art::load(&mut images),
        heading_font: server.load("ui/fonts/Jost-700-Bold.ttf"),
        body_font: server.load("ui/fonts/Jost-400-Book.ttf"),
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
            node,
        ))
        .id()
}
fn title(page: Page) -> &'static str {
    match page {
        Page::Splash | Page::Main => "ROAD / DASH",
        Page::Online => "RACE TOGETHER",
        Page::Host => "HOST A RACE",
        Page::Join => "JOIN A RACE",
        Page::Lobby => "RACE LOBBY",
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
        Page::Online | Page::Host | Page::Join | Page::Lobby => 7,
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
        Page::Online | Page::Host | Page::Join | Page::Lobby => String::new(),
        Page::Main => format!("{} CREDITS  /  {} WINS", profile.credits, profile.wins),
        Page::Rider => "YOU / BLUE HELMET\nYOUR CURRENT RIDER AND MOTORCYCLE".into(),
        Page::Street => format!("{}\n{} CREDITS", course_name, profile.credits),
        Page::Courses => format!("{}\n{:.2} KM / {}\n15 RIDERS", course_name, track.race_interval().length()/1000.0, if track.definition.closed {"CIRCUIT"} else {"POINT TO POINT"}),
        Page::Garage => "YOUR ROAD BIKE / READY TO RIDE\nBIKE PURCHASES UNAVAILABLE".into(),
        Page::Portrait => "RIDER PORTRAITS".into(),
        Page::Bar => "THE LOCALS DON'T LIKE YOU.\n\nWatch the aggressive riders.\nA clean pass is safer than a fight.\n\nConversations coming later.".into(),
        Page::Starting => format!("{}\nW / S: THROTTLE / BRAKE    A / D: STEER\nMOUSE: PUNCH    SPACE: KICK\nRIDE OUT...",course_name),
        Page::Results if race.arrested => "BUSTED\n\nNo payout.\nTry your luck again.".into(),
        Page::Results => format!("P{} / 15    TIME {:02}:{:05.2}\nTAKEDOWNS {}\n+{} CREDITS    BALANCE {}",race.place,(race.elapsed/60.0) as u32,race.elapsed%60.0,race.takedowns,race.earned,profile.credits),
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
    mut root: Query<(&mut Node, &mut BackgroundColor), With<Root>>,
    canvas: Query<(Entity, Option<&Children>), With<Canvas>>,
    online: Option<Res<crate::online::Online>>,
) {
    let active = front.online_pause || matches!(race.phase, Phase::Garage | Phase::Finished);
    let (mut root, mut background) = root.single_mut().expect("frontend root");
    *background = BackgroundColor(if front.online_pause {
        Color::NONE
    } else {
        Color::BLACK
    });
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
        online_menu::signature(&front, online.as_deref()),
    );
    if cache.key.as_ref() == Some(&key) {
        return;
    }
    cache.key = Some(key);
    let (canvas, children) = canvas.single().expect("frontend canvas");
    if let Some(children) = children {
        for e in children {
            commands.entity(*e).despawn();
        }
    }
    if front.online_pause {
        online_menu::render_pause(&mut commands, canvas, online.as_deref());
        return;
    }
    let page = front.flow.page;
    let image = commands
        .spawn((
            ImageNode::new(if page == Page::Portrait {
                art.images[ART.len() + front.flow.portrait].clone()
            } else {
                art.originals
                    .get(picture(page))
                    .and_then(Clone::clone)
                    .unwrap_or_else(|| {
                        art.images[if picture(page) >= ART.len() {
                            1
                        } else {
                            picture(page)
                        }]
                        .clone()
                    })
            }),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                ..default()
            },
        ))
        .id();
    commands.entity(image).insert(Backdrop);
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
    if !loading
        && !failed
        && online_menu::render(&mut commands, canvas, &front, online.as_deref(), &race)
    {
        return;
    }
    let value = if failed {
        "ARTWORK COULD NOT LOAD. Restore assets/ui and relaunch.".into()
    } else if loading {
        "PREPARING THE ROAD...".into()
    } else {
        body(page, &profile, &track, &race)
    };
    layout::render(&mut commands, canvas, page, title(page), &value, loading);
}

#[allow(clippy::type_complexity)]
fn focus(
    front: Res<Frontend>,
    mut buttons: Query<(
        &MenuButton,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        Option<&layout::MenuChoice>,
        &Children,
    )>,
    mut labels: Query<&mut TextColor>,
) {
    for (b, i, mut bg, mut border, classic, children) in &mut buttons {
        let selected = b.index == front.selected || *i == Interaction::Hovered;
        if classic.is_some() {
            let selected = b.index == front.selected;
            bg.set_if_neq(BackgroundColor(if *i == Interaction::Hovered {
                Color::srgba(0.96, 0.40, 0.22, 0.025)
            } else {
                Color::NONE
            }));
            border.set_if_neq(BorderColor::all(if selected {
                layout::ACCENT
            } else {
                Color::NONE
            }));
            for child in children {
                if let Ok(mut color) = labels.get_mut(*child) {
                    color.set_if_neq(TextColor(layout::PAPER));
                }
            }
            continue;
        }
        bg.set_if_neq(BackgroundColor(if selected {
            Color::srgba(0.04, 0.035, 0.04, 0.96)
        } else {
            Color::srgba(0.015, 0.012, 0.02, 0.94)
        }));
        border.set_if_neq(BorderColor::all(if selected {
            layout::ACCENT
        } else {
            Color::srgba(1., 1., 1., 0.12)
        }));
    }
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn resize(
    windows: Query<&Window>,
    front: Res<Frontend>,
    images: Res<Assets<Image>>,
    art: Res<Art>,
    mut backdrops: Query<(&ImageNode, &mut Node), (With<Backdrop>, Without<Canvas>)>,
    mut canvas: Query<&mut Node, (With<Canvas>, Without<Backdrop>)>,
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
    let gallery = matches!(front.flow.page, Page::Bar | Page::Portrait);
    let width = if gallery {
        window.width().min(window.height() * 4. / 3.)
    } else {
        window.width()
    };
    let height = if gallery {
        width * 3. / 4.
    } else {
        window.height()
    };
    for (image, mut node) in &mut backdrops {
        if let Some(asset) = images.get(&image.image) {
            let size = asset.size_f32();
            let original = art.originals.iter().flatten().any(|h| h == &image.image);
            let scale = if original {
                (width / size.x).min(height / size.y)
            } else {
                (width / size.x).max(height / size.y)
            };
            let w = size.x * scale / width * 100.;
            let h = size.y * scale / height * 100.;
            let left = percent((100. - w) * 0.5);
            let top = percent((100. - h) * 0.5);
            if node.width != percent(w)
                || node.height != percent(h)
                || node.left != left
                || node.top != top
            {
                node.width = percent(w);
                node.height = percent(h);
                node.left = left;
                node.top = top;
            }
        }
    }
    let Ok(mut node) = canvas.single_mut() else {
        return;
    };
    if node.width != px(width) || node.height != px(height) {
        node.width = px(width);
        node.height = px(height);
    }
    for (base, mut font) in &mut fonts {
        let size = base.0
            * if gallery {
                width / 1440.
            } else {
                (width / 1440.).min(height / 810.)
            };
        if font.font_size != size {
            font.font_size = size;
        }
    }
}
fn typography(art: Res<Art>, mut labels: Query<(&FontSize, &mut TextFont)>) {
    for (base, mut font) in &mut labels {
        let face = if base.0 >= 40.0 {
            &art.heading_font
        } else {
            &art.body_font
        };
        if font.font != *face {
            font.font = face.clone();
        }
    }
}
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FrontendSet {
    Input,
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
                    .in_set(FrontendSet::Input)
                    .after(crate::controls::capture)
                    .before(crate::game::input),
            )
            .add_systems(
                Update,
                (
                    view,
                    typography,
                    focus,
                    resize,
                    scene_camera,
                    mask_online_hud,
                )
                    .chain()
                    .after(crate::ui::update_ui),
            );
    }
}

// update_ui restores normal HUD visibility each frame; mask it only while the
// live race menu is open, without hiding the world or pausing its simulation.
fn mask_online_hud(front: Res<Frontend>, mut hud: Query<&mut Node, With<crate::ui::Hud>>) {
    if front.online_pause {
        for mut node in &mut hud {
            node.display = Display::None;
        }
    }
}

fn scene_camera(
    race: Res<Race>,
    panel: Res<SettingsPanel>,
    mut cameras: Query<&mut Camera, With<Camera3d>>,
) {
    for mut camera in &mut cameras {
        let active = !panel.open && !matches!(race.phase, Phase::Garage | Phase::Finished);
        if camera.is_active != active {
            camera.is_active = active;
        }
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
    fn online_race_menu_resumes_or_queues_leave_without_pausing_simulation() {
        for host in [true, false] {
            let mut app = app();
            app.insert_resource(crate::online::Online {
                phase: crate::online::OnlinePhase::Racing,
                lobby: Some(crate::online::LobbyView {
                    code: "AAAQAAK3CY".into(),
                    public: true,
                    host,
                    map: crate::online::OnlineMap::Metropolis,
                    players: vec![],
                    countdown: None,
                }),
                ..default()
            });
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.world_mut().resource_mut::<Controls>().pause = true;
            app.update();
            assert!(app.world().resource::<Frontend>().online_pause);
            assert_eq!(app.world().resource::<Race>().phase, Phase::Racing);
            assert!(!app.world().resource::<Controls>().pause);
            action(&mut app, Action::Continue);
            assert!(!app.world().resource::<Frontend>().online_pause);
            assert_eq!(app.world().resource::<Race>().phase, Phase::Racing);
            app.world_mut().resource_mut::<Controls>().pause = true;
            app.update();
            action(&mut app, Action::Back);
            assert!(!app.world().resource::<Frontend>().online_pause);
            assert!(matches!(
                app.world_mut()
                    .resource_mut::<crate::online::Online>()
                    .actions
                    .pop_front(),
                Some(crate::online::OnlineAction::Leave)
            ));
            assert_eq!(app.world().resource::<Race>().phase, Phase::Racing);
        }
    }
    #[test]
    fn join_code_typing_owns_remapped_start_pause_and_mute_edges() {
        let mut app = app();
        {
            let mut front = app.world_mut().resource_mut::<Frontend>();
            front.flow.page = Page::Join;
            front.online_menu.editing = true;
        }
        {
            let mut controls = app.world_mut().resource_mut::<Controls>();
            controls.start = true;
            controls.pause = true;
            controls.mute = true;
        }
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyM);
        app.update();
        let front = app.world().resource::<Frontend>();
        assert_eq!(front.flow.page, Page::Join);
        assert!(front.online_menu.editing);
        assert_eq!(front.online_menu.code, "M");
        assert!(!app.world().resource::<Controls>().mute);
        assert_eq!(app.world().resource::<Race>().phase, Phase::Garage);
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
    fn results_restart_edge_retries_finish_and_bust_once_through_starting() {
        for busted in [false, true] {
            let mut app = app();
            let officer = app
                .world_mut()
                .spawn((
                    crate::game::Rider::new(crate::police::OFFICER_ID),
                    Transform::default(),
                    Visibility::Visible,
                    crate::recovery::bike_body(),
                ))
                .id();
            let credits = app.world().resource::<Profile>().credits;
            app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
            app.update();
            {
                let mut race = app.world_mut().resource_mut::<Race>();
                race.phase = Phase::Finished;
                race.arrested = busted;
            }
            if busted {
                app.world_mut()
                    .resource_mut::<crate::police::Police>()
                    .mind
                    .state = rr_core::police::State::Arrested;
            }
            app.update();
            // R is a retry shortcut even when MAIN MENU has keyboard selection.
            app.world_mut().resource_mut::<Frontend>().selected = 1;
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::KeyR);
            app.update();
            assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Starting);
            assert_eq!(app.world().resource::<Race>().phase, Phase::Finished);
            assert!(!app.world().resource::<Controls>().restart);
            for _ in 0..25 {
                app.update();
            }
            assert_eq!(app.world().resource::<Race>().phase, Phase::Countdown);
            assert!(!app.world().resource::<Race>().arrested);
            assert_eq!(
                app.world().resource::<crate::police::Police>().mind.state,
                rr_core::police::State::Patrol
            );
            assert!(
                app.world()
                    .get::<crate::game::Rider>(officer)
                    .unwrap()
                    .police
            );
            let countdown = app.world().resource::<Race>().countdown;
            for _ in 0..10 {
                app.update();
            }
            assert!(
                app.world().resource::<Race>().countdown < countdown,
                "held R restarted the countdown more than once"
            );
            assert_eq!(app.world().resource::<Profile>().credits, credits);
            assert_eq!(app.world().resource::<Profile>().races, 0);
            assert!(!app.world().resource::<Controls>().restart);
        }
    }
    #[test]
    fn restart_shortcut_cannot_launch_garage_or_bypass_results_settings() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        for _ in 0..25 {
            app.update();
        }
        assert_eq!(app.world().resource::<Race>().phase, Phase::Garage);
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Splash);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyR);
        app.update();
        app.world_mut().resource_mut::<Race>().phase = Phase::Finished;
        app.world_mut().resource_mut::<SettingsPanel>().open = true;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        app.update();
        assert_eq!(app.world().resource::<Frontend>().flow.page, Page::Results);
        app.world_mut().resource_mut::<SettingsPanel>().open = false;
        for _ in 0..25 {
            app.update();
        }
        assert_eq!(app.world().resource::<Race>().phase, Phase::Finished);
        assert!(!app.world().resource::<Controls>().restart);
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
    fn multiplayer_is_selectable_and_results_menu_preserves_profile() {
        let mut app = app();
        action(&mut app, Action::Continue);
        app.world_mut().resource_mut::<Frontend>().selected = 1;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowRight);
        app.update();
        assert_eq!(app.world().resource::<Frontend>().selected, 2);
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
