# Road Rash handling investigation — 2026-10-30

Requested target: the handling in arkits/road-rash, rather than speculative modern
steering tuning. Inspected exact HEAD `4e052aca9552a5690c2b145b0f53856d5c31e980`
(2026-10-30). This is source research, not an independently executed reference game.
No C++ implementation, original binary/resource file or reference media imported.

## Confirmed steering behavior

[`bike.cpp`, steer / steer_input](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/bike.cpp#L241-L315):

- A digital left/right request selects a full steering angle (45 degrees in its
  internal sine convention). An angle-and-speed calculation requests a bounded
  steering force; it does not instantly assign a constant lateral velocity.
- Current steering force persists between ticks. A held direction moves it toward
  the target each tick, so turn strength builds. Release moves it toward zero;
  countersteering must unwind existing force before it points the other way.
- The rate differs when magnitude grows versus shrinks; bike/rider spec values
  determine those rates. The ordinary force cap is 0x2e00. Separate manual lean
  modifiers can increase the cap by a quarter and alter response rates; this is
  distinct from simply holding a turn longer.
- Steering depends on speed; stopped bikes have no steering-induced lateral
  motion after traction processing. Full turn output is capped, not unlimited.

[`traction`](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/bike.cpp#L403-L480)
uses the steering-force history for grip/damping/skid calculations and computes
lean from that force. At the documented ordinary force limit, the nominal bank
ratio is 46/74 before speed-dependent adjustments. Road curvature is not added to
this lean calculation. Road-bend displacement is handled separately in
[`Race::move`](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/race.cpp#L1950-L1998).

This confirms the user's recollection of stronger steering as a direction is held.
It does **not** establish an exact number of seconds for every original bike.

## Other handling paths inspected

| Subsystem | Verified source behavior | RoadDash exact-match status |
| --- | --- | --- |
| Throttle / brake | Persistent signed drive force, separate rise/fall rates; gear coefficients convert force; both input bits are processed in order | Existing RoadDash torque/brake model retained; not a verified original reproduction |
| Speed / drag | Drive minus contact/aero and lateral-demand losses, with fixed-point rounding and stopped/reverse guards | Existing 3D force/drag model retained; not exact |
| Gears | Spec-defined shift thresholds and ratios | Current authored six gears remain; original ratios unavailable |
| Traction | Surface grip, steer gain/damping, combined demand, skid state/duration, brake steering multiplier | Current combined 3D grip remains; exact tables/branches not reproduced |
| Nitro | Charges and minimum activation; multiplier on throttle force; charge drain/recharge and stop rules | Current regenerating boost retained; original charge behavior not reproduced |
| Road motion | Forward/lateral history, yaw derived from motion, bend displacement and vertical/contact state | 3D route/physics adapter retained; not fixed-point parity |

Source: [drive/throttle/brake/gear/speed functions](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/bike.cpp#L166-L359),
[think/move order](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/race.cpp#L3494-L3564),
[nitro](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/race.cpp#L4180-L4231).
The reference runs at 60 Hz (`race.hpp:27–30`). Its repository's claimed capture
parity was not independently reproduced in this session.

## Why the entire handling is not yet copied

[`bike.hpp:9–32`](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/bike.hpp#L9-L32)
and [`bike.cpp` loaders](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/bike.cpp#L21-L77)
require BIKESPEC.RSC (89 values per bike) and ROADRASH.EXE physics tables. These
supply mass/response, engine/braking, gear, nitro, surface, sine/atan and behavior
values. The public `synthetic()` helper is explicitly test data, not the original
handling. It cannot be substituted and described as an exact copy.

No such files were found in the project/Downloads search, and the user confirmed
that they do not have them. Full original-bike calibration/numeric replay remains
**blocked on verified, authorized reference values**, not checked off. Even if
values become available, SI/3D integration and fixed-point rounding need explicit
comparison rather than an assumption of equivalence.

No project-wide source reuse license was found in the inspected root/README.
`third_party/LICENSE` is Sean Barrett's third-party notice, not a grant for the
port's own bike/race source. Record R01 remains REFERENCE ONLY with source reuse
permission UNVERIFIED. We independently implement functional steering behavior;
we do not copy the C++ or extract/distribute original game files. The source's
README also requires an owner's original disc and includes no game-data grant.

## Current implemented correction

`rr_core::steering::SteeringMemory` independently implements persistent,
speed-dependent bounded turn buildup, distinct return rate and countersteering.
It feeds real fixed-tick player intentions in both riding adapters. Raw input and
resolved force are separately logged. Crash/reset clears memory. The native 3D
physics, existing acceleration/braking/gears/boost and requested collision/
ragdoll rules remain in place; AI retains its existing controller.

`content/handling.json` supplies **provisional RoadDash-authored** steering timing
(light 0.425/0.225 s, balanced 0.5/0.3 s, heavy 0.6/0.375 s rise/release; full target
speed 12 m/s). These include the user’s explicit 2x faster full-lean request; they are not recovered original-bike values or verified parity.
Schema-1 files without these fields receive validated defaults. P03-020 tracks
full source/data-based handling equivalence; it remains unchecked.

Bike and rider now bank together from accumulated steering only, with road bank
removed from the rendered roll. Grade/pitch and actual physical support remain.
Camera follow lag is bounded to compact near/default/far views, with velocity
feed-forward and at most three degrees of optional speed FOV. These are fixes for
the user's 3D presentation feedback, not copied sprite-camera behavior.

Evidence: [handling correction receipt](../testing/HANDLING_CORRECTION.md).

Latest turn-strength adjustment (P03-022): profile turn response increases 25%
(light 1.5, balanced 1.25, heavy 1.025). Dynamic steering and the assisted
adapter use the same profile gain. Maximum visual bank and the doubled lean
response are unchanged. These remain authored RoadDash settings.
