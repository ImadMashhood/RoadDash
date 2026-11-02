# One-kilometre city preview — 2026-11-01

Scope: isolated architectural/streetscape study; no racing-map, physics, input,
saves, audio or ragdoll implementation changed. Branch codex/city-kilometre-preview.
Run instructions/config: [preview](../../../art/buildings/city_preview/README.md).

## Implemented and checked

- Exactly 1,000 m of main avenue, ten blocks and two parallel side roads.
- 414 stable seeded lots, continuous touching frontage on all four rows, 3–6
  floors, six BUSS-family concept palettes, recessed openings, canopies, roof
  plants, parapets, trim, curbs, crosswalks, lamps and sparse trees.
- 61 batched native triangle meshes, 141,631 input boxes; geometry is actually
  rendered. These counts are not performance acceptance or production asset counts.
- Config/seed and complete lot bounds are recorded in
  art/buildings/city_preview/generated/layout.json. scene.json is regenerated,
  not committed. Tests check exact block closure, clear roads/rear courts, stable
  IDs/seed variation, finite supported geometry, family coverage, invalid input
  and outward mesh winding/bounds.

## Actual checks

- `cargo fmt --all --check`: pass.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: pass after
  correcting a complex tuple alias and collapsible conditional in the new viewer.
- `cargo test --workspace --locked`: 164 passed, zero failed, one pre-existing
  ten-minute soak ignored. Includes one new preview mesh test and existing
  differential model/physics/recovery tests. Preview binary retested after final
  camera/capture-only corrections: one passed.
- `cargo build --locked`: pass; `cargo build --locked --bin city_preview`: pass.
- `python3 -m unittest discover -s tools -p 'test_*.py'`: 29 passed, including five
  new city authoring tests. Prior four facade tests remain passing.
- `python3 tools/audit.py`: pass (declaration/hash audit, not legal certification).
- Native `target/debug/city_preview --capture docs/testing/city-preview`: exit 0,
  four 1600×971 PNGs (desktop-adjusted client area; requested window was 1600×1000) and native-report.json. Inspected all views; corrected a
  junction camera initially inside a building before capturing final evidence.
- Existing game `target/debug/road-dash --smoke-test`: exit status recorded in
  verification.json; initializes four playback sinks. No audio-listening or
  Windows hardware certification claimed.

Native files: 01-avenue-overview.png, 02-street-height.png, 03-junction.png and
04-block-grid.png. These are Bevy screenshots, not image-generation mockups.
Timing in native-report.json is bounded wall-frame time from 389 frames, with
vsync, screenshot IO and debug build. RTX3080 Linux host. No GPU-only timings,
full-race cost or 60fps production gate is claimed. Production LOD/party-wall
optimization remains open; current sample does not demonstrate 16.7ms p95.

## Research / remaining acceptance

Read phiresky/procedural-cities at f0c2c86e4289cea63e05fb55a78994c5a8ef99f5:
mapgen.ts/config.ts road growth, quadtree/local constraints, snapping and paper's
split-grammar discussion. Root AGPL3 verified; source itself identifies additional
upstream adaptation. Study only, no code/media copied. Record CITY-REF-01 in
third_party/REPOSITORIES.md. CGA paper:
https://doi.org/10.1145/1179352.1141931 . ADR039 selects explicit grid/lot splitting
for predictable street fronts rather than introducing a road-growth dependency.

This is a shared rectangular-opening vocabulary, not a faithful final mesh for
each concept. Material maps/brickwork/sign art, arches and family-specific forms,
corner/party-wall cleanup, finished vegetation, remaining catalog families, live
traffic/pedestrians, collision/navigation, independent LOD and Valley integration
remain open. README V2-04-003/004/006 updated as partial; no checkbox changed, no
new task IDs needed and no major-part exit gate closed. User art review remains.
