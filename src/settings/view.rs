//! Grouped settings editor, sharing the entry menu typography and palette.
use super::*;
const WHITE: Color = Color::srgb(0.94, 0.93, 0.90);
const ACCENT: Color = Color::srgb(0.96, 0.40, 0.22);
#[derive(Component)]
pub struct Card;
#[derive(Component)]
pub struct Tab(Section);
#[derive(Component)]
pub struct Title;
#[derive(Component)]
pub struct Description;
#[derive(Component)]
pub struct Size(f32);
#[derive(Resource)]
struct Fonts(Handle<Font>, Handle<Font>);
fn text(c: &mut Commands, fonts: &Fonts, value: &str, size: f32, color: Color) -> Entity {
    c.spawn((
        Text::new(value),
        TextFont {
            font: if size >= 28. {
                fonts.1.clone().into()
            } else {
                fonts.0.clone().into()
            },
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(color),
        Size(size),
    ))
    .id()
}
fn button(
    c: &mut Commands,
    fonts: &Fonts,
    label: &str,
    action: SettingsAction,
    node: Node,
) -> Entity {
    let e = c
        .spawn((
            Button,
            action,
            node,
            BackgroundColor(if matches!(action, SettingsAction::Close) {
                Color::srgb(0.60, 0.20, 0.09)
            } else {
                Color::srgb(0.065, 0.06, 0.07)
            }),
            BorderColor::all(Color::srgba(1., 1., 1., 0.12)),
        ))
        .id();
    let t = text(c, fonts, label, 18., WHITE);
    c.entity(e).add_child(t);
    e
}
fn abs(x: f32, y: f32, w: f32, h: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: percent(x),
        top: percent(y),
        width: percent(w),
        height: percent(h),
        ..default()
    }
}
pub fn setup(mut c: Commands, server: Option<Res<AssetServer>>) {
    let fonts = Fonts(
        server
            .as_ref()
            .map(|s| s.load("ui/fonts/Jost-400-Book.ttf"))
            .unwrap_or_default(),
        server
            .as_ref()
            .map(|s| s.load("ui/fonts/Jost-700-Bold.ttf"))
            .unwrap_or_default(),
    );
    let root = c
        .spawn((
            SettingsRoot,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100.),
                height: percent(100.),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgb(0.012, 0.012, 0.02)),
            bevy::ui::FocusPolicy::Block,
            GlobalZIndex(100),
        ))
        .id();
    let title = text(&mut c, &fonts, "SETTINGS", 48., WHITE);
    c.entity(title).insert(abs(5., 5., 80., 9.));
    c.entity(root).add_child(title);
    let hint = text(
        &mut c,
        &fonts,
        "Page Up / Down: category     Arrows: select / adjust     Enter: change     Tab: binding device",
        15.,
        WHITE.with_alpha(0.6),
    );
    c.entity(hint).insert(abs(5., 15., 90., 5.));
    c.entity(root).add_child(hint);
    for (i, section) in Section::ALL.into_iter().enumerate() {
        let mut n = abs(5., 25. + i as f32 * 9., 20., 7.);
        n.align_items = AlignItems::Center;
        n.padding = UiRect::left(px(16.));
        n.border = UiRect::left(px(3.));
        let e = button(
            &mut c,
            &fonts,
            section.name(),
            SettingsAction::Tab(section),
            n,
        );
        c.entity(e).insert(Tab(section));
        c.entity(root).add_child(e);
    }
    let heading = text(&mut c, &fonts, "Camera", 28., WHITE);
    c.entity(heading).insert((Title, abs(30., 20., 64., 6.)));
    c.entity(root).add_child(heading);
    let description = text(&mut c, &fonts, "", 15., WHITE.with_alpha(0.6));
    c.entity(description)
        .insert((Description, abs(30., 25., 64., 4.)));
    c.entity(root).add_child(description);
    let mut grid = abs(30., 30., 64., 51.);
    grid.display = Display::Grid;
    grid.grid_template_columns = vec![GridTrack::flex(1.); 2];
    grid.grid_template_rows = vec![GridTrack::flex(1.); 8];
    grid.column_gap = px(10.);
    grid.row_gap = px(4.);
    let body = c.spawn(grid).id();
    c.entity(root).add_child(body);
    for row in 0..ROWS {
        let e = c
            .spawn((
                Button,
                SettingsRow(row),
                Card,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::axes(px(10.), px(4.)),
                    min_height: px(0.),
                    min_width: px(0.),
                    border: UiRect::left(px(2.)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.038, 0.036, 0.045)),
                BorderColor::all(Color::NONE),
            ))
            .id();
        let label = text(&mut c, &fonts, caption(row), 16., WHITE.with_alpha(0.65));
        c.entity(label).insert(Node {
            max_width: percent(44.),
            ..default()
        });
        c.entity(e).add_child(label);
        let value = text(&mut c, &fonts, "", 17., WHITE);
        c.entity(value).insert((
            SettingsRow(row),
            Node {
                max_width: percent(42.),
                ..default()
            },
        ));
        c.entity(e).add_child(value);
        if binding_action(row).is_none() && row != 8 && row != 9 {
            for (label, d) in [("−", -1), ("+", 1)] {
                let n = Node {
                    width: px(28.),
                    height: percent(90.),
                    min_width: px(22.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                };
                let a = button(&mut c, &fonts, label, SettingsAction::Adjust(row, d), n);
                c.entity(e).add_child(a);
            }
        }
        c.entity(body).add_child(e);
    }
    let footer = text(
        &mut c,
        &fonts,
        "Changes apply immediately. Save & close to keep them.",
        15.,
        WHITE.with_alpha(0.65),
    );
    c.entity(footer)
        .insert((SettingsText, abs(30., 83., 64., 6.)));
    c.entity(root).add_child(footer);
    for (label, action, x, w) in [
        ("Save & close", SettingsAction::Close, 30., 24.),
        ("Close without saving", SettingsAction::Dismiss, 57., 37.),
    ] {
        let mut n = abs(x, 91., w, 6.);
        n.align_items = AlignItems::Center;
        n.justify_content = JustifyContent::Center;
        n.border = UiRect::all(px(1.));
        let e = button(&mut c, &fonts, label, action, n);
        c.entity(root).add_child(e);
    }
    c.insert_resource(fonts);
}
fn caption(row: usize) -> &'static str {
    match row {
        0 => "Field of view",
        1 => "Camera response",
        2 => "Camera shake",
        3 => "Anti-nausea",
        4 => "Speed FOV",
        5 => "Speed lines",
        6 => "Controller deadzone",
        7 => "Binding device",
        8 => "Camera defaults",
        9 => "Save",
        10 => "Throttle",
        11 => "Brake",
        12 => "Steer left",
        13 => "Steer right",
        14 => "Nitro (disabled)",
        15 => "Punch left",
        16 => "Punch right",
        17 => "Start / mount",
        18 => "Pause",
        19 => "Restart",
        20 => "Camera",
        21 => "Mute key",
        22 => "Master volume",
        23 => "VSync",
        24 => "Fullscreen",
        25 => "Mute all audio",
        26 => "Pick up weapon",
        27 => "Drop weapon",
        28 => "Engine volume",
        29 => "Wind volume",
        30 => "Effects volume",
        31 => "Window size",
        32 => "Speed units",
        33 => "Menu motion",
        34 => "Proximity chat",
        35 => "Microphone",
        36 => "Input mode",
        37 => "Activation level",
        38 => "Voice volume",
        39 => "Push-to-talk key",
        40 => "Select rider",
        41 => "Mute rider",
        42 => "World shadows",
        43 => "Minimap",
        44 => "Standings",
        _ => "",
    }
}
fn value(
    row: usize,
    panel: &SettingsPanel,
    prefs: &Preferences,
    online: Option<&crate::online::Online>,
) -> String {
    let c = &prefs.file.comfort;
    let on = |b| if b { "On" } else { "Off" };
    let percent = |gain: f32| format!("{:.0}%", gain * 100.);
    let voice = &prefs.file.voice;
    let target = || {
        panel.voice_target.and_then(|slot| {
            online?
                .lobby
                .as_ref()?
                .players
                .iter()
                .find(|p| !p.bot && p.slot == slot)
        })
    };
    match row {
        0 => format!("{:.0}°", c.fov_degrees),
        1 => format!("{:.0}", c.smoothing),
        2 => format!("{:.0}%", c.shake * 100.),
        3 => on(c.anti_nausea).into(),
        4 => on(c.speed_fov).into(),
        5 => on(c.speed_lines).into(),
        6 => format!("{:.0}%", c.deadzone * 100.),
        7 => if panel.controller {
            "Controller"
        } else {
            "Keyboard"
        }
        .into(),
        8 => "Restore".into(),
        9 => "Save".into(),
        22 => format!("{:.0}%", prefs.file.master_volume * 100.),
        23 => on(prefs.file.vsync).into(),
        24 => on(prefs.file.fullscreen).into(),
        25 => on(prefs.file.audio_muted).into(),
        28 => percent(prefs.file.mix.engine),
        29 => percent(prefs.file.mix.ambience),
        30 => percent(prefs.file.mix.effects),
        31 => prefs
            .file
            .window_size
            .dimensions()
            .map_or_else(|| "Automatic".into(), |(w, h)| format!("{w} × {h}")),
        32 => if prefs.file.imperial_speed {
            "mph"
        } else {
            "km/h"
        }
        .into(),
        33 => on(!prefs.file.static_menu).into(),
        34 => on(voice.enabled).into(),
        35 => on(voice.microphone).into(),
        36 => match voice.mode {
            crate::online::voice::Mode::Activation => "Voice activation",
            crate::online::voice::Mode::PushToTalk => "Push-to-talk",
        }
        .into(),
        37 => format!("{:.1}%", voice.threshold * 100.),
        38 => percent(voice.volume),
        39 if !prefs.file.voice_key_available(&voice.push_key) => "Choose a free key".into(),
        39 => pretty_binding(&voice.push_key),
        40 => target().map_or_else(|| "Select".into(), |p| p.name.clone()),
        42 => on(prefs.file.shadows).into(),
        43 => on(prefs.file.show_minimap).into(),
        44 => on(prefs.file.show_standings).into(),
        41 => target()
            .and_then(|p| online.map(|o| on(o.voice_muted(p.slot))))
            .unwrap_or("—")
            .into(),
        _ => {
            let Some(action) = binding_action(row) else {
                return String::new();
            };
            if panel.rebinding && panel.selected == row {
                return "Press key / button...".into();
            }
            let s = if panel.controller {
                prefs
                    .file
                    .controller
                    .get(action)
                    .cloned()
                    .unwrap_or("Unbound".into())
            } else {
                prefs
                    .file
                    .keyboard
                    .actions
                    .get(action)
                    .map(|a| a.join(" / "))
                    .unwrap_or_else(|| match action {
                        "pickup" => "KeyE".into(),
                        "drop" => "KeyQ".into(),
                        _ => "Unbound".into(),
                    })
            };
            pretty_binding(&s)
        }
    }
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn display(
    panel: Res<SettingsPanel>,
    prefs: Res<Preferences>,
    online: Option<Res<crate::online::Online>>,
    mut root: Query<&mut Node, With<SettingsRoot>>,
    mut labels: Query<(&SettingsRow, &mut Text)>,
    mut cards: Query<
        (&SettingsRow, &mut Node, &mut BorderColor),
        (With<Card>, Without<Tab>, Without<SettingsRoot>),
    >,
    mut tabs: Query<(&Tab, &mut BorderColor), (Without<Card>, Without<SettingsRoot>)>,
    mut title: Query<
        &mut Text,
        (
            With<Title>,
            Without<SettingsRow>,
            Without<SettingsText>,
            Without<Description>,
        ),
    >,
    mut desc: Query<
        &mut Text,
        (
            With<Description>,
            Without<SettingsRow>,
            Without<SettingsText>,
            Without<Title>,
        ),
    >,
    mut footer: Query<
        &mut Text,
        (
            With<SettingsText>,
            Without<SettingsRow>,
            Without<Title>,
            Without<Description>,
        ),
    >,
    windows: Query<&Window>,
    mut fonts: Query<(&Size, &mut TextFont)>,
) {
    for mut n in &mut root {
        let d = if panel.open {
            Display::Flex
        } else {
            Display::None
        };
        if n.display != d {
            n.display = d;
        }
    }
    if !panel.open {
        return;
    }
    let scale = windows
        .iter()
        .next()
        .map(|w| (w.width() / 1440.).min(w.height() / 900.))
        .unwrap_or(1.)
        .clamp(0.7, 2.5);
    for (size, mut f) in &mut fonts {
        let s = size.0 * scale;
        if f.font_size != FontSize::Px(s) {
            f.font_size = FontSize::Px(s);
        }
    }
    for (row, mut n, mut border) in &mut cards {
        let d = if panel.section.rows().contains(&row.0) {
            Display::Flex
        } else {
            Display::None
        };
        if n.display != d {
            n.display = d;
        }
        border.set_if_neq(BorderColor::all(if panel.selected == row.0 {
            ACCENT
        } else {
            Color::NONE
        }));
    }
    for (tab, mut border) in &mut tabs {
        border.set_if_neq(BorderColor::all(if panel.section == tab.0 {
            ACCENT
        } else {
            Color::NONE
        }));
    }
    for (row, mut t) in &mut labels {
        t.set_if_neq(Text::new(value(row.0, &panel, &prefs, online.as_deref())));
    }
    for mut t in &mut title {
        t.set_if_neq(Text::new(panel.section.name()));
    }
    for mut t in &mut desc {
        t.set_if_neq(Text::new(match panel.section {
            Section::Camera => "Adjust field of view, camera motion and comfort effects.",
            Section::Controls => {
                "Tab switches device. Kick: Space; mouse buttons also punch. RB: next category."
            }
            Section::Audio => "Adjust the overall mix or individual game sounds.",
            Section::Display => "Window size applies in windowed mode. VSync reduces tearing.",
            Section::Accessibility => "Choose speed units, HUD elements and menu motion.",
            Section::Voice => {
                "Nearby racers can hear your microphone when enabled. Uses system devices."
            }
        }));
    }
    for mut t in &mut footer {
        let status = if panel.section == Section::Voice
            && let Some(online) = online.as_ref().filter(|o| !o.voice_status.is_empty())
        {
            &online.voice_status
        } else if prefs.status.is_empty() {
            "Changes apply immediately."
        } else {
            &prefs.status
        };
        let keys = if panel.rebinding {
            "Esc / Start: cancel rebinding"
        } else {
            "F2 / Esc / Start: save & close     Backspace / Select: close without saving"
        };
        t.set_if_neq(Text::new(format!("{status}\n{keys}")));
    }
}
