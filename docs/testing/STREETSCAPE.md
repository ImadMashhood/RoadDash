# City streetscape — 2026-10-31

CITY-03 addresses the user's sidewalk, street-tree and building-height correction.
Main sidewalks use flat normals instead of averaging curb and top faces, a muted
rough concrete material, 2 m sampling and subtle expansion joints. The two
continuous meshes retain solid rideable curb ramps.

Original procedural trees stand in flush beds near the curb at 22 m intervals,
excluding junctions and structural blockers. Trunks and conservative crowns are
solid even when visuals cull. More than 150 sites are checked; each trunk leaves
over 2 m of sidewalk width available on its building side. Trees are intentional
obstacles, not decoration that riders pass through.

Urban buildings gain 60% more floors, rounded up, via city_height_scale=1.6.
Supplemental buildings first derive a valid whole-floor base from their archetype.
All roofs, windows, collision and shared-edge infill follow the resulting height.
The 247 canonical lots, 410 supplemental buildings, IDs and footprints remain.
Rural heights and original planning snapshots remain unchanged.

Implementation: rr_content construction validation; world_gen layout,
neighborhood, surfaces, streetscape, module registration and regression tests;
valley construction content. Documentation and provenance are updated alongside
the change. No files removed, new dependencies, external assets or new GitHub
investigations. Original procedural source is recorded in ASSETS.csv. ADR-024
records alternatives, tradeoffs and rollback.

Acceptance includes automated flat-normal, solid-trunk, walking-clearance and
whole-floor/unchanged-lot checks, plus the existing physical-world, controls,
combat, recovery and race-completion regressions. Native survey screenshots and
scripted smoke evidence are stored in streetscape/. These do not certify Windows
hardware, human subjective handling, or finished production architecture/art.

## Results

115 automated tests passed, one optional soak ignored. Workspace Clippy with
warnings denied, formatting, locked debug/release builds, four source-input
checks and provenance audit passed. Full-world inspection passed. Native survey
captured all 15 sections: **9.66 ms p95 at 1920×1080** on this Linux/Vulkan host,
within its 16.7 ms target. Reviewed road-height and elevated city captures show
flat pavement, curbside trees and a taller continuous skyline.

Native smoke passed farm crash → remount → city/hills → hill crash → remount →
finish; audio initialized with four playback sinks. Benign device mapping,
Vulkan extension/presentation and monitor-selection warnings appear in receipts;
neither native run failed.

CITY-03 is checked in README and PROGRESS. No additional roadmap tasks were
introduced. Production art and broader platform/human validation retain their
existing open gates. Next step is user review of the new city appearance before
further art direction; no new development part was started.
