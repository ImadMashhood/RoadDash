# Actual mesh milestone — 2026-11-01

Branch codex/building-mesh-assets. Six first-sheet designs are genuinely authored:
BUSS1/BUSS10/BUSS2/BUSS4/BUSS5/BUSS6. Files and run instructions:
../../../art/buildings/mesh_assets/README.md. This does not close the broader
all-family production mesh/art or gameplay-map integration gates.

## Implemented

- Editable Blender4.5.1 scene with semantic geometric parts, bevel modifiers,
  actual openings/glazing/frames/doors/hardware, arches/canopy/bays/letters,
  segmented masonry, side/rear detail and roof equipment.
-18 standalone self-contained GLBs,3geometric LODs per model, joined mesh per file
  with indexed material primitives and embedded original512×512 material tiles.
  No facade image/card is embedded. Detailed geometry is approximately108k–122k
  triangles; coarse geometry approximately2.6k–3.2k.
- Blender renders of the modeled geometry and an independent GLB re-import/clay
  render with all images removed. The clay check exposed initially buried brick
  geometry; corrected outward placement and re-exported. Binary validation now
  confirms52.5mm masonry projection beyond backing in each detailed GLB.
- Native Bevy viewer imports the GLBs, places1,360 modules inside the1km street
  study's416reserved lots, uses shared scene handles and distance LOD/hysteresis.
  Dedicated proof gallery forces all six highest-detail assets. Complete native
  report records facade_cards_loaded=0, mesh_designs=6, glb_lods=18.

## Executed verification

- cargo fmt --all --check: pass.
- cargo clippy --workspace --locked --all-targets -- -D warnings: pass.
- cargo test --workspace --locked:167 passed,0failed,1existing ignored soak.
  Includes unchanged model/physics/recovery regressions, placement/facing/uniform
  scale, LOD selection and static glTF scene reflection regression.
- cargo build --locked: pass (both binaries).
- python3 -m unittest discover -s tools -p 'test_*.py':36pass, including3new mesh
  tests plus retained legacy facade tests. Roadmap tests rerun after adding the
  two explicit remaining-work IDs:5pass. No status lost or falsely checked.
- python3 tools/verify_building_mesh_assets.py:18/18GLBs pass header/chunk/buffer,
  index, finite position/normal/UV, metre-scale volume, image embedding, square
  tile-only materials, hash, LOD budget/reduction and visible brick-depth checks.
  Full records: glb-validation.json.
- Blender re-import of delivered BUSS1_lod0.glb succeeds; clay render saved and
  inspected. Editable source and individual/gallery model renders inspected.
- target/debug/city_preview --capture docs/testing/building-meshes: exit0,
  five native screenshots inspected; exact file hashes/dimensions in
  verification.json. Fixed an initial unregistered static glTF component panic
  by registering needed types in this isolated viewer; pinned dependency features
  and versions remain unchanged.
- python3 tools/audit.py: pass. Declaration/hash audit is not commercial-release
  rights clearance. Draft model/media records are not production-shipped assets.

Timing is a bounded debug-build wall-frame sample with vsync, asset/LOD scene
replacement and screenshot IO on Linux RTX3080. Initial cold sample showed spikes;
latest native-report.json is not a GPU/racing/Windows or steady60fps certificate.
Performance acceptance remains open. No default-game source, dependencies,
ragdoll, controls, saves or physics architecture was changed. Default-game native
smoke is rerun for this session; result recorded in verification.json.

## Unfinished / decisions / tracking

Materials and damage are cleaner/simpler than approved concepts. Small trim
intersections, repetitive side/back variations, exact silhouettes/proportions,
all-family material fidelity, artist/user acceptance and further performance/LOD
work remain. Visual meshes are not watertight physical collider meshes; no
interiors, traffic, navigation, new building colliders or Valley integration.
Other94facades and46structure/surface references are explicitly unmodeled.
MESH-01/02 added under Part04 with dependencies/acceptance/verification; no broad
exit gate checked. README/progress/task placement synchronized. ADR041 records
actual-model architecture and local rollback. Remaining mesh counts are not hidden
behind a claim that PNG concepts already constitute assets.

No new repository was adopted or source copied. Existing Blender4.5.1 binary
(b0a72b245dcf) and its bundled GPL2-or-later copyright were inspected as authoring
tooling; original generator/material/mesh code was written locally. Bevy0.17.3
installed scene-spawner source was studied for documented SceneRoot replacement
and reflection usage. No external source incorporated or library migrated.

Git changes remain local/uncommitted; no push, PR or merge performed. Prior
uncommitted art/docs and rejected-history evidence are preserved.
