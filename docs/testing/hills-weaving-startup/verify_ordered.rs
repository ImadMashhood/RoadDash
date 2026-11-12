//! Standalone reproducible planner equivalence stress fixture, no engine required.
//! rustc --edition=2024 docs/testing/hills-weaving-startup/verify_ordered.rs -O -o /tmp/rd-ordered && /tmp/rd-ordered
#[allow(dead_code)]
#[path = "../../../crates/rr_core/src/traffic_avoidance.rs"]
mod traffic;
use traffic::{Car, Memory, Request};
fn main() {
    let mut seed = 0x283a89_u64;
    let mut random = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (seed >> 32) as u32 as f32 / u32::MAX as f32
    };
    let mut comparisons = 0;
    for _ in 0..1000 {
        let mut a = Memory::default();
        let mut b = Memory::default();
        for _ in 0..10 {
            let request = Request {
                lane: random() * 14. - 7.,
                speed: random() * 50.,
                desired: random() * 14. - 7.,
                cruise: 30. + random() * 20.,
                bounds: (-8.05, 8.05),
            };
            let cars: Vec<_> = (0..8)
                .map(|_| Car {
                    gap: random() * 400. - 60.,
                    lane: random() * 13. - 6.5,
                    speed: random() * 70. - 20.,
                    lateral_speed: (random() - 0.5) * 0.6,
                    lateral_goal: None,
                    half_width: 0.5 + random(),
                    half_length: 1. + random() * 3.,
                })
                .collect();
            let wall = random() * 14. - 7.;
            let safe = |goal: f32, speed: f32| (goal - wall).abs() > 0.7 || speed < 5.;
            let x = a.plan_local_ordered(request, &cars, Some(4.), false, safe);
            let y = b.plan_local_ordered(request, &cars, Some(4.), true, safe);
            assert_eq!((x.lane, x.speed, x.avoiding), (y.lane, y.speed, y.avoiding));
            assert_eq!(a.committed_lane(), b.committed_lane());
            assert_eq!(a.home_lane(), b.home_lane());
            comparisons += 1;
        }
    }
    println!(
        "{comparisons} sequential seeded planner comparisons match, including static blockers and passing memory"
    );
}
