use crate::track::Track;
use bevy::prelude::*;
#[cfg(test)]
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
pub struct Art {
    pub cube: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub wheel: Handle<Mesh>,
    pub dark: Handle<StandardMaterial>,
    pub metal: Handle<StandardMaterial>,
    pub white: Handle<StandardMaterial>,
    pub colors: Vec<Handle<StandardMaterial>>,
}

pub fn part(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    pos: Vec3,
    scale: Vec3,
    rotation: Quat,
) -> Entity {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform {
                translation: pos,
                scale,
                rotation,
            },
        ))
        .id()
}

/// Volumetric scenery that can stop bikes and receive ragdoll impacts.
#[derive(Component)]
pub struct Obstacle;
#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct SceneryFootprint {
    pub radius: f32,
}

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    track: Res<Track>,
) {
    let art = Art {
        cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        sphere: meshes.add(Sphere::new(1.0).mesh().uv(16, 10)),
        wheel: meshes.add(Cylinder::new(0.40, 0.20).mesh().resolution(20)),
        dark: materials.add(Color::srgb(0.025, 0.035, 0.045)),
        metal: materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.48, 0.52),
            metallic: 0.85,
            perceptual_roughness: 0.24,
            ..default()
        }),
        white: materials.add(Color::srgb(0.85, 0.9, 0.89)),
        colors: [
            0x23e3bd, 0xff644a, 0x77aaff, 0xffcb55, 0xc792ff, 0xeb6ab0, 0xb3df60, 0xfaf6ed,
        ]
        .map(|hex| {
            materials.add(Color::srgb_u8(
                (hex >> 16) as u8,
                (hex >> 8) as u8,
                hex as u8,
            ))
        })
        .to_vec(),
    };
    commands.insert_resource(ClearColor(Color::srgb(0.39, 0.62, 0.72)));
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.96, 0.98, 1.0),
        brightness: 6500.0,
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            illuminance: 25000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.75, -0.5, 0.0)),
    ));
    crate::environment::build(&mut commands, &mut meshes, &mut materials, &track, &art);
    commands.insert_resource(art);
}

/// All riding models face local +Z. Their root is at road level.
#[derive(Component)]
pub struct SeatedRiderVisual;

pub fn bike(commands: &mut Commands, art: &Art, color: usize, with_rider: bool) -> Entity {
    let root = commands
        .spawn((Transform::default(), Visibility::default()))
        .id();
    let mut children = Vec::new();
    for z in [-0.78, 0.78] {
        children.push(part(
            commands,
            art.wheel.clone(),
            art.dark.clone(),
            Vec3::new(0.0, 0.42, z),
            Vec3::ONE,
            Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
        ));
        children.push(part(
            commands,
            art.wheel.clone(),
            art.metal.clone(),
            Vec3::new(0.0, 0.42, z),
            Vec3::new(0.52, 1.05, 0.52),
            Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
        ));
    }
    for (p, s, mat) in [
        (
            Vec3::new(0.0, 0.77, 0.0),
            Vec3::new(0.40, 0.38, 1.15),
            art.metal.clone(),
        ),
        (
            Vec3::new(0.0, 1.03, 0.24),
            Vec3::new(0.57, 0.38, 0.73),
            art.colors[color].clone(),
        ),
        (
            Vec3::new(0.0, 1.02, -0.52),
            Vec3::new(0.47, 0.16, 0.65),
            art.dark.clone(),
        ),
        (
            Vec3::new(0.0, 1.04, 0.87),
            Vec3::new(0.56, 0.48, 0.34),
            art.colors[color].clone(),
        ),
        (
            Vec3::new(0.0, 1.3, 0.60),
            Vec3::new(0.93, 0.07, 0.08),
            art.metal.clone(),
        ),
        (
            Vec3::new(0.0, 1.01, 1.055),
            Vec3::new(0.36, 0.15, 0.03),
            art.white.clone(),
        ),
    ] {
        children.push(part(commands, art.cube.clone(), mat, p, s, Quat::IDENTITY));
    }
    let rider_start = children.len();
    if with_rider {
        children.push(part(
            commands,
            art.cube.clone(),
            art.colors[color].clone(),
            Vec3::new(0.0, 1.40, -0.12),
            Vec3::new(0.58, 0.65, 0.38),
            Quat::IDENTITY,
        ));
        for side in [-1.0, 1.0] {
            children.push(part(
                commands,
                art.cube.clone(),
                art.dark.clone(),
                Vec3::new(side * 0.31, 0.96, -0.25),
                Vec3::new(0.21, 0.68, 0.28),
                Quat::from_rotation_x(-0.45),
            ));
            let arm = part(
                commands,
                art.cube.clone(),
                art.colors[color].clone(),
                Vec3::new(side * 0.36, 1.43, 0.30),
                Vec3::new(0.19, 0.23, 0.70),
                Quat::from_rotation_x(0.35),
            );
            commands.entity(arm).insert(Arm(side));
            children.push(arm);
        }
        children.push(part(
            commands,
            art.sphere.clone(),
            art.white.clone(),
            Vec3::new(0.0, 1.96, 0.08),
            Vec3::new(0.29, 0.32, 0.29),
            Quat::IDENTITY,
        ));
        children.push(part(
            commands,
            art.cube.clone(),
            art.dark.clone(),
            Vec3::new(0.0, 1.99, 0.32),
            Vec3::new(0.46, 0.14, 0.11),
            Quat::IDENTITY,
        ));
    }
    for &entity in &children[rider_start..] {
        commands.entity(entity).insert(SeatedRiderVisual);
    }
    crate::runtime::attach_visuals(commands, root, &children);
    root
}

