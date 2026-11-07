# Full architectural draft catalog — 2026-11-01

Branch: codex/all-building-models. Current user request: show the existing map,
then author all remaining models. Previous six-model evidence is retained in
../building-meshes; the original 18 GLB hashes remain unchanged.

## Delivered and verified coverage

146 IDs: 100 facades, 28 standalone structures, 18 surface modules. Three geometry
LODs each, 438 self-contained GLBs. 25 editable sheet source scenes (one accepted
first-six source plus24 new), original material library, individual actual mesh
renders and25 labeled render sheets. Explicit per-ID design parameters derive
from visual inspection of every concept sheet. Actual modeled features replace
image cards; no original retail facade pixels are embedded.

The 1km isolated Bevy preview now places all100 facade IDs across416 touching lots,
1,360 modules, with side streets, sidewalks and rear courts. A25-page native
catalog survey imports all146 models. Details load on demand, preserving uniform
scale and frontage-facing orientations. Surface modules/structures stay out of
random facade frontages. See map/ and native-gallery/ for actual captures.
Gallery page order follows catalog_designs.json in groups of six; this differs
from source-sheet boundaries after the four-item facades_17 sheet.

## Checks and receipts

- fmt.txt, clippy.txt, build.txt: required locked Rust checks.
- rust-tests.txt: workspace regressions, including existing physics/recovery.
- python-tests.txt: 38 tests, including exact catalog/type/reference coverage,
  preserved first-six hashes, actual-model map/no-image-card assertions.
- glb-validation.json/.txt: all438 binary files, finite positions/normal/UV streams,
  indexed triangles, positive volumes, embedded512-square authored tiles,
  standalone scene structure, LOD reduction and source/output SHA reconciliation.
- audit.json: repository declaration/provenance audit, not legal clearance.
- native-map.txt and map/native-report.json: five captured map/mesh views.
- native-gallery.txt and native-gallery/native-report.json: all25 pages captured.
- default-game-smoke.txt: farm crash/remount, city/hills, hill crash/remount, finish.
- package.json: final downloadable archive integrity/count/hash receipt.

The initial Python run reached coverage before structures_08 finished and failed
its four missing-ID check; rerun after completion passes. Initial Clippy flagged
large Bevy system signatures; narrowly documented system-parameter exceptions
were added and strict Clippy rerun. A pagoda roof cross-section orientation error
was corrected and affected sheets explicitly rebuilt before final reconciliation.

Map wall-frame sample on Linux RTX3080 at1600×971: median9.41ms, p9515.49ms,
479samples. Catalog survey: median/p95 in its report, 2,279samples. These include
vsync, scene loading/replacement and screenshots; they are neither GPU timings
nor production racing/Windows performance approval.

## Checklist, decisions and limitations

MESH-01/02 draft coverage is implemented. Those broad tasks remain unchecked
because artist/user fidelity review and production intake are unfinished.
V2-04-003/004/006 also retain full world-grammar/art/performance/integration gates.
No new roadmap IDs or repositories/dependencies were necessary. ADR042 records
batched typed geometry, separate surfaces and lazy preview loading. Asset
provenance includes every GLB and source; reference-derived visuals remain draft,
not a release clearance claim.

Materials are intentionally cleaner than concepts; wear, exact silhouettes,
small trim intersections, side/back variation, material polish, colliders,
foot navigation, canonical Valley/world_gen integration and production profiling
remain. Existing racing/ragdoll source, dependency lockfile, saves and inputs were
not modified by this catalog work. No next major part automatically started.

## Reproduce / download

See ../../../art/buildings/mesh_assets/README.md for all authoring/viewer commands.
The archive in /home/imad/Downloads/RoadDash_Complete_Building_Catalog.zip contains
all438 GLBs,25 editable scenes, material library, manifests, renders, reference
concepts, original authoring/validation scripts and this evidence guide. It excludes
the Blender executable and temporary scene backups. First-six ZIP remains intact.
