//! Civilian overtaking decisions in a common road frame (metres, seconds).
//! Only motor vehicles enter this perception set. Upright bikers deliberately
//! do not veto an opposing-lane pass; the native solid collision policy remains.
#[derive(Clone, Copy, Debug)]
pub struct Vehicle {
    pub id: u64,
    pub station: f32,
    pub lane: f32,
    pub home: f32,
    pub speed: f32,
    pub cruise: f32,
    pub front: f32,
    pub rear: f32,
    pub half_width: f32,
    pub reserved: bool,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Follow,
    Pass,
    Abort,
    Merge,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Memory {
    pub phase: Phase,
    /// Continuous opposing-lane blend; never changes the authoritative station.
    pub blend: f32,
    target: Option<u64>,
    wait: f32,
    age: f32,
    cooldown: f32,
    observed_leader: Option<u64>,
    previous_leader_speed: f32,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Clearance {
    pub depart: bool,
    pub merge: bool,
    /// Authored forward pavement remaining; infinity for closed routes.
    pub forward_distance: f32,
}
impl Clearance {
    pub const OPEN: Self = Self {
        depart: true,
        merge: true,
        forward_distance: f32::INFINITY,
    };
}
#[derive(Clone, Copy, Debug)]
pub struct Intent {
    pub speed: f32,
    pub blend: f32,
}
impl Memory {
    /// Stable leader identity for opt-in diagnostics; not a mutable behavior hook.
    pub fn target(&self) -> Option<u64> {
        self.target
    }
    pub fn needs_clearance(&self) -> bool {
        self.reserved() || self.wait >= 0.9
    }
    pub fn reserved(&self) -> bool {
        self.phase != Phase::Follow || self.blend > 0.
    }
    /// Peers must share a road corridor, with lanes normalized to its divider.
    /// Stable fleet iteration publishes reservations before the next decision.
    pub fn step(
        &mut self,
        me: Vehicle,
        peers: &[Vehicle],
        opposite: f32,
        clearance: Clearance,
        speed_limit: f32,
        dt: f32,
    ) -> Intent {
        let dt = dt.clamp(0., 0.1);
        let direction = me.cruise.signum();
        let cruise = me.cruise.abs().min(speed_limit.max(0.));
        let gap = |v: &Vehicle| (v.station - me.station) * direction;
        let same = |v: &Vehicle| v.cruise.signum() == direction;
        let home = |v: &Vehicle| (v.lane - me.home).abs() < me.half_width + v.half_width + 0.4;
        let target = peers.iter().find(|v| Some(v.id) == self.target);
        let leader = peers
            .iter()
            .filter(|v| same(v) && home(v) && gap(v) > 0.)
            .min_by(|a, b| gap(a).total_cmp(&gap(b)));
        let room = |station: f32, horizon: f32| {
            peers.iter().all(|v| {
                if !same(v) || (v.home - me.home).abs() >= me.half_width + v.half_width + 0.4 {
                    return true;
                }
                let separation = (v.station + v.speed * horizon - station) * direction;
                if separation > 0. {
                    separation > me.front + v.rear + 8.
                } else {
                    -separation > me.rear + v.front + 8.
                }
            })
        };
        let clear = |horizon: f32| {
            peers.iter().all(|v| {
                let g = gap(v);
                if v.reserved && g.abs() < 300. {
                    return false;
                }
                if same(v) || g < -me.rear - v.front - 8. {
                    return true;
                }
                // Whole passing corridor: a large oncoming vehicle in either of
                // its home lanes remains a veto, including a moving lane change.
                g > (cruise + v.speed.abs().max(v.cruise.abs())) * horizon
                    + me.front
                    + v.front
                    + 12.
            })
        };
        self.cooldown = (self.cooldown - dt).max(0.);
        let mut speed = cruise;
        match self.phase {
            Phase::Follow => {
                let impeded = leader.filter(|v| gap(v) < 65. && cruise > v.speed.abs() + 2.);
                if let Some(lead) = impeded {
                    let accelerating = lead.speed.abs() > self.previous_leader_speed + 0.2 * dt;
                    self.wait = if self.observed_leader == Some(lead.id) && !accelerating {
                        self.wait + dt
                    } else {
                        0.
                    };
                    self.observed_leader = Some(lead.id);
                    self.previous_leader_speed = lead.speed.abs();
                } else {
                    self.wait = 0.;
                    self.observed_leader = None;
                }
                if let Some(lead) = impeded.filter(|v| {
                    clearance.depart
                        && self.cooldown == 0.
                        && gap(v) >= me.front + v.rear + 1.
                        && ((cruise > v.cruise.abs() + 2. && self.wait >= 1.) || self.wait >= 6.)
                }) {
                    let distance = gap(lead) + lead.front + me.rear + 10.;
                    // A moving slow class may accelerate back to its authored
                    // cruise. A same-class leader must first demonstrate six
                    // seconds without useful acceleration; predict a 1m/s
                    // recovery margin and keep revalidating during the pass.
                    let leader_pace = if cruise > lead.cruise.abs() + 2. {
                        lead.speed.abs().max(lead.cruise.abs())
                    } else {
                        lead.speed.abs() + 1.
                    };
                    let start_speed = me.speed.abs().min(lead.speed.abs());
                    let horizon = pass_horizon(distance, start_speed, cruise, leader_pace);
                    let moving_time = horizon - 3.;
                    let accelerating_time = moving_time.min(((cruise - start_speed) / 2.).max(0.));
                    let travel = start_speed * 3.
                        + start_speed * accelerating_time
                        + accelerating_time * accelerating_time
                        + cruise * (moving_time - accelerating_time);
                    let merge_station = me.station + direction * travel;
                    if horizon < 35.
                        && travel + me.front.max(me.rear) + 8. < clearance.forward_distance
                        && clear(horizon)
                        && room(merge_station, horizon)
                    {
                        self.phase = Phase::Pass;
                        self.target = Some(lead.id);
                        self.age = 0.;
                    }
                }
            }
            Phase::Pass => {
                self.age += dt;
                let complete = target.is_none_or(|v| gap(v) < -(me.rear + v.front + 8.));
                if complete && clearance.merge && room(me.station, 0.) {
                    self.phase = Phase::Merge;
                } else if !clearance.depart || !clear(5.) || self.age > 35. {
                    self.phase = if clearance.merge && room(me.station, 0.) {
                        Phase::Merge
                    } else {
                        Phase::Abort
                    };
                }
            }
            Phase::Abort => {
                if clearance.merge && room(me.station, 0.) {
                    self.phase = Phase::Merge;
                } else {
                    speed = target.map_or(0., |v| (v.speed.abs() - 4.).max(0.));
                }
            }
            Phase::Merge => {
                // Re-check before each lateral step. A new follower cannot be
                // swept through just because the merge began on the last tick.
                if !clearance.merge || !room(me.station, 0.) {
                    self.phase = Phase::Abort;
                }
            }
        }
        let previous_blend = self.blend;
        match self.phase {
            Phase::Pass => self.blend = (self.blend + dt / 3.).min(1.),
            Phase::Abort => {
                // Hold the established lateral clearance while falling behind.
                // Never increase opposing-lane exposure after a new obstruction.
                speed = target.map_or(0., |v| (v.speed.abs() - 4.).max(0.));
            }
            Phase::Merge => {
                self.blend = (self.blend - dt / 3.).max(0.);
            }
            Phase::Follow => self.blend = 0.,
        }
        let candidate = me.home + (opposite - me.home) * self.blend;
        for v in peers {
            let g = gap(v);
            let other_front = if same(v) { v.front } else { v.rear };
            let other_rear = if same(v) { v.rear } else { v.front };
            if g > -me.rear - other_front - 2.
                && g < me.front + other_rear + 2.
                && (candidate - v.lane).abs() < me.half_width + v.half_width + 0.4
                && (candidate - v.lane).abs() < (me.lane - v.lane).abs()
            {
                self.blend = previous_blend;
                speed = speed.min((v.speed.abs() - 2.).max(0.));
            }
        }
        if self.phase == Phase::Merge && self.blend == 0. {
            self.phase = Phase::Follow;
            self.target = None;
            self.wait = 0.;
            self.cooldown = 2.;
        }
        let lane = me.home + (opposite - me.home) * self.blend;
        for v in peers {
            let g = gap(v);
            if g > 0.
                && (v.lane - lane).abs().min((v.lane - me.lane).abs())
                    < me.half_width + v.half_width + 0.4
            {
                let lead_speed = if same(v) { v.speed.abs() } else { 0. };
                speed = speed.min(super::traffic::following_speed(
                    cruise,
                    g,
                    me.front,
                    if same(v) { v.rear } else { v.front },
                    lead_speed,
                ));
            }
        }
        Intent {
            speed: speed.min(cruise),
            blend: self.blend,
        }
    }
}

/// Conservative 2m/s² native acceleration and two complete three-second lane
/// transitions. Starting from rest must not use cruise-relative speed alone.
fn pass_horizon(distance: f32, start: f32, cruise: f32, leader: f32) -> f32 {
    let acceleration_time = ((cruise - start) / 2.).max(0.);
    let relative = start - leader;
    let distance = distance + (leader - start).max(0.) * 3.;
    let gained = relative * acceleration_time + acceleration_time * acceleration_time;
    let catch = if gained >= distance {
        (-relative + (relative * relative + 4. * distance).sqrt()) * 0.5
    } else {
        acceleration_time + (distance - gained) / (cruise - leader).max(1.)
    };
    catch + 6.
}

#[cfg(test)]
mod tests {
    use super::*;
    fn car(id: u64, station: f32, cruise: f32) -> Vehicle {
        Vehicle {
            id,
            station,
            lane: 3.,
            home: 3.,
            speed: cruise,
            cruise,
            front: 2.,
            rear: 2.,
            half_width: 0.95,
            reserved: false,
        }
    }
    #[test]
    fn predicted_pass_and_return_must_fit_authored_road() {
        for direction in [-1., 1.] {
            for available in [140., 500.] {
                let mut me = car(0, 0., 18. * direction);
                me.speed = 15. * direction;
                let lead = car(1, 30. * direction, 15. * direction);
                let mut memory = Memory::default();
                for _ in 0..70 {
                    memory.step(
                        me,
                        &[lead],
                        -3.,
                        Clearance {
                            forward_distance: available,
                            ..Clearance::OPEN
                        },
                        18.,
                        1. / 60.,
                    );
                }
                assert_eq!(memory.reserved(), available > 140.);
            }
        }
    }
    #[test]
    fn pass_requires_open_motor_vehicle_horizon_and_merge_room() {
        for direction in [-1., 1.] {
            for heavy in [6., 12.] {
                let mut me = car(0, 0., 18. * direction);
                let mut lead = car(1, 25. * direction, heavy * direction);
                lead.rear = if heavy == 12. { 13.4 } else { 3. };
                me.lane *= direction;
                me.home *= direction;
                lead.lane *= direction;
                lead.home *= direction;
                let mut memory = Memory::default();
                for _ in 0..70 {
                    memory.step(
                        me,
                        &[lead],
                        -me.home,
                        Clearance::OPEN,
                        me.cruise.abs(),
                        1. / 60.,
                    );
                }
                assert_eq!(memory.phase, Phase::Pass);
                let mut oncoming = car(2, 100. * direction, -18. * direction);
                oncoming.lane = -me.home;
                oncoming.home = -me.home;
                let mut blocked = Memory::default();
                for _ in 0..120 {
                    blocked.step(
                        me,
                        &[lead, oncoming],
                        -me.home,
                        Clearance::OPEN,
                        me.cruise.abs(),
                        1. / 60.,
                    );
                }
                assert_eq!(blocked.phase, Phase::Follow);
                memory.step(
                    me,
                    &[lead, oncoming],
                    -me.home,
                    Clearance::OPEN,
                    me.cruise.abs(),
                    1. / 60.,
                );
                assert_eq!(memory.phase, Phase::Merge);
                // Aborting behind the target can immediately reclaim the clear home lane.
                memory.step(
                    me,
                    &[lead],
                    -me.home,
                    Clearance::OPEN,
                    me.cruise.abs(),
                    1. / 60.,
                );
                assert_eq!(memory.phase, Phase::Merge);
            }
        }
    }
    #[test]
    fn partial_departure_returns_immediately_but_occupied_merge_holds_clearance() {
        let mut me = car(0, 0., 18.);
        let lead = car(1, 25., 6.);
        let mut incoming = car(2, 100., -18.);
        incoming.lane = -3.;
        incoming.home = -3.;
        let mut memory = Memory {
            phase: Phase::Pass,
            blend: 0.2,
            target: Some(1),
            ..Default::default()
        };
        me.lane = 3. - 6. * memory.blend;
        let old = memory.blend;
        memory.step(
            me,
            &[lead, incoming],
            -3.,
            Clearance::OPEN,
            me.cruise.abs(),
            1. / 60.,
        );
        assert_eq!(memory.phase, Phase::Merge);
        assert!(memory.blend < old);
        // Beside the tractor: cannot sweep back through its occupied envelope.
        let mut beside = lead;
        beside.station = 0.;
        memory.phase = Phase::Pass;
        memory.blend = 0.8;
        me.lane = 3. - 6. * memory.blend;
        let old = memory.blend;
        let intent = memory.step(
            me,
            &[beside, incoming],
            -3.,
            Clearance::OPEN,
            me.cruise.abs(),
            1. / 60.,
        );
        assert_eq!(memory.phase, Phase::Abort);
        assert_eq!(memory.blend, old);
        assert!(intent.speed < beside.speed);
        beside.station = 30.;
        memory.step(
            me,
            &[beside, incoming],
            -3.,
            Clearance::OPEN,
            me.cruise.abs(),
            1. / 60.,
        );
        assert_eq!(memory.phase, Phase::Merge);
        assert!(memory.blend < old);
    }
    #[test]
    fn zero_speed_yield_preserves_direction_and_can_return_from_pass() {
        for direction in [-1., 1.] {
            let mut me = car(0, 0., 18. * direction);
            me.speed = 0.;
            me.lane = -1.8;
            let lead = car(1, 30. * direction, 6. * direction);
            let mut memory = Memory {
                phase: Phase::Pass,
                blend: 0.8,
                target: Some(1),
                ..Default::default()
            };
            let intent = memory.step(
                me,
                &[lead],
                -3.,
                Clearance {
                    depart: false,
                    merge: true,
                    ..Clearance::OPEN
                },
                0.,
                1. / 60.,
            );
            assert_eq!(intent.speed, 0.);
            assert_eq!(memory.phase, Phase::Merge);
            assert!(memory.blend < 0.8);
        }
    }
    #[test]
    fn horizon_includes_acceleration_from_a_stopped_following_queue() {
        assert!(pass_horizon(22., 0., 18., 1.) > 11.);
        assert!(pass_horizon(40., 12., 18., 12.) > 13.);
        assert!(pass_horizon(40., 0., 18., 12.) > pass_horizon(40., 12., 18., 12.));
    }
    #[test]
    fn equal_cruise_launch_queue_does_not_start_a_spurious_overtake() {
        let mut me = car(0, 0., 18.);
        me.speed = 0.;
        let mut lead = car(1, 0.04, 18.);
        lead.speed = 0.;
        let mut memory = Memory::default();
        for _ in 0..600 {
            memory.step(me, &[lead], -3., Clearance::OPEN, 18., 1. / 60.);
        }
        assert_eq!(memory.phase, Phase::Follow);
        assert_eq!(memory.blend, 0.);
    }
    #[test]
    fn persistent_same_class_stall_can_be_passed_but_accelerating_leader_resets_delay() {
        let mut me = car(0, 0., 18.);
        me.speed = 0.;
        let mut lead = car(1, 25., 18.);
        lead.speed = 0.;
        let mut memory = Memory::default();
        for _ in 0..370 {
            memory.step(me, &[lead], -3., Clearance::OPEN, 18., 1. / 60.);
        }
        assert_eq!(memory.phase, Phase::Pass);
        let mut warming = Memory::default();
        for tick in 0..500 {
            lead.speed = if tick < 300 {
                0.
            } else {
                ((tick - 300) as f32 / 30.).min(8.)
            };
            warming.step(me, &[lead], -3., Clearance::OPEN, 18., 1. / 60.);
            assert_eq!(
                warming.phase,
                Phase::Follow,
                "accelerating queue is not a permanent stall"
            );
        }
    }
    #[test]
    fn reservation_blocks_competitor_until_merge_finishes() {
        let me = car(0, 0., 18.);
        let lead = car(1, 25., 6.);
        let mut passing = car(2, -15., 18.);
        passing.reserved = true;
        passing.lane = -3.;
        let mut memory = Memory::default();
        for _ in 0..120 {
            memory.step(
                me,
                &[lead, passing],
                -3.,
                Clearance::OPEN,
                me.cruise.abs(),
                1. / 60.,
            );
        }
        assert_eq!(memory.phase, Phase::Follow);
        memory.step(me, &[lead], -3., Clearance::OPEN, me.cruise.abs(), 1. / 60.);
        assert_eq!(memory.phase, Phase::Pass);
    }
}

#[cfg(test)]
mod motion_tests {
    use super::*;
    fn vehicle(id: u64, station: f32, cruise: f32, rear: f32) -> Vehicle {
        Vehicle {
            id,
            station,
            lane: 3.,
            home: 3.,
            speed: cruise,
            cruise,
            front: 2.,
            rear,
            half_width: 0.95,
            reserved: false,
        }
    }
    #[test]
    fn new_oncoming_vehicle_mid_pass_aborts_without_overlap_and_eventually_retries() {
        let mut fleet = [
            vehicle(0, 0., 18., 2.),
            vehicle(1, 30., 6., 3.),
            vehicle(2, 2000., -18., 2.),
        ];
        fleet[2].lane = -3.;
        fleet[2].home = -3.;
        let mut memories = [Memory::default(); 3];
        let mut aborted = false;
        let mut retried = false;
        for tick in 0..3600 {
            if tick == 230 {
                fleet[2].station = fleet[0].station + 100.;
            }
            let mut snapshot = fleet;
            for i in 0..3 {
                let peers: Vec<_> = snapshot
                    .iter()
                    .enumerate()
                    .filter_map(|(j, v)| (i != j).then_some(*v))
                    .collect();
                let plan = memories[i].step(
                    snapshot[i],
                    &peers,
                    -snapshot[i].home,
                    Clearance::OPEN,
                    snapshot[i].cruise.abs(),
                    1. / 60.,
                );
                snapshot[i].reserved = memories[i].reserved();
                let v = &mut fleet[i];
                let dir = v.cruise.signum();
                let speed = if plan.speed < v.speed.abs() {
                    (v.speed.abs() - 6. / 60.).max(plan.speed)
                } else {
                    (v.speed.abs() + 2. / 60.).min(plan.speed)
                };
                v.speed = dir * speed;
                v.station += v.speed / 60.;
                v.lane = v.home * (1. - 2. * plan.blend);
                v.reserved = snapshot[i].reserved;
            }
            aborted |= memories[0].phase == Phase::Abort;
            retried |= aborted && memories[0].phase == Phase::Pass;
            for a in 0..3 {
                for b in a + 1..3 {
                    assert!(
                        (fleet[a].lane - fleet[b].lane).abs() > 1.9
                            || (fleet[a].station - fleet[b].station).abs() > 5.,
                        "mid-pass overlap tick={tick}: {fleet:?}"
                    );
                }
            }
        }
        assert!(
            aborted && retried && fleet[0].station > fleet[1].station + 15.,
            "failed abort/retry progress: {fleet:?}"
        );
        assert_eq!(memories[0].phase, Phase::Follow);
    }
    #[test]
    fn two_followers_eventually_pass_tractor_and_long_trailer_without_overlap() {
        for heavy in [6., 12.] {
            for direction in [-1., 1.] {
                let mut fleet = [
                    vehicle(0, 0., 18., 2.),
                    vehicle(1, -30., 18., 2.),
                    vehicle(2, 30., heavy, if heavy == 12. { 13.4 } else { 3. }),
                ];
                fleet[2].half_width = 1.5;
                fleet[2].front = 3.5;
                for v in &mut fleet {
                    v.station *= direction;
                    v.speed *= direction;
                    v.cruise *= direction;
                }
                let mut memories = [Memory::default(); 3];
                let mut pass_ticks = [0; 2];
                for _ in 0..6000 {
                    let mut snapshot = fleet;
                    for i in 0..3 {
                        let peers: Vec<_> = snapshot
                            .iter()
                            .enumerate()
                            .filter_map(|(j, v)| (i != j).then_some(*v))
                            .collect();
                        let plan = memories[i].step(
                            snapshot[i],
                            &peers,
                            -3.,
                            Clearance::OPEN,
                            snapshot[i].cruise.abs(),
                            1. / 60.,
                        );
                        snapshot[i].reserved = memories[i].reserved();
                        let v = &mut fleet[i];
                        let speed = if plan.speed < v.speed.abs() {
                            (v.speed.abs() - 6. / 60.).max(plan.speed)
                        } else {
                            (v.speed.abs() + 2. / 60.).min(plan.speed)
                        };
                        v.speed = direction * speed;
                        v.station += v.speed / 60.;
                        v.lane = 3. - 6. * plan.blend;
                        v.reserved = snapshot[i].reserved;
                        if i < 2 && memories[i].phase == Phase::Pass {
                            pass_ticks[i] += 1;
                        }
                    }
                    assert!(
                        fleet.iter().filter(|v| v.reserved).count() <= 1,
                        "competing claims: {fleet:?}"
                    );
                    for a in 0..3 {
                        for b in a + 1..3 {
                            let (front, back) =
                                if (fleet[a].station - fleet[b].station) * direction > 0. {
                                    (fleet[a], fleet[b])
                                } else {
                                    (fleet[b], fleet[a])
                                };
                            assert!(
                                (front.lane - back.lane).abs()
                                    >= front.half_width + back.half_width
                                    || (front.station - back.station) * direction
                                        > front.rear + back.front,
                                "overlap: {fleet:?}"
                            );
                        }
                    }
                }
                for i in 0..2 {
                    assert!(
                        pass_ticks[i] > 0
                            && (fleet[i].station - fleet[2].station) * direction > 15.,
                        "queue never cleared: {fleet:?}, ticks={pass_ticks:?}"
                    );
                    assert_eq!(memories[i].phase, Phase::Follow);
                }
            }
        }
    }
}

#[cfg(test)]
mod benchmark {
    use super::*;
    #[test]
    #[ignore = "isolated release CPU microbenchmark; not a GPU/frame acceptance gate"]
    fn civilian_planner_cost() {
        let fleet: Vec<_> = (0..144)
            .map(|i| Vehicle {
                id: i,
                station: i as f32 * 35.,
                lane: if i % 2 == 0 { 3. } else { -3. },
                home: if i % 2 == 0 { 3. } else { -3. },
                speed: if i % 2 == 0 { 18. } else { -13. },
                cruise: if i % 2 == 0 { 18. } else { -13. },
                front: 2.,
                rear: 2.,
                half_width: 0.95,
                reserved: false,
            })
            .collect();
        let mut memories = vec![Memory::default(); fleet.len()];
        let mut peers = Vec::with_capacity(143);
        let mut windows = vec![];
        for _ in 0..20 {
            let start = std::time::Instant::now();
            for _ in 0..100 {
                for i in 0..fleet.len() {
                    peers.clear();
                    peers.extend(
                        fleet
                            .iter()
                            .enumerate()
                            .filter_map(|(j, v)| (i != j).then_some(*v)),
                    );
                    std::hint::black_box(memories[i].step(
                        fleet[i],
                        &peers,
                        -fleet[i].home,
                        Clearance::OPEN,
                        18.,
                        1. / 60.,
                    ));
                }
            }
            windows.push(start.elapsed().as_secs_f64() * 1e6 / 100.);
        }
        windows.sort_by(f64::total_cmp);
        println!(
            "civilian 144-vehicle fleet decision CPU microseconds/tick median={:.3} p95={:.3}; includes reusable perception copying, excludes native geometry sweeps/physics/rendering",
            windows[10], windows[18]
        );
    }
}
