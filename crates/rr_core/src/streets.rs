//! Arcade American lane layouts within a fixed street reservation; metres and SI speeds.
use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub route: usize,
    pub start: f32,
    pub end: f32,
    pub outbound: u8,
    pub inbound: u8,
    pub lane_width: f32,
}
impl Section {
    pub fn lanes(&self) -> u8 {
        self.outbound + self.inbound
    }
    pub fn half_width(&self) -> f32 {
        self.lanes() as f32 * self.lane_width * 0.5
    }
    pub fn divider(&self) -> f32 {
        (self.outbound as f32 - self.inbound as f32) * self.lane_width * 0.5
    }
    fn center(&self, outgoing: bool, slot: usize) -> f32 {
        let count = if outgoing {
            self.outbound
        } else {
            self.inbound
        } as usize;
        self.divider()
            + if outgoing { -1. } else { 1. } * (slot.min(count - 1) as f32 + 0.5) * self.lane_width
    }
}
pub fn section(sections: &[Section], route: usize, s: f32) -> &Section {
    sections
        .iter()
        .find(|p| p.route == route && s >= p.start && s < p.end)
        .or_else(|| sections.iter().rev().find(|p| p.route == route))
        .expect("validated complete lane profile")
}
fn blend(sections: &[Section], route: usize, s: f32) -> (&Section, &Section, f32) {
    let current = section(sections, route, s.max(0.));
    let previous = sections
        .iter()
        .find(|p| p.route == route && (p.end - current.start).abs() < 0.01)
        .unwrap_or(current);
    let t = ((s - current.start) / 100.).clamp(0., 1.);
    (previous, current, t * t * (3. - 2. * t))
}
pub fn lane_center(sections: &[Section], route: usize, s: f32, outgoing: bool, slot: usize) -> f32 {
    let (a, b, t) = blend(sections, route, s);
    let x = a.center(outgoing, slot);
    x + (b.center(outgoing, slot) - x) * t
}
/// (direction divider, usable lane width, marked half-width) interpolate through100m tapers.
pub fn markings(sections: &[Section], route: usize, s: f32) -> (f32, f32, f32) {
    let (a, b, t) = blend(sections, route, s);
    (
        a.divider() + (b.divider() - a.divider()) * t,
        a.lane_width + (b.lane_width - a.lane_width) * t,
        a.half_width() + (b.half_width() - a.half_width()) * t,
    )
}
/// A dropping lane must follow its destination queue before its bodies overlap.
/// Compare both slots in one street frame, including the next 120 metres; this
/// works in either travel direction and does not depend on render-frame poses.
pub fn shares_lane_ahead(
    sections: &[Section],
    route: usize,
    station: f32,
    outgoing: bool,
    slot: usize,
    other_slot: usize,
) -> bool {
    let ahead = station + if outgoing { 120. } else { -120. };
    [station, ahead].into_iter().any(|s| {
        (lane_center(sections, route, s, outgoing, slot)
            - lane_center(sections, route, s, outgoing, other_slot))
        .abs()
            < 1.9
    })
}
pub fn following_speed(cruise: f32, gap: Option<(f32, f32)>) -> f32 {
    gap.map_or(cruise.abs(), |(gap, leader)| {
        if gap < 8. {
            0.
        } else {
            (leader.abs().powi(2) + 8. * (gap - 8.))
                .sqrt()
                .min(cruise.abs())
        }
    })
}
/// Independent of rendering/camera: every racer keeps approaching traffic physically active.
pub fn collision_active(nearest_racer_m: f32, active: bool) -> bool {
    nearest_racer_m < if active { 220. } else { 170. }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn profiles() -> Vec<Section> {
        vec![
            Section {
                route: 0,
                start: 0.,
                end: 1000.,
                outbound: 1,
                inbound: 1,
                lane_width: 3.,
            },
            Section {
                route: 0,
                start: 1000.,
                end: 2000.,
                outbound: 2,
                inbound: 1,
                lane_width: 2.8,
            },
            Section {
                route: 0,
                start: 2000.,
                end: 3000.,
                outbound: 2,
                inbound: 2,
                lane_width: 2.4,
            },
        ]
    }
    #[test]
    fn american_centres_fit_and_merges_are_continuous() {
        let p = profiles();
        assert_eq!(lane_center(&p, 0, 500., true, 0), -1.5);
        assert_eq!(lane_center(&p, 0, 500., false, 0), 1.5);
        for d in 0..2900 {
            for direction in [true, false] {
                for slot in 0..2 {
                    let a = lane_center(&p, 0, d as f32, direction, slot);
                    let b = lane_center(&p, 0, d as f32 + 0.1, direction, slot);
                    assert!((a - b).abs() < 0.02);
                    assert!(a.abs() + 0.95 < 5.);
                }
            }
        }
    }
    #[test]
    fn entry_merging_slots_share_queue_before_bodies_touch() {
        let p = vec![
            Section {
                route: 0,
                start: 0.,
                end: 2800.,
                outbound: 2,
                inbound: 2,
                lane_width: 3.,
            },
            Section {
                route: 0,
                start: 2800.,
                end: 3350.,
                outbound: 1,
                inbound: 1,
                lane_width: 3.,
            },
            Section {
                route: 0,
                start: 3350.,
                end: 12000.,
                outbound: 2,
                inbound: 1,
                lane_width: 3.,
            },
        ];
        assert!(!shares_lane_ahead(&p, 0, 2600., true, 0, 1));
        assert!(shares_lane_ahead(&p, 0, 2780., true, 0, 1));
        assert!(
            (lane_center(&p, 0, 2780., true, 0) - lane_center(&p, 0, 2780., true, 1)).abs() > 1.9
        );
        for s in 2900..3350 {
            for outgoing in [true, false] {
                for slot in 0..2 {
                    let center = lane_center(&p, 0, s as f32, outgoing, slot);
                    assert!(center.abs() + 0.95 < markings(&p, 0, s as f32).2);
                }
            }
        }
        // A side-by-side pair approaching the four-to-two taper must establish
        // longitudinal separation before the physical 1.9m bodies converge.
        let mut progress = [2740_f32; 2];
        let mut speed = [18_f32; 2];
        for _ in 0..1200 {
            let old = progress;
            let old_speed = speed;
            for car in 0..2 {
                let other = 1 - car;
                let gap = old[other] - old[car];
                let leader = shares_lane_ahead(&p, 0, old[car], true, car, other)
                    && (gap > 0.05 || (gap.abs() <= 0.05 && other < car));
                let desired =
                    following_speed(18., leader.then_some((gap.max(0.05), old_speed[other])));
                speed[car] = if desired < speed[car] {
                    (speed[car] - 6. / 60.).max(desired)
                } else {
                    (speed[car] + 2. / 60.).min(desired)
                };
                progress[car] += speed[car] / 60.;
            }
            let lateral = (lane_center(&p, 0, progress[0], true, 0)
                - lane_center(&p, 0, progress[1], true, 1))
            .abs();
            assert!(lateral >= 1.9 || (progress[0] - progress[1]).abs() >= 3.9);
        }
        assert!(progress[1] > 2900., "merging queue must keep moving");
        assert!(shares_lane_ahead(&p, 0, 3380., false, 0, 1));
    }
    #[test]
    fn traffic_collision_interest_has_margin_and_hysteresis() {
        assert!(collision_active(160., false));
        assert!(collision_active(200., true));
        assert!(!collision_active(200., false));
        assert!(!collision_active(230., true));
    }
    #[test]
    fn cars_brake_for_a_queue_and_resume() {
        assert_eq!(following_speed(18., Some((7., 0.))), 0.);
        assert!(following_speed(18., Some((12., 0.))) < 6.);
        assert_eq!(following_speed(18., None), 18.);
        let (mut gap, mut speed) = (60., 18.);
        for _ in 0..1200 {
            let target = following_speed(18., Some((gap, 0.)));
            speed = if target < speed {
                (speed - 6. / 60.).max(target)
            } else {
                (speed + 2. / 60.).min(target)
            };
            gap -= speed / 60.;
            assert!(gap > 3.8);
        }
        assert!(speed < 0.01);
    }
}
