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
}
#[derive(Component)]
struct Label;
const LABELS: [&str; 11] = [
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
    .insert_resource(Review { smoke, ..default() })
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .insert_resource(Race {
        phase: crate::game::Phase::Racing,
        ..default()
    })
    .add_systems(Startup, setup)
    .add_systems(Update, crate::vehicle::visuals)
    .add_systems(FixedUpdate, drive.before(super::update))
    .add_systems(Update, review.after(crate::vehicle::visuals));
    if smoke {
        std::fs::create_dir_all("screenshots/animation-part03").unwrap();
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
    let dark = materials.add(Color::srgb(0.12, 0.14, 0.17));
    let white = materials.add(Color::WHITE);
    let art = world::Art {
        models: Some(models::AuthoredModels::load(
            &mut meshes,
            &mut materials,
            &mut binds,
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
        bike_colors: vec![white; 15],
    };
    let root = world::bike(&mut commands, &art, 4, true);
    let (runner, pose) = world::runner(&mut commands, &art, 4);
    commands.entity(runner).insert((
        Transform::from_xyz(0.0, 0.95, 0.0),
        OwnedBody {
            owner: root,
            kind: BodyKind::Runner,
        },
    ));
    commands.entity(root).insert((
        Rider::new(0),
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
    commands.spawn((
        Camera3d::default(),
        bevy::core_pipeline::tonemapping::Tonemapping::Reinhard,
        Transform::from_xyz(0.0, 2.0, -4.8).looking_at(Vec3::Y * 1.2, Vec3::Y),
    ));
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
    state: Res<Review>,
    time: Res<Time<Fixed>>,
    mut race: ResMut<Race>,
    mut riders: Query<(&mut Rider, &mut Recovery, Entity)>,
    mut visible: Query<&mut Visibility>,
) {
    race.elapsed += time.delta_secs();
    for (mut r, mut rec, root) in &mut riders {
        let foot = state.mode >= 8;
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
        r.speed = if state.mode == 1 { 65.0 } else { 30.0 };
        r.drive_intent.throttle = true;
        r.lean = match state.mode {
            2 => -0.65,
            3 => 0.65,
            _ => 0.0,
        };
        if (4..8).contains(&state.mode) {
            if r.attack <= 0.0 {
                r.attack_serial += 1;
                r.attack = 0.65;
            }
            r.attack -= time.delta_secs();
            r.attack_side = if state.mode.is_multiple_of(2) {
                -1.0
            } else {
                1.0
            };
            r.kicking = state.mode >= 6;
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
) {
    state.frame += 1;
    if state.smoke {
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
            ][state.view],
        )
        .looking_at(Vec3::Y * 1.2, Vec3::Y);
    }
    for mut l in &mut labels {
        **l = format!(
            "{} | {}\n1-8 mounted / 9 idle / 0 walk / R run / C camera / Esc close",
            LABELS[state.mode],
            ["REAR", "FRONT", "SIDE"][state.view]
        );
    }
    if state.smoke && state.frame > 120 && state.frame % 60 == 50 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/animation-part03/{:02}-{}-{}.png",
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
        || (state.smoke && state.frame >= 90 + LABELS.len() as u32 * 180)
    {
        println!("Animation review passed: eleven motions, three views");
        exit.write(AppExit::Success);
    }
}
