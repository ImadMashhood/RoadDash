//! Authored meshes driven by existing visual joints. No physics systems or components.
use crate::{human, world};
mod articulation;
use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::NoFrustumCulling,
    mesh::{
        Indices, PrimitiveTopology, VertexAttributeValues,
        skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
    },
    prelude::*,
    transform::helper::TransformHelper,
};

/// Prefer assets beside a packaged binary; Cargo builds use the source checkout.
/// Bevy otherwise resolves relative to target/{debug,release}, not the repository.
pub fn asset_directory() -> String {
    let beside_binary = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("assets")));
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let cwd = std::env::current_dir().ok().map(|p| p.join("assets"));
    beside_binary
        .into_iter()
        .chain([source])
        .chain(cwd)
        .find(|p| p.is_dir())
        .unwrap_or_else(|| std::path::PathBuf::from("assets"))
        .to_string_lossy()
        .into_owned()
}

pub const BUNDLE: &[u8] = include_bytes!("../assets/models/riding.rdmesh");
#[derive(Clone)]
struct Part {
    mesh: Handle<Mesh>,
    materials: Vec<Handle<StandardMaterial>>,
    group: String,
    rest: Transform,
}
pub struct AuthoredModels {
    parts: Vec<Part>,
    standing_bind: Handle<SkinnedMeshInverseBindposes>,
    seated_bind: Handle<SkinnedMeshInverseBindposes>,
    textures: Vec<Handle<Image>>,
    standing_frames: [Transform; 16],
    seated_frames: [Transform; 16],
}
#[derive(Component)]
pub struct AuthoredHuman;
#[derive(Component)]
pub struct AuthoredMotorcycle;
#[derive(Component)]
struct AuthoredWheel {
    rest: Transform,
    front: bool,
}

