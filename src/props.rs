//! Authored prop presentation. Physical owners and the accepted ragdoll are never modified.
use bevy::{
    asset::io::AssetSourceBuilder, camera::visibility::NoFrustumCulling,
    mesh::skinning::SkinnedMesh, prelude::*, transform::helper::TransformHelper,
};
use rr_content::props::{Catalog, Prop};
use std::collections::{HashMap, HashSet};

pub const JSON: &str = include_str!("../content/props.json");
pub fn register_assets(app: &mut App) {
    let packaged = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("assets/props")));
    let source = packaged.filter(|p| p.is_dir()).unwrap_or_else(|| {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("art/props/mesh_assets")
    });
    app.register_asset_source(
        "props",
        AssetSourceBuilder::platform_default(&source.to_string_lossy(), None),
    );
}

pub struct PropArt {
    pub catalog: Catalog,
    pub server: Option<AssetServer>,
}
impl PropArt {
    pub fn load(server: &AssetServer) -> Option<Self> {
        (std::env::var("ROADDASH_LEGACY_PROPS").as_deref() != Ok("1")).then(|| Self {
            catalog: Catalog::from_json(JSON).expect("validated visual props"),
            server: Some(server.clone()),
        })
    }
    fn scene(&self, path: String) -> Handle<Scene> {
        self.server
            .as_ref()
            .map(|s| s.load(path))
            .unwrap_or_default()
    }
    pub fn named(&self, name: &str) -> &Prop {
        self.catalog
            .records
            .iter()
            .find(|p| p.name == name)
            .expect("known prop name")
    }
    pub fn choose(&self, builder: &str, seed: usize) -> &Prop {
        let count = self
            .catalog
            .records
            .iter()
            .filter(|p| p.builder == builder)
            .count();
        self.catalog
            .records
            .iter()
            .filter(|p| p.builder == builder)
            .nth(seed % count)
            .expect("known semantic prop builder")
    }
    pub fn visual(
        &self,
        commands: &mut Commands,
        prop: &Prop,
        transform: Transform,
        family: &'static str,
    ) -> Entity {
        let path = format!("props://{}#Scene0", prop.files[2]);
        commands
            .spawn((
                Name::new(format!("prop {} / {}", prop.id, prop.name)),
                PropVisual {
                    id: prop.id.clone(),
                    files: prop.files.clone(),
                    lod: 2,
                    family,
                },
                SceneRoot(self.scene(path)),
                transform,
                Visibility::default(),
            ))
            .id()
    }
    pub fn fitted(
        &self,
        commands: &mut Commands,
        prop: &Prop,
        base: Vec3,
        rotation: Quat,
        size: Vec3,
        family: &'static str,
    ) -> Entity {
        let scale = size / Vec3::from_array(prop.size);
        let offset = Vec3::new(
            -prop.center[0],
            prop.size[1] * 0.5 - prop.center[1],
            -prop.center[2],
        ) * scale;
        self.visual(
            commands,
            prop,
            Transform::from_translation(base + rotation * offset)
                .with_rotation(rotation)
                .with_scale(scale),
            family,
        )
    }
    pub fn vehicle(&self, commands: &mut Commands, seed: usize) -> Entity {
        let models = ["hatchback", "sedan", "sports_car", "taxi", "van", "pickup"];
        let p = self.choose(models[seed % models.len()], seed / models.len());
        self.fitted(
            commands,
            p,
            Vec3::Y * 0.05,
            Quat::IDENTITY,
            Vec3::new(1.85, 1.65, 3.8),
            "traffic",
        )
    }
    pub fn skin(&self, commands: &mut Commands, prop: &Prop, drivers: &[Entity]) -> Entity {
        assert_eq!(prop.bones, drivers.len());
        commands
            .spawn((
                Name::new(format!("prop actor skin {}", prop.name)),
                SceneRoot(self.scene(format!("props://{}#Scene0", prop.skin_files[0]))),
                Transform::IDENTITY,
                Visibility::default(),
                ActorBinding {
                    drivers: drivers.to_vec(),
                    files: prop.skin_files.clone(),
                    lod: 0,
                },
                ActorAppearance(prop.name.clone()),
            ))
            .id()
    }
}
#[derive(Component)]
pub struct PropVisual {
    pub id: String,
    files: [String; 3],
    lod: usize,
    pub family: &'static str,
}
#[derive(Component, Clone)]
pub struct ActorAppearance(pub String);
#[derive(Component)]
pub(crate) struct ActorBinding {
    drivers: Vec<Entity>,
    files: Vec<String>,
    lod: usize,
}
#[derive(Resource, Default)]
struct SceneCache(HashMap<String, Handle<Scene>>);
#[derive(Resource, Default)]
pub struct PropStats {
    pub rendered: usize,
    pub skins: usize,
}

