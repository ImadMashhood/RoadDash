use crate::{
    game::{Phase, Profile, Race, Rider},
    track::{LAPS, Track},
};
use bevy::{prelude::*, ui::Pressed, ui_widgets::Button};

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
pub(crate) struct RivalArrow;
#[derive(Component)]
pub(crate) struct SpeedFill;
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
                font_size: FontSize::Px(size),
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
        &track.definition.name,
        12.0,
        MINT,
        33.0,
        58.0,
    ));
    kids.push(
        commands
            .spawn((
                Text::default(),
                TextFont {
                    font_size: FontSize::Px(31.),
                    ..default()
                },
                TextColor(WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(34),
                    bottom: px(95),
                    ..default()
                },
                Readout(0),
            ))
            .id(),
    );
    let rival = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(34),
                bottom: px(64),
                align_items: AlignItems::Center,
                column_gap: px(10),
                ..default()
            },
            children![
                (
                    Text::default(),
                    TextFont {
                        font_size: FontSize::Px(19.),
                        ..default()
                    },
                    TextColor(WHITE),
                    Readout(5)
                ),
                (
                    Node {
                        width: px(10),
                        height: px(10),
                        border: UiRect {
                            left: px(3),
                            top: px(3),
                            ..default()
                        },
                        display: Display::None,
                        ..default()
                    },
                    BorderColor::all(Color::NONE),
                    RivalArrow,
                )
            ],
        ))
        .id();
    kids.push(rival);
    let lap = label(&mut commands, "", 17.0, WHITE, 34.0, 84.0);
    commands.entity(lap).insert(Readout(1));
    kids.push(lap);
    let speed = commands
        .spawn((
            Text::new("000"),
            TextFont {
                font_size: FontSize::Px(80.0),
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
            Readout(12),
            TextFont {
                font_size: FontSize::Px(13.0),
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
    let voice = commands
        .spawn((
            Text::default(),
            TextFont {
                font_size: FontSize::Px(14.),
                ..default()
            },
            TextColor(MINT),
            Readout(13),
            Node {
                position_type: PositionType::Absolute,
                left: px(34.),
                bottom: px(50.),
                max_width: percent(60.),
                ..default()
            },
        ))
        .id();
    kids.push(voice);
    let status = commands
        .spawn((
            Text::new(""),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            },
            TextColor(WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: px(34),
                top: px(118),
                ..default()
            },
            Readout(3),
        ))
        .id();
    kids.push(status);
    let help = commands
        .spawn((
            Text::new("WASD  RIDE     NITRO  DISABLED     MOUSE  PUNCH   SPACE  KICK   E PICKUP / Q DROP     C  CAMERA     ESC  PAUSE     M  MUTE"),
            TextFont {
                font_size: FontSize::Px(12.0),
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
                font_size: FontSize::Px(24.0),
                ..default()
            },
            TextColor(MINT),
            TextLayout::justify(Justify::Center),
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
    let meter = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: px(50),
                right: px(38),
                width: px(152),
                height: px(5),
                ..default()
            },
            BackgroundColor(INK.with_alpha(0.83)),
        ))
        .id();
    let fill = commands
        .spawn((
            Node {
                width: percent(0),
                height: percent(100),
                ..default()
            },
            BackgroundColor(MINT),
            SpeedFill,
        ))
        .id();
    commands.entity(meter).add_child(fill);
    kids.push(meter);
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
                border_radius: BorderRadius::all(px(4)),
                ..default()
            },
            BackgroundColor(MINT),
            StartButton,
        ))
        .id();
    let text = commands
        .spawn((
            Text::new("ENTER / RIDE OUT"),
            TextFont {
                font_size: FontSize::Px(21.0),
                ..default()
            },
            TextColor(INK),
            Readout(8),
        ))
        .id();
    commands.entity(button).add_child(text);
    menu.push(button);
    menu.push(label(&mut commands, "W / S   THROTTLE & BRAKE     A / D   STEER\nNITRO DISABLED     MOUSE / J / K   PUNCH    SPACE   KICK    E PICKUP / Q DROP\nC   CAMERA     R   RESTART     ESC   PAUSE     M   MUTE", 14.0, MUTED, 70.0, 661.0));
    let profile = label(&mut commands, "", 13.0, MINT, 70.0, 752.0);
    commands.entity(profile).insert(Readout(9));
    menu.push(profile);
    commands.entity(overlay).add_children(&menu);
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn update_ui(
    race: Res<Race>,
    profile: Res<Profile>,
    screen: Option<Res<State<crate::runtime::AppScreen>>>,
    track: Res<Track>,
    riders: Query<&Rider>,
    recoveries: Query<(&Rider, &crate::recovery::Recovery)>,
    positions: Query<&Transform>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    mut readouts: Query<(&Readout, &mut Text)>,
    mut nodes: Query<
        (&mut Node, Has<Hud>, Has<Overlay>),
        (
            Or<(With<Hud>, With<Overlay>)>,
            Without<RivalArrow>,
            Without<SpeedFill>,
        ),
    >,
    mut arrows: Query<
        (&mut Node, &mut BorderColor, &mut UiTransform),
        (With<RivalArrow>, Without<SpeedFill>),
    >,
    mut speed_fills: Query<&mut Node, (With<SpeedFill>, Without<RivalArrow>)>,
    panel: Res<crate::settings::SettingsPanel>,
    police: Res<crate::police::Police>,
    frontend: Option<Res<crate::frontend::Frontend>>,
    (online, online_ui, prefs): (
        Option<Res<crate::online::NetState>>,
        Option<Res<crate::online::Online>>,
        Res<crate::settings::Preferences>,
    ),
) {
    let local_slot = online.as_ref().map_or(0, |s| s.local_slot);
    let Some(player) = riders.iter().find(|r| r.id == local_slot) else {
        return;
    };
    let loading = screen.as_ref().is_some_and(|s| {
        matches!(
            s.get(),
            crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
        )
    });
    let recovery = recoveries
        .iter()
        .find(|(r, _)| r.id == local_slot)
        .map(|(_, recovery)| recovery.stage)
        .unwrap_or_default();
    let menu = matches!(race.phase, Phase::Garage | Phase::Paused | Phase::Finished);
    for (mut node, hud, overlay) in &mut nodes {
        if hud {
            node.display = if menu || panel.open {
                Display::None
            } else {
                Display::Flex
            };
        }
        if overlay {
            node.display =
                if menu && !panel.open && !frontend.as_ref().is_some_and(|f| f.owns(race.phase)) {
                    Display::Flex
                } else {
                    Display::None
                };
        }
    }
    let position = 1 + riders
        .iter()
        .filter(|r| {
            !r.police
                && r.id != local_slot
                && ((!r.online_busted && player.online_busted)
                    || (r.online_busted == player.online_busted && r.progress > player.progress))
        })
        .count();
    let count = riders.iter().filter(|r| !r.police).count();
    let nearest = nearest_rival(player, riders.iter());
    for (mut node, mut color, mut transform) in &mut arrows {
        let display = if nearest.is_some() {
            Display::Flex
        } else {
            Display::None
        };
        if node.display != display {
            node.display = display;
        }
        if let Some((_, delta)) = nearest {
            let ahead = delta >= 0.;
            let tint = if ahead {
                Color::srgb(1., 0.22, 0.20)
            } else {
                Color::srgb(0.29, 0.94, 0.35)
            };
            color.set_if_neq(BorderColor::all(tint));
            transform.set_if_neq(UiTransform::from_rotation(Rot2::radians(if ahead {
                45_f32.to_radians()
            } else {
                225_f32.to_radians()
            })));
        }
    }
    for mut fill in &mut speed_fills {
        let width = percent(100. * (player.speed / 75.).clamp(0., 1.));
        if fill.width != width {
            fill.width = width;
        }
    }
    for (field, mut text) in &mut readouts {
        let next = match field.0 {
            0 => format!("P{position:02} / {count:02}"),
            1 if !track.definition.closed => format!("ROUTE {:03}%  {:02}:{:05.2}",(track.race_interval().fraction(player.progress)*100.) as u32,(race.elapsed/60.) as u32,race.elapsed%60.),
            1 => format!("LAP {:02} / {:02}    {:02}:{:05.2}", ((player.progress.max(0.0) / track.length) as u32 + 1).min(LAPS), LAPS, (race.elapsed / 60.0) as u32, race.elapsed % 60.0),
            2 => {
                let multiplier = if prefs.file.imperial_speed { 2.236_936 } else { 3.6 };
                format!("{:03}", (player.speed * multiplier) as u32)
            }
            12 => if prefs.file.imperial_speed { "MPH   /   6-SPEED" } else { "KM/H   /   6-SPEED" }.into(),
            13 => online_ui.as_ref().map_or_else(String::new, |o| o.voice_status.clone()),
            3 => format!("ARMOR  {}  {:3.0}%\nNITRO DISABLED", bar(player.health), player.health.max(0.0)) + &player.weapon.map_or_else(|| "\nUNARMED".to_owned(), |w| format!("\n{} EQUIPPED", w.label())) + &format!("\n{}", police.hud()),
            4 if track.metropolis.is_some() && player.progress>=track.race_interval().finish && player.finished.is_none() => "ROUTE INCOMPLETE / R TO RETRY".into(),
            4 if !track.definition.closed && track.metropolis.is_none() && player.progress>=track.race_interval().finish && player.finished.is_none() => format!("MISSED GATE {} / R TO RETRY",player.gates.next),
            4 if player.finished_on_foot => "FINISHED ON FOOT".into(),
            4 if recoveries.iter().any(|(r,s)|r.id==0 && s.stage==crate::recovery::RecoveryStage::Running) => {
                let hint = recoveries.iter().find(|(r,_)|r.id==0).and_then(|(_,s)|s.runner.zip(s.wreck))
                    .and_then(|(r,b)|positions.get(r).ok().zip(positions.get(b).ok()))
                    .map(|(r,b)| {
                        let delta=(b.translation-r.translation).with_y(0.0);
                        let forward=cameras.iter().next().map_or(Vec3::Z,|c|*c.forward()).with_y(0.0).normalize_or_zero();
                        let side=delta.dot(forward.cross(Vec3::Y)); let ahead=delta.dot(forward);
                        let direction=if side.abs()>ahead.abs() {if side>0.0 {"RIGHT >"}else{"< LEFT"}} else if ahead>0.0 {"AHEAD ^"}else{"BEHIND v"};
                        format!("BIKE {direction} / {:.0} M",delta.length())
                    }).unwrap_or_else(||"FIND YOUR BIKE".into());
                format!("{hint} / WASD MOVE / ENTER MOUNT")
            },
            4 => if race.phase == Phase::Countdown { format!("{}", race.countdown.ceil() as u32) } else if player.crashed > 0.0 { match recovery { crate::recovery::RecoveryStage::Falling => "WIPED OUT / WAIT FOR THE LANDING", crate::recovery::RecoveryStage::GettingUp => "GETTING BACK UP", crate::recovery::RecoveryStage::Running => "GET BACK TO THE BIKE", crate::recovery::RecoveryStage::Mounting => "BACK IN THE SADDLE", _ => "" }.into() } else if race.message_time > 0.0 { race.message.clone() } else { String::new() },
            5 => nearest.map_or_else(String::new, |(rival, gap)| {
                let name = if rival.roster_slot == 0 { rr_core::cast::FULL_NAMES[0] } else { crate::game::NAMES[rival.roster_slot] };
                format!("{name}  {:.0} m", gap.abs())
            }),
            6 if loading => "PREPARING THE ROAD".into(),
            7 if loading => "Getting your bike ready...".into(),
            8 if loading => "LOADING...".into(),
            6 if race.arrested => "BUSTED / R TO RETRY".into(),
            7 if race.arrested => "You were caught after a sustained police pursuit.\n\nNo payout. Press Enter for a fresh grid.".into(),
            6 => match race.phase { Phase::Paused => "TAKE A BREATHER".into(), Phase::Finished => format!("FINISHED / P{} OF 15", race.place), _ => track.definition.name.clone() },
            7 => match race.phase {
                Phase::Finished => format!("TIME  {:02}:{:05.2}     TAKEDOWNS  {}\n\n+{} CREDITS\nPress Enter to race again.", (race.elapsed / 60.0) as u32, race.elapsed % 60.0, race.takedowns, race.earned),
                Phase::Paused => "Race paused.\n\nPress Esc to get back on the road.\nPress R for a fresh grid.\nF2 / controller LB: comfort & controls.".into(),
                _ if track.is_hills()=>"Fifteen riders. City to winding hills.\n\nChoose either western town street.\nRejoin inside town; finish in the dunes.\nF2 / controller LB: comfort & controls.".into(),
                _ if track.metropolis.is_some()=>"Fifteen riders. Farms to downtown.\n\nChoose Old Market or Station Avenue.\nRace to the finish in the city.\nF2 / controller LB: comfort & controls.".into(),
                _ => "Eight riders. Farmland to city to hills.\n\nPass every gate. Dodge cross traffic.\nFight your way to the summit.\nF2 / controller LB: comfort & controls.".into(),
            },
            8 => if race.phase == Phase::Paused { "ESC / BACK TO IT".into() } else { "ENTER / RIDE OUT".into() },
            9 => format!("GARAGE   /   {} CREDITS   /   {} WINS IN {} RACES", profile.credits, profile.wins, profile.races),
            _ => String::new(),
        };
        text.set_if_neq(Text::new(next));
    }
}

