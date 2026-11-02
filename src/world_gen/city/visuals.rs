//! Loads real standalone building GLBs. No facade images or image planes.
use bevy::prelude::*;
use serde::Deserialize;
use std::f32::consts::FRAC_PI_2;

pub struct BuildingSceneTypesPlugin;
impl Plugin for BuildingSceneTypesPlugin {
    fn build(&self, app: &mut App) {
        // The project disables Bevy automatic reflection; static glTF scenes need
        // these specific types to instantiate. No dependency features are changed.
        app.register_type::<Name>()
            .register_type::<Transform>()
            .register_type::<GlobalTransform>()
            .register_type::<bevy::transform::components::TransformTreeChanged>()
            .register_type::<Visibility>()
            .register_type::<InheritedVisibility>()
            .register_type::<ViewVisibility>()
            .register_type::<ChildOf>()
            .register_type::<Children>()
            .register_type::<Mesh3d>()
            .register_type::<MeshMaterial3d<StandardMaterial>>()
            .register_type::<bevy::camera::primitives::Aabb>()
            .register_type::<bevy::gltf::GltfExtras>()
            .register_type::<bevy::gltf::GltfSceneExtras>()
            .register_type::<bevy::gltf::GltfMeshExtras>()
            .register_type::<bevy::gltf::GltfMaterialExtras>()
            .register_type::<bevy::gltf::GltfMeshName>()
            .register_type::<bevy::gltf::GltfMaterialName>();
    }
}

#[derive(Deserialize)]
struct Designs {
    designs: Vec<Design>,
}
#[derive(Deserialize)]
struct Design {
    id: String,
    width: f32,
    depth: f32,
    height: f32,
    asset_type: String,
}

pub use rr_content::city::BuildingLot as Lot;
#[derive(Resource)]
pub struct ModelCatalog {
    pub scenes: Vec<[Option<Handle<Scene>>; 3]>,
    ids: Vec<String>,
    front_scenes: Vec<[Option<Handle<Scene>>; 3]>,
    pub facade_designs: usize,
    pub instances: usize,
}
impl ModelCatalog {
    pub fn requested_lods(&self) -> usize {
        self.scenes
            .iter()
            .chain(self.front_scenes.iter())
            .flatten()
            .flatten()
            .count()
    }
    pub fn ready(&self, server: &AssetServer) -> bool {
        self.scenes
            .iter()
            .chain(self.front_scenes.iter())
            .flatten()
            .flatten()
            .all(|h| {
                if let Some(bevy::asset::LoadState::Failed(error)) = server.get_load_state(h.id()) {
                    panic!("Required city asset failed: {error}");
                }
                server.is_loaded_with_dependencies(h.id())
            })
    }
}
#[derive(Component)]
pub struct BuildingInstance {
    kind: usize,
    lod: usize,
    proof: bool,
}

pub fn lod_for_distance(distance: f32) -> usize {
    if distance < 28.0 {
        0
    } else if distance < 75.0 {
        1
    } else {
        2
    }
}

#[cfg(test)]
pub fn placement(face: f32, centre_z: f32, inward: f32, width: f32) -> Transform {
    let scale = width / 3.2;
    Transform::from_xyz(face + inward * 3.0 * scale, 0.16, centre_z)
        .with_rotation(Quat::from_rotation_y(-inward * FRAC_PI_2))
        .with_scale(Vec3::splat(scale))
}

