use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Suit {
    #[default]
    #[serde(alias = "classic")]
    Player,
    #[serde(alias = "ember")]
    Mike,
    Cydney,
    Jim,
    Pearl,
    Jon,
    Rhonda,
    Bose,
    #[serde(alias = "glacier")]
    Axle,
    Eric,
    Buzz,
    Druid,
    Hog,
    Serge,
    Slash,
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
    pub const ALL: [Self; 15] = [
        Self::Player,
        Self::Mike,
        Self::Cydney,
        Self::Jim,
        Self::Pearl,
        Self::Jon,
        Self::Rhonda,
        Self::Bose,
        Self::Axle,
        Self::Eric,
        Self::Buzz,
        Self::Druid,
        Self::Hog,
        Self::Serge,
        Self::Slash,
    ];
    pub fn name(self) -> &'static str {
        [
            "Player",
            "Teflon Mike",
            "Cydney Bass",
            "Slim Jim",
            "Pearl McKurdy",
            "Milwaukee Jon",
            "Rhonda Rash",
            "Bose Jefferson",
            "Axle",
            "Eric",
            "Buzz",
            "Druid",
            "Hog",
            "Serge",
            "Slash",
        ][self as usize]
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

#[derive(Default)]
struct HelmetGeometry {
    positions: Vec<Vec3>,
    indices: Vec<u32>,
    vertices: std::collections::HashMap<[u32; 3], u32>,
}

impl HelmetGeometry {
    fn quad(&mut self, points: [Vec3; 4]) {
        let [a, b, c, d] = points.map(|point| {
            *self
                .vertices
                .entry(point.to_array().map(f32::to_bits))
                .or_insert_with(|| {
                    let index = self.positions.len() as u32;
                    self.positions.push(point);
                    index
                })
        });
        self.indices.extend([a, b, c, a, c, d]);
    }

    fn mesh(self, head: Transform) -> Mesh {
        let count = self.positions.len();
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            self.positions
                .into_iter()
                .map(|point| head.transform_point(point).to_array())
                .collect::<Vec<_>>(),
        );
        mesh.insert_indices(Indices::U32(self.indices));
        mesh.compute_smooth_normals();
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_JOINT_INDEX,
            VertexAttributeValues::Uint16x4(vec![[1, 0, 0, 0]; count]),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, vec![[1., 0., 0., 0.]; count]);
        mesh
    }
}

