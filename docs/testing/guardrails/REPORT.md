# Guardrail follow-up — issue33

Status: integrated verification in progress on branch `codex/issue-33-guardrails`.
RFU-RAIL-01..05 remain open. ADR148 records the policy and provenance.

The existing Hills generator placed both sides on bends with fixed0.8m posts
referenced to sampled road positions. The replacement selects the explicit turn
side using horizontal signed yaw, keeps runs at most96m, and leaves16m gaps.
Rail height and support feet are independent: road elevation drives rail height;
actual resident shoulder triangles and terrain drive posts. Ends flare outward
within their run and have solid broad terminal shoes. Suspension bridge edges
remain unrailed. The legacy authored Guardrail enum has no current content instances. A later
source audit found an unnamed Sector::Hills barrier loop in environment.rs; it now
uses the shared turn-side/gap/terminal generator with captured resident collision
triangles and the existing Valley lot/access exclusions. Its shared authored fence
adapter and all Valley quotas are preserved.

Original geometry only; existing cube and metal material reused. Source research:
`geometry.rs`, `bridge.rs`, `recovery_hills_tests.rs`, `track.rs`, `environment.rs`,
content tracks, ADR136/137/138 and current lockfile. No external repository, copied
asset, dependency upgrade or engine migration. Bevy skill0.20 API examples were not
adopted over installed0.17.3 APIs; engine-neutral grounding/traversability principles
were applied. Level-design, procedural-gen, performance, Bevy rendering/physics
skills informed topology and verification; no performance gain is asserted.

Prepared checks:

- Pure signed-turn and bounded-run/gap tests in rr_core.
- Actual resident collider rays for every post foot, all terminal contact, and
  bidirectional capsule/full-chassis sweeps through the central12m of every16m gap.
- Actual recovery controller return on each turn side with zero exceptional rescues.
- `--guardrail-survey`: original rendered runner with production capsule/controller
  walks across a real opening and back; captures posts/terminals and emits a receipt.

Executed at checkpoint: `cargo fmt --all` and `git diff --check`.
Workspace package clean commands ran for both debug/release (already dependency-only
cache: removed0files). No compiler, test, native fixture, audit or performance run
has executed for this draft. Final command receipts and captures must replace this
status before delivery. Native runs must hold `/tmp/roaddash-native.lock`, use
isolated settings/save/working directories, validate successful completion and check
logs for panic/ERROR/shader failures. Quiet timing requires the root's exclusive window.

Remaining: compile and fix tests; strict clippy/workspace test/debug build/audit;
release build; native dedicated gap survey, both Hills branch recoveries with both
rig controls, model differential tests and paired headless lab; native visual review;
integrate latest main preserving29 sand/25 terrain cache/31 river edits; regenerate
checklist indexes; commit, publish PR, root review and serialized authorized merge.
Human full-course aesthetics and broad hardware/performance approval stay open.

## Diagnostic follow-up (2026-11-11)

Merged performance main76d4fe15, preserving exact terrain caching and the terrain
profile span. The first full preflight found a closure ownership compile error;
root corrected the three test loops to borrow their captured vectors. The ensuing
workspace run had421passing native tests, two failures, one ignored. Core117passed.
Preserved failure receipts are in `failures/`.

At station1852, the actual shoulder collider is0.488m above the road. The rail is
raised by its independent clearance rule, so the former road-height ray passed
below it. The revised probe derives its height from independent resident collision.
All post-foot rays, terminal contacts and full-footprint gap sweeps had passed before
that assertion. The recovery failure occurred at right-side gap1908: its synthetic
start used analytic off-road terrain and the runner ended beneath the shoulder.
The revised setup warms collision, then uses the actual Ground support ray to place
the starting capsule, matching production get-up support and the native fixture.
These changes await their focused rerun; no production rig/controller tuning.

The native survey uses the production grounded/airborne downward-speed rule, avoiding
the previously rejected strong downward drive on slopes. `run_guardrail_checks.py`
serializes real native runs using flock and records isolated completion receipts.
Five roadmap checks pass after updating per-part counts for all new open IDs.

Focused rerun03 passes both initial failing tests. Actual support measurements:
left gap2276 analytic start79.92342 vs collider79.45771; right gap1908 analytic
start63.509716 vs collider64.67205. Both now reach the actual wreck without rescue.
The old right fixture began1.162m too low. Longest-run midpoint and actual Valley
placed-support/reservations fixtures are added for the next focused run.

Focused run04 passes Valley actual support/reservations/endcaps, Hills geometry and
both gap-centre recoveries, but the new maximum-run midpoint fixture fails after
40simulated seconds at1852. Preserved as `failures/midrun-04.txt`. Production
player recovery previously had no authored rail access path. The follow-up adds
bounded two-candidate, collision-grounded and swept waypoints through actual ends
or gaps, sharing existing walk_path/controller/manual override and the one-plan
budget/1.5s retry. Focused run05 is pending. No rig or contact parameters change.

## Integrated checkpoint (2026-11-11)

Merged GitHub main at 37f53322c, preserving issue #28 movement, #27 civilian
overtaking, #41 hands, expanded scope, and all five open guardrail IDs. The two
focused Hills gap recovery cases pass after using resident collider support;
the 96m midpoint case now reaches the actual wreck through a bounded end/gap
path with zero exceptional rescues. No rig, controller speed, crash force or
bridge-edge physics changed. Strict workspace Clippy, format, source/hash audit
and five roadmap checks pass. The integrated locked workspace suite passes
623 tests (two explicit ignored benchmarks, zero failures). Debug build,
release native functional checks and close visual review remain pending at this
checkpoint.
The main branch's new four-city-ID scope is retained; README now totals
224 verified /297 open.
