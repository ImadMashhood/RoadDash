use crate::track::Track;
use bevy::prelude::*;
#[cfg(test)]
use bevy_rapier3d::prelude::*;

pub const RIDER_COLORS: [u32; 15] = [
    0xe7e4dd, 0xd5b43f, 0xd2b64c, 0x2957bb, 0xa44195, 0x379b47, 0xe2e2dd, 0xaabbd4, 0x743927,
    0x34373b, 0xc9b5d9, 0xe1bd34, 0x328e9f, 0xbe2927, 0xddbf32,
];
pub const HELMET_COLORS: [u32; 15] = [
    0x2436b4, 0xd2b437, 0xd3b23b, 0xd7d9da, 0xb32b28, 0x278a40, 0x285dc1, 0x222427, 0xb53b24,
    0xb7b43b, 0xd33b24, 0x2e8e43, 0x7aa441, 0xbbc4c7, 0xa5b33e,
];
pub const BIKE_COLORS: [u32; 15] = [
    0xb52322, 0x70378e, 0x343638, 0x2c973c, 0x743278, 0x21963b, 0xd6d6d1, 0x32449d, 0x773729,
    0x303236, 0x74379c, 0x39843b, 0x2587a2, 0xb12624, 0x6137a2,
];
pub fn shirt_color(index: usize) -> Color {
    let hex = RIDER_COLORS[index % RIDER_COLORS.len()];
    Color::srgb_u8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}
#[derive(Resource)]
pub struct Art {
    pub props: Option<crate::props::PropArt>,
    pub models: Option<crate::models::AuthoredModels>,
    pub helmets: Vec<Handle<StandardMaterial>>,
    pub bike_colors: Vec<Handle<StandardMaterial>>,
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
    mut track: ResMut<Track>,
    plan: Option<Res<crate::world_gen::WorldPlan>>,
    server: Option<Res<AssetServer>>,
    mut binds: Option<ResMut<Assets<bevy::mesh::skinning::SkinnedMeshInverseBindposes>>>,
) {
    let swatch = |hex: u32| Color::srgb_u8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8);
    // Unrendered world/physics fixtures have no asset server or GPU skin assets.
    let models = match (server.as_deref(), binds.as_deref_mut()) {
        (Some(server), Some(binds))
            if std::env::var("ROADDASH_LEGACY_MODELS").as_deref() != Ok("1") =>
        {
            Some(crate::models::AuthoredModels::load(
                &mut meshes,
                &mut materials,
                binds,
                Some(server),
            ))
        }
        _ => None,
    };
    let art = Art {
        props: server.as_deref().and_then(crate::props::PropArt::load),
        models,
        helmets: HELMET_COLORS
            .iter()
            .map(|&c| materials.add(swatch(c)))
            .collect(),
        bike_colors: BIKE_COLORS
            .iter()
            .map(|&c| materials.add(swatch(c)))
            .collect(),
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
        colors: (0..15).map(|i| materials.add(shirt_color(i))).collect(),
    };
    commands.insert_resource(ClearColor(Color::srgb(0.39, 0.62, 0.72)));
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.96, 0.98, 1.0),
        brightness: if track.city_surface.is_some() {
            2500.0
        } else {
            6500.0
        },
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            illuminance: if track.city_surface.is_some() {
                16000.0
            } else {
                25000.0
            },
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.75, -0.5, 0.0)),
    ));
    let generated = if track.definition.id.as_str() == "valley-run-01" {
        let builtin;
        let plan = if let Some(ref plan) = plan {
            &**plan
        } else {
            builtin = crate::world_gen::WorldPlan::from_json(
                crate::world_gen::WORLD_JSON,
                crate::world_gen::ARCHETYPES_JSON,
                &track,
            )
            .expect("world plan");
            &builtin
        };
        let settings = crate::world_gen::layout::Construction::from_json(
            crate::world_gen::layout::CONSTRUCTION_JSON,
            &track,
        )
        .expect("construction settings");
        let layout = crate::world_gen::layout::WorldLayout::generate(plan, &track, settings)
            .expect("generate every base-world parcel");
        track.earthworks = Some(Box::new(crate::world_gen::ground::Earthworks::new(
            &layout, &track,
        )));
        Some(layout)
    } else {
        None
    };
    if track.definition.id.as_str() == rr_content::city::TRACK_ID {
        crate::world_gen::city::spawn(
            &mut commands,
            &mut meshes,
            &mut materials,
            server.as_deref(),
            &art,
            &track,
        );
        commands.insert_resource(art);
        return;
    }
    crate::environment::build(
        &mut commands,
        &mut meshes,
        &mut materials,
        &track,
        &art,
        generated.as_ref(),
    );
    if let Some(layout) = generated {
        let builtin;
        let plan = if let Some(ref plan) = plan {
            &**plan
        } else {
            builtin = crate::world_gen::WorldPlan::from_json(
                crate::world_gen::WORLD_JSON,
                crate::world_gen::ARCHETYPES_JSON,
                &track,
            )
            .expect("world plan");
            &builtin
        };
        crate::world_gen::buildings::spawn(
            &mut commands,
            &mut meshes,
            &mut materials,
            &art,
            plan,
            &layout,
        );
        crate::world_gen::frontage::spawn(&mut commands, &mut meshes, &mut materials, &layout);
        commands.insert_resource(layout);
    }
    commands.insert_resource(art);
}

