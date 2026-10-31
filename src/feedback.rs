//! Bounded screen-space traction and optional peripheral speed cues.
use bevy::prelude::*;
#[derive(Component)]
pub struct Traction;
#[derive(Component)]
pub struct SpeedLine(pub u8);
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Traction,
        Text::new(""),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.75, 0.3)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(32),
            right: px(36),
            ..default()
        },
    ));
    for i in 0..12 {
        commands.spawn((
            SpeedLine(i),
            Node {
                position_type: PositionType::Absolute,
                width: px(2),
                height: px(30),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.8, 0.95, 1.0, 0.25)),
        ));
    }
}
#[allow(clippy::type_complexity)]
pub fn update(
    time: Res<Time>,
    race: Res<crate::game::Race>,
    prefs: Res<crate::settings::Preferences>,
    panel: Res<crate::settings::SettingsPanel>,
    racers: Query<(
        &crate::game::Rider,
        Option<&crate::vehicle::BikeState>,
        &crate::recovery::Recovery,
    )>,
    mut labels: Query<&mut Text, With<Traction>>,
    mut lines: Query<(&SpeedLine, &mut Node)>,
) {
    let player = racers.iter().find(|(r, _, _)| r.id == 0);
    let visible = race.phase == crate::game::Phase::Racing && !panel.open;
    let riding = player.is_some_and(|(_, _, s)| s.stage == crate::recovery::RecoveryStage::Riding);
    for mut text in &mut labels {
        **text = if visible && riding {
            player.and_then(|(_, b, _)| b).map_or(String::new(), |b| {
                if !b.grounded.iter().any(|v| *v) {
                    "AIRBORNE".into()
                } else if b.slip > 0.15 {
                    format!("TRACTION LIMIT / SLIP {:02}%", (b.slip * 100.0) as u32)
                } else if b.slip_angle.abs() > 0.12 {
                    format!(
                        "SLIDING / SIDE ANGLE {:.0} DEG",
                        b.slip_angle.abs().to_degrees()
                    )
                } else {
                    String::new()
                }
            })
        } else {
            String::new()
        };
    }
    let speed = player.map_or(0.0, |(r, _, _)| r.speed);
    for (line, mut node) in &mut lines {
        let show = visible
            && riding
            && speed > 35.0
            && prefs.file.comfort.speed_lines
            && !prefs.file.comfort.anti_nausea;
        node.display = if show { Display::Flex } else { Display::None };
        let i = line.0;
        let t = (time.elapsed_secs() * speed * 0.008 + (i / 2) as f32 / 6.0).fract();
        node.left = percent(if i % 2 == 0 {
            4.0 + 4.0 * t
        } else {
            92.0 - 4.0 * t
        });
        node.top = percent(12.0 + t * 70.0);
        node.height = px(12.0 + (speed - 35.0).max(0.0) * 0.8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn traction_is_readable_and_speed_cues_stay_bounded_and_respect_comfort() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<crate::settings::Preferences>()
            .init_resource::<crate::settings::SettingsPanel>()
            .init_resource::<crate::game::Race>()
            .add_systems(Startup, setup)
            .add_systems(Update, update);
        app.world_mut().resource_mut::<crate::game::Race>().phase = crate::game::Phase::Racing;
        app.world_mut()
            .resource_mut::<crate::settings::Preferences>()
            .file
            .comfort
            .speed_lines = true;
        let mut r = crate::game::Rider::new(0);
        r.speed = 50.0;
        app.world_mut().spawn((
            r,
            crate::vehicle::BikeState {
                grounded: [true; 2],
                slip: 0.5,
                ..default()
            },
            crate::recovery::Recovery::default(),
        ));
        for _ in 0..100 {
            app.update();
        }
        let world = app.world_mut();
        let text = world
            .query_filtered::<&Text, With<Traction>>()
            .single(world)
            .unwrap();
        assert!(text.0.contains("TRACTION LIMIT") && text.0.contains("50%"));
        assert_eq!(
            world
                .query_filtered::<&Node, With<SpeedLine>>()
                .iter(world)
                .filter(|n| n.display == Display::Flex)
                .count(),
            12
        );
        world
            .resource_mut::<crate::settings::Preferences>()
            .file
            .comfort
            .anti_nausea = true;
        app.update();
        let world = app.world_mut();
        assert_eq!(
            world
                .query_filtered::<&SpeedLine, With<Node>>()
                .iter(world)
                .count(),
            12
        );
        assert!(
            world
                .query_filtered::<&Node, With<SpeedLine>>()
                .iter(world)
                .all(|n| n.display == Display::None)
        );
    }
}
