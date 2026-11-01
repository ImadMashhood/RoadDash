//! Deterministic, bounded arcade pursuit decisions. No rendering or physics state.
#[derive(Clone, Copy)]
pub struct Target {
    pub id: usize,
    pub progress: f32,
    pub lane: f32,
    pub speed: f32,
    pub downed: bool,
}
pub fn choose(id: usize, elapsed: f32, me: Target, targets: &[Target]) -> Option<Target> {
    if elapsed < 8.0 + (id % 4) as f32 || (elapsed + id as f32 * 1.7) % 12.0 > 8.0 {
        return None;
    }
    targets
        .iter()
        .copied()
        .filter(|t| {
            let gap = t.progress - me.progress;
            t.id != id
                && (-10.0..60.0).contains(&gap)
                && (t.lane - me.lane).abs() < 9.0
                && (!t.downed || ((0.0..28.0).contains(&gap) && id.is_multiple_of(3)))
        })
        .min_by(|a, b| {
            let score =
                |t: &Target| (t.progress - me.progress).abs() + (t.lane - me.lane).abs() * 2.0;
            score(a).total_cmp(&score(b)).then(a.id.cmp(&b.id))
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pursuit_has_start_grace_speed_matching_and_selective_downed_hunts() {
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
        assert!(choose(3, 2.0, me, &[t]).is_none());
        let (lane, speed) = approach(me, t, 45.0);
        assert!((speed - 30.0).abs() < 1.0);
        assert!((lane - t.lane).abs() > 1.5);
        let down = Target { downed: true, ..t };
        assert!(choose(3, 12.0, me, &[down]).is_some());
        assert!(choose(2, 12.0, me, &[down]).is_none());
        assert_eq!(approach(me, down, 45.0), (2.0, 22.0));
    }
}