pub fn spawn(commands: &mut Commands, server: &AssetServer, lots: &[Lot], gallery: bool) {
    let designs: Designs = serde_json::from_str(include_str!(
        "../../../art/buildings/mesh_assets/catalog_designs.json"
    ))
    .expect("valid catalog designs");
    let ids: Vec<String> = designs.designs.iter().map(|d| d.id.clone()).collect();
    let scenes: Vec<[Option<Handle<Scene>>; 3]> = ids.iter().map(|_| [None, None, None]).collect();
    let mut front_scenes = scenes.clone();
    for lot in lots {
        let kind = ids
            .iter()
            .position(|id| *id == lot.family)
            .expect("actual facade ID");
        if front_scenes[kind][2].is_none() {
            front_scenes[kind][2] = Some(server.load(format!(
                "architecture://front_modules/{}_lod2.glb#Scene0",
                lot.family
            )));
        }
    }
    let facade_designs = designs
        .designs
        .iter()
        .filter(|d| d.asset_type == "facade")
        .count();
    let mut instances = 0;
    for lot in lots {
        let width = (lot.z_end - lot.z_start) / lot.bays as f32;
        let inward = lot.side as f32 * if lot.row == "avenue" { 1.0 } else { -1.0 };
        let kind = ids
            .iter()
            .position(|id| *id == lot.family)
            .expect("actual facade ID");
        let scale = lot.facade_scale;
        assert!(
            (designs.designs[kind].width * scale - width).abs() < 0.005,
            "front run/lot width mismatch"
        );
        assert!(
            (designs.designs[kind].height * scale - lot.height_m).abs() < 0.005,
            "front/roof height mismatch"
        );
        for bay in 0..lot.bays {
            let t =
                Transform::from_xyz(lot.front_x, 0.16, lot.z_start + (bay as f32 + 0.5) * width)
                    .with_rotation(Quat::from_rotation_y(-inward * FRAC_PI_2))
                    .with_scale(Vec3::splat(scale));
            commands.spawn((
                Name::new(format!("{} / {} / module {}", ids[kind], lot.id, bay)),
                BuildingInstance {
                    kind,
                    lod: 2,
                    proof: false,
                },
                SceneRoot(
                    front_scenes[kind][2]
                        .as_ref()
                        .expect("coarse scene")
                        .clone(),
                ),
                t,
            ));
            instances += 1;
        }
    }
    // Actual modeled returns on exposed cross-street ends; one shared body/roof remains per lot.
    for lot in lots {
        let kind = ids
            .iter()
            .position(|id| *id == lot.family)
            .expect("facade ID");
        let width = (lot.z_end - lot.z_start) / lot.bays as f32;
        let count = ((lot.depth - 0.8) / width).floor() as usize;
        for (exposed, station, direction) in [
            (lot.exposed_start, lot.z_start, -1.0),
            (lot.exposed_end, lot.z_end, 1.0),
        ] {
            if !exposed {
                continue;
            }
            for i in 0..count {
                commands.spawn((
                    Name::new(format!("{} / side return {}", lot.id, i)),
                    BuildingInstance {
                        kind,
                        lod: 2,
                        proof: false,
                    },
                    SceneRoot(
                        front_scenes[kind][2]
                            .as_ref()
                            .expect("coarse front")
                            .clone(),
                    ),
                    Transform::from_xyz(
                        lot.front_x + lot.inward() * (0.4 + (i as f32 + 0.5) * width),
                        0.16,
                        station,
                    )
                    .with_rotation(Quat::from_rotation_y(if direction > 0.0 {
                        0.0
                    } else {
                        std::f32::consts::PI
                    }))
                    .with_scale(Vec3::splat(lot.facade_scale)),
                ));
                instances += 1;
            }
        }
    }
    // Six specimens per page; structures stay in this gallery, not city frontages.
    for (kind, design) in designs.designs.iter().enumerate().filter(|_| gallery) {
        let page = kind / 6;
        let scale = (4.0 / design.width)
            .min(9.5 / design.height)
            .min(8.0 / design.depth);
        let face = 2000.0 + page as f32 * 50.0;
        commands.spawn((
            Name::new(format!("Catalog {}", design.id)),
            BuildingInstance {
                kind,
                lod: 2,
                proof: true,
            },
            SceneRoot(Handle::default()),
            Transform::from_xyz(
                face + design.depth * 0.5 * scale,
                0.16,
                (kind % 6) as f32 * 4.5 + 2.0,
            )
            .with_rotation(Quat::from_rotation_y(-FRAC_PI_2))
            .with_scale(Vec3::splat(scale)),
        ));
    }
    println!(
        "ACTUAL GLB CITY: {instances} modules, {facade_designs} facade designs; {} total catalog assets",
        ids.len()
    );
    commands.insert_resource(ModelCatalog {
        scenes,
        ids,
        front_scenes,
        facade_designs,
        instances,
    });
}

