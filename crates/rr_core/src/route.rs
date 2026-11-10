//! Branch choice is local to each racer and locks after leaving the fork throat.
pub fn choice(current: usize, progress: f32, distances: [f32; 2], player: bool) -> usize {
    choice_between(current, progress, distances, player, 3320., 3500.)
}
/// A course-specific decision throat with the same bounded lane hysteresis.
pub fn choice_between(
    current: usize,
    progress: f32,
    distances: [f32; 2],
    player: bool,
    start: f32,
    end: f32,
) -> usize {
    if !player || !(start..=end).contains(&progress) {
        return current.min(1);
    }
    let other = 1 - current.min(1);
    if distances[other] + 1.5 < distances[current.min(1)] {
        other
    } else {
        current.min(1)
    }
}
/// Split branches are independent lanes; outside the fork the routes are shared.
pub fn same_corridor(a: usize, sa: f32, b: usize, sb: f32) -> bool {
    same_corridor_between(a, sa, b, sb, 3350., 5362.5)
}
pub fn same_corridor_between(a: usize, sa: f32, b: usize, sb: f32, split: f32, merge: f32) -> bool {
    a == b || !(split..merge).contains(&sa) || !(split..merge).contains(&sb)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn choice_has_hysteresis_and_locks_out_shortcuts() {
        assert_eq!(choice(0, 3400., [2., 1.], true), 0);
        assert_eq!(choice(0, 3400., [4., 1.], true), 1);
        assert_eq!(choice(0, 4000., [100., 0.], true), 0);
        assert_eq!(choice(1, 3400., [0., 5.], false), 1);
    }
}
