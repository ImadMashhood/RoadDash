//! Predictive traffic intent in a rider's road-relative frame. No collision exemptions.
//! Metres, seconds, and signed forward speeds; bounds describe the usable physical corridor.
/// Road-space proxy for a car or an upright rider occupying a traffic escape.
#[derive(Clone, Copy, Debug)]
pub struct Car {
    pub gap: f32,
    pub lane: f32,
    pub speed: f32,
    pub lateral_speed: f32,
    /// Committed lane endpoint; stop extrapolating once measured motion reaches it.
    pub lateral_goal: Option<f32>,
    pub half_width: f32,
    pub half_length: f32,
}
impl Car {
    fn lateral_stop(self) -> f32 {
        self.lateral_goal.map_or(f32::INFINITY, |goal| {
            let time = (goal - self.lane) / self.lateral_speed;
            if time.is_finite() && time >= 0.0 {
                time
            } else {
                // A newly changed destination must not erase measured motion
                // still travelling away from it during the transition tick.
                f32::INFINITY
            }
        })
    }
    fn lane_at(self, time: f32) -> f32 {
        self.lane + self.lateral_speed * time.min(self.lateral_stop())
    }
    fn advance(self, time: f32, observer_speed: f32) -> Self {
        Self {
            gap: self.gap + (self.speed - observer_speed) * time,
            lane: self.lane_at(time),
            lateral_speed: if time >= self.lateral_stop() {
                0.0
            } else {
                self.lateral_speed
            },
            ..self
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Plan {
    pub lane: f32,
    pub speed: f32,
    pub avoiding: bool,
}
#[cfg(test)]
const CLEARANCE: f32 = 2.1;

/// Local same-direction race flow needs time to complete a lane change, not
/// an empty lane all the way to the terminal-speed forecast seven seconds out.
/// Recomputed every fixed tick. Crossing/oncoming forecasts are never shortened.
pub fn local_flow_horizon(speed: f32) -> f32 {
    (3.5 + speed.abs() / 40.).clamp(3.5, 5.)
}

// Conservative lane-change envelope: allow steering to unwind before lateral travel.
pub fn lane_at(from: f32, to: f32, speed: f32, t: f32) -> f32 {
    let rate = 0.65 * (speed / 4.).clamp(0.05, 1.);
    from + (to - from) * (1.0 - (-rate * (t - 0.4).max(0.0)).exp())
}
fn clear(from: f32, to: f32, speed: f32, car: Car, horizon: f32) -> bool {
    clear_with_padding(from, to, speed, car, horizon, 1.3)
}
fn clear_with_padding(
    from: f32,
    to: f32,
    speed: f32,
    car: Car,
    horizon: f32,
    padding: f32,
) -> bool {
    let closing = speed - car.speed;
    let length = car.half_length + 3.55;
    let clearance = car.half_width + padding;
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
    // travel until its committed endpoint. Check endpoints, both motion kinks and
    // the one possible extremum before the car settles;
    // endpoint-only snapshots can miss a car crossing the whole lane in this interval.
    let relative = |t| lane_at(from, to, speed, t) - car.lane_at(t);
    let mut low = relative(start).min(relative(end));
    let mut high = relative(start).max(relative(end));
    let rate = 0.65 * (speed / 4.).clamp(0.05, 1.);
    let ratio = car.lateral_speed / ((to - from) * rate);
    let extremum = if ratio > 0.0 {
        0.4 - ratio.ln() / rate
    } else {
        -1.0
    };
    for t in [0.4, car.lateral_stop(), extremum] {
        if t >= start && t <= end && (t != extremum || (t >= 0.4 && t <= car.lateral_stop())) {
            low = low.min(relative(t));
            high = high.max(relative(t));
        }
    }
    low > clearance || high < -clearance
}
// A front bike may leave an already-overlapping comfort envelope at walking
// speed. Its stationary follower stays behind and separation only increases;
// actual colliders/CCD still resolve contact. Never grant this to cars, an
// oncoming body, a leader ahead, or a faster following rider.
fn clear_local(from: f32, to: f32, speed: f32, car: Car, horizon: f32, local: bool) -> bool {
    // A steady oncoming lane uses the bike's 0.38m half-width plus
    // 0.27m clearance, rather than an extra 1.3m that erases usable gaps.
    // Keep the full time horizon; crossing motion retains the wider margin.
    let padding = if local && car.speed < 0. && car.lateral_speed.abs() <= 0.25 {
        0.65
    } else {
        1.3
    };
    if clear_with_padding(from, to, speed, car, horizon, padding) {
        return true;
    }
    let side = from - car.lane;
    local
        && speed <= 4.5
        && car.half_width <= 0.5
        && car.half_length <= 1.3
        && car.speed >= 0.
        && car.speed <= speed + 0.25
        && (-(car.half_length + 3.55)..-0.5).contains(&car.gap)
        && side.abs() >= 0.5
        && side * (to - from) > 0.
        && side * car.lateral_speed <= 0.
        && (lane_at(from, to, speed, horizon) - car.lane_at(horizon)).abs() > side.abs() + 0.1
}

fn crossing_conflict(lane: f32, home: f32, speed: f32, cars: &[Car]) -> bool {
    cars.iter()
        .any(|c| c.lateral_speed.abs() > 0.25 && !clear(lane, home, speed, *c, 12.))
}
/// Require enough relative speed to clear the whole nearby queue and return
/// within twelve seconds. This is a forecast, never a collision exemption.
/// Inbound candidates must remain clear through both outward and return legs.
pub fn oncoming_pass_clear(lane: f32, goal: f32, home: f32, pace: f32, cars: &[Car]) -> bool {
    if pace < 1. || crossing_conflict(lane, home, pace, cars) {
        // Yield only when crossing motion intersects our home corridor while
        // longitudinal envelopes overlap. Nearby parallel roads and traffic that
        // clears before arrival must not veto an otherwise safe passing route.
        return false;
    }
    let mut return_at = 0.0_f32;
    for c in cars.iter().filter(|c| {
        c.gap > -15. && c.gap < 120. && (c.lane - home).abs() < c.half_width + 1.3 && c.speed < pace
    }) {
        return_at = return_at.max((c.gap + c.half_length + 15.) / (pace - c.speed));
    }
    if return_at > 8. {
        return false;
    }
    let return_at = return_at.max(0.5);
    cars.iter().all(|c| {
        clear(lane, goal, pace, *c, return_at + 4.) && {
            let shifted = c.advance(return_at, pace);
            clear(goal, home, pace, shifted, 4.)
        }
    })
}
/// Slow upright riders are traffic leaders even when no car triggered avoidance.
/// Keep ordinary race-speed combat spacing separate from a crawling queue.
pub fn slow_queue_ahead(lane: f32, cruise: f32, rider: Car) -> bool {
    rider.gap > 0.
        && rider.gap < 120.
        && rider.speed < (cruise * 0.75).max(4.)
        && rider.speed < cruise - 3.
        && (rider.lane - lane).abs() < rider.half_width + 1.3
}
/// Filter resting contact-solver jitter without discarding meaningful backing.
pub fn rider_forward_speed(speed: f32) -> f32 {
    if speed.abs() < 0.1 { 0. } else { speed }
}
/// A stopped upright rider inside the reserved launch gap needs room restored
/// before another forward pass can be planned. The adapter must independently
/// sweep the rear before using its bounded reverse controller.
pub fn queue_launch_deficit(lane: f32, rider: Car) -> f32 {
    if rider.gap > 0. && rider.speed < 0.65 && (rider.lane - lane).abs() < rider.half_width + 1.3 {
        (rider.half_length + 3.55 + 7. - rider.gap).max(0.)
    } else {
        0.
    }
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
    Memory::default().plan(
        Request {
            lane,
            speed,
            desired,
            cruise,
            bounds,
        },
        cars,
        |_, _| true,
    )
}

/// Fixed-tick passing commitment. Safety is re-evaluated every tick; memory never
/// grants a lane reservation or an exemption from collision.
#[derive(Clone, Copy, Debug, Default)]
pub struct Memory {
    pass: Option<(f32, f32)>, // committed lane, original lane
}
#[derive(Clone, Copy, Debug)]
pub struct Request {
    pub lane: f32,
    pub speed: f32,
    pub desired: f32,
    pub cruise: f32,
    pub bounds: (f32, f32),
}
impl Memory {
    /// A rider may choose any road lane. An inbound choice must admit a complete
    /// return forecast, even when the rider has no immediate queue to pass.
    /// The adapter supplies lazy road geometry and resident static-body sweeps.
    pub fn plan_with_oncoming(
        &mut self,
        input: Request,
        cars: &[Car],
        road_bounds: impl FnOnce() -> (f32, f32),
        drivable: impl Fn(f32, f32) -> bool,
    ) -> Plan {
        let inbound = |x: f32| x < input.bounds.0 || x > input.bounds.1;
        if input.bounds.0 > input.bounds.1 {
            return self.plan(input, cars, drivable);
        }
        let held_inbound = self.committed_lane().is_some_and(inbound);
        let wants_inbound = inbound(input.desired);
        let original = *self;
        let home = original
            .home_lane()
            .unwrap_or(input.desired)
            .clamp(input.bounds.0, input.bounds.1);
        // A failed opposite-lane forecast must return the rider well inside the
        // outbound lane. Clamping an inbound wish to the divider edge leaves a
        // full-size bike inside an oncoming car's clearance envelope.
        let refuge = (input.bounds.0 + input.bounds.1) * 0.5;
        let direct_inbound = wants_inbound
            && !held_inbound
            && oncoming_pass_clear(input.lane, input.desired, home, input.cruise, cars)
            && drivable(input.desired, input.cruise);
        let outbound = if held_inbound {
            None
        } else {
            Some(self.plan(
                Request {
                    desired: if wants_inbound && !direct_inbound {
                        refuge
                    } else {
                        home
                    },
                    ..input
                },
                cars,
                &drivable,
            ))
        };
        if held_inbound
            || wants_inbound
            || outbound.is_some_and(|p| p.speed < input.cruise && !self.active())
        {
            let mut candidate = original;
            // A committed pass still returns to its home lane after clearing
            // the leader; fresh free-lane intent applies on a later tick.
            let pass = candidate.plan(
                Request {
                    bounds: road_bounds(),
                    desired: if direct_inbound { input.desired } else { home },
                    ..input
                },
                cars,
                |goal, pace| {
                    (!inbound(goal)
                        || (!crossing_conflict(input.lane, home, input.speed.max(1.), cars)
                            && oncoming_pass_clear(input.lane, goal, home, pace, cars)))
                        && drivable(goal, pace)
                },
            );
            if held_inbound
                || (inbound(pass.lane)
                    && (direct_inbound || outbound.is_some_and(|p| pass.speed > p.speed)))
            {
                *self = candidate;
                return pass;
            }
        }
        outbound.unwrap_or(Plan {
            lane: input.lane,
            speed: 0.,
            avoiding: true,
        })
    }
    pub fn home_lane(&self) -> Option<f32> {
        self.pass.map(|(_, home)| home)
    }
    pub fn committed_lane(&self) -> Option<f32> {
        self.pass.map(|(lane, _)| lane)
    }
    pub fn active(&self) -> bool {
        self.pass.is_some()
    }
    pub fn plan(
        &mut self,
        input: Request,
        cars: &[Car],
        drivable: impl Fn(f32, f32) -> bool,
    ) -> Plan {
        self.plan_local(input, cars, None, drivable)
    }

    /// Replan local same-direction flow sooner on dense streets. Oncoming and
    /// crossing traffic always retain the full forecast; no contact is exempted.
    pub fn plan_local(
        &mut self,
        input: Request,
        cars: &[Car],
        local_horizon: Option<f32>,
        drivable: impl Fn(f32, f32) -> bool,
    ) -> Plan {
        self.plan_local_ordered(input, cars, local_horizon, true, drivable)
    }

    /// Same deterministic cost order as exhaustive selection, with optional
    /// short-circuiting after the cheapest safe/static-clear lane. The reference
    /// path remains available for matched verification.
    pub fn plan_local_ordered(
        &mut self,
        input: Request,
        cars: &[Car],
        local_horizon: Option<f32>,
        ordered: bool,
        drivable: impl Fn(f32, f32) -> bool,
    ) -> Plan {
        let Request {
            lane,
            speed,
            desired,
            cruise,
            bounds,
        } = input;
        let speed = speed.max(0.0);
        if bounds.0 > bounds.1 {
            self.pass = None;
            return Plan {
                lane,
                speed: 0.,
                avoiding: true,
            };
        }
        let returning = self.active();
        if let Some((held, home)) = self.pass {
            // Keep the same side until the entire queue is behind. Returning is
            // still subjected to the rear/alongside predictive envelope below.
            if held < bounds.0
                || held > bounds.1
                || !cars.iter().any(|c| {
                    c.gap > -15.
                        && c.gap < 80.
                        && c.speed < cruise - 1.
                        && (c.lane - home).abs() < c.half_width + 1.3
                })
            {
                self.pass = None;
            }
        }
        let requested = self.pass.map_or(desired, |(held, _)| held);
        let full_horizon = (2.5 + speed / 12.0).clamp(6.0, 7.0);
        let horizon = |car: Car| {
            if car.speed < 0. || car.lateral_speed.abs() > 0.25 {
                full_horizon
            } else {
                local_horizon
                    .unwrap_or(full_horizon)
                    .clamp(3.5, full_horizon)
            }
        };
        let safe = |to, pace| {
            cars.iter().all(|c| {
                clear_local(lane, to, pace, *c, horizon(*c), local_horizon.is_some())
                    && clear_local(
                        lane,
                        to,
                        speed.max(1.),
                        *c,
                        horizon(*c),
                        local_horizon.is_some(),
                    )
                    && ((to - lane).abs() <= 0.5
                        || clear_local(to, to, pace, *c, horizon(*c), local_horizon.is_some()))
            })
        };
        // Evaluate intended acceleration too: a matched-speed follower otherwise
        // sees no hazard, accelerates into its leader, brakes, and repeats forever.
        let pace = cruise.max(speed).max(1.);
        let static_blocked = !drivable(requested, pace);
        let avoiding = returning || !safe(requested, pace) || !safe(lane, pace) || static_blocked;
        if !avoiding {
            return Plan {
                lane: requested,
                speed: cruise,
                avoiding: false,
            };
        }
        let desired = requested.clamp(bounds.0, bounds.1);
        let mut candidates: Vec<_> = [desired, lane.clamp(bounds.0, bounds.1), bounds.0, bounds.1]
            .into_iter()
            .chain(cars.iter().flat_map(|car| {
                let clearance = car.half_width
                    + if local_horizon.is_some()
                        && car.speed < 0.
                        && car.lateral_speed.abs() <= 0.25
                    {
                        0.85
                    } else {
                        1.55
                    };
                [car.lane - clearance, car.lane + clearance]
            }))
            .filter(|x| *x >= bounds.0 && *x <= bounds.1)
            .collect();
        if local_horizon.is_some() {
            candidates.push((bounds.0 + bounds.1) * 0.5);
            // Distant cars in the same lane differ by sub-millimetre projection
            // jitter. Do not repeat an entire world sweep for each duplicate.
            candidates.sort_by(f32::total_cmp);
            candidates.dedup_by(|a, b| (*a - *b).abs() < 0.025);
        }
        let compare = |a: &f32, b: &f32| {
            let cost = |x: f32| (x - desired).abs() + 0.35 * (x - lane).abs();
            cost(*a).total_cmp(&cost(*b)).then(a.total_cmp(b))
        };
        let ordered = ordered && local_horizon.is_some();
        if ordered {
            candidates.sort_by(compare);
        }
        // A queued bike can physically steer at walking pace. The old mandatory
        // 8m/s prediction rejected all close passes, even with empty adjacent road.
        // A safe current-speed plan must not become a permanent governor.
        // Try a modest acceleration forecast before falling back to matching.
        let step = if local_horizon.is_some() {
            (speed + 4.).min(pace)
        } else {
            pace
        };
        for trial in [pace, step, speed.max(4.).min(pace), 4., 2., 1.] {
            if (trial < speed - 0.25 && !static_blocked) || trial > pace {
                continue;
            }
            let allowed = |to: &f32| {
                safe(*to, trial)
                    && (trial >= pace
                        || local_horizon.is_some()
                        || cars
                            .iter()
                            // Do not settle into a nearby blocked line, but
                            // do not let distant full-cruise speculation veto
                            // a lateral walking-speed escape that is replanned.
                            .filter(|c| (to - lane).abs() <= 0.5 || c.gap.abs() < 60.)
                            .all(|c| clear(*to, *to, pace, *c, horizon(*c))))
                    && drivable(*to, trial)
            };
            let goal = if ordered {
                candidates.iter().copied().find(&allowed)
            } else {
                candidates.iter().copied().filter(&allowed).min_by(compare)
            };
            if let Some(goal) = goal {
                if (goal - lane).abs() > 0.5 && self.pass.is_none() {
                    self.pass = Some((goal, lane));
                } else if let Some((_, home)) = self.pass {
                    self.pass = Some((goal, home));
                }
                return Plan {
                    lane: goal,
                    speed: cruise.min(trial),
                    avoiding: true,
                };
            }
        }
        let mut limit = cruise;
        for car in cars {
            if car.gap < -(car.half_length + 3.55)
                || (clear(lane, lane, pace, *car, horizon(*car))
                    && clear(lane, lane, speed.max(1.), *car, horizon(*car)))
            {
                continue;
            }
            // A bike cannot launch a lane change from the collision envelope's
            // nose-to-tail stop point. Preserve room for the steering delay and
            // convergence at walking pace when a slow queue temporarily seals
            // every passing candidate; otherwise that queue can never reopen.
            let maneuver_room = if car.speed.abs() < 4. { 7. } else { 0. };
            let gap = (car.gap - car.half_length - 3.55 - speed * 0.5 - maneuver_room).max(0.);
            let leader = car.speed.max(0.);
            limit = limit.min((leader * leader + 12. * gap).sqrt());
            if car.speed < 0. {
                limit = 0.;
            }
        }
        // If geometry invalidated an otherwise clear route, stop and let the
        // existing bounded physical escape planner recover/replan.
        if !drivable(lane.clamp(bounds.0, bounds.1), speed.max(1.)) {
            limit = 0.;
        }
        Plan {
            lane: lane.clamp(bounds.0, bounds.1),
            speed: limit.max(0.),
            avoiding: true,
        }
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
            lateral_goal: None,
            half_width: 0.95,
            half_length: 1.95,
        }
    }
    #[test]
    fn ordered_lane_search_matches_exhaustive_choice_with_fewer_static_queries() {
        let request = Request {
            lane: 2.,
            speed: 30.,
            desired: 2.,
            cruise: 40.,
            bounds: (-7., 7.),
        };
        let cars = [car(35., 2., 12.), car(200., -3., 18.), car(210., 3.2, 18.)];
        let reference_calls = std::cell::Cell::new(0);
        let ordered_calls = std::cell::Cell::new(0);
        let a = Memory::default().plan_local_ordered(request, &cars, Some(4.), false, |_, _| {
            reference_calls.set(reference_calls.get() + 1);
            true
        });
        let b = Memory::default().plan_local_ordered(request, &cars, Some(4.), true, |_, _| {
            ordered_calls.set(ordered_calls.get() + 1);
            true
        });
        assert_eq!((a.lane, a.speed, a.avoiding), (b.lane, b.speed, b.avoiding));
        assert!(
            ordered_calls.get() < reference_calls.get(),
            "{}/{}",
            ordered_calls.get(),
            reference_calls.get()
        );
    }

    #[test]
    fn paired_oncoming_cars_admit_a_real_middle_or_sidewalk_gap_but_not_a_closed_one() {
        let request = Request {
            lane: -1.8,
            speed: 40.,
            desired: 0.,
            cruise: 45.,
            bounds: (-8.05, 8.05),
        };
        let cars = [car(260., -1.8, -13.), car(280., 1.8, -13.)];
        let middle = Memory::default().plan_local(
            request,
            &cars,
            Some(local_flow_horizon(40.)),
            |goal, _| goal.abs() < 0.2,
        );
        assert!(middle.lane.abs() < 0.2 && middle.speed == 45., "{middle:?}");
        let outer = Memory::default().plan_local(
            request,
            &cars,
            Some(local_flow_horizon(40.)),
            |goal, _| goal > 6.3,
        );
        assert!(outer.lane > 6.3 && outer.speed == 45., "{outer:?}");
        let closed = Memory::default().plan_local(request, &cars, Some(4.), |_, _| false);
        assert_eq!(closed.speed, 0.);
        // Cars too close together cannot be passed through their actual bodies.
        let tight = [car(260., -1.3, -13.), car(280., 1.3, -13.)];
        let stop =
            Memory::default().plan_local(request, &tight, Some(4.), |goal, _| goal.abs() < 0.2);
        assert!(stop.speed < 40., "{stop:?}");
        let late = [car(15., -1.8, -13.), car(20., 1.8, -13.)];
        assert!(
            Memory::default()
                .plan_local(request, &late, Some(4.), |goal, _| goal.abs() < 0.2)
                .speed
                < 40.
        );
    }

    #[test]
    fn local_flow_accelerates_incrementally_instead_of_latching_current_speed() {
        let request = Request {
            lane: 2.,
            speed: 32.,
            desired: 2.,
            cruise: 50.,
            bounds: (1.9, 2.1),
        };
        let leader = car(32., 2., 32.);
        let old = Memory::default().plan(request, &[leader], |_, _| true);
        let next = Memory::default().plan_local(
            request,
            &[leader],
            Some(local_flow_horizon(32.)),
            |_, _| true,
        );
        assert!(next.speed > old.speed, "{old:?} {next:?}");
        assert!(next.speed > 32. && next.speed <= 36., "{next:?}");
        assert!(clear(
            2.,
            next.lane,
            next.speed,
            leader,
            local_flow_horizon(32.)
        ));
    }

    #[test]
    fn stopped_front_bike_can_separate_from_a_close_follower_without_contact_exemptions() {
        // Captured Hills650m pair: front at652.43/-0.85, follower650.64/-1.58.
        let follower = Car {
            half_width: 0.36,
            half_length: 1.05,
            ..car(-1.79, -1.58, 0.)
        };
        let request = Request {
            lane: -0.85,
            speed: 0.,
            desired: -2.72,
            cruise: 59.,
            bounds: (-6.85, 6.85),
        };
        let plan = Memory::default().plan_local(
            request,
            &[follower],
            Some(local_flow_horizon(0.)),
            |_, _| true,
        );
        assert!(
            plan.speed > 0. && plan.speed <= 4.5 && plan.lane > request.lane,
            "{plan:?}"
        );
        for danger in [
            Car {
                gap: 1.79,
                ..follower
            },
            Car {
                speed: -1.,
                ..follower
            },
            Car {
                speed: 12.,
                ..follower
            },
            Car {
                half_width: 0.95,
                ..follower
            },
        ] {
            assert!(!clear_local(-0.85, 0.2, 4., danger, 4., true), "{danger:?}");
        }
        assert!(!clear_local(-0.85, -2., 4., follower, 4., true));
        let wall = Memory::default().plan_local(request, &[follower], Some(4.), |_, _| false);
        assert_eq!(wall.speed, 0.);
    }

    #[test]
    fn local_flow_ignores_distant_future_occupancy_but_retains_real_hazards() {
        let request = Request {
            lane: 2.,
            speed: 40.,
            desired: 2.,
            cruise: 50.,
            bounds: (1.8, 2.2),
        };
        let distant = car(250., 2., 0.);
        let conservative = Memory::default().plan(request, &[distant], |_, _| true);
        let local = Memory::default().plan_local(
            request,
            &[distant],
            Some(local_flow_horizon(40.)),
            |_, _| true,
        );
        assert!(conservative.avoiding);
        assert!(!local.avoiding && local.speed == 50.);
        for hazard in [
            car(35., 2., 18.),
            car(250., 2., -13.),
            Car {
                lateral_speed: 1.,
                ..car(220., -3., 0.)
            },
        ] {
            let full = Memory::default().plan(request, &[hazard], |_, _| true);
            let local = Memory::default().plan_local(
                request,
                &[hazard],
                Some(local_flow_horizon(40.)),
                |_, _| true,
            );
            assert!(local.avoiding, "{hazard:?} {local:?}");
            assert!(local.speed <= full.speed, "{hazard:?} {full:?} {local:?}");
        }
        let wall = Memory::default().plan_local(request, &[], Some(4.), |_, _| false);
        assert_eq!(wall.speed, 0.);
    }

    #[test]
    fn committed_lane_endpoint_remains_occupied_after_transition() {
        let moving = Car {
            lateral_speed: 4.,
            lateral_goal: Some(2.),
            ..car(100., -8., 0.)
        };
        assert!(!clear(2., 2., 20., moving, 12.));
        assert!(clear(
            2.,
            2.,
            20.,
            Car {
                lateral_goal: None,
                ..moving
            },
            12.
        ));
        let settled = moving.advance(4., 20.);
        assert_eq!(settled.lane, 2.);
        assert_eq!(settled.lateral_speed, 0.);
        assert!(!clear(2., 2., 20., settled, 4.));
    }
    #[test]
    fn committed_lane_forecast_does_not_teleport_when_held_or_reversing() {
        let held = Car {
            lateral_goal: Some(2.),
            ..car(20., -8., 0.)
        };
        assert_eq!(held.lane_at(12.), -8.);
        let reversing = Car {
            lateral_speed: -1.,
            ..held
        };
        assert_eq!(reversing.lane_at(12.), -20.);
    }
    #[test]
    fn slow_rider_queue_is_a_hazard_without_car_traffic() {
        let rider = Car {
            half_width: 0.36,
            half_length: 1.05,
            ..car(18., 2., 2.)
        };
        assert!(slow_queue_ahead(2., 40., rider));
        assert!(!slow_queue_ahead(
            2.,
            40.,
            Car {
                speed: 35.,
                ..rider
            }
        ));
        assert!(!slow_queue_ahead(2., 40., Car { gap: -18., ..rider }));
        let queue = [rider, Car { gap: 30., ..rider }, Car { gap: 42., ..rider }];
        let mut memory = Memory::default();
        let input = Request {
            lane: 2.,
            speed: 2.,
            desired: 2.,
            cruise: 40.,
            bounds: (0.65, 5.85),
        };
        let p = memory.plan(input, &queue, |_, _| true);
        assert!(p.avoiding && p.lane > 3.6 && p.speed > 0., "{p:?}");
        // A temporarily closed corridor must not be latched forever. Re-evaluate
        // every tick and resume the committed side as soon as the opening is safe.
        let blocked = memory.plan(input, &queue, |_, _| false);
        assert_eq!(blocked.speed, 0.);
        let resumed = memory.plan(input, &queue, |_, _| true);
        assert!(resumed.lane > 3.6 && resumed.speed > 0., "{resumed:?}");
    }
    #[test]
    fn crawling_rider_can_advance_when_distant_rivals_block_full_cruise() {
        // The Metropolis station-3105 trace had a temporarily stopped rider
        // with distant vehicles and rivals across several lines. A cruise-speed lookahead
        // must not veto a safe bounded 4 m/s step that is replanned next tick.
        let request = Request {
            lane: 2.58,
            speed: 0.1,
            desired: 1.47,
            cruise: 54.,
            bounds: (-5.05, 5.05),
        };
        let riders = [
            car(100., -4., 0.),
            car(100., 0., 0.),
            car(100., 4., 0.),
            Car {
                half_width: 0.36,
                half_length: 1.05,
                ..car(26., 5.1, 0.)
            },
        ];
        let mut memory = Memory::default();
        let plan = memory.plan(request, &riders, |_, _| true);
        assert!(plan.avoiding && plan.speed >= 4., "{plan:?}");
        let horizon = (2.5 + request.speed / 12.).clamp(6., 7.);
        assert!(
            riders
                .iter()
                .any(|r| !clear(plan.lane, plan.lane, request.cruise, *r, horizon)),
            "{plan:?}"
        );
        for rider in riders {
            assert!(clear(request.lane, plan.lane, plan.speed, rider, horizon));
            assert!(clear(
                request.lane,
                plan.lane,
                request.speed.max(1.),
                rider,
                horizon
            ));
        }
    }
    #[test]
    fn rider_speed_deadband_does_not_turn_resting_jitter_into_oncoming_traffic() {
        for speed in [-0.099, 0., 0.099] {
            assert_eq!(rider_forward_speed(speed), 0.);
        }
        for speed in [-2.5, -0.1, 0.1, 2.5] {
            assert_eq!(rider_forward_speed(speed), speed);
        }
        let request = |speed| {
            plan(
                1.,
                20.,
                1.,
                40.,
                (0.65, 2.35),
                &[car(40., 1., rider_forward_speed(speed))],
            )
        };
        assert!(request(-0.099).speed > 1.);
        assert_eq!(request(-0.1).speed, 0.);
    }
    #[test]
    fn compressed_queue_reposition_is_only_for_close_stopped_leaders() {
        let rider = Car {
            half_width: 0.36,
            half_length: 1.05,
            ..car(4.7, 1., 0.)
        };
        assert!(queue_launch_deficit(1., rider) > 0.5);
        for other in [
            Car { gap: -4.7, ..rider },
            Car { gap: 11.6, ..rider },
            Car { speed: 2., ..rider },
            Car {
                lane: -2.35,
                ..rider
            },
        ] {
            assert!(queue_launch_deficit(1., other) <= 0.5, "{other:?}");
        }
    }
    #[test]
    fn stopped_queue_selects_walking_speed_pass_and_holds_it() {
        let mut memory = Memory::default();
        let front = car(16., 2., 0.);
        let request = Request {
            lane: 2.,
            speed: 0.,
            desired: 2.,
            cruise: 40.,
            bounds: (0.65, 5.85),
        };
        let p = memory.plan(request, &[front], |_, _| true);
        assert!(p.lane > 4.5 && p.speed > 0. && p.speed <= 4., "{p:?}");
        assert!(memory.active());
        let held = memory.plan(
            Request {
                lane: 4.8,
                speed: 3.,
                desired: 0.65,
                ..request
            },
            &[car(8., 2., 0.)],
            |_, _| true,
        );
        assert!(held.lane > 4.5, "{held:?}");
        let returned = memory.plan(
            Request {
                lane: 5.85,
                speed: 10.,
                ..request
            },
            &[car(-25., 2., 0.)],
            |_, _| true,
        );
        assert_eq!(returned.lane, 2.);
        memory.plan(
            Request {
                lane: 2.,
                speed: 10.,
                ..request
            },
            &[car(-35., 2., 0.)],
            |_, _| true,
        );
        assert!(!memory.active());
    }
    #[test]
    fn matched_speed_follower_plans_before_accelerating_into_queue() {
        let p = plan(2., 8., 2., 40., (0.65, 5.85), &[car(20., 2., 8.)]);
        assert!(p.avoiding);
        assert!(p.lane >= 4.5 && p.speed <= 8., "{p:?}");
    }
    #[test]
    fn stopped_queue_waits_for_occupied_rear_and_oncoming_gaps() {
        for obstruction in [
            car(0., 5.85, 0.),
            car(-15., 5.85, 20.),
            car(35., 5.85, -15.),
        ] {
            let p = plan(
                2.,
                0.,
                2.,
                40.,
                (0.65, 5.85),
                &[car(16., 2., 0.), obstruction],
            );
            assert!(p.lane < 4.5 && p.speed < 40., "{p:?}");
        }
    }
    #[test]
    fn committed_pass_rechecks_new_obstacles_and_a_dropping_lane() {
        let mut memory = Memory::default();
        let request = Request {
            lane: 2.,
            speed: 0.,
            desired: 2.,
            cruise: 40.,
            bounds: (0.65, 5.85),
        };
        assert!(memory.plan(request, &[car(16., 2., 0.)], |_, _| true).lane > 4.5);
        let blocked = memory.plan(request, &[car(16., 2., 0.), car(0., 5.85, 0.)], |_, _| true);
        assert!(blocked.lane < 4.5 && blocked.speed < 40.);
        let merged = memory.plan(
            Request {
                bounds: (0.65, 2.35),
                ..request
            },
            &[car(16., 2., 0.)],
            |_, _| true,
        );
        assert!(merged.lane <= 2.35 && !memory.active());
    }
    #[test]
    fn free_sidewalk_line_is_allowed_but_a_solid_outer_line_is_rejected() {
        let request = Request {
            lane: 2.,
            speed: 15.,
            desired: 5.,
            cruise: 20.,
            bounds: (-5.5, 5.5),
        };
        let open = Memory::default().plan(request, &[], |_, _| true);
        assert_eq!(open.lane, 5.);
        let solid = Memory::default().plan(request, &[], |goal, _| goal < 4.);
        assert!(solid.lane < 4., "solid outer line chosen: {solid:?}");
    }
    #[test]
    fn narrowed_road_and_solid_pass_corridor_wait_instead_of_using_sidewalk() {
        let p = plan(2., 0., 2., 40., (0.65, 2.35), &[car(16., 2., 0.)]);
        assert!(p.lane <= 2.35 && p.speed < 40.);
        let mut memory = Memory::default();
        let p = memory.plan(
            Request {
                lane: 2.,
                speed: 0.,
                desired: 2.,
                cruise: 40.,
                bounds: (0.65, 5.85),
            },
            &[car(16., 2., 0.)],
            |lane, _| lane < 4.,
        );
        assert!(p.lane < 4. && p.speed < 40.);
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

#[cfg(test)]
mod inbound_tests {
    use super::*;
    fn car(gap: f32, lane: f32, speed: f32) -> Car {
        Car {
            gap,
            lane,
            speed,
            lateral_speed: 0.,
            lateral_goal: None,
            half_width: 0.95,
            half_length: 1.95,
        }
    }
    #[test]
    fn opposite_lane_pass_requires_a_complete_clear_return() {
        let front = car(60., 2., 10.);
        assert!(oncoming_pass_clear(2., -2., 2., 25., &[front]));
        assert!(!oncoming_pass_clear(
            2.,
            -2.,
            2.,
            25.,
            &[front, car(200., -2., -25.)]
        ));
        assert!(!oncoming_pass_clear(2., -2., 2., 11., &[front]));
        assert!(!oncoming_pass_clear(
            2.,
            -2.,
            2.,
            25.,
            &[front, car(0., 2., 25.)]
        ));
    }
    #[test]
    fn unrelated_crossing_motion_does_not_veto_a_clear_pass() {
        let front = car(60., 2., 10.);
        for crossing in [
            Car {
                lateral_speed: 6.,
                ..car(45., 100., 0.)
            },
            Car {
                lateral_speed: 6.,
                ..car(45., -2., 0.)
            },
            Car {
                lateral_speed: -6.,
                ..car(100., -12., 0.)
            },
        ] {
            assert!(
                oncoming_pass_clear(2., -2., 2., 25., &[front, crossing]),
                "{crossing:?}"
            );
        }
    }
    #[test]
    fn crossing_priority_checks_current_arrival_not_only_optimistic_cruise() {
        let crossing = Car {
            lateral_speed: 6.,
            ..car(45., -12., 0.)
        };
        assert!(!crossing_conflict(2., 2., 40., &[crossing]));
        assert!(crossing_conflict(2., 2., 20., &[crossing]));
        let p = Memory::default().plan_with_oncoming(
            Request {
                lane: 2.,
                speed: 20.,
                desired: 2.,
                cruise: 40.,
                bounds: (0.65, 2.35),
            },
            &[crossing],
            || (-2.35, 2.35),
            |_, _| true,
        );
        assert!(p.lane > 0. && p.speed < 20., "{p:?}");
    }
    #[test]
    fn actual_crossing_conflict_keeps_junction_priority() {
        let crossing = Car {
            lateral_speed: 6.,
            ..car(60., -12., 0.)
        };
        assert!(!clear(2., 2., 25., crossing, 12.));
        assert!(!oncoming_pass_clear(2., -2., 2., 25., &[crossing]));
    }
    #[test]
    fn queued_stop_preserves_room_to_launch_a_later_opposite_pass() {
        let parked = Car {
            half_width: 0.36,
            half_length: 1.05,
            ..car(11.6, 1., 0.)
        };
        let input = Request {
            lane: 1.,
            speed: 0.,
            desired: 1.,
            cruise: 40.,
            bounds: (0.65, 2.35),
        };
        let stopped = Memory::default().plan(input, &[parked], |_, _| true);
        assert!(stopped.speed < 0.01, "{stopped:?}");
        let released =
            Memory::default().plan_with_oncoming(input, &[parked], || (-2.35, 2.35), |_, _| true);
        assert!(released.lane < 0. && released.speed >= 1., "{released:?}");
    }
    #[test]
    fn clear_oncoming_corridor_can_pass_a_blocked_outbound_lane() {
        let cars = [car(60., 2., 10.)];
        let input = Request {
            lane: 2.,
            speed: 20.,
            desired: 2.,
            cruise: 35.,
            bounds: (0.65, 2.35),
        };
        let mut memory = Memory::default();
        let p = memory.plan_with_oncoming(input, &cars, || (-2.35, 2.35), |_, _| true);
        assert!(p.lane < 0. && p.speed >= 20., "{p:?}");
        let blocked = [cars[0], car(100., -2., -25.)];
        let returned = memory.plan_with_oncoming(
            Request {
                lane: p.lane,
                desired: -2.,
                ..input
            },
            &[car(-30., 2., 10.)],
            || (-2.35, 2.35),
            |_, _| true,
        );
        assert!(returned.lane >= 0.65, "{returned:?}");
        let p =
            Memory::default().plan_with_oncoming(input, &blocked, || (-2.35, 2.35), |_, _| true);
        assert!(p.lane > 0. && p.speed < 35., "{p:?}");
    }
    #[test]
    fn clear_racing_line_can_use_any_paved_lane_without_a_queue() {
        let input = Request {
            lane: 1.5,
            speed: 24.,
            desired: -1.5,
            cruise: 35.,
            bounds: (0.65, 2.35),
        };
        let chosen =
            Memory::default().plan_with_oncoming(input, &[], || (-2.35, 2.35), |_, _| true);
        assert!(chosen.lane < 0. && chosen.speed == 35., "{chosen:?}");

        let approaching = [car(100., -1.5, -25.)];
        let yielded = Memory::default().plan_with_oncoming(
            input,
            &approaching,
            || (-2.35, 2.35),
            |_, _| true,
        );
        assert!(yielded.lane >= 0.65, "{yielded:?}");
    }
    #[test]
    fn denied_inbound_line_uses_outbound_refuge_before_oncoming_arrival() {
        let input = Request {
            lane: 0.,
            speed: 25.,
            desired: 0.,
            cruise: 55.,
            bounds: (-2.35, -0.65),
        };
        // On this one-lane-per-direction road, clamping to -0.65 leaves the
        // bike inside the car's lateral clearance; -1.5 gives it room.
        let approaching = [car(300., 1.5, -15.)];
        assert!(!oncoming_pass_clear(0., 0., -0.65, 55., &approaching));
        let refuge = Memory::default().plan_with_oncoming(
            input,
            &approaching,
            || (-2.35, 2.35),
            |_, _| true,
        );
        assert!(refuge.lane <= -1.4 && refuge.speed > 0., "{refuge:?}");
    }
    #[test]
    fn outbound_refuge_never_overrides_an_occupied_merge_or_oncoming_brake() {
        let input = Request {
            lane: 0.,
            speed: 25.,
            desired: 0.,
            cruise: 55.,
            bounds: (-2.35, -0.65),
        };
        let blocked = [car(46., 1.5, -15.), car(-10., -1.5, 35.)];
        let yielded =
            Memory::default().plan_with_oncoming(input, &blocked, || (-2.35, 2.35), |_, _| true);
        assert!(
            yielded.speed < input.speed && yielded.lane > -1.4,
            "{yielded:?}"
        );
    }
}