pub fn update_lod(
    mut catalog: ResMut<ModelCatalog>,
    server: Res<AssetServer>,
    cameras: Query<&Transform, With<Camera3d>>,
    mut buildings: Query<(
        &Transform,
        &mut BuildingInstance,
        &mut SceneRoot,
        &mut Visibility,
    )>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    for (t, mut b, mut scene, mut visible) in &mut buildings {
        let distance = t.translation.distance(camera.translation);
        let max_distance = if camera.translation.y > 150.0 {
            20000.0
        } else {
            700.0
        };
        *visible = if distance > max_distance {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        if distance > max_distance {
            continue;
        }
        let next = if b.proof && distance < 45.0 {
            0
        } else {
            lod_for_distance(distance)
        };
        // Hysteresis prevents repeated scene replacement near an LOD boundary.
        if next != b.lod && (distance - 28.0).abs() > 3.0 && (distance - 75.0).abs() > 5.0 {
            let id = catalog.ids[b.kind].clone();
            let source = if b.proof { "models" } else { "front_modules" };
            let scenes = if b.proof {
                &mut catalog.scenes
            } else {
                &mut catalog.front_scenes
            };
            if scenes[b.kind][next].is_none() {
                scenes[b.kind][next] =
                    Some(server.load(format!("architecture://{source}/{id}_lod{next}.glb#Scene0")));
            }
            let handle = scenes[b.kind][next].as_ref().expect("requested scene");
            if server.is_loaded_with_dependencies(handle.id()) {
                scene.0 = handle.clone();
                b.lod = next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mounted_glbs_face_the_street_and_preserve_uniform_scale() {
        for side in [-1.0, 1.0] {
            let t = placement(side * 10.2, 20.0, side, 2.8);
            let front = t.transform_point(Vec3::new(0.0, 0.0, 3.0));
            assert!((front.x - side * 10.2).abs() < 1e-5);
            assert_eq!(t.scale.x, t.scale.y);
            assert_eq!(t.scale.y, t.scale.z);
            let facing = t.rotation * Vec3::Z;
            assert!((facing.x + side).abs() < 1e-5);
        }
    }
    #[test]
    fn near_models_keep_geometry_and_far_models_use_coarse_lod() {
        assert_eq!(lod_for_distance(10.0), 0);
        assert_eq!(lod_for_distance(50.0), 1);
        assert_eq!(lod_for_distance(200.0), 2);
    }
    #[test]
    fn static_gltf_components_can_be_cloned_without_automatic_reflection_features() {
        let mut app = App::new();
        app.add_plugins(BuildingSceneTypesPlugin);
        let mut source = World::new();
        source.spawn((
            Name::new("Imported static GLB mesh"),
            Transform::default(),
            Visibility::Inherited,
            Mesh3d::default(),
            MeshMaterial3d::<StandardMaterial>::default(),
            bevy::gltf::GltfMeshName("Building geometry".into()),
            bevy::camera::primitives::Aabb::default(),
        ));
        let scene = Scene::new(source);
        let registry = app.world().resource::<AppTypeRegistry>().clone();
        let mut map = bevy::ecs::entity::EntityHashMap::default();
        assert!(
            scene
                .write_to_world_with(app.world_mut(), &mut map, &registry)
                .is_ok()
        );
        assert_eq!(map.len(), 1);
    }
}
