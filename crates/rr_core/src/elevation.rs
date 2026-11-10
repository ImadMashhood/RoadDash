//! Smooth authored land profile. Callers validate ordered, finite knots at intake.
/// Requires at least two strictly ordered finite knots and a positive crossfall wavelength.
/// These are authoring intake preconditions; an empty knot slice panics.
pub fn height(knots: &[[f32; 2]], crossfall: [f32; 2], x: f32, z: f32) -> f32 {
    let mut y = if z <= knots[0][0] {
        knots[0][1]
    } else {
        knots[knots.len() - 1][1]
    };
    for pair in knots.windows(2) {
        let [[a, lo], [b, hi]] = [pair[0], pair[1]];
        if (a..=b).contains(&z) {
            let t = (z - a) / (b - a);
            let blend = t * t * t * (10. + t * (-15. + 6. * t));
            y = lo + (hi - lo) * blend;
            break;
        }
    }
    y + crossfall[0] * (x / crossfall[1]).sin()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smooth_crests_clamp_and_do_not_overshoot() {
        let knots = [[-100., 10.], [0., 20.], [100., 12.]];
        let h = |z| height(&knots, [0., 1.], 0., z);
        assert_eq!(h(-200.), 10.);
        assert_eq!(h(200.), 12.);
        assert_eq!(h(-50.), 15.);
        for z in -100..=100 {
            assert!((10. ..=20.).contains(&h(z as f32)));
        }
        for z in [-100., 0., 100.] {
            assert!((h(z + 0.1) - h(z - 0.1)).abs() < 0.0001);
        }
    }
}
