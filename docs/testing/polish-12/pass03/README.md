# Pass03 — elevation and terrain integration

Following pass02 `3c88ba368dbc50ec9d12e4274636666476285f4a` on the isolated
`codex/gameplay-graphics-polish` worktree. Implementation complete; verification receipts below.

The old course already had analytic elevation: farm sine waves and a rising city
trend with three more waves, duplicated in Python and Rust. It was not a flat
map. Sampled old field triangles differed from the analytic support by62.95mm,
and pavement (including the single long starting apron) by28.68mm. These are
centroid/edge-midpoint samples, not exhaustive maxima or measured rider symptoms.

One authored JSON profile now defines broad farm rises, a climbing approach,
long city slopes and smooth crests through the urban finish. Quintic intervals
join with continuous first/second derivatives. Gentle crossfall is shared by
both branches and all terrain consumers. The source route spans12.60–62.48m;
maximum sampled route grade2.996%. No hills district or horizontal redesign.
rr_core owns interpolation; rr_content validates ordered finite knots, bounded
grades, field patch area and Python-baked path heights. Route lanes and the
extended starting grid follow land height. Traffic paths resample height after
projecting onto the actual simplified street.

The height-only updater preserves approved source XZ/IDs/blocks/ROW. Approved
progress fractions are frozen before changing heights so 3D arc-length changes
cannot shift furniture or navigation authoring. The ordinary bake emits the
shared profile, new path heights,32m apron tiles and field surface patches.
Field boundary/fence rings remain original; no new internal fences. Invalid
source field polygons are repaired before clipping, matching original bake
semantics. Coverage tests compare the unions of actual triangles, including
multi-part field shapes, and reject holes/overlap. No new dependencies/assets,
external research imports, facade material/mesh/atlas changes or physics tuning.
Existing model-derived facade hashes remain unchanged; source-layout hash updates.

Tests, native screenshots and final limits will be recorded below. Existing
ENV-001/WORLD-EXTEND/ROUTE-SPLIT gates stay open; no new IDs, moved tasks or
checklist closures. ADR107; README and generated PROGRESS remain synchronized.

Actual pavement support regression samples both branches every5m, the race
center, section-derived marked traffic lanes, and starting-grid lanes back to
−300m: maximum deviation9.457mm. It originally sampled arbitrary race-polyline
±4.5m offsets; at route0/station4370/+4.5m that position is91mm outside the
simplified asphalt (pre-existing XZ geometry). The fixture now uses the actual
street lane API; no widening through existing properties. Broad arbitrary
shoulder/off-road traversal remains a separate gate. Existing real Rapier
foot-crossing fixtures test both directions/branches and solid-wall blocking.

The first field-tiling implementation failed the strict coverage and runtime
area checks because it clipped invalid source polygons before repairing them.
Those failed suite/audit receipts are retained in initial-failed; repair-before-
clipping passes the original union/overlap thresholds without weakening them.
Source field boundaries are unchanged. The new content schema also rejects
stale Python/Rust path elevation and malformed profiles at intake.

Authoring regressions:16 Metropolis tests plus4 race-first tests pass. Both
pavement and field triangle sample errors are below10mm. Visible field triangles
increase674→6276 (+5602); the apron adds17 patches. Native census and bounded
wall-frame measurements below quantify integrated geometry/cost, without GPU
or general performance claims. All24 facade families, source bands and mip
hashes remain unchanged.

## Verification and native cost

Mandatory fmt, strict workspace locked all-target Clippy,419 workspace tests
(1 existing ignored soak), locked debug/release build, provenance audit, five
roadmap tests and progress synchronization pass. Actual support and differential
physics/recovery fixtures pass. `checks.json`, `support-test.txt`,
`authoring-tests.txt`, `elevation-python.txt` record commands/results.

Deterministic rebake printed exact before/after hashes; its tool session then
returned143 despite completed output. This anomaly is recorded, not described
as a clean process exit. Independent final-file SHA256 comparison passed;
`determinism-verification.json` and `determinism.txt` preserve both facts.

The fresh release native metro survey exits0. Render triangles2,903,477→2,909,113
(+5636,~0.194%); collision triangles2,607,483→2,613,119; resident chunks3065→3067,
static batches2529→2531. Building19068, prop1494, parked122 and pedestrian96 counts
remain unchanged; counts supplement actual triangles/coverage and native views.
New isolated survey:19.978ms average /38.934ms p95 wall-frame time. The16.7ms gate
is NOT met. No GPU/full-route/platform or added-cost affordability certification.
The before visual survey overlapped a Python authoring bake, so its timing
(19.132/40.027ms) is unsuitable as a controlled performance baseline. Do not use
that pair to claim a speedup or causal regression. New native runs wait for all
compiler/baker work to stop and retain process snapshots; no concurrent games.

Actual matched survey views cover the starting grid,900m farm,3250m entry,
both4300m alternatives, long city and urban finish. Farm and entry views were
inspected: surfaces remain continuous/readable; broad grades are deliberately
gentle, not a dramatic mountain scene. Profile plot is authored data, not a
substitute for the native captures. Existing lighting/fog/cloud/material issues
remain for later authorized passes. No human art/feel approval claimed.

Legacy Valley `--world-survey` completes15 sections/657 building roots and all
geometry assertions, but exits1 because its isolated1080p p9519.208ms exceeds
16.7ms (average16.280ms). This is a recorded failed performance gate, not a clean
survey pass. Legacy source/quotas are unchanged. The runner resumes required
recovery work after recognizing this specific completed-report performance
failure; unexpected nonzero native results still stop it. Retained JSON and log
show the original exit; no threshold was changed.


Representative native images retained (redundant captures removed to bound the
receipt size): [farm before](before/screenshots/metro-01.png) /
[farm after](after/screenshots/metro-01.png),
[entry before](before/screenshots/metro-03.png) /
[entry after](after/screenshots/metro-03.png),
[west branch](after/screenshots/metro-05.png),
[east branch](after/screenshots/metro-06.png),
[city slope](after/screenshots/metro-09.png),
[urban finish](after/screenshots/metro-13.png),
[authored profile](../../../planning/farms-long-city/elevation-profile.png).

Both native recovery rigs exit0: anatomy route0 and eleven-body baseline route1.
Each passes15-racer retry, reverse−1.898m/s, farm/city crash recovery and two
remounts with0.0000m pickup error; five audio sinks initialize. Both NPC actor
smokes exit0 with10 cow bodies and16/11 pedestrian bodies. Paired18-case ragdoll
lab exits0, all finite including15 simultaneous crashes per rig. Actual anatomy
remount image inspected with both wheel contacts true. These scripted fixtures
reposition riders and inject results; they are not uninterrupted full races.
No rig/mass/joint/muscle/control/save/audio changes. Full-course manual branches,
merge/traffic, every property clearance, confined recovery, art/feel and
Windows/controller/hardware coverage remain open.

Files: shared profile + rr_core evaluator; rr_content intake; track adapters;
planner height updater/helper and normal bake; tiled field rendering; source
LAYOUT/profile image and baked manifest; tests/provenance/README/PROGRESS,
ADR107/issues/development history and these receipts. Existing task IDs only;
no task placement/count changes. No commits, push or merge performed by agent.
