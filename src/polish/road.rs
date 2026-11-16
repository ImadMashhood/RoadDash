//! Hills asphalt detail and optional surface comparisons.
use bevy::{
    asset::RenderAssetUsages,
    image::{CompressedImageFormats, ImageSampler, ImageType},
    prelude::*,
};

const SIZE: usize = 512;

pub(crate) fn apply(images: &mut Assets<Image>, material: &mut StandardMaterial) {
    let variant = std::env::var("ROADDASH_POLISH_ROAD").unwrap_or_else(|_| "poly".into());
    let bytes: &[u8] = match variant.as_str() {
        "ambient" => include_bytes!("../../assets/textures/polish/asphalt004.png"),
        "poly" => include_bytes!("../../assets/textures/polish/asphalt_track.png"),
        "procedural" => {
            material.base_color_texture = None;
            material.base_color = Color::srgb(0.32, 0.33, 0.34);
            return;
        }
        _ => return,
    };
    let packed = Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        false,
        ImageSampler::linear(),
        RenderAssetUsages::default(),
    )
    .expect("verified packed road surface maps");
    let packed = packed.data.expect("decoded road surface bytes");
    assert_eq!(packed.len(), SIZE * SIZE * 4);
    let mut normal = Vec::with_capacity(packed.len());
    let mut rough = Vec::with_capacity(packed.len());
    for pixel in packed.chunks_exact(4) {
        normal.extend([pixel[0], pixel[1], pixel[2], 255]);
        rough.extend([255, pixel[3], 0, 255]);
    }
    material.normal_map_texture = Some(images.add(crate::road_detail::image(normal, SIZE, false)));
    material.metallic_roughness_texture =
        Some(images.add(crate::road_detail::image(rough, SIZE, false)));
}
