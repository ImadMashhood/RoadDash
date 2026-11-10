//! Offline district palette contract; geometry and simulation remain independent.
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApproachStation {
    pub z: f32,
    pub weights: BTreeMap<String, f32>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Approach {
    pub max_lowrise_floors: u32,
    pub stations: Vec<ApproachStation>,
    pub pools: BTreeMap<String, Vec<String>>,
}
impl Approach {
    pub fn validate(&self) -> Result<(), String> {
        let roles = ["roadside", "workshop", "neighborhood"];
        if self.max_lowrise_floors != 5
            || self.pools.len() != roles.len()
            || roles.iter().any(|role| !self.pools.contains_key(*role))
            || self.pools.values().any(|pool| {
                pool.len() < 3
                    || pool.iter().enumerate().any(|(i, name)| {
                        !super::metropolis::FACADE_FAMILIES.contains(&name.as_str())
                            || name.starts_with("OFFC")
                            || pool[..i].contains(name)
                    })
            })
            || self.stations.len() < 3
            || self.stations.first().is_none_or(|s| s.z != -180.)
            || self.stations.last().is_none_or(|s| {
                s.z != 1800. || s.weights.len() != 1 || s.weights.get("district") != Some(&1.)
            })
            || self.stations.windows(2).any(|s| s[0].z >= s[1].z)
            || self.stations.iter().any(|s| {
                !s.z.is_finite()
                    || s.weights.iter().any(|(role, weight)| {
                        !(roles.contains(&role.as_str()) || role == "district")
                            || !weight.is_finite()
                            || !(0. ..=1.).contains(weight)
                    })
                    || (s.weights.values().sum::<f32>() - 1.).abs() > 0.00001
            })
        {
            return Err("invalid Metropolis approach palette".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_approach_authoring_rejects_broken_weights_order_and_families() {
        let palette: serde_json::Value = serde_json::from_str(include_str!(
            "../../../content/world/metropolis_facades.json"
        ))
        .unwrap();
        let a: Approach = serde_json::from_value(palette["approach"].clone()).unwrap();
        a.validate().unwrap();
        for i in 0..5 {
            let mut b = a.clone();
            match i {
                0 => b.stations[1].z = -200.,
                1 => *b.stations[1].weights.get_mut("roadside").unwrap() = 0.9,
                2 => b.pools.get_mut("roadside").unwrap()[0] = "OFFC1".into(),
                3 => b.pools.get_mut("roadside").unwrap()[0] = "missing".into(),
                _ => b.stations[1].z = f32::NAN,
            }
            assert!(b.validate().is_err());
        }
    }
}
