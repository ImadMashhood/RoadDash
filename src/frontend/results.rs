//! Offline race standings and post-race controls.
use super::*;
use crate::game::{NAMES, Rider};
use rr_core::race::WasteCause;

pub(super) fn can_continue(race: &Race) -> bool {
    race.race_in_progress && !race.arrested
}

pub(super) fn entries(race: &Race) -> Vec<Entry> {
    let mut buttons = vec![
        entry("FINISH", Action::Back),
        entry("RESTART", Action::Ride),
    ];
    if can_continue(race) {
        buttons.push(entry("KEEP RACE RUNNING", Action::ContinueRace));
    }
    buttons
}

fn standings<'a>(riders: impl Iterator<Item = &'a Rider>) -> Vec<&'a Rider> {
    let mut riders: Vec<_> = riders.filter(|r| !r.police).collect();
    riders.sort_by(|a, b| a.race_rank().standings_cmp(b.race_rank()));
    riders
}

pub(super) fn render<'a>(
    commands: &mut Commands,
    canvas: Entity,
    race: &Race,
    riders: impl Iterator<Item = &'a Rider>,
) {
    let riders = standings(riders);
    let player_time = riders
        .iter()
        .find(|rider| rider.id == 0)
        .and_then(|rider| rider.finished)
        .unwrap_or(race.elapsed);
    let wasted = race.player_wasted.map(|(cause, _)| cause);
    let heading = if wasted.is_some() {
        "WASTED"
    } else if race.arrested {
        "BUSTED"
    } else {
        "RACE STANDINGS"
    };
    let accent = Color::srgb(1., 0.28, 0.22);
    let panel = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                ..default()
            },
            BackgroundColor(Color::srgba(0.025, 0.035, 0.05, 0.97)),
        ))
        .id();
    commands.entity(canvas).add_child(panel);
    let heading_node = text(
        commands,
        heading,
        42.,
        Node {
            position_type: PositionType::Absolute,
            left: percent(5),
            top: percent(5),
            ..default()
        },
        if wasted.is_some() {
            accent
        } else {
            layout::PAPER
        },
    );
    let summary = match wasted {
        Some(WasteCause::Bike) => "BIKE DESTROYED".into(),
        Some(WasteCause::Rider) => "RIDER INJURED".into(),
        Some(WasteCause::River) => "CRASHED INTO THE RIVER".into(),
        None if race.arrested => "ARRESTED / NO PAYOUT".into(),
        None => format!(
            "P{}     {:02}:{:05.2}     +{} CREDITS",
            race.place,
            (player_time / 60.) as u32,
            player_time % 60.,
            race.earned
        ),
    };
    let summary_node = text(
        commands,
        summary,
        17.,
        Node {
            position_type: PositionType::Absolute,
            left: percent(5),
            top: percent(15),
            ..default()
        },
        layout::PAPER.with_alpha(0.82),
    );
    commands
        .entity(panel)
        .add_children(&[heading_node, summary_node]);

    for (i, rider) in riders.into_iter().enumerate() {
        let wasted = rider.wasted.is_some();
        let status = if wasted {
            "   Wasted".into()
        } else if let Some(time) = rider.finished {
            format!("   {:02}:{:05.2}", (time / 60.) as u32, time % 60.)
        } else if rider.id == 0 && race.arrested {
            "   Busted".into()
        } else {
            "   Racing".into()
        };
        let name = if rider.id == 0 {
            "You"
        } else {
            NAMES[rider.roster_slot]
        };
        let row = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(5. + (i / 8) as f32 * 46.),
                    top: percent(23. + (i % 8) as f32 * 6.5),
                    width: percent(43),
                    height: percent(5.9),
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(px(10)),
                    ..default()
                },
                BackgroundColor(Color::srgba(1., 1., 1., 0.055)),
            ))
            .id();
        let line = text(
            commands,
            format!("{:02}   {name}{status}", i + 1),
            20.,
            Node::default(),
            if wasted {
                Color::srgb(0.50, 0.50, 0.52)
            } else {
                layout::PAPER
            },
        );
        commands.entity(row).add_child(line);
        if wasted {
            let strike = commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: percent(50),
                        left: px(8),
                        right: px(8),
                        height: px(2),
                        ..default()
                    },
                    BackgroundColor(accent),
                ))
                .id();
            commands.entity(row).add_child(strike);
        }
        commands.entity(panel).add_child(row);
    }
    let buttons = entries(race);
    let width = 92. / buttons.len() as f32;
    for (i, button) in buttons.iter().enumerate() {
        layout::button(
            commands,
            panel,
            i,
            button,
            [5. + i as f32 * width, 82., width - 2., 10.],
            false,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasted_riders_follow_classified_and_active_racers() {
        let mut riders = [Rider::new(0), Rider::new(1), Rider::new(2), Rider::new(3)];
        riders[0].progress = 200.;
        riders[1].finished = Some(15.);
        riders[2].progress = 250.;
        riders[2].wasted = Some(WasteCause::River);
        riders[3].progress = 150.;
        assert_eq!(
            standings(riders.iter())
                .iter()
                .map(|rider| rider.id)
                .collect::<Vec<_>>(),
            [1, 0, 3, 2]
        );
    }
}
