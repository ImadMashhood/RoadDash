//! Optional user-supplied pictures stay outside the repository and asset bundle.
use super::*;
use bevy::{
    asset::RenderAssetUsages,
    image::{CompressedImageFormats, ImageSampler, ImageType},
};
use std::path::PathBuf;

pub(super) fn load(images: &mut Assets<Image>) -> Vec<Option<Handle<Image>>> {
    let kit = std::env::var_os("ROADDASH_UI_REFERENCES")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .map(|home| PathBuf::from(home).join("Documents/ChatGPT/RoadDash-UI-References"))
        });
    // Complete composed screens preserve inset offsets; narrow postcard source is
    // deliberately never stretched to become a full background.
    let slots = [
        "reference-images/UI-IMG-0142.png",
        "reference-images/UI-IMG-0068.png",
        "menu-previews/RR-0E.png",
        "reference-images/UI-IMG-0146.png",
        "menu-previews/RR-0A.png",
        "reference-images/UI-IMG-0071.png",
        "reference-images/UI-IMG-0072.png",
        "reference-images/UI-IMG-0045.png",
    ];
    slots
        .iter()
        .map(|slot| {
            let bytes = std::fs::read(kit.as_ref()?.join(slot)).ok()?;
            match Image::from_buffer(
                &bytes,
                ImageType::Extension("png"),
                CompressedImageFormats::NONE,
                true,
                ImageSampler::linear(),
                RenderAssetUsages::default(),
            ) {
                Ok(image) => Some(images.add(image)),
                Err(error) => {
                    warn!("Optional UI picture {slot} failed; using project artwork: {error}");
                    None
                }
            }
        })
        .collect()
}
