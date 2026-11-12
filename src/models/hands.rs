//! Shared GPU glove morphs and visual sockets. No physics components or mesh writes per frame.
use super::*;
use bevy::mesh::morph::{MeshMorphWeights, MorphAttributes};
use rr_content::hand_contacts::HandContacts;

#[derive(Component, Clone, Copy, Debug)]
pub struct HandSocket {
    /// 0 = left, 1 = right. Weapon +Y follows the configured handle axis.
    pub side: usize,
    pub weapon: bool,
    pub seated: bool,
}
#[derive(Component)]
pub(super) struct GloveShape(Entity);

pub(super) fn config() -> HandContacts {
    HandContacts::from_json(include_str!("../../content/hand-contacts.json"))
        .expect("validated visual glove contacts")
}
fn frame(bind: &[Transform; 16], seated: bool, side: usize, tuning: &HandContacts) -> Transform {
    let mut frame = bind[if side == 0 { 13 } else { 12 }];
    if seated {
        frame.rotation *= Quat::from_array(tuning.seated_basis[side]);
    }
    frame
}
/// Piecewise phalanx bending keeps knuckles fixed and leaves the wrist untouched.
fn fold(p: Vec3, angles: [f32; 3], hinges: [f32; 3]) -> (Vec3, Quat) {
    if p.y < 0.015 {
        return (p, Quat::IDENTITY);
    }
    let mut point = p;
    let mut rotation = Quat::IDENTITY;
    let thumb =
        ((p.x.abs() - 0.048) / 0.015).clamp(0.0, 1.0) * ((0.10 - p.y) / 0.025).clamp(0.0, 1.0);
    for (hinge, angle) in hinges.into_iter().zip(angles).rev() {
        let influence = ((p.y - hinge + 0.004) / 0.008).clamp(0.0, 1.0) * (1.0 - thumb);
        let q = Quat::from_rotation_x(angle * influence);
        let anchor = Vec3::Y * hinge;
        point = anchor + q * (point - anchor);
        rotation = q * rotation;
    }
    let q = Quat::from_rotation_z(p.x.signum() * angles[0] * thumb)
        * Quat::from_rotation_y(-p.x.signum() * angles[0] * 0.8 * thumb);
    let anchor = Vec3::new(p.x.signum() * 0.035, 0.025, 0.0);
    (anchor + q * (point - anchor), q * rotation)
}

