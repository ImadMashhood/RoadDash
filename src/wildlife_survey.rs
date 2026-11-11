//! Actual native herd motion/LOD receipts. Frame wall time is not GPU timing.
use crate::{
    game::{ChaseCamera, Phase, Race},
    track::Track,
    world_gen::hills::wildlife::{self, WildAnimal, Wildlife},
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};
use bevy_rapier3d::prelude::Collider;
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    captures: u32,
    last: Option<std::time::Instant>,
    wall: Vec<f64>,
    cpu: Vec<f64>,
    first: Option<Vec3>,
    moved: f32,
    max_active: usize,
    max_limbs: usize,
    max_meshes: usize,
}
fn captured(_: On<ScreenshotCaptured>, mut s: ResMut<Survey>) {
    s.captures += 1;
}
#[allow(clippy::too_many_arguments)]
pub fn run(
    mut commands: Commands,
    mut s: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    mut race: ResMut<Race>,
    wild: Res<Wildlife>,
    animals: Query<(&WildAnimal, &Transform, Option<&Collider>), Without<ChaseCamera>>,
    mut camera: Query<&mut Transform, With<ChaseCamera>>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    s.frame += 1;
    let frame = s.frame;
    let now = std::time::Instant::now();
    if let Some(last) = s.last
        && frame > 120
    {
        s.wall.push(now.duration_since(last).as_secs_f64() * 1000.);
        s.cpu.push(wild.update_micros);
    }
    s.last = Some(now);
    race.phase = Phase::Racing;
    for (e, parent) in &ui {
        if parent.is_none() {
            commands.entity(e).insert(Visibility::Hidden);
        }
    }
    let enabled = std::env::var("ROADDASH_WILDLIFE").as_deref() != Ok("0");
    assert_eq!(
        animals.iter().count(),
        if enabled {
            rr_core::wildlife::POPULATION
        } else {
            0
        }
    );
    for (animal, t, c) in &animals {
        assert!(
            c.is_none(),
            "scenic wildlife must not silently become collision hazards"
        );
        if animal.herd == 0 && animal.member == 0 && frame > 5 && frame < 300 {
            let first = *s.first.get_or_insert(t.translation);
            s.moved = s.moved.max(first.distance(t.translation));
        }
    }
    s.max_active = s.max_active.max(wild.active_animals);
    s.max_limbs = s.max_limbs.max(wild.animated_limbs);
    s.max_meshes = s.max_meshes.max(wild.visible_meshes);
    let perf = std::env::var("ROADDASH_WILDLIFE_PERF").is_ok();
    let (herd, distance) = if perf || frame < 300 {
        (0, 24.)
    } else if frame < 550 {
        (3, 24.)
    } else if frame < 700 {
        (0, 180.)
    } else if frame < 850 {
        (0, 460.)
    } else {
        ((frame as usize - 850) / 50 % 6, 80.)
    };
    let (p, q) = wildlife::center(&track, herd);
    let target = p.with_y(wildlife::ground_point(&track, herd, 0, 0.).y + 1.);
    if let Ok(mut c) = camera.single_mut() {
        *c = Transform::from_translation(
            target + q * Vec3::new(distance, distance * 0.25 + 3., distance * 0.25),
        )
        .looking_at(target, Vec3::Y);
    }
    if !perf && [120, 240, 380, 500, 640, 790, 900, 1000].contains(&frame) {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/wildlife-{frame}.png")))
            .observe(captured);
    }
    if frame == 1100 {
        if enabled {
            assert!(s.moved > 1., "native herd did not move");
            assert!(s.max_limbs > 0);
        }
        if !perf {
            assert_eq!(s.captures, 8);
        }
        s.wall.sort_by(f64::total_cmp);
        s.cpu.sort_by(f64::total_cmp);
        let n = s.wall.len();
        let report = serde_json::json!({"complete":true,"enabled":enabled,"frames":frame,"captures":s.captures,"population":animals.iter().count(),"movement_m":s.moved,"max_active_animals":s.max_active,"max_animated_limbs":s.max_limbs,"max_visible_meshes":s.max_meshes,"wall_median_ms":s.wall[n/2],"wall_p95_ms":s.wall[n*95/100],"wildlife_cpu_median_us":s.cpu[n/2],"wildlife_cpu_p95_us":s.cpu[n*95/100],"contact_policy":"scenic-only; no road crossings or colliders","gpu_timing":false});
        std::fs::write(
            "screenshots/wildlife.json",
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .unwrap();
        info!("WILDLIFE_SURVEY_COMPLETE {report}");
        exit.write(AppExit::Success);
    }
}
