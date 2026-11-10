# Pass03 handoff

Ready for parent review/commit; no agent commits. Worktree only:
/home/imad/Documents/ChatGPT/RoadDash-polish, codex/gameplay-graphics-polish.
Previous3c88ba368dbc50ec9d12e4274636666476285f4a. Do not begin04 before review.

Implemented one serialized C2 knot elevation profile in authoring/planning/bake,
rr_core interpolation, rr_content finite/grade/parity/field-area validation;
terrain/lanes/grid/traffic consume compatible heights. Farm rises→climbing
approach→long city slopes/crests→urban finish. Source elevations12.60–62.48m,
max sampled grade2.996%. No hills district. Frozen approved progress prevents
height-driven furniture shifts (position roundoff≤2.1e-12m only).

Approved source XZ/blocks/IDs/ROW unchanged; runtime buildings/streets/junctions/
parked cars/bounds/field boundary shapes exact. Field visible/contact patches
and start apron tiled32m; original field rings retain fences. Invalid source
fields must be repaired BEFORE clipping: strict union/area tests exposed and
fixed initial bad tiling, failed receipts retained. No facade sources/materials/
atlas/mips changed; source manifest/provenance hashes updated. No physics tuning,
UI artwork, dependencies or external media/code. ADR107. ENV-001 partial evidence;
README/PROGRESS synced; no new IDs, movement/count changes or broad closures.

419 tests pass,1 existing ignored. fmt, strict all-target locked Clippy, locked
debug/release builds, audit,5roadmap tests, progress check all pass.16Metropolis
Python+4race-first tests pass. Exact marked-lane/grid pavement sampling every5m
both routes: worst9.457mm; all triangle centroid/edge samples≤9.56mm pavement /
9.47mm fields. Old measurements28.68/62.95mm. Actual foot crossings, wall blocking,
branch gates/rejoin, navigation and model differential recovery regressions pass.
Initial support fixture wrongly assumed arbitrary race-line±4.5m means asphalt:
route0/4370/+4.5 lies91mm outside simplified road. Corrected to actual marked
lane API +race center +grid lanes; footprints not widened or tests loosened.

Fresh native metro exit0; both recovery and NPC rigs exit0. Reverse−1.898m/s,
four pickups0.0000m error,15-racer retry/audio pass. Paired18-case lab exit0/all
finite including15crashes per rig. Actual farm/entry/finish/remount images viewed.
Legacy Valley world survey completes15sections/657roots and geometry assertions,
but exits1 solely failed performance target: avg16.280/p9519.208ms>16.7. Preserve
that failure; runner resumes recovery after this specific completed report.

Metro census +5636render/collision triangles (~0.194% render),+2chunks/+2static
batches.19068buildings/1494props/122parked/96walkers unchanged. Current isolated
survey avg19.978/p9538.934ms, NOT16.7ms compliant, not GPU/full-route certification.
Before VISUAL survey overlapped Python bake: its timings are contaminated and
MUST NOT be used for a causal comparison. Later native starts guard against
compiler/baker overlap and retain process snapshots. No concurrent games.

Deterministic rebake printed exact equal hashes but tool session returned143
post-output; both facts preserved. Independent current-file hash check passes:
metropolis f212ce8e64c4bc1582afc1bbd0f64ff38d1b760efe62cd8b20e2fe5e01068a64.
No clean rebake-exit claim. Normal authoring bake earlier exited0. Retained
representative real screenshots/receipts total~40MiB; redundant PNGs removed.

Unmet: human slope/art/feel review, uninterrupted full races/merges/traffic,
every property/confined clearance, performance and Windows/controller/hardware.
Native scripts reposition and inject results; not complete manually driven races.
Original dirty RoadRash UI/saves untouched. Shared target release NOW contains
pass03; later agents must rebuild their changes. No push/merge.

Proposed commit: feat(world): author shared smooth Metropolis elevation
Changed files are in git status; docs/testing/polish-12/pass03/README.md provides
links and precise measurements. Parent must review diff and commit exact SHA.