pub(super) fn targets(
    mesh: &Mesh,
    bind: &[Transform; 16],
    seated: bool,
    tuning: &HandContacts,
) -> Option<Vec<MorphAttributes>> {
    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?.as_float3()?;
    let normals = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)?.as_float3()?;
    let VertexAttributeValues::Uint16x4(joints) = mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX)?
    else {
        return None;
    };
    let VertexAttributeValues::Float32x4(weights) = mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT)?
    else {
        return None;
    };
    let VertexAttributeValues::Float32x4(tangents) = mesh.attribute(Mesh::ATTRIBUTE_TANGENT)?
    else {
        return None;
    };
    if !joints.iter().zip(weights).any(|(j, w)| {
        j.iter()
            .zip(w)
            .any(|(j, w)| matches!(j, 12 | 13) && *w > 0.0)
    }) {
        return None;
    }
    let targets: Vec<Vec<MorphAttributes>> = (0..2)
        .flat_map(|side| (0..2).map(move |shape| (side, shape)))
        .map(|(side, shape)| {
            let f = frame(bind, seated, side, tuning);
            let joint = if side == 0 { 13 } else { 12 };
            let angles = if shape == 0 {
                tuning.grip_angles_rad
            } else {
                tuning.fist_angles_rad
            };
            positions
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let influence: f32 = joints[i]
                        .iter()
                        .zip(weights[i])
                        .filter(|(j, _)| **j == joint)
                        .map(|(_, w)| w)
                        .sum();
                    if influence < 0.001 {
                        return MorphAttributes::default();
                    }
                    let p = Vec3::from_array(*p);
                    let local = f.rotation.inverse() * (p - f.translation);
                    if local.y < 0.015 {
                        return MorphAttributes::default();
                    }
                    let (folded, q) = fold(local, angles, tuning.hinges_m);
                    let world_q = f.rotation * q * f.rotation.inverse();
                    let n = Vec3::from_array(normals[i]);
                    let t = Vec3::from_array(tangents[i][..3].try_into().unwrap());
                    MorphAttributes {
                        position: (f.transform_point(folded) - p) * influence,
                        normal: (world_q * n - n) * influence,
                        tangent: (world_q * t - t) * influence,
                        ..default()
                    }
                })
                .collect()
        })
        .collect();
    Some(targets.into_iter().flatten().collect())
}
pub(super) fn attach(commands: &mut Commands, mesh: Entity, root: Entity) {
    commands.entity(mesh).insert((
        GloveShape(root),
        MeshMorphWeights::Value {
            weights: vec![0.0; 4],
        },
    ));
}
pub(super) fn sockets(
    commands: &mut Commands,
    joints: &[Entity; 16],
    seated: bool,
    tuning: &HandContacts,
) {
    for side in 0..2 {
        let basis = if seated {
            Quat::from_array(tuning.seated_basis[side])
        } else {
            Quat::IDENTITY
        };
        for weapon in [false, true] {
            let rotation = basis
                * if weapon {
                    Quat::from_array(tuning.weapon_rotation_xyzw[side])
                } else {
                    Quat::IDENTITY
                };
            let socket = commands
                .spawn((
                    Name::new(format!(
                        "{} socket {}",
                        if weapon { "Weapon" } else { "Grip" },
                        if side == 0 { "L" } else { "R" }
                    )),
                    HandSocket {
                        side,
                        weapon,
                        seated,
                    },
                    Transform::from_translation(
                        basis * Vec3::from_array(tuning.grip_sockets_m[side]),
                    )
                    .with_rotation(rotation),
                ))
                .id();
            commands
                .entity(joints[if side == 0 { 13 } else { 12 }])
                .add_child(socket);
        }
    }
}
pub(super) fn animate(
    rigs: Query<(&ConnectedRig, Option<&crate::animation::Binding>)>,
    animators: Query<&crate::animation::Animator>,
    owners: Query<&crate::recovery::OwnedBody>,
    fixed: Option<Res<Time<Fixed>>>,
    mut meshes: Query<(&GloveShape, &mut MeshMorphWeights)>,
) {
    for (shape, mut weights) in &mut meshes {
        let value = rigs
            .get(shape.0)
            .ok()
            .and_then(|(_, binding)| binding)
            .and_then(|binding| match binding {
                crate::animation::Binding::Riding(e) => Some(*e),
                crate::animation::Binding::Runner(e) => owners.get(*e).ok().map(|o| o.owner),
            })
            .and_then(|owner| animators.get(owner).ok())
            .filter(|a| a.ready)
            .map(|a| {
                a.previous
                    .blend(
                        a.current,
                        fixed.as_ref().map_or(1.0, |t| t.overstep_fraction()),
                    )
                    .hands
            })
            .unwrap_or([[0.0; 2]; 2]);
        if let MeshMorphWeights::Value { weights } = &mut *weights {
            weights.copy_from_slice(&[value[0][0], value[0][1], value[1][0], value[1][1]]);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Authoring runs in Blender/Python; compare its exported numeric results
    /// against the GPU target recipe, including thumb signs and hinge boundaries.
    #[test]
    fn blender_authoring_fold_matches_runtime_positions_and_normal_rotations() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../docs/testing/part02-authoring/fold-fixtures.json"
        ))
        .unwrap();
        let tuning = config();
        let vector = |v: &serde_json::Value| {
            Vec3::new(
                v[0].as_f64().unwrap() as f32,
                v[1].as_f64().unwrap() as f32,
                v[2].as_f64().unwrap() as f32,
            )
        };
        for sample in fixture["samples"].as_array().unwrap() {
            let angles = if sample["shape"] == "grip" {
                tuning.grip_angles_rad
            } else {
                tuning.fist_angles_rad
            };
            let (point, rotation) = fold(vector(&sample["point"]), angles, tuning.hinges_m);
            assert!(
                point.distance(vector(&sample["folded"])) < 3e-7,
                "position {sample}: {point:?}"
            );
            assert!(
                (rotation * Vec3::Z).distance(vector(&sample["normal"])) < 1e-6,
                "normal {sample}: {:?}",
                rotation * Vec3::Z
            );
        }
    }

    #[test]
    fn knuckles_and_wrist_remain_fixed_and_finger_sections_keep_length() {
        let h = config();
        for y in [-0.1, 0.0, 0.014] {
            let p = Vec3::new(0., y, 0.);
            assert_eq!(fold(p, h.fist_angles_rad, h.hinges_m).0, p);
        }
        let start = Vec3::new(0., 0.12, 0.);
        let end = Vec3::new(0., 0.14, 0.);
        let a = fold(start, h.fist_angles_rad, h.hinges_m).0;
        let b = fold(end, h.fist_angles_rad, h.hinges_m).0;
        assert!((a.distance(b) - start.distance(end)).abs() < 1e-6);
        assert!(b.y < end.y && b.z > 0.02);
    }
}

#[cfg(test)]
mod contact_tests {
    use super::*;
    #[test]
    fn seated_glove_sockets_agree_with_the_universal_grip_contacts() {
        let h = config();
        let bindings = rr_content::riding_model::Bindings::from_json(include_str!(
            "../../assets/models/rider-bindings.json"
        ))
        .unwrap();
        let bind = articulation::bind_frames(
            bindings
                .seated
                .map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m))),
        );
        let lib = crate::animation::Library::default();
        let pose = lib.skin_frames(lib.key(2020), bind, true);
        let motion: serde_json::Value =
            serde_json::from_str(include_str!("../../assets/models/motion.json")).unwrap();
        let mut error = 0.0f32;
        for side in 0..2 {
            let f = frame(&pose, true, side, &h);
            let socket = f.transform_point(Vec3::from_array(h.grip_sockets_m[side]));
            let limb = motion["limbs"]
                .as_array()
                .unwrap()
                .iter()
                .find(|l| l["bit"] == side)
                .unwrap();
            let target = Vec3::new(
                limb["target"][0].as_f64().unwrap() as f32,
                limb["target"][1].as_f64().unwrap() as f32,
                limb["target"][2].as_f64().unwrap() as f32,
            );

            error = error.max(socket.distance(target));
        }
        assert!(error < 0.006, "grip socket error {error}");
    }
}

