# Part04 — district facade palette and full/coarse identity

2026-11-08; focused local branch `codex/part04-district-facades`, inherited
`7465f98` (Parts01–03). Bounded PALETTE-01 progress; **all29 Part04 requirements
remain open**, no new task IDs, no Part05 implementation and no push/merge.

Metropolis expands12→24 existing original facade families. Five frontage pools
follow the approved planning district; >=13floors and every interior tower use
six office families. Assignment uses stable BLAKE2b over building IDs. Authoring
rejects missing/unknown/empty/duplicate pools, non-facade models and invalid tower
rules. Every selected design is actually assigned in the19068building output;
coverage by district/role is recorded in palette-tests.txt. These are original
existing editable storey-kit meshes, not newly downloaded art or placeholders.

Added CHINA13/14, CLSTGA12/13/14, OFFC11/12/13, RUNDWN12, VICT13/14 and WHSE12.
Existing family indices are retained. Detailed sourceLOD1/2 stays resident and
material-ready before Loading exits. The existing caps remain28selected faces
plus8retiring slots,6000actual source primitive instances and1.5Mtriangles,
including retiring geometry. Envelopes/roofs/collision are independently resident.

A discovered coarse mismatch omitted crown bands and used uniform4m bays/3.2m
storeys. Full and coarse now share fitted bay/ground/upper/crown dimensions; the
expanded atlas includes all72source bands and5actual source-derived mip crops.
BUSS4 upper intentionally maps to CLSTGA11 except one detailed RADIO sign per
frontage. The provenance explicitly records this exception;24families is not a
claim of24unique upper-band sources. Source models/materials and the canonical
128design atlas remain untouched. Per-source digest/mip/pixel checks independently
reconstruct every delivered tile, rather than merely trusting a new output hash.

The expanded resident input cost is explicit: LOD1/2GLB files7,156,672→15,467,928B;
source triangles94,063→197,303; coarse atlas RGBA with mips2,095,104→6,285,312B.
The atlas is1024×1152 (below the existing2048×4096 global atlas),128px cells/16px
wrapped gutters/one-pixel final mip guard. Shared texture files remain unchanged.
File sizes and CPU vertex bytes are not allocator/VRAM measurements or a
performance improvement. Native readiness/draw/frame evidence is recorded below.

`preservation-and-residency.json` proves exact before/after equality of the entire
runtime manifest after removing only `style`: all19068footprints/triangles/IDs/
heights, both routes/gates/grid, ground, props/traffic/fields are preserved.
17329style values change. Original Valley quotas/IDs, legacy City and accepted
physics/recovery/input/audio/save files are untouched. ADR082 records decisions.

## Reproduce and verification

Authoring uses the **existing** `/tmp/roaddash-layout-venv/bin/python`, with
Shapely2.1.2, Pillow12.0.0 and numpy2.3.4 (authoring-environment.txt). System
Python lacks Shapely; no global packages were installed. Run:

```
/tmp/roaddash-layout-venv/bin/python tools/bake_metropolis.py
/tmp/roaddash-layout-venv/bin/python tools/test_metropolis_facades.py
/tmp/roaddash-layout-venv/bin/python tools/verify_facade_lod.py
```

Native commands/results, measured frame times and captured review views are
recorded in RESULTS.md. The independent three-test Python suite covers invalid
palettes, every placed district/tower assignment and all72bands×5mips. Rust checks
palette/runtime/provenance order, style validation,24family storey fitting and
accepted model differential physics/recovery regressions. Mandatory format,
Clippy, workspace tests/build, release build, asset audit, roadmap/progress and
diff checks have individual receipts. No manual complete race, external visual
approval, Windows/controller hardware, full-course memory/GPU profiling or legal
clearance is implied. No external repository/media was adopted; local original
catalogs and installed engine APIs suffice. ASSETS.csv hashes only changed
registered files and the new original palette.

## Remaining per-task disposition

Every task below remains unchecked; earlier technical evidence is retained.

| Task | Remaining acceptance after this pass |
| --- | --- |
| PALETTE-01 | Expanded24family district/height coverage is technical progress; remaining eligible catalog, unique signage/corner/foundation art, full/coarse user acceptance, extended streaming and performance remain. |
| LAYOUT-05 | Approved topology unchanged; whole construction, junction traffic/playtest and performance acceptance remain. |
| LAYOUT-04 | Approved heights/detail tiers unchanged; final skyline/native art acceptance remains. |
| LAYOUT-03 | Approved one-fork farm→long-city layout unchanged; final construction/native art acceptance remains. |
| WORLD-EXTEND-01 | Existing Metropolis integration retained; wider support/performance acceptance remains. |
| LAYOUT-02 | Historical two-split plan superseded; city-center goals require final native acceptance. |
| ROUTE-SPLIT-01 | Existing one-fork routes retained; production merge/traffic/human playtest remains. |
| LAYOUT-01 | Historical corridor plan superseded; final street/property/corner art acceptance remains. |
| LOD-03 | Source bands/fitting improve; prolonged high-speed/reverse transitions and user visual acceptance remain. |
| V2-04-001 | Complete local reference inventory/provenance acceptance remains; no retail files imported. |
| V2-04-002 | Original production texture/fidelity and art/rights acceptance remain. |
| V2-04-003 | District rules expanded; full terrain/site/transition grammar and approval remain. |
| V2-04-004 | Existing detailed drafts reused; production whole-block/corner/material/art acceptance remains. |
| V2-04-006 | Measured bounded native samples only; full-course racing/memory/platform targets remain. |
| V2-04-007 | Umbrella art/world/performance/user exit gate remains unmet. |
| KIT-03 | Valley infill and bespoke parking/service/farm geometry unchanged and unfinished. |
| MESH-01 | Existing94draft facade sources preserved; artist/user fidelity approval remains. |
| MESH-02 | Existing28structure/18surface drafts preserved; final materials/type-aware production intake remains. |
| P11-002 | Full farm/cliff/coast/woodland art bible remains. |
| P11-004 | Complete modular roads/props/landmarks family production remains. |
| P11-005 | Full world/encounter/foot-accessible pipeline acceptance remains. |
| ENV-001 | Broader spline/shoulder/vertical-curve/start-finish authoring acceptance remains. |
| ENV-002 | Full biome art/audio transition acceptance remains. |
| ENV-003 | Broad critical sightline/lot/terrain validation remains. |
| ENV-005 | Broad bike/on-foot trap/barrier validation remains; cosmetic detail adds no colliders. |
| ENV-007 | Complete WorldZone schema/fields/validation remains. |
| ARCH-005 | Complete authoring bundle and separate collider export contract remains. |
| RIGHTS-007 | Broader roadmap production authoring/rights verification remains. |
| P02-004 | Existing Metropolis split/merge preserved; complete generalized branch authoring acceptance remains. |
