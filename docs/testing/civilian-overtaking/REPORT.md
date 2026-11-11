# Civilian overtaking — issue27

Implementation is complete on codex/issue-27-traffic. Automated checks pass;
eight selected native overtakes and both recovery controls pass. Normal-fleet
observations and quiet CPU receipts pass; extended diagnostics exposed and corrected an endpoint admission defect.
The corrected240-second census has nine completed passes and one healthy ongoing merge.

RFU-TRAF-01–04 / ADR142: pure fixed-step Follow/Pass/Abort/Merge decisions
live in rr_core/civilian.rs. Existing kinematic traffic, solid colliders,
road samplers, slow-vehicle cruise hierarchy, downed-person yielding/horns,
scene interest, course retry and both physical rider rigs remain integrated.
Authoritative lane transitions take three seconds; passing claims persist through
merge and are published in deterministic entity order. Passing duration includes
relative speed, tractor/trailer length, acceleration and both lane transitions.
The planner revalidates oncoming cars and the return gap every fixed step.
Separate departure/return full-collider sweeps sample twelve curved-road
segments at5Hz while impeded or passing. Each sample checks the full vehicle
width against the physical street-centered pavement reservation; asymmetric
lane dividers only normalize domain coordinates. Reused Local fleet/perception
buffers avoid adding per-tick allocation. The current curve speed is a separate
cap, so stopping for a downed person never erases signed travel direction.
An initial full-suite check exposed a launch-queue regression: temporarily slow
same-class leaders could trigger a pass from an already compressed fixture.
The corrected incentive resets its sustained-impediment timer while a leader
accelerates, rejects departure from overlapping initial envelopes, and requires
six steady seconds for a stalled same-class leader (one second for an actually
slower authored class). A disabled same-class motor remains passable; its authored
cruise is not an unconditional veto. Oncoming/moving-class forecasts anticipate
acceleration back to authored cruise. The earlier failure receipt is retained as
`tests-before-incentive-fix.txt`; only subsequent final checks count as validation.

A late obstacle sends a partial departure directly home when clear; otherwise
it holds lateral clearance and brakes behind the leader. Merge clearance is
rechecked, including downed people in the swept home lane. Reservations persist
until the actual return finishes. Pauses freeze the new state; retries/recycles reset it.
Upright bikers deliberately do not enter the oncoming-car veto perception set.

Research: inspected current traffic.rs, traffic_avoidance.rs, streets.rs,
heavy_traffic.rs, game.rs and AI-QUEUE-02 circular-wait evidence. Applied local
game-ai, ai-behavior-trees-utility-ai and performance-optimization guidance:
small explicit FSM, persistent action state, safety revalidation, cooldown and
separate motion adapter. No reusable BT framework needed for four states.
The author's [MOBIL lane-change explanation](https://traffic-simulation.de/info/info_MOBIL.html)
separates incentive from safety and describes a threshold/cooldown to suppress
lane hopping. This is conceptual background only: RoadDash's opposing-lane
reservation model is original and is not an implementation of MOBIL/IDM.
No repository source, third-party media or dependencies imported; existing
MIT project code/assets and exact lockfile pins remain. No legal-clearance claim.

Automated verification:609 workspace tests pass (427 game library,1 binary,
57 content,124 core), with two explicit ignores: the pre-existing soak and the
new isolated CPU microbenchmark. Strict format/Clippy, locked debug/release builds,
provenance declaration/hash audit and all5 roadmap checks pass. Logs are alongside
this report. The trusted release binary is `/tmp/road-dash-issue27`, SHA256
`4b7758c5edf711746fba08ea8b9253a7663b208ea4f644152131353aad62288e`.
All three workspace packages were explicitly cleaned in the release profile
before rebuilding; `build-receipt.json` includes source hashes. Native receipts
are recorded separately below;
no native outcome or timing is inferred from unit tests. Core motion checks
cover two followers behind a tractor/long trailer in both directions, competing
claims, blocked oncoming traffic, a partial departure, occupied return, zero-speed
yield, and an oncoming car appearing mid-pass followed by safe abort/retry.
Native --civilian-survey requires passing AND completed home-lane return,
checks actual selected collider intersections and sampler support every active
frame, and captures multiple motion stages. Its120-frame paused setup is followed
by the ordinary one-second impediment delay (over60 active physics steps) before
a pass can start, so the geometry gate sees an initialized broadphase. Metropolis
captures use its existing ordinary car models at tractor/semi cruise; full-length
trailer coverage comes from Hills production models and deterministic fixtures. Native/CPU timing is separate from GPU/full-race and
Windows hardware acceptance. Human gameplay balance and long-race acceptance
remain open; no new requirement is checked complete merely for unit coverage.


