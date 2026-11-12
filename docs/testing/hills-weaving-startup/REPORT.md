# Hills weaving and startup — 2026-11-12

Branch `codex/hills-weaving-startup`, base `dd9fde51`. ADR170/171 record decisions
and rollback. AI-HILLS-02 verified; PERF-START-01 remains partial. No new assets,
dependencies or copied reference source. Installed Bevy0.20.1 scene-readiness API
review recorded under R08; no engine migration or physical-rig changes.

## Findings and implementation

User log `performance-2400160`:15774→39284 entities during initial expansion,
near39000 thereafter. At14–16s simulation time, wall frames264–267ms and fixed
gameplay201–207ms/frame, versus physics40–44ms/frame. Aggregate CPU/wall evidence,
not GPU attribution. World setup5.43s, assets ready8.71s, menu8.73s, race8.97s.

- `rr_core::traffic_avoidance`: Hills uses actual bike-width plus clearance for
  steady oncoming gaps, considers centre and sidewalk routes, keeps6–7s temporal
  forecasts and wider crossing buffers. Deduplicate equivalent goals within2.5cm.
  Sort by identical cost/tie-break and stop at the first safe route.
- `game`: Hills imported city has the existing2.7m sidewalk allowance. Static
  sweeps still reject furniture, buildings and trees. Explicit current/requested/
  committed routes determine the next-tick obstacle guard, independently of the
  order of speculative search. No collision exemptions or teleported movement.
- `ai_work`: bounded256 exact goal/speed-bit static results, scoped to one rider
  and fixed tick. Dynamic traffic always replans. `ROADDASH_AI_STATIC_CACHE=0`
  selects exhaustive search without reuse; default/1 enables optimization.
- `runtime`: all resident scene roots must be instantiated and required
  assets/materials/tangents ready; eight stable mesh-count frames precede menu.
  Headless/no-AssetServer fixtures retain their behavior.
- `diagnostics`: bounded async logs add `ai_work_window` drive/static CPU,
  requests, cache hits and actual casts. No per-frame disk writes.

No traffic, buildings, collision shapes or gameplay content removed. Saves,
controls, accepted ragdolls, momentum and recovery remain integrated.

## Evidence, including rejected trials

[Cache-only trial](cache-only-trial.json):300 sampled states/plans match, but
22.61→22.48ms main-world medians are within variation; not a meaningful win.
[First ordered trial](ordered-guard-trial.json): CPU work falls but226/300 plans
differ; rejected as equivalence evidence. It exposed speculative probes changing
the guard timer, fixed by explicit route checks above.

Final binary/source hashes: [source receipt](weave-final-source.json).
[Opening5–12s](early-comparison.json):45 sampled rider rows match exactly in
progress/lane/speed, plans and recovery. Static casts4081→729/tick (~82% fewer),
static CPU10.31→2.49ms/tick, drive CPU11.25→3.31ms/tick; mean main-world
CPU26.83→19.75ms/frame. These are one local Linux pair and window aggregates,
not per-tick replay or a target-hardware FPS guarantee. Window boundaries differ.

[Whole5–50s comparison](final-comparison.json): later trajectories diverge
(first observed at~15s with very small numeric differences, growing thereafter).
Consequently its23.47→18.88ms mean main CPU cannot certify equivalent-scene
sustained performance. Keep PERF-START-01, RFU-PERF-01/02/04 and AI-11 open.
The generated scene remains ~39000 entities; this is not an entity-budget fix.

## Checks

- `cargo fmt --all --check`: pass.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: pass.
- `cargo test --workspace --locked`:477 pass, two existing failures, one ignored;
  execution stops on the main lib failures. Separate locked rr_core/content run:
  165+59 pass, one ignored. Total701 Rust tests pass.
- Existing failures: `biomechanics::tests::wall_impact_and_high_speed_ejection_remain_bounded`
  and `world_gen::metropolis::geometry::tests::standalone_static_chunks_keep_contact_ccd_and_support_trajectories`.
  No tolerances, solver or anatomy changed to hide these failures.
- Physical `hills_rival_physically_clears_two_solid_oncoming_cars`: normal dynamic
  bike passes two moving solid cars, stays Riding, health100, retains speed and
  uses steering. Pure tests retain tight/late/blocked braking cases.
- Existing model differential, traffic and recovery checks pass. Cache lifetime,
  guard-route selection and late mesh-settle tests pass.
- [Standalone seeded fixture](verify_ordered.rs):10000 sequential reference/ordered
  comparisons match, including static blockers and committed pass memory.
- `cargo build --release --locked` and `cargo build --locked`: pass.
- Nine Python roadmap/native-harness tests and `python3 tools/audit.py`: pass.
- Native3600-frame Hills reference/optimized opening runs: exit0, no render errors;
  these are opening runs, not completed races. Hills recovery: both remounts pass,
  exceptional rescues[0,0]. Metropolis recovery: both remounts pass, exit0, no render errors.
  Same-mode optimized repeat matches225 sampled rows exactly.
  [Repeat comparison](repeat-comparison.json) therefore does not explain away
  the cross-mode divergence; per-tick integrated equivalence remains unverified.

Commands, logs and receipts accompany this report. Raw earlier trials and full
recovery images remain locally under `output/testing/hills-weaving-startup/raw-native/`;
compact receipts retain their binary identity. Final opening JSONL streams remain
in `docs/testing/issue26-racer-ai/weave-final-*` for comparison reproducibility.

## Limits and next verification

Synchronous initial world generation still takes seconds. This pass gates scene
readiness and removes redundant AI work; it does not supply asynchronous world
construction, GPU attribution, a sustained16.7ms budget, Windows/hardware coverage
or full-course human traffic-risk/difficulty approval. Those gates remain open.
Do not automatically advance to another development part.

## Local integration

Verified binary installed atomically for `./play-bevy20.sh --hills`;
[installation receipt](installation.json) matches the final source hash record.
Only AI-HILLS-02 moved to completed Part00 (234 verified/350 open).
PERF-START-01 stays open with its partial evidence. No assets/dependencies added;
no push or merge claimed.
