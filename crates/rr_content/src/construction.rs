//! GPU-independent structural construction and sampling/LOD authoring contract.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Junction {
    pub id: String,
    pub station_m: f32,
    pub half_width_m: f32,
    pub extent_m: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Construction {
    #[serde(default)]
    pub city_grid: CityGrid,
    pub schema_version: u32,
    pub layout_version: u32,
    pub track_id: String,
    pub field_count: usize,
    pub field_end_m: f32,
    pub sidewalk_start_m: f32,
    pub sidewalk_end_m: f32,
    pub sidewalk_width_m: f32,
    pub curb_height_m: f32,
    pub pad_flat_margin_m: f32,
    pub pad_blend_m: f32,
    pub max_retained_relief_m: f32,
    pub terrain_step_m: f32,
    pub terrain_far_step_m: f32,
    pub terrain_near_m: f32,
    pub render_near_m: f32,
    pub render_far_m: f32,
    pub junctions: Vec<Junction>,
}
impl Construction {
    pub fn from_json(json: &str, track_id: &str, length: f32) -> Result<Self, String> {
        let c: Self = serde_json::from_str(json).map_err(|e| format!("construction: {e}"))?;
        c.city_grid.validate(length)?;
        let values = [
            c.field_end_m,
            c.sidewalk_start_m,
            c.sidewalk_end_m,
            c.sidewalk_width_m,
            c.curb_height_m,
            c.pad_flat_margin_m,
            c.pad_blend_m,
            c.max_retained_relief_m,
            c.terrain_step_m,
            c.terrain_far_step_m,
            c.terrain_near_m,
            c.render_near_m,
            c.render_far_m,
        ];
        if !length.is_finite()
            || !(100.0..=20000.0).contains(&length)
            || rr_core::ContentId::try_from(c.track_id.clone()).is_err()
            || c.schema_version != 1
            || c.layout_version != 1
            || c.track_id != track_id
            || !values.iter().all(|v| v.is_finite())
            || !(36..=48).contains(&c.field_count)
            || !c.field_count.is_multiple_of(2)
            || c.field_end_m <= 0.0
            || c.field_end_m > length
            || c.sidewalk_start_m < 0.0
            || c.sidewalk_end_m <= c.sidewalk_start_m
            || c.sidewalk_end_m > length
            || !(2.0..=3.0).contains(&c.sidewalk_width_m)
            || !(0.1..=0.16).contains(&c.curb_height_m)
            || !(0.1..=2.0).contains(&c.pad_flat_margin_m)
            || !(1.0..=8.0).contains(&c.pad_blend_m)
            || !(1.0..=8.0).contains(&c.max_retained_relief_m)
            || !(2.0..=5.0).contains(&c.terrain_step_m)
            || !(8.0..=20.0).contains(&c.terrain_far_step_m)
            || !(250.0..=600.0).contains(&c.terrain_near_m)
            || (200.0 / c.terrain_step_m - (200.0 / c.terrain_step_m).round()).abs() > 0.0001
            || (200.0 / c.terrain_far_step_m - (200.0 / c.terrain_far_step_m).round()).abs()
                > 0.0001
            || c.render_near_m < 50.0
            || c.render_far_m < c.render_near_m + 100.0
            || c.junctions.len() < 7
            || c.junctions.len() > 20
        {
            return Err("invalid construction dimensions/version/budgets".into());
        }
        let mut ids = std::collections::HashSet::new();
        let mut station = -100.0;
        for j in &c.junctions {
            if j.id.is_empty()
                || !ids.insert(&j.id)
                || ![j.station_m, j.half_width_m, j.extent_m]
                    .iter()
                    .all(|v| v.is_finite())
                || j.station_m < 100.0
                || j.station_m > length - 100.0
                || j.station_m - station < 40.0
                || !(4.0..=8.0).contains(&j.half_width_m)
                || !(60.0..=120.0).contains(&j.extent_m)
            {
                return Err(format!("{}: invalid junction/reservation", j.id));
            }
            station = j.station_m;
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construction_rejects_bad_binding_grid_dimensions_junctions_and_unknown_fields() {
        let input = include_str!("../../../content/world/valley_construction_v1.json");
        let parse = |s: &str, id: &str, length: f32| Construction::from_json(s, id, length);
        assert!(parse(input, "valley-run-01", 6847.3076).is_ok());
        assert!(parse(input, "another-track", 6847.3076).is_err());
        assert!(parse(input, "valley-run-01", f32::NAN).is_err());
        let original: serde_json::Value = serde_json::from_str(input).unwrap();
        for case in 0..5 {
            let mut json = original.clone();
            match case {
                0 => json["terrain_step_m"] = 3.0.into(),
                1 => json["curb_height_m"] = 0.8.into(),
                2 => json["junctions"][1]["id"] = json["junctions"][0]["id"].clone(),
                3 => json["junctions"][0]["station_m"] = (-20.0).into(),
                _ => json["unknown"] = true.into(),
            };
            assert!(
                parse(&json.to_string(), "valley-run-01", 6847.3076).is_err(),
                "accepted case {case}"
            );
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CityGrid {
    pub start_m: f32,
    pub end_m: f32,
    pub avenues: Vec<f32>,
    pub frontage_rows: Vec<f32>,
    pub block_length_m: f32,
    pub parcel_pitch_m: f32,
}
impl Default for CityGrid {
    fn default() -> Self {
        Self {
            start_m: 3150.0,
            end_m: 4900.0,
            avenues: vec![105.0, 165.0, 225.0, 285.0],
            frontage_rows: vec![20.0, 52.0, 82.0, 124.0, 146.0, 184.0, 206.0, 244.0, 266.0],
            block_length_m: 100.0,
            parcel_pitch_m: 24.0,
        }
    }
}
impl CityGrid {
    fn validate(&self, length: f32) -> Result<(), String> {
        if ![
            self.start_m,
            self.end_m,
            self.block_length_m,
            self.parcel_pitch_m,
        ]
        .iter()
        .chain(&self.avenues)
        .chain(&self.frontage_rows)
        .all(|v| v.is_finite())
            || self.start_m < 0.0
            || self.end_m > length
            || self.end_m <= self.start_m
            || !(60.0..=180.0).contains(&self.block_length_m)
            || !(22.0..=60.0).contains(&self.parcel_pitch_m)
            || !(2..=5).contains(&self.avenues.len())
            || self.frontage_rows.len() > 12
            || self.avenues.iter().any(|v| !(90.0..=300.0).contains(v))
            || self.avenues.windows(2).any(|v| v[1] - v[0] < 45.0)
            || self
                .frontage_rows
                .iter()
                .any(|v| !(20.0..=300.0).contains(v))
        {
            return Err("invalid city grid bounds or budgets".into());
        }
        Ok(())
    }
}
