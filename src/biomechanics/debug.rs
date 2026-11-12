use super::*;
/// F7 toggles exact proxy outlines, local frames, limit rays and target axes.
/// Textual per-joint readout is emitted once per second, not every frame.
#[allow(clippy::type_complexity)]
pub(super) fn draw(
    mut settings: ResMut<Settings>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut last: Local<f32>,
    mut g: Gizmos,
    bodies: Query<(
        Entity,
        &Transform,
        &Velocity,
        &Segment,
        Option<&ImpulseJoint>,
        Option<&Muscle>,
        Option<&Brain>,
    )>,
    transforms: Query<(&Transform, &Velocity)>,
) {
    if keys.just_pressed(KeyCode::F7) {
        settings.debug = !settings.debug;
        info!(
            "Ragdoll debug {} (F7); cyan actual, yellow target, magenta anchor error",
            settings.debug
        );
    }
    if !settings.debug {
        return;
    }
    let specs = specs();
    let report = time.elapsed_secs() - *last > 1.0;
    if report {
        *last = time.elapsed_secs();
    }
    for (e, t, v, s, j, m, b) in &bodies {
        let spec = specs[s.part];
        let color = Color::srgba(0.1, 0.8, 1.0, 0.8);
        if spec.parent.is_none() || spec.size.y < spec.size.z {
            g.primitive_3d(
                &Cuboid::from_size(spec.size),
                Isometry3d::new(t.translation, t.rotation),
                color,
            );
        } else {
            let r = spec.size.x.min(spec.size.z) * 0.45;
            g.primitive_3d(
                &Capsule3d {
                    radius: r,
                    half_length: (spec.size.y * 0.5 - r).max(0.005),
                },
                Isometry3d::new(t.translation, t.rotation),
                color,
            );
        }
        if let Some(b) = b {
            g.sphere(
                Isometry3d::from_translation(t.translation),
                0.06,
                Color::srgb(1.0 - b.control.activation, b.control.activation, 0.0),
            );
            if report {
                info!(
                    "RAGDOLL {e:?} {:?} activation={:.3} supported={}",
                    b.control.state, b.control.activation, b.supported
                );
            }
        }
        let (Some(j), Some(m)) = (j, m) else {
            continue;
        };
        let Ok((p, parent_velocity)) = transforms.get(j.parent) else {
            continue;
        };
        let raw: &GenericJoint = j.data.as_ref();
        let a = p.transform_point(raw.local_anchor1());
        let b = t.transform_point(raw.local_anchor2());
        g.line(a, b, Color::srgb(1.0, 0.0, 1.0));
        for axis in [Vec3::X, Vec3::Y, Vec3::Z] {
            g.line(a, a + t.rotation * axis * 0.13, Color::srgb(0.0, 0.8, 1.0));
            g.line(
                a,
                a + p.rotation * m.target * axis * 0.18,
                Color::srgb(1.0, 0.8, 0.0),
            );
        }
        for i in 0..3 {
            let axis = [Vec3::X, Vec3::Y, Vec3::Z][i];
            for limit in settings.config.joints[s.part].limits[i] {
                let ray = Quat::from_axis_angle(axis, limit) * [Vec3::Y, Vec3::Z, Vec3::X][i];
                g.line(
                    a,
                    a + p.rotation * ray * 0.24,
                    Color::srgba(0.5, 0.8, 0.4, 0.5),
                );
            }
        }
        if report {
            info!(
                "{} activation={:.3} actual={:?} target={:?} relative_omega={:?} anchor_error_m={:.5}",
                settings.config.joints[s.part].name,
                m.activation,
                coordinates(p.rotation.inverse() * t.rotation),
                coordinates(m.target),
                p.rotation.inverse() * (v.angular - parent_velocity.angular),
                a.distance(b)
            );
        }
    }
}
