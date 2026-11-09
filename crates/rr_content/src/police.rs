//! One resident patrol; all values are original prototype tuning.
pub fn from_json(json: &str) -> Result<rr_core::police::Rules, String> {
    let r: rr_core::police::Rules = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let values = [
        r.speed_limit,
        r.detection_radius,
        r.launch_grace,
        r.warning_seconds,
        r.lose_seconds,
        r.search_seconds,
        r.arrest_seconds,
        r.arrest_radius,
        r.arrest_speed,
        r.patrol_speed,
        r.patrol_fraction,
    ];
    if values.iter().any(|v| !v.is_finite() || *v <= 0.)
        || !(20.0..=40.0).contains(&r.speed_limit)
        || !(30.0..=120.0).contains(&r.detection_radius)
        || !(5.0..=30.0).contains(&r.launch_grace)
        || !(1.0..=5.0).contains(&r.warning_seconds)
        || !(2.0..=10.0).contains(&r.lose_seconds)
        || !(2.0..=15.0).contains(&r.search_seconds)
        || !(1.0..=5.0).contains(&r.arrest_seconds)
        || !(1.0..=2.0).contains(&r.arrest_radius)
        || !(0.1..=1.5).contains(&r.arrest_speed)
        || !(5.0..=r.speed_limit).contains(&r.patrol_speed)
        || !(0.01..=0.8).contains(&r.patrol_fraction)
    {
        return Err("police tuning outside bounded fair-policy envelope".into());
    }
    Ok(r)
}
pub fn builtin() -> rr_core::police::Rules {
    from_json(include_str!("../../../content/police.json")).expect("validated police tuning")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_unfair_and_unknown_authoring() {
        let source = include_str!("../../../content/police.json");
        assert!(from_json(source).is_ok());
        for (old, new) in [
            ("1.8", "8.0"),
            ("1.2", "20.0"),
            ("8.0", "0.0"),
            ("90.0", "900.0"),
        ] {
            assert!(from_json(&source.replace(old, new)).is_err());
        }
        assert!(from_json(&source.replace("speed_limit", "magic_speed")).is_err());
    }
}
