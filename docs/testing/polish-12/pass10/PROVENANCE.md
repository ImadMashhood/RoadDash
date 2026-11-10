# Sky provenance and renderer research

The visible sky/cloud field is original RoadDash procedural source in src/sky.rs;
no image, third-party skybox, reference artwork or proprietary media was imported.
Original legacy radiance cubes remain unchanged; the new display cube is generated
locally. No dependency/version/lockfile change or facade source/atlas intake.

Inspected the installed Bevy0.17.3 dependency source:
- bevy_core_pipeline/src/skybox/{mod.rs,skybox.wgsl}: cube Z reflection and CPU
  brightness/exposure multiplication; no automatic non-HDR sky tonemap.
- bevy_core_pipeline/src/tonemapping/tonemapping_shared.wgsl: analytic Reinhard
  and SomewhatBoringDisplayTransform equations.
- bevy_pbr/src/render/{pbr_functions.wgsl,fog.wgsl}: exposed lighting, fog then
  non-HDR in-shader tonemap ordering (carried from pass09).

The optional SomewhatBoringDisplayTransform CPU implementation adapts Tomasz
Stachowiak's operator as distributed by Bevy0.17.3. Attribution is also adjacent
to the source. This is licensed algorithm/source adaptation, not a claim of zero
external code influence. Bevy license: MIT OR Apache-2.0; repository notices are
retained at docs/third_party/licenses/Bevy-MIT.txt and Apache-2.0.txt. Source:
https://github.com/bevyengine/bevy/blob/v0.17.3/crates/bevy_core_pipeline/src/tonemapping/tonemapping_shared.wgsl
https://github.com/bevyengine/bevy/blob/v0.17.3/LICENSE-MIT
No broader legal clearance is inferred. Existing ASSETS.csv original-cloud-daylight
hash/description is updated; REPOSITORIES.md records this installed-source review.
