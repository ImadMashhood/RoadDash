//! Original procedural civic PBR: cut limestone, oxidized copper, aged brass.
//! Three shared 128px triplets, full mip chains; no external art or facade changes.
use bevy::prelude::*;

const SIDE: usize = 128;
pub(super) const TILE_METERS: f32 = 2.0;

fn hash(x: usize, y: usize) -> f32 {
    let mut n = (x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(668265263);
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    ((n ^ (n >> 16)) & 65535) as f32 / 65535.0
}
/// Periodic smooth value noise; wrapping the lattice avoids seams at any scale.
fn noise(x: usize, y: usize, cells: usize) -> f32 {
    let u = x as f32 * cells as f32 / SIDE as f32;
    let v = y as f32 * cells as f32 / SIDE as f32;
    let ix = u.floor() as usize;
    let iy = v.floor() as usize;
    let smooth = |a: f32| a * a * (3.0 - 2.0 * a);
    let a = smooth(u.fract());
    let b = smooth(v.fract());
    let h = |dx, dy| hash((ix + dx) % cells, (iy + dy) % cells);
    (h(0, 0) * (1.0 - a) + h(1, 0) * a) * (1.0 - b) + (h(0, 1) * (1.0 - a) + h(1, 1) * a) * b
}
fn relief(x: usize, y: usize, kind: usize) -> f32 {
    let grain = hash(x % SIDE, y % SIDE);
    if kind == 0 {
        // Fine pores and tool-work; joints belong to the actual stone blocks.
        grain * 0.13 + noise(x, y, 32) * 0.17
    } else {
        grain * 0.025 + noise(x, y, 16) * 0.04
    }
}
fn pixels(kind: usize) -> [Vec<u8>; 3] {
    let mut maps = std::array::from_fn(|_| Vec::with_capacity(SIDE * SIDE * 4));
    for y in 0..SIDE {
        for x in 0..SIDE {
            let cloud = noise(x, y, 4) * 0.65 + noise(x, y, 16) * 0.35;
            let grain = hash(x, y);
            let (color, roughness, metal) = match kind {
                0 => {
                    let tone = 0.84 + cloud * 0.15 + (grain - 0.5) * 0.07;
                    ([tone, tone * 0.985, tone * 0.96], 0.81 + grain * 0.14, 0.0)
                }
                1 => {
                    // Broad patina islands with occasional warm copper showing through.
                    let bare = ((cloud - 0.5) * 3.0).clamp(0.0, 0.65);
                    (
                        [0.15 + bare * 0.40, 0.31 - bare * 0.10, 0.28 - bare * 0.16],
                        0.86 - bare * 0.40,
                        bare * 0.85,
                    )
                }
                _ => {
                    let tone = 0.83 + cloud * 0.17;
                    (
                        [0.52 * tone, 0.37 * tone, 0.13 * tone],
                        0.42 + cloud * 0.18,
                        0.6,
                    )
                }
            };
            maps[0].extend(color.map(|v| (v * 255.0) as u8));
            maps[0].push(255);
            let dx = relief((x + 1) % SIDE, y, kind) - relief((x + SIDE - 1) % SIDE, y, kind);
            let dy = relief(x, (y + 1) % SIDE, kind) - relief(x, (y + SIDE - 1) % SIDE, kind);
            let n = Vec3::new(-dx, -dy, 1.0).normalize();
            maps[1].extend(n.to_array().map(|v| ((v * 0.5 + 0.5) * 255.0) as u8));
            maps[1].push(255);
            maps[2].extend([255, (roughness * 255.0) as u8, (metal * 255.0) as u8, 255]);
        }
    }
    maps
}
pub(super) fn materials(images: &mut Assets<Image>) -> [StandardMaterial; 5] {
    let triplets: [[Handle<Image>; 3]; 3] = std::array::from_fn(|kind| {
        let [albedo, normal, rough] = pixels(kind);
        [
            images.add(crate::road_detail::image(albedo, SIDE, true)),
            images.add(crate::road_detail::image(normal, SIDE, false)),
            images.add(crate::road_detail::image(rough, SIDE, false)),
        ]
    });
    std::array::from_fn(|i| {
        if i == 2 {
            return StandardMaterial {
                base_color: Color::srgb(0.065, 0.095, 0.095),
                perceptual_roughness: 0.86,
                ..default()
            };
        }
        let family = i.saturating_sub(2);
        let [albedo, normal, rough] = triplets[family].clone();
        StandardMaterial {
            base_color: match i {
                0 => Color::srgb(0.65, 0.60, 0.48),
                1 => Color::srgb(0.94, 0.89, 0.74),
                _ => Color::WHITE,
            },
            base_color_texture: Some(albedo),
            normal_map_texture: Some(normal),
            metallic_roughness_texture: Some(rough),
            metallic: 1.0,
            perceptual_roughness: 1.0,
            ..default()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn civic_maps_are_periodic_bounded_and_fully_mipmapped() {
        for kind in 0..3 {
            for y in 0..SIDE {
                assert_eq!(relief(0, y, kind), relief(SIDE, y, kind));
                assert_eq!(relief(y, 0, kind), relief(y, SIDE, kind));
            }
            for (i, map) in pixels(kind).into_iter().enumerate() {
                assert_eq!(map.len(), SIDE * SIDE * 4);
                assert!(map.chunks_exact(4).all(|p| p[3] == 255));
                let image = crate::road_detail::image(map, SIDE, i == 0);
                assert_eq!(image.texture_descriptor.mip_level_count, 8);
                assert_eq!(image.data.unwrap().len(), 87380);
            }
        }
    }
}
