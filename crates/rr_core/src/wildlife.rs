//! Deterministic scenic herds: no gameplay contacts or road crossings.
use std::f32::consts::PI;
pub const HERD_COUNT: usize = 6;
pub const COUNTS: [usize; HERD_COUNT] = [8, 6, 10, 7, 9, 8];
pub const POPULATION: usize = 48;
pub const HALF_WIDTH: f32 = 25.;
pub const HALF_LENGTH: f32 = 42.;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    Horse,
    Bison,
}
pub fn species(herd: usize) -> Species {
    if herd < 3 {
        Species::Bison
    } else {
        Species::Horse
    }
}
/// Integrated positive variable speed: no accumulated error after pause/retry.
pub fn phase(herd: usize, elapsed: f32) -> f32 {
    let t = elapsed.max(0.);
    let rate = if species(herd) == Species::Horse {
        0.075
    } else {
        0.035
    };
    t * rate + 0.45 * (t * 0.045).sin() + herd as f32 * 1.37
}
/// Shared ellipse plus stable loose formation. Members retain their spacing
/// through turns; individual phase lag avoids mechanically identical strides.
pub fn local(herd: usize, member: usize, angle: f32) -> [f32; 2] {
    let x = ((member * 7 + herd * 3) % 5) as f32 * 3.8 - 7.6;
    let z = (member / 5) as f32 * 6.5 - 3.25 + (member % 3) as f32 * 1.2;
    [12. * angle.cos() + x, 25. * angle.sin() + z]
}
pub fn stride(herd: usize, member: usize, leg: usize, elapsed: f32) -> f32 {
    let a = phase(herd, elapsed);
    let cadence = if species(herd) == Species::Horse {
        58.
    } else {
        72.
    };
    (a * cadence + member as f32 * 0.83 + [0., PI, PI * 1.5, PI * 0.5][leg]).sin()
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::TAU;
    #[test]
    fn bounded_population_paths_and_coherent_separated_members() {
        assert_eq!(COUNTS.iter().sum::<usize>(), POPULATION);
        for (h, &count) in COUNTS.iter().enumerate() {
            for tick in 0..512 {
                let a = tick as f32 / 512. * TAU;
                for m in 0..count {
                    let p = local(h, m, a);
                    assert!(p[0].abs() + 2. < HALF_WIDTH && p[1].abs() + 2. < HALF_LENGTH);
                    for n in 0..m {
                        let q = local(h, n, a);
                        assert!((p[0] - q[0]).hypot(p[1] - q[1]) > 3.5);
                    }
                }
            }
        }
    }
    #[test]
    fn clock_is_continuous_bounded_and_restartable() {
        for h in 0..HERD_COUNT {
            for tick in 0..12000 {
                let t = tick as f32 / 60.;
                let da = phase(h, t + 1. / 60.) - phase(h, t);
                assert!(da > 0. && da < 0.002);
            }
            assert_eq!(phase(h, -1.), phase(h, 0.));
            assert_eq!(stride(h, 0, 0, 12.), stride(h, 0, 0, 12.));
        }
    }
}
