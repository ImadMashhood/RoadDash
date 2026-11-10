# Pass08 handoff

Work only RoadDash-polish / codex/gameplay-graphics-polish. Previous commit
669cb094f8d1930b81f783c350275f65bbeabac1. Parent reviews/commits; no commit made.

Offline authored approach palette reuses current24families. Five-storey-or-less
buildings before northing1800m blend roadside business→workshop→residential→
original city pool at−180/250/650/1100/1800m stations. Original source tower role
is relaxed only for that low-rise approach. All taller/later assignments compare
against original hash/pool rule exactly. Groups of three authored footprint
indices share use selection; per-ID family variation remains. No claim that
index groups guarantee geometric adjacency.

Full tools/bake_metropolis.py rebuild from unchanged approved LAYOUT. Exactly
1,338style IDs changed (278×2,519×3,194×4,347×5floors), block northings≈0–1529.94m.
Every other field in entire world equals pass07, including building shapes/
heights/envelopes, roads/ROW/access, terrain/elevation, fields/fences, vegetation,
traffic/props/walkers/IDs. Both Metro pixel atlases and source far manifest are
byte-identical; palette provenance updated. No new source art/textures/materials,
no simulation/rig/controls/saves/audio/race or legacy-world changes. ADR112.

20Python tests across six authoring groups +383-band atlas validation pass;
new rr_content authoring station/pool validation with negative cases. Existing
source-family coverage and all24×3×5crop checks pass. Preservation receipt lists
all changed IDs. Native before/final survey and final recovery exit0; reverse
−1.898m/s, two pickup errors0.0000m; shader/pipeline/panic logs clean. Fresh release
and assets frozen, no native/compiler/baker overlap. Entry03 and approach04/06
inspected; parent inspected entry03 too. Not user art approval.

Wall frames before19.029avg/36.966p95 → after19.288avg/34.572p95ms; both miss16.7.
Entry03 actual details2708→4862meshes and1495690→1298748triangles: higher meshes,
lower triangles, all views within6000/1500000caps. Resident geometry census
identical. Whole survey mainly measures final view; retained one-second approach
samples may straddle camera moves. No causal speedup/GPU/affordability claim.

README+PROGRESS synced; no IDs/moves/closures/count changes. PALETTE-01/V2-04-003
still open for complete site grammar, detached farmstead art, human transition/
racing-speed review, long streaming and target hardware/performance. No external
repository/media intake; original local sources only. Source ASSETS hashes updated.

Next09 owns atmospheric depth; do not hide missing geometry/LOD with fog. Keep
current lighting/shadow/material/road and approach presentation. Source facade
changes still need atlas/hash/mips+both Metro crops; this pass only remaps existing
family styles. UI art paused. Stop08.

Proposed commit: feat(world): blend low-rise farm-to-city district facades

Final mandatory checks: fmt/strict locked workspace all-target Clippy/
427tests(one existing ignored)/locked build/audit/5roadmap tests/progress all exit0.
Locked release also passes; model180tick/body/joint differential and support/
recovery tests pass. No new legacy native/paired rig sweeps for this source-style
only change. `git diff --check` passes. Git12tracked modified+3untracked top-level
entries. No commit/push/merge. Final receipts in checks.json/authoring-checks.json.
