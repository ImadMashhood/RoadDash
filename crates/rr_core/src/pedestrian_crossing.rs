//! Bounded pedestrian crossing admission; host simulation owns decisions.
pub const MAX_ACTIVE: usize = 2;
pub const RELEASE_INTERVAL: f32 = 12.;
pub const WALK_SPEED: f32 = 1.35;
pub const REST_SECONDS: f32 = 45.;
/// Closest relative approach over the entire crossing, not just the first step.
pub fn conflicts(start: [f32; 2], end: [f32; 2], actor: [f32; 2], velocity: [f32; 2]) -> bool {
    if !start
        .into_iter()
        .chain(end)
        .chain(actor)
        .chain(velocity)
        .all(f32::is_finite)
    {
        return true;
    }
    let delta = [end[0] - start[0], end[1] - start[1]];
    let distance = delta[0].hypot(delta[1]);
    if distance < 1. {
        return true;
    }
    let duration = distance / WALK_SPEED;
    let relative = [
        velocity[0] - delta[0] / duration,
        velocity[1] - delta[1] / duration,
    ];
    let offset = [actor[0] - start[0], actor[1] - start[1]];
    let speed2 = relative[0] * relative[0] + relative[1] * relative[1];
    let t = if speed2 > 1e-6 {
        (-(offset[0] * relative[0] + offset[1] * relative[1]) / speed2).clamp(0., duration + 2.)
    } else {
        0.
    };
    (offset[0] + relative[0] * t).hypot(offset[1] + relative[1] * t) < 3.5
}
pub fn may_start(
    now: f32,
    ready_at: f32,
    next_release: f32,
    active: usize,
    nearby: bool,
    clear: bool,
) -> bool {
    now.is_finite()
        && now >= ready_at
        && now >= next_release
        && active < MAX_ACTIVE
        && nearby
        && clear
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn predicts_fast_approach_and_allows_a_departing_gap() {
        assert!(conflicts([-9., 0.], [9., 0.], [0., -100.], [0., 15.]));
        assert!(!conflicts([-9., 0.], [9., 0.], [0., -100.], [0., -15.]));
        assert!(!conflicts([-9., 0.], [9., 0.], [30., 0.], [0., 0.]));
        assert!(conflicts([-9., 0.], [9., 0.], [f32::NAN, 0.], [0., 0.]));
    }
    #[test]
    fn pace_and_active_cap_prevent_a_pedestrian_stream() {
        assert!(may_start(20., 10., 12., 1, true, true));
        for (n, next, near, clear) in [
            (2, 12., true, true),
            (0, 24., true, true),
            (0, 12., false, true),
            (0, 12., true, false),
        ] {
            assert!(!may_start(20., 10., next, n, near, clear));
        }
        assert!(!may_start(f32::NAN, 0., 0., 0, true, true));
    }
}
