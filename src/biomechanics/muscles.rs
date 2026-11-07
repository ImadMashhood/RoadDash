use super::*;
#[derive(Default)]
pub(super) struct Measure {
    speed: f32,
    angular: f32,
    supported: bool,
    severity: f32,
    activation: f32,
    age: f32,
    targets: [Quat; 16],
}
/// Reuses scratch buckets and visits each segment/joint once per fixed tick.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(super) fn update(
    settings: Res<Settings>,
    time: Res<Time<Fixed>>,
    race: Option<Res<crate::game::Race>>,
    context: ReadRapierContext,
    mut forces: MessageReader<ContactForceEvent>,
    mut brains: Query<(&OwnedBody, &mut Brain)>,
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
    for (e, o, v, _) in &segments {
        let stats = scratch.entry(o.owner).or_default();
        stats.speed = stats.speed.max(v.linvel.length());
        stats.angular = stats.angular.max(v.angvel.length());
        if let Some(context) = &context {
            for pair in context.contact_pairs_with(e) {
                let other = if pair.collider1() == Some(e) {
                    pair.collider2()
                } else {
                    pair.collider1()
                };
                if pair.has_any_active_contact()
                    && other
                        .is_some_and(|e| bodies.get(e).map_or(true, |b| *b != RigidBody::Dynamic))
                {
                    stats.supported = true;
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
    for (owner, mut brain) in &mut brains {
        let Some(stats) = scratch.get_mut(&owner.owner) else {
            continue;
        };
        brain.control.secondary_impact(stats.severity);
        brain.supported = stats.supported;
        brain.control.step(
            time.delta_secs(),
            stats.speed,
            stats.angular,
            stats.supported,
            &settings.config.activation,
        );
        stats.activation = brain.control.activation;
        stats.age = brain.control.age;
        stats.targets = brain.targets;
    }
    for (owner, s, mut joint, mut muscle) in &mut joints {
        let Some(stats) = scratch.get(&owner.owner) else {
            continue;
        };
        let c = &settings.config.joints[s.part];
        let target = stats.targets[s.part].slerp(
            rotation(c.brace),
            (stats.age / settings.config.activation.impact_seconds).clamp(0.0, 1.0),
        );
        if (muscle.activation - stats.activation).abs() > 0.00001
            || muscle.target.dot(target).abs() < 0.99999
        {
            set_drive(joint.data.as_mut(), c, stats.activation, target);
            muscle.activation = stats.activation;
            muscle.target = target;
        }
    }
}