/// Preserve the original triangles/skin/UVs, but restrict morph shader work to
/// the glove surfaces. The body never pays four-target texture fetches.
pub(super) fn split(
    data: rr_content::riding_model::MeshData,
) -> Vec<rr_content::riding_model::MeshData> {
    use rr_content::riding_model::MeshData;
    let mut pieces: Vec<MeshData> = (0..2)
        .map(|i| MeshData {
            name: format!("{}-{}", data.name, if i == 0 { "body" } else { "gloves" }),
            group: data.group.clone(),
            material: data.material.clone(),
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            weights: Vec::new(),
            joints: Vec::new(),
            indices: Vec::new(),
        })
        .collect();
    let mut maps = [
        std::collections::HashMap::new(),
        std::collections::HashMap::new(),
    ];
    for triangle in data.indices.chunks_exact(3) {
        let hand = usize::from(triangle.iter().any(|i| {
            data.joints[*i as usize]
                .iter()
                .zip(data.weights[*i as usize])
                .any(|(j, w)| matches!(j, 12 | 13) && w > 0.2)
        }));
        let piece = &mut pieces[hand];
        for old in triangle {
            let index = *maps[hand].entry(*old).or_insert_with(|| {
                let i = *old as usize;
                let next = piece.positions.len() as u32;
                piece.positions.push(data.positions[i]);
                piece.normals.push(data.normals[i]);
                piece.uvs.push(data.uvs[i]);
                piece.weights.push(data.weights[i]);
                piece.joints.push(data.joints[i]);
                next
            });
            piece.indices.push(index);
        }
    }
    pieces
        .into_iter()
        .filter(|p| !p.indices.is_empty())
        .collect()
}

/// Add phalanx edge samples only where the original rigid digits had long
/// straight faces. Shared edge midpoints keep the glove surface watertight.
pub(super) fn refine(data: &mut rr_content::riding_model::MeshData) {
    let mut edges = std::collections::HashMap::new();
    let hand_weight = |j: [u16; 4], w: [f32; 4]| {
        j.into_iter()
            .zip(w)
            .filter(|(j, _)| matches!(j, 12 | 13))
            .map(|(_, w)| w)
            .sum::<f32>()
    };
    for _ in 0..3 {
        let mut indices = Vec::with_capacity(data.indices.len() * 2);
        for triangle in data.indices.chunks_exact(3) {
            let (a, b, c) = [(0, 1, 2), (1, 2, 0), (2, 0, 1)]
                .into_iter()
                .max_by(|(a, b, _), (c, d, _)| {
                    let length = |i, j| {
                        Vec3::from_array(data.positions[triangle[i] as usize]).distance_squared(
                            Vec3::from_array(data.positions[triangle[j] as usize]),
                        )
                    };
                    length(*a, *b).total_cmp(&length(*c, *d))
                })
                .unwrap();
            let ia = triangle[a] as usize;
            let ib = triangle[b] as usize;
            if data.joints[ia] != data.joints[ib]
                || hand_weight(data.joints[ia], data.weights[ia]) < 0.95
                || hand_weight(data.joints[ib], data.weights[ib]) < 0.95
                || Vec3::from_array(data.positions[ia])
                    .distance(Vec3::from_array(data.positions[ib]))
                    <= 0.035
            {
                indices.extend_from_slice(triangle);
                continue;
            }
            let key = if ia < ib { (ia, ib) } else { (ib, ia) };
            let middle = *edges.entry(key).or_insert_with(|| {
                let i = data.positions.len() as u32;
                data.positions.push(
                    ((Vec3::from_array(data.positions[ia]) + Vec3::from_array(data.positions[ib]))
                        * 0.5)
                        .to_array(),
                );
                data.normals.push(
                    (Vec3::from_array(data.normals[ia]) + Vec3::from_array(data.normals[ib]))
                        .normalize_or_zero()
                        .to_array(),
                );
                data.uvs.push(std::array::from_fn(|n| {
                    (data.uvs[ia][n] + data.uvs[ib][n]) * 0.5
                }));
                data.weights.push(std::array::from_fn(|n| {
                    (data.weights[ia][n] + data.weights[ib][n]) * 0.5
                }));
                data.joints.push(data.joints[ia]);
                i
            });
            indices.extend_from_slice(&[
                triangle[a],
                middle,
                triangle[c],
                middle,
                triangle[b],
                triangle[c],
            ]);
        }
        data.indices = indices;
    }
}
