//! Offline city assembly contract. Geometry/physics adapters consume this plan.
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
pub const TRACK_ID: &str = "city-run-5300";
pub type BoxRecord = (String, [f32; 3], [f32; 3], [u8; 3]);
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CitySettings {
    pub seed: u64,
    pub length_m: f32,
    pub block_pitch_m: f32,
    pub cross_street_width_m: f32,
    pub road_width_m: f32,
    pub sidewalk_width_m: f32,
    pub parallel_road_offset_m: f32,
    pub parallel_road_width_m: f32,
    pub building_depth_m: f32,
    pub terrain_margin_m: f32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Zone {
    pub start_m: f32,
    pub end_m: f32,
    pub district: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildingLot {
    pub id: String,
    pub family: String,
    pub district: String,
    pub block: usize,
    pub row: String,
    pub side: i32,
    pub z_start: f32,
    pub z_end: f32,
    pub front_x: f32,
    pub center_x: f32,
    pub bays: usize,
    pub depth: f32,
    pub height_m: f32,
    pub facade_scale: f32,
    pub exposed_start: bool,
    pub exposed_end: bool,
}
impl BuildingLot {
    pub fn inward(&self) -> f32 {
        self.side as f32 * if self.row == "avenue" { 1.0 } else { -1.0 }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CityChunk {
    pub name: String,
    #[serde(default)]
    pub material: Option<String>,
    pub boxes: Vec<BoxRecord>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CityPlan {
    pub schema_version: u32,
    pub seed: u64,
    pub length_m: f32,
    pub config: CitySettings,
    pub district_zones: Vec<Zone>,
    pub lots: Vec<BuildingLot>,
    pub chunks: Vec<CityChunk>,
    pub model_design_ids: Vec<String>,
}
impl CityPlan {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let plan: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        plan.validate()?;
        Ok(plan)
    }
    pub fn validate(&self) -> Result<(), String> {
        let c = &self.config;
        if self.schema_version != 3
            || !self.length_m.is_finite()
            || self.length_m < 100.0
            || self.length_m > 20000.0
            || (self.length_m - c.length_m).abs() > 0.01
            || self.seed != c.seed
        {
            return Err("city schema, seed or length mismatch".into());
        }
        for v in [
            c.block_pitch_m,
            c.cross_street_width_m,
            c.road_width_m,
            c.sidewalk_width_m,
            c.parallel_road_offset_m,
            c.parallel_road_width_m,
            c.building_depth_m,
            c.terrain_margin_m,
        ] {
            if !v.is_finite() || v <= 0.0 {
                return Err("invalid city dimensions".into());
            }
        }
        if self.length_m % c.block_pitch_m > 0.01
            || c.cross_street_width_m + 2.0 * c.sidewalk_width_m + 16.0 >= c.block_pitch_m
        {
            return Err("city requires whole usable blocks".into());
        }
        if c.terrain_margin_m < 200.0 {
            return Err("insufficient ejection terrain runoff".into());
        }
        let mut cursor = 0.0;
        for z in &self.district_zones {
            if z.start_m != cursor
                || !z.end_m.is_finite()
                || z.end_m <= cursor
                || z.end_m > self.length_m
                || z.district.is_empty()
            {
                return Err("district coverage gap/overlap".into());
            }
            cursor = z.end_m;
        }
        if cursor != self.length_m {
            return Err("incomplete district coverage".into());
        }
        let mut ids = HashSet::new();
        let families: HashSet<_> = self.model_design_ids.iter().collect();
        let mut rows: HashMap<(usize, i32, &str), Vec<&BuildingLot>> = HashMap::new();
        let outer = c.parallel_road_offset_m - c.parallel_road_width_m / 2.0 - c.sidewalk_width_m;
        for b in &self.lots {
            if !ids.insert(&b.id)
                || !families.contains(&b.family)
                || ![-1, 1].contains(&b.side)
                || !["avenue", "outer"].contains(&b.row.as_str())
                || b.bays == 0
                || b.bays > 16
            {
                return Err("invalid lot identity/role".into());
            }
            if [
                b.z_start,
                b.z_end,
                b.front_x,
                b.center_x,
                b.depth,
                b.height_m,
                b.facade_scale,
            ]
            .iter()
            .any(|v| !v.is_finite())
                || b.z_start >= b.z_end
                || b.depth <= 0.0
                || b.height_m <= 0.0
                || b.facade_scale <= 0.0
            {
                return Err("invalid lot geometry".into());
            }
            let start = b.block as f32 * c.block_pitch_m
                + c.cross_street_width_m / 2.0
                + c.sidewalk_width_m;
            let end = (b.block + 1) as f32 * c.block_pitch_m
                - c.cross_street_width_m / 2.0
                - c.sidewalk_width_m;
            let expected = b.side as f32
                * if b.row == "avenue" {
                    c.road_width_m / 2.0 + c.sidewalk_width_m
                } else {
                    outer
                };
            if b.z_start < start - 0.01
                || b.z_end > end + 0.01
                || (b.front_x - expected).abs() > 0.01
                || (b.center_x - b.front_x - b.inward() * b.depth / 2.0).abs() > 0.01
                || b.depth > c.building_depth_m + 0.01
            {
                return Err("lot crosses street reservation".into());
            }
            let zone = self
                .district_zones
                .iter()
                .find(|z| b.z_start >= z.start_m && b.z_end <= z.end_m)
                .ok_or("lot crosses district boundary")?;
            if b.district != zone.district {
                return Err("lot district differs from zone".into());
            }
            rows.entry((b.block, b.side, &b.row)).or_default().push(b);
        }
        if rows.len() != (self.length_m / c.block_pitch_m) as usize * 4 {
            return Err("missing block frontage".into());
        }
        for ((block, _, _), mut row) in rows {
            row.sort_by(|a, b| a.z_start.total_cmp(&b.z_start));
            let start =
                block as f32 * c.block_pitch_m + c.cross_street_width_m / 2.0 + c.sidewalk_width_m;
            let end = (block + 1) as f32 * c.block_pitch_m
                - c.cross_street_width_m / 2.0
                - c.sidewalk_width_m;
            if (row[0].z_start - start).abs() > 0.01
                || (row.last().unwrap().z_end - end).abs() > 0.01
                || row
                    .windows(2)
                    .any(|p| (p[0].z_end - p[1].z_start).abs() > 0.01)
            {
                return Err("frontage has gaps or overlaps".into());
            }
        }
        if outer - (c.road_width_m / 2.0 + c.sidewalk_width_m) - 2.0 * c.building_depth_m < 4.0 {
            return Err("rear recovery court too narrow".into());
        }
        for chunk in &self.chunks {
            for (name, centre, size, _) in &chunk.boxes {
                if name == "cross_street_sidewalk"
                    && (centre[0].abs() - size[0] / 2.0 < c.road_width_m / 2.0 - 0.01
                        || centre[0].abs() + size[0] / 2.0
                            > c.parallel_road_offset_m - c.parallel_road_width_m / 2.0 + 0.01)
                {
                    return Err("cross-street sidewalk intrudes on a road".into());
                }
                if centre.iter().chain(size).any(|v| !v.is_finite())
                    || size.iter().any(|v| *v <= 0.0)
                {
                    return Err("invalid structural geometry".into());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn plan() -> CityPlan {
        CityPlan::from_json(include_str!("../../../content/world/city5300.json")).unwrap()
    }
    #[test]
    fn generated_city_has_complete_reserved_blocks_and_whole_buildings() {
        let p = plan();
        assert_eq!(p.length_m, 5300.0);
        assert_eq!(p.lots.len(), 977);
        assert_eq!(p.model_design_ids.len(), 85);
        let roof_count = p
            .chunks
            .iter()
            .flat_map(|c| &c.boxes)
            .filter(|b| b.0 == "shared_closed_roof")
            .count();
        assert_eq!(roof_count, p.lots.len());
        assert!(p.chunks.iter().any(|c| c.material.is_some()));
        assert!(
            p.chunks
                .iter()
                .flat_map(|c| &c.boxes)
                .any(|b| b.0 == "recessed_rear_glass")
        );
    }
    #[test]
    fn malformed_sites_district_gaps_and_missing_frontage_are_rejected() {
        let mut p = plan();
        p.lots[0].front_x = 0.0;
        assert!(p.validate().is_err());
        let mut p = plan();
        p.district_zones[1].start_m += 1.0;
        assert!(p.validate().is_err());
        let mut p = plan();
        p.lots.remove(0);
        assert!(p.validate().is_err());
        let mut p = plan();
        p.chunks[0].boxes[0].2[0] = f32::NAN;
        assert!(p.validate().is_err());
    }
}