impl HelmetModel {
    pub fn build(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        head: Transform,
        helmet: Helmet,
    ) -> Self {
        const STEPS: u32 = 80;
        const FRONT: u32 = STEPS / 2;
        let rally = helmet == Helmet::Rally;
        let eye = if rally { 16 } else { 18 };
        let rings = if rally {
            [
                (-0.021, 0.110, 0.120, -0.004),
                (0.018, 0.146, 0.177, 0.031),
                (0.062, 0.162, 0.213, 0.043),
                (0.119, 0.176, 0.210, 0.044),
                (0.235, 0.197, 0.168, 0.020),
                (0.269, 0.194, 0.173, 0.008),
                (0.310, 0.174, 0.155, -0.010),
                (0.342, 0.135, 0.123, -0.020),
                (0.365, 0.078, 0.070, -0.027),
                (0.374, 0.001, 0.001, -0.028),
            ]
        } else {
            [
                (-0.024, 0.112, 0.126, -0.016),
                (0.010, 0.153, 0.161, 0.009),
                (0.051, 0.176, 0.187, 0.024),
                (0.119, 0.189, 0.198, 0.023),
                (0.230, 0.190, 0.174, 0.015),
                (0.268, 0.183, 0.175, 0.001),
                (0.308, 0.161, 0.151, -0.014),
                (0.338, 0.126, 0.117, -0.020),
                (0.357, 0.068, 0.064, -0.023),
                (0.363, 0.001, 0.001, -0.024),
            ]
        };
        let point = |ring: usize, step: u32| {
            let (height, width, depth, shift) = rings[ring];
            let theta = (step as f32 / STEPS as f32 * 2. - 1.) * std::f32::consts::PI;
            let (sin, cos) = theta.sin_cos();
            let front = cos.max(0.);
            let lift = match ring {
                2 => -0.015 * front,
                3 => 0.017 * (1. - front),
                4 => 0.021 * front,
                _ => 0.,
            };
            Vec3::new(sin * width, height + lift, shift + cos * depth)
        };
        let mut shell_geometry = HelmetGeometry::default();
        let mut lens_geometry = HelmetGeometry::default();
        // Split the shell at the eyeport. The lens occupies the opening and
        // follows the curved brow and chin edges, with a narrow painted rim.
        for ring in 0..rings.len() - 1 {
            for step in 0..STEPS {
                let corners = [
                    point(ring, step),
                    point(ring, step + 1),
                    point(ring + 1, step + 1),
                    point(ring + 1, step),
                ];
                if ring != 3 || !(FRONT - eye..FRONT + eye).contains(&step) {
                    shell_geometry.quad(corners);
                } else {
                    let lower_left = corners[0].lerp(corners[3], 0.09);
                    let lower_right = corners[1].lerp(corners[2], 0.09);
                    let upper_left = corners[3].lerp(corners[0], 0.08);
                    let upper_right = corners[2].lerp(corners[1], 0.08);
                    let trim = Vec3::Z * 0.005;
                    lens_geometry
                        .quad([lower_left, lower_right, upper_right, upper_left].map(|p| p + trim));
                    shell_geometry.quad([
                        corners[0],
                        corners[1],
                        lower_right + trim,
                        lower_left + trim,
                    ]);
                    shell_geometry.quad([
                        upper_left + trim,
                        upper_right + trim,
                        corners[2],
                        corners[3],
                    ]);
                    if step == FRONT - eye {
                        shell_geometry.quad([
                            corners[0],
                            lower_left + trim,
                            upper_left + trim,
                            corners[3],
                        ]);
                    } else if step + 1 == FRONT + eye {
                        shell_geometry.quad([
                            lower_right + trim,
                            corners[1],
                            corners[2],
                            upper_right + trim,
                        ]);
                    }
                }
            }
        }
        if rally {
            // A swept visor peak, with a raised middle and tapered side mounts.
            for step in FRONT - 15..FRONT + 15 {
                let edge = |step| {
                    let mount = point(5, step);
                    let t = (step as f32 - FRONT as f32) / 15.;
                    let reach = (1. - t * t).max(0.);
                    let tip =
                        mount + Vec3::new(0.010 * t, 0.027 * reach - 0.007, 0.104 * reach + 0.008);
                    (mount, tip)
                };
                let (a, b) = edge(step);
                let (c, d) = edge(step + 1);
                shell_geometry.quad([a, b, d, c]);
                shell_geometry.quad([a, c, d - Vec3::Y * 0.008, b - Vec3::Y * 0.008]);
                lens_geometry.quad([b, b - Vec3::Y * 0.008, d - Vec3::Y * 0.008, d]);
            }
        }
        // Intake grilles follow the front contour instead of floating on a flat plane.
        for step in FRONT - 5..FRONT + 5 {
            let vent = |step, height| point(2, step).lerp(point(3, step), height) + Vec3::Z * 0.003;
            lens_geometry.quad([
                vent(step, 0.31),
                vent(step + 1, 0.31),
                vent(step + 1, 0.59),
                vent(step, 0.59),
            ]);
        }
        let shell = meshes.add(shell_geometry.mesh(head));
        let visor = meshes.add(lens_geometry.mesh(head));
        Self {
            shell,
            visor,
            shell_material: materials.add(StandardMaterial {
                base_color: swatch(if rally { 0xdce0e4 } else { 0x20242b }),
                metallic: 0.15,
                perceptual_roughness: if rally { 0.52 } else { 0.29 },
                ..default()
            }),
            visor_material: materials.add(StandardMaterial {
                base_color: swatch(if rally { 0x49352a } else { 0x203947 }),
                metallic: 0.84,
                perceptual_roughness: 0.21,
                ..default()
            }),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
                (Some(PartKind::Suit | PartKind::Shell), _) if gear.suit != Suit::Player => (
                    &original.mesh,
                    &original.materials[gear.suit as usize % original.materials.len()],
                ),
                _ => (&original.mesh, original_material),
            };
            mesh.set_if_neq(Mesh3d(new_mesh.clone()));
            material.set_if_neq(MeshMaterial3d(new_material.clone()));
        }
        rig.gear = gear;
    }
}