fn bar(amount: f32) -> String {
    let filled = (amount.clamp(0.0, 100.0) / 10.0).round() as usize;
    format!("{}{}", "|".repeat(filled), ".".repeat(10 - filled))
}

fn nearest_rival<'a>(
    player: &Rider,
    riders: impl Iterator<Item = &'a Rider>,
) -> Option<(&'a Rider, f32)> {
    riders
        .filter(|rider| rider.id != player.id && !rider.police && !rider.online_busted)
        .map(|rider| (rider, rider.progress - player.progress))
        .min_by(|(a, da), (b, db)| da.abs().total_cmp(&db.abs()).then_with(|| a.id.cmp(&b.id)))
}

#[derive(Resource, Default)]
pub struct MenuAction(pub bool);

/// Run before input so mouse activation follows the same menu actions.
pub fn buttons(
    buttons: Query<Entity, (Added<Pressed>, With<StartButton>)>,
    mut action: ResMut<MenuAction>,
) {
    if !buttons.is_empty() {
        action.0 = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn race_hud_updates_speed_position_and_arrow_in_the_same_frame() {
        use bevy::ecs::system::RunSystemOnce;
        let mut app = crate::game::tests::simulation();
        let world = app.world_mut();
        world.resource_mut::<Race>().phase = Phase::Racing;
        world
            .resource_mut::<crate::settings::Preferences>()
            .file
            .imperial_speed = false;
        let mut player = world.query::<&mut Rider>().single_mut(world).unwrap();
        player.progress = 500.;
        player.speed = 25.;
        let mut rival = Rider::new(1);
        rival.progress = 530.;
        let entity = world.spawn(rival).id();
        world.run_system_once(setup_ui).unwrap();
        for (progress, position, rotation) in [(530., "P02 / 02", 45_f32), (470., "P01 / 02", 225.)]
        {
            world.get_mut::<Rider>(entity).unwrap().progress = progress;
            world.run_system_once(update_ui).unwrap();
            let mut readouts = world.query::<(&Readout, &Text)>();
            assert!(
                readouts
                    .iter(world)
                    .any(|(id, text)| id.0 == 0 && text.0 == position)
            );
            assert!(
                readouts
                    .iter(world)
                    .any(|(id, text)| id.0 == 2 && text.0 == "090")
            );
            assert!(
                readouts
                    .iter(world)
                    .any(|(id, text)| id.0 == 5 && text.0.ends_with("30 m"))
            );
            let mut arrows = world.query_filtered::<&UiTransform, With<RivalArrow>>();
            assert_eq!(
                *arrows.single(world).unwrap(),
                UiTransform::from_rotation(Rot2::radians(rotation.to_radians())),
            );
        }
    }

    #[test]
    fn nearest_rival_keeps_the_signed_gap_and_excludes_police() {
        let mut player = Rider::new(0);
        player.progress = 500.;
        let mut ahead = Rider::new(1);
        ahead.progress = 550.;
        let mut behind = Rider::new(2);
        behind.progress = 470.;
        let mut officer = Rider::new(3);
        officer.police = true;
        officer.progress = 501.;
        let closest =
            nearest_rival(&player, [&player, &ahead, &behind, &officer].into_iter()).unwrap();
        assert_eq!((closest.0.id, closest.1), (2, -30.));
        let closest = nearest_rival(&player, [&ahead].into_iter()).unwrap();
        assert_eq!((closest.0.id, closest.1), (1, 50.));
    }
}
