//! Authoring validation for sampled scenic herd paths, independent of renderer.
/// Each sample includes world position and clearance from every road corridor.
/// Reject unsupported, excessively steep or road-intruding authored paths.
pub fn validate_path(points: &[[f32; 3]], road_clear: &[bool]) -> Result<(), String> {
    if points.len() < 32 || points.len() != road_clear.len() {
        return Err("wildlife path needs at least32 matching support/clearance samples".into());
    }
    for (i, p) in points.iter().enumerate() {
        if !p.iter().all(|v| v.is_finite()) || !road_clear[i] {
            return Err(format!("invalid wildlife support/road clearance at{i}"));
        }
        let q = points[(i + 1) % points.len()];
        let horizontal = (p[0] - q[0]).hypot(p[2] - q[2]);
        if horizontal <= 0.001 || (p[1] - q[1]).abs() / horizontal > 0.55 {
            return Err(format!("wildlife path too steep/degenerate at{i}"));
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_nonfinite_steep_and_road_intrusion() {
        let p: Vec<_> = (0..32)
            .map(|i| {
                let a = i as f32 / 32. * std::f32::consts::TAU;
                [a.cos() * 12., 0., a.sin() * 25.]
            })
            .collect();
        let mut road = vec![true; 32];
        assert!(validate_path(&p, &road).is_ok());
        road[8] = false;
        assert!(validate_path(&p, &road).is_err());
        road[8] = true;
        let mut bad = p.clone();
        bad[2][1] = 100.;
        assert!(validate_path(&bad, &road).is_err());
        bad[2][1] = f32::NAN;
        assert!(validate_path(&bad, &road).is_err());
    }
}
