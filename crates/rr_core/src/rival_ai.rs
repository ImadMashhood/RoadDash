//! Race-local intent: retaliation is keyed by racer ID, never shared personality columns.
use crate::pursuit::Target;
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Racer,
    Killer,
}
#[derive(Clone, Debug)]
pub struct Mind {
    pub role: Role,
    aggressors: BTreeMap<usize, f32>,
    target: Option<usize>,
    alongside: bool,
    side: f32,
    pub decision: Decision,
    pub desired_speed: f32,
}
impl Mind {
    pub fn new(id: usize) -> Self {
        Self {
            role: if crate::cast::character(id).is_some_and(|c| c.aggressive) {
                Role::Killer
            } else {
                Role::Racer
            },
            aggressors: BTreeMap::new(),
            target: None,
            alongside: false,
            side: 1.,
            decision: Decision::Race,
            desired_speed: 0.,
        }
    }
    pub fn struck_by(&mut self, attacker: usize) {
        self.aggressors.insert(attacker, 24.);
    }
    pub fn tick(&mut self, dt: f32) {
        self.aggressors.retain(|_, life| {
            *life -= dt.max(0.);
            *life > 0.
        });
    }
    pub fn target_id(&self) -> Option<usize> {
        self.target
    }
    pub fn can_fight(&self, target: usize) -> bool {
        self.role == Role::Killer || self.aggressors.contains_key(&target)
    }
    pub fn cooldown(&self, base: f32) -> f32 {
        if self.role == Role::Killer {
            (base * 0.6).max(0.75)
        } else {
            base
        }
    }
    pub fn choose(
        &self,
        elapsed: f32,
        me: Target,
        targets: &[Target],
        reach: f32,
    ) -> Option<Target> {
        // Keep bounded pursuit and launch grace, but do not inherit personality hostility.
        if elapsed < 8. {
            return None;
        }
        // A forward dependency cannot form a braking cycle. Rear combat is only
        // opportunistic within 10 road-space metres; it never dictates a slowdown.
        let eligible = |t: &&Target| {
            t.id != me.id && self.can_fight(t.id) && !t.downed && (t.lane - me.lane).abs() < 9.
        };
        let ahead = targets
            .iter()
            .filter(eligible)
            .filter(|t| {
                (t.progress > me.progress || (t.progress == me.progress && t.id < me.id))
                    && (self.role == Role::Killer || t.progress - me.progress < reach.max(4.) * 16.)
            })
            .min_by(|a, b| a.progress.total_cmp(&b.progress).then(a.id.cmp(&b.id)));
        let ahead = ahead.map(|nearest| {
            targets
                .iter()
                .find(|t| {
                    Some(t.id) == self.target
                        && eligible(t)
                        && (t.progress > me.progress || (t.progress == me.progress && t.id < me.id))
                        && t.progress <= nearest.progress + 0.75
                })
                .unwrap_or(nearest)
        });
        ahead
            .or_else(|| {
                targets
                    .iter()
                    .filter(eligible)
                    .filter(|t| distance(me, **t) <= 10.)
                    .min_by(|a, b| {
                        distance(me, **a)
                            .total_cmp(&distance(me, **b))
                            .then(a.id.cmp(&b.id))
                    })
            })
            .copied()
    }