impl AuthoredModels {
    pub fn load(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        binds: &mut Assets<SkinnedMeshInverseBindposes>,
        server: Option<&AssetServer>,
    ) -> Self {
        let bindings = rr_content::riding_model::Bindings::from_json(include_str!(
            "../assets/models/rider-bindings.json"
        ))
        .expect("validated source bind frames");
        let standing_frames = bindings
            .standing
            .map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let seated_frames = bindings
            .seated
            .map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let standing_frames = articulation::bind_frames(standing_frames);
        let seated_frames = articulation::bind_frames(seated_frames);
        let decoded =
            rr_content::riding_model::decode(BUNDLE).expect("validated authored visual bundle");
        let base = server.map(|s| s.load("textures/rider-base.png"));
        let normal = server.map(|s| {
            s.load_with_settings(
                "textures/rider-normal.png",
                |settings: &mut bevy::image::ImageLoaderSettings| settings.is_srgb = false,
            )
        });
        let roughness = server.map(|s| {
            s.load_with_settings(
                "textures/rider-roughness.png",
                |settings: &mut bevy::image::ImageLoaderSettings| settings.is_srgb = false,
            )
        });
        let mut parts = Vec::new();
        for mut data in decoded {
            if matches!(data.group.as_str(), "standing" | "seated") {
                let standing = data.group == "standing";
                articulation::weights(
                    &mut data,
                    if standing {
                        &standing_frames
                    } else {
                        &seated_frames
                    },
                );
            }
            let wheel = matches!(data.group.as_str(), "front-wheel" | "rear-wheel");
            let rest = if wheel {
                Transform::from_xyz(
                    0.0,
                    0.42,
                    if data.group == "front-wheel" {
                        0.876
                    } else {
                        -0.876
                    },
                )
            } else {
                Transform::IDENTITY
            };
            if wheel {
                for p in &mut data.positions {
                    *p = (Vec3::from_array(*p) - rest.translation).to_array();
                }
            }
            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs);
            mesh.insert_indices(Indices::U32(data.indices));
            let skinned = matches!(data.group.as_str(), "standing" | "seated");
            if skinned {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_JOINT_INDEX,
                    VertexAttributeValues::Uint16x4(data.joints),
                );
                mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, data.weights);
                mesh.generate_tangents()
                    .expect("authored rider UVs support tangent normals");
            }
            let region = &data.material.region;
            let variants = if matches!(region.as_str(), "shirt" | "helmet" | "paint") {
                15
            } else {
                1
            };
            let mut palette = Vec::new();
            for color in 0..variants {
                let tint = match region.as_str() {
                    "shirt" => world::shirt_color(color),
                    "helmet" => swatch(world::HELMET_COLORS[color]),
                    "paint" => swatch(world::BIKE_COLORS[color]),
                    _ => Color::WHITE,
                };
                let mat = if skinned {
                    StandardMaterial {
                        base_color: tint,
                        base_color_texture: base.clone(),
                        normal_map_texture: normal.clone(),
                        metallic_roughness_texture: roughness.clone(),
                        metallic: if region == "visor" { 0.9 } else { 0.12 },
                        perceptual_roughness: if region == "visor" { 0.2 } else { 0.85 },
                        ..default()
                    }
                } else {
                    let rgba = data.material.color.unwrap_or([1.0; 4]);
                    StandardMaterial {
                        base_color: if region == "paint" {
                            tint
                        } else {
                            Color::linear_rgba(rgba[0], rgba[1], rgba[2], rgba[3])
                        },
                        metallic: data.material.metallic,
                        perceptual_roughness: data.material.roughness,
                        alpha_mode: if data.material.alpha < 0.99 {
                            AlphaMode::Blend
                        } else {
                            AlphaMode::Opaque
                        },
                        ..default()
                    }
                };
                palette.push(materials.add(mat));
            }
            parts.push(Part {
                mesh: meshes.add(mesh),
                materials: palette,
                group: data.group,
                rest,
            });
        }
        Self {
            parts,
            textures: [base, normal, roughness].into_iter().flatten().collect(),
            standing_bind: binds.add(SkinnedMeshInverseBindposes::from(
                standing_frames
                    .iter()
                    .map(|t| t.to_matrix().inverse())
                    .collect::<Vec<_>>(),
            )),
            seated_bind: binds.add(SkinnedMeshInverseBindposes::from(
                seated_frames
                    .iter()
                    .map(|t| t.to_matrix().inverse())
                    .collect::<Vec<_>>(),
            )),
            standing_frames,
            seated_frames,
        }
    }
    pub(crate) fn physical_frames(&self, physical: &[Transform]) -> Option<[Transform; 16]> {
        let input: [Transform; 11] = physical.get(..11)?.try_into().ok()?;
        let source: [Transform; 11] = self.standing_frames[..11].try_into().unwrap();
        let legacy = human::SPECS.map(|s| Transform::from_translation(s.0));
        Some(articulation::extend(
            connected_pose(input, source, legacy),
            self.standing_frames,
            Some(physical),
        ))
    }
    pub fn ready(&self, server: &AssetServer) -> bool {
        self.textures.iter().all(|h| {
            if let Some(bevy::asset::LoadState::Failed(error)) = server.get_load_state(h.id()) {
                panic!("Required rider texture failed to load: {error}");
            }
            server.is_loaded_with_dependencies(h.id())
        })
    }
    pub fn human(
        &self,
        commands: &mut Commands,
        color: usize,
        joints: &[Entity],
        seated: bool,
    ) -> Vec<Entity> {
        assert_eq!(
            joints.len(),
            11,
            "visual skin uses the existing eleven body parts"
        );
        let source = if seated {
            self.seated_frames
        } else {
            self.standing_frames
        };
        let bind = source;
        let source: [Transform; 11] = source[..11].try_into().unwrap();
        let legacy = std::array::from_fn(|i| {
            if seated {
                human::seated(i)
            } else {
                Transform::from_translation(human::SPECS[i].0)
            }
        });
        let root = commands
            .spawn((
                Name::new("Connected visual skeleton"),
                Transform::IDENTITY,
                Visibility::default(),
            ))
            .id();
        let skin_joints: [Entity; 16] = std::array::from_fn(|i| {
            commands
                .spawn((Name::new(format!("Skin joint {i}")), SkinJoint(i), bind[i]))
                .id()
        });
        commands
            .entity(root)
            .add_children(&skin_joints)
            .insert(ConnectedRig {
                drivers: joints.try_into().unwrap(),
                joints: skin_joints,
                source,
                legacy,
                bind,
                seated,
            });
        for p in self
            .parts
            .iter()
            .filter(|p| p.group == if seated { "seated" } else { "standing" })
        {
            let mesh = commands
                .spawn((
                    Name::new("Authored rider skin"),
                    AuthoredHuman,
                    Mesh3d(p.mesh.clone()),
                    MeshMaterial3d(p.materials[color % p.materials.len()].clone()),
                    Transform::IDENTITY,
                    Visibility::default(),
                    NoFrustumCulling,
                    SkinnedMesh {
                        inverse_bindposes: if seated {
                            self.seated_bind.clone()
                        } else {
                            self.standing_bind.clone()
                        },
                        joints: skin_joints.to_vec(),
                    },
                ))
                .id();
            commands.entity(root).add_child(mesh);
        }
        vec![root]
    }
    pub fn motorcycle(&self, commands: &mut Commands, color: usize) -> Vec<Entity> {
        self.parts
            .iter()
            .filter(|p| !matches!(p.group.as_str(), "standing" | "seated"))
            .map(|p| {
                let mut e = commands.spawn((
                    Name::new("Authored motorcycle mesh"),
                    AuthoredMotorcycle,
                    Mesh3d(p.mesh.clone()),
                    MeshMaterial3d(p.materials[color % p.materials.len()].clone()),
                    p.rest,
                ));
                if matches!(p.group.as_str(), "front-wheel" | "rear-wheel") {
                    e.insert(AuthoredWheel {
                        rest: p.rest,
                        front: p.group == "front-wheel",
                    });
                }
                e.id()
            })
            .collect()
    }
}
fn swatch(hex: u32) -> Color {
    Color::srgb_u8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}
