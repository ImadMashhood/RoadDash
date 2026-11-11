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
    NearbyStrike,
    Retaliate,
    Flee,
}

#[derive(Clone, Debug)]
pub struct Mind {
    id: usize,
    incident: u32,
    response: Option<(usize, Response, f32)>,
    target: Option<usize>,
    side: f32,
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
        let response = if (roll ^ (roll >> 31)) & 1 == 0 {
            Response::Fight
        } else {
            Response::Flee
        };
        self.response = Some((attacker, response, 8.));
        response
    }

    pub fn tick(&mut self, dt: f32) {
        if let Some((_, _, life)) = &mut self.response {
            *life -= dt.max(0.);
            if *life <= 0. {
                self.response = None;
            }
        }
    }

    pub fn response(&self) -> Option<Response> {
        self.response.map(|(_, response, _)| response)
    }

    pub fn target_id(&self) -> Option<usize> {
        self.target
    }

    pub fn can_fight(&self, _target: usize) -> bool {
        self.response() != Some(Response::Flee)
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
        if elapsed < 8. {
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
        let attacker = self
            .response
            .and_then(|(id, response, _)| (response == Response::Fight).then_some(id));
        targets
            .iter()
            .filter(|t| {
                t.id != me.id
                    && !t.downed
                    && distance(me, **t) <= 4.0
                    && (t.progress >= me.progress - 2.0)
            })
            .min_by(|a, b| {
                let a_key = (Some(a.id) != attacker, distance(me, **a));
                let b_key = (Some(b.id) != attacker, distance(me, **b));
                a_key
                    .0
                    .cmp(&b_key.0)
                    .then(a_key.1.total_cmp(&b_key.1))
                    .then(a.id.cmp(&b.id))
            })
            .copied()
    }

    /// Contact intent changes lateral line only. No target can reduce race pace.
    pub fn drive(&mut self, me: Target, target: Option<Target>, cruise: f32) -> Option<(f32, f32)> {
        self.desired_speed = cruise;
        if self.response() == Some(Response::Flee) {
            self.target = None;
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
            return None;
        };
        if self.target != Some(target.id) {
            self.target = Some(target.id);
            self.side =
                if me.lane > target.lane || (me.lane == target.lane && me.id.is_multiple_of(2)) {
                    1.
                } else {
                    -1.
                };
        }
        self.decision = if self.response().is_some_and(|r| r == Response::Fight) {
            Decision::Retaliate
        } else {
            Decision::NearbyStrike
        };
        Some((target.lane + self.side * 1.8, cruise))
    }
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
    fn every_rider_races_at_own_pace_and_only_notices_close_targets() {
        for id in 1..15 {
            let mut mind = Mind::new(id);
            let me = rider(id, 100., 0., 20.);
            assert!(
                mind.choose(10., me, &[rider(30, 120., 0., 0.)], 4.)
                    .is_none()
            );
            assert!(mind.drive(me, None, 35.).is_none());
            assert_eq!(mind.desired_speed, 35.);
            let near = rider(30, 102., 1., 0.);
            let goal = mind.choose(10., me, &[near], 4.);
            assert_eq!(mind.drive(me, goal, 35.).unwrap().1, 35.);
            assert_eq!(mind.decision, Decision::NearbyStrike);
        }
    }
    #[test]
    fn confirmed_incidents_make_reproducible_fair_coin_choices() {
        let mut mind = Mind::new(2);
        let mut fights = 0;
        let mut flees = 0;
        for _ in 0..1000 {
            match mind.struck_by(3) {
                Response::Fight => fights += 1,
                Response::Flee => flees += 1,
            }
        }
        assert!((450..=550).contains(&fights), "fight/flee {fights}/{flees}");
        assert_eq!(fights + flees, 1000);
        let mut replay = Mind::new(2);
        for _ in 0..1000 {
            replay.struck_by(3);
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
        assert_eq!(mind.drive(me, Some(attacker), 36.).unwrap().1, 36.);
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
