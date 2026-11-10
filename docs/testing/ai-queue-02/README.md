# AI-QUEUE-02 — multi-rider crawling queues

The car-only preliminary return in `game::drive` skipped upright rider occupancy
unless a car had already triggered avoidance or passing was already committed.
A rider-only crawling queue therefore bypassed predictive clearance, passing
commitment and outbound bounds. This follow-up admits a slow forward rider
queue to the existing planner: a leader within 120 m, on the current line,
below 75% of desired cruise and at least 3 m/s slower than cruise. Ordinary
race-speed combat/pursuit retains its previous policy.

Once activated, all eligible upright riders are included, including rear and
fast alongside racers. Existing wrapped-course gaps, branch corridor filtering,
velocity prediction, curved static sweeps, 120 m lane-drop bounds, passing memory
and normal physical steering/braking remain authoritative. No rigid-body,
collision, player handling, reverse/recovery tuning, city population or map edits.
A blocked pass continues to re-evaluate its safe corridor every tick; a new core
regression verifies it resumes after a temporary closure without leaving bounds.

## Evidence

The physical fixture uses four separate real Rapier dynamic bikes at 2 m/s,
12 m apart. It tests a rider-only queue behind a physically driven slow leader
and a second queue behind a solid 2 m/s car. It runs 30 simulation seconds and
requires all three followers to get at least 8 m beyond the obstruction, retain
100 health/Riding state, remain inside outbound road, and hold an actual passing
lane for at least 30 consecutive fixed ticks. It records pair separation and
obstruction clearance; cars require >2 m and narrower bikes require >1.2 m.

- **Before:** original car-only gate with the new physical fixture fails:
  rider 3 crosses the inbound boundary (lane −0.00586 on a positive-outbound
  fixture). Receipt: `before-physical.log`. This is a reproduced unsafe queue
  response; that run stops at the first violation and does not quantify the
  complete original crawl duration.
- **After:** both cases pass. The slow rider ends at 133.26 m; the three
  followers reach 1038.53/1178.78/1262.56 m, with 504/644/229 consecutive passing
  lane ticks and minimum rider separation 1.776 m. The slow car ends at 155.01 m;
  followers reach 686.69/892.91/952.40 m, with 1076/847/628 passing ticks and
  minimum car clearance 2.366 m. Every rider stays healthy, mounted and inside
  outbound bounds. See `after-physical.log`.
- **Core:** all 15 traffic-avoidance tests pass (`core.log`), including narrow
  roads, occupied/rear/oncoming gaps, invalidated commitment, static corridor
  closure/reopening, matched-speed following and preserved ordinary combat.
- **Regressions:** `existing-physical.log` covers original real dynamic
  stop/follow/pass, crossing cars and narrow four-rider queue fixtures (3 pass);
  `existing-passing.log` covers stopped-car passage without exceptional escape (1 pass).
- `cargo fmt --all --check` and `git diff --check` are local checks. Full workspace
  tests/clippy/build/provenance and graphics-native review are parent integration
  gates. No existing dedicated native multi-rider queue scenario was found;
  these receipts are actual headless fixed-step Bevy/Rapier physics, not GPU
  captures or a performance measurement.

The first instrumented test accidentally applied its two-metre *car* clearance
threshold to the narrower rider leader. Its recorded 1.776 m safe side-by-side
spacing violated that overbroad fixture assertion, with no impact/recovery or
outbound violation. The final fixture distinguishes body kinds; the rejected
receipt is retained as `fixture-clearance-rejected.log`.

Commands use the existing pinned toolchain/dependencies, without PKG_CONFIG_PATH
overrides. `/tmp` user quota prevented even a small scratch backup, so Rust scratch
uses the existing home filesystem; no other agent's files were deleted:

```sh
TMPDIR=/home/imad/.cache/roaddash-ai-queue-tmp CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo test --locked --lib crawling_rider_queue_passes_without_a_car_trigger -- --nocapture
TMPDIR=/home/imad/.cache/roaddash-ai-queue-tmp CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo test --locked -p rr_core traffic_avoidance -- --nocapture
TMPDIR=/home/imad/.cache/roaddash-ai-queue-tmp CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo test --locked --lib predictive_ai -- --nocapture
TMPDIR=/home/imad/.cache/roaddash-ai-queue-tmp CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo test --locked --lib queued_ai_physically -- --nocapture
```

Project-source research only: AGENTS, current checklist/progress/decisions/issues,
relevant frozen AI blueprint, Cargo.lock, `traffic_avoidance`, `rival_ai`, `cast`,
`escape`, game adapter and prior physical queue fixtures. No external code,
assets, dependency or licensing changes. Parent owns shared documentation and
provenance updates for AI-QUEUE-02. Human race balance, arbitrary full-course
queue/deadlock behavior, performance and hardware coverage remain open; sealed
or occupied roads must still wait rather than pass through inbound/sidewalk space.
