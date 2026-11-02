# Downed contact routing — 2026-11-01

COLLISION-04, Part01 origin, now verified in Part00. Branch:
`codex/original-contact-routing`. No commit/push/merge performed. Existing large
city/catalog work in this checkout is preserved.

## Result and scope

Removed the >0.75m/s unconditional wipeout on any foreign OwnedBody. Ragdoll and
wreck contacts retain solid Rapier collision but do not script another mounted
wipeout. Wheel support accepts actually low geometry (height/top <=0.8m relative
to road); assisted motion raises/re-sweeps clearance, leaving tall blockers solid.
Mounted glances nudge rather than entering the direct closing-speed gate.
Standing runners remain separately hittable; repeat knockdowns inherit their
current movement. Parked cars no longer repeatedly knock recovering runners down.
Ordinary car/tall obstacle response and120/40=>80km/h entry remain unchanged.

This is independently written original-style routing, not full retail collision
parity. Modern50km/h direct gate and0.8m support calibration are not native6001/
height units. Existing CRASHREF parity gates remain open. No original source,
media, dependency, ragdoll body specs, masses, joints, motors or damping imported
or changed. ADR045 and R01 follow-up record research and tradeoffs.

## Verification

- `cargo fmt --all --check`: pass (`fmt.txt`).
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: pass (`clippy.txt`).
- `cargo test --workspace --locked`:176 passed,0 failed,1 explicitly ignored
  pre-existing ten-minute soak (`tests.txt`). Includes three new core policy tests,
  player/AI × both crash modes × own/foreign × ragdoll/wreck matrix with eleven
  solid contacts per case, and dynamic physical traversal of a0.6m-high wreck.
  The latter checks forward passage, actual physical response on the wreck,
  remaining solid collider and no new falling state. Existing runner re-knock,
  car momentum, cross-traffic, heading, energy, remount and body differential
  tests also pass. Headless tests run real Rapier; they do not prove subjective feel.
- `cargo build --locked`: pass (`build.txt`).
- `python3 tools/audit.py`: pass after refreshing the changed original vehicle
  source hash (`audit.txt`); declaration audit, not legal clearance.
- `python3 tools/test_roadmap.py`: pass (`roadmap.txt`).
- Native `timeout 70s target/debug/road-dash --smoke-test`: pass; City Run renders,
  retries, two crashes and both recoveries, then finishes (`native-smoke.txt`,
  `smoke-metrics.json`, `native-crash.png`, visually inspected). This is regression
  smoke, not an interactive pile-up approval or performance benchmark. Existing
  unmapped gamepad/suboptimal Vulkan presentation warnings remain.

Initial regression run exposed changed car gates; those changes were removed.
First physical fixture was only0.36m high and did not contact the chassis; the
final0.6m fixture verifies actual solid contact. No tests were weakened to hide a
runtime failure. Windows hardware and exact retail airborne-debris behavior were
not verified. Next: interactive pile-up feel and existing CRASHREF acceptance.
