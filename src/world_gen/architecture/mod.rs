//! Deterministic visual construction over immutable Valley lots and colliders.
use super::{WorldPlan, layout::WorldLayout};
use bevy::prelude::*;
use rr_content::architecture::{Catalog, Design, Rules};
pub mod render;
#[cfg(test)]
mod tests;
pub const CATALOG: &str =
    include_str!("../../../art/buildings/mesh_assets/storey_kit/MANIFEST.json");
pub const RULES: &str = include_str!("../../../content/world/architecture_rules.json");
#[derive(Clone)]
pub struct Assembly {
    pub id: String,
    pub design: usize,
    pub position: Vec3,
    pub rotation: Quat,
    pub size: Vec3,
}
#[derive(Resource)]
pub struct Architecture {
    pub catalog: Catalog,
    pub rules: Rules,
    pub buildings: Vec<Assembly>,
}
#[derive(Clone)]
pub struct Panel {
    pub band: usize,
    pub transform: Transform,
}
fn hash(s: &str) -> u64 {
    s.bytes().fold(2166136261u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(16777619)
    })
}
impl Architecture {
    pub fn new(plan: &WorldPlan, layout: &WorldLayout) -> Result<Self, String> {
        let catalog = Catalog::from_json(CATALOG)?;
        let rules = Rules::from_json(RULES, &catalog)?;
        for a in &plan.archetypes.archetypes {
            if !rules.rules.contains_key(&a.id) {
                return Err(format!("missing architecture role {}", a.id));
            }
        }
        let mut buildings = Vec::new();
        for b in layout.all_buildings() {
            let ids = rules
                .rules
                .get(&b.archetype_id)
                .ok_or("unmapped building role")?;
            // Same archetype/block shares a family, with stable per-lot variants.
            let key = format!(
                "{}:{}:{}",
                b.section_id,
                (b.station_m / 100.0) as u32,
                b.archetype_id
            );
            let choice =
                &ids[(hash(&key).wrapping_add(hash(&b.id) % 3) % ids.len() as u64) as usize];
            let design = catalog
                .records
                .iter()
                .position(|d| &d.id == choice)
                .ok_or("unknown kit ID")?;
            buildings.push(Assembly {
                id: b.id.clone(),
                design,
                position: Vec3::from_array(b.position),
                rotation: Quat::from_rotation_y(b.yaw),
                size: Vec3::from_array(b.size),
            });
        }
        Ok(Self {
            catalog,
            rules,
            buildings,
        })
    }
}
impl Assembly {
    pub fn face_mask(&self, camera: Vec3) -> u8 {
        let d = self.rotation.inverse() * (camera - self.position);
        u8::from(d.x >= 0.0)
            | (u8::from(d.x < 0.0) << 1)
            | (u8::from(d.z >= 0.0) << 2)
            | (u8::from(d.z < 0.0) << 3)
    }
    pub fn panels(&self, d: &Design, mask: u8) -> Vec<Panel> {
        let mut panels = Vec::new();
        let faces = [
            (
                Vec3::X,
                self.size.x,
                self.size.z,
                std::f32::consts::FRAC_PI_2,
            ),
            (
                -Vec3::X,
                self.size.x,
                self.size.z,
                -std::f32::consts::FRAC_PI_2,
            ),
            (Vec3::Z, self.size.z, self.size.x, 0.0),
            (-Vec3::Z, self.size.z, self.size.x, std::f32::consts::PI),
        ];
        for (face, (normal, depth, width, yaw)) in faces.into_iter().enumerate() {
            if mask & (1 << face) == 0 {
                continue;
            }
            let bays = if d.asset_type == "structure" {
                1
            } else {
                (width / d.width).round().clamp(1.0, 8.0) as usize
            };
            let scale = width / (bays as f32 * d.width);
            let ground = d.heights[0] * scale;
            let cap = d.heights[2] * scale;
            let remaining = self.size.y - ground - cap;
            let repeats = if d.asset_type == "structure" || remaining <= 0.3 {
                0
            } else {
                (remaining / (d.heights[1] * scale))
                    .round()
                    .clamp(1.0, 24.0) as usize
            };
            let ground_height = if repeats == 0 {
                (self.size.y - cap).max(self.size.y * 0.6)
            } else {
                ground
            };
            let cap_height = if repeats == 0 {
                self.size.y - ground_height
            } else {
                cap
            };
            let mut rows = vec![(0, 0.0, ground_height)];
            if repeats > 0 {
                let h = remaining / repeats as f32;
                for row in 0..repeats {
                    rows.push((1, ground + row as f32 * h, h));
                }
            }
            if cap_height > 0.001 && d.heights[2] > 0.001 {
                rows.push((2, self.size.y - cap_height, cap_height));
            }
            let rotation = Quat::from_rotation_y(yaw);
            for (band, y, height) in rows {
                if d.lods[2].triangles[band] == 0 {
                    continue;
                }
                let outward = d
                    .lods
                    .iter()
                    .map(|l| l.bounds[band].max[2])
                    .fold(0.0f32, f32::max);
                for bay in 0..bays {
                    let across = -width * 0.5 + (bay as f32 + 0.5) * width / bays as f32;
                    panels.push(Panel {
                        band,
                        transform: Transform::from_translation(
                            normal * (depth * 0.5 - outward * scale)
                                + rotation * Vec3::X * across
                                + Vec3::Y * y,
                        )
                        .with_rotation(rotation)
                        .with_scale(Vec3::new(
                            scale,
                            height / d.heights[band].max(0.001),
                            scale,
                        )),
                    });
                }
            }
        }
        panels
    }
    pub fn inset(&self, d: &Design) -> f32 {
        let span = d
            .lods
            .iter()
            .flat_map(|l| l.bounds.iter())
            .map(|b| b.max[2] - b.min[2])
            .fold(0.0f32, f32::max);
        (span * 1.3).clamp(0.5, (self.size.x.min(self.size.z) * 0.5 - 0.5).max(0.5))
    }
}
