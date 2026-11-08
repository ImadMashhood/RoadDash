# World repair: first verified batch, full art work remains open

2026-11-07, branch `codex/world-repair-plan`, base 894ed7a.
[Measured plans and 3D art direction](../../planning/world-redesign/README.md).

## Delivered

- Both static visual backup cores fit behind the deepest actual cross-LOD facade
  bands, including bay scaling. Collision and authored exterior assets are unchanged.
  All 657 lots/four faces/three LODs are covered by a geometric regression.
- Static-wall contacts use normal closing speed: 1 km/h touches and fast parallel
  scrapes do not detach; hard direct impacts do. Existing car response remains covered.
- Visual leg IK now applies through pickup/running/getup, using a forward knee
  pole and same-side ankle targets. Segment lengths and held contacts remain tested.
  This changes animation, not the accepted physical rig or muscle implementation.
- Nearest rivals within 12 m choose the visual kick side even outside hit reach.
  Invalid/out-of-reach contacts still do no damage/remote push. Attack clips run at 1.8×
  with 55 ms entry / 80 ms exit; kick has its own synthesized impact and confirmed-hit
  camera feedback. Visible contact-frame hit resolution is still future work.
- Roster ranking sets ordinary rival pace to 94–106% of the modern player's
  ordinary top speed. Personalities/aggression stay authored; no positional catchup.
- 24 route cars (previously 12) brake for downed riders in lane, stop 5 m short, honk
  at most every 3 s and resume when cleared. A ten-second integrated scenario checks
  stopping, bounded horn count and resumption. Signed actual speed remains available
  to collision policy. General intersection/car-to-car traffic queues remain open.
- Rural track profiles are 7.2 m wide in farms and 6.8 m in hills, with 0.8 m verges.
  Widths interpolate into the unchanged wider city road. Traffic preserves its
  authored relative lane position through transitions. Road-support rays sample
  actual road widths, replacing old tests that assumed every carriageway was 13 m.

## Evidence actually executed

- `cargo fmt --all --check`, strict locked workspace Clippy, locked workspace
  tests and locked build. Final command logs are alongside this file.
- Workspace tests: 271 passed, 1 existing ignored. New queue/resume, knee-plane,
  low-speed wall, unreachable kick-side, pace and verge fixtures pass.
- `python3 tools/audit.py`, roadmap tests and synchronized progress check.
- Native large-facade 1080 render-frame sweep: 79 direct crossfades, bounded budgets.
  [Receipt](native-large-lod.json), [near](large-near.jpg), [mid-distance](large-mid.jpg).
  The survey camera can move outside the active player's terrain-stream corridor;
  its missing distant ground is not a claim about playable-world support.
- Native world survey completed 15 sections / 657 roots and actual geometry/budget
  assertions. **Performance failed:** 22.377508 ms p95 vs 16.7 ms at 1080p, isolated from
  compilation. [Receipt](native-world-survey.json), [city route](route-city.jpg).
  Do not call this a performance improvement or a production 60 FPS result.
- Native default Valley smoke passes farm crash/remount, city/hills, hill
  crash/remount and finish, with audio sinks active. [Receipt](native-smoke.json).
- Native human/cow actor smoke passes solid contact/lifecycle/entity bounds.
  [Receipt](native-actors.json). It does not approve civilian skin seams.
- Native prop census shows 28 traffic visuals: 24 route cars plus 4 existing crossings.
  [Receipt](native-valley-props.json).

## Findings and limits

The actual opaque-shell defect is different from the first zero-width shader-range
hypothesis. `cargo run --locked --bin visibility_probe` renders three reference
cubes (no range/default degenerate start/finite start) to `/tmp/roaddash-visibility-probe.png`.
On this GPU all three render: 4347 red / 3840 green / 4347 blue saturated pixels. That probe
**did not reproduce disappearance**; it is not evidence that zero-range math caused
this user's problem. The all-lot shell-depth test demonstrates the separate scaled
occlusion problem and exercises cases deeper than the old fixed inset.

LOD-03 stays open for prolonged high-speed/reverse-camera and user visual approval.
The city-wide material rebuild, property-led rearrangement, new landscape/sky art,
three/four-lane marking plan, civilian skin seam correction, contact-timed strikes,
subjective combat/race balance and Windows/hardware checks remain open. The current
3D Come Bye reference is recorded as art direction only; no game media/code copied.
No dependencies, proprietary textures or third-party source added.

Optional City Run moving-camera regression passed 381 ready crossfades
([receipt](native-city-lod.json)). Native animation preview completed eleven
motions/three view modes; walking front/side captures were visually inspected.
Pickup's 100-sample chain/plane checks passed, but a separate full pickup visual
playtest remains part of broader animation approval. Rival cruising wander now
also respects the narrower rural road with 0.9 m edge clearance; chasing retains
its explicitly selected target rather than forcing every AI onto the same line.

Final optional City Run gameplay smoke also passed two crashes, both recoveries
and race finish with the new traffic behavior integrated
([receipt](native-city-smoke.json)). No subjective audio or art approval is inferred.