fn lods(
    server: Res<AssetServer>,
    mut cache: ResMut<SceneCache>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    mut props: Query<(
        &GlobalTransform,
        &mut PropVisual,
        &mut SceneRoot,
        &mut Visibility,
    )>,
    mut stats: ResMut<PropStats>,
    bindings: Query<(), With<ActorBinding>>,
) {
    let Some(camera) = cameras.iter().next() else {
        return;
    };
    stats.rendered = props.iter().count();
    stats.skins = bindings.iter().count();
    for (transform, mut prop, mut scene, mut visible) in &mut props {
        let distance = transform.translation().distance(camera.translation());
        *visible = if distance > 500.0 {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        let next = if distance < 32.0 {
            0
        } else if distance < 95.0 {
            1
        } else {
            2
        };
        let boundary = match (prop.lod, next) {
            (0, _) => 36.0,
            (1, 0) => 28.0,
            (1, _) => 105.0,
            (2, _) => 85.0,
            _ => 95.0,
        };
        let change =
            next < prop.lod && distance < boundary || next > prop.lod && distance > boundary;
        if !change {
            continue;
        }
        let path = format!("props://{}#Scene0", prop.files[next]);
        let handle = cache
            .0
            .entry(path.clone())
            .or_insert_with(|| server.load(path.clone()))
            .clone();
        if let Some(bevy::asset::LoadState::Failed(error)) = server.get_load_state(handle.id()) {
            panic!("Prop scene failed {path}: {error}");
        }
        if server.is_loaded_with_dependencies(handle.id()) {
            scene.0 = handle;
            prop.lod = next;
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn drive_actor_skins(
    mut commands: Commands,
    bindings: Query<(Entity, &ActorBinding, &Visibility)>,
    children: Query<&Children>,
    skins: Query<(&SkinnedMesh, Option<&NoFrustumCulling>)>,
    parents: Query<&ChildOf>,
    mut transforms: ParamSet<(
        TransformHelper,
        Query<&mut Transform, Without<bevy_rapier3d::prelude::RigidBody>>,
    )>,
    mut updates: Local<Vec<(Entity, Transform)>>,
    mut seen: Local<HashSet<Entity>>,
) {
    updates.clear();
    {
        let helper = transforms.p0();
        for (root, binding, visible) in &bindings {
            if *visible == Visibility::Hidden {
                continue;
            }
            seen.clear();
            for child in children.iter_descendants(root) {
                let Ok((skin, no_culling)) = skins.get(child) else {
                    continue;
                };
                if no_culling.is_none() {
                    commands.entity(child).insert(NoFrustumCulling);
                }
                for (&joint, &driver) in skin.joints.iter().zip(&binding.drivers) {
                    if !seen.insert(joint) {
                        continue;
                    }
                    assert!(
                        !binding.drivers.contains(&joint),
                        "visual skin cannot write authoritative drivers"
                    );
                    let Ok(world) = helper.compute_global_transform(driver) else {
                        continue;
                    };
                    let Ok(parent) = parents.get(joint) else {
                        continue;
                    };
                    let Ok(parent_world) = helper.compute_global_transform(parent.parent()) else {
                        continue;
                    };
                    updates.push((
                        joint,
                        Transform::from_matrix(
                            parent_world.to_matrix().inverse() * world.to_matrix(),
                        ),
                    ));
                }
            }
        }
    }
    let mut query = transforms.p1();
    for (joint, pose) in updates.drain(..) {
        if let Ok(mut t) = query.get_mut(joint) {
            *t = pose;
        }
    }
}

#[allow(clippy::type_complexity)]
fn actor_lods(
    server: Res<AssetServer>,
    mut cache: ResMut<SceneCache>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    mut actors: Query<(
        &GlobalTransform,
        &mut ActorBinding,
        &mut SceneRoot,
        &mut Visibility,
    )>,
) {
    let Some(camera) = cameras.iter().next() else {
        return;
    };
    for (transform, mut binding, mut scene, mut visible) in &mut actors {
        let distance = transform.translation().distance(camera.translation());
        *visible = if distance > 250.0 {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        let next = if distance > 90.0 {
            1
        } else if distance < 70.0 {
            0
        } else {
            binding.lod
        };
        if next != binding.lod {
            let path = format!("props://{}#Scene0", binding.files[next]);
            let handle = cache
                .0
                .entry(path.clone())
                .or_insert_with(|| server.load(path))
                .clone();
            if server.is_loaded_with_dependencies(handle.id()) {
                scene.0 = handle;
                binding.lod = next;
            }
        }
    }
}

pub struct PropPlugin;

#[allow(clippy::too_many_arguments)]
fn record_native_intake(
    mut exits: MessageReader<AppExit>,
    track: Res<crate::track::Track>,
    art: Res<crate::world::Art>,
    props: Query<&PropVisual>,
    actors: Query<&ActorAppearance, With<ActorBinding>>,
    skins: Query<&SkinnedMesh>,
) {
    if exits.read().next().is_none()
        || !std::env::args().any(|a| {
            matches!(
                a.as_str(),
                "--world-survey" | "--actor-smoke-test" | "--smoke-test"
            )
        })
    {
        return;
    }
    let Some(art) = &art.props else { return };
    let mut ids = std::collections::BTreeSet::new();
    let mut families = std::collections::BTreeMap::new();
    for p in &props {
        ids.insert(p.id.clone());
        *families.entry(p.family).or_insert(0) += 1;
    }
    for a in &actors {
        ids.insert(art.named(&a.0).id.clone());
    }
    let folder = std::path::Path::new("docs/testing/props-in-game");
    std::fs::create_dir_all(folder).expect("native prop evidence folder");
    let report = serde_json::json!({"track":track.definition.id,"selected_ids":ids,"families":families,"actor_bindings":actors.iter().count(),"loaded_skinned_meshes_including_riders":skins.iter().count(),"production_art_approval":false});
    std::fs::write(
        folder.join(format!("native-{}.json", track.definition.id.as_str())),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .expect("native prop evidence");
}
impl Plugin for PropPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SkinnedMesh>()
            .init_resource::<SceneCache>()
            .init_resource::<PropStats>()
            .add_systems(Update, (lods, actor_lods).chain())
            .add_systems(Last, record_native_intake)
            .add_systems(
                PostUpdate,
                drive_actor_skins
                    .after(crate::runtime::interpolate_visuals)
                    .before(bevy::transform::TransformSystems::Propagate),
            );
    }
}

pub fn dress_city(commands: &mut Commands, art: &PropArt, city: &rr_content::city::CityPlan) {
    use bevy_rapier3d::prelude::*;
    let mut index = 0;
    for chunk in &city.chunks {
        for (name, center, size, _) in &chunk.boxes {
            let center = Vec3::from_array(*center);
            let size = Vec3::from_array(*size);
            match name.as_str() {
                "trunk" => {
                    let tree = art.choose(
                        if index % 4 == 0 {
                            "conifer"
                        } else {
                            "broadleaf"
                        },
                        index,
                    );
                    art.fitted(
                        commands,
                        tree,
                        Vec3::new(center.x, 0.16, center.z),
                        Quat::IDENTITY,
                        Vec3::new(2.2, 5.3, 2.2),
                        "tree",
                    );
                    index += 1;
                }
                "lamp" => {
                    let lamp = art.choose("street_lamp", index);
                    art.fitted(
                        commands,
                        lamp,
                        center - Vec3::Y * size.y / 2.0,
                        Quat::from_rotation_y(if center.x > 0.0 {
                            std::f32::consts::FRAC_PI_2
                        } else {
                            -std::f32::consts::FRAC_PI_2
                        }),
                        Vec3::new(0.6, size.y, 0.9),
                        "lamp",
                    );
                }
                _ => {}
            }
        }
    }
    for socket in crate::world_gen::city::props::sockets(&city.config) {
        let prop = art.choose(socket.builder, socket.seed);
        let visual = art.fitted(
            commands,
            prop,
            Vec3::ZERO,
            Quat::IDENTITY,
            socket.size,
            "furniture",
        );
        let owner = commands
            .spawn((
                Name::new(format!("sidewalk {}", prop.name)),
                Transform::from_translation(socket.position).with_rotation(socket.rotation),
                Visibility::default(),
                RigidBody::Fixed,
                crate::world::Obstacle,
                Collider::compound(vec![(
                    Vec3::Y * socket.size.y / 2.0,
                    Quat::IDENTITY,
                    Collider::cuboid(
                        socket.size.x / 2.0,
                        socket.size.y / 2.0,
                        socket.size.z / 2.0,
                    ),
                )]),
                Friction::coefficient(0.7),
            ))
            .id();
        commands.entity(owner).add_child(visual);
    }
}
