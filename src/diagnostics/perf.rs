//! Bounded asynchronous local performance evidence; never waits for disk on the game thread.
use super::*;
use std::{
    io::{BufWriter, Write},
    sync::mpsc::{SyncSender, TrySendError},
};
#[derive(Resource)]
pub(super) struct PerfLog {
    sender: Option<SyncSender<serde_json::Value>>,
    cpu_threads: usize,
    last: Instant,
    frames: u64,
    game_ms: f32,
    physics_ms: f32,
    physics_phases_ms: [f32; 3],
    main_ms: f32,
    steps: u64,
    dropped: u64,
}
impl Default for PerfLog {
    fn default() -> Self {
        let sender = if std::env::var("ROADDASH_PERF_LOG").as_deref() == Ok("0") {
            None
        } else {
            let dir = std::env::var_os("ROADDASH_LOG_DIR")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| std::env::temp_dir().join("roaddash-performance"));
            let path = dir.join(format!("performance-{}.jsonl", std::process::id()));
            let (tx, rx) = std::sync::mpsc::sync_channel::<serde_json::Value>(8);
            if let Err(e) = std::fs::create_dir_all(&dir) {
                eprintln!("Performance log unavailable: {e}");
                None
            } else {
                eprintln!(
                    "Performance log: {} (1-second samples; CPU/wall timing, not GPU timestamps)",
                    path.display()
                );
                std::thread::spawn(move || {
                    let result = (|| -> std::io::Result<()> {
                        let mut out = BufWriter::new(std::fs::File::create(&path)?);
                        let mut bytes = 0;
                        for mut value in rx {
                            value["linux_process_memory"] = linux_fields(
                                "/proc/self/status",
                                &["VmRSS:", "RssAnon:", "RssFile:", "VmSwap:"],
                            );
                            value["linux_system_memory"] =
                                linux_fields("/proc/meminfo", &["MemAvailable:", "SwapFree:"]);
                            value["linux_system_paging_counters"] =
                                linux_fields("/proc/vmstat", &["pswpin", "pswpout", "pgmajfault"]);
                            let text = serde_json::to_string(&value)?;
                            if bytes + text.len() > 16 * 1024 * 1024 {
                                out.flush()?;
                                drop(out);
                                std::fs::rename(&path, path.with_extension("jsonl.old"))?;
                                out = BufWriter::new(std::fs::File::create(&path)?);
                                bytes = 0;
                            }
                            writeln!(out, "{text}")?;
                            out.flush()?;
                            bytes += text.len() + 1;
                        }
                        Ok(())
                    })();
                    if let Err(e) = result {
                        eprintln!("Performance log stopped: {e}");
                    }
                });
                Some(tx)
            }
        };
        Self {
            sender,
            cpu_threads: std::thread::available_parallelism().map_or(1, usize::from),
            last: Instant::now(),
            frames: 0,
            game_ms: 0.,
            physics_ms: 0.,
            physics_phases_ms: [0.; 3],
            main_ms: 0.,
            steps: 0,
            dropped: 0,
        }
    }
}
fn linux_fields(path: &str, keys: &[&str]) -> serde_json::Value {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut map = serde_json::Map::new();
    for line in text.lines() {
        let mut words = line.split_whitespace();
        if let Some(key) = words.next()
            && keys.contains(&key)
        {
            map.insert(
                key.trim_end_matches(':').into(),
                words.collect::<Vec<_>>().join(" ").into(),
            );
        }
    }
    serde_json::Value::Object(map)
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn sample(
    mut log: ResMut<PerfLog>,
    metrics: Res<Metrics>,
    clock: Res<SimulationClock>,
    render: Res<RenderTimes>,
    race: Option<Res<crate::game::Race>>,
    riders: Query<&crate::game::Rider>,
    meshes: Option<Res<Assets<Mesh>>>,
    materials: Option<Res<Assets<StandardMaterial>>>,
    views: Query<&ViewVisibility, With<Mesh3d>>,
    bodies: Query<(&RigidBody, Option<&RigidBodyDisabled>)>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    context: ReadRapierContext,
) {
    if log.sender.is_none() {
        return;
    }
    log.frames += 1;
    log.game_ms += metrics.frame_gameplay_ms;
    log.physics_ms += metrics.frame_physics_ms;
    for (sum, phase) in log
        .physics_phases_ms
        .iter_mut()
        .zip(metrics.frame_physics_phases_ms)
    {
        *sum += phase;
    }
    log.steps += metrics.frame_fixed_steps;
    log.main_ms += metrics.frame_start.elapsed().as_secs_f32() * 1000.;
    if log.last.elapsed().as_secs_f32() < 1. {
        return;
    }
    let frames = log.frames.max(1) as f32;
    let window_wall_ms = log.last.elapsed().as_secs_f64() * 1000. / f64::from(frames);
    let backend = context.single().ok().map(|ctx| {
        serde_json::json!({
            "rigid_bodies": ctx.rigidbody_set.bodies.len(),
            "backend_colliders": ctx.colliders.colliders.len(),
            "contact_pairs": ctx.simulation.narrow_phase.contact_pairs().count(),
            "contact_manifolds": ctx.simulation.narrow_phase.contact_pairs().map(|p| p.manifolds.len()).sum::<usize>(),
            "awake_dynamic_bodies": ctx.rigidbody_set.bodies.iter().filter(|(_, b)| b.is_dynamic() && !b.is_sleeping() && b.is_enabled()).count(),
            "impulse_joints": ctx.joints.impulse_joints.len(),
            "note": "instantaneous graph census, not per-substep timing or GPU profiling"
        })
    });
    let player = riders.iter().find(|r| r.id == 0);
    let value = serde_json::json!({"schema_version":1,"window_wall_ms_per_frame":window_wall_ms,"physics_backend":backend,"parallel_physics":cfg!(feature="parallel_physics"),"available_cpu_threads":log.cpu_threads,"requested_rayon_threads":std::env::var("RAYON_NUM_THREADS").ok(),"last_render_extract_ms":render.0[0].load(std::sync::atomic::Ordering::Relaxed)as f64/1e6,"last_render_schedule_ms":render.0[1].load(std::sync::atomic::Ordering::Relaxed)as f64/1e6,"elapsed_s":clock.ticks as f64/60.,"frames":log.frames,"wall_average_ms":metrics.average_ms,"wall_p95_ms":metrics.p95_ms,
 "fixed_steps":log.steps,"fixed_gameplay_ms_per_frame":log.game_ms/frames,"physics_ms_per_frame":log.physics_ms/frames,"physics_sync_ms_per_frame":log.physics_phases_ms[0]/frames,"physics_step_ms_per_frame":log.physics_phases_ms[1]/frames,"physics_writeback_ms_per_frame":log.physics_phases_ms[2]/frames,"main_world_ms_per_frame":log.main_ms/frames,
 "entities":metrics.entities,"colliders":metrics.colliders,"dynamic_bodies":bodies.iter().filter(|(b,d)|**b==RigidBody::Dynamic&&d.is_none()).count(),
 "mesh_entities":views.iter().count(),"visible_mesh_entities":views.iter().filter(|v|v.get()).count(),"mesh_assets":meshes.map_or(0,|a|a.len()),"material_assets":materials.map_or(0,|a|a.len()),
 "camera_xyz":cameras.iter().next().map(|t|t.translation().to_array()),"player_progress_m":player.map(|r|r.progress),"player_route":player.map(|r|r.route),"racing":race.is_some_and(|r|r.phase==crate::game::Phase::Racing),"dropped_log_samples":log.dropped,
 "measurement":"wall/CPU timing; main excludes render extraction and GPU; render fields are latest CPU/wait spans, may overlap main and are not GPU timestamps; system paging counters are global, not per-game; memory values retain proc units"});
    if let Some(tx) = &log.sender {
        match tx.try_send(value) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) => log.dropped += 1,
            Err(TrySendError::Disconnected(_)) => {
                log.sender = None;
                eprintln!("Performance log worker unavailable");
            }
        }
    }
    log.last = Instant::now();
    log.frames = 0;
    log.game_ms = 0.;
    log.physics_ms = 0.;
    log.physics_phases_ms = [0.; 3];
    log.main_ms = 0.;
    log.steps = 0;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linux_memory_sample_is_small_and_contains_only_requested_keys() {
        let fields = linux_fields("/proc/self/status", &["VmRSS:"]);
        assert!(fields.as_object().unwrap().len() <= 1);
        assert!(
            linux_fields("/missing/roaddash", &[])
                .as_object()
                .unwrap()
                .is_empty()
        );
    }
}

