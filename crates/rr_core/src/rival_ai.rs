//! Race-first rival intent with short, incident-based combat responses.
use crate::pursuit::Target;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Response {
    Fight,
    Flee,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Race,
    Approach,
    NearbyStrike,
    Retaliate,
    Flee,
}

/// At launch forecast a small speed increment, not instant terminal velocity.
/// Actual speed, real obstacle velocities and the existing safety tests remain
/// in the planner. This is a forecast only; it does not limit engine throttle.
pub fn planning_pace(elapsed: f32, speed: f32, cruise: f32) -> f32 {
    if elapsed < 5. {
        cruise.min(speed.max(0.) + 0.5)
    } else {
        cruise
    }
}

/// Five authored aggressors spread through the fourteen-rider grid.
pub const FIGHTERS: [usize; 5] = [1, 4, 7, 10, 13];
pub fn is_fighter(id: usize) -> bool {
    FIGHTERS.contains(&id)
}

#[derive(Clone, Debug)]
pub struct Mind {
    id: usize,
    incident: u32,
    response: Option<(usize, Response, f32)>,
    target: Option<usize>,
    side: f32,
    exchange_age: f32,
    exchange_speed: f32,
    race_window: f32,
    lane_handoff: bool,
    pub decision: Decision,
    pub desired_speed: f32,
}