    /// Intent uses road-space distance (station/lateral metres), after the adapter
    /// excludes different branches and non-riding/finished racers. Two metres of
    /// exit hysteresis and a fixed side prevent threshold/side chatter.
    pub fn drive(&mut self, me: Target, target: Option<Target>, cruise: f32) -> Option<(f32, f32)> {
        self.desired_speed = cruise;
        let Some(t) = target else {
            self.target = None;
            self.alongside = false;
            self.decision = Decision::Race;
            return None;
        };
        if self.target != Some(t.id) {
            self.target = Some(t.id);
            self.alongside = false;
            self.side = if me.lane > t.lane || (me.lane == t.lane && me.id.is_multiple_of(2)) {
                1.
            } else {
                -1.
            };
        }
        self.alongside = distance(me, t) <= if self.alongside { 12. } else { 10. };
        let gap = t.progress - me.progress;
        let lane = t.lane + self.side * 1.8;
        let speed = if gap < 0. || (gap == 0. && t.id > me.id) {
            self.decision = Decision::RearOpportunity;
            // The leading rider supplies race pace; its follower can match it.
            cruise
        } else if t.speed
            < if self.decision == Decision::PassSlow {
                7.
            } else {
                5.
            }
        {
            self.decision = Decision::PassSlow;
            // Standing combat is not a race goal. Let predictive passing find a
            // reachable lane at a safe speed rather than match a stopped target.
            cruise
        } else if self.alongside {
            self.decision = Decision::Alongside;
            (t.speed + gap * 0.5).min(cruise + 5.)
        } else {
            self.decision = Decision::Pursue;
            (t.speed + 5.).max(cruise).min(cruise + 5.)
        };
        self.desired_speed = speed;
        Some((lane, speed))
    }
}
/// Recorded intent before collision/curve limits; useful for decision traces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Race,
    Pursue,
    Alongside,
    RearOpportunity,
    PassSlow,
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
    fn t(id: usize) -> Target {
        Target {
            id,
            progress: 100. + id as f32,
            lane: 0.,
            speed: 30.,
            downed: false,
        }
    }
    #[test]
    fn race_focused_riders_return_to_racing_after_a_grudge_expires() {
        let mut mind = Mind::new(1);
        mind.struck_by(3);
        mind.tick(23.);
        assert!(mind.can_fight(3));
        mind.tick(1.1);
        assert!(!mind.can_fight(3));
        assert!(Mind::new(2).can_fight(3));
    }
    #[test]
    fn roster_has_seven_of_each_and_racers_never_start_fights() {
        let minds: Vec<_> = (1..15).map(Mind::new).collect();
        assert_eq!(minds.iter().filter(|m| m.role == Role::Killer).count(), 7);
        for m in minds.iter().filter(|m| m.role == Role::Racer) {
            assert!(m.choose(20., t(1), &[t(0), t(2), t(10)], 3.).is_none());
        }
    }
    #[test]
    fn retaliation_is_exact_identity_even_for_shared_personalities() {
        let mut m = Mind::new(1);
        m.struck_by(10);
        assert!(m.can_fight(10));
        for id in [0, 9, 11, 12, 14] {
            assert!(!m.can_fight(id));
        }
        assert!(m.choose(20., t(1), &[t(0), t(11)], 3.).is_none());
        assert_eq!(m.choose(20., t(1), &[t(10), t(11)], 3.).unwrap().id, 10);
        assert!(!Mind::new(1).can_fight(10));
    }
    #[test]
    fn killers_hunt_unprovoked_but_respect_launch_grace() {
        let m = Mind::new(2);
        assert!(m.choose(2., t(2), &[t(3)], 3.).is_none());
        assert_eq!(m.choose(20., t(2), &[t(3)], 3.).unwrap().id, 3);
        assert!(m.cooldown(1.8) < 1.8);
    }
    #[test]
    fn racers_pass_and_do_not_hunt_fallen_attackers() {
        let mut m = Mind::new(1);
        m.struck_by(3);
        assert!(
            m.choose(
                20.,
                t(1),
                &[Target {
                    downed: true,
                    ..t(3)
                }],
                3.
            )
            .is_none()
        );
        assert!(passing_lane(t(1), &[t(3)]).unwrap().abs() > 2.);
        assert!(passing_lane(t(3), &[t(1)]).is_none());
    }
}

