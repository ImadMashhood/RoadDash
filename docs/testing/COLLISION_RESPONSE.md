# Collision response and mounted heading — 2026-10-31

This receipt describes the accepted **default** RoadDash policy. The separate
`--original-crashes` comparison intentionally uses recovered original contact
boosts and permits reversals; its scope and remaining gates are documented in
[ORIGINAL_CRASHES.md](ORIGINAL_CRASHES.md). It does not change the default or the
biomechanical proposal.

The full animation production section is merged in [PR #8](https://github.com/ImadMashhood/RoadDash/pull/8).
This follow-up implements COLLISION-01/02/03; authored animation clips and the
remaining V2 vault/combat-swerve design are not completed by these fixes.

## Implemented behavior

| Scenario | Entry policy |
| --- | --- |
| Rider 120 km/h, oncoming car 40 km/h | 80 km/h along rider's incoming horizontal direction; closing speed is 160 km/h |
| Rider 120 km/h, outgoing car 40 km/h, rear impact | 80 km/h along rider's incoming horizontal direction; closing speed is 80 km/h |
| Car speed greater than rider speed | Clamp retained speed to zero, never reverse it |
| Stationary obstacle | Preserve incoming entry speed; solid contacts dissipate it |
| Cow/pedestrian contact | Share velocity by inelastic source/target mass fraction; both react without the old scripted clearance jump |
| Mounted glancing contact | Translation/physical pitch/roll remain possible; contact spin does not turn steering yaw |
| Detached rider/wreck | Zero artificial spawn spin/upward kick; gravity and contacts tumble the original physical rig |

The speed subtraction is the user-requested arcade rule, not a real-world
collision equation. Both car velocities are sampled in world space; road and
cross traffic use authored signed direction/speed. The response seeds all twelve
detached bodies. Read-back mass/inertia and COM heights bound aggregate linear+
angular kinetic energy to the crash's initial kinetic plus released gravitational
potential. Negative aggregate approach-direction COM velocity is removed without
forcing every limb to point forward. Scaling only dissipates energy. Ordinary
solid collisions and original joint/motor/damping settings remain enabled.

The wreck begins at a 0.45 rad unstable lean with zero angular velocity so gravity
can topple it. Its initial tilted COM is included in the budget. There is no
fixed spin impulse or height-based launch. The old P06-022 clearance-jump receipt
is retained as superseded history. Deliberate melee shoves remain gameplay;
kinematic traffic means this is not full-world momentum conservation.

## Test evidence

See [command receipts](collision-response/checks.json), [workspace test output](collision-response/tests.log),
[real car contacts](collision-response/real-cars.log) and [600 s soak](collision-response/soak.log).
**136 regular Rust tests pass**, one optional test is excluded from the normal
suite; that complete ten-minute soak was also run explicitly and passes. Format,
warnings-denied workspace Clippy, locked debug/release builds, eleven Python
tests, tracker sync, handling/track/world validators and provenance audit pass.
The release/native checks preceded only test/CI/doc edits; runtime sources are
identical to that tested native binary.

Tests cover both car directions for player and AI (120/40 -> 80, twelve bodies,
zero scripted spin/upward velocity), actual physical rear/oncoming contacts,
reverse COM rejection, aggregate kinetic/angular/gravitational budget, and
repeated mounted contact spin at 30/60/120 Hz. The actual physical-contact test
records about **2.924 m forward rider travel over eight fixed ticks** in each
car-direction scenario, with no backwards flight.

Existing acceleration/braking, curbs, grass, stronger turn/bank, high-rate input,
NPC contact, slope sliding, gravity-led wreck toppling, retry, recovery/remount,
camera, finish and saved-payout tests remain covered. Original eleven-body visual
comparison records 180 ticks and zero maximum positional difference. Full-scene
remount now permits 2 cm contact-order/projection tolerance (observed 6–8 mm),
while stage timing, health and speed remain strict. This is explicitly broader
than the previous 2 mm full-scene tolerance; it is not claimed bit-identical.

Native Linux/RTX 3080/Vulkan race smoke passes retry of fifteen racers, farm/hill
crash → recovery → remount → finish and initialized audio. Separate actor smoke
passes bounded human/quadruped ragdolls and solid contacts. The fifteen-section
1920×1080 world survey reports warmed city wall-frame p95 **12.79 ms** versus
16.7 ms. This is a bounded host sample, not GPU timing or hardware certification.
Native builds used the existing host libudev pkg-config shim.

The prior PR's Windows CI passed; Linux failed an old soak assertion that required
turning with only straight throttle. After correcting that expectation, the small
1.3 km fixture floor ended at about 27 simulated seconds. Extending the isolated
soak floor to cover its possible path preserves the numeric bounds and completes
all 600 simulated seconds. The new soak measures neutral yaw, acceleration,
braking and continuous alternating-input stability. No test was disabled.

## Remaining scope and files

Runtime changes: rr_core::crash, vehicle/recovery/NPC adapters and fixed post-
physics ordering. Tests: collision_tests, vehicle/game regressions and explicit
full-scene comparison tolerance. README/PROGRESS/placement, ADR-032, history,
issues, test matrix and original-code asset hashes stay aligned. No new assets,
repositories investigated, dependencies or licenses adopted. New task IDs are
COLLISION-01/02/03 in Part 01, moved to Part 00 only after local verification.

Coarse car proxies can stop/pin a wreck. Full nose-down/over-car vaults, combat
swerve choreography, production animation, broader vehicle-shape/height scenarios,
new Windows runtime and user acceptance remain open. See README V2-01-005–008;
these broader gates must not close merely because the present corrections pass.
