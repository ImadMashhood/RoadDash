use crate::{
    game::{NAMES, Phase, Profile, Race, Rider},
    track::{LAPS, Track},
};
use bevy::prelude::*;

const INK: Color = Color::srgb(0.025, 0.045, 0.06);
const MINT: Color = Color::srgb(0.14, 0.89, 0.74);
const WHITE: Color = Color::srgb(0.91, 0.95, 0.94);
const MUTED: Color = Color::srgb(0.54, 0.65, 0.68);
#[derive(Component)]
pub struct Hud;
#[derive(Component)]
pub struct Overlay;
#[derive(Component)]
pub struct Readout(pub u8);
#[derive(Component)]
pub struct MapDot(pub usize);
#[derive(Component)]
pub struct StartButton;

fn label(
    commands: &mut Commands,
    text: &str,
    size: f32,
    color: Color,
    left: f32,
    top: f32,
) -> Entity {
    commands
        .spawn((
            Text::new(text),
            TextFont {
                font_size: size,
                ..default()
            },
            TextColor(color),
            Node {
                position_type: PositionType::Absolute,
                left: px(left),
                top: px(top),
                ..default()
            },
        ))
        .id()
}
fn map_point(p: Vec3) -> (f32, f32) {
    (108.0 + p.x * 0.36, 126.0 + p.z * 0.36)
}

