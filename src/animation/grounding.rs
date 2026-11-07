//! Renderer-only stance locking and sole alignment against existing solid supports.
use super::*;
use bevy_rapier3d::prelude::ReadRapierContext;
pub(super) fn apply(
    lib: &Library,
    a: &mut Animator,
    rec: &Recovery,
    body: &Transform,
    context: &ReadRapierContext,
    dt: f32,
    stride: f32,
) {
    let Some(runner) = rec.runner else { return };
    let Ok(ctx) = context.single() else { return };
    let anchor = body.mul_transform(Transform::from_xyz(0.0, -0.95, 0.0));
    let inverse = anchor.to_matrix().inverse();
    let frames = lib.world_pose(a.current);
    let phase = (rec.walk_distance / stride).rem_euclid(1.0);
    for limb in &lib.data.limbs {
        if limb.bit < 2 {
            continue;
        }
        let i = (limb.bit - 2) as usize;
        let stance = rec.stage == RecoveryStage::GettingUp
            || rec.walk_velocity.with_y(0.0).length() < 0.2
            || (phase < 0.5) == (i == 0);
        let sole =
            anchor.transform_point(frames[limb.end].transform_point(Vec3::from_array(limb.offset)));
        if stance
            && (a.plants[i].is_none() || a.plants[i].is_some_and(|(p, _)| p.distance(sole) > 0.9))
        {
            use bevy_rapier3d::prelude::*;
            a.plants[i] = ctx
                .cast_ray_and_get_normal(
                    sole + Vec3::Y * 0.45,
                    -Vec3::Y,
                    1.0,
                    true,
                    QueryFilter::exclude_dynamic()
                        .exclude_sensors()
                        .exclude_collider(runner),
                )
                .filter(|(_, h)| h.normal.y > 0.4)
                .map(|(_, h)| (sole + Vec3::Y * (0.465 - h.time_of_impact), h.normal));
        }
        a.plant_weight[i] = motion::approach(
            a.plant_weight[i],
            if stance && a.plants[i].is_some() {
                1.0
            } else {
                0.0
            },
            dt,
            0.06,
        );
        if a.plant_weight[i] <= 0.0 {
            a.plants[i] = None;
        }
        if let Some((point, normal)) = a.plants[i] {
            a.current.planted[i] =
                Some(inverse.transform_point3(sole.lerp(point, a.plant_weight[i])));
            a.current.normals[i] = anchor.rotation.inverse() * normal;
        }
    }
}
