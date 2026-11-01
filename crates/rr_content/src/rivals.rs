//! Validated normalized reference observations; no original media is loaded.
use rr_core::pursuit::Profile;
pub fn from_json(json: &str) -> Result<Vec<Profile>, String> {
    let profiles: Vec<Profile> = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if profiles.len() != 15
        || profiles.iter().enumerate().any(|(id, p)| {
            p.personality != rr_core::pursuit::personality(id)
                || !(50.0..=200.0).contains(&p.cruise_units)
                || !(0.0..=p.cruise_units).contains(&p.minimum_units)
                || !(1..=127).contains(&p.kick_roll)
                || !(1.0..=5.0).contains(&p.reach)
                || !(1.0..=10.0).contains(&p.strength)
                || !(500.0..=5000.0).contains(&p.stamina)
                || !(80.0..=300.0).contains(&p.weight_lb)
                || !p
                    .relationships
                    .iter()
                    .all(|v| v.is_finite() && v.abs() <= 1024.0)
                || !p.hit_reaction.is_finite()
                || p.hit_reaction.abs() > 100.0
                || !(0.0..=10.0).contains(&p.calm_per_second)
        })
    {
        return Err("invalid fifteen-rider personality profiles".into());
    }
    Ok(profiles)
}
pub fn builtin() -> Vec<Profile> {
    from_json(include_str!("../../../content/rival_profiles.json"))
        .expect("validated rival profiles")
}
pub fn profile(id: usize) -> Profile {
    static PROFILES: std::sync::OnceLock<Vec<Profile>> = std::sync::OnceLock::new();
    PROFILES.get_or_init(builtin)[id % 15]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn observed_riders_have_distinct_stats_and_relationships() {
        let p = builtin();
        assert_eq!(p[1].strength, 7.0);
        assert_eq!(p[2].strength, 5.0);
        assert!(p[2].cruise_mps() > p[8].cruise_mps());
        assert!(p[1].relationships[4] >= 32.0);
        assert!(p[1].relationships[0] < 32.0);
        assert!(p[3].hit_reaction > 0.0 && p[1].hit_reaction < 0.0);
        assert!(from_json("[]").is_err());
    }
}
