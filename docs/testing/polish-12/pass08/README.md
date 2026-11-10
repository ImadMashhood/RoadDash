# Pass08 — Farm-to-city district coherence

2026-11-09, `codex/gameplay-graphics-polish`; previous commit
`669cb094f8d1930b81f783c350275f65bbeabac1`. ADR112. Existing PALETTE-01 and
V2-04-003 evidence extended; no task IDs added, moved or closed. Parent reviews
and commits. Original RoadRash working tree remains untouched.

The approved source already contains fields/fences, public access/setbacks,
sidewalk vegetation and a1.8km height ramp. Its facade assignment still used
dense old-city families at the boundary and office designs for low capped
interior towers. An offline authored use blend now progresses from existing
roadside businesses through workshops and residential fronts to the original
city pool. Stations at northing−180/250/650/1100/1800m have normalized weights.
Only buildings at most five storeys before1800m participate. Taller buildings
and all later assignments retain the previous source-role rule exactly.

Groups of three authored footprint indices share a deterministic use choice;
individual IDs retain family variety. Index groups are not a guarantee of
geometric adjacency. This avoids introducing a single identical facade row
without inventing new lots or randomly adding props. Existing height groups,
property boundaries, setbacks and vegetation remain in their approved layout.
Detached farmstead art, redesigned parcels or additional vegetation were not
introduced. The source24families and complete near/far identity are reused.

The full approved-source Metropolis bake regenerates world content and both
albedo/packed crops. Every non-style field in every building and every other
world field must compare equal to pass07, including routes/elevation, surfaces,
props, traffic, walkers, fields, IDs and collision envelope geometry. Source
LAYOUT.json, GLBs, source far atlas and both crop payloads remain byte-identical;
only palette provenance changes. No simulation, controls, rig, recovery,
saves, audio, race, legacy map or Valley quota change.

`rr_content::metropolis_palette` validates finite ordered/normalized transition
stations and existing non-office pool membership. Python negative/coverage tests
exercise the actual offline compiler, capped-tower exception, role progression,
family diversity and exact original assignment outside scope. Preservation
receipt records every changed ID plus height/northing bounds. Existing pavement,
clearance, vegetation/population, elevation and source/mip tests are rerun.

## Reproduction

Run serially with no game, compiler or baker overlap; freeze assets/shaders for
whole native runs. The native runner isolates saves and records process state.

```
python3 docs/testing/polish-12/pass08/run_native.py before
/tmp/roaddash-layout-venv/bin/python tools/bake_metropolis.py
python3 docs/testing/polish-12/pass08/verify_preservation.py
python3 docs/testing/polish-12/pass08/run_authoring.py
CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo build --release --locked
python3 docs/testing/polish-12/pass08/run_native.py after
python3 docs/testing/polish-12/pass08/run_native.py after-recovery
python3 docs/testing/polish-12/pass08/run_checks.py
```

For before capture use the previous commit's executable/content; the current
compiled world includes this pass. Native survey includes both routes, farm,
entry, dense city, resident envelope checks and measured ready detail budgets.
Recovery smoke includes low-speed reverse, crash/pickup/remount/results. Broad
paired rig labs are unnecessary here because physical parameters are untouched;
the mandatory suite retains model differential physics/recovery fixtures.

Research: local approved LAYOUT, massing/race-first contracts, street-life height
compiler, facade catalog/atlas and runtime mesh fitting only. No external code,
media, dependency or license intake. Hash declarations updated in ASSETS.csv.
Human art/racing-speed transition readability, full site grammar, long streaming,
target GPU cost, Windows/controller and broad hardware acceptance remain open.
Static screenshots and wall frames cannot establish those gates. Stop after08.

## Authoring results

20 Python tests across six groups and the383-band/five-level far-atlas audit
pass. The full bake changes1,338of19,068style IDs:278two-storey,519three-storey,
194four-storey and347five-storey buildings; block northings span approximately
0–1,529.94m. All other content compares exactly equal. Full changed-ID receipt:
`preservation.json`. All taller/outside-scope assignments are checked against
the original algorithm. Existing source geometry/hashes and both albedo/packed
RGBA payloads match the prior commit exactly; no source-art rebake is necessary.

README/progress retain the same counts/placement/states; no new requirements
were needed for this bounded extension of PALETTE-01/V2-04-003.

## Native results and measured limits

Final release survey and recovery smoke exit0; no shader/pipeline validation
errors or panics. Reverse−1.898m/s; both pickup errors0.0000m. All assets stayed
frozen during each run. Source census unchanged:19,068buildings,3,067chunks,
2,909,113resident render triangles,2,613,119collision triangles,2,531static
collision batches and two routes. Both branch survey views retain ready detail.

Matched entry03 actual detail:2,708→4,862mesh instances and
1,495,690→1,298,748triangles. Existing6,000mesh/1.5million triangle caps remain.
Different existing families increase mesh count while lowering triangles here;
this is not a claim that the changed presentation is free. No new source meshes,
textures or runtime algorithm were added. Per-view actual draw counts and raw
one-second CPU/wall telemetry are retained in `comparison.json` and each
`performance.jsonl`. Approach windows can straddle a camera move and differ in
sample count; use them as observations, not controlled per-location benchmarks.

Whole-app bounded wall frames: before19.029ms average /36.966ms p95;
after19.288ms average /34.572ms p95. Both miss the16.7ms target. No causal speedup,
GPU timing or broad hardware acceptance is claimed. Final survey timing primarily
samples the finish, while per-view receipts expose changed approach workload.

Inspected matched city-entry03 before/after and final approach04 and branch06. They show the
new business/workshop/residential mix on the same continuous street walls;
retained heights, road/sidewalks and later dense city remain legible. Fine repeated
signage and facade proportions remain source-art limitations. Static views do
not establish racing-speed temporal quality or user art approval.

- [Before entry](before/screenshots/metro-03.png)
- [After entry](after/screenshots/metro-03.png)
- [Approach into denser blocks](after/screenshots/metro-04.png)
- [Branch neighborhood](after/screenshots/metro-06.png)
- [Recovery](after-recovery/screenshots/06-remounting.png)

Changed implementation files: `tools/bake_metropolis.py`,
`content/world/metropolis_facades.json`, generated `content/world/metropolis.json`
and `assets/metropolis/PROVENANCE.json`; rr_content palette validator/module;
source/transition tests. Records: README/PROGRESS, ADR112, known issues,
development history, provenance and this evidence folder. No dependencies or
source-art asset changes. No new IDs, checklist closures or task moves.

## Final mandatory regression receipts

Fmt, strict locked workspace/all-target Clippy, all427workspace tests
(one pre-existing ignored), locked debug build, asset/dependency audit, five
roadmap tests and progress synchronization all pass. Locked release build also
passes. Source model differential body/joint/180tick trajectories and existing
crash/run/remount and physical ground-support regressions pass in that suite.
Exact commands/exits: `checks.json`; seven authoring groups: `authoring-checks.json`.
`git diff --check` passes. No claims of unrun paired rig labs, new legacy native
sweeps, Windows/controller/GPU tests, user approval, commits, pushes or merges.

Git at handoff:12tracked modified files plus3untracked top-level entries
(new rr_content validator, transition test, pass08 evidence directory). Dedicated
integration branch unchanged; no commits made. Parent reviews/integrates.
