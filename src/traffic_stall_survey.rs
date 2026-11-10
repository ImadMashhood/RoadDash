//! Opt-in rendered regression for a compressed civilian-traffic queue.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

#[derive(Resource, Default)]
struct Survey {
    frame: u32,
    selected: Vec<Entity>,
    cases: Vec<serde_json::Value>,
}

pub(crate) fn install(app: &mut App) {
    app.insert_resource(bevy::winit::WinitSettings::continuous())
        .init_resource::<Survey>()
        .add_systems(
            PostUpdate,
            run.after(super::camera)
                .before(bevy::transform::TransformSystems::Propagate),
        );
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    mut race: ResMut<Race>,
    mut cars: Query<
        (Entity, &mut Traffic, &mut TrafficYield, &mut Transform),
        (Without<Rider>, Without<Camera3d>),
    >,
    mut riders: Query<&mut Rider>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    assert!(
        track.metropolis.is_some(),
        "traffic stall fixture requires Metropolis"
    );
    state.frame += 1;
    let case = (state.frame - 1) / 1500;
    let tick = (state.frame - 1) % 1500 + 1;
    let direction = if case == 0 { 1. } else { -1. };
    race.phase = if tick <= 300 {
        Phase::Paused
    } else {
        Phase::Racing
    };
    for mut rider in &mut riders {
        rider.finished = Some(0.);
    }
    for (entity, parent) in &ui {
        if parent.is_none() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    if tick == 1 {
        race.elapsed = 10.;
        let mut ids: Vec<_> = cars.iter().map(|x| x.0).collect();
        ids.sort();
        assert!(ids.len() >= 3);
        state.selected = ids[..3].to_vec();
        for (i, entity) in ids.into_iter().enumerate() {
            let (_, mut car, mut yielding, mut pose) = cars.get_mut(entity).unwrap();
            let progress = if i < 3 {
                2900. + direction * i as f32 * 0.04
            } else {
                8500. + i as f32 * 12.
            };
            car.route = 0;
            car.progress = progress;
            car.speed = 0.;
            *yielding = TrafficYield {
                cruise: Some(if i < 3 { direction * 18. } else { 0. }),
                lane_slot: 0,
                start_progress: Some(progress),
                ..default()
            };
            car.lane = rr_core::streets::lane_center(
                &track.metropolis.as_ref().unwrap().street_life.lane_sections,
                0,
                progress,
                direction > 0.,
                0,
            );
            let (p, q) = track.street_sample(0, progress, car.lane);
            *pose = Transform::from_translation(p).with_rotation(
                q * if direction < 0. {
                    Quat::from_rotation_y(std::f32::consts::PI)
                } else {
                    Quat::IDENTITY
                },
            );
        }
    }
    let center = state
        .selected
        .iter()
        .map(|e| cars.get(*e).unwrap().3.translation)
        .sum::<Vec3>()
        / 3.;
    let progress = state
        .selected
        .iter()
        .map(|e| cars.get(*e).unwrap().1.progress)
        .sum::<f32>()
        / 3.;
    let q = track.sample_route(0, progress, 0.).1;
    for mut camera in &mut cameras {
        *camera = Transform::from_translation(
            center - q * Vec3::Z * direction * 25. + q * Vec3::X * 10. + Vec3::Y * 12.,
        )
        .looking_at(center + Vec3::Y, Vec3::Y);
    }
    if [280, 1400].contains(&tick) {
        let phase = if tick == 280 { "compressed" } else { "moving" };
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/traffic-{case}-{phase}.png"
            )));
    }
    if tick == 1500 {
        let results: Vec<_> = state
            .selected
            .iter()
            .map(|e| {
                let (_, c, _, _) = cars.get(*e).unwrap();
                assert!(
                    (c.progress - 2900.) * direction > 20. && c.speed * direction > 1.,
                    "compressed traffic queue failed to drain"
                );
                serde_json::json!({"station": c.progress, "speed_mps": c.speed})
            })
            .collect();
        state.cases.push(
            serde_json::json!({"direction": direction, "queue_drained": true, "cars": results}),
        );
        if case == 1 {
            std::fs::create_dir_all("screenshots").unwrap();
            std::fs::write("screenshots/traffic-stall.json", serde_json::to_string_pretty(&serde_json::json!({"cases":state.cases,"frames":state.frame,"limits":"Deliberately compressed production car queue; racers and other traffic excluded from the scenario. Functional rendering/queue evidence, not performance or whole-race coverage."})).unwrap()).unwrap();
            exit.write(AppExit::Success);
        }
    }
}
