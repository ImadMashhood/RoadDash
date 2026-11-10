//! Independently authored held-weapon rules; no renderer or physics writes.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Weapon {
    Club,
    Chain,
    Whip,
    RollingPin,
    BaseballBat,
}
impl Weapon {
    pub const ALL: [Self; 5] = [
        Self::Club,
        Self::Chain,
        Self::Whip,
        Self::RollingPin,
        Self::BaseballBat,
    ];
    pub fn id(self) -> &'static str {
        match self {
            Self::Club => "club",
            Self::Chain => "chain",
            Self::Whip => "whip",
            Self::RollingPin => "rolling_pin",
            Self::BaseballBat => "baseball_bat",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Club => "CLUB",
            Self::Chain => "CHAIN",
            Self::Whip => "WHIP",
            Self::RollingPin => "ROLLING PIN",
            Self::BaseballBat => "BASEBALL BAT",
        }
    }
    /// Stable family choice independent of query order, with no live RNG.
    pub fn for_grid(seed: u64, race: u64, rider: usize) -> Self {
        let mixed = seed
            .wrapping_add(race.wrapping_mul(31))
            .wrapping_add((rider as u64).wrapping_mul(17));
        Self::ALL[(mixed % Self::ALL.len() as u64) as usize]
    }
}
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ability {
    pub source_contact: f32,
    pub cooldown: f32,
    pub stamina: f32,
    pub lateral_reach: f32,
    pub longitudinal_reach: f32,
    pub pickup_radius: f32,
    /// Direct-hit count remains one; the existing three-hit rule owns knockoffs.
    pub direct_hits: u8,
}
impl Ability {
    pub fn reaches(self, lateral: f32, longitudinal: f32, height: f32, side: f32) -> bool {
        (0.65..=self.lateral_reach).contains(&lateral.abs())
            && lateral * side > 0.0
            && longitudinal.abs() <= self.longitudinal_reach
            && height.abs() <= 0.9
    }
    pub fn can_start(self, stamina: f32) -> bool {
        stamina >= self.stamina
    }
}

/// Stable seeded selection, independent of entity/query order. No simulation RNG
/// is consumed while driving; the authority chooses one loadout per new grid.
pub fn armed_rivals(seed: u64, ids: impl IntoIterator<Item = usize>, count: usize) -> Vec<usize> {
    fn rank(mut x: u64) -> u64 {
        x = x.wrapping_add(0x9e3779b97f4a7c15);
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        x ^ (x >> 31)
    }
    let mut ids: Vec<_> = ids.into_iter().filter(|id| *id != 0).collect();
    ids.sort_unstable();
    ids.dedup();
    ids.sort_unstable_by_key(|id| (rank(seed ^ *id as u64), *id));
    ids.truncate(count);
    ids.sort_unstable();
    ids
}

#[cfg(test)]
mod grid_tests {
    use super::*;
    #[test]
    fn arsenal_ids_and_grid_families_are_stable_and_complete() {
        assert_eq!(
            Weapon::ALL.map(Weapon::id),
            ["club", "chain", "whip", "rolling_pin", "baseball_bat"]
        );
        let ids: std::collections::BTreeSet<_> = Weapon::ALL.map(Weapon::id).into_iter().collect();
        assert_eq!(ids.len(), 5);
        let families: std::collections::BTreeSet<_> = (1..15)
            .map(|id| Weapon::for_grid(123, 1, id).id())
            .collect();
        assert_eq!(families, ids);
        assert_ne!(Weapon::for_grid(123, 1, 1), Weapon::for_grid(123, 2, 1));
    }
    #[test]
    fn loadout_is_unique_bounded_reproducible_and_query_order_independent() {
        let ids = armed_rivals(123, 0..15, 7);
        assert_eq!(ids.len(), 7);
        assert!(!ids.contains(&0));
        assert_eq!(ids, armed_rivals(123, (0..15).rev(), 7));
        assert_eq!(ids, armed_rivals(123, (0..15).chain(0..15), 7));
        assert!(armed_rivals(123, 0..15, 0).is_empty());
        assert_eq!(armed_rivals(123, [0, 1, 1, 2], 99), vec![1, 2]);
        let seen: std::collections::BTreeSet<_> =
            (0..64).map(|seed| armed_rivals(seed, 1..15, 7)).collect();
        assert!(
            seen.len() > 32,
            "different seeds must vary the chosen riders"
        );
    }
}
