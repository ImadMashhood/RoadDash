//! Native visual counterpart of the headless fixtures. No race/save side effects.
use super::lab::Scenario;
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource)]
struct View {
    scenario: Scenario,
    owners: Vec<Entity>,
    age: f32,
    injected: bool,
    frame: u32,
    smoke: bool,
    reset: bool,
}
#[derive(Component)]
struct Hud;
#[derive(Component)]
struct Wall;
pub fn build(smoke: bool) -> App {
    let mut app = App::new();
    let mut settings = Settings::selected().expect("validated ragdoll settings");
    settings.enabled = !std::env::args().any(|a| a == "--ragdoll-baseline");
    let substeps = if settings.enabled {
        settings.config.physics_substeps
    } else {
        3
    };
    app.insert_resource(settings)
        .insert_resource(View {
            scenario: Scenario::Drop,
            owners: vec![],
            age: 0.0,
            injected: false,
            frame: 0,
            smoke,
            reset: true,
        })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: crate::models::asset_directory(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RoadDash | Ragdoll comparison lab".into(),
                        resolution: (1280, 800).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
            BiomechanicsPlugin,
            crate::models::RoadModelPlugin,
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps,
        })
        .configure_sets(
            FixedUpdate,
            crate::runtime::SimSet::Gameplay.before(PhysicsSet::SyncBackend),
        )
        .add_systems(
            FixedUpdate,
            crate::recovery::constrain_crash_energy.after(PhysicsSet::Writeback),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, controls);
    install_debug(&mut app);
    if smoke {
        for mode in ["biomechanical", "baseline"] {
            std::fs::create_dir_all(format!("screenshots/ragdoll-{mode}-frames"))
                .expect("screenshot output");
        }
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ));
    }
    app
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut binds: ResMut<Assets<bevy::mesh::skinning::SkinnedMeshInverseBindposes>>,
    server: Res<AssetServer>,
) {
    let mut art = super::lab::art();
    art.cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    art.sphere = meshes.add(Sphere::new(1.0));
    art.dark = materials.add(Color::srgb(0.10, 0.13, 0.16));
    art.metal = materials.add(Color::srgb(0.35, 0.40, 0.45));
    art.white = materials.add(Color::WHITE);
    art.colors = vec![materials.add(Color::srgb(0.48, 0.02, 0.12)); 15];
    art.helmets = art.colors.clone();
    art.bike_colors = art.colors.clone();
    art.models = Some(crate::models::AuthoredModels::load(
        &mut meshes,
        &mut materials,
        &mut binds,
        Some(&server),
    ));
    let floor = crate::world::part(
        &mut commands,
        art.cube.clone(),
        art.dark.clone(),
        Vec3::new(0.0, -0.2, 0.0),
        Vec3::new(80.0, 0.4, 80.0),
        Quat::IDENTITY,
    );
    commands
        .entity(floor)
        .insert((RigidBody::Fixed, Collider::cuboid(40.0, 0.2, 40.0)));
    let wall = crate::world::part(
        &mut commands,
        art.cube.clone(),
        art.metal.clone(),
        Vec3::new(0.0, -10.0, 9.0),
        Vec3::new(16.0, 2.0, 1.2),
        Quat::IDENTITY,
    );
    commands
        .entity(wall)
        .insert((Wall, RigidBody::Fixed, Collider::cuboid(8.0, 1.0, 0.6)));
    commands.insert_resource(art);
    commands.insert_resource(ClearColor(Color::srgb(0.12, 0.16, 0.20)));
    commands.insert_resource(AmbientLight {
        brightness: 450.0,
        ..default()
    });
    commands.spawn((
        Camera3d::default(),
        bevy::core_pipeline::tonemapping::Tonemapping::Reinhard,
        Transform::from_xyz(5.0, 4.0, -6.0).looking_at(Vec3::new(0.0, 1.5, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 18000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Hud,
        Text::new("Ragdoll lab loading"),
        TextFont {
            font_size: 17.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(16.0),
            top: Val::Px(16.0),
            ..default()
        },
    ));
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn controls(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut state: ResMut<View>,
    mut settings: ResMut<Settings>,
    mut timestep: ResMut<TimestepMode>,
    art: Res<crate::world::Art>,
    bodies: Query<(Entity, &OwnedBody, &Candidate)>,
    mut walls: Query<&mut Transform, With<Wall>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Wall>)>,
    mut hud: Query<&mut Text, With<Hud>>,
    brains: Query<&Brain>,
    mut exit: MessageWriter<AppExit>,
) {
    state.frame += 1;
    state.age += time.delta_secs();
    if keys.just_pressed(KeyCode::Space) {
        if virtual_time.is_paused() {
            virtual_time.unpause();
        } else {
            virtual_time.pause();
        }
    }
    if keys.just_pressed(KeyCode::KeyB) || (state.smoke && state.frame == 1000) {
        settings.enabled = !settings.enabled;
        state.reset = true;
        *timestep = TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: if settings.enabled {
                settings.config.physics_substeps
            } else {
                3
            },
        };
    }
    if keys.just_pressed(KeyCode::KeyR) {
        state.reset = true;
    }
    for (i, key) in [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ]
    .into_iter()
    .enumerate()
    {
        if keys.just_pressed(key) {
            state.scenario = Scenario::ALL[i];
            state.reset = true;
        }
    }
    if state.reset {
        // Every articulated body is owned; recursive despawn includes its visual children.
        for (e, o, _) in &bodies {
            if state.owners.contains(&o.owner) {
                commands.entity(e).despawn();
            }
        }
        // Added spine/hands/feet have no Candidate; find them through an exclusive owner cleanup.
        let old = std::mem::take(&mut state.owners);
        commands.queue(move |world: &mut World| {
            let entities: Vec<_> = world
                .query::<(Entity, &OwnedBody)>()
                .iter(world)
                .filter(|(_, o)| old.contains(&o.owner))
                .map(|(e, _)| e)
                .collect();
            for e in entities {
                world.despawn(e);
            }
            for e in old {
                world.despawn(e);
            }
        });
        state.age = 0.0;
        state.injected = false;
        state.reset = false;
        let count = if matches!(state.scenario, Scenario::Pileup) {
            15
        } else {
            1
        };
        for i in 0..count {
            let velocity = match state.scenario {
                Scenario::EjectionLow => Vec3::Z * 8.0,
                Scenario::EjectionHigh => Vec3::Z * 75.0,
                Scenario::Wall | Scenario::Tumble => Vec3::Z * 18.0,
                _ => Vec3::ZERO,
            };
            let omega = if matches!(state.scenario, Scenario::Tumble) {
                Vec3::X * 2.0
            } else {
                Vec3::ZERO
            };
            let owner = commands
                .spawn((
                    Velocity {
                        linvel: velocity,
                        angvel: omega,
                    },
                    ImpactSeverity(0.4),
                ))
                .id();
            state.owners.push(owner);
            let origin = Vec3::new((i % 5) as f32 * 1.6, 2.5, (i / 5) as f32 * 2.5);
            let parts = crate::recovery::humanoid(
                &mut commands,
                &art,
                owner,
                4,
                origin,
                Quat::IDENTITY,
                velocity,
            );
            if !matches!(
                state.scenario,
                Scenario::LimbForce | Scenario::TorsoPush | Scenario::HingeStress
            ) {
                for (p, e) in parts.into_iter().enumerate() {
                    commands.entity(e).insert((
                        ReadMassProperties::default(),
                        crate::recovery::CrashEnvelope::new(
                            velocity,
                            origin + crate::human::SPECS[p].0,
                            Quat::IDENTITY,
                            Vec3::Z,
                        ),
                    ));
                }
            }
        }
        for mut wall in &mut walls {
            wall.translation.y = if matches!(state.scenario, Scenario::Wall | Scenario::Tumble) {
                1.0
            } else {
                -10.0
            };
        }
        let (position, target) = if count > 1 {
            (Vec3::new(12.0, 9.0, -15.0), Vec3::new(3.0, 1.5, 3.0))
        } else if matches!(
            state.scenario,
            Scenario::Wall | Scenario::Tumble | Scenario::EjectionHigh | Scenario::EjectionLow
        ) {
            (Vec3::new(10.0, 9.0, -8.0), Vec3::new(0.0, 1.5, 8.0))
        } else {
            (Vec3::new(5.0, 4.0, -6.0), Vec3::new(0.0, 1.5, 0.0))
        };
        for mut camera in &mut camera {
            *camera = Transform::from_translation(position).looking_at(target, Vec3::Y);
        }
    }
    let auto = state.age > 0.5 && !state.injected;
    let force = if keys.just_pressed(KeyCode::KeyL)
        || (auto && matches!(state.scenario, Scenario::LimbForce))
    {
        Some((4, Vec3::new(0.0, 0.0, 4.0), Vec3::X))
    } else if keys.just_pressed(KeyCode::KeyT)
        || (auto && matches!(state.scenario, Scenario::TorsoPush))
    {
        Some((0, Vec3::Z * 45.0, Vec3::ZERO))
    } else if auto && matches!(state.scenario, Scenario::HingeStress) {
        Some((8, Vec3::ZERO, Vec3::new(-2.0, 1.0, 1.0)))
    } else {
        None
    };
    if let Some((part, impulse, torque_impulse)) = force
        && let Some((e, _, _)) = bodies
            .iter()
            .find(|(_, o, c)| state.owners.contains(&o.owner) && c.0 == part)
    {
        commands.entity(e).insert(ExternalImpulse {
            impulse,
            torque_impulse,
        });
        state.injected = true;
    }
    let mode = if settings.enabled {
        "BIOMECHANICAL / 16 bodies"
    } else {
        "BASELINE / 11 bodies"
    };
    let status = brains
        .iter()
        .next()
        .map_or("original passive motors".into(), |b| {
            format!(
                "{:?} | activation {:.3} | supported {}",
                b.control.state, b.control.activation, b.supported
            )
        });
    for mut text in &mut hud {
        **text = format!(
            "{mode} | {} | {status}\n1 drop  2 limb force  3 torso push  4 hinge stress  5/6 ejection  7 tumble  8 wall  9 fifteen riders\nB compare/reset | R reset | L limb impulse | T torso impulse | F7 joint debug | Space pause",
            state.scenario.name()
        );
    }
    if state.smoke {
        let frame = state.frame;
        let local = if frame >= 1000 { frame - 1000 } else { frame };
        if (20..=260).contains(&local) && (local - 20).is_multiple_of(6) {
            let mode = if settings.enabled {
                "biomechanical"
            } else {
                "baseline"
            };
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!(
                    "screenshots/ragdoll-{mode}-frames/{local:04}.png"
                )));
        }

        for (when, label) in [
            (40, "airborne"),
            (100, "impact"),
            (220, "tumble"),
            (900, "late_settling"),
        ] {
            if frame == when || frame == when + 1000 {
                let mode = if settings.enabled {
                    "biomechanical"
                } else {
                    "baseline"
                };
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(format!(
                        "screenshots/ragdoll-{mode}-{label}.png"
                    )));
            }
        }
        if frame == 1960 {
            println!("Ragdoll comparison view smoke passed both modes");
            exit.write(AppExit::Success);
        }
    }
}
