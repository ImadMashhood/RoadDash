# Valley Run structural world — B00–B02

2026-10-30. Implementation branch: `feat/base-world-foundation`.
Baseline: merged main `78d6a828268d85abd73df28b72c23df0407c4ee3` (PR #5),
with the same source tree as `bd6712cd45f7841fcf6f9153de6cdbe51ff41088`.

The user requested starting procedural construction of the first map and review
of three supplied design files. This session implements the first dependency
stage: measured baseline, validated structural content, quota allocation and
station/district resolution. **It does not claim that 247 new buildings or the
finished procedural map have been placed.** Existing scenery/gameplay remain.

## Input review and decisions

The supplied [PDF](source/RoadDash_Base_World_Building_Terrain_Plan_v1.pdf),
[JSON](source/RoadDash_Base_World_Spec_v1.json) and
[CSV](source/RoadDash_Base_World_Sections_v1.csv) are retained unchanged.
Their counts reconcile: 15 sections; 247 buildings; 177 near/70 secondary;
35 farm, 63 residential, 46 commercial, 73 urban, 30 hillside; 19 archetypes.
They are proposed original RoadDash design targets, not an original-game census.
Input identities and SHA-256 digests are in [source manifest](source/MANIFEST.json).

The PDF's suggested agent handoff and imperative language are design guidance,
not independent authorization to alter unrelated systems. Its proposed B00–B02
first stage is selected here because placement depends on a correct data model
and road-station binding. B03–B10 remain visible, unchecked requirements.
The PDF's source snapshot `78d6a828` matches the current merged source tree.
Its exclusions concern NEW work: existing people/cows/traffic/trees/props are
preserved, rather than removed from the game.

No new external repository was needed for this foundation. Existing Bevy mesh,
Rapier and serde infrastructure already suffices. No engine migration,
third-party geometry library, C++ source or proprietary media was introduced.

## Runtime contracts

- [World sections](../../content/world/valley_world_v1.json): schema/generator
  version 1, seed 17031, exact source quotas, seven district types, referenced
  lot rules and a 30 m presentation blending band on each side of a boundary.
- [Archetypes](../../content/world/building_archetypes_v1.json): all 19 supplied
  identities/quantities/height ranges, typed family, structural footprint ranges,
  floor height and flat/pitched/sawtooth roof choice.
- Source stations, identities, descriptions, quotas and heights are preserved.
  Footprint ranges, floor height, intermediate district lot ranges and access
  widths are authored provisional parameters, not measurements from the PDF or
  the original game. PDF example lot ranges/pad tolerances guide applicable rules.
  B05/B07 must prove actual fit, support and access before placing geometry.
- `rr_content::world` validates without Bevy/GPU: versions, finite dimensions,
  contiguous exact station boundaries, unique IDs, row/family/registry totals,
  known lot-rule references, bounded counts, height/footprint/access constraints.
- Allocation consumes exact global archetype quantities and exact per-section
  family/row counts. Versioned xorshift64* and Fisher-Yates use explicit integer
  operations and ordered pools. IDs are course/generator/section/ordinal, stable
  under repeated runs. No per-frame RNG, entity spawning, terrain or collision
  changes occur. Seed/algorithm changes require reviewed content/version policy.
- Allocations are planning records with no transforms/footprints/foundations.
  Their FNV-1a64 fingerprint covers assignments only and is not a completed-world
  geometry hash or cryptographic content-integrity check.
- Bind to the matching open track ID and seed. Authored length 6847.3 m may differ
  from baked arc length by at most 0.25 m. Only the last endpoint is snapped to the
  baked length; internal stations are never scaled or derived from world Z.
  Reject a different course, closed route, seed mismatch or excessive length drift.
- Station ownership is right-continuous: S02 owns exactly 450 m. Start belongs to
  S01, final baked endpoint to S15; nonfinite/outside-route lookups are rejected.
  Smoothstep blending affects debug/presentation colors only, never quotas or
  existing three-sector gameplay classification.
- The built-in Valley plan initializes once at startup. Other track IDs retain
  their current behavior without a mismatched overlay. Edited built-in data is
  embedded on rebuild, matching the project's current content packaging pattern.

## Use and evidence

```sh
cargo run --locked -- --validate-world content/world/valley_world_v1.json content/world/building_archetypes_v1.json content/tracks/valley.json --output world-validation.json
cargo run --locked -- --inspect-world content/world/valley_world_v1.json content/world/building_archetypes_v1.json content/tracks/valley.json --output world-census.json
```

Both commands initialize no window/audio/GPU and exit 2 on invalid inputs or
report write failure. `--validate-world` checks and allocates. `--inspect-world`
also builds the existing environment through the game's real setup function and
measures building masses, mesh/collider entities and road/terrain chunks.
The report labels target allocations and measured legacy entities separately.
Each existing building mass has a diagnostic ID/station tag; roof/window/tree
entities are not counted as separate buildings. These tags do not change meshes,
colliders or placement. Triangle counters cover existing ribbons/terrain only;
mesh entities are not draw-call counts.

During play, **F4** adds colored district edges and section-start crossbars/poles
along the actual spline, alongside the existing inspector. Color transitions:
green farms → yellow rural edge → orange suburb → red inner city → purple core →
cyan hill departure → blue hills. Racer minimap shirt colors are unaffected.
`ROADDASH_WORLD_REPORT=/path/report.json` optionally exports the startup census.

See [test receipt](BASE_WORLD_TESTS.md), machine-readable reports and native
before/after screenshots in `evidence/`. Baseline generation runs twice in tests;
sorted building IDs/transforms/sizes/collider flags must compare exactly. Source
CSV/JSON and runtime conversion are independently compared in Python/CI.

## Roadmap, dependencies and acceptance

All tasks were added because the supplied base-world design introduces a new
structural pipeline. This is first-course environment work under Part 02/11,
not permission to start additional courses or automatically advance major parts.

| ID / stage | Work and reason | Dependencies | Acceptance / verification |
| --- | --- | --- | --- |
| BW-B00 | Snapshot existing course/world before replacement | current track and real environment setup | Track stats, measured building census, repeatable legacy geometry fingerprint and native before views; report/tests/screenshots |
| BW-B01 | Validated section/archetype/lot contracts and exact quota allocation | B00, supplied JSON/CSV | 15/19 registry, 247/177/70 and family totals; deterministic unique allocations; malformed topology/budgets/dimensions rejected; content/Python tests and CLI |
| BW-B02 | True-distance district resolver and blending preview | B01, baked spline | All section borders/endpoints owned correctly, continuous blend, wrong track/seed/length rejected, F4 overlay visible; core/client tests/native screenshot |
| BW-B03 | Field polygons and district ground classification | B02, shared terrain sampler | 36–48 broad farm parcels, urban/hill material masks and all 15 sections distinguishable without new decorative props; mesh/material census and native survey |
| BW-B04 | Rights of way and structural streets | B02/B03 | At least seven stable junctions, valid stubs, continuous curb/sidewalk cuts, no road gaps or blocked gates/spawns; footprint/road-ray tests and native views |
| BW-B05 | Real lots and allocation-to-parcel placement | B01/B04, terrain support | Frontages subtract reservations, 177/70 plots legally placed, no lot/road/junction overlap, secondary access retained; deterministic spatial tests and plan export |
| BW-B06 | All structural building masses | B05 | Exactly 247 rooted/identified structures across 19 types, owned roof/mass/collider and full-footprint clearance; census and collision tests/native survey |
| BW-B07 | Shared pad grading and hill earthworks | B03/B05/B06 | Corner/edge/center support, bounded local cut/fill, grade transition and render/collision match; surface/raycast tests and native slope views |
| BW-B08 | Walkable base world and escape paths | B04/B06/B07 | Farm/city alley/hill crash-to-bike paths, no sealed traps, fall/rescue policy and retained vertical travel; recovery integration runs |
| BW-B09 | Spatial chunk/LOD performance | B06/B07/B08 | Stable logical IDs/collision through presentation reload, measured representative 1080p p95 ≤16.7 ms target; LOD/visibility tests and native profiling |
| BW-B10 | Full-map survey and exit gate | B00–B09 | Complete atlas/plan/counts/geometry hashes/rights/regressions/platform receipts; all supplied appendix criteria evidenced; no release/performance claim from this first stage |

## Session report

Implementation: content validation, exact planning allocation, baked-station
binding, F4 preview, measured legacy census, CLI and CI coverage.
Files: `crates/rr_content/src/world.rs`, `content/world/`, `src/world_gen/`,
minimal environment/startup/CLI integration, tests and permanent tracking.
Research: only supplied design inputs and current local implementation; no new
repository adoption or dependency. Architecture: additive overlay and plan state,
separate from live geometry/physics, with explicit version/seed/length contracts.
Known limitations: new target masses/lots/pads/streets/ground masks/LOD are not yet
generated; current three gameplay sectors and legacy scenery remain. Next stage:
B03/B04 ground/reservations before B05/B06 lots and building massing. Git and exact
verification results are reported with the completed session, not assumed here.
