//! Pure rider relationships and bounded racing/following/fighting decisions.
use serde::{Deserialize, Serialize};
/// Arcade corner budgets for rival lookahead and the physical bike torque cap.
/// The slightly higher current-point budget lets a rider finish a turn without
/// dropping drive force earlier than the lookahead planner requested.
pub const RACE_LOOKAHEAD_LATERAL_ACCEL_MPS2: f32 = 14.0;
pub const RACE_CURRENT_LATERAL_ACCEL_MPS2: f32 = 16.0;
/// Rival differences stay inside the selected unboosted bike class. Race place
/// never enters this calculation: nearby combat uses ordinary engine drive.
pub fn rival_race_pace(profile: Profile, character_pace: f32, player_top_mps: f32) -> f32 {
    (profile.race_pace(player_top_mps) * character_pace).min(player_top_mps)
}
pub fn rival_drive_ceiling(
    profile: Profile,
    character_pace: f32,
    bike_top: f32,
    requested: f32,
) -> f32 {
    rival_race_pace(profile, character_pace, bike_top)
        .max(requested)
        .min(bike_top)
}
/// As in the reference speed controller, drive fully below the requested pace.
/// Braking, obstacles and corner limits independently gate this request.
pub fn rival_throttle_amount(elapsed: f32, speed: f32, target: f32) -> f32 {
    if elapsed < 5.0 || speed <= target {
        1.0
    } else {
        0.0
    }
}
pub fn race_corner_speed(curvature: f32, lateral_accel_mps2: f32) -> f32 {
    (lateral_accel_mps2 / curvature.abs().max(0.0001)).sqrt()
}
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
    /// Adapt original roster pacing to the modern player's bike class.
    /// Preserve relative rider ranking without making every rival slower than
    /// the player's ordinary (unboosted) top speed. No positional rubber-band.
    pub fn race_pace(self, player_top_mps: f32) -> f32 {
        player_top_mps * (0.94 + (self.cruise_units - 87.0) / 17.0 * 0.12).clamp(0.90, 1.08)
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

/// Rate-limited lane correction for rivals, normalized to the existing tire response.
/// Small lane errors request small banks instead of full-lock left/right chatter.
pub fn lane_steering(
    error: f32,
    lateral_speed: f32,
    speed: f32,
    turn_response: f32,
    current: f32,
    dt: f32,
) -> f32 {
    if [error, lateral_speed, speed, turn_response, current, dt]
        .iter()
        .any(|v| !v.is_finite())
        || dt <= 0.0
    {
        return 0.0;
    }
    let target = if error.abs() < 0.12 && lateral_speed.abs() < 0.2 {
        0.0
    } else {
        ((error * 1.2 - lateral_speed * 0.4)
            / ((1.8 + speed.max(0.0) * 0.085) * turn_response.max(0.1)))
        .clamp(-0.85, 0.85)
    };
    current + (target - current).clamp(-dt.min(0.1) * 1.8, dt.min(0.1) * 1.8)
}
#[cfg(test)]
mod steering_tests {
    use super::*;
    #[test]
    fn routine_corrections_stay_shallow_and_reversals_unwind_gradually() {
        let mut steer = 0.0;
        for _ in 0..60 {
            steer = lane_steering(0.5, 0.3, 30.0, 2.0, steer, 1.0 / 60.0);
        }
        assert!(steer > 0.0 && steer < 0.1);
        let reverse = lane_steering(-4.0, 1.0, 30.0, 2.0, 0.7, 1.0 / 60.0);
        assert!(reverse > 0.65 && reverse < 0.7);
        assert_eq!(lane_steering(0.05, 0.05, 20.0, 2.0, 0.0, 1.0 / 60.0), 0.0);
        assert_eq!(
            lane_steering(f32::NAN, 0.0, 20.0, 2.0, 0.0, 1.0 / 60.0),
            0.0
        );
    }
    #[test]
    fn lane_feedback_settles_without_repeated_full_bank() {
        for hz in [30, 60, 120] {
            let dt = 1.0 / hz as f32;
            let mut lane = -3.0;
            let mut lateral = 0.0;
            let mut steer = 0.0;
            for _ in 0..hz * 12 {
                let previous = steer;
                steer = lane_steering(-lane, lateral, 30.0, 2.0, steer, dt);
                assert!((steer - previous).abs() <= 1.8 * dt + 0.0001);
                lateral +=
                    (steer * (1.8 + 30.0 * 0.085) * 2.0 - lateral) * (1.0 - (-dt * 8.0).exp());
                lane += lateral * dt;
            }
            assert!(
                lane.abs() < 0.125 && steer.abs() < 0.03,
                "{hz}: {lane} {steer}"
            );
        }
    }
}

#[cfg(test)]
mod pace_tests {
    use super::*;
    #[test]
    fn roster_pace_spans_player_speed_without_rubber_banding() {
        let mut p = Profile {
            personality: 0,
            cruise_units: 87.0,
            minimum_units: 60.0,
            kick_roll: 0,
            reach: 1.0,
            strength: 4.0,
            stamina: 100.0,
            weight_lb: 150.0,
            relationships: [0.0; 10],
            hit_reaction: 0.0,
            calm_per_second: 0.0,
        };
        assert!((p.race_pace(55.0) - 51.7).abs() < 0.001);
        p.cruise_units = 104.0;
        assert!((p.race_pace(55.0) - 58.3).abs() < 0.001);
        assert_eq!(p.cruise_mps(), 104.0 * 0.44704);
    }
    #[test]
    fn racing_corner_budget_is_faster_but_still_bounded_on_tight_turns() {
        let old = (10.0_f32 / 0.02).sqrt();
        let revised = race_corner_speed(0.02, RACE_LOOKAHEAD_LATERAL_ACCEL_MPS2);
        assert!(revised > old && revised < 30.0);
        assert!(race_corner_speed(0.02, RACE_CURRENT_LATERAL_ACCEL_MPS2) > revised);
        assert!(race_corner_speed(0.0, RACE_LOOKAHEAD_LATERAL_ACCEL_MPS2).is_finite());
    }
    #[test]
    fn rivals_use_the_bike_class_ceiling_and_full_drive_without_rubber_banding() {
        let profile = Profile {
            cruise_units: 104.,
            minimum_units: 70.,
            personality: 0,
            kick_roll: 2,
            reach: 3.,
            strength: 4.,
            stamina: 300.,
            weight_lb: 180.,
            relationships: [0.; 10],
            hit_reaction: 0.,
            calm_per_second: 0.,
        };
        let top = 55.;
        assert!((rival_race_pace(profile, 1.2, top) - top).abs() < 0.001);
        assert!(rival_race_pace(profile, 0.8, top) < top);
        assert_eq!(rival_throttle_amount(0.0, 0.0, 2.0), 1.0);
        assert_eq!(rival_throttle_amount(4.99, 20.0, 21.0), 1.0);
        assert_eq!(rival_throttle_amount(50.0, 20.0, 21.0), 1.0);
        assert_eq!(rival_throttle_amount(50.0, 22.0, 21.0), 0.0);
        assert_eq!(rival_drive_ceiling(profile, 0.8, top, 70.), top);
    }
}
