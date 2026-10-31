# Development history

## 2026-10-30 - Before Parts 00-01 (preserved baseline)

- 81485f6: initial procedural Bevy/Rapier racing prototype and nine tests.
- 03f12b2: corrected steering; physical scenery/vehicle interactions, attacker
  launch/bounce, ragdoll-follow camera, assisted get-up/run/remount, synthesized
  audio with existing CPAL ALSA recovery patch; 18 behavior tests recorded.
- a0902c9: preserved supplied blueprint and created README's 17-part checklist.

## 2026-10-30 - Parts 00-01 implementation

- Created permanent agent/progress/decision/known-issue/testing/provenance records,
  short prioritized GDD, one-page slice contract, content sheets and 23 immutable
  documentary research observations. No original game media/code imported.
- Added real core/content workspace libraries; retained compatible engine/physics
  pins after checking the newer plugin manifests. Created subsystem plugins and
  Boot/Loading/MainMenu/Race/Results lifecycle.
- Moved gameplay and Rapier into explicit 60 Hz fixed scheduling; buffered input
  edges before the loop and interpolated render-only children/camera afterward.
- Added validated authored IDs/tuning/keyboard bindings, legacy-safe schema-1
  profiles, local panic reports, F3 CPU/wall/fixed/entity/collider diagnostics,
  optional physics visualization and minimal offline feature build.
- Added Linux/Windows CI builds/headless tests and declared-license/media/research
  audits; excluded optional unverified LUT media and retained the OFL font notice.
- New acceptance tests cover 30/60/120 render rates, zero-tick input and 100
  crash/pause/resume/reset cycles. Execution receipts and remaining platform gates
  are recorded in testing/PARTS_00_01.md and PROGRESS.md.

- User-reported crash/recovery fixes: preserve momentum, choose separate clear
  rider placement, omit seated visuals from wrecks, remount at the rotated physical
  shape, stop runner impulse chasing and protect shoulder damage after remount.
  Repeat knockdowns use current runner velocity.
  Added raw velocity/separation and twelve fallen-bike approach regressions.

This entry describes implementation; only explicit receipts close acceptance.
No later gameplay part is automatically completed by this foundation work.

## 2026-10-30 - User fixes and Part 02 scenic point-to-point graybox

- User rejected a proposed return loop: shipped route is farmland -> city -> hills,
  with distinct start/finish and no endpoint wrap. Previous closed handling road
  remains only as an additional regression/tuning fixture.
- Added validated schema-1 authoring, Catmull-Rom centerline/width/bank/elevation/
  sectors, 3D arc length, ordered physical gates and endpoint/retry behavior.
- Added independent road meshes/collision, bank-safe terrain, seeded farm dressing,
  solid cows/lift contacts, city buildings/cross traffic/walking pedestrians and
  elevated hills/guardrails. Added CLI and F4 inspector; CI validates both tracks.
- Smooth boost release/exhaustion; positive grass movement, no grass-only damage;
  tighter data-driven steering with legacy catalog defaults.
- Replaced generic ragdoll ball limits with directional hinges, anatomical stops
  and bounded passive tissue motors. Wrecks slide on slopes, stop on flat ground,
  and remount height follows terrain. Retained controls/combat/audio/save behavior.
- Added full-route controller/gate test, endpoint/shortcut/import tests, road support
  rays, grass/boost/cow/ramp regressions, 100 physical drops and native farm/hill
  recovery + zone/inspector captures. See testing/PART_02.md for actual receipts.
- Branch/split/merge support and representative busy-race pacing remain explicitly
  incomplete; no automatic advancement to the next major production part.

- Prior foundation CI follow-up: Ubuntu passed; Windows builds/Rust tests passed
  but the metadata audit used cp1252. Switched metadata/text decoding to UTF-8,
  added a locale regression and kept the new remote acceptance gate open.

## 2026-10-30 — Part 03 dynamic vehicle and requested gameplay corrections

Native default now uses dynamic chassis/two-wheel suspension with three validated
handling presets, RPM/gears/combined traction, weaker assistance and bounded
telemetry. AI/player share intentions. Direct three-hit combos replace broad
proximity knockouts; relative impacts distinguish nudges from wipeouts. Original
cow/pedestrian articulated knockdowns have bounded safe lifecycle. Low sidewalks
support bikes; US traffic lanes and launch grace prevent start pileups. Minimap
colors match racer shirts only. Added curve-only outer hill rails, trees and
procedural building/cross-traffic/pedestrian detail. Fixed stale dynamic remount
velocity and preserved class dry wreck mass discovered during verification.

