use super::*;
#[derive(Default)]
pub(super) struct Measure {
    speed: f32,
    angular: f32,
    supported: bool,
    resting_support: bool,
    severity: f32,
    activation: f32,
    age: f32,
    targets: [Quat; 16],
    frontal_release: bool,
}
/// Reuses scratch buckets and visits each segment/joint once per fixed tick.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(super) fn update(
    settings: Res<Settings>,
    reaction: Res<super::reactions::Tuning>,
    velocity_tuning: Res<super::velocity::Tuning>,
    time: Res<Time<Fixed>>,
    race: Option<Res<crate::game::Race>>,
    context: ReadRapierContext,
    controls: Option<Res<crate::controls::Controls>>,
    recovery_tuning: Option<Res<crate::recovery::RecoveryTuning>>,
    riders: Query<&crate::game::Rider>,
    mut forces: MessageReader<ContactForceEvent>,
    mut brains: Query<(&OwnedBody, &mut Brain, &super::velocity::Pilot)>,
    segments: Query<(Entity, &OwnedBody, &Velocity, &Segment)>,
    mut joints: Query<(&OwnedBody, &Segment, &mut ImpulseJoint, &mut Muscle)>,
    bodies: Query<&RigidBody>,
    mut scratch: Local<std::collections::HashMap<Entity, Measure>>,
) {
    if !settings.enabled || race.is_some_and(|r| !r.phase.simulating()) {
        return;
    }
    scratch.clear();
    let context = context.single().ok();
    for (e, o, v, part) in &segments {
        let stats = scratch.entry(o.owner).or_default();
        stats.speed = stats.speed.max(v.linear.length());
        stats.angular = stats.angular.max(v.angular.length());
        if let Some(context) = &context {
            for pair in context.contact_pairs_with(e) {
                let other = if pair.collider1() == Some(e) {
                    pair.collider2()
                } else {
                    pair.collider1()
                };
                if pair.has_any_active_contact()
                    && other.is_some_and(|e| bodies.get(e).is_ok_and(|b| *b == RigidBody::Fixed))
                {
                    let first = pair.collider1() == Some(e);
                    let upward = pair.manifolds().any(|m| {
                        let n = if first { -m.normal() } else { m.normal() };
                        m.num_solver_contacts() > 0 && n.y >= velocity_tuning.0.support_normal_y
                    });
                    stats.resting_support |= upward;
                    stats.supported |= upward && matches!(part.part, 0 | 2 | 11);
                }
            }
        }
    }
    for event in forces.read() {
        for e in [event.collider1, event.collider2] {
            if let Ok((_, o, _, _)) = segments.get(e) {
                let stats = scratch.entry(o.owner).or_default();
                stats.severity = stats.severity.max(
                    (event.total_force_magnitude * time.delta_secs() / 70.0 / 25.0).clamp(0.0, 1.0),
                );
            }
        }
    }
    for (owner, mut brain, pilot) in &mut brains {
        let Some(stats) = scratch.get_mut(&owner.owner) else {
            continue;
        };
        // Reuse the controller's previous fixed-tick verified support proxy.
        // It requires an actual fixed-body contact and a nearby core surface.
        stats.supported |=
            pilot.supported || (stats.resting_support && stats.speed < 0.05 && stats.angular < 0.1);
        brain.control.secondary_impact(stats.severity);
        brain.supported = stats.supported;
        brain.control.step(
            time.delta_secs(),
            stats.speed,
            stats.angular,
            stats.supported,
            &settings.config.activation,
        );
        let bracing = controls
            .as_ref()
            .is_some_and(|c| c.foot.length_squared() > 0.01)
            && riders.get(owner.owner).is_ok_and(|r| r.id == 0)
            && stats.supported
            && brain.control.age >= 0.8;
        stats.activation = if bracing {
            brain.control.activation.max(
                recovery_tuning
                    .as_ref()
                    .map_or(0.12, |t| t.0.brace_activation),
            )
        } else {
            brain.control.activation
        };
        stats.age = brain.control.age;
        stats.targets = brain.targets;
        stats.frontal_release = brain.frontal_release;
    }
    for (owner, s, mut joint, mut muscle) in &mut joints {
        let Some(stats) = scratch.get(&owner.owner) else {
            continue;
        };
        let c = &settings.config.joints[s.part];
        let mut target = stats.targets[s.part].slerp(
            rotation(c.brace),
            (stats.age / settings.config.activation.impact_seconds).clamp(0.0, 1.0),
        );
        let mut activation = stats.activation;
        if stats.frontal_release
            && reaction.0.release_activation > 0.0
            && stats.age < reaction.0.release_seconds
            && matches!(s.part, 7..=10)
        {
            let weight = (std::f32::consts::PI * stats.age / reaction.0.release_seconds)
                .sin()
                .max(0.0);
            let angle = if matches!(s.part, 7 | 9) {
                reaction.0.hip_back
            } else {
                reaction.0.knee_back
            };
            target = target.slerp(rotation([angle, 0.0, 0.0]), weight);
            activation = activation.max(reaction.0.release_activation * weight);
        }
        if (muscle.activation - activation).abs() > 0.00001
            || muscle.target.dot(target).abs() < 0.99999
        {
            // Method B retains passive native constraints; its active drive is
            // the reciprocal angular-impulse controller after this system.
            set_drive(joint.data.as_mut(), c, 0.0, target);
            muscle.activation = activation;
            muscle.target = target;
        }
    }
}
