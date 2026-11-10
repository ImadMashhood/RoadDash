//! Baked curb furniture, independent solid contact and an open building-side walk.
use super::*;
pub fn spawn(commands: &mut Commands, art: &Art, track: &Track) -> usize {
    spawn_props(
        commands,
        art,
        track,
        &track.metropolis.as_ref().unwrap().street_props,
    )
}
pub fn spawn_props(
    commands: &mut Commands,
    art: &Art,
    track: &Track,
    items: &[rr_content::metropolis::StreetProp],
) -> usize {
    let Some(props) = &art.props else {
        return 0;
    };
    for (i, p) in items.iter().enumerate() {
        let base = Vec3::new(
            p.position[0],
            track.terrain_height(p.position[0], p.position[1]) + 0.14,
            p.position[1],
        );
        let q = Quat::from_rotation_y(p.heading);
        let size = Vec3::from_array(p.size);
        crate::destruction::register(
            commands,
            props,
            &p.kind,
            i % 4,
            &p.id,
            Transform::from_translation(base).with_rotation(q),
            size,
            Some(i),
        );
    }
    items.len()
}
pub fn paving(images: &mut Assets<Image>) -> StandardMaterial {
    let mut pixels = Vec::with_capacity(256 * 256 * 4);
    for y in 0..256u32 {
        for x in 0..256u32 {
            let seam = x % 64 < 1 || y % 64 < 1;
            let grain = ((x * 137 + y * 313) % 7) as u8;
            let slab = ((x / 64 * 7 + y / 64 * 11) % 9) as u8;
            // Narrow drainage/expansion grooves, with accumulated dirt feathered
            // into each slab. All detail is in the existing mipmapped surface.
            let edge = (x % 64).min(63 - x % 64).min((y % 64).min(63 - y % 64));
            let dirt = (4 - edge.min(4)) as u8 * 2;
            let v = if seam { 126 } else { 172 + grain + slab - dirt };
            pixels.extend([v, v.saturating_sub(3), v.saturating_sub(9), 255]);
        }
    }
    StandardMaterial {
        base_color_texture: Some(images.add(crate::road_detail::image(pixels, 256, true))),
        perceptual_roughness: 0.95,
        ..default()
    }
}