/// Presentation only. Wrecks have no BikeState and simply retain their wheel pose.
fn wheels(
    bikes: Query<&crate::vehicle::BikeState>,
    roots: Query<&crate::runtime::VisualRoot>,
    mut wheels: Query<(&AuthoredWheel, &ChildOf, &mut Transform)>,
) {
    for (wheel, parent, mut t) in &mut wheels {
        if let Ok(root) = roots.get(parent.parent())
            && let Ok(state) = bikes.get(root.owner)
        {
            *t = wheel.rest;
            t.translation.y -= state.suspension[usize::from(wheel.front)].clamp(-0.2, 0.2);
            t.rotation = Quat::from_rotation_y(if wheel.front {
                -state.raw_steer * 0.2
            } else {
                0.0
            }) * Quat::from_rotation_x(state.wheel_spin);
        }
    }
}
#[derive(Component)]
pub struct SkinJoint(pub usize);
#[derive(Component)]
struct ConnectedRig {
    drivers: [Entity; 11],
    joints: [Entity; 16],
    source: [Transform; 11],
    legacy: [Transform; 11],
    bind: [Transform; 16],
    seated: bool,
}
const RIG_ORDER: [usize; 11] = [2, 0, 1, 3, 4, 5, 6, 7, 8, 9, 10];
/// Source-anatomy forward kinematics: driver rotations, never disconnected driver translations.
fn connected_pose(
    input: [Transform; 11],
    source: [Transform; 11],
    legacy: [Transform; 11],
) -> [Transform; 11] {
    let desired: [Transform; 11] = std::array::from_fn(|i| {
        Transform::from_matrix(
            input[i].to_matrix() * legacy[i].to_matrix().inverse() * source[i].to_matrix(),
        )
    });
    let mut out = desired;
    for i in RIG_ORDER {
        let parent = rr_content::riding_model::RENDER_PARENTS[i];
        if parent < 0 {
            continue;
        }
        let p = parent as usize;
        let offset = source[p].rotation.inverse() * (source[i].translation - source[p].translation);
        out[i].translation = out[p].translation + out[p].rotation * offset;
        out[i].scale = Vec3::ONE;
        if matches!(i, 4 | 6 | 8 | 10) {
            let relative = input[p].rotation.inverse() * input[i].rotation;
            let rest = legacy[p].rotation.inverse() * legacy[i].rotation;
            let delta = relative * rest.inverse();
            let mut angle = 2.0 * delta.x.atan2(delta.w);
            angle = (angle + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU)
                - std::f32::consts::PI;
            let a = source[p].rotation * Vec3::Y;
            let b = source[i].rotation * Vec3::Y;
            let bend = a.angle_between(b);
            let axis = a.cross(b).try_normalize().unwrap_or(Vec3::X);
            let target = if i < 7 {
                (bend - angle).clamp(0.0, 2.35)
            } else {
                (bend + angle).clamp(0.0, 2.45)
            };
            let bind_relative = source[p].rotation.inverse() * source[i].rotation;
            let local_axis = source[p].rotation.inverse() * axis;
            out[i].rotation = (out[p].rotation
                * Quat::from_axis_angle(local_axis, target - bend)
                * bind_relative)
                .normalize();
        }
    }
    out
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn retarget(
    rigs: Query<(
        Entity,
        &ConnectedRig,
        Option<&crate::animation::Binding>,
        Option<&RagdollOwner>,
    )>,
    physical_rigs: Query<&crate::biomechanics::PhysicalRig>,
    physical: Query<
        (&Transform, Option<&crate::runtime::PoseHistory>),
        With<bevy_rapier3d::prelude::RigidBody>,
    >,
    library: Res<crate::animation::Library>,
    animators: Query<(&crate::animation::Animator, &crate::recovery::Recovery)>,
    owners: Query<&crate::recovery::OwnedBody>,
    fixed: Option<Res<Time<Fixed>>>,
    mut transforms: ParamSet<(
        TransformHelper,
        Query<&mut Transform, (With<SkinJoint>, Without<bevy_rapier3d::prelude::RigidBody>)>,
    )>,
) {
    let mut updates = Vec::new();
    {
        let helper = transforms.p0();
        for (root, rig, binding, ragdoll) in &rigs {
            let Ok(root) = helper.compute_global_transform(root) else {
                continue;
            };
            let frames: Result<Vec<_>, _> = rig
                .drivers
                .iter()
                .map(|e| {
                    helper
                        .compute_global_transform(*e)
                        .map(|t| t.compute_transform())
                })
                .collect();
            let Ok(frames) = frames else {
                continue;
            };
            let base = connected_pose(frames.try_into().unwrap(), rig.source, rig.legacy);
            let physical_frames =
                ragdoll
                    .and_then(|r| physical_rigs.get(r.0).ok())
                    .and_then(|rig| {
                        rig.0
                            .iter()
                            .map(|e| {
                                physical.get(*e).ok().map(|(t, h)| {
                                    crate::runtime::interpolated(
                                        t,
                                        h,
                                        fixed.as_ref().map_or(1.0, |t| t.overstep_fraction()),
                                    )
                                })
                            })
                            .collect::<Option<Vec<_>>>()
                    });
            let mut pose = articulation::extend(base, rig.bind, physical_frames.as_deref());
            let inverse = root.affine().inverse();
            let owner = binding.and_then(|b| match *b {
                crate::animation::Binding::Riding(e) => Some(e),
                crate::animation::Binding::Runner(e) => owners.get(e).ok().map(|o| o.owner),
            });
            if let Some((a, rec)) = owner
                .and_then(|e| animators.get(e).ok())
                .filter(|(a, _)| a.ready)
            {
                let alpha = fixed.as_ref().map_or(1.0, |t| t.overstep_fraction());
                let sampled =
                    library.skin_frames(a.previous.blend(a.current, alpha), rig.bind, rig.seated);
                if rec.stage == crate::recovery::RecoveryStage::Mounting {
                    if let Some(anchor) = rec
                        .wreck
                        .and_then(|e| helper.compute_global_transform(e).ok())
                    {
                        let anchor = crate::animation::bike_anchor(anchor.compute_transform());
                        pose = sampled.map(|p| anchor.mul_transform(p));
                    }
                } else if rec.stage != crate::recovery::RecoveryStage::Falling {
                    pose = sampled.map(|p| root.compute_transform().mul_transform(p));
                }
            }
            for (joint, pose) in rig.joints.iter().zip(pose) {
                let mut local = Transform::from_matrix(Mat4::from(inverse) * pose.to_matrix());
                local.scale = Vec3::ONE;
                updates.push((*joint, local));
            }
        }
    }
    let mut output = transforms.p1();
    for (joint, pose) in updates {
        if let Ok(mut t) = output.get_mut(joint) {
            *t = pose;
        }
    }
}

pub struct RoadModelPlugin;
impl Plugin for RoadModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::animation::MotionPlugin)
            .add_systems(Update, wheels.after(crate::vehicle::visuals))
            .add_systems(
                PostUpdate,
                retarget.before(bevy::transform::TransformSystems::Propagate),
            );
    }
}
/// Attach to the first visual joint: it has the same cleanup lifetime as the body.
/// GPU skinning uses all joint GlobalTransforms; this never writes body transforms.
#[derive(Component)]
struct RagdollOwner(Entity);
pub fn ragdoll(
    commands: &mut Commands,
    art: &world::Art,
    owner: Entity,
    color: usize,
    joints: &[Entity],
) {
    if let Some(models) = &art.models {
        let skins = models.human(commands, color, joints, false);
        for &skin in &skins {
            commands.entity(skin).insert(RagdollOwner(owner));
        }
        commands.entity(joints[0]).add_children(&skins);
    }
}
#[cfg(test)]
mod tests;
