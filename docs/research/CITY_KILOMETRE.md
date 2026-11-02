# Route-first procedural city study

The immediate goal is an inspectable kilometre, not a new unrestricted city
simulation. Established procedural-city research separates street layout from
block/parcel subdivision and building shape rules. CGA's split/repeat vocabulary
is a good fit for constant-size windows and floors:
https://doi.org/10.1145/1179352.1141931 . phiresky's source/reference survey shows
priority-queue growth, intersection snapping and local quadtree checks; this is
more relevant to later irregular neighborhoods than to the requested grid.
Exact source/license findings: [CITY-REF-01](../third_party/REPOSITORIES.md).

## Selected pipeline

1. Reserve the race avenue first; define side roads and intersections in metres.
   Never let random building choices determine the playable carriageway.
2. Carve explicit blocks and side streets. A 100 m pitch with a 12 m cross street
   leaves 88 m of developable frontage. Two rear-to-rear building rows produce
   side-street depth, not a single row of scenic shells.
3. Partition each row by integer bay counts. A common small bay-width adjustment
   closes the block, rather than stretching a random final building/window.
4. Choose a family and floor count from a seeded generator. Stable IDs identify
   block, side, row and lot. Keep IDs separate from individual decorative boxes.
5. Build openings, trim, storefronts/canopies, end walls and roofs as real meshes.
   Generated angled PNG concepts supply design references; they are not UV maps.
6. Merge geometry by block/row into a handful of mesh draws and share material.
   This preview does not yet have independent render LOD or collisions.
7. Judge actual street-height and overview captures before committing to all
   catalog families or replacing the supported Valley world.

Default study: 1,000 m, 414 buildings / 61 mesh chunks, 14 m four-lane avenue,
3.2 m sidewalks, parallel side streets at ±68 m and 13.6 m rear courts. Six BUSS
concept families are interpreted with an intentionally shared preliminary mesh
vocabulary. Floor counts 3–6; 40 m tree spacing. These are authored modern metric
choices, not measurements recovered from the original sprites.

## Production follow-through (existing V2-04 tasks, not completed here)

- Author distinct high-quality family meshes and original material atlases from
  reviewed concepts. Add arches, stone/brick courses, storefront doors and signage,
  corner/end conditions and correctly exposed party walls. Include all approved
  catalog families by district/role, not randomly every family in one city.
- Add bounded correlated height/role rules and landmarks to reduce repetitive
  rooflines while retaining touching frontage. Grid density should be reviewed
  at riding speed; an allocation count alone is not visual quality evidence.
- Integrate through world_gen's existing canonical section/lots/pads/rights of way.
  Curved route frames, corners, full-lot support and intersections need dedicated
  validation; do not replace the established quota/ID system with this toy grid.
- Separate coarse colliders and foot-accessible navigation from render meshes.
  Add independent near/mid/far LOD, chunk-local origins and streaming/culling.
- Restore city life through existing vehicle/pedestrian systems, then benchmark
  the active race and on-foot recovery. Current native preview has no simulation;
  its ~32 ms p95 sample does not close the 16.7 ms production gate.

Run and evidence: [city preview](../../art/buildings/city_preview/README.md),
[native test receipt](../testing/city-preview/README.md). ADR039 explains isolation
and rollback. No new source dependencies, external source copies or raw original
textures were imported into this viewer.

## User correction — actual artwork binding

The palette-only result above was rejected: concept-inspired colors did not meet
the instruction to use the new facades. Current code now maps 100 actual enhanced
facade images (17 unchanged sheets) directly onto textured relief surfaces in
416 buildings. UV/body bounds and source hashes are tracked, and a native proof
gallery shows the first six for comparison. No new image is generated to replace
the selected art. The 3D concept render sheets remain mesh-shape references; the
flat enhanced sheets supply the actual texels. Shared fronts use baked/unlit
lighting, with shallow cornice and first-six window/canopy/bay depth. This does
not claim finished 3D reconstruction; gables, roofs, side/rear art and full-family
relief still need modeling. ADR040 supersedes ADR039's color-only front approach.
Current receipts: ../testing/city-preview/README.md.