Detailed actual commands, measurements, native receipts and open gates:
[Part 03](testing/PART_03.md). No new dependency or third-party media. R03 studied
as reference only with unresolved reuse license; no code imported. Ten human
rides/beginner/fun acceptance remain open; no Part 04 advancement.

## 2026-10-30 — Part 04 follow-up to merged PR #3

Added compatible native gamepad backend; normalized analog engine/brake input,
continuous steering/freelook and deterministic disconnect fallback. F2/LB opens a
modal comfort/rebinding panel with versioned, corruption-safe preferences. Camera
uses interpolated heading, upright horizon, sphere-sweep clipping protection,
turn anticipation and warp snapping; FOV/shake/anti-nausea are configurable.
Regular HUD traction feedback and bounded optional speed lines; distinct on-foot
input context preserves assisted recovery. Added high-refresh camera, analog,
modal/rebinding/persistence tests. New SDL controller mapping data is audited with
its exact source/hash/Zlib notice. No Part 05 advancement; real controller and
external human acceptance remain open. Evidence: testing/PART_04.md.

Integration note: the user requested the same PR, but PR #3 was already merged
as dddcf3a before Part 04 publication. Preserve that history; use a new focused
follow-up branch/PR instead of appending invisible commits to the merged PR.

Additional user steering before publication: removed cosmetic steering lean;
shared seated/pedestrian/runner/ragdoll human meshes and outfit; increased/mass-
scaled civilian launch to clear both cow/pedestrian height while preserving
horizontal approach velocity. Crossing cars now have explicit wipeout handling,
all-type contacts and CCD. Added six physical clearance cases, four crossing-car
cases and model/no-roll regressions. No ordinary-crash launch was introduced.

## 2026-10-30 — Reference-steering investigation and chase/lean correction

Source-traced R01 commit 4e052aca; confirmed held steering ramps a persistent
force, with release/reversal/speed effects. User lacks original SPEC/EXE data, so
full handling parity remains explicitly open. Independently implemented pure
steering memory and validated provisional response settings, clearing crash/reset
history. Restored bike/rider bank together from input only; physical road/chassis
roll cannot introduce automatic visual lean. Compact camera distances with
velocity lead fix speed-dependent zoom-out; optional speed FOV limited to 3 deg.
Research, source URLs/rights, checklist and testing evidence recorded in
research/ROAD_RASH_HANDLING.md and testing/HANDLING_CORRECTION.md. No new
third-party dependency/media/code imported. No Part 05 advancement.

Latest user adjustment: doubled left/right full-lean response by halving all
authored buildup/return times and increasing visual response from 8 to 16 Hz.
Maximum steering force/lean angle unchanged; input-only bank still enforced.

Latest turn-strength adjustment (P03-022): profile turn response increases 25%
(light 1.5, balanced 1.25, heavy 1.025). Dynamic steering and the assisted
adapter use the same profile gain. Maximum visual bank and the doubled lean
response are unchanged. These remain authored RoadDash settings.

## 2026-10-30 — First-map structural generation foundation

Started B00–B02 from the user-supplied world PDF/JSON/CSV. Preserved source inputs,
converted exact 15-section/19-archetype/247-building quotas, added validated lot/
structural rules and deterministic planning allocations. Added headless world
validation/census, actual legacy building tags/counts and station-based district
blending/F4 preview. Added CI and source conversion checks; recorded the B03–B10
backlog and additive-overlay ADR-019. Existing scenery/route/gameplay retained;
247 new masses, lots, pads, streets and full-world performance are not claimed.
Exact checks and native captures: world/BASE_WORLD_TESTS.md.

## 2026-10-30 — First-map structural world B03–B10

Replaced canonical Valley legacy buildings with exact, deterministic 247-root
massing across 19 archetypes/15 sections, legal lots, sampled foundations and
bounded retaining repairs. Added 42 field parcels, nine solid junction stubs,
16 block faces, ramped sidewalks/outer approaches and access alleys. Added
building/terrain LOD with persistent collision, footprint-aware recovery routing
and exceptional rescue feedback/penalty. Added spatial/support/seam/capsule tests,
full JSON/SVG export and native 15-section/1080p survey. Exact measured gates and
remaining platform limits are recorded in world/COMPLETION_TESTS.md. No new media,
third-party source, dependency or additional course; prior evidence retained.
