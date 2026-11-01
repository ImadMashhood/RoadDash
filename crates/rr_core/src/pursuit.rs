//! Pure rider relationships and bounded racing/following/fighting decisions.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub personality: usize,
    pub cruise_units: f32,
    pub minimum_units: f32,
    pub kick_roll: u32,
    pub reach: f32,
    pub strength: f32,
    pub stamina: f32,
    pub weight_lb: f32,
    pub relationships: [f32; 10],
    pub hit_reaction: f32,
    pub calm_per_second: f32,
}
impl Profile {
    pub fn cruise_mps(self) -> f32 {
        self.cruise_units * 0.44704
    }
    pub fn attack_cooldown(self) -> f32 {
        1.2 + (8.0 - self.strength) * 0.12
    }
}
#[derive(Clone, Copy)]
pub struct Target {
    pub id: usize,
    pub progress: f32,
    pub lane: f32,
    pub speed: f32,
    pub downed: bool,
}
pub fn personality(id: usize) -> usize {
    if id < 9 { id } else { 0 }
}
pub fn hostility(ratings: &[f32; 10], target: usize) -> f32 {
    ratings[personality(target)]
}
pub fn choose(
    id: usize,
    elapsed: f32,
    me: Target,
    targets: &[Target],
    ratings: &[f32; 10],
    reach: f32,
) -> Option<Target> {
    if elapsed < 8.0 {
        return None;
    }
    targets
        .iter()
        .copied()
        .filter(|t| {
            let gap = t.progress - me.progress;
            let score = hostility(ratings, t.id);
            t.id != id
                && (-10.0..reach * 16.0).contains(&gap)
                && (t.lane - me.lane).abs() < 9.0
                && score >= 16.0
                && (!t.downed || (score >= 32.0 && (0.0..28.0).contains(&gap)))
        })
        .min_by(|a, b| {
            let rank = |t: &Target| {
                if hostility(ratings, t.id) >= 32.0 {
                    2
                } else {
                    1
                }
            };
            rank(b)
                .cmp(&rank(a))
                .then_with(|| {
                    (a.progress - me.progress)
                        .abs()
                        .total_cmp(&(b.progress - me.progress).abs())
                })
                .then(a.id.cmp(&b.id))
        })
}
pub fn approach(me: Target, target: Target, cruise: f32) -> (f32, f32) {
    let gap = target.progress - me.progress;
    let side = if me.lane >= target.lane { 1.0 } else { -1.0 };
    let lane = target.lane + if target.downed { 0.0 } else { side * 1.8 };
    let speed = if target.downed {
        22.0
    } else {
        (target.speed + gap * 0.3).clamp(8.0, cruise + 5.0)
    };
    (lane.clamp(-5.5, 5.5), speed)
}
pub fn calm(ratings: &mut [f32; 10], profile: Profile, dt: f32) {
    for (value, base) in ratings.iter_mut().zip(profile.relationships) {
        *value +=
            (base - *value).clamp(-profile.calm_per_second * dt, profile.calm_per_second * dt);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relationships_distinguish_racing_following_and_hostile_downed_hunts() {
        let me = Target {
            id: 3,
            progress: 100.0,
            lane: 0.0,
            speed: 30.0,
            downed: false,
        };
        let t = Target {
            id: 0,
            progress: 102.0,
            lane: 2.0,
            speed: 30.0,
            downed: false,
        };
        let mut ratings = [12.0; 10];
        assert!(choose(3, 20.0, me, &[t], &ratings, 3.0).is_none());
        ratings[0] = 24.0;
        assert!(choose(3, 2.0, me, &[t], &ratings, 3.0).is_none());
        assert!(choose(3, 20.0, me, &[t], &ratings, 3.0).is_some());
        let down = Target { downed: true, ..t };
        assert!(choose(3, 20.0, me, &[down], &ratings, 3.0).is_none());
        ratings[0] = 32.0;
        assert!(choose(3, 20.0, me, &[down], &ratings, 3.0).is_some());
        let (lane, speed) = approach(me, t, 45.0);
        assert!((speed - 30.0).abs() < 1.0 && (lane - t.lane).abs() > 1.5);
    }
}
