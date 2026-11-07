# Actual facade correction — 2026-11-01

The preceding geometric/color-only street was rejected by the user because it
did not use the new artwork. Its images/receipt are retained in rejected-geometric;
none of those images count as facade acceptance. This receipt supersedes that
preview. Branch: codex/city-kilometre-preview. No default-game physics, world,
ragdoll, input, audio or save implementation changed.

## What is actually loaded

- 100 new enhanced facade IDs, all 17 facades_*.png sheets, read unchanged from
  art/buildings/realistic_sheets. The separate 46 structures/surface IDs are not
  treated as city facades. No original retail PNG is read by this viewer.
- facade_assets.json maps each ID to its exact source PNG SHA256, image dimensions
  and body UV rectangle. Pixel scanning creates metadata only, never edited images.
  Generator validates source hashes and fails on changed art or invalid UV bounds.
- 416 buildings along the exact kilometre/ten blocks; all 100 source IDs selected
  before catalog reuse. Actual image pixels texture fronts and end/rear returns.
  There is no generated window lattice covering the artwork.
- Continuous, UV-preserving shallow relief: cornices plus hand-authored first-six
  window, door, canopy and bay profiles. Basic shells and roofs provide depth.
  62 shell/furniture batches (5,044 boxes), 17 texture batches (22,974 quads),
  including the proof gallery. These are design-study counts, not final asset counts.
- 05-actual-facade-proof.png shows BUSS1, BUSS10, BUSS2, BUSS4, BUSS5, BUSS6 from
  the first actual sheet in that order. Textured source signs are recognizable.
  Caught mirrored lettering in an early render, corrected basis/winding, added
  a two-sided-road orientation regression, then recaptured the evidence.

## Actual checks

- cargo fmt --all --check: pass.
- cargo clippy --workspace --locked --all-targets -- -D warnings: pass.
- cargo test --workspace --locked: 165 passed, zero failed, one existing ignored
  ten-minute soak. Includes unchanged model/physics/recovery regressions plus
  two preview mesh tests (one new textured UV/degenerate-geometry test).
- cargo build --locked and --bin city_preview: pass.
- python3 -m unittest discover -s tools -p 'test_*.py': 33 passed, including nine
  city authoring tests: exact block closure, road/rear-court clearance, seeded
  coverage, all 100 IDs visibly assigned to textured surfaces, exact source hashes,
  bounded UVs, relief, finite support, catalog-change rejection and non-mirrored
  lettering. Tests explicitly forbid the rejected generic window/sign geometry.
- python3 tools/audit.py: pass. Production asset rights remain unverified; local
  preview textures have false shipped flags. The custom tests separately verify
  all preview image hashes, since the production audit intentionally skips drafts.
- target/debug/city_preview --capture docs/testing/city-preview: exit 0, five
  native PNGs and timing report. Final views inspected, PNG decode/dimensions/hash
  evidence in verification.json. These are Bevy screenshots, not new AI renders.
- Source PNG checks: all 17 files match the original enhanced-art manifest hashes.
  No bitmap edits, regeneration, retail-source import or new dependency.

Actual client captures: 1600×971 (desktop adjusted from requested 1600×1000).
479 wall-frame samples with vsync, debug build and capture IO; latest timing in
native-report.json. Linux RTX3080 only, not GPU-only timing, active-race cost,
Windows certification or production 60fps acceptance. The prior default-game
native/audio smoke belongs to the earlier receipt and was not rerun for this
presentation-only correction; current full simulation/model tests passed.

## Remaining / checklist

This is actual facade artwork on textured relief shells, not detailed all-family
3D reconstruction. Unlit fronts retain baked lighting; existing AI sign mistakes
remain. Gable background cutouts, full relief/silhouettes, dedicated side/rear art,
real roof profiles, metric scale approval, full production models/materials/LOD,
traffic/pedestrians, collision/navigation and Valley integration remain open.
The generic tree/road dressing is unchanged. New facades are local preview media,
not publicly pushed production assets; derivative-art rights still need intake.

README/progress V2-04-003/004/006 updated as partial. No new task IDs, checked
exit gates, repository dependencies, commits, push or merge. ADR040 records the
correction. CITY-REF-01 research from the preceding session is unchanged; no new
repository investigated or external source copied in this correction.
