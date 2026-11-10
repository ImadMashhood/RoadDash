//! Predictive traffic intent in a rider's road-relative frame. No collision exemptions.
//! Metres, seconds, and signed forward speeds; lane bounds describe usable outbound road.
/// Road-space proxy for a car or an upright rider occupying a traffic escape.
#[derive(Clone, Copy, Debug)]
pub struct Car {
    pub gap: f32,
    pub lane: f32,
    pub speed: f32,
    pub lateral_speed: f32,
    pub half_width: f32,
    pub half_length: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct Plan {
    pub lane: f32,
    pub speed: f32,
    pub avoiding: bool,
}
#[cfg(test)]
const CLEARANCE: f32 = 2.1;

// Conservative lane-change envelope: allow steering to unwind before lateral travel.
fn lane_at(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * (1.0 - (-0.65 * (t - 0.4).max(0.0)).exp())
}
fn clear(from: f32, to: f32, speed: f32, car: Car, horizon: f32) -> bool {
    let closing = speed - car.speed;
    let length = car.half_length + 3.55;
    let clearance = car.half_width + 1.3;
    let (start, end) = if closing.abs() < 0.01 {
        if car.gap.abs() >= length {
            return true;
        }
        (0.0, horizon)
    } else {
        let a = (car.gap - length) / closing;
        let b = (car.gap + length) / closing;
        (a.min(b).max(0.0), a.max(b).min(horizon))
    };
    if start > end || end < 0.0 || start > horizon {
        return true;
    }
    // Relative lateral motion is exponential rider convergence minus linear car
    // travel. Check endpoints, the steering delay kink and its one possible extremum;
    // endpoint-only snapshots can miss a car crossing the whole lane in this interval.
    let relative = |t| lane_at(from, to, t) - car.lane - car.lateral_speed * t;
    let mut low = relative(start).min(relative(end));
    let mut high = relative(start).max(relative(end));
    let ratio = car.lateral_speed / ((to - from) * 0.65);
    let extremum = if ratio > 0.0 {
        0.4 - ratio.ln() / 0.65
    } else {
        -1.0
    };
    for t in [0.4, extremum] {
        if t >= start && t <= end && t >= 0.4 {
            low = low.min(relative(t));
            high = high.max(relative(t));
        }
    }
    low > clearance || high < -clearance
}
/// Pick a reachable gap, otherwise follow/brake. Desired combat/passing intent is retained
/// when safe; it cannot override a car speed limit or request an occupied lane.
pub fn plan(
    lane: f32,
    speed: f32,
    desired: f32,
    cruise: f32,
    bounds: (f32, f32),
    cars: &[Car],
) -> Plan {
    let requested = desired;
    let desired = desired.clamp(bounds.0, bounds.1);
    let speed = speed.max(0.0);
    // At race pace a 35m trigger is shorter than both steering and stopping distance.
    let horizon = (2.5 + speed / 12.0).clamp(3.0, 7.0);
    let safe = |to| {
        cars.iter()
            .all(|c| clear(lane, to, speed.max(8.0), *c, horizon))
    };
    let avoiding = !safe(requested) || !safe(lane);
    if !avoiding {
        return Plan {
            lane: requested,
            speed: cruise,
            avoiding: false,
        };
    }
    let mut goal = desired;
    if !safe(goal) {
        let candidates = [lane.clamp(bounds.0, bounds.1), bounds.0, bounds.1]
            .into_iter()
            .chain(cars.iter().flat_map(|car| {
                let clearance = car.half_width + 1.55;
                [car.lane - clearance, car.lane + clearance]
            }));
        goal = candidates
            .filter(|x| *x >= bounds.0 && *x <= bounds.1 && safe(*x))
            .min_by(|a, b| {
                let cost = |x: f32| (x - lane).abs() + 0.35 * (x - desired).abs();
                cost(*a).total_cmp(&cost(*b)).then(a.total_cmp(b))
            })
            .unwrap_or(lane.clamp(bounds.0, bounds.1));
    }
    let mut limit = cruise;
    for car in cars {
        if car.gap < -(car.half_length + 3.55) || clear(lane, goal, speed.max(8.0), *car, horizon) {
            continue;
        }
        // Follow a slower car with a time headway and a conservative 6m/s² stopping
        // budget. Opposing/parked traffic is a stop obstruction until a pass is clear.
        let gap = (car.gap - car.half_length - 3.55 - speed * 0.5).max(0.0);
        let leader = car.speed.max(0.0);
        limit = limit.min((leader * leader + 12.0 * gap).sqrt());
        if car.speed < 0.0 {
            limit = 0.0;
        }
    }
    Plan {
        lane: goal,
        speed: limit.max(0.0),
        avoiding,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn car(gap: f32, lane: f32, speed: f32) -> Car {
        Car {
            gap,
            lane,
            speed,
            lateral_speed: 0.0,
            half_width: 0.95,
            half_length: 1.95,
        }
    }
    #[test]
    fn moving_cross_car_yields_before_lane_entry_and_resumes_after_clearance() {
        let crossing = Car {
            lateral_speed: 6.0,
            half_width: 1.95,
            half_length: 0.95,
            ..car(45., -12., 0.)
        };
        let before = plan(2., 20., 2., 40., (0.65, 2.35), &[crossing]);
        assert!(before.avoiding && before.speed < 20.);
        // A frozen point outside the road would look safe; velocity makes the conflict.
        assert_eq!(
            plan(
                2.,
                20.,
                2.,
                40.,
                (0.65, 2.35),
                &[Car {
                    lateral_speed: 0.,
                    ..crossing
                }]
            )
            .speed,
            40.
        );
        let cleared = Car {
            gap: 10.,
            lane: 12.,
            ..crossing
        };
        assert_eq!(plan(2., 10., 2., 40., (0.65, 2.35), &[cleared]).speed, 40.);
    }
    #[test]
    fn alongside_cross_motion_cannot_be_steered_through() {
        let cross = Car {
            lateral_speed: 5.,
            half_width: 1.95,
            half_length: 0.95,
            ..car(0., -4., 20.)
        };
        assert!(!clear(2., 5., 20., cross, 3.));
        assert!(!clear(2., 2., 20., cross, 3.));
        // Both interval endpoints are separated, but exponential rider convergence
        // catches a car mid-interval before it pulls away laterally again.
        let nonlinear = Car {
            lateral_speed: 2.,
            ..car(0., 4., 20.)
        };
        assert!(!clear(0., 10., 20., nonlinear, 5.));
    }
    #[test]
    fn occupied_rider_pass_lane_keeps_traffic_queue_clear() {
        let front = car(55., 2., 0.);
        let alongside = Car {
            half_width: 0.36,
            half_length: 1.05,
            ..car(0., 4.5, 35.)
        };
        let p = plan(2., 35., 4.5, 45., (0.65, 5.85), &[front, alongside]);
        assert!(p.speed < 35.);
        assert!((p.lane - alongside.lane).abs() > 1.5);
    }
    #[test]
    fn predicts_fast_approach_beyond_old_35m_trigger() {
        let p = plan(-2., 50., -2., 50., (-5., -0.7), &[car(100., -2., 15.)]);
        assert!((p.lane + 2.).abs() > CLEARANCE || p.speed < 50.);
    }
    #[test]
    fn occupied_pass_brakes_and_never_chooses_oncoming_side() {
        let cars = [car(60., -2., 15.), car(45., -4.5, -20.), car(80., 2., -20.)];
        let p = plan(-2., 40., 2., 45., (-5.5, -0.7), &cars);
        assert!(p.lane < 0.);
        assert!(p.speed < 40.);
    }
    #[test]
    fn cannot_swerve_through_a_car_alongside() {
        let p = plan(
            -2.,
            40.,
            -5.,
            40.,
            (-5., -0.7),
            &[car(0., -4., 40.), car(50., -2., 10.)],
        );
        assert!(p.lane > -3.);
        assert!(p.speed < 40.);
    }
    #[test]
    fn unobstructed_combat_and_racing_intents_are_unchanged() {
        let chase = plan(2., 35., -4., 45., (0.65, 5.85), &[]);
        assert_eq!(chase.lane, -4.); // A safe police/combat chase retains its old width.
        assert!(!chase.avoiding);
        for desired in [-4., -2., -0.7] {
            let p = plan(-2., 35., desired, 45., (-5., -0.7), &[car(-40., -2., 10.)]);
            assert_eq!(p.lane, desired);
            assert_eq!(p.speed, 45.);
        }
    }
    #[test]
    fn stationary_blockage_stops_and_cleared_traffic_resumes() {
        let p = plan(-2., 20., -2., 40., (-3., -1.), &[car(12., -2., 0.)]);
        assert_eq!(p.speed, 0.);
        assert_eq!(plan(-2., 0., -2., 40., (-3., -1.), &[]).speed, 40.);
    }
    #[test]
    fn results_do_not_depend_on_traffic_iteration_order() {
        let mut cars = [car(70., -2., 15.), car(45., -4., -20.), car(-20., -1., 50.)];
        let a = plan(-2., 40., -2., 45., (-5., -0.7), &cars);
        cars.reverse();
        let b = plan(-2., 40., -2., 45., (-5., -0.7), &cars);
        assert_eq!(a.lane, b.lane);
        assert_eq!(a.speed, b.speed);
    }
}
