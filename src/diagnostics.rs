//! Bounded local diagnostics. No telemetry is transmitted.
use crate::runtime::{SimSet, SimulationClock};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::{collections::VecDeque, time::Instant};
#[derive(Resource)]
pub struct Metrics {
    last_frame: Instant,
    fixed_start: Instant,
    physics_start: Instant,
    samples: VecDeque<f32>,
    pub frames: u64,
    pub average_ms: f32,
    pub p95_ms: f32,
    pub gameplay_ms: f32,
    pub physics_ms: f32,
    pub entities: usize,
    pub colliders: usize,
    pub visible: bool,
    refresh: f32,
}
impl Default for Metrics {
    fn default() -> Self {
        Self {
            last_frame: Instant::now(),
            fixed_start: Instant::now(),
            physics_start: Instant::now(),
            samples: VecDeque::with_capacity(240),
            frames: 0,
            average_ms: 0.0,
            p95_ms: 0.0,
            gameplay_ms: 0.0,
            physics_ms: 0.0,
            entities: 0,
            colliders: 0,
            visible: false,
            refresh: 0.0,
        }
    }
}
impl Metrics {
    pub fn evidence(&self, clock: &SimulationClock) -> serde_json::Value {
        serde_json::json!({"render_frames":self.frames,"fixed_ticks":clock.ticks,"active_fixed_ticks":clock.active_ticks,"target_hz":rr_core::SIMULATION_HZ,"wall_frame_average_ms":self.average_ms,"wall_frame_p95_ms":self.p95_ms,"last_gameplay_ms":self.gameplay_ms,"last_physics_ms":self.physics_ms,"entities":self.entities,"colliders":self.colliders,"measurement":"bounded wall-clock sample, not GPU timing or a hardware certification"})
    }
}
fn begin(mut metrics: ResMut<Metrics>) {
    metrics.fixed_start = Instant::now();
}
fn physics_begin(mut metrics: ResMut<Metrics>) {
    metrics.gameplay_ms = metrics.fixed_start.elapsed().as_secs_f32() * 1000.0;
    metrics.physics_start = Instant::now();
}
fn physics_end(mut metrics: ResMut<Metrics>) {
    metrics.physics_ms = metrics.physics_start.elapsed().as_secs_f32() * 1000.0;
}
#[derive(Component)]
struct DebugOverlay;
#[cfg(feature = "devtools")]
fn setup(mut commands: Commands) {
    commands.spawn((
        DebugOverlay,
        Text::new(""),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.3, 1.0, 0.8)),
        BackgroundColor(Color::srgba(0.01, 0.02, 0.03, 0.92)),
        GlobalZIndex(100),
        Node {
            position_type: PositionType::Absolute,
            right: px(24),
            bottom: px(210),
            padding: UiRect::all(px(12)),
            display: Display::None,
            ..default()
        },
    ));
}
#[allow(clippy::too_many_arguments)]
fn frame(
    time: Res<Time<bevy::time::Real>>,
    keys: Option<Res<ButtonInput<KeyCode>>>,
    mut metrics: ResMut<Metrics>,
    clock: Res<SimulationClock>,
    entities: Query<Entity>,
    colliders: Query<&Collider>,
    mut text: Query<(&mut Text, &mut Node), With<DebugOverlay>>,
) {
    let now = Instant::now();
    let ms = now.duration_since(metrics.last_frame).as_secs_f32() * 1000.0;
    metrics.last_frame = now;
    metrics.frames += 1;
    if metrics.samples.len() == 240 {
        metrics.samples.pop_front();
    }
    metrics.samples.push_back(ms);
    if keys.as_ref().is_some_and(|k| k.just_pressed(KeyCode::F3)) {
        metrics.visible = !metrics.visible;
    }
    metrics.refresh += time.delta_secs();
    if metrics.refresh >= 0.5 || metrics.frames == 1 {
        metrics.refresh = 0.0;
        let mut ordered: Vec<_> = metrics.samples.iter().copied().collect();
        ordered.sort_by(f32::total_cmp);
        metrics.average_ms = ordered.iter().sum::<f32>() / ordered.len() as f32;
        metrics.p95_ms = ordered[(ordered.len().saturating_sub(1) as f32 * 0.95).ceil() as usize];
        metrics.entities = entities.iter().count();
        metrics.colliders = colliders.iter().count();
    }
    for (mut text, mut node) in &mut text {
        node.display = if metrics.visible {
            Display::Flex
        } else {
            Display::None
        };
        **text = format!(
            "F3 DIAGNOSTICS\nFPS {:5.1}  avg {:5.2}ms  p95 {:5.2}ms\n60 Hz ticks {} / active {}\ngame {:5.2}ms  physics {:5.2}ms\nentities {}  colliders {}",
            1000.0 / metrics.average_ms.max(0.01),
            metrics.average_ms,
            metrics.p95_ms,
            clock.ticks,
            clock.active_ticks,
            metrics.gameplay_ms,
            metrics.physics_ms,
            metrics.entities,
            metrics.colliders
        );
    }
}
pub struct RoadDiagnosticsPlugin;
impl Plugin for RoadDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Metrics>()
            .add_systems(FixedPreUpdate, begin)
            .add_systems(
                FixedUpdate,
                physics_begin
                    .after(SimSet::Gameplay)
                    .before(PhysicsSet::SyncBackend),
            )
            .add_systems(
                FixedUpdate,
                physics_end
                    .after(PhysicsSet::Writeback)
                    .before(SimSet::AfterPhysics),
            )
            .add_systems(Update, frame);
        #[cfg(feature = "devtools")]
        app.add_systems(Startup, setup);
    }
}
/// Install before the engine starts worker threads. Preserve the default report.
pub fn install_panic_report() {
    let old = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let base = std::env::var_os("ROADDASH_LOG_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join("roaddash-crashes"));
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let path = base.join(format!("panic-{timestamp}-{}.log", std::process::id()));
        if std::fs::create_dir_all(&base)
            .and_then(|_| {
                std::fs::write(
                    &path,
                    format!(
                        "RoadDash {}\nOS {}\n{info}\n{}\n",
                        env!("CARGO_PKG_VERSION"),
                        std::env::consts::OS,
                        std::backtrace::Backtrace::force_capture()
                    ),
                )
            })
            .is_ok()
        {
            eprintln!("Crash report: {}", path.display());
        }
        old(info);
    }));
}
