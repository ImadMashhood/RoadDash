//! Structural world authoring and quota allocation, independent of Bevy/physics.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum District {
    Farms,
    RuralEdge,
    Suburban,
    InnerCity,
    CityCore,
    CityToHills,
    Hills,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Family {
    Farm,
    Residential,
    Commercial,
    Urban,
    Hill,
}
impl Family {
    pub const ALL: [Self; 5] = [
        Self::Farm,
        Self::Residential,
        Self::Commercial,
        Self::Urban,
        Self::Hill,
    ];
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct FamilyCounts {
    pub farm: u32,
    pub residential: u32,
    pub commercial: u32,
    pub urban: u32,
    pub hill: u32,
}
impl FamilyCounts {
    pub fn get(&self, family: Family) -> u32 {
        match family {
            Family::Farm => self.farm,
            Family::Residential => self.residential,
            Family::Commercial => self.commercial,
            Family::Urban => self.urban,
            Family::Hill => self.hill,
        }
    }
    fn total(&self) -> u64 {
        Family::ALL.iter().map(|f| u64::from(self.get(*f))).sum()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub id: String,
    pub name: String,
    pub district: District,
    pub start_m: f32,
    pub end_m: f32,
    pub near: u32,
    pub secondary: u32,
    pub farm: u32,
    pub residential: u32,
    pub commercial: u32,
    pub urban: u32,
    pub hill: u32,
    pub character: String,
    pub lot_rules_id: String,
    pub total: u32,
}
impl Section {
    pub fn counts(&self) -> FamilyCounts {
        FamilyCounts {
            farm: self.farm,
            residential: self.residential,
            commercial: self.commercial,
            urban: self.urban,
            hill: self.hill,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorldDef {
    pub schema_version: u32,
    pub generator_version: u32,
    pub track_id: String,
    pub seed: u32,
    pub road_length_m: f32,
    pub length_tolerance_m: f32,
    pub transition_blend_m: f32,
    pub building_target_total: u32,
    pub near_road: u32,
    pub secondary_row: u32,
    pub archetype_counts: FamilyCounts,
    pub sections: Vec<Section>,
    pub lot_rules: Vec<LotRules>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Archetype {
    pub id: String,
    pub name: String,
    pub quantity: u32,
    pub family: Family,
    pub height_min_m: f32,
    pub height_max_m: f32,
    pub brief: String,
    pub footprint_width_m: DimensionRange,
    pub footprint_depth_m: DimensionRange,
    pub floor_height_m: f32,
    pub roof: Roof,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArchetypeCatalog {
    pub schema_version: u32,
    pub archetypes: Vec<Archetype>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Roof {
    Flat,
    Pitched,
    Sawtooth,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionRange {
    pub min_m: f32,
    pub max_m: f32,
}
impl DimensionRange {
    fn valid(&self, minimum: f32) -> bool {
        self.min_m.is_finite()
            && self.max_m.is_finite()
            && self.min_m >= minimum
            && self.max_m >= self.min_m
            && self.max_m <= 200.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LotRules {
    pub id: String,
    pub frontage_m: DimensionRange,
    pub depth_m: DimensionRange,
    pub setback_m: DimensionRange,
    pub pad_max_raw_relief_m: f32,
    pub earthwork_margin_m: f32,
    pub access_width_m: f32,
    pub secondary_access_width_m: f32,
}

impl WorldDef {
    pub fn from_json(world: &str, archetypes: &str) -> Result<(Self, ArchetypeCatalog), String> {
        let world: Self = serde_json::from_str(world).map_err(|e| format!("world JSON: {e}"))?;
        let catalog =
            serde_json::from_str(archetypes).map_err(|e| format!("archetypes JSON: {e}"))?;
        world.validate(&catalog)?;
        Ok((world, catalog))
    }
    pub fn validate(&self, catalog: &ArchetypeCatalog) -> Result<(), String> {
        if self.schema_version != 1 || self.generator_version != 1 || catalog.schema_version != 1 {
            return Err("unsupported world/archetype schema or generator version".into());
        }
        rr_core::ContentId::try_from(self.track_id.clone())
            .map_err(|e| format!("world track ID: {e}"))?;
        if !self.road_length_m.is_finite()
            || !(100.0..=20000.0).contains(&self.road_length_m)
            || !self.length_tolerance_m.is_finite()
            || !(0.001..=1.0).contains(&self.length_tolerance_m)
            || !self.transition_blend_m.is_finite()
            || !(0.0..=100.0).contains(&self.transition_blend_m)
            || self.sections.is_empty()
            || self.sections.len() > 128
            || self.building_target_total == 0
            || self.building_target_total > 100000
            || catalog.archetypes.is_empty()
            || catalog.archetypes.len() > 128
        {
            return Err("invalid world length, tolerance, blend, or bounded counts".into());
        }
        let mut rules = HashSet::new();
        if self.lot_rules.is_empty() || self.lot_rules.len() > 128 {
            return Err("missing/unbounded lot rules".into());
        }
        for rule in &self.lot_rules {
            if !valid_id(&rule.id)
                || !rules.insert(rule.id.as_str())
                || !rule.frontage_m.valid(4.0)
                || !rule.depth_m.valid(4.0)
                || !rule.setback_m.valid(0.0)
                || !rule.pad_max_raw_relief_m.is_finite()
                || !(0.1..=10.0).contains(&rule.pad_max_raw_relief_m)
                || !rule.earthwork_margin_m.is_finite()
                || !(1.0..=20.0).contains(&rule.earthwork_margin_m)
                || !rule.access_width_m.is_finite()
                || !(2.0..=20.0).contains(&rule.access_width_m)
                || !rule.secondary_access_width_m.is_finite()
                || !(2.0..=20.0).contains(&rule.secondary_access_width_m)
            {
                return Err(format!("{}: invalid/duplicate lot rules", rule.id));
            }
        }
        let mut ids = HashSet::new();
        let mut end = 0.0;
        for section in &self.sections {
            if !rules.contains(section.lot_rules_id.as_str()) {
                return Err(format!(
                    "{}: unknown lot rules {}",
                    section.id, section.lot_rules_id
                ));
            }
            if !ids.insert(&section.id) || !valid_id(&section.id) {
                return Err(format!("invalid/duplicate section ID {}", section.id));
            }
            if !section.start_m.is_finite()
                || !section.end_m.is_finite()
                || section.start_m != end
                || section.end_m <= section.start_m
                || section.end_m - section.start_m < self.transition_blend_m * 2.0
            {
                return Err(format!(
                    "{}: nonfinite, gap/overlap, reversed or too-short station span",
                    section.id
                ));
            }
            if u64::from(section.near) + u64::from(section.secondary) != u64::from(section.total)
                || section.counts().total() != u64::from(section.total)
            {
                return Err(format!(
                    "{}: row/family counts do not equal section total",
                    section.id
                ));
            }
            end = section.end_m;
        }
        if end != self.road_length_m {
            return Err("last section endpoint must equal authored world length".into());
        }
        let near: u64 = self.sections.iter().map(|s| u64::from(s.near)).sum();
        let secondary: u64 = self.sections.iter().map(|s| u64::from(s.secondary)).sum();
        if near != u64::from(self.near_road)
            || secondary != u64::from(self.secondary_row)
            || near + secondary != u64::from(self.building_target_total)
            || self.archetype_counts.total() != near + secondary
        {
            return Err("world row/family/total budget mismatch".into());
        }
        let mut archetype_ids = HashSet::new();
        for a in &catalog.archetypes {
            if !a.footprint_width_m.valid(2.0)
                || !a.footprint_depth_m.valid(2.0)
                || !a.floor_height_m.is_finite()
                || !(2.0..=8.0).contains(&a.floor_height_m)
            {
                return Err(format!("{}: invalid footprint/floor rules", a.id));
            }
            if !archetype_ids.insert(&a.id)
                || !valid_id(&a.id)
                || a.quantity == 0
                || a.quantity > self.building_target_total
                || !a.height_min_m.is_finite()
                || !a.height_max_m.is_finite()
                || !(1.0..=100.0).contains(&a.height_min_m)
                || a.height_max_m < a.height_min_m
                || a.height_max_m > 100.0
            {
                return Err(format!(
                    "{}: invalid/duplicate archetype or height/quantity",
                    a.id
                ));
            }
        }
        for family in Family::ALL {
            let sections: u64 = self
                .sections
                .iter()
                .map(|s| u64::from(s.counts().get(family)))
                .sum();
            let registry: u64 = catalog
                .archetypes
                .iter()
                .filter(|a| a.family == family)
                .map(|a| u64::from(a.quantity))
                .sum();
            if sections != u64::from(self.archetype_counts.get(family)) || registry != sections {
                return Err(format!(
                    "{family:?}: section/registry/family budget mismatch"
                ));
            }
        }
        Ok(())
    }
    /// Right-continuous lookup; final endpoint belongs to the last section.
    pub fn section_at(&self, station: f32) -> Option<usize> {
        if self.sections.is_empty()
            || !station.is_finite()
            || station < 0.0
            || station > self.road_length_m
        {
            return None;
        }
        Some(
            self.sections
                .partition_point(|s| s.end_m <= station)
                .min(self.sections.len().saturating_sub(1)),
        )
    }
    /// Blending changes presentation weights only, never a quota's section owner.
    pub fn district_blend(&self, station: f32) -> Option<(District, District, f32)> {
        let i = self.section_at(station)?;
        let section = &self.sections[i];
        let width = self.transition_blend_m;
        if width > 0.0 {
            for boundary in [i, i + 1] {
                if boundary == 0 || boundary >= self.sections.len() {
                    continue;
                }
                let distance = station - self.sections[boundary].start_m;
                if distance.abs() <= width {
                    let t = (distance + width) / (width * 2.0);
                    return Some((
                        self.sections[boundary - 1].district,
                        self.sections[boundary].district,
                        t * t * (3.0 - 2.0 * t),
                    ));
                }
            }
        }
        Some((section.district, section.district, 0.0))
    }
    /// B01 quota allocation, NOT placed building geometry or a WorldLayout.
    pub fn allocate(&self, catalog: &ArchetypeCatalog) -> Result<Vec<BuildingAssignment>, String> {
        self.validate(catalog)?;
        let mut random = u64::from(self.seed) + 1;
        let mut pools = BTreeMap::<Family, Vec<String>>::new();
        for a in &catalog.archetypes {
            pools
                .entry(a.family)
                .or_default()
                .extend(std::iter::repeat_n(a.id.clone(), a.quantity as usize));
        }
        for pool in pools.values_mut() {
            shuffle(pool, &mut random);
        }
        let mut result = Vec::with_capacity(self.building_target_total as usize);
        for section in &self.sections {
            let mut selected = Vec::with_capacity(section.total as usize);
            for family in Family::ALL {
                for _ in 0..section.counts().get(family) {
                    let id = pools
                        .get_mut(&family)
                        .and_then(Vec::pop)
                        .ok_or_else(|| format!("{}: exhausted {family:?} registry", section.id))?;
                    selected.push(id);
                }
            }
            shuffle(&mut selected, &mut random);
            for (i, archetype_id) in selected.into_iter().enumerate() {
                result.push(BuildingAssignment {
                    id: format!(
                        "{}/g{}/{}/B{:03}",
                        self.track_id,
                        self.generator_version,
                        section.id,
                        i + 1
                    ),
                    section_id: section.id.clone(),
                    archetype_id,
                    row: if i < section.near as usize {
                        Row::Near
                    } else {
                        Row::Secondary
                    },
                });
            }
        }
        Ok(result)
    }
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Row {
    Near,
    Secondary,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct BuildingAssignment {
    pub id: String,
    pub section_id: String,
    pub archetype_id: String,
    pub row: Row,
}
// Versioned, specified xorshift64* + Fisher-Yates. No platform RNG/hash ordering.
fn shuffle<T>(items: &mut [T], state: &mut u64) {
    for i in (1..items.len()).rev() {
        *state ^= *state >> 12;
        *state ^= *state << 25;
        *state ^= *state >> 27;
        let random = state.wrapping_mul(2685821657736338717);
        items.swap(i, (random % (i as u64 + 1)) as usize);
    }
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;
    fn canonical() -> (WorldDef, ArchetypeCatalog) {
        WorldDef::from_json(
            include_str!("../../../content/world/valley_world_v1.json"),
            include_str!("../../../content/world/building_archetypes_v1.json"),
        )
        .unwrap()
    }
    #[test]
    fn canonical_quotas_and_seeded_assignments_match_every_section_and_archetype() {
        let (mut world, catalog) = canonical();
        assert_eq!(world.sections.len(), 15);
        assert_eq!(catalog.archetypes.len(), 19);
        assert_eq!(
            (
                world.near_road,
                world.secondary_row,
                world.building_target_total
            ),
            (177, 70, 247)
        );
        let assignments = world.allocate(&catalog).unwrap();
        assert_eq!(assignments, world.allocate(&catalog).unwrap());
        assert_eq!(
            assignments
                .iter()
                .map(|a| &a.id)
                .collect::<HashSet<_>>()
                .len(),
            247
        );
        for section in &world.sections {
            let entries: Vec<_> = assignments
                .iter()
                .filter(|a| a.section_id == section.id)
                .collect();
            assert_eq!(entries.len(), section.total as usize);
            assert_eq!(
                entries.iter().filter(|a| a.row == Row::Near).count(),
                section.near as usize
            );
            for family in Family::ALL {
                assert_eq!(
                    entries
                        .iter()
                        .filter(|e| catalog
                            .archetypes
                            .iter()
                            .any(|a| a.id == e.archetype_id && a.family == family))
                        .count(),
                    section.counts().get(family) as usize
                );
            }
        }
        for a in &catalog.archetypes {
            assert_eq!(
                assignments
                    .iter()
                    .filter(|b| b.archetype_id == a.id)
                    .count(),
                a.quantity as usize
            );
        }
        world.seed += 1;
        let changed = world.allocate(&catalog).unwrap();
        assert_ne!(assignments, changed);
        assert_eq!(
            assignments.iter().map(|a| &a.id).collect::<Vec<_>>(),
            changed.iter().map(|a| &a.id).collect::<Vec<_>>()
        );
    }
    #[test]
    fn station_ownership_and_blending_are_continuous_at_all_borders() {
        let (world, _) = canonical();
        assert_eq!(world.section_at(0.0), Some(0));
        assert_eq!(world.section_at(world.road_length_m), Some(14));
        for (i, s) in world.sections.iter().enumerate().skip(1) {
            assert_eq!(world.section_at(s.start_m - 0.01), Some(i - 1));
            assert_eq!(world.section_at(s.start_m), Some(i));
            assert_eq!(world.section_at(s.start_m + 0.01), Some(i));
            let (left, right, weight) = world.district_blend(s.start_m).unwrap();
            assert_eq!(left, world.sections[i - 1].district);
            assert_eq!(right, s.district);
            assert!((weight - 0.5).abs() < 1e-6);
            let a = world.district_blend(s.start_m - 0.01).unwrap().2;
            let b = world.district_blend(s.start_m + 0.01).unwrap().2;
            assert!((a - b).abs() < 0.001);
        }
        for station in [-1.0, world.road_length_m + 1.0, f32::NAN, f32::INFINITY] {
            assert!(world.section_at(station).is_none());
        }
    }
    #[test]
    fn rejects_invalid_topology_quotas_ids_heights_versions_and_unknown_fields() {
        let (world, catalog) = canonical();
        for edit in 0..16 {
            let mut w = world.clone();
            let mut c = catalog.clone();
            match edit {
                0 => w.sections[1].start_m += 0.01,
                1 => w.sections[1].start_m -= 0.01,
                2 => w.sections[0].end_m = f32::NAN,
                3 => w.sections[1].id = w.sections[0].id.clone(),
                4 => w.sections[0].farm += 1,
                5 => w.near_road += 1,
                6 => c.archetypes[0].quantity += 1,
                7 => c.archetypes[0].height_min_m = f32::INFINITY,
                8 => c.archetypes[1].id = c.archetypes[0].id.clone(),
                9 => w.generator_version += 1,
                10 => w.sections[0].total = u32::MAX,
                11 => w.sections[14].end_m += 1.0,
                12 => w.sections[0].lot_rules_id = "missing".into(),
                13 => w.lot_rules[0].access_width_m = 0.5,
                14 => c.archetypes[0].footprint_width_m.min_m = f32::NAN,
                _ => w.lot_rules[0].frontage_m.max_m = 0.0,
            }
            assert!(w.validate(&c).is_err(), "accepted invalid case {edit}");
        }
        let mut json = serde_json::to_value(&world).unwrap();
        json["unknown"] = true.into();
        assert!(
            WorldDef::from_json(&json.to_string(), &serde_json::to_string(&catalog).unwrap())
                .is_err()
        );
    }
}
