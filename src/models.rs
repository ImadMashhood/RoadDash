//! Authored meshes driven by existing visual joints. No physics systems or components.
use crate::{human, world};
use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::NoFrustumCulling,
    mesh::{
        Indices, PrimitiveTopology, VertexAttributeValues,
        skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
    },
    prelude::*,
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
                human::SPECS
                    .iter()
                    .map(|s| Mat4::from_translation(-s.0))
                    .collect::<Vec<_>>(),
            )),
            seated_bind: binds.add(SkinnedMeshInverseBindposes::from(
                (0..11)
                    .map(|i| human::seated(i).to_matrix().inverse())
                    .collect::<Vec<_>>(),
            )),
        }
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
        self.parts
            .iter()
            .filter(|p| p.group == if seated { "seated" } else { "standing" })
            .map(|p| {
                commands
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
                            joints: joints.to_vec(),
                        },
                    ))
                    .id()
            })
            .collect()
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
pub struct RoadModelPlugin;
impl Plugin for RoadModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wheels.after(crate::vehicle::visuals));
    }
}
/// Attach to the first visual joint: it has the same cleanup lifetime as the body.
/// GPU skinning uses all joint GlobalTransforms; this never writes body transforms.
pub fn ragdoll(commands: &mut Commands, art: &world::Art, color: usize, joints: &[Entity]) {
    if let Some(models) = &art.models {
        let skins = models.human(commands, color, joints, false);
        commands.entity(joints[0]).add_children(&skins);
    }
}
#[cfg(test)]
mod tests;
