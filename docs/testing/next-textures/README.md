# Original civic surface detail — 2026-11-09

Focused continuation from `9ceae5c2`, branch `codex/next-textures`.
Existing MAT-02 and CITY-CIVIC-01 remain partial; no new requirement IDs or
checklist closures. Root integration owns shared README/PROGRESS/ADR/provenance.

The five resident City Hall ornament batches now carry face-aligned UVs measured
in world metres, including the existing 2.8 hall scale. Original periodic
limestone pores, copper patina islands and aged brass use albedo, tangent-space
normal and roughness/metallic maps. Stone and pale trim share one triplet; dark
reveals retain their solid material. Each face gets valid tangents. Actual block
joints, roof seams, arches, architecture, positions, indices, contact and map
placement are unchanged. This affects civic ornament only; facade source families,
atlas bands/hashes and the underlying closed civic envelope are untouched.

## Provenance and budgets

`src/world_gen/metropolis/hall_surface.rs` is the editable original procedural
source. No external assets, repositories, media or copied reference code were
used. Smooth periodic value noise and hashed fine grain are generated once when
the hall spawns, not in frame updates. Bevy 0.17.3/Rapier 0.32.0 stay pinned.

- Three texture families × three RGBA8 maps × 128², eight mip levels down to 1².
- 87,380 bytes per full chain; nine maps total 786,420 bytes (0.750 MiB), excluding
  GPU allocation alignment, temporary generation buffers and CPU retained data.
- One tile covers 2m: 64 source texels/metre, consistent across rotated/narrow
  ornament faces. Repeat addressing and linear mip sampling use the existing
  road-image helper. Low-amplitude normal slopes avoid exaggerated stone relief.
- Exactly five resident mesh/material batches remain. UV/tangent vertex attributes
  increase CPU/GPU mesh data; triangle count and draw-batch count do not increase.
- Copper also covers the existing blue-green pane geometry in its shared batch;
  patina is deliberately restrained. Separating glazing would require a separate
  authoring/batching decision and is outside this pass.

## Validation and remaining gates

Source includes a face-edge texel-density check across every ornament quad,
existing outward winding/bounds/triangle-budget assertions, and periodicity,
map dimensions/alpha and full-mip payload checks. Cargo/native execution is
currently held by the shared build/GPU slot; results will be recorded when run.
No performance improvement or native visual acceptance is claimed yet. Full
near/far art, riding-speed temporal review, broader world materials, hardware
coverage and the 16.7ms frame gate remain open.

## Handoff evidence

Completed locally: direct `rustfmt --edition 2024` on changed Rust files,
`git diff --check`, and Python compilation of the native runner. **Not run:**
cargo fmt, Clippy, workspace/targeted tests, debug/release builds, audit.py,
new native views or performance measurements. Root reserved the shared build/GPU
slot for another integration and explicitly requested source handoff; these checks
must run on integration before treating this pass as verified.

Targeted Rust tests: `resident_stonework_is_bounded_and_outward_wound` and
`civic_maps_are_periodic_bounded_and_fully_mipmapped`. Also retain full mandated
workspace checks and native recovery/NPC regressions during integration.

Historical baseline views (not newly captured, not a same-binary A/B):
`../next-town-hall/final/screenshots/metro-00.png` through `metro-04.png`.
Their `../next-town-hall/final/receipt.json` identifies the frozen hall binary SHA256
`1c171d50720486bc3f667385e9dca7e5f6ec0f4cbed7b3bd71522a92393c5458`.
They predate this branch parent and later integrated crop/lighting changes; compare
material appearance cautiously rather than attributing all pixel changes here.
For new views run `python3 docs/testing/next-textures/run_native.py after` with
`ROADDASH_TEST_BINARY` set to the frozen integrated release executable. Runner
hashes that executable and content, using the same five hall review cameras.

Original generator SHA256 (formatted source):
`84bdfe4fc64c29deb4f6a35a90ec8820a75423f4c3e25f68cf00e8e9b705206c`.
Register `src/world_gen/metropolis/hall_surface.rs` with the existing project's
original procedural-source provenance convention during root integration. No
file-based runtime textures are added; generated maps stay in Bevy image assets.