pub fn setup_ui(mut commands: Commands, track: Res<Track>) {
    let hud = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                ..default()
            },
            Hud,
        ))
        .id();
    let mut kids = vec![];
    kids.push(label(&mut commands, "ROAD / DASH", 25.0, WHITE, 32.0, 24.0));
    kids.push(label(
        &mut commands,
        "PACIFIC RUN   /   COASTAL CIRCUIT",
        12.0,
        MINT,
        33.0,
        58.0,
    ));
    let standings = label(&mut commands, "", 17.0, WHITE, 34.0, 116.0);
    commands.entity(standings).insert(Readout(0));
    kids.push(standings);
    let lap = label(&mut commands, "", 17.0, WHITE, 34.0, 84.0);
    commands.entity(lap).insert(Readout(1));
    kids.push(lap);
    let speed = commands
        .spawn((
            Text::new("000"),
            TextFont {
                font_size: 80.0,
                ..default()
            },
            TextColor(WHITE),
            Node {
                position_type: PositionType::Absolute,
                right: px(36),
                bottom: px(85),
                ..default()
            },
            Readout(2),
        ))
        .id();
    kids.push(speed);
    let units = commands
        .spawn((
            Text::new("KM/H   /   6-SPEED"),
            TextFont {
                font_size: 13.0,
                ..default()
            },
            TextColor(MINT),
            Node {
                position_type: PositionType::Absolute,
                right: px(38),
                bottom: px(68),
                ..default()
            },
        ))
        .id();
    kids.push(units);
    let status = commands
        .spawn((
            Text::new(""),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: px(34),
                bottom: px(64),
                ..default()
            },
            Readout(3),
        ))
        .id();
    kids.push(status);
    let help = commands
        .spawn((
            Text::new("WASD  RIDE     SHIFT  NITRO     J / K  STRIKE     C  CAMERA     ESC  PAUSE"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(MUTED),
            Node {
                position_type: PositionType::Absolute,
                left: px(34),
                bottom: px(24),
                ..default()
            },
        ))
        .id();
    kids.push(help);
    let message = commands
        .spawn((
            Text::new(""),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(MINT),
            TextLayout::new_with_justify(Justify::Center),
            Node {
                position_type: PositionType::Absolute,
                top: percent(20),
                width: percent(100),
                ..default()
            },
            Readout(4),
        ))
        .id();
    kids.push(message);
    let map = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: px(28),
                top: px(24),
                width: px(218),
                height: px(272),
                ..default()
            },
            BackgroundColor(INK.with_alpha(0.83)),
            BorderRadius::all(px(12)),
        ))
        .id();
    let mut dots = vec![];
    for p in track.points.iter().step_by(4) {
        let (x, y) = map_point(*p);
        dots.push(
            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(x),
                        top: px(y),
                        width: px(3),
                        height: px(3),
                        ..default()
                    },
                    BackgroundColor(MUTED.with_alpha(0.65)),
                ))
                .id(),
        );
    }
    for id in (0..8).rev() {
        dots.push(
            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: px(if id == 0 { 8 } else { 5 }),
                        height: px(if id == 0 { 8 } else { 5 }),
                        ..default()
                    },
                    BackgroundColor(if id == 0 { MINT } else { WHITE }),
                    BorderRadius::all(px(5)),
                    MapDot(id),
                ))
                .id(),
        );
    }
    dots.push(label(&mut commands, "COURSE / 01", 12.0, MINT, 14.0, 12.0));
    dots.push(label(&mut commands, "", 12.0, MUTED, 14.0, 248.0));
    if let Some(last) = dots.last() {
        commands.entity(*last).insert(Readout(5));
    }
    commands.entity(map).add_children(&dots);
    kids.push(map);
    commands.entity(hud).add_children(&kids);

    let overlay = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                ..default()
            },
            BackgroundColor(INK.with_alpha(0.77)),
            Overlay,
        ))
        .id();
    let mut menu = vec![];
    menu.push(label(
        &mut commands,
        "A MOTORCYCLE COMBAT RACER",
        14.0,
        MINT,
        70.0,
        80.0,
    ));
    menu.push(label(
        &mut commands,
        "ROAD\n/ DASH",
        100.0,
        WHITE,
        62.0,
        112.0,
    ));
    let title = label(&mut commands, "", 27.0, MINT, 70.0, 370.0);
    commands.entity(title).insert(Readout(6));
    menu.push(title);
    let body = label(&mut commands, "", 17.0, WHITE, 70.0, 420.0);
    commands.entity(body).insert(Readout(7));
    menu.push(body);
    let button = commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: px(70),
                top: px(568),
                width: px(340),
                height: px(62),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(MINT),
            BorderRadius::all(px(4)),
            StartButton,
        ))
        .id();
    let text = commands
        .spawn((
            Text::new("ENTER / RIDE OUT"),
            TextFont {
                font_size: 21.0,
                ..default()
            },
            TextColor(INK),
            Readout(8),
        ))
        .id();
    commands.entity(button).add_child(text);
    menu.push(button);
    menu.push(label(&mut commands, "W / S   THROTTLE & BRAKE     A / D   STEER\nSHIFT   NITRO     J / K   LEFT / RIGHT PUNCH\nC   CAMERA     R   RESTART     ESC   PAUSE", 14.0, MUTED, 70.0, 661.0));
    let profile = label(&mut commands, "", 13.0, MINT, 70.0, 752.0);
    commands.entity(profile).insert(Readout(9));
    menu.push(profile);
    commands.entity(overlay).add_children(&menu);
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn update_ui(
    race: Res<Race>,
    profile: Res<Profile>,
    track: Res<Track>,
    riders: Query<&Rider>,
    mut readouts: Query<(&Readout, &mut Text)>,
    mut nodes: Query<(&mut Node, Option<&Hud>, Option<&Overlay>), Without<MapDot>>,
    mut dots: Query<(&MapDot, &mut Node)>,
) {
    let Some(player) = riders.iter().find(|r| r.id == 0) else {
        return;
    };
    let menu = matches!(race.phase, Phase::Garage | Phase::Paused | Phase::Finished);
    for (mut node, hud, overlay) in &mut nodes {
        if hud.is_some() {
            node.display = if menu { Display::None } else { Display::Flex };
        }
        if overlay.is_some() {
            node.display = if menu { Display::Flex } else { Display::None };
        }
    }
    let mut sorted: Vec<_> = riders.iter().collect();
    sorted.sort_by(|a, b| b.progress.total_cmp(&a.progress));
    let position = sorted.iter().position(|r| r.id == 0).unwrap_or(7) + 1;
    for (field, mut text) in &mut readouts {
        **text = match field.0 {
            0 => {
                let mut s = format!("P{position:02} / 08\n\n");
                for (i, r) in sorted.iter().enumerate() {
                    let marker = if r.id == 0 { ">" } else { " " };
                    s.push_str(&format!("{marker} {}  {:5}\n", i + 1, NAMES[r.id]));
                }
                s
            },
            1 => format!("LAP {:02} / {:02}    {:02}:{:05.2}", ((player.progress.max(0.0) / track.length) as u32 + 1).min(LAPS), LAPS, (race.elapsed / 60.0) as u32, race.elapsed % 60.0),
            2 => format!("{:03}", (player.speed * 3.6) as u32),
            3 => format!("ARMOR  {}  {:3.0}%\nNITRO  {}  {:3.0}%", bar(player.health), player.health.max(0.0), bar(player.nitro), player.nitro),
            4 => if race.phase == Phase::Countdown { format!("{}", race.countdown.ceil() as u32) } else if player.crashed > 0.0 { format!("BACK ON THE ROAD IN {:.1}", player.crashed) } else if race.message_time > 0.0 { race.message.clone() } else { String::new() },
            5 => format!("{:.2} KM  /  {} LAPS", track.length / 1000.0, LAPS),
            6 => match race.phase { Phase::Paused => "TAKE A BREATHER".into(), Phase::Finished => format!("FINISHED / P{} OF 8", race.place), _ => "01 / PACIFIC RUN".into() },
            7 => match race.phase {
                Phase::Finished => format!("TIME  {:02}:{:05.2}     TAKEDOWNS  {}\n\n+{} CREDITS\nPress Enter to race again.", (race.elapsed / 60.0) as u32, race.elapsed % 60.0, race.takedowns, race.earned),
                Phase::Paused => "Race paused.\n\nPress Esc to get back on the road.\nPress R for a fresh grid.".into(),
                _ => "Eight riders. Two laps. One stretch of coast.\n\nCarve the bends. Dodge oncoming traffic.\nFight your way to the front.".into(),
            },
            8 => if race.phase == Phase::Paused { "ESC / BACK TO IT".into() } else { "ENTER / RIDE OUT".into() },
            9 => format!("GARAGE   /   {} CREDITS   /   {} WINS IN {} RACES", profile.credits, profile.wins, profile.races),
            _ => String::new(),
        };
    }
    for (dot, mut node) in &mut dots {
        if let Some(r) = riders.iter().find(|r| r.id == dot.0) {
            let (x, y) = map_point(track.sample(r.progress, 0.0).0);
            node.left = px(x - 2.0);
            node.top = px(y - 2.0);
        }
    }
}

fn bar(amount: f32) -> String {
    let filled = (amount.clamp(0.0, 100.0) / 10.0).round() as usize;
    format!("{}{}", "|".repeat(filled), ".".repeat(10 - filled))
}

#[derive(Resource, Default)]
pub struct MenuAction(pub bool);

/// Run before input so mouse activation follows the same menu actions.
pub fn buttons(
    buttons: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut action: ResMut<MenuAction>,
) {
    for interaction in &buttons {
        if *interaction == Interaction::Pressed {
            action.0 = true;
        }
    }
}
