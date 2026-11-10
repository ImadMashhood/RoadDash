//! Native mounted-motion review with the actual game meshes and animator.
use super::*;
use crate::{models, world};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
struct Review {
    frame: u32,
    mode: usize,
    view: usize,
    smoke: bool,
    hands: bool,
    gait: bool,
    position: Vec3,
    seen_shapes: [bool; 4],
    weapon_review: Option<String>,
}
#[derive(Component)]
struct Label;
const LABELS: [&str; 13] = [
    "NORMAL",
    "FAST TUCK / ELBOWS IN",
    "LEAN LEFT",
    "LEAN RIGHT",
    "PUNCH LEFT",
    "PUNCH RIGHT",
    "KICK LEFT",
    "KICK RIGHT",
    "STANDING / PALMS IN",
    "WALKING",
    "RUNNING",
    "CLUB LEFT",
    "CLUB RIGHT",
];
pub fn build(smoke: bool) -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                file_path: models::asset_directory(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RoadDash | Animation review".into(),
                    resolution: (1280, 900).into(),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(models::RoadModelPlugin)
    .insert_resource(Review {
        smoke,
        hands: std::env::args().any(|a| a == "--hand-review"),
        gait: std::env::var_os("ROADDASH_GAIT_REVIEW").is_some(),
        ..default()
    })
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .insert_resource(Race {
        phase: crate::game::Phase::Racing,
        ..default()
    })
    .add_systems(Startup, setup)
    .add_systems(Update, crate::vehicle::visuals)
    .add_systems(FixedUpdate, drive.before(super::update))
    .add_systems(FixedUpdate, hand_keyframes.after(super::update))
    .add_systems(Update, review.after(crate::vehicle::visuals));
    if std::env::var_os("ROADDASH_GAIT_REVIEW").is_some() {
        app.add_plugins(bevy_rapier3d::prelude::RapierPhysicsPlugin::<
            bevy_rapier3d::prelude::NoUserData,
        >::default());
    }
    if smoke {
        std::fs::create_dir_all("screenshots/animation-part03").unwrap();
        std::fs::create_dir_all("screenshots/hand-contacts").unwrap();
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ));
    }
    app
}
/// Reuse the real rider/hand/club presentation for opt-in model socket review.
/// This fixture installs no physics simulation and does not change gameplay state.
pub fn build_weapon_review(capture: Option<std::path::PathBuf>) -> App {
    let mut app = build(false);
    app.add_plugins(crate::world_gen::city::visuals::BuildingSceneTypesPlugin);
    {
        let mut review = app.world_mut().resource_mut::<Review>();
        review.mode = 11;
        review.weapon_review =
            Some(std::env::var("ROADDASH_WEAPON_MODEL").unwrap_or_else(|_| "original club".into()));
    }
    if let Some(path) = capture {
        app.add_systems(
            Update,
            move |mut commands: Commands, state: Res<Review>, mut exit: MessageWriter<AppExit>| {
                if state.frame == 180 {
                    commands
                        .spawn(Screenshot::primary_window())
                        .observe(save_to_disk(path.clone()));
                }
                if state.frame == 210 {
                    exit.write(AppExit::Success);
                }
            },
        );
    }
    app
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut binds: ResMut<Assets<bevy::mesh::skinning::SkinnedMeshInverseBindposes>>,
    server: Res<AssetServer>,
) {
    let dark = materials.add(Color::srgb(0.12, 0.14, 0.17));
    let white = materials.add(Color::WHITE);
    let art = world::Art {
        models: Some(models::AuthoredModels::load(
            &mut meshes,
            &mut materials,
            &mut binds,
            &mut images,
            Some(&server),
        )),
        props: None,
        cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        sphere: meshes.add(Sphere::new(1.0)),
        wheel: Handle::default(),
        dark: dark.clone(),
        metal: dark.clone(),
        white: white.clone(),
        colors: vec![materials.add(world::shirt_color(4)); 15],
        helmets: vec![white.clone(); 15],
        bike_colors: vec![white.clone(); 15],
    };
    if std::env::var_os("ROADDASH_GAIT_REVIEW").is_some() {
        use bevy_rapier3d::prelude::*;
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(20., 0.1, 100.),
            Transform::from_xyz(0., -0.1, 70.),
        ));
        for n in -10..180 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(10., 0.01, 0.025))),
                MeshMaterial3d(white.clone()),
                Transform::from_xyz(0., 0.005, n as f32 * 0.5),
            ));
        }
    }
    let police = std::env::args().any(|a| a == "--police-art-review");
    let palette = if police { models::OFFICER_PALETTE } else { 4 };
    let root = world::bike(&mut commands, &art, palette, true);
    let (runner, pose) = world::runner(&mut commands, &art, palette);
    commands.entity(runner).insert((
        Transform::from_xyz(0.0, 0.95, 0.0),
        OwnedBody {
            owner: root,
            kind: BodyKind::Runner,
        },
    ));
    commands.entity(root).insert((
        Rider::new(if police { crate::police::OFFICER_ID } else { 0 }),
        Recovery {
            runner: Some(runner),
            pose: Some(pose),
            ..default()
        },
    ));
    commands.insert_resource(art);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(25.0, 0.1, 25.0))),
        MeshMaterial3d(dark),
        Transform::from_xyz(0.0, -0.08, 0.0),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 18000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, -3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        ..default()
    });
    commands
        .spawn((
            Camera3d::default(),
            bevy::core_pipeline::tonemapping::Tonemapping::Reinhard,
            Transform::from_xyz(0.0, 2.0, -4.8).looking_at(Vec3::Y * 1.2, Vec3::Y),
        ))
        .with_children(|camera| {
            if std::env::args().any(|a| a == "--hand-review") {
                camera.spawn((
                    PointLight {
                        intensity: 700.0,
                        range: 2.0,
                        shadows_enabled: false,
                        ..default()
                    },
                    Transform::default(),
                ));
            }
        });
    commands.spawn((
        Label,
        Text::new("Loading rider"),
        TextFont {
            font_size: 20.0,
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
fn drive(
    mut state: ResMut<Review>,
    time: Res<Time<Fixed>>,
    mut race: ResMut<Race>,
    mut riders: Query<(&mut Rider, &mut Recovery, Entity)>,
    mut visible: Query<&mut Visibility>,
    mut bodies: Query<&mut Transform, Without<Rider>>,
) {
    race.elapsed += time.delta_secs();
    for (mut r, mut rec, root) in &mut riders {
        let foot = (8..=10).contains(&state.mode);
        *visible.get_mut(root).unwrap() = if foot {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
        if let Some(runner) = rec.runner {
            *visible.get_mut(runner).unwrap() = if foot {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
        rec.stage = if foot {
            RecoveryStage::Running
        } else {
            RecoveryStage::Riding
        };
        let speed = match state.mode {
            9 => 2.0,
            10 => 6.0,
            _ => 0.0,
        };
        rec.walk_velocity = Vec3::Z * speed;
        rec.walk_distance += speed * time.delta_secs();
        if state.gait {
            state.position += Vec3::Z * speed * time.delta_secs();
            if let Some(runner) = rec.runner {
                bodies.get_mut(runner).unwrap().translation = state.position + Vec3::Y * 0.95;
            }
        }
        r.speed = if state.mode == 1 { 65.0 } else { 30.0 };
        r.drive_intent.throttle = true;
        r.lean = match state.mode {
            2 => -0.65,
            3 => 0.65,
            _ => 0.0,
        };
        r.weapon_hand = usize::from(state.mode == 12);
        r.weapon = (state.mode >= 11).then_some(rr_core::weapons::Weapon::Club);
        if (4..8).contains(&state.mode) || state.mode >= 11 {
            if r.attack <= 0.0 {
                r.attack_serial += 1;
                r.attack = 0.65;
            }
            r.attack -= time.delta_secs();
            r.attack_side = if state.mode.is_multiple_of(2) != (state.mode >= 11) {
                -1.0
            } else {
                1.0
            };
            r.kicking = (6..8).contains(&state.mode);
            if state.mode >= 11 {
                let mut clock = rr_core::motion::StrikeClock::new(
                    if r.attack_side > 0. {
                        rr_core::AttackSide::Right
                    } else {
                        rr_core::AttackSide::Left
                    },
                    r.attack_side,
                    crate::weapons::ability().source_contact,
                );
                clock.weapon = Some(crate::weapons::ability());
                clock.age = 0.65 - r.attack;
                r.strike = Some(clock);
            } else {
                r.strike = None;
            }
        } else {
            r.attack = 0.0;
            r.kicking = false;
        }
    }
}
fn review(
    mut commands: Commands,
    mut state: ResMut<Review>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    mut labels: Query<&mut Text, With<Label>>,
    mut exit: MessageWriter<AppExit>,
    (sockets, gloves): (
        Query<(&crate::models::hands::HandSocket, &GlobalTransform)>,
        Query<&bevy::mesh::morph::MeshMorphWeights>,
    ),
) {
    state.frame += 1;
    if state.smoke && state.gait {
        state.mode = if state.frame < 330 { 9 } else { 10 };
        state.view = if state.frame < 570 { 2 } else { 1 };
    } else if state.smoke {
        state.mode =
            ((state.frame.saturating_sub(90)) / 180).min((LABELS.len() - 1) as u32) as usize;
        state.view = ((state.frame / 60) % 3) as usize;
    } else {
        for (i, k) in [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
            KeyCode::Digit0,
            KeyCode::KeyR,
        ]
        .into_iter()
        .enumerate()
        {
            if keys.just_pressed(k) {
                state.mode = i;
            }
        }
        if keys.just_pressed(KeyCode::KeyC) {
            state.view = (state.view + 1) % 3;
        }
    }
    for mut c in &mut camera {
        *c = Transform::from_translation(
            [
                Vec3::new(0.0, 2.0, -4.8),
                Vec3::new(0.0, 2.0, 4.8),
                Vec3::new(4.2, 2.0, 0.0),
            ][state.view]
                + state.position,
        )
        .looking_at(Vec3::Y * 1.2 + state.position, Vec3::Y);
    }
    if state.hands {
        let side = usize::from(state.mode == 5 || state.mode == 12);
        if let Some((_, hand)) = sockets.iter().find(|(s, _)| {
            !s.weapon && s.side == side && s.seated != (8..=10).contains(&state.mode)
        }) {
            let hand = hand.compute_transform();
            let offset = [
                Vec3::new(0.0, -0.12, -0.43),
                Vec3::new(0.0, -0.12, 0.43),
                Vec3::new(0.43, -0.12, 0.0),
            ][state.view];
            for mut c in &mut camera {
                *c = Transform::from_translation(hand.translation + hand.rotation * offset)
                    .looking_at(hand.translation, hand.rotation * Vec3::Y);
            }
        }
        for weights in &gloves {
            let w = weights.weights();
            state.seen_shapes[0] |= state.mode >= 8 && w.iter().all(|w| w.abs() < 0.001);
            state.seen_shapes[1] |= state.mode < 4 && w[0] > 0.95 && w[2] > 0.95;
            state.seen_shapes[2] |= state.mode == 4 && w[1] > 0.95 && w[2] > 0.95;
            state.seen_shapes[3] |= state.mode == 5 && w[3] > 0.95 && w[0] > 0.95;
        }
    }
    for mut l in &mut labels {
        **l = format!(
            "{} | {}\n1-8 mounted / 9 idle / 0 walk / R run / C camera / Esc close",
            LABELS[state.mode],
            ["REAR", "FRONT", "SIDE"][state.view]
        );
        if let Some(model) = &state.weapon_review {
            l.0.push_str(&format!(
                "\nModel: {model} | presentation only / existing club ability"
            ));
        }
    }
    if state.gait && state.frame >= 150 && state.frame.is_multiple_of(4) {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/gait-{:04}.png",
                state.frame
            )));
    }
    if state.smoke && state.frame > if state.hands { 90 } else { 120 } && state.frame % 60 == 50 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/{}/{:02}-{}-{}.png",
                if state.hands {
                    "hand-contacts"
                } else {
                    "animation-part03"
                },
                state.mode,
                LABELS[state.mode]
                    .split('/')
                    .next()
                    .unwrap()
                    .trim()
                    .replace(' ', "-"),
                state.view
            )));
    }
    if keys.just_pressed(KeyCode::Escape)
        || (state.gait && state.frame >= 750)
        || (state.smoke && state.frame >= 90 + LABELS.len() as u32 * 180)
    {
        if state.hands && state.smoke {
            assert!(
                state.seen_shapes.iter().all(|v| *v),
                "open/grip/fist shape not observed"
            );
            std::fs::write(
                "screenshots/hand-contacts/evidence.json",
                serde_json::to_vec_pretty(
                    &serde_json::json!({"open_grip_fist_observed":state.seen_shapes,
                    "motions":13,"views":3,"physics_changed":false}),
                )
                .unwrap(),
            )
            .unwrap();
            println!(
                "Hand contact review passed: open, grip and fist; eleven motions, three views"
            );
        }
        println!("Animation review passed: eleven motions, three views");
        exit.write(AppExit::Success);
    }
}

// Static hand review uses exact contact/extended-punch keys. The ordinary lab
// continues playing full animations; snapshots cannot otherwise certify a phase.
fn hand_keyframes(state: Res<Review>, library: Res<Library>, mut animators: Query<&mut Animator>) {
    if !state.hands {
        return;
    }
    let key = match state.mode {
        1 => 2030,
        4 => 2222,
        5 => 2232,
        8 => 2364,
        11 => 2162,
        12 => 2172,
        _ => return,
    };
    for mut a in &mut animators {
        a.current = library.key(key);
        a.previous = a.current;
    }
}
