//! Visual-only distal articulation. Source mesh vertices and physical drivers stay independent.
use super::*;
// Original motion.json shin-to-foot endpoint length. Keep this visual pivot at
// the authored ankle: a shorter approximation makes distal flex orbit the shin.
// The source mesh, weights and physical rig pivots are deliberately independent.
const ANKLE_DISTANCE: f32 = 0.558_046_2;
pub const PARENTS: [usize; 5] = [0, 4, 6, 8, 10];
pub fn bind_frames(base: [Transform; 11]) -> [Transform; 16] {
    let mut out = [Transform::IDENTITY; 16];
    out[..11].copy_from_slice(&base);
    out[11] = base[0];
    out[11].translation = base[2].translation.lerp(base[0].translation, 0.6);
    for (i, parent, distance) in [
        (12, 4, 0.27),
        (13, 6, 0.27),
        (14, 8, ANKLE_DISTANCE),
        (15, 10, ANKLE_DISTANCE),
    ] {
        out[i] = base[parent];
        out[i].translation = base[parent].transform_point(Vec3::Y * distance);
    }
    out
}
pub fn weights(data: &mut rr_content::riding_model::MeshData, bind: &[Transform; 16]) {
    for i in 0..data.positions.len() {
        let vertex = Vec3::from_array(data.positions[i]);
        let mut influences: Vec<(u16, f32)> = Vec::new();
        for (joint, weight) in data.joints[i].into_iter().zip(data.weights[i]) {
            if weight <= 0.0 {
                continue;
            }
            let along = bind[joint as usize].rotation.inverse()
                * (vertex - bind[joint as usize].translation);
            let (extra, t) = match joint {
                0 => (11, ((-along.y - 0.06) / 0.16).clamp(0.0, 1.0)),
                4 => (12, ((along.y - 0.22) / 0.07).clamp(0.0, 1.0)),
                6 => (13, ((along.y - 0.22) / 0.07).clamp(0.0, 1.0)),
                8 => (14, ((along.y - 0.44) / 0.10).clamp(0.0, 1.0)),
                10 => (15, ((along.y - 0.44) / 0.10).clamp(0.0, 1.0)),
                _ => (joint, 0.0),
            };
            for (j, w) in [(joint, weight * (1.0 - t)), (extra, weight * t)] {
                if let Some(v) = influences.iter_mut().find(|v| v.0 == j) {
                    v.1 += w;
                } else {
                    influences.push((j, w));
                }
            }
        }
        influences.sort_by(|a, b| b.1.total_cmp(&a.1));
        influences.truncate(4);
        let sum = influences.iter().map(|v| v.1).sum::<f32>();
        data.joints[i] = [0; 4];
        data.weights[i] = [0.0; 4];
        for (n, (j, w)) in influences.into_iter().enumerate() {
            data.joints[i][n] = j;
            data.weights[i][n] = w / sum;
        }
    }
}
pub fn extend(
    base: [Transform; 11],
    bind: [Transform; 16],
    physical: Option<&[Transform]>,
) -> [Transform; 16] {
    let mut out = base.map_extend();
    for (n, parent) in PARENTS.into_iter().enumerate() {
        let i = n + 11;
        let relative = bind[parent].to_matrix().inverse() * bind[i].to_matrix();
        out[i] = base[parent].mul_transform(Transform::from_matrix(relative));
        if let Some(p) = physical.filter(|p| p.len() == 16) {
            let delta = p[parent].rotation.inverse() * p[i].rotation;
            out[i].rotation = (base[parent].rotation
                * delta
                * bind[parent].rotation.inverse()
                * bind[i].rotation)
                .normalize();
        }
    }
    out
}
trait ExtendFrames {
    fn map_extend(self) -> [Transform; 16];
}
impl ExtendFrames for [Transform; 11] {
    fn map_extend(self) -> [Transform; 16] {
        let mut v = [Transform::IDENTITY; 16];
        v[..11].copy_from_slice(&self);
        v
    }
}
