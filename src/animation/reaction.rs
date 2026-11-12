//! Additive upper-body flinch. No root, pelvis, foot or physical state writes.
use super::*;

pub(super) fn apply(mut pose: Pose, hit: motion::HitReaction, now: f32) -> Pose {
    let w = hit.weight(now);
    if w == 0.0 {
        return pose;
    }
    // Positive local X impact travels right in author space; negative Z roll
    // carries the upper body in that direction. Kick folds the trunk more;
    // punch gives the head a sharper response. Existing IK keeps grips/pegs.
    let roll = -hit.direction * w * 2.2;
    let fold = if hit.kicking { 0.22 } else { 0.08 } * w;
    // The fast tuck has little remaining grip reach. Keep its trunk within
    // that envelope while retaining the stronger head/neck recoil.
    let trunk = 1.0 - pose.tuck * 0.6;
    for (joint, pitch, bank) in [
        (2, fold * 0.6 * trunk, roll * 0.075 * trunk),
        (3, fold * 0.4 * trunk, roll * 0.065 * trunk),
        (
            4,
            -fold * 0.3,
            roll * if hit.kicking { 0.025 } else { 0.055 },
        ),
        (5, 0.0, roll * if hit.kicking { 0.02 } else { 0.045 }),
    ] {
        pose.rotations[joint] =
            (Quat::from_rotation_z(bank) * Quat::from_rotation_x(pitch) * pose.rotations[joint])
                .normalize();
    }
    pose
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recoil_preserves_root_pelvis_held_landmarks_and_chain_lengths() {
        let lib = Library::default();
        for key in [2020, 2030, 2040, 2900, 2901] {
            let base = lib.key(key);
            for kicking in [false, true] {
                for side in [-1.0, 1.0] {
                    for step in 0..24 {
                        let pose = apply(
                            base,
                            motion::HitReaction::new(0.0, side, kicking),
                            step as f32 / 60.0,
                        );
                        assert_eq!(pose.root, base.root);
                        assert_eq!(pose.rotations[0..2], base.rotations[0..2]);
                        let world = lib.world_pose(pose);
                        for limb in &lib.data.limbs {
                            if pose.held & (1 << limb.bit) != 0 {
                                let contact =
                                    world[limb.end].transform_point(Vec3::from_array(limb.offset));
                                let error = contact.distance(Vec3::from_array(limb.target));
                                assert!(
                                    error < 2e-5,
                                    "key={key} side={side} kick={kicking} limb={} error={error}",
                                    limb.bit
                                );
                            }
                            let upper = world[limb.upper].translation;
                            let lower = world[limb.lower].translation;
                            let end = world[limb.end].translation;
                            assert!(
                                (upper.distance(lower)
                                    - Vec3::from_array(lib.data.offsets[limb.lower]).length())
                                .abs()
                                    < 2e-5
                            );
                            assert!(
                                (lower.distance(end)
                                    - Vec3::from_array(lib.data.offsets[limb.end]).length())
                                .abs()
                                    < 2e-5
                            );
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn reaction_cost_receipt() {
        let base = Library::default().key(2020);
        let hit = motion::HitReaction::new(0.0, 1.0, false);
        let mut samples = Vec::new();
        for _ in 0..7 {
            let start = std::time::Instant::now();
            for _ in 0..100_000 {
                std::hint::black_box(apply(
                    std::hint::black_box(base),
                    std::hint::black_box(hit),
                    std::hint::black_box(0.05),
                ));
            }
            samples.push(start.elapsed().as_nanos() as f64 / 100_000.0);
        }
        samples.sort_by(f64::total_cmp);
        println!(
            "reaction CPU sample: 100000 active layers/batch,7 batches,median_ns={:.2},min_ns={:.2},max_ns={:.2},pulse_option_bytes={},Rider_bytes={}; test opt-level1, no GPU/frame-budget claim",
            samples[3],
            samples[0],
            samples[6],
            std::mem::size_of::<Option<motion::HitReaction>>(),
            std::mem::size_of::<Rider>()
        );
    }
    #[test]
    fn reaction_has_no_authority_and_attack_pause_recovery_reset_keep_priority() {
        let (mut app, e) = super::super::tests::state_app();
        for _ in 0..120 {
            super::super::tests::tick(&mut app);
        }
        let physical = *app.world().get::<Transform>(e).unwrap();
        let velocity = app.world().get::<Rider>(e).unwrap().motion_velocity;
        let at = app.world().resource::<Race>().elapsed;
        app.world_mut().get_mut::<Rider>(e).unwrap().hit_reaction =
            Some(motion::HitReaction::new(at, 1.0, false));
        super::super::tests::tick(&mut app);
        let hit_pose = app.world().get::<Animator>(e).unwrap().current;
        app.world_mut().resource_mut::<Race>().phase = crate::game::Phase::Paused;
        // Real clock is paused; update must not change the pose even if a caller
        // supplies a fixed delta. The live scheduler separately freezes the pulse.
        app.world_mut().run_schedule(FixedUpdate);
        assert_eq!(
            app.world().get::<Animator>(e).unwrap().current.rotations,
            hit_pose.rotations
        );
        app.world_mut().resource_mut::<Race>().phase = crate::game::Phase::Racing;
        {
            let mut r = app.world_mut().get_mut::<Rider>(e).unwrap();
            r.attack_serial += 1;
            r.attack = 0.65;
            r.attack_side = 1.0;
            r.strike = Some(motion::StrikeClock::new(
                rr_core::AttackSide::Right,
                1.0,
                0.26,
            ));
            r.strike.as_mut().unwrap().age = motion::ATTACK_ENTER + 0.26 / motion::ATTACK_PLAYBACK;
        }
        super::super::tests::tick(&mut app);
        let expected = app
            .world()
            .resource::<Library>()
            .sample("punch_right", 0.26);
        for (actual, expected) in app
            .world()
            .get::<Animator>(e)
            .unwrap()
            .current
            .rotations
            .iter()
            .zip(expected.rotations)
        {
            assert!(actual.dot(expected).abs() > 0.999999);
        }
        assert_eq!(*app.world().get::<Transform>(e).unwrap(), physical);
        assert_eq!(
            app.world().get::<Rider>(e).unwrap().motion_velocity,
            velocity
        );
        app.world_mut().get_mut::<Recovery>(e).unwrap().stage = RecoveryStage::Falling;
        super::super::tests::tick(&mut app);
        assert!(app.world().get::<Animator>(e).unwrap().attack.is_none());
        // Reset clock makes the old pulse invalid even without rider reconstruction.
        assert_eq!(motion::HitReaction::new(at, 1.0, false).weight(0.0), 0.0);
    }
}
