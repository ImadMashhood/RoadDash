# Part 03 verification receipt — 2026-10-30

Scope: requested combat/NPC/collision/curb/traffic/map-detail/assistance fixes,
shirt-colored racer minimap, and start Part 03. Branch: dev/part-03-vehicle-combat.
Pinned Rust 1.94.0, Bevy 0.17.3, Rapier 0.32.0; no dependency migration.

## Automated evidence

[Local command log](part-03/local-checks.txt) records exit codes for:

```sh
cargo fmt --all --check
cargo clippy --workspace --locked --all-targets -- -D warnings
cargo test --workspace --locked
cargo build --locked
cargo build --release --locked
cargo check --locked --no-default-features --features offline
cargo check --locked --features debug_physics
target/release/road-dash --validate-track content/tracks/valley.json
target/release/road-dash --validate-track content/tracks/handling.json
target/release/road-dash --validate-handling content/handling.json
python3 tools/test_audit.py
python3 tools/audit.py --output docs/testing/part-03/declaration-audit.json
python3 tools/sync_progress.py --check
git diff --check
```

Regular Rust suite: 70 passed (57 client, seven content, six core), one explicitly
ignored soak. Python audit regression suite: two passed. New adapter tests select
Dynamic explicitly; older assisted-controller regressions remain valuable but
are not represented as dynamic tests. Source: [vehicle tests](../../src/vehicle_tests.rs),
[game tests](../../src/game.rs), [NPC tests](../../src/npc.rs),
[core rules](../../crates/rr_core/src/lib.rs), [palette test](../../src/ui.rs).

Coverage includes three preset acceleration/braking/two-wheel support, 30/60/120
render FPS with fixed acceleration and braking, 20-degree lean recovery and exact
inversion self-righting, stationary/moving curb support without knockout,
eight-rider grid launch, stationary offset without centering, correct preset dry
wreck mass, and remount with full health and no stale velocity. Combo tests cover
0/2/4-second hits, >3-second reset, duplicates and direct geometry. Contact tests
cover relative threshold/light taps/fast rear wipeout. NPC checks cover distinct
body/joint counts, solid finite physics, eight-active budget/reset and delaying
restoration while the original location is occupied. Existing controls/race/gates,
boost/grass, ragdoll/recovery, save and audio regressions remain in the suite.

## Numeric handling and soak

Explicit command (also Linux CI):

```sh
cargo test --locked ten_minute_aggressive -- --ignored --nocapture
```

One passed. [Exact measurements](part-03/benchmarks.json),
[raw benchmark output](part-03/benchmark-log.txt). On the closed handling fixture:

| Preset | 0–100 km/h | Braking distance after 10 s acceleration | Steady turn yaw rate |
| --- | --- | --- | --- |
| Light/agile | 4.583 s | 93.175 m | −0.141 rad/s |
| Balanced | 4.150 s | 97.228 m | −0.145 rad/s |
| Heavy/powerful | 3.917 s | 111.908 m | −0.159 rad/s |

Braking starts at each preset's speed after ten seconds of acceleration, **not**
a standardized 100 km/h brake test. The soak runs 36,000 fixed ticks (600 simulated
seconds), alternating steering every 1.5 seconds. Assertions reject nonfinite
pose/velocity, excessive speed and ground penetration/large vertical excursions.
Race gate bookkeeping is reset in this isolated handling fixture to keep it
racing. It is not ten wall-clock minutes, ten human rides, a busy-route endurance
race or exhaustive collision proof. Trace frequency is the real 60 Hz simulation;
100 Hz interpolation is not fabricated.

## Native verification

Serial Fedora/RTX 3080/Vulkan release runs and final images are recorded below.
The main smoke fixture scripts state changes/scene teleports/results after
recovery; it is not a complete no-teleport dynamic race or subjective playtest.
The actor fixture triggers physical hit messages directly; native manual melee
input/targeting is covered by headless rules, not claimed from that fixture.

An initial final smoke attempt failed the renderer's >0.25-second running-stage
assumption: a nearby rider can legitimately reach the bike sooner. Verification
now observes every recovery stage at fixed-tick resolution, with screenshot
capture independent of duration. Gameplay stage timing was not extended to make
an image. Failed attempt retained in [log](part-03/smoke-initial-failure.txt).

Final main smoke **passed**: farm crash → get up → run → remount → city/hills
→ hill crash → run/remount → results. All three recovery stages observed in fixed
ticks; running also captured by the renderer. Four audio playback sinks active.
[Main log](part-03/native-smoke-log.txt), [metrics](part-03/smoke-metrics.json).
Final serial actor smoke **passed**: independent 11-body human/10-body cow,
solid finite bodies, no NPC motorcycle wrecks, initialized audio.
[Actor log](part-03/actor-smoke-log.txt), [metrics](part-03/actor-smoke.json).

Visually reviewed final release captures:

- [Racing and shirt palette](part-03/02-racing.jpg)
- [Farm recovery](part-03/05-running-to-bike.jpg)
- [City windows/doors/trees/cross traffic/pedestrians](part-03/08-city.jpg)
- [Hill outer-curve rail/open straight](part-03/09-hills.jpg)
- [Hill running recovery](part-03/11-hill-running.jpg)
- [Distinct human/cow ragdolls](part-03/14-actors-ragdoll.jpg)

The main native run measured 948 fixed ticks / 848 active ticks and 949 render
frames on NVIDIA GeForce RTX 3080/Vulkan. Its bounded wall samples were average
7.909 ms / P95 16.957 ms; these include scripted scenes and are not GPU timing,
a representative busy-race benchmark or a claim that the 60 FPS P95 gate passed.
Startup Vulkan extension/suboptimal-first-present warnings did not prevent output.
The final runs had no audio underrun warnings. Four active sinks establish runtime
playback, not a human review of subjective mix/quality.

[Telemetry excerpt](part-03/vehicle-trace-excerpt.jsonl) contains the first ten,
middle ten and last ten rows of the native 848-row export; full ignored local
`screenshots/vehicle-trace.jsonl` is regenerated by `--smoke-test`. It is bounded
60 Hz tick data with explicit identities, intent and physical values.
[Handling process probes](part-03/handling-process-probes.json) passed: valid
catalog exits 0; future version, negative grip and duplicate ID each exit 2.

## Remaining acceptance

P03-010 remains unchecked: simulated soak passed, but ten actual test rides and
controller feedback have not been collected. P03-011 remains unchecked: distinct
feel, beginner completion and fun need human acceptance. No Part 04 advance.
Windows hardware/audio/input, manual foot control/finish, free-form crowd stress,
production animal/get-up art, traffic navigation and 100 Hz profiling where
needed remain open. Dynamic physics is a road-assisted arcade approximation;
see [contract](../VEHICLE_AND_COMBAT.md) and [known issues](../KNOWN_ISSUES.md).

Prior Part 02 Ubuntu/Windows CI run 41141131978 succeeded; the new Part 03 PR's
remote matrix status must be checked independently. Local evidence is not a
claim that a remote run or real Windows GPU session passed.
