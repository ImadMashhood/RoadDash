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
        let reach = if self.role == Role::Killer {
            reach.max(4.)
        } else {
            reach
        };
        targets
            .iter()
            .copied()
            .filter(|t| {
                let gap = t.progress - me.progress;
                t.id != me.id
                    && self.can_fight(t.id)
                    && (-10.0..reach * 16.).contains(&gap)
                    && (t.lane - me.lane).abs() < 9.
                    && (!t.downed || (self.role == Role::Killer && (0.0..28.).contains(&gap)))
            })
            .min_by(|a, b| {
                (a.progress - me.progress)
                    .abs()
                    .total_cmp(&(b.progress - me.progress).abs())
                    .then(a.id.cmp(&b.id))
            })
    }
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
