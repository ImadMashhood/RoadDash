use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Suit {
    #[default]
    Classic,
    Ember,
    Glacier,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Helmet {
    #[default]
    Classic,
    Apex,
    Rally,
}

impl Suit {
    pub const ALL: [Self; 3] = [Self::Classic, Self::Ember, Self::Glacier];
    pub fn name(self) -> &'static str {
        ["Classic", "Ember", "Glacier"][self as usize]
    }
}

impl Helmet {
    pub const ALL: [Self; 3] = [Self::Classic, Self::Apex, Self::Rally];
    pub fn name(self) -> &'static str {
        ["Classic", "Apex", "Rally"][self as usize]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Gear {
    pub suit: Suit,
    pub helmet: Helmet,
}

pub(super) struct HelmetModel {
    pub shell: Handle<Mesh>,
    pub visor: Handle<Mesh>,
    pub shell_material: Handle<StandardMaterial>,
    pub visor_material: Handle<StandardMaterial>,
}

impl HelmetModel {
    pub fn build(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        head: Transform,
        helmet: Helmet,
    ) -> Self {
        let rally = helmet == Helmet::Rally;
        let rings = if rally {
            [
                (0.005, 0.108, 0.133, 0.054),
                (0.057, 0.161, 0.176, 0.066),
                (0.161, 0.195, 0.190, 0.026),
                (0.228, 0.200, 0.200, 0.031),
                (0.292, 0.168, 0.173, 0.001),
                (0.345, 0.111, 0.117, -0.008),
                (0.368, 0.002, 0.002, -0.012),
            ]
        } else {
            [
                (-0.009, 0.109, 0.132, 0.012),
                (0.042, 0.157, 0.171, 0.025),
                (0.140, 0.194, 0.193, 0.020),
                (0.220, 0.197, 0.192, 0.003),
                (0.286, 0.151, 0.153, -0.015),
                (0.331, 0.095, 0.103, -0.021),
                (0.355, 0.002, 0.002, -0.027),
            ]
        };
        let point = |ring: usize, theta: f32, offset: f32| {
            let (height, width, depth, shift) = rings[ring];
            let (sin, cos) = theta.sin_cos();
            let profile = if rally { 0.80 } else { 0.93 };
            Vec3::new(
                sin.signum() * sin.abs().powf(profile) * (width + offset),
                height,
                shift + cos.signum() * cos.abs().powf(profile) * (depth + offset),
            )
        };
        let surface = |rows: std::ops::RangeInclusive<usize>, steps: u32, arc: f32, offset| {
            let mut positions = Vec::new();
            let mut indices = Vec::new();
            for (row, ring) in rows.enumerate() {
                for step in 0..=steps {
                    let theta = arc * (2. * step as f32 / steps as f32 - 1.);
                    positions.push(head.transform_point(point(ring, theta, offset)).to_array());
                    if row > 0 && step > 0 {
                        let d = row as u32 * (steps + 1) + step;
                        let a = d - steps - 2;
                        indices.extend([a, a + 1, d, a, d, d - 1]);
                    }
                }
            }
            let count = positions.len();
            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_indices(Indices::U32(indices));
            mesh.compute_smooth_normals();
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_JOINT_INDEX,
                VertexAttributeValues::Uint16x4(vec![[1, 0, 0, 0]; count]),
            );
            mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, vec![[1., 0., 0., 0.]; count]);
            mesh
        };
        let shell = meshes.add(surface(0..=6, 40, std::f32::consts::PI, 0.));
        let visor = meshes.add(surface(2..=3, 20, if rally { 0.98 } else { 1.35 }, 0.010));
        Self {
            shell,
            visor,
            shell_material: materials.add(StandardMaterial {
                base_color: swatch(if rally { 0xe7cfaa } else { 0x22363f }),
                metallic: 0.30,
                perceptual_roughness: if rally { 0.52 } else { 0.29 },
                ..default()
            }),
            visor_material: materials.add(StandardMaterial {
                base_color: swatch(if rally { 0x402722 } else { 0x11384d }),
                metallic: 0.84,
                perceptual_roughness: 0.21,
                ..default()
            }),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum PartKind {
    Suit,
    Shell,
    Visor,
}

#[allow(clippy::type_complexity)]
pub(super) fn apply(
    art: Option<Res<world::Art>>,
    preferences: Option<Res<crate::settings::Preferences>>,
    network: Option<Res<crate::online::NetState>>,
    mut rigs: Query<(
        &mut ConnectedRig,
        Option<&crate::animation::Binding>,
        Option<&RagdollOwner>,
    )>,
    owners: Query<&crate::recovery::OwnedBody>,
    riders: Query<&crate::game::Rider>,
    mut parts: Query<(&mut Mesh3d, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    use crate::animation::Binding;

    let Some(models) = art.as_ref().and_then(|art| art.models.as_ref()) else {
        return;
    };
    let Some(preferences) = preferences else {
        return;
    };
    let local_slot = network.as_ref().map_or(0, |state| state.local_slot);
    for (mut rig, binding, ragdoll) in &mut rigs {
        if rig.gear_parts.is_empty()
            || (preferences.file.gear == Gear::default() && rig.gear == Gear::default())
        {
            continue;
        }
        let owner = match binding {
            Some(Binding::Riding(rider)) => Some(*rider),
            Some(Binding::Runner(runner)) => owners.get(*runner).ok().map(|body| body.owner),
            None => ragdoll.map(|body| body.0),
        };
        let gear = if owner
            .and_then(|entity| riders.get(entity).ok())
            .is_some_and(|rider| rider.id == local_slot && !rider.police)
        {
            preferences.file.gear
        } else {
            Gear::default()
        };
        if gear == rig.gear {
            continue;
        }
        for &(entity, index, palette) in &rig.gear_parts {
            let Ok((mut mesh, mut material)) = parts.get_mut(entity) else {
                continue;
            };
            let original = &models.parts[index];
            let helmet = (gear.helmet != Helmet::Classic).then(|| {
                &models.helmets[usize::from(original.group == "seated")][gear.helmet as usize - 1]
            });
            let original_material = &original.materials[palette];
            let (new_mesh, new_material) = match (&original.gear, helmet) {
                (Some(PartKind::Shell), Some(helmet)) => (&helmet.shell, &helmet.shell_material),
                (Some(PartKind::Visor), Some(helmet)) => (&helmet.visor, &helmet.visor_material),
                (Some(PartKind::Suit), _) if gear.suit != Suit::Classic => (
                    &original.mesh,
                    original
                        .suit_materials
                        .as_ref()
                        .map_or(original_material, |materials| {
                            &materials[gear.suit as usize - 1]
                        }),
                ),
                _ => (&original.mesh, original_material),
            };
            mesh.set_if_neq(Mesh3d(new_mesh.clone()));
            material.set_if_neq(MeshMaterial3d(new_material.clone()));
        }
        rig.gear = gear;
    }
}