The native normal-fleet mode (`ROADDASH_CIVILIAN_FLEET=1 --civilian-survey`)
observes120 simulated seconds without relocating or changing production civilian
spawns, lane slots or cruises. Racers park to separate motor-traffic opportunities
from downed/combat interruptions. Counters record initiated passes, completed
home returns ahead of the original leader, aborted returns and outstanding claims.
Native-only timing records new planner CPU including static sweeps, plus the
whole civilian adapter separately. These CPU/wall values do not imply GPU timing
or a before/after frame-rate improvement. Quiet measurement receipts are separate
from functional tests run alongside compilation.


Completed native functional verification used the frozen binary above on Linux/
Vulkan/RTX3080. Hills and Metropolis each completed four selected overtakes
(tractor/semi cruise, both directions), reaching Follow with blend0 after
max blend1, zero selected-pair collider intersections and zero sampler support
error. Hills uses actual heavy models; Metropolis uses ordinary car bodies at
heavy cruise. Active route0 and baseline route1 Hills recovery both remounted
unassisted with zero exceptional rescues. The four receipts report exit0 and
no render errors. These runs overlapped compilation: their embedded CPU/frame
timings are loaded functional observations and are not quiet performance evidence.

Reviewed capture stages show lateral separation and return to the road lane.
Hills semi case2 frame480 is partly occluded by a foreground building; case3
frame480 shows the real trailer and adjacent passing car clearly. Images alone
do not establish swept clearance; the per-frame collider/support assertions
and final phase fields supply that evidence. Full raw captures remain beside
the receipts; compact JSON/log evidence is retained for review.

Completed functional and quiet measurement commands:

```sh
python3 tools/run_civilian_checks.py --binary /tmp/road-dash-issue27 hills metro active baseline
# Quiet host: no competing build/game for meaningful CPU timing.
python3 tools/run_civilian_checks.py --binary /tmp/road-dash-issue27 fleet-hills fleet-metro
CARGO_TARGET_DIR=target/shared-snapshot cargo test --release -p rr_core --locked civilian_planner_cost -- --ignored --nocapture
```

The runner rejects any existing case folder (including interrupted evidence);
use a new `--output-root` for another attempt. It validates each JSON completion/
merge/intersection/support result, isolates saves/preferences, hashes the
binary, detects process failure/timeouts/render errors, and preserves each case's
JSON and motion captures. The original full-fleet and isolated CPU commands passed;
extended Metropolis diagnosis and corrected repeat are recorded below. RFU-TRAF-01–04 remain open; user/full-course acceptance and
broader hardware/performance gates are not covered by the automated counts.

## Quiet production-fleet observation

The original frozen binary/source above ran with exclusive native access and
no competing builds/games. The validated user-owned file indexer was paused by
an owned pidfd and restored in finally; `quiet-indexer.json` records identity,
start-time check and restoration. Both fleets completed119.994 simulated seconds
(7199 sampled ticks); Linux/Vulkan/RTX3080. Racers were parked, civilian spawns,
cruises and lane slots were unchanged. No thresholds changed to force counts.

| Production fleet | Vehicles | Started / completed / aborted / active at cutoff | Planner mean / P95 / max ms | Whole adapter mean / P95 ms |
|---|---:|---|---|---|
| Hills |38|11 /11 /0 /0|0.114 /0.176 /0.289|0.206 /0.274|
| Metropolis |144|3 /1 /0 /2|0.767 /1.031 /1.570|1.995 /2.603|

Peak simultaneous reservations were5 and2 respectively. Both processes exited0
without render errors. These counts demonstrate ordinary passing opportunities;
they do not establish that every queue clears. Metropolis final camera is
occluded by a foreground building. Its two ongoing claims prompted the extended
transition diagnostics and correction recorded below.

