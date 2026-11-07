//! One-shot grounded roll initiation, paid entirely from existing translational energy.
use super::*;
#[derive(Resource)]
pub struct Tuning(pub rr_content::crash_motion::Config);
impl Default for Tuning {
    fn default() -> Self {
        Self(
            rr_content::crash_motion::Config::from_json(include_str!(
                "../../content/crash_motion.json"
            ))
            .expect("validated crash motion"),
        )
    }
}
#[derive(Component)]
pub(crate) struct FrontalRelease;
#[allow(clippy::type_complexity)]
pub fn roll(
    settings: Res<Settings>,
    tuning: Res<Tuning>,
    mut brains: Query<(&OwnedBody, &mut Brain)>,
    riders: Query<&crate::game::Rider>,
    mut bodies: Query<(&OwnedBody, &Transform, &ReadMassProperties, &mut Velocity), With<Segment>>,
) {
    if !settings.enabled || tuning.0.roll_energy_fraction <= 0.0 {
        return;
    }
    for (owner, mut brain) in &mut brains {
        if brain.roll_assisted
            || !brain.supported
            || brain.control.age < tuning.0.roll_min_age
            || !riders.contains(owner.owner)
        {
            continue;
        }
        let mut mass = 0.0;
        let mut center = Vec3::ZERO;
        let mut momentum = Vec3::ZERO;
        for (o, t, m, v) in &bodies {
            if o.owner == owner.owner {
                mass += m.mass;
                center += t.transform_point(m.local_center_of_mass) * m.mass;
                momentum += v.linvel * m.mass;
            }
        }
        if mass < 1.0 {
            continue;
        }
        center /= mass;
        let com = momentum / mass;
        let horizontal = com.with_y(0.0);
        let speed = horizontal.length();
        if speed < tuning.0.roll_min_speed {
            continue;
        }
        brain.roll_assisted = true;
        let w = Vec3::Y.cross(horizontal / speed) * tuning.0.roll_rate;
        let mut quadratic = 0.0;
        let mut linear = 0.0;
        for (o, t, m, v) in &bodies {
            if o.owner != owner.owner {
                continue;
            }
            let dv = w.cross(t.transform_point(m.local_center_of_mass) - center);
            let frame = (t.rotation * m.principal_inertia_local_frame).inverse();
            let dw = frame * w;
            let old = frame * v.angvel;
            quadratic += 0.5
                * (m.mass * dv.length_squared() + (m.principal_inertia * dw * dw).element_sum());
            linear += m.mass * v.linvel.dot(dv) + (m.principal_inertia * old * dw).element_sum();
        }
        let budget = 0.5 * mass * speed * speed * tuning.0.roll_energy_fraction;
        let scale = if quadratic > 1e-6 {
            ((-linear + (linear * linear + 4.0 * quadratic * budget).sqrt()) / (2.0 * quadratic))
                .clamp(0.0, 1.0)
        } else {
            0.0
        };
        let cost = (quadratic * scale * scale + linear * scale).max(0.0);
        let new_speed = (speed * speed - 2.0 * cost / mass).max(0.0).sqrt();
        let brake = horizontal / speed * (new_speed - speed);
        for (o, t, m, mut v) in &mut bodies {
            if o.owner != owner.owner {
                continue;
            }
            v.linvel += w.cross(t.transform_point(m.local_center_of_mass) - center) * scale + brake;
            v.angvel += w * scale;
        }
    }
}
