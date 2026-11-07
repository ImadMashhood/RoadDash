//! Reproducible real-backend fixtures. Reports observations, never fabricates acceptance.
use super::*;
use bevy::ecs::system::RunSystemOnce;
use serde::Serialize;
#[derive(Clone, Copy)]
pub enum Scenario {
    Drop,
    LimbForce,
    TorsoPush,
    HingeStress,
    EjectionLow,
    EjectionHigh,
    Tumble,
    Wall,
    Pileup,
}
impl Scenario {
    pub const ALL: [Self; 9] = [
        Self::Drop,
        Self::LimbForce,
        Self::TorsoPush,
        Self::HingeStress,
        Self::EjectionLow,
        Self::EjectionHigh,
        Self::Tumble,
        Self::Wall,
        Self::Pileup,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Drop => "drop",
            Self::LimbForce => "limb_force",
            Self::TorsoPush => "torso_push",
            Self::HingeStress => "hinge_stress",
            Self::EjectionLow => "ejection_low",
            Self::EjectionHigh => "ejection_high",
            Self::Tumble => "road_tumble",
            Self::Wall => "static_wall",
            Self::Pileup => "fifteen_crashes",
        }
    }
}
#[derive(Serialize, Debug)]
pub struct Measurement {
    pub scenario: String,
    pub mode: String,
    pub bodies: usize,
    pub simulated_seconds: f32,
    pub max_anchor_error_m: f32,
    pub final_anchor_error_m: f32,
    pub max_limit_excess_rad: f32,
    pub max_speed_mps: f32,
    pub peak_speed_part: usize,
    pub peak_angular_part: usize,
    pub peak_angular_radps: f32,
    pub max_elbow_flex_rad: f32,
    pub max_knee_flex_rad: f32,
    pub physics_substeps: usize,
    pub extra_solver_iterations: usize,
    pub final_max_speed_mps: f32,
    pub final_max_angular_radps: f32,
    pub sleeping_bodies: usize,
    pub resting_ragdolls: usize,
    pub finite: bool,
    pub initial_mass_kg: f32,
    pub initial_momentum_error: f32,
    pub peak_mechanical_energy_j: f64,
    pub initial_mechanical_energy_j: f64,
    pub headless_wall_ms: f64,
}
pub(super) fn art() -> crate::world::Art {
    crate::world::Art {
        models: None,
        props: None,
        cube: Handle::default(),
        sphere: Handle::default(),
        wheel: Handle::default(),
        dark: Handle::default(),
        metal: Handle::default(),
        white: Handle::default(),
        colors: vec![Handle::default(); 15],
        helmets: vec![Handle::default(); 15],
        bike_colors: vec![Handle::default(); 15],
    }
}
pub fn fixture(enabled: bool, scenario: Scenario) -> App {
    let mut app = App::new();
    let settings = Settings {
        enabled,
        ..Settings::selected().expect("validated lab settings")
    };
    let substeps = if enabled {
        settings.config.physics_substeps
    } else {
        3
    };
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
        RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
        BiomechanicsPlugin,
    ))
    .insert_resource(settings)
    .insert_resource(art())
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .insert_resource(TimestepMode::Fixed {
        dt: 1.0 / 60.0,
        substeps,
    })
    .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        std::time::Duration::from_secs_f64(1.0 / 60.0),
    ))
    .configure_sets(
        FixedUpdate,
        crate::runtime::SimSet::Gameplay.before(PhysicsSet::SyncBackend),
    );
    app.add_systems(
        FixedUpdate,
        crate::recovery::constrain_crash_energy.after(PhysicsSet::Writeback),
    );
    app.world_mut().spawn((
        RigidBody::Fixed,
        Collider::cuboid(500.0, 0.5, 500.0),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
    if matches!(scenario, Scenario::Wall | Scenario::Tumble) {
        app.world_mut().spawn((
            RigidBody::Fixed,
            Collider::cuboid(8.0, 1.0, 0.6),
            Transform::from_xyz(0.0, 1.0, 9.0),
        ));
    }
    let count = if matches!(scenario, Scenario::Pileup) {
        15
    } else {
        1
    };
    app.world_mut()
        .run_system_once(move |mut commands: Commands, art: Res<crate::world::Art>| {
            for i in 0..count {
                let velocity = match scenario {
                    Scenario::EjectionLow => Vec3::Z * 8.0,
                    Scenario::EjectionHigh => Vec3::Z * 75.0,
                    Scenario::Tumble | Scenario::Wall => Vec3::Z * 18.0,
                    _ => Vec3::ZERO,
                };
                let owner = commands
                    .spawn((
                        Velocity {
                            linvel: velocity,
                            angvel: if matches!(scenario, Scenario::Tumble) {
                                Vec3::X * 2.0
                            } else {
                                Vec3::ZERO
                            },
                        },
                        ImpactSeverity(0.4),
                    ))
                    .id();
                crate::recovery::humanoid(
                    &mut commands,
                    &art,
                    owner,
                    4,
                    Vec3::new((i % 5) as f32 * 1.6, 2.5, (i / 5) as f32 * 2.5),
                    Quat::IDENTITY,
                    velocity,
                );
            }
        })
        .unwrap();
    let w = app.world_mut();
    let records: Vec<_> = w
        .query::<(Entity, &Transform, &Velocity, &OwnedBody)>()
        .iter(w)
        .map(|(e, t, v, _)| (e, *t, *v))
        .collect();
    for (e, t, v) in records {
        w.entity_mut(e)
            .insert((ReadMassProperties::default(), Sleeping::default()));
        if !matches!(
            scenario,
            Scenario::LimbForce | Scenario::TorsoPush | Scenario::HingeStress
        ) {
            w.entity_mut(e).insert(crate::recovery::CrashEnvelope::new(
                v.linvel,
                t.translation,
                t.rotation,
                Vec3::Z,
            ));
        }
    }
    initialize(app.world_mut());
    let w = app.world_mut();
    let inertia: Vec<_> = w
        .query::<(Entity, &Collider, &ColliderMassProperties)>()
        .iter(w)
        .filter_map(|(e, c, m)| {
            let ColliderMassProperties::Mass(m) = m else {
                return None;
            };
            let p = c.raw.mass_properties(1.0);
            let i = p.principal_inertia() * (*m / p.mass());
            let q = p.principal_inertia_local_frame;
            Some((
                e,
                LabInertia {
                    principal: Vec3::new(i.x, i.y, i.z),
                    frame: Quat::from_xyzw(q.i, q.j, q.k, q.w),
                },
            ))
        })
        .collect();
    for (e, i) in inertia {
        w.entity_mut(e).insert(i);
    }
    app
}
fn mass(world: &World, e: Entity) -> f32 {
    match world.get::<ColliderMassProperties>(e) {
        Some(ColliderMassProperties::Mass(m)) => *m,
        _ => 0.0,
    }
}
#[derive(Component)]
struct LabInertia {
    principal: Vec3,
    frame: Quat,
}
fn energy(world: &World, e: Entity, t: &Transform, v: &Velocity) -> f64 {
    let m = mass(world, e) as f64;
    let angular = world.get::<LabInertia>(e).map_or(0.0, |p| {
        let omega = (t.rotation * p.frame).inverse() * v.angvel;
        (omega.as_dvec3() * omega.as_dvec3() * p.principal.as_dvec3()).element_sum() * 0.5
    });
    m * (0.5 * v.linvel.length_squared() as f64 + 9.81 * t.translation.y as f64) + angular
}
pub fn measure(enabled: bool, scenario: Scenario, ticks: usize) -> Measurement {
    let mut app = fixture(enabled, scenario);
    let config = app.world().resource::<Settings>().config.clone();
    let w = app.world_mut();
    let initial: Vec<_> = w
        .query::<(Entity, &Transform, &Velocity, &OwnedBody)>()
        .iter(w)
        .map(|(e, t, v, _)| (e, *t, *v))
        .collect();
    let mass_total = initial.iter().map(|(e, _, _)| mass(w, *e)).sum::<f32>();
    let momentum = initial
        .iter()
        .map(|(e, _, v)| v.linvel * mass(w, *e))
        .sum::<Vec3>();
    let expected = initial
        .iter()
        .map(|(e, _, _)| {
            let o = w.get::<OwnedBody>(*e).unwrap();
            w.get::<Velocity>(o.owner).map_or(Vec3::ZERO, |v| v.linvel) * mass(w, *e)
        })
        .sum::<Vec3>();
    let mut result = Measurement {
        scenario: scenario.name().into(),
        mode: if enabled { "biomechanical" } else { "baseline" }.into(),
        bodies: initial.len(),
        simulated_seconds: ticks as f32 / 60.0,
        max_anchor_error_m: 0.0,
        final_anchor_error_m: 0.0,
        max_limit_excess_rad: 0.0,
        max_speed_mps: 0.0,
        peak_speed_part: 0,
        peak_angular_part: 0,
        peak_angular_radps: 0.0,
        max_elbow_flex_rad: 0.0,
        max_knee_flex_rad: 0.0,
        physics_substeps: if enabled { config.physics_substeps } else { 3 },
        extra_solver_iterations: if enabled {
            config.extra_solver_iterations
        } else {
            0
        },
        final_max_speed_mps: 0.0,
        final_max_angular_radps: 0.0,
        sleeping_bodies: 0,
        resting_ragdolls: 0,
        finite: true,
        initial_mass_kg: mass_total,
        initial_momentum_error: (momentum - expected).length(),
        peak_mechanical_energy_j: 0.0,
        initial_mechanical_energy_j: initial.iter().map(|(e, t, v)| energy(w, *e, t, v)).sum(),
        headless_wall_ms: 0.0,
    };
    let start = std::time::Instant::now();
    for tick in 0..ticks {
        if tick == 30 {
            let target = match scenario {
                Scenario::LimbForce => Some((4, Vec3::new(0.0, 0.0, 4.0), Vec3::X)),
                Scenario::HingeStress => Some((8, Vec3::ZERO, Vec3::new(-2.0, 1.0, 1.0))),
                Scenario::TorsoPush => Some((0, Vec3::Z * 45.0, Vec3::ZERO)),
                _ => None,
            };
            if let Some((part, impulse, torque_impulse)) = target {
                let w = app.world_mut();
                let e = w
                    .query::<(Entity, &Candidate)>()
                    .iter(w)
                    .find(|(_, c)| c.0 == part)
                    .map(|(e, _)| e)
                    .unwrap();
                w.entity_mut(e).insert(ExternalImpulse {
                    impulse,
                    torque_impulse,
                });
            }
        }
        app.update();
        let w = app.world_mut();
        let records: Vec<_> = w
            .query::<(Entity, &Transform, &Velocity, &OwnedBody)>()
            .iter(w)
            .map(|(e, t, v, _)| (e, *t, *v))
            .collect();
        let mut total = 0.0;
        result.final_anchor_error_m = 0.0;
        result.final_max_speed_mps = 0.0;
        result.final_max_angular_radps = 0.0;
        for (e, t, v) in records {
            result.finite &= t.translation.is_finite()
                && t.rotation.is_finite()
                && v.linvel.is_finite()
                && v.angvel.is_finite();
            if v.linvel.length() > result.max_speed_mps {
                result.peak_speed_part = w
                    .get::<Segment>(e)
                    .map_or_else(|| w.get::<Candidate>(e).map_or(0, |c| c.0), |s| s.part);
            }
            if v.angvel.length() > result.peak_angular_radps {
                result.peak_angular_part = w
                    .get::<Segment>(e)
                    .map_or_else(|| w.get::<Candidate>(e).map_or(0, |c| c.0), |s| s.part);
            }
            result.max_speed_mps = result.max_speed_mps.max(v.linvel.length());
            result.peak_angular_radps = result.peak_angular_radps.max(v.angvel.length());
            result.final_max_speed_mps = result.final_max_speed_mps.max(v.linvel.length());
            result.final_max_angular_radps = result.final_max_angular_radps.max(v.angvel.length());
            total += energy(w, e, &t, &v);
            if let Some(j) = w.get::<ImpulseJoint>(e) {
                let p = w.get::<Transform>(j.parent).unwrap();
                let raw: &GenericJoint = j.data.as_ref();
                let error = p
                    .transform_point(raw.local_anchor1())
                    .distance(t.transform_point(raw.local_anchor2()));
                result.max_anchor_error_m = result.max_anchor_error_m.max(error);
                result.final_anchor_error_m = result.final_anchor_error_m.max(error);
                let angles = coordinates(p.rotation.inverse() * t.rotation);
                if let Some(c) = w.get::<Candidate>(e) {
                    if matches!(c.0, 4 | 6) {
                        result.max_elbow_flex_rad = result.max_elbow_flex_rad.max(-angles.x);
                    }
                    if matches!(c.0, 8 | 10) {
                        result.max_knee_flex_rad = result.max_knee_flex_rad.max(angles.x);
                    }
                }
                for (i, axis) in AXES.into_iter().enumerate() {
                    if let Some(l) = raw.limits(axis) {
                        let excess = (l.min - angles[i]).max(angles[i] - l.max).max(0.0);
                        result.max_limit_excess_rad = result.max_limit_excess_rad.max(excess);
                    }
                }
            }
        }
        result.peak_mechanical_energy_j = result.peak_mechanical_energy_j.max(total);
    }
    let w = app.world_mut();
    result.sleeping_bodies = w
        .query::<(&OwnedBody, &Sleeping)>()
        .iter(w)
        .filter(|(_, s)| s.sleeping)
        .count();
    result.resting_ragdolls = w
        .query::<&Brain>()
        .iter(w)
        .filter(|b| b.control.state == rr_core::biomechanics::State::Resting)
        .count();
    result.headless_wall_ms = start.elapsed().as_secs_f64() * 1000.0;
    result
}
pub fn run(path: &std::path::Path) -> Result<(), String> {
    let mut results = vec![];
    for scenario in Scenario::ALL {
        for enabled in [false, true] {
            let r = measure(enabled, scenario, 900);
            println!(
                "RAGDOLL_LAB {} {} finite={} anchor_max={:0.4} final_speed={:0.3} asleep={}",
                r.scenario,
                r.mode,
                r.finite,
                r.max_anchor_error_m,
                r.final_max_speed_mps,
                r.sleeping_bodies
            );
            results.push(r);
        }
    }
    std::fs::write(
        path,
        serde_json::to_string_pretty(&results).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if results.iter().any(|r| !r.finite) {
        return Err("non-finite ragdoll state".into());
    }
    Ok(())
}