The separate release core144-vehicle microbenchmark passed: median242.059µs,
P95 245.491µs per tick, including reused perception copies and excluding native
geometry/physics/rendering. This is not comparable directly to whole-adapter
cost. Metropolis planner P95 1.031ms is material in a16.67ms frame budget;
combined game/frame/GPU performance after integration remains an open gate.
The first diagnostic rebuild changed survey reporting only; subsequent endpoint
behavior correction is separate. Loaded runs do not replace these original quiet measurements.

## Endpoint diagnosis and verified correction

The first extended240-second Metropolis run (`extended/`) exposed a73.38-second
stationary Abort near12048m. Its execution receipt passes, but the separate
`behavior-review.json` rejects behavior: a pass admitted near11862m ran beyond
the12000m authored road support and then both geometry gates vetoed motion.
Another aborted actor only returned to Follow through ordinary recycling.
Preserve this failed evidence; it is not a successful merge receipt.

Admission now compares the existing acceleration-integrated completion/return
travel plus vehicle extent and8m against authored forward road remaining.
Both directions reject140m and retain a500m safe opening; the production adapter
has a matching two-direction endpoint regression. The gate applies at admission,
so ongoing claims retain normal merge/abort handling. No broad maximum-time
no-passing zone or lowered safety thresholds were introduced.

The endpoint-fixed frozen binary SHA256 is
`fda415372598f2d3e6e650df869edc77017d5a02faac1a5597dfe694e28a5bca`;
`endpoint-build-receipt.json` records source hashes. The corrected240-second
Metropolis census (`endpoint-fixed/`) reports144 vehicles,10 initiated,
9 completed,0 aborted returns and1 active at cutoff, peak2 claims. The remaining
actor is a healthy Merge: age9.88s, blend0.0389, speed19m/s and both geometry gates
clear. It started at230.09s and began returning at237.10s. Do not claim all passes
completed. The earlier stranded actor waited through the endpoint, recycled
normally, and completed a later supported pass at192.68s. No errors;489.98s wall.
This run overlapped builds and supplies behavior evidence only.

The runner now rejects final claims aged45s or more, records timeout bounds,
and accepts `--timeout-seconds 1800` for extended diagnostics under compilation.
Survey-only transition timestamps and final actor/gate details make a stuck
abort distinguishable from a recent pass. Reproduce with a fresh output root:

```sh
ROADDASH_CIVILIAN_FLEET_SECONDS=240 python3 tools/run_civilian_checks.py --binary /tmp/road-dash-issue27-endpoint --timeout-seconds 1800 --output-root NEW_PATH fleet-metro
```

Final endpoint source:611 workspace tests pass (428 library,1 binary,57 content,
125 core), including unchanged differential rider physics/recovery controls.
Strict fmt/Clippy, locked debug/release builds, provenance audit and five roadmap
checks pass; final-* logs record results. Original quiet CPU costs refer to
the earlier frozen source; no fresh quiet speedup claim follows the endpoint fix.

## Integration with performance defaults

Integrated main76d4fe154baa169bbbf19e163cb0c0868d18d0c4 (PR35), preserving all
roadmap tasks and both decisions. Combined615 tests pass (432 game,1 binary,
57 content,125 core), strict fmt/Clippy, locked debug/release builds and audit.
Roadmap/performance Python checks pass with `PYTHONPATH=tools`; the initial
package invocation lacked that import path and its failed log is retained.

Frozen integrated SHA256:
`6f17c4986451e9874bb7a3593ae8def8db4dce295ebbacace2917af5137118f1`.
`integration-build-receipt.json` records commit/source hashes. Combined Hills
selected4cases pass206.68s wall: actual heavy bodies, complete returns, zero
pair intersections/support error. Combined Metropolis240s passes313.62s wall:
144 civilians,10 started,9 completed,0 aborted,1 healthy final Merge(age9.88s,
blend0.0389,19m/s,both geometry gates clear). No errors or stranded45s claims.
These repeated behavior outcomes use the new default executor/loading settings;
loaded wall times are not a performance comparison. Prior quiet timing receipts
retain their original-source scope. Broad human/full-course/GPU/hardware gates
remain open; this integration does not close issue27 automatically.