#[derive(Resource, Clone)]
pub(super) struct RenderTimes(std::sync::Arc<[std::sync::atomic::AtomicU64; 2]>);
impl Default for RenderTimes {
    fn default() -> Self {
        Self(std::sync::Arc::new(std::array::from_fn(|_| {
            std::sync::atomic::AtomicU64::new(0)
        })))
    }
}
#[derive(Resource)]
struct RenderClock(Instant);
fn render_begin(mut clock: ResMut<RenderClock>) {
    clock.0 = Instant::now();
}
fn render_end(clock: Res<RenderClock>, times: Res<RenderTimes>) {
    times.0[1].store(
        clock.0.elapsed().as_nanos().min(u64::MAX as u128) as u64,
        std::sync::atomic::Ordering::Relaxed,
    );
}
pub(super) fn install_render(app: &mut App) {
    use bevy::render::{Render, RenderApp, RenderSystems};
    let shared = app.world().resource::<RenderTimes>().clone();
    if let Some(render) = app.get_sub_app_mut(RenderApp) {
        let timing = shared.clone();
        let mut extract = render.take_extract();
        render.set_extract(move |main, world| {
            let start = Instant::now();
            if let Some(f) = extract.as_mut() {
                f(main, world);
            }
            timing.0[0].store(
                start.elapsed().as_nanos().min(u64::MAX as u128) as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
        });
        render
            .insert_resource(shared)
            .insert_resource(RenderClock(Instant::now()))
            .add_systems(Render, render_begin.before(RenderSystems::ExtractCommands))
            .add_systems(Render, render_end.after(RenderSystems::PostCleanup));
    }
}
