//! Opt-in CPU rule-cost comparison; excludes native queries/rendering/physics.
use rr_core::traffic_avoidance::{Car, Memory, Request};
use std::{cell::Cell, hint::black_box, time::Instant};
fn main() {
    for case in ["clear", "blocked_outbound", "oncoming"] {
        let mut cars = [Car {
            gap: -1000.,
            lane: -2.,
            speed: 20.,
            lateral_speed: 0.,
            half_width: 0.95,
            half_length: 1.95,
        }; 96];
        for (i, car) in cars.iter_mut().enumerate() {
            car.gap = -15. * (i + 1) as f32;
        }
        if case != "clear" {
            cars[0].gap = 60.;
            cars[0].lane = 2.;
            cars[0].speed = 10.;
        }
        if case == "oncoming" {
            cars[1].gap = 100.;
            cars[1].speed = -25.;
        }
        let input = Request {
            lane: 2.,
            speed: 20.,
            desired: 2.,
            cruise: 35.,
            bounds: (0.65, 2.35),
        };
        for extended in [false, true] {
            let queries = Cell::new(0_u64);
            let start = Instant::now();
            for _ in 0..10000 {
                let mut memory = Memory::default();
                let drivable = |_: f32, _: f32| {
                    queries.set(queries.get() + 1);
                    true
                };
                let plan = if extended {
                    memory.plan_with_oncoming(
                        black_box(input),
                        black_box(&cars),
                        || (-2.35, 2.35),
                        drivable,
                    )
                } else {
                    memory.plan(black_box(input), black_box(&cars), drivable)
                };
                black_box(plan);
            }
            println!(
                "{{\"case\":\"{case}\",\"oncoming_enabled\":{extended},\"iterations\":10000,\"nanoseconds_per_plan\":{},\"static_callbacks_per_plan\":{},\"scope\":\"pure rules only; not native frame time\"}}",
                start.elapsed().as_nanos() / 10000,
                queries.get() as f64 / 10000.
            );
        }
    }
}