#[derive(Component)]
pub struct Arm(pub f32);

pub fn animate_riders(
    riders: Query<&crate::game::Rider>,
    visuals: Query<&crate::runtime::VisualRoot>,
    mut arms: Query<(&Arm, &ChildOf, &mut Transform)>,
) {
    for (arm, parent, mut transform) in &mut arms {
        if let Ok(visual) = visuals.get(parent.parent())
            && let Ok(rider) = riders.get(visual.owner)
        {
            let extension = if rider.attack > 0.3 && rider.attack_side == -arm.0 {
                ((0.65 - rider.attack) / 0.35 * std::f32::consts::PI)
                    .sin()
                    .max(0.0)
            } else {
                0.0
            };
            transform.translation.x = arm.0 * (0.36 + extension * 0.45);
            transform.rotation =
                Quat::from_rotation_y(arm.0 * extension * 1.5) * Quat::from_rotation_x(0.35);
        }
    }
}

#[derive(Component)]
pub struct RunnerLimb {
    pub side: f32,
    pub leg: bool,
}

/// Upright rider model used for getting up, running, and remounting.
/// The root is at capsule center; the pose pivot is at the feet.
pub fn runner(commands: &mut Commands, art: &Art, color: usize) -> (Entity, Entity) {
    let root = commands
        .spawn((Transform::default(), Visibility::default()))
        .id();
    let pose = commands
        .spawn((Transform::from_xyz(0.0, -0.95, 0.0), Visibility::default()))
        .id();
    let mut parts = Vec::new();
    parts.push(part(
        commands,
        art.cube.clone(),
        art.colors[color].clone(),
        Vec3::new(0.0, 1.35, 0.0),
        Vec3::new(0.55, 0.65, 0.32),
        Quat::IDENTITY,
    ));
    parts.push(part(
        commands,
        art.sphere.clone(),
        art.white.clone(),
        Vec3::new(0.0, 1.95, 0.0),
        Vec3::new(0.28, 0.31, 0.28),
        Quat::IDENTITY,
    ));
    parts.push(part(
        commands,
        art.cube.clone(),
        art.dark.clone(),
        Vec3::new(0.0, 1.96, 0.25),
        Vec3::new(0.45, 0.16, 0.08),
        Quat::IDENTITY,
    ));
    for side in [-1.0, 1.0] {
        for leg in [true, false] {
            let pivot = commands
                .spawn((
                    Transform::from_xyz(
                        side * if leg { 0.17 } else { 0.39 },
                        if leg { 1.0 } else { 1.57 },
                        0.0,
                    ),
                    Visibility::default(),
                    RunnerLimb { side, leg },
                ))
                .id();
            let visual = part(
                commands,
                art.cube.clone(),
                if leg {
                    art.dark.clone()
                } else {
                    art.colors[color].clone()
                },
                Vec3::new(0.0, if leg { -0.475 } else { -0.38 }, 0.0),
                Vec3::new(0.22, if leg { 0.95 } else { 0.78 }, 0.25),
                Quat::IDENTITY,
            );
            commands.entity(pivot).add_child(visual);
            parts.push(pivot);
        }
    }
    commands.entity(pose).add_children(&parts);
    crate::runtime::attach_visuals(commands, root, &[pose]);
    (root, pose)
}

pub fn animate_runners(
    time: Res<Time>,
    race: Res<crate::game::Race>,
    recovery: Query<&crate::recovery::Recovery>,
    mut limbs: Query<(&RunnerLimb, &ChildOf, &mut Transform)>,
    parents: Query<&ChildOf>,
    owners: Query<&crate::recovery::OwnedBody>,
    visuals: Query<&crate::runtime::VisualRoot>,
) {
    if race.phase != crate::game::Phase::Racing {
        return;
    }
    for (limb, pose, mut transform) in &mut limbs {
        let Ok(root) = parents.get(pose.parent()) else {
            continue;
        };
        let Ok(visual) = visuals.get(root.parent()) else {
            continue;
        };
        let Ok(owner) = owners.get(visual.owner) else {
            continue;
        };
        let Ok(recovery) = recovery.get(owner.owner) else {
            continue;
        };
        let swing = if recovery.stage == crate::recovery::RecoveryStage::Running {
            (time.elapsed_secs() * 13.0).sin() * 0.65
        } else {
            0.0
        };
        transform.rotation =
            Quat::from_rotation_x(swing * limb.side * if limb.leg { 1.0 } else { -1.0 });
    }
}

/// Check real mesh footprints against every section of the road.
pub fn validate_scenery(track: Res<Track>, scenery: Query<(&Transform, &SceneryFootprint)>) {
    for (transform, footprint) in &scenery {
        assert!(
            track.scenery_fits(transform.translation, footprint.radius, 1.0),
            "Scenery overlaps the road at {:?}",
            transform.translation
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scenery_is_solid_and_every_footprint_clears_the_whole_circuit() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Track>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Startup, (setup_world, validate_scenery).chain());
        app.update();
        let mut footprints = app
            .world_mut()
            .query::<(&Transform, &SceneryFootprint, &Collider)>();
        assert!(footprints.iter(app.world()).count() > 10);
        for (t, footprint, _) in footprints.iter(app.world()) {
            assert!(app.world().resource::<Track>().scenery_fits(
                t.translation,
                footprint.radius,
                1.0
            ));
        }
        for (_, collider) in app
            .world_mut()
            .query::<(&Obstacle, Option<&Collider>)>()
            .iter(app.world())
        {
            assert!(collider.is_some());
        }
    }
}