/// All riding models face local +Z. Their root is at road level.
#[derive(Component)]
pub struct SeatedRiderVisual;
#[derive(Component)]
pub struct RidingHumanPose {
    pub owner: Entity,
}

pub fn bike(commands: &mut Commands, art: &Art, color: usize, with_rider: bool) -> Entity {
    let root = commands
        .spawn((Transform::default(), Visibility::default()))
        .id();
    let mut children = Vec::new();
    if let Some(models) = &art.models {
        children.extend(models.motorcycle(commands, color));
    } else {
        for z in [-0.78, 0.78] {
            let wheel_start = children.len();
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
            for e in &children[wheel_start..] {
                commands
                    .entity(*e)
                    .insert(crate::vehicle::WheelVisual(z > 0.0));
            }
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
                art.bike_colors[color].clone(),
            ),
            (
                Vec3::new(0.0, 1.02, -0.52),
                Vec3::new(0.47, 0.16, 0.65),
                art.dark.clone(),
            ),
            (
                Vec3::new(0.0, 1.04, 0.87),
                Vec3::new(0.56, 0.48, 0.34),
                art.bike_colors[color].clone(),
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
    }
    let rider_start = children.len();
    if with_rider {
        for i in 0..crate::human::SPECS.len() {
            let pose = crate::human::seated(i);
            let e = crate::human::segment(commands, art, color, i, pose.translation, pose.rotation);
            if let Some((side, leg, shoulder)) = crate::human::limb(i) {
                commands.entity(e).insert(Arm {
                    leg,
                    lower_leg: matches!(i, 8 | 10),
                    side,
                    rest: pose,
                    shoulder,
                });
            }
            children.push(e);
        }
    }
    if with_rider {
        let mut rider_parts = children.split_off(rider_start);
        if let Some(models) = &art.models {
            let skins = models.human(commands, color, &rider_parts, true);
            for &skin in &skins {
                commands
                    .entity(skin)
                    .insert(crate::animation::Binding::Riding(root));
            }
            rider_parts.extend(skins);
        }
        for &entity in &rider_parts {
            commands.entity(entity).insert(SeatedRiderVisual);
        }
        let pose = commands
            .spawn((
                RidingHumanPose { owner: root },
                Transform::default(),
                Visibility::default(),
            ))
            .id();
        commands.entity(pose).add_children(&rider_parts);
        children.push(pose);
    }
    crate::runtime::attach_visuals(commands, root, &children);
    root
}

#[derive(Component)]
pub struct Arm {
    pub leg: bool,
    pub lower_leg: bool,
    pub side: f32,
    pub rest: Transform,
    pub shoulder: Vec3,
}

pub fn animate_riders(
    riders: Query<&crate::game::Rider>,
    poses: Query<&RidingHumanPose>,
    mut arms: Query<(&Arm, &ChildOf, &mut Transform)>,
) {
    for (arm, parent, mut transform) in &mut arms {
        if let Ok(pose) = poses.get(parent.parent())
            && let Ok(rider) = riders.get(pose.owner)
        {
            let extension =
                if rider.attack > 0.3 && rider.attack_side == -arm.side && rider.kicking == arm.leg
                {
                    ((0.65 - rider.attack) / 0.35 * std::f32::consts::PI)
                        .sin()
                        .max(0.0)
                } else {
                    0.0
                };
            if arm.leg {
                let reach = if arm.lower_leg { 0.68 } else { 0.28 };
                let target = arm.shoulder + Vec3::new(arm.side * reach, -0.04, 0.12);
                transform.translation = arm.rest.translation.lerp(target, extension);
                transform.rotation = arm.rest.rotation.slerp(
                    Quat::from_rotation_z(arm.side * std::f32::consts::FRAC_PI_2),
                    extension,
                );
            } else {
                let turn =
                    Quat::from_rotation_z(arm.side * extension * std::f32::consts::FRAC_PI_2);
                transform.translation = arm.shoulder + turn * (arm.rest.translation - arm.shoulder);
                transform.rotation = turn * arm.rest.rotation;
            }
        }
    }
}

#[derive(Component)]
pub struct RunnerLimb {
    pub side: f32,
    pub leg: bool,
    pub rest: Transform,
    pub pivot: Vec3,
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
    for (i, (position, _, _)) in crate::human::SPECS.iter().enumerate() {
        let e = crate::human::segment(commands, art, color, i, *position, Quat::IDENTITY);
        if let Some((side, leg, pivot)) = crate::human::limb(i) {
            commands.entity(e).insert(RunnerLimb {
                side,
                leg,
                rest: Transform::from_translation(*position),
                pivot,
            });
        }
        parts.push(e);
    }
    if let Some(models) = &art.models {
        let skins = models.human(commands, color, &parts, false);
        for &skin in &skins {
            commands
                .entity(skin)
                .insert(crate::animation::Binding::Runner(root));
        }
        parts.extend(skins);
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
        let turn = Quat::from_rotation_x(swing * limb.side * if limb.leg { 1.0 } else { -1.0 });
        transform.translation = limb.pivot + turn * (limb.rest.translation - limb.pivot);
        transform.rotation = turn * limb.rest.rotation;
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
