# Original Road Rash crash calculations — 2026-11-01

This branch is an independent comparison against main, not the biomechanical
ragdoll branch. The calculation model is in `rr_core::original_crash`; the 3D
bridge is `src/original_crash.rs`. Run `cargo run --release -- --original-crashes`
to opt in. Without that flag the accepted RoadDash collision policy remains active.
`cargo run -- --original-crash-report` prints reproducible numeric examples
without opening a window. The committed JSON receipt contains the same report.

## Evidence and limits

Inspected arkits/road-rash commit
[`aa9cb6d3181e20736af71022cd4d6aac0e2788d1`](https://github.com/arkits/road-rash/tree/aa9cb6d3181e20736af71022cd4d6aac0e2788d1),
not an original EA source release. Primary paths:
[race.cpp](https://github.com/arkits/road-rash/blob/aa9cb6d3181e20736af71022cd4d6aac0e2788d1/src/race.cpp)
(`separate`, `contact`, `contact_car`, `knock_off`, `vertical`),
[crash.cpp](https://github.com/arkits/road-rash/blob/aa9cb6d3181e20736af71022cd4d6aac0e2788d1/src/crash.cpp)
(`begin_crash`, `advance_actor_motion`, `recover_crash_actor`), and
[bike.cpp](https://github.com/arkits/road-rash/blob/aa9cb6d3181e20736af71022cd4d6aac0e2788d1/src/bike.cpp)
(`integrate_speed`, object initialization).

Also inspected the user's retail `ROADRASH.EXE`, SHA256
`73d7f9744b5b1a9482b3909950cdf3fe34119fe13d023b031e9c6739a60f18a2`.
`objdump` and bounded Unicorn execution verify selected machine-code arithmetic
and data, independently of the reconstruction. No retail binary, sprites, audio,
C++ source or disassembly bytes are redistributed. Root source reuse permission
for the reconstruction remains unverified. This is an independent mathematical
implementation, not imported source. The full original game was not run; this is
not a claim of complete executable parity or original Windows gameplay testing.

A material discrepancy: the retail contact gate at **0x404ed9** subtracts the
other object's **+0x108 signed forward velocity**, then compares to 6000 and
branches when greater. The reconstruction's `contact_car` uses `car.speed`
(scalar), whereas its ordinary bike helper uses +0x108. This implementation
follows the verified signed native field. Oncoming speed therefore increases the
gap. The Rust parameter `partner_gap_speed` must receive that signed component.

## 1. Contact separation and raw velocity changes

Positions are road-relative fixed-point values, not arbitrary 3D world metres.
The original checks overlap using object width/length/height and previous
positions, selects longitudinal/lateral/vertical separation axes, then applies
mass-weighted positional corrections. A low object may be stacked/ridden over.
It also records whether the other object's width contains the rider's central
half-width (strict inequalities) and its height exceeds 8192.

For a selected axis, let `v1`, `v2` be its two native motion components and
`m1`, `m2` the original object's weighting fields. With `T = m1 + m2`:

```
moving partner: delta1 = trunc(wrap32(wrap32(v2-v1) * m2 * 2) / T)
static partner: delta1 = wrap32((v2-v1) * trunc(wrap32(2*m2) / T))
delta2 = -delta1
```

Intermediate products wrap to 32 bits. Division truncates toward zero; explicit
right shifts in the formulas below are arithmetic. Static ratio division happens
BEFORE multiplication: it can discard almost half the response. The partner
gets the negative of the first result, not a separately mass-weighted Newtonian
impulse. **Pair order therefore matters with unequal masses.** Do not substitute
a textbook elastic two-body solver and label that the original.

Original object templates read from this EXE: mounted bike weight **256**, rider
half **25**, traffic contact weight **1800**, contact elasticity **192/256**.
These native weights are not verified kilograms. Bike gravity/landing fields may
be overridden by BIKESPEC; the rider template does not use that override.

## 2. Magnitude, class, elasticity and damage

Raw longitudinal/lateral/vertical changes are `dx, dy, dz`:

```
full = abs(dx) + abs(dy) + abs(dz)
M = full >> 10
scaled_delta = trunc(raw_delta * elasticity / 256)   # each axis
```

Magnitude is calculated **before** elasticity; halving elasticity does not halve
damage. Class 0: low object (<8192), separate ride-over path. Class 3: glancing,
uncontained hit. Class 1: contained, other height <10240, non-car. Class 2:
contained tall object/car. The modern bridge models the three tall-contact classes.

Input damage: car contacts pass `(stamina=M, bike=M)`; bike contacts pass
`(stamina=0, bike=M>>4)`. The knock-off helper then scales both parameters:

| Reason | Meaning in this context | Damage multiplier |
| --- | --- | --- |
| 0 | low-object routing | 0 |
| 1 | bike contact | 1 |
| 2 | tall/car contact | 7/4, truncating |
| 3 | glance | 1/8, truncating |
| 4 | unarmed punch | 1 |
| 5 | other attack path | 1 |
| other | e.g. wall entry reason 6 | 0 in this helper |

For scaled stamina damage `p`, bike damage `q`:

```
max_stamina = max(base_stamina/2, max_stamina - 4*p)
stamina     = max(0, stamina - 64*p)
bike_health = max(0, bike_health - q)
knocked_off = stamina == 0 OR bike_health == 0
```

This describes the local/offline branch. Remote-owner bypasses, sound RNG,
attack hit detection and the separate reason-5 lateral multiplier are not part
of the new contact model. Existing RoadDash three-direct-hit combat stays intact.

## 3. Intact contact vs damage knockout

If damage did not already cause a crash, a contained class-1/2 contact checks:

```
bike.scalar_speed - other.forward_velocity >= 6001
```

Below that boundary, apply the scaled forward delta to bike speed. At/above it,
enter the **speed-gap crash** with its own impulse pair; this path does not first
subtract the forward contact delta. Equality at 6000 does not trigger it.

For a glancing hit that leaves the rider mounted, cancel lateral motion toward
the object, add `abs(scaled_forward_delta)/2` away from it, and subtract
`abs(scaled_forward_delta)/4` from speed (minimum zero). A glance that already
knocked the rider off instead adds `abs(scaled_forward_delta)/256` to the bike's
vertical velocity. Pure lateral contacts need not behave like frontal ones.

For a **damage knockout**, copy rider motion first. Class 2 then still changes
the **bike's** forward speed by scaled forward delta; class 1 does not. This
ordering can leave the rider flying forward while the bike reverses. It is a
real property of the recovered rule, not evidence of random launch direction.

The bike contact tail clamps scalar speed to **[-1000, 30000]**, vertical speed
to **[-10000, 1536]**, clears impulse/flag fields, separates positions sideways,
and may kick steering if the initial damage decision did not knock the rider off.
Only the velocity clamps are part of the pure contact result; positional nudges
and steering kicks are left to the modern collision/heading controller.

## 4. Crash entry is intentionally energetic

The original copies bike motion into the detached rider, then adds a call-site
specific pair in `(forward speed, steering-rate channel, vertical velocity)`:

| Entry path | Rider delta | Bike delta |
| --- | --- | --- |
| Damage knockout, usual | (-256, 0, 1792) | (0, 0, 1024) |
| Reason-4 unarmed knockout | (-80, 0, 0) | (0, 0, 0) |
| Intact contact above speed-gap threshold | (0, 0, 512) | (0, 0, 256) |
| Wall entry | (0, 0, 80) | (0, 0, 16) |
| Generic traction crash | (0, 0, 0) | (0, 0, 0) |

**The original adds energy on several paths.** A physically conservative policy
and exact reproduction of these boosts cannot both be claimed. Reason alone
is insufficient: class-2 damage and class-2 speed-gap use different pairs.
Retail checks: default/punch at **0x40535f–0x4053b2**; contact pair at
**0x404ee7–0x404f13**. Begin-crash is **0x403d60**.

Entry also raises the rider by 19200 height units and the bike by its clearance,
sets sliding fields to rider 70/bike 95, switches wreck dimensions to
(7680,64,2048), resets rider roll, and starts sprite 29 for reason 2, 35 for
reasons 4/5, otherwise 33. Those shape/height/sprite operations are research facts;
modern ragdoll geometry and safe detachment positions are retained.

## 5. Drag, gravity, bounce and recovery

Scalar drag (`0x4390c0`) is implemented and tested in the pure reference model:

```
a = abs(speed)
base = grounded ? a : (a >> 2)
contact_drag = grounded ? ((surface_drag*a)>>14) + sliding : 0
drag = (drag_gain*(base>>8))>>8
lateral_penalty = abs(lateral_force)>>4
accel = throttle - drag - contact_drag - lateral_penalty  # forward motion
speed_next = speed + dt*accel                           # stop without crossing 0
```

Negative motion uses positive braking acceleration and also stops at zero. The
rider template has drag gain 130 and sliding 70. The modern client uses Rapier
friction/damping after entry; it does not apply this scalar rule to each limb.

Original vertical motion (`0x43a190`) is separate: integrate height relative to
moving ground, multiply intermediate height by 100/99, apply native gravity, and
on a hard landing use `rebound = -trunc(rebound_gain*vy/256)` plus
`(rise+5)*(speed>>7)` (signed reverse variant for negative speed). Default rider
gravity is 74/tick, rebound gain 24, landing threshold -2560. These researched
bounce/terrain rules are documented, not transplanted over Rapier's 3D contacts.

Original tumble poses are sprite state, not anatomical muscles. Scalar speed
reaching zero starts get-up; following stages walk toward the bike, then pickup/
remount, with native distance thresholds and a timeout. RoadDash retains its
existing get-up/on-foot/mount system and eleven-body articulated rig.

## 6. Modern units and explicit boundaries

Forward conversion derives from the unrounded HUD coefficient:
`native_speed * 41/4096 mph`, then `mph * 0.44704 = m/s`.
Consequently 6001 is about **96.7 km/h** of relative forward speed.
Do NOT apply that scale to the native vertical or steering-rate channel.

The bridge calibrates vertical/lateral velocity with
`9.81/(74*60) m/s per native unit`, choosing Earth gravity at 60 Hz. This is a
modern authoring choice, not an original metre conversion. At this scale the
usual rider boost is about 3.96 m/s, speed-gap rider boost 1.13 m/s. The native
steering-rate channel is only approximated by local side velocity in the bridge.

The opt-in mode handles mounted bike/bike and bike/car (including cross-traffic)
contacts with pre-contact world velocities. Runtime contact axes/containment come
from upright local 3D geometry and approximate widths; they are not byte-identical
native previous-frame route boxes. Conditions start at stamina 2500/bike 1000 as
comparison tuning, reset after remount/restart, and do not replace saved progression
or current combat. These are not a claim of recovered individual rival stats.
Pair ordering is approximated by the sign of longitudinal position; traffic
movement stays under the modern traffic controller, rather than receiving the
reciprocal native delta. Negative mounted speed is clamped by modern driving.

Rider/bike entry velocities are separate; the old forward-only crash constraint
is disabled for these contacts so original reversals can be observed. The modern
aggregate energy bound remains, seeded with the calculated launch velocities plus
gravity. All other contacts (static obstacles, civilians, downed bodies), combat,
traction, recovery, heading control and rig construction keep existing behavior.
The extra original entry variants and drag can be exercised as pure rules; this
branch is **not a complete port of every original crash caller and trajectory**.
See CRASHREF-004 for the remaining scope, rather than treating this comparison as
original-physics parity. No dependency or asset changes are required by the game.

## Reproduce the native verification

```
cargo run -- --original-crash-report > /tmp/original-crash-report.json
python3 -m venv /tmp/original-crash-verify
/tmp/original-crash-verify/bin/pip install unicorn==2.1.4
/tmp/original-crash-verify/bin/python tools/verify_original_crash.py \
  /path/to/your/ROADRASH.EXE /tmp/original-crash-report.json
```

The tool rejects other EXE hashes. It executes bounded retail blocks and the
scalar drag function, stubbing only surface classification with a selected table
input. It checks Rust-generated expectations, not a Python rewrite of the same
math. Retail-dependent checks are optional and never silently skipped by CI:
normal CI runs Rust rules/integration tests, while this manual tool requires both
Unicorn and the user's original data. Results are in the testing receipt.
