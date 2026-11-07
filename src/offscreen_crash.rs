//! User-requested cheaper recovery for crashes behind the player and outside the view.
use crate::{
    game::Rider,
    recovery::{Recovery, RecoveryStage},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
#[derive(Clone, Copy)]
pub struct Deferred {
    pub origin: Vec3,
    pub ticks: u16,
}
pub const DELAY_TICKS: u16 = rr_core::performance::OFFSCREEN_RECOVERY_TICKS;
pub fn eligible(player: Transform, camera: Transform, point: Vec3) -> bool {
    rr_core::performance::defer_rear_crash(
        player.translation.to_array(),
        (player.rotation * Vec3::Z).to_array(),
        camera.translation.to_array(),
        camera.forward().to_array(),
        point.to_array(),
    )
}
pub fn classify(
    cameras: Query<&Transform, (With<Camera3d>, Without<Rider>)>,
    mut riders: Query<(&Rider, &Transform, &mut Recovery)>,
) {
    let player = riders
        .iter()
        .find(|(r, _, _)| r.id == 0)
        .map(|(_, t, _)| *t);
    let camera = cameras.single().ok().copied();
    for (r, t, mut rec) in &mut riders {
        rec.defer_allowed = r.id != 0
            && rec.stage == RecoveryStage::Riding
            && player
                .zip(camera)
                .is_some_and(|(p, c)| eligible(p, c, t.translation));
    }
}
pub fn start(
    commands: &mut Commands,
    owner: Entity,
    rider: &mut Rider,
    recovery: &mut Recovery,
    origin: Vec3,
) -> bool {
    if rider.id == 0 || !recovery.defer_allowed || recovery.stage != RecoveryStage::Riding {
        return false;
    }
    recovery.deferred = Some(Deferred {
        origin,
        ticks: DELAY_TICKS,
    });
    recovery.stage = RecoveryStage::Falling;
    recovery.elapsed = 0.0;
    rider.crashed = 1.0;
    rider.health = 0.0;
    rider.speed = 0.0;
    rider.motion_velocity = Vec3::ZERO;
    rider.motion_angular_velocity = Vec3::ZERO;
    rider.drive_intent = default();
    commands.entity(owner).insert((
        RigidBodyDisabled,
        ColliderDisabled,
        Visibility::Hidden,
        Velocity::zero(),
        ExternalForce::default(),
    ));
    true
}
// Used only before ordinary wreck recovery: no physical wreck or ragdoll is allocated.
#[allow(clippy::too_many_arguments)]
pub fn finish(
    commands: &mut Commands,
    owner: Entity,
    rider: &mut Rider,
    recovery: &mut Recovery,
    root: &mut Transform,
    visibility: &mut Visibility,
    track: &crate::track::Track,
) -> bool {
    let Some(pending) = &mut recovery.deferred else {
        return false;
    };
    pending.ticks = pending.ticks.saturating_sub(1);
    if pending.ticks == 0 {
        let (station, _, _) = track.project(pending.origin, rider.progress);
        let (p, q) = track.sample(station, 0.0);
        *root = Transform::from_translation(p).with_rotation(q);
        rider.progress = station;
        rider.lane = 0.0;
        rider.crashed = 0.0;
        rider.health = 100.0;
        rider.original_condition = crate::original_crash::initial_condition();
        rider.speed = 0.0;
        rider.side_velocity = 0.0;
        rider.hop_height = 0.0;
        rider.hop_velocity = 0.0;
        rider.lean = 0.0;
        rider.steering = default();
        rider.raw_steer = 0.0;
        rider.motion_velocity = Vec3::ZERO;
        rider.motion_angular_velocity = Vec3::ZERO;
        rider.hits = default();
        rider.immunity = 1.0;
        rider.impact_cooldown = 0.6;
        *visibility = Visibility::Visible;
        *recovery = Recovery::default();
        commands
            .entity(owner)
            .remove::<RigidBodyDisabled>()
            .remove::<ColliderDisabled>()
            .insert((
                Velocity::zero(),
                ExternalForce::default(),
                crate::runtime::PoseHistory {
                    previous: *root,
                    current: *root,
                },
            ));
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_distant_unseen_rear_riders_are_eligible() {
        let player = Transform::default();
        let camera = Transform::from_xyz(0.0, 3.0, -6.0).looking_at(Vec3::Z * 40.0, Vec3::Y);
        assert!(eligible(player, camera, Vec3::Z * -60.0));
        assert!(!eligible(player, camera, Vec3::Z * 60.0));
        assert!(!eligible(player, camera, Vec3::Z * -10.0));
        let rear = Transform::from_xyz(0.0, 3.0, -6.0).looking_at(Vec3::Z * -60.0, Vec3::Y);
        assert!(!eligible(player, rear, Vec3::Z * -60.0));
    }
    #[test]
    fn delay_is_five_simulated_seconds() {
        assert_eq!(DELAY_TICKS, 300);
    }
}
