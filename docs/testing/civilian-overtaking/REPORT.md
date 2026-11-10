# Civilian overtaking — issue27

Implementation is complete on codex/issue-27-traffic. Automated checks pass;
scheduled native motion/recovery and quiet CPU receipts remain pending.

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
are recorded separately when completed;
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


Pending native commands (run after obtaining the final frozen binary):

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
JSON and motion captures. Do not replace pending entries with pass claims until
these runs finish. RFU-TRAF-01–04 remain open; user/full-course acceptance and
broader hardware/performance gates are not covered by the automated counts.
