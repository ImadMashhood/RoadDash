# Hills opening AI — 2026-11-12

Scope: Hills only, user clarifies fourth place occurs during the start.
Branch codex/rival-race-pressure, base c4b14b56. ADR169 records the policy.

## Investigation

Prior final Hills native samples show low-speed avoidance around600m among
otherwise healthy riders, while the physical straight fixture reaches177km/h.
The planner assumes constant terminal-speed travel with6–7s empty-lane
requirements, so distant same-direction occupancy can veto local acceleration.
A new pure fixture demonstrates that false restriction and separately retains
near-car braking, distant oncoming/crossing rejection and solid-wall stops.
All31 pure traffic planner tests pass before full integration checks.

A literal hold-W native control on the actual Hills starting grid does **not**
reproduce fourth place: it reaches the first bend's wall and stays15th near4m.
The separate manually driven playtest log reaches1500m around42s into its
second race. These are different input trajectories; do not equate them.
Matched all-AI opening samples can test flow, but human fourth-place difficulty
remains an open acceptance gate.

## Candidate

Hills only: same-direction traffic uses a3.5–5s local horizon recomputed each
fixed tick; oncoming/crossing keep the original full horizon. Keep current and
requested-speed occupancy checks, static geometry, traffic collisions, player
handling, engine/corner limits and all accepted ragdoll settings. Other maps
and civilian controllers keep the default policy.

## Observed failure and correction

The first local-horizon candidate (16b83b7c…) is **not** accepted. Its race
window ended after36s of simulation, before a final survey; recovery also ended
before its completion marker. The old harness returned race success on process
exit alone. It now requires recorded sample-window completion, a real slot-zero
finish or a successful explicit segment. Four isolated fake-process tests pass.

The retained performance log shows camera-rider plans117–124km/h near0m and
rivals5/11 explicitly assigned0m/s near650m. A targeted56.98s diagnostic
(8d2210d2…) confirms target51.7m/s and no corner braking at the120km/h plateau.
Its full proxy trace captures the stopped pair: rider5 at652.43m/-0.85m,
rider11 at650.64m/-1.58m. Padded clearance overlap blocks their departure.

The revised candidate adds safe incremental-speed trials and a bounded
front-bike departure from an overlapping follower comfort envelope. All actual
colliders/CCD stay active. No such departure is allowed toward a leader ahead,
car, oncoming/faster follower, inward motion or static obstruction. All33 pure
traffic tests pass, including the captured pair and unsafe variations. The
initial horizon-only fix was insufficient and remains a failed trial.

## Final verification

Final release SHA256: `d2979c3148102b196e3c06e9ed74af7537df1b1c31aa0458d42fe425d5d668cd`.
[Source hashes](hills-separation-source.json), [commands](hills-separation-checks.json)
and `hills-separation-*.txt` logs cover the final code. Cache/build environment:
`CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadDash-worktrees/issue-37-bevy-020/target`,
`CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2`.

- Format, strict workspace/all-target Clippy, locked debug/release builds and audit pass.
- Main library:474 passed, two pre-existing Bevy/Rapier failures and one ignored.
  Core:163 passed/one ignored; content:59 passed. Total696 passing Rust tests.
  All12 model/rig differential checks remain strict and pass. No rig values changed.
- All33 traffic-rule tests pass, including captured follower overlap, incremental
  acceleration and forbidden front/car/oncoming/faster-follower/inward/wall variants.
- `python3 tools/test_racer_ai_checks.py`:4 real fake-process receipt-gate tests pass.
  Roadmap/progress tests pass. Interrupted trials remain failures/incomplete.
- Final [Hills race receipt](../issue26-racer-ai/hills-separation-race/receipt.json):
  full7200-frame sample, no runtime errors. It does not finish the12500m race.
- Final [Hills recovery](../issue26-racer-ai/hills-separation-recovery/receipt.json):
  two pickups, zero position error, no exceptional rescue; native audio initializes.

[Matched opening comparison](final-comparison.json), same scripted all-AI controller,
traffic and content as the previous final release:

| Race time | Previous median rival speed | New | Previous median progress | New |
| --- | ---: | ---: | ---: | ---: |
| 15s |132km/h|155km/h|344m|388m|
| 30s |54km/h|160km/h|628m|1038m|
| 45s |70km/h|154km/h|836m|1662m|
| 60s |145km/h|147km/h|1371m|2312m|

Periodic first-minute stopped **Riding** samples:7→0. The asynchronous final
performance stream independently has0 stationary rivals in1092 first-minute
rival observations. Median sampled time past1500m:64.48→41.98s. The riders
previously stopped around650m (5/11) now pass that region and reach roughly
860–970m at27s, at150–160km/h. Recovery states are counted separately; the
final zero-speed rider near4419m is GettingUp after a crash, not a new queue.
These are functional AI-flow measurements, not a frame-time benchmark or human
fourth-place/difficulty approval. Keep RFU-RACE-03/AI-11 and AI-ENGAGE-01 open.

## Reproduction, files and tracking

Run `./play-bevy20.sh --hills`; [installed binary receipt](installed-launcher.json).
For diagnostics: `ROADDASH_AI_TRACE_OPENING=1 python3 tools/run_racer_ai_checks.py
--binary <absolute-release-path> --case race --track hills --frames 3600
--label <unique-name>`. The ordinary performance log now includes plan lane,
effective inputs, combat/escape intent and race time, even after early exit.
Reproduce comparisons with `python3 docs/testing/hills-opening-ai/compare.py
<old-racer-ai.json> <new-racer-ai.json>`.

Changed implementation: rr_core traffic planner; game adapter/static-hit traces;
performance and opt-in survey diagnostics; native runner and its new isolated
receipt tests. Source hashes and selected frames/logs are retained; full raw
screenshots/performance streams live in ignored local output/testing/rival-race-pressure.

AI-HILLS-01 is verified in Part00 (233 verified/349 open). Human start/race
challenge remains RFU-RACE-03/AI-11; ordinary combat frequency remains
AI-ENGAGE-01. No next major part started. No dependency, external repository
or asset intake; original shared code only. ADR169 records alternatives,
tradeoffs and rollback. Existing high-speed ragdoll and static-chunk trajectory
parity failures remain known; no physical tolerance was relaxed.