#[cfg(test)]
mod followup_tests {
    use super::*;
    fn rider(id: usize, progress: f32, lane: f32, speed: f32) -> Target {
        Target {
            id,
            progress,
            lane,
            speed,
            downed: false,
        }
    }
    #[test]
    fn forward_target_wins_over_a_closer_rear_rider_and_downed_rider() {
        let m = Mind::new(2);
        let me = rider(2, 100., 0., 30.);
        let front = rider(3, 120., 0., 30.);
        let rear = rider(4, 99., 0., 10.);
        let down = Target {
            downed: true,
            ..rider(5, 102., 0., 0.)
        };
        assert_eq!(m.choose(20., me, &[rear, front, down], 4.).unwrap().id, 3);
        assert!(m.choose(20., me, &[rider(4, 89., 0., 30.)], 4.).is_none());
        assert!(m.choose(20., me, &[rider(4, 92., 8., 30.)], 4.).is_none());
    }
    #[test]
    fn equal_station_tie_has_one_racing_leader_and_far_ahead_is_pursued() {
        let mut a = Mind::new(2);
        let mut b = Mind::new(2);
        let first = rider(1, 100., 0., 20.);
        let second = rider(2, 100., 2., 20.);
        let target = a.choose(20., first, &[second], 4.);
        assert_eq!(a.drive(first, target, 45.).unwrap().1, 45.);
        assert_eq!(a.decision, Decision::RearOpportunity);
        let target = b.choose(20., second, &[first], 4.);
        assert_eq!(b.drive(second, target, 45.).unwrap().1, 20.);
        assert_eq!(b.decision, Decision::Alongside);
        assert_eq!(
            a.choose(20., first, &[rider(4, 1000., 0., 30.)], 4.)
                .unwrap()
                .id,
            4
        );
    }
    #[test]
    fn leader_never_brakes_for_rear_target_and_stopped_target_is_passed() {
        let mut m = Mind::new(2);
        let me = rider(2, 100., 0., 30.);
        for speed in [0., 2., 20., 40.] {
            assert_eq!(
                m.drive(me, Some(rider(4, 98., 2., speed)), 45.).unwrap().1,
                45.
            );
        }
        assert_eq!(
            m.drive(me, Some(rider(4, 105., 0., 0.)), 45.).unwrap().1,
            45.
        );
        assert_eq!(m.decision, Decision::PassSlow);
    }
    #[test]
    fn moving_combat_uses_ten_metre_entry_twelve_exit_and_stable_side() {
        let mut m = Mind::new(2);
        let mut me = rider(2, 100., 0., 30.);
        let mut front = rider(4, 111., 0., 30.);
        assert!(m.drive(me, Some(front), 45.).unwrap().1 >= 45.);
        assert_eq!(m.decision, Decision::Pursue);
        front.progress = 109.;
        let first = m.drive(me, Some(front), 45.).unwrap();
        assert_eq!(m.decision, Decision::Alongside);
        assert!((first.1 - 30.).abs() < 5.);
        me.lane = -0.01;
        front.progress = 110.5;
        assert_eq!(m.drive(me, Some(front), 45.).unwrap().0, first.0);
        assert_eq!(m.decision, Decision::Alongside);
        front.progress = 113.;
        m.drive(me, Some(front), 45.);
        assert_eq!(m.decision, Decision::Pursue);
    }
    #[test]
    fn close_target_order_noise_has_hysteresis_but_real_nearer_rider_wins() {
        let mut m = Mind::new(2);
        let me = rider(2, 100., 0., 30.);
        let held = rider(3, 110., 0., 30.);
        m.drive(me, Some(held), 45.);
        assert_eq!(
            m.choose(20., me, &[rider(4, 109.8, 0., 30.), held], 4.)
                .unwrap()
                .id,
            3
        );
        assert_eq!(
            m.choose(20., me, &[held, rider(4, 108., 0., 30.)], 4.)
                .unwrap()
                .id,
            4
        );
    }
    #[test]
    fn multi_killer_matching_chain_has_a_free_racing_leader() {
        let mut minds: Vec<_> = (0..4).map(|_| Mind::new(2)).collect();
        let mut riders: Vec<_> = (0..4)
            .map(|i| rider(i, 100. + i as f32 * 8., 0., 0.))
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
        for r in riders {
            assert!(
                r.progress > 700. && r.speed > 30.,
                "{} / {}",
                r.progress,
                r.speed
            );
        }
    }
}
