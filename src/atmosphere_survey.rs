//! Opt-in rendered atmosphere review; never installed in ordinary gameplay.
use crate::{
    game::{ChaseCamera, Phase, Race},
    track::Track,
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};
#[derive(Resource, Default)]
pub struct Survey {
    frame: usize,
    captures: usize,
    previous: Option<std::time::Instant>,
    samples: Vec<f64>,
}
fn captured(_: On<ScreenshotCaptured>, mut state: ResMut<Survey>) {
    state.captures += 1;
}
#[allow(clippy::too_many_arguments)]
pub fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    mut race: ResMut<Race>,
    mut camera: Query<&mut Transform, With<ChaseCamera>>,
    mut windows: Query<&mut Window>,
    ui: Query<Entity, (With<Node>, Without<ChildOf>)>,
    budget: Res<crate::world_gen::hills::clouds::CloudBudget>,
    birds: Query<Entity, With<crate::world_gen::hills::atmosphere::Bird>>,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    let now = std::time::Instant::now();
    if state.frame >= 180
        && let Some(previous) = state.previous
    {
        state
            .samples
            .push(now.duration_since(previous).as_secs_f64() * 1000.);
    }
    state.previous = Some(now);
    state.frame += 1;
    let frame = state.frame;
    if frame == 1 {
        for mut window in &mut windows {
            window.present_mode = bevy::window::PresentMode::AutoNoVsync;
            window.resolution.set(1440., 900.);
        }
        race.phase = Phase::Racing;
        for entity in &ui {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    let perf = std::env::var("ROADDASH_ATMOSPHERE_PERF").as_deref() == Ok("1");
    let segment = (frame.saturating_sub(180) / 180).min(5);
    let station = if track.is_hills() {
        [2020., 2120., 5700., 5800., 9600., 9700.][segment]
    } else {
        [920., 1020., 4520., 4620., 8320., 8420.][segment]
    };
    let travel = (frame.saturating_sub(180) % 180) as f32 * 0.5;
    let (p, q) = track.sample_route(0, station + travel, 0.);
    if let Ok(mut camera) = camera.single_mut() {
        let eye = p - q * Vec3::Z * 8. + Vec3::Y * 4.;
        // Paired ordinary riding-height and raised sightline views; continuous
        // translation within each segment exposes cloud/bird parallax.
        let lift = if segment.is_multiple_of(2) { 3. } else { 22. };
        *camera = Transform::from_translation(eye)
            .looking_at(p + q * Vec3::Z * 90. + Vec3::Y * lift, Vec3::Y);
    }
    if !perf && frame >= 180 && frame % 180 == 150 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/atmosphere-{segment}.png"
            )))
            .observe(captured);
    }
    if frame == 1260 {
        state.samples.sort_by(f64::total_cmp);
        let samples = &state.samples;
        let report = serde_json::json!({"map":track.definition.id.as_str(),"frames":samples.len(),
            "frame_median_ms":samples[samples.len()/2],"frame_p95_ms":samples[samples.len()*95/100],
            "frame_p99_ms":samples[samples.len()*99/100],"clouds":&*budget,"birds":birds.iter().count(),
            "captures":state.captures,"performance_mode":perf,
            "note":"CPU wall frame intervals; not GPU timestamps. Moving camera fixture; excludes first180frames."});
        std::fs::create_dir_all("screenshots").unwrap();
        std::fs::write(
            "screenshots/atmosphere.json",
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .unwrap();
        info!("Atmosphere survey: {report}");
        if !perf {
            assert_eq!(state.captures, 6);
        }
        exit.write(AppExit::Success);
    }
}
