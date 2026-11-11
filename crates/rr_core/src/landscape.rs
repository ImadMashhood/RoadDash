//! Deterministic landform rules; no render or physics dependencies.
/// Low mesas and intervening washes beyond the protected western settlement.
/// The smooth shoulders are finite and preserve the first 205 metres exactly.
pub fn western_mesa(lateral: f32, longitudinal: f32) -> f32 {
    let smooth = |x: f32| {
        let t = x.clamp(0., 1.);
        t * t * (3. - 2. * t)
    };
    let outskirts = smooth((lateral - 205.) / 95.);
    let ridge = (0.5 + 0.5 * (longitudinal * 0.008 + lateral * 0.003).sin()).powf(0.65);
    outskirts * (18. + 55. * smooth((ridge - 0.25) / 0.55))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mesas_preserve_settlement_and_have_bounded_continuous_relief() {
        for z in 0..13000 {
            assert_eq!(western_mesa(205., z as f32), 0.);
            for x in [206., 240., 280., 320., 600.] {
                let h = western_mesa(x, z as f32);
                assert!((0. ..=73.).contains(&h));
                assert!((h - western_mesa(x + 0.01, z as f32)).abs() < 0.02);
            }
        }
    }
}
