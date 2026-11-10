# Lighting research — 2026-11-09

Installed revisions: Bevy **0.17.3**, Rapier **0.32.0**, unchanged Cargo.lock.
Reviewed the live official pages below and the matching installed Rust/WGSL
sources. A current main-branch example is not treated as a 0.17 API contract.

- [Bevy 0.17 release](https://bevy.org/news/bevy-0-17/) introduces realtime-filtered
  environment maps; Solari remains experimental. No engine migration or raytracing
  hardware requirement is appropriate for this task.
- [Pinned environment generator](https://github.com/bevyengine/bevy/blob/v0.17.3/crates/bevy_pbr/src/light_probe/generate.rs)
  explains Lambertian diffuse and GGX specular convolution, preferring prefiltered
  maps when practical. Its realtime path needs compute/storage support and runs
  filtering render nodes. The stationary procedural RoadDash sky does not need
  recurring GPU filtering: use bounded CPU preparation once at startup.
- [Pinned environment shader](https://github.com/bevyengine/bevy/blob/v0.17.3/crates/bevy_pbr/src/light_probe/environment_map.wgsl)
  chooses specular mip by perceptual roughness and reverses sample Z for cube
  handedness. It expects already-integrated diffuse radiance (Lambert integral
  divided by pi). Raw directional sky samples and ordinary box-downsampled mips
  do not satisfy that contract. The previous environment also mirrored its solar
  hemisphere independently of the already-corrected visible sky.
- [Pinned SSAO example](https://github.com/bevyengine/bevy/blob/v0.17.3/examples/3d/ssao.rs)
  and [implementation](https://github.com/bevyengine/bevy/blob/v0.17.3/crates/bevy_pbr/src/ssao/mod.rs):
  SSAO adds depth/normal prepasses and compute passes, requires MSAA off, and
  recommends temporal AA to reduce noise. It helps local indirect contacts,
  not direct sunlight. It is deferred here: preserving current MSAA and avoiding
  unmeasured AA/temporal/compute regressions is preferable for this default.
- [Filament physical rendering documentation](https://google.github.io/filament/Filament.html#lighting/imagebasedlights)
  is the additional engine reference linked by Bevy's environment-map API.
  The site redirected to its overview in the web reader; the pinned Bevy source
  is the actual inspected integration contract, not a claim to a full review of
  Filament's implementation.

Selected implementation: original standard cosine-weighted hemisphere sampling
for diffuse irradiance and GGX half-vector importance sampling for the split-sum
specular prefilter. Work is deterministic, linear-light, bounded to 64 samples,
16px diffuse / 128px specular cube faces, eight specular mips. The original
procedural cloud radiance supplies both the visible display and environment;
muted ground bounce supplies the lower hemisphere. Direct sun remains separately
shadowed, with no high-energy duplicate sun in the environment. Reduce the
uniform ambient floor and increase directional sky fill to reveal surface shape.

No external implementation or art copied; mathematical algorithms are independently
implemented in src/sky/lighting.rs. No shader, model, texture, package, engine
version or proprietary media imported. Bevy remains MIT OR Apache-2.0; inspected
[pinned MIT license](https://github.com/bevyengine/bevy/blob/v0.17.3/LICENSE-MIT).
Existing third-party notices and earlier SBDT adaptation remain applicable.
Research permission is not asserted as blanket legal clearance.
