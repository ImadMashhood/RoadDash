# Pass01 — performance diagnosis and facade selection stability

Scope: PERF-08, integration branch `codex/gameplay-graphics-polish`, baseline
`b52ab5e` plus setup-only `6518379`. No new task IDs or completed requirements;
README/PROGRESS retain all gates. No task movement; placement is unchanged.

## Implementation

Metropolis authored detail now keeps the existing 25m radius / 20m ranking
bonuses only while a group remains selected. Retiring groups qualify as new.
Selected faces have a 2m rear-plane exit deadband while new faces still require
front-side visibility. This prevents tiny tangent-camera crossings from reversing
selection. A full 28-slot Bevy update-system regression exercises repeated
crossings and requires every group to remain wanted at full weight; a separate
boundary test covers exit and retiring bonuses. Production churn frequency was
not measured, and the retiring bonus correction is not evidence of a measured
performance improvement.

Unchanged: geometry, atlas/material hashes (no rebake necessary), resident closed
envelopes and collision, source identities, mesh/triangle overlap budgets, fade
duration, fixed simulation, accepted velocity muscles, eleven-body baseline,
recovery, audio, saves and controls. No engine/dependency/media/UI artwork changes.
ADR105 records the choice. No new external research or assets; inspected project
source and prior ADR089/PERF-08 receipts, locked Bevy0.17.3/Rapier adapter0.32.0.

## Native reproduction and measurement limits

Release builds use the existing target cache. Each native process runs alone,
with isolated save directory, from its evidence subdirectory. Commands:
`road-dash --metro-survey`, `road-dash --actor-smoke-test`, and
`road-dash --smoke-test`. All captures are actual native screenshots. Survey:
14 farm/city-entry/downtown camera views; actors remain near the farm. Actor smoke:
simultaneous human/quadruped impacts and bounded solid-contact ragdolls. Recovery:
real farm and city crashes, two remounts and results, with fixture teleports.
These are not a sustained driven downtown race or simultaneous multi-rider crash
stress certificate. The short NPC sample is especially unsuitable for statistical
performance conclusions.

`before/` and `after/` initial runs overlapped an independently launched original
checkout game (PID2871704; `concurrent-process.txt`). They are retained as
**non-causal functional evidence**, excluded from controlled comparison. The user
closed that game; `controlled-before/` and `controlled-after/` contain replacement
runs, startup process snapshots and exit codes. No compiler ran during those
measurements. Local GPU reported RTX3080/driver615.78.08; no GPU timestamps were
captured. Window1440x900, existing default presentation settings, unchanged rig.

Summaries use `tools/summarize_performance.py` with four startup samples removed
and racing-only filtering. Independent-window means and physics/fixed-step work
are distinct from rolling240-frame p95 and latest render-schedule CPU/wait spans.
No raw-frame percentile or additive CPU/GPU decomposition is claimed.

## Results and gates

| Stage / fixture | Racing windows | Median independent frame mean (ms) | Physics/fixed step (ms) | Step phase/fixed step (ms) | Latest extract median (ms) | Latest render schedule median (ms) |
|---|---:|---:|---:|---:|---:|---:|
| controlled-before / metro | 17 | 16.69 | 8.33 | 6.57 | 0.49 | 7.20 |
| controlled-before / actors | 1 | 16.90 | 9.42 | 7.55 | 0.50 | 5.14 |
| controlled-before / recovery | 14 | 16.32 | 7.58 | 5.86 | 0.52 | 6.65 |
| controlled-after / metro | 17 | 16.43 | 8.01 | 6.25 | 0.53 | 7.75 |
| controlled-after / actors | 1 | 18.59 | 10.49 | 8.50 | 0.42 | 5.78 |
| controlled-after / recovery | 14 | 16.30 | 7.83 | 6.05 | 0.47 | 5.72 |

Single matched runs show no established performance gain. Survey16.69→16.43ms is
a small difference; physics and render values vary despite unchanged simulation.
The dominant measured CPU cost remains physics, especially step work. The short
actor fixture yields only one selected window and is functional stress evidence,
not a stable benchmark. GPU bottlenecks remain unmeasured. Recoveries passed with
both pickup errors0.0000m, seven racers/seven killers and four audio sinks.
Actual downtown view09 before/after was visually inspected: resident facades
remain continuous; screenshots cannot establish temporal pop-free behavior.



PERF-08 remains open: controlled repeated full-route/downtown/multi-rider-crash
stress, GPU timestamp attribution, target p95, human pop/oscillation review,
Windows/controller and broad hardware acceptance. Existing evidence identifies
CPU physics/catch-up as a major cost; this scoped visibility fix does not justify
changing accepted solver/rig parameters. No broad physics optimization performed.

Controlled before/after Metropolis census is exactly equal:19,068 buildings,3,065
resident chunks,2,903,477 render triangles,2,607,483 collision triangles,2,529
static batches,30 crossing cars,1,494 street props,122 parked cars and96 walkers.
The source-layout SHA remains unchanged. Representative retained PNGs are in
controlled-before/ and controlled-after/ under screenshots; redundant PNGs from
the contaminated runs and unused views were omitted, while all JSON/logs remain.

## Verification

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: passed.
- `cargo test --workspace --locked`:415 passed,0 failed,1 existing ignored;
  includes both added facade regressions and model differential physics/recovery tests.
- `cargo build --locked` and release build: passed.
- `python3 tools/audit.py`: passed after refreshing the original facade-source
  ledger hash/ADR105 note. No asset/license/dependency adoption.
- Five roadmap tests and progress synchronization: passed. No IDs moved/closed.
- Both legacy LOD surveys exited0: City391 direct crossfades (max48 cached,8
  transitions); Valley54 direct crossfades. Actual native PNG/report receipts
  retained under lod-city/ and lod-valley/.
- All twelve initial/replacement Metropolis/actor/recovery processes exited0;
  only six replacement runs are controlled timing evidence.

`checks.json` lists exact commands/statuses; separate text receipts retain output.
Native warnings include the existing unavailable Vulkan DRM extension, gamepad
default mapping and suboptimal initial present, with successful process exits.

Paired release headless lab:18 cases per build, all finite, including fifteen
simultaneous crashes for both eleven-body control and biomechanical rigs.
This is isolated backend work, without full course/rendering; physical settings
are unchanged and timing differences are not a facade speedup.
- Fifteen crashes / baseline:900 fixed updates, 165 bodies; headless wall 3372.65→3424.90ms.
- Fifteen crashes / biomechanical:900 fixed updates, 240 bodies; headless wall 6655.23→6607.55ms.
