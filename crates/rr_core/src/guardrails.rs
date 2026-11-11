//! Ordinary roadside barriers: signed horizontal turns and bounded recovery access.
//! Bridge containment is authored separately by the world adapter.
pub const SEGMENT_M: f32 = 8.;
pub const MAX_RUN_M: f32 = 96.;
pub const GAP_M: f32 = 16.;
pub const RAIL_CENTER_M: f32 = 1.05;
pub const RAIL_DEPTH_M: f32 = 0.32;
/// Lane sign follows the course convention: positive is rider-right.
/// Flattening before measuring yaw keeps hill grade out of turn handedness.
pub fn turn_side(before: [f32; 2], after: [f32; 2]) -> Option<i8> {
    let cross = before[1] * after[0] - before[0] * after[1];
    let dot = before[0] * after[0] + before[1] * after[1];
    let yaw = cross.atan2(dot);
    if yaw.abs() <= 0.12 {
        None
    } else {
        Some(if yaw < 0. { 1 } else { -1 })
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Run {
    pub start: f32,
    pub end: f32,
    pub side: i8,
}
/// Merge adjacent equal-side cells, then leave a full gap between bounded runs.
/// Small leftover tails stay open instead of creating tiny isolated barriers.
pub fn runs(cells: &[(f32, Option<i8>)]) -> Vec<Run> {
    let mut continuous: Vec<Run> = Vec::new();
    for &(start, side) in cells {
        let Some(side) = side else { continue };
        if let Some(last) = continuous.last_mut()
            && last.side == side
            && (last.end - start).abs() < 0.01
        {
            last.end += SEGMENT_M;
        } else {
            continuous.push(Run {
                start,
                end: start + SEGMENT_M,
                side,
            });
        }
    }
    let mut result = Vec::new();
    for run in continuous {
        let mut start = run.start;
        while run.end - start >= SEGMENT_M {
            let end = (start + MAX_RUN_M).min(run.end);
            result.push(Run {
                start,
                end,
                side: run.side,
            });
            start = end + GAP_M;
        }
    }
    result
}
/// Authored access routes for an automatic rider separated from the wreck by a
/// rail. At most two candidates, each with bounded four-metre station steps.
/// World adapters must ground and collision-sweep every segment before use.
pub fn reentry_candidates(
    runs: &[Run],
    start: [f32; 2],
    goal: [f32; 2],
    half: f32,
) -> Vec<Vec<[f32; 2]>> {
    let Some(run) = runs.iter().find(|r| {
        start[0] >= r.start
            && start[0] <= r.end
            && start[1] * r.side as f32 > half + 2.5
            && goal[1] * (r.side as f32) < half + 1.
    }) else {
        return vec![];
    };
    if (goal[0] - start[0]).abs() > MAX_RUN_M + GAP_M {
        return vec![];
    }
    let outside = start[1];
    let mut ends = [run.start - GAP_M * 0.5, run.end + GAP_M * 0.5];
    ends.sort_by(|a, b| (a - start[0]).abs().total_cmp(&(b - start[0]).abs()));
    ends.into_iter()
        .map(|access| {
            let mut route = Vec::new();
            let mut along = |from: f32, to: f32, lane: f32| {
                let steps = ((to - from).abs() / 4.).ceil() as usize;
                for i in 1..=steps {
                    route.push([from + (to - from) * i as f32 / steps as f32, lane]);
                }
            };
            along(start[0], access, outside);
            route.push([access, goal[1]]);
            let steps = ((goal[0] - access).abs() / 4.).ceil() as usize;
            for i in 1..=steps {
                route.push([
                    access + (goal[0] - access) * i as f32 / steps as f32,
                    goal[1],
                ]);
            }
            route
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn right_and_left_are_signed_and_straights_open() {
        assert_eq!(turn_side([0., 1.], [-0.3, 1.]), Some(1));
        assert_eq!(turn_side([0., 1.], [0.3, 1.]), Some(-1));
        assert_eq!(turn_side([0., 1.], [0.01, 1.]), None);
        assert_eq!(turn_side([0., 0.7], [-0.15, 0.5]), Some(1));
    }
    #[test]
    fn long_runs_have_full_reentry_gaps_and_never_join_across_side_changes() {
        let cells: Vec<_> = (0..80)
            .map(|i| (i as f32 * SEGMENT_M, Some(if i < 40 { 1 } else { -1 })))
            .collect();
        let result = runs(&cells);
        assert!(result.iter().all(|r| r.end - r.start <= MAX_RUN_M));
        for pair in result.windows(2).filter(|r| r[0].side == r[1].side) {
            assert!(pair[1].start - pair[0].end >= GAP_M);
        }
        assert!(result.iter().any(|r| r.side == 1));
        assert!(result.iter().any(|r| r.side == -1));
    }
}
