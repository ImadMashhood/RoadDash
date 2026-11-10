# Atmosphere provenance

No media, dependency, external repository or reference-code intake. Original
local `src/sky.rs` procedural cubemap remains byte-equivalent in authored colour
inputs; this pass exposes its existing horizon constant in typed local content.
Original code changes select Bevy's already-installed built-in fog mode.

Inspected pinned registry Bevy0.17.3 `bevy_pbr/src/fog.rs`,
`bevy_pbr/src/render/fog.wgsl`, `pbr_functions.wgsl`, and
`bevy_core_pipeline/src/skybox/{mod.rs,skybox.wgsl}` to verify falloff and
exposure/tonemapping ordering. No shader source copied into this repository.
Existing dependency license/provenance unchanged. No new legal clearance claim.

No model, facade material, atlas payload/hash/mip, approved layout or generated
world asset changed. The original-cloud-daylight source declaration in ASSETS.csv is updated for
changed src/sky.rs; no media/atlas hash update is required.