impl Mind {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            incident: 0,
            response: None,
            target: None,
            side: 1.,
            exchange_age: 0.,
            exchange_speed: 0.,
            race_window: 0.,
            lane_handoff: false,
            decision: Decision::Race,
            desired_speed: 0.,
        }
    }

    /// A reproducible fair coin per confirmed incident, independent of frame rate.
    pub fn struck_by(&mut self, attacker: usize) -> Response {
        self.incident = self.incident.wrapping_add(1);
        // SplitMix64 gives a reproducible, well-mixed bit for each incident.
        // This avoids a predictable fight/flee alternation in a repeated duel.
        let mut roll = (self.id as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ (attacker as u64).rotate_left(21)
            ^ (self.incident as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        roll = (roll ^ (roll >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        roll = (roll ^ (roll >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        let response = if !is_fighter(self.id) || (roll ^ (roll >> 31)) & 1 == 0 {
            Response::Fight
        } else {
            Response::Flee
        };
        self.response = Some((attacker, response, 8.));
        response
    }

    pub fn tick(&mut self, dt: f32) {
        self.race_window = (self.race_window - dt.max(0.)).max(0.);
        if self.exchange_age > 0. {
            self.exchange_age += dt.max(0.);
            if self.exchange_age >= 4. {
                self.finish_exchange();
            }
        }
        if let Some((_, _, life)) = &mut self.response {
            *life -= dt.max(0.);
            if *life <= 0. {
                self.response = None;
            }
        }
    }

    /// Confirmed combat incidents received since spawn/retry, including kicks.
    /// Diagnostic readers must not substitute this for contact validation.
    pub fn incident_count(&self) -> u32 {
        self.incident
    }

    pub fn response(&self) -> Option<Response> {
        self.response.map(|(_, response, _)| response)
    }

    pub fn target_id(&self) -> Option<usize> {
        self.target
    }

    /// A local pursuit needs available engine headroom, not merely a higher
    /// throttle goal behind the same lower cruising governor. No extra torque,
    /// position-based assistance or speed beyond the bike class is granted.
    pub fn motor_request(&self, bike_top: f32) -> f32 {
        if self.target.is_some() {
            bike_top
        } else {
            self.desired_speed
        }
    }

    /// Release a stale passing destination once when entering a nearby moving
    /// encounter. The adapter must run its complete safety planner again.
    pub fn take_lane_handoff(&mut self, me: Target, target: Option<Target>) -> bool {
        if !self.lane_handoff
            && target.is_some_and(|t| {
                self.target == Some(t.id)
                    && !t.downed
                    && t.speed >= 8.
                    && (t.progress - me.progress).abs() <= 8.
            })
        {
            self.lane_handoff = true;
            true
        } else {
            false
        }
    }

    /// The actual attack scheduler starts the bounded exchange, not an
    /// approximate road-space distance several bike lengths before contact.
    /// Repeated attacks never extend the same four-second window.
    pub fn strike_started(&mut self, speed: f32) {
        if self.target.is_some() && self.exchange_age == 0. {
            self.exchange_age = f32::EPSILON;
            self.exchange_speed = speed;
        }
    }

    pub fn finish_exchange(&mut self) {
        self.target = None;
        self.exchange_age = 0.;
        self.race_window = 2.;
        self.decision = Decision::Race;
    }

    pub fn interrupt_exchange(&mut self, speed: f32, target: Option<Target>) -> bool {
        const LIMIT: f32 = 40. / 3.6;
        if self.exchange_age > 0.
            && (speed <= LIMIT
                || self.exchange_speed - speed >= LIMIT - 1e-4
                || target.map(|t| t.id) != self.target)
        {
            self.finish_exchange();
            true
        } else {
            false
        }
    }

    pub fn can_fight(&self, target: usize) -> bool {
        if is_fighter(self.id) {
            !is_fighter(target) && self.response() != Some(Response::Flee)
        } else {
            self.response.is_some_and(|(attacker, response, _)| {
                attacker == target && response == Response::Fight
            })
        }
    }

    pub fn cooldown(&self, base: f32) -> f32 {
        base
    }

    /// Only nearby riders are combat targets. The motion planner independently
    /// passes slower traffic, so an absent target never makes a rider follow.
    pub fn choose(
        &self,
        elapsed: f32,
        me: Target,
        targets: &[Target],
        _reach: f32,
    ) -> Option<Target> {
        if elapsed < 8. || self.race_window > 0. {
            return None;
        }
        if self.response() == Some(Response::Flee) {
            return self.response.and_then(|(id, _, _)| {
                targets
                    .iter()
                    .find(|t| t.id == id && !t.downed && distance(me, **t) <= 10.)
                    .copied()
            });
        }
        // Keep a moving opponent through the overtake instead of dropping the
        // target exactly when the two riders finally reach striking distance.
        if let Some(id) = self.target
            && let Some(target) = targets.iter().find(|t| {
                t.id == id
                    && self.can_fight(id)
                    && !t.downed
                    && t.speed >= 8.
                    && (-8.0..40.0).contains(&(t.progress - me.progress))
                    && (t.lane - me.lane).abs() < 10.
            })
        {
            return Some(*target);
        }
        if is_fighter(self.id) {
            // The next nonfighter ahead is the only proactive target. A downed
            // rider is immediately ineligible, so the next place takes over.
            targets
                .iter()
                .filter(|t| {
                    !is_fighter(t.id)
                        && !t.downed
                        && t.speed >= 8.
                        && (0.0..30.0).contains(&(t.progress - me.progress))
                        && (t.lane - me.lane).abs() <= 4.5
                })
                .min_by(|a, b| a.progress.total_cmp(&b.progress).then(a.id.cmp(&b.id)))
                .copied()
        } else {
            // Nonfighters defend themselves against the confirmed attacker only.
            self.response.and_then(|(attacker, response, _)| {
                (response == Response::Fight).then(|| {
                    targets
                        .iter()
                        .find(|t| {
                            t.id == attacker
                                && !t.downed
                                && t.speed >= 8.
                                && distance(me, **t) <= 12.0
                        })
                        .copied()
                })?
            })
        }
    }

    /// Bounded moving exchanges use speed matching, then release back to racing.
    /// Slow/stopped targets never hold a queue of riders in a combat follow.
    pub fn drive(&mut self, me: Target, target: Option<Target>, cruise: f32) -> Option<(f32, f32)> {
        self.drive_with_spacing(me, target, cruise, 1.75)
    }

    pub fn drive_with_spacing(
        &mut self,
        me: Target,
        target: Option<Target>,
        cruise: f32,
        spacing: f32,
    ) -> Option<(f32, f32)> {
        self.desired_speed = cruise;
        if self.response() == Some(Response::Flee) {
            self.target = None;
            self.exchange_age = 0.;
            self.decision = Decision::Flee;
            self.desired_speed = cruise + 4.;
            let lane = target.map_or(me.lane, |attacker| {
                me.lane + if me.lane >= attacker.lane { 2.4 } else { -2.4 }
            });
            return Some((lane, self.desired_speed));
        }
        let Some(target) = target else {
            self.target = None;
            self.decision = Decision::Race;
            self.exchange_age = 0.;
            return None;
        };
        if target.speed < 8. {
            self.target = None;
            self.exchange_age = 0.;
            self.decision = Decision::Race;
            return None;
        }
        if self.target != Some(target.id) {
            self.exchange_age = 0.;
            self.lane_handoff = false;
            self.target = Some(target.id);
            self.side =
                if me.lane > target.lane || (me.lane == target.lane && me.id.is_multiple_of(2)) {
                    1.
                } else {
                    -1.
                };
        }
        let gap = target.progress - me.progress;
        self.desired_speed = if gap.abs() <= 6. {
            (target.speed + (gap * 1.2).clamp(-2., 3.)).max(12.)
        } else {
            cruise.max(target.speed + 3.)
        };
        // Do not spend the short fight window while still crossing the road or
        // shedding approach speed. Those are alignment, not a melee exchange.
        self.decision = if gap.abs() > 6.
            || (target.lane - me.lane).abs() > crate::DIRECT_STRIKE_MAX_LATERAL_M
            || (target.speed - me.speed).abs() > 6.
        {
            Decision::Approach
        } else if self.response().is_some_and(|r| r == Response::Fight) {
            Decision::Retaliate
        } else {
            Decision::NearbyStrike
        };
        Some((target.lane + self.side * spacing, self.desired_speed))
    }
}

/// Start an existing strike only if its contact frame is likely to meet the
/// opponent. This does not enlarge the actual combat collider or award a hit.
pub fn strike_window(
    me: Target,
    target: Target,
    windup: f32,
    longitudinal: f32,
    lateral: f32,
) -> bool {
    let gap = target.progress - me.progress;
    let relative = target.speed - me.speed;
    let travel = relative * windup.max(0.);
    !target.downed
        && relative.abs() <= 6.
        && (target.lane - me.lane).abs() >= 1.0
        && (target.lane - me.lane).abs() <= lateral
        && gap.abs() <= longitudinal + travel.abs().min(3.)
        && (gap + travel).abs() <= longitudinal
}

/// Express a candidate in the attacker-relative lateral frame at contact.
/// Velocities are [forward, lateral] metres/second from the physical bodies,
/// not unsigned display speeds. Crossing sides during windup invalidates a hit.
pub fn contact_frame(
    mut me: Target,
    mut target: Target,
    velocities: [[f32; 2]; 2],
    windup: f32,
) -> Option<(Target, Target)> {
    let side = target.lane - me.lane;
    target.lane += (velocities[1][1] - velocities[0][1]) * windup.max(0.);
    if side * (target.lane - me.lane) <= 0. {
        return None;
    }
    me.speed = velocities[0][0];
    target.speed = velocities[1][0];
    Some((me, target))
}

/// Predict the same horizontal volume used by real unarmed hit validation.
/// Height and line-of-sight still belong to the actual contact resolver.
pub fn direct_strike_window(me: Target, target: Target, windup: f32) -> bool {
    let lateral = target.lane - me.lane;
    let longitudinal = target.progress - me.progress + (target.speed - me.speed) * windup.max(0.);
    strike_window(
        me,
        target,
        windup,
        crate::DIRECT_STRIKE_MAX_LONGITUDINAL_M,
        crate::DIRECT_STRIKE_MAX_LATERAL_M,
    ) && crate::direct_strike_geometry(lateral, longitudinal, 0., lateral.signum())
}

fn distance(a: Target, b: Target) -> f32 {
    (a.progress - b.progress).hypot(a.lane - b.lane)
}
/// Pass the nearest slower rider without selecting them as a combat target.
pub fn passing_lane(me: Target, targets: &[Target]) -> Option<f32> {
    targets
        .iter()
        .filter(|t| {
            t.id != me.id
                && !t.downed
                && (0.0..24.).contains(&(t.progress - me.progress))
                && (t.lane - me.lane).abs() < 2.8
                && t.speed <= me.speed + 2.
        })
        .min_by(|a, b| a.progress.total_cmp(&b.progress).then(a.id.cmp(&b.id)))
        .map(|t| {
            t.lane
                + if me.lane > t.lane || (me.lane == t.lane && me.id.is_multiple_of(2)) {
                    2.4
                } else {
                    -2.4
                }
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn rider(id: usize, station: f32, lane: f32, speed: f32) -> Target {
        Target {
            id,
            progress: station,
            lane,
            speed,
            downed: false,
        }
    }
    #[test]
    fn speed_loss_or_floor_releases_an_exchange_and_blocks_immediate_retarget() {
        for (start, keep, release) in [(120., 81., 79.9), (65., 41., 40.)] {
            let mut mind = Mind::new(1);
            let me = rider(1, 100., 0., start / 3.6);
            let target = rider(2, 101., 1.7, 30.);
            mind.drive(me, Some(target), 42.);
            mind.strike_started(me.speed);
            assert!(!mind.interrupt_exchange(keep / 3.6, Some(target)));
            assert!(mind.interrupt_exchange(release / 3.6, Some(target)));
            assert!(mind.choose(20., me, &[target], 4.).is_none());
        }
    }

    #[test]
    fn five_fighters_select_the_next_nonfighter_ahead_and_clear_downed_targets() {
        assert_eq!(FIGHTERS.len(), 5);
        for id in 1..15 {
            let mut mind = Mind::new(id);
            let me = rider(id, 100., 0., 20.);
            let mut next = rider(30, 102., 1., 20.);
            let farther = rider(31, 110., -1., 20.);
            let fellow = rider(4, 101., 0., 20.);
            let chosen = mind.choose(10., me, &[farther, fellow, next], 4.);
            assert_eq!(chosen.map(|t| t.id), is_fighter(id).then_some(30));
            next.downed = true;
            assert_eq!(
                mind.choose(10., me, &[farther, fellow, next], 4.)
                    .map(|t| t.id),
                is_fighter(id).then_some(31)
            );
            assert!(mind.drive(me, None, 35.).is_none());
            assert_eq!(mind.desired_speed, 35.);
            assert_eq!(mind.can_fight(30), is_fighter(id));
        }
    }
    #[test]
    fn proactive_selection_prefers_the_next_reachable_lane_not_the_far_side_of_the_road() {
        let mind = Mind::new(1);
        let me = rider(1, 100., 0., 35.);
        let across_avenue = rider(2, 101., 8., 35.);
        let adjacent = rider(3, 106., 2., 35.);
        assert_eq!(
            mind.choose(20., me, &[across_avenue, adjacent], 4.)
                .unwrap()
                .id,
            3
        );
    }
    #[test]
    fn nonfighters_always_fight_back_and_fighters_keep_reproducible_incident_coin() {
        let mut defender = Mind::new(2);
        for _ in 0..100 {
            assert_eq!(defender.struck_by(1), Response::Fight);
            assert!(defender.can_fight(1));
            assert!(!defender.can_fight(3));
        }
        let mut mind = Mind::new(1);
        let mut fights = 0;
        let mut flees = 0;
        for _ in 0..1000 {
            match mind.struck_by(2) {
                Response::Fight => fights += 1,
                Response::Flee => flees += 1,
            }
        }
        assert!((450..=550).contains(&fights), "fight/flee {fights}/{flees}");
        assert_eq!(fights + flees, 1000);
        let mut replay = Mind::new(1);
        for _ in 0..1000 {
            replay.struck_by(2);
        }
        assert_eq!(replay.response(), mind.response());
    }
    #[test]
    fn flee_suppresses_contact_until_timeout_and_fight_retaliates_nearby() {
        let mut mind = Mind::new(1);
        let me = rider(1, 100., 0., 25.);
        let attacker = rider(2, 102., 1., 25.);
        while mind.struck_by(2) != Response::Flee {}
        assert!(!mind.can_fight(2));
        assert_eq!(mind.choose(12., me, &[attacker], 4.).unwrap().id, 2);
        let escape = mind.drive(me, Some(attacker), 36.).unwrap();
        assert!(escape.1 > 36. && (escape.0 - attacker.lane).abs() > 1.);
        mind.tick(8.1);
        assert!(mind.can_fight(2));
        while mind.struck_by(2) != Response::Fight {}
        assert_eq!(mind.choose(12., me, &[attacker], 4.).unwrap().id, 2);
        let pace = mind.drive(me, Some(attacker), 36.).unwrap().1;
        assert!((25. ..=28.).contains(&pace));
    }
    #[test]
    fn launch_forecast_holds_a_spaced_grid_but_still_detects_a_real_blocker() {
        use crate::traffic_avoidance::{Car, Memory, Request};
        let mut passing = Memory::default();
        for speed in [0., 5., 15., 25.] {
            let leader = Car {
                gap: 20.,
                lane: 2.,
                speed,
                lateral_speed: 0.,
                lateral_goal: None,
                half_width: 0.36,
                half_length: 1.05,
            };
            let input = Request {
                lane: 2.,
                speed,
                desired: 2.,
                cruise: planning_pace(2., speed, 55.),
                bounds: (-6., 6.),
            };
            let plan = passing.plan(input, &[leader], |_, _| true);
            assert!(!plan.avoiding && plan.lane == 2., "{plan:?}");
        }
        let blocker = Car {
            gap: 12.,
            lane: 2.,
            speed: 0.,
            lateral_speed: 0.,
            lateral_goal: None,
            half_width: 0.95,
            half_length: 1.95,
        };
        let plan = passing.plan(
            Request {
                lane: 2.,
                speed: 15.,
                desired: 2.,
                cruise: planning_pace(2., 15., 55.),
                bounds: (-6., 6.),
            },
            &[blocker],
            |_, _| true,
        );
        assert!(plan.avoiding);
        assert_eq!(planning_pace(6., 15., 55.), 55.);
    }
    #[test]
    fn moving_exchange_retains_a_passed_target_then_returns_to_racing() {
        let mut mind = Mind::new(1);
        let me = rider(1, 100., 0., 40.);
        let mut other = rider(2, 102., 1.8, 40.);
        assert!(mind.choose(7., me, &[other], 4.).is_none());
        mind.drive(me, Some(other), 42.);
        other.progress = 99.;
        assert_eq!(mind.choose(10., me, &[other], 4.).unwrap().id, 2);
        assert!(mind.drive(me, Some(other), 42.).unwrap().1 < 42.);
        mind.strike_started(me.speed);
        mind.tick(4.1);
        assert!(mind.choose(15., me, &[other], 4.).is_none());
        mind.tick(2.1);
        other.progress = 102.;
        other.downed = true;
        assert!(mind.choose(17., me, &[other], 4.).is_none());
        other.downed = false;
        other.speed = 0.;
        assert!(mind.choose(17., me, &[other], 4.).is_none());
        assert!(mind.drive(me, Some(other), 42.).is_none());
    }
    #[test]
    fn nearby_catchup_is_bounded_and_never_follows_a_stopped_opponent() {
        let mut mind = Mind::new(1);
        let me = rider(1, 100., 0., 40.);
        assert_eq!(
            mind.drive(me, Some(rider(2, 120., 1.8, 42.)), 40.)
                .unwrap()
                .1,
            45.
        );
        assert_eq!(mind.decision, Decision::Approach);
        assert_eq!(mind.motor_request(55.), 55.);
        assert!(
            mind.drive(me, Some(rider(2, 102., 1.8, 8.)), 40.)
                .unwrap()
                .1
                >= 12.
        );
        assert!(mind.drive(me, Some(rider(2, 102., 1.8, 0.)), 40.).is_none());
    }
    #[test]
    fn alignment_does_not_spend_exchange_time_and_moving_opponents_can_be_matched() {
        let mut mind = Mind::new(1);
        let mut me = rider(1, 100., -4., 25.);
        let mut other = rider(2, 102., 4., 25.);
        mind.drive(me, Some(other), 50.);
        assert_eq!(mind.decision, Decision::Approach);
        mind.tick(5.);
        assert_eq!(mind.choose(20., me, &[other], 4.).unwrap().id, 2);
        other.lane = -2.25;
        mind.drive(me, Some(other), 50.);
        assert_eq!(mind.decision, Decision::NearbyStrike);
        mind.strike_started(me.speed);
        mind.tick(2.);
        mind.strike_started(me.speed);
        other.lane = 4.;
        mind.drive(me, Some(other), 50.);
        assert_eq!(mind.decision, Decision::Approach);
        mind.tick(2.1);
        assert!(mind.choose(20., me, &[other], 4.).is_none());
        for speed in [10., 25., 40.] {
            let mut mind = Mind::new(1);
            me.speed = speed;
            other.speed = speed;
            other.progress = me.progress;
            let desired = mind.drive(me, Some(other), 50.).unwrap().1;
            assert!(desired >= 12. && (desired - speed).abs() <= 2.);
        }
    }
    #[test]
    fn moving_exchange_requests_one_safe_lane_replan_per_target() {
        let mut mind = Mind::new(1);
        let me = rider(1, 100., 0., 25.);
        let mut other = rider(2, 120., 2., 25.);
        mind.drive(me, Some(other), 40.);
        assert!(!mind.take_lane_handoff(me, Some(other)));
        other.progress = 104.;
        mind.drive(me, Some(other), 40.);
        assert!(mind.take_lane_handoff(me, Some(other)));
        assert!(!mind.take_lane_handoff(me, Some(other)));
        mind.drive(me, None, 40.);
        mind.drive(me, Some(other), 40.);
        assert!(mind.take_lane_handoff(me, Some(other)));
    }
    #[test]
    fn contact_prediction_checks_lateral_motion_and_uses_forward_not_display_speed() {
        let me = rider(1, 100., 0., 40.);
        let target = rider(2, 100.5, crate::DIRECT_STRIKE_MAX_LATERAL_M - 0.1, 44.);
        let (a, b) = contact_frame(me, target, [[35., 0.], [35., 1.]], 0.25).unwrap();
        assert!(!direct_strike_window(a, b, 0.25));
        let (a, b) = contact_frame(me, target, [[35., 0.], [35., -1.]], 0.25).unwrap();
        assert!(direct_strike_window(a, b, 0.25));
        assert!(
            contact_frame(me, rider(2, 100.5, 0.1, 35.), [[35., 0.], [35., -1.]], 0.25).is_none()
        );
    }
    #[test]
    fn unarmed_windup_admission_never_exceeds_the_actual_direct_hit_volume() {
        let me = rider(1, 100., 0., 35.);
        for lateral in [1.2, 1.75, 2.2] {
            for gap in [0., 0.8, 1.2] {
                let target = rider(2, me.progress + gap, lateral, me.speed);
                if direct_strike_window(me, target, 0.25) {
                    assert!(crate::direct_strike_geometry(
                        lateral,
                        gap,
                        0.,
                        lateral.signum()
                    ));
                }
            }
        }
        assert!(direct_strike_window(me, rider(2, 100.5, 1.75, 35.), 0.25));
        assert!(direct_strike_window(me, rider(2, 101.2, 2.2, 35.), 0.25));
        assert!(!direct_strike_window(me, rider(2, 101.5, 2.5, 35.), 0.25));
    }
    #[test]
    fn strike_admission_predicts_contact_instead_of_swinging_after_a_fast_pass() {
        let me = rider(1, 100., 0., 40.);
        assert!(strike_window(me, rider(2, 102., 1.8, 37.), 0.3, 1.6, 2.5));
        assert!(!strike_window(me, rider(2, 100., 1.8, 50.), 0.3, 1.6, 2.5));
        assert!(!strike_window(me, rider(2, 100., 4., 40.), 0.3, 1.6, 2.5));
        assert!(!strike_window(me, rider(2, 100., 0., 40.), 0.3, 1.6, 2.5));
        assert!(!strike_window(me, rider(2, 102., 1.8, 43.), 0.3, 1.6, 2.5));
    }
    #[test]
    fn close_chain_keeps_a_racing_leader() {
        let mut minds: Vec<_> = (0..4).map(Mind::new).collect();
        let mut riders: Vec<_> = (0..4)
            .map(|i| rider(i, 100. + i as f32 * 3., 0., 0.))
            .collect();
        for _ in 0..1800 {
            let previous = riders.clone();
            for (mind, me) in minds.iter_mut().zip(&mut riders) {
                let goal = mind.choose(20., *me, &previous, 4.);
                let desired = mind.drive(*me, goal, 40.).map_or(40., |x| x.1);
                me.speed += (desired - me.speed).clamp(-0.15, 0.1);
                me.progress += me.speed / 60.;
            }
        }
        assert!(riders.iter().all(|r| r.progress > 700. && r.speed > 30.));
    }
}
