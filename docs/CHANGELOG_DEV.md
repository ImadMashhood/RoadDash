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

## 2026-10-31 — Initial city/combat follow-up (partial)

Added buffered Space auto-target lateral kicks and mouse punches; excluded rival rider roots from camera obstruction. Added kick edge consumption/context-clear test. CITY-01, COMBAT-01 and RIDE-01 remain unchecked. No city geometry, original-spacing calibration or lean/contact-heading fix is claimed. Original disc remains outside Git.

## 2026-10-31 — Complete city/combat correction

Added 410 supported solid city buildings, four avenues on each side, cross streets, closer frontages, curb-adjacent sidewalks, window facades and batched street meshes. Retained original 247 IDs and course gates. Verified original City lane/shoulder metadata with local disc, no original media shipped. Completed typed auto-side kicks, leg poses, mouse punches, supported contact-yaw stabilization and seven symmetric steering-only banks. Corrected camera obstruction by rivals. 112 tests passed; format/Clippy/debug/release/audit passed. Native 1080p city survey: 657 roots and 9.56 ms p95; farm/hill crash-recovery-to-finish smoke passed. CITY-01/COMBAT-01/RIDE-01 checked; no next part started.

## 2026-10-31 — CITY-02 shared-edge frontage correction

Closed detached-city gaps with 347 solid full-depth infill sections, procedural windows and flat facade normals. Exact shared edges follow curved blocks. Streets and unrelated buildings are excluded; all original lot IDs remain. Recovery now searches around joined blocks and excludes interior graph corners. Added edge/road-clearance regression and physical ray coverage for every infill. All 113 tests, fmt, Clippy, debug/release builds, audit and source-input checks passed. Native road-height/grid views reviewed; survey 9.79 ms p95 at 1080p; crash/remount/finish smoke passed. Evidence: testing/FRONTAGES.md.

## 2026-10-31 — CITY-03 sidewalk, tree and skyline correction

Replaced rounded sidewalk shading with flat normals and batched concrete paving
with expansion joints. Added original collidable street trees in flush curbside
beds, retaining over 2 m beside trunks and clear junctions. Increased urban floors
with an authored 1.6 multiplier and consistent facade/roof/collision heights;
preserved source snapshots, lot IDs, quotas and rural heights. ADR-024 records
the design. 115 tests, locked builds, checks/audit, native survey (9.66 ms p95 at
1080p) and crash/remount/finish smoke pass. Evidence: testing/STREETSCAPE.md.

## 2026-10-31 — Roster, kicks, steering, wrecks and pursuit

User corrections: stronger kicks, fewer sidewalk trees, original roster/color
schemes, less automatic steering, fallen motorcycles, 50% stronger turning and
measured aggressive rivals. Implemented 15-racer CITY lineup and independent
helmet/shirt/bike swatches, 8 m/s kicks, 44 m tree spacing, body-heading player
forces, stronger authored turn response/caps, physical wreck roll and deterministic
pursuit with speed matching/selective downed hunts. R01 inspected reference-only;
no source/media copied. ADR-025 and testing/RIDERS.md record verification/limits.

## 2026-10-31 — AI-02 individual personalities and rear starting grid

Replaced generic pursuit/periodic attacks and ID-based speed with validated
original level-zero profile observations and relationship-driven decisions.
Integrated individual pace, reach, kick odds, strength, attack stamina and physical
rider weight; added per-rider hit responses/return to baseline. Player starts last,
12 m between racers, with extended solid approach and negative-progress projection.
Retry accepts legacy shorter grids. Original full-career/random/event parity is
tracked under AI-03; ADR-026 and testing/PERSONALITIES.md record scope/evidence.

AI-02 verification: 120 tests, format/Clippy/locked builds, provenance/source checks,
final world inspection and native survey/smoke pass (1080p p95 14.74 ms).
Native testing caught and fixed terrain intrusion beneath the expanded approach;
new test rejects that condition. AI-02 closed; AI-03 full parity stays open.

## 2026-10-31 — ART-01 character-source research

Reviewed user art references and Synty, Quaternius and a CGTrader biker candidate.
Recorded styles, source formats, rig claims, costs and redistribution limitations
in third_party/CHARACTER_CANDIDATES.md. Found conflicting CC0 pack labels versus
Quaternius's current QAL; no new archive license inferred. No assets purchased,
downloaded or integrated. ART-01 research complete; ART-02 pilot integration open.

## 2026-10-31 — ART-03 authored models, unchanged accepted ragdoll

- Created codex/seated-models-preserve-ragdoll for the explicit user request.
- Integrated the independently authored textured rider and sport motorcycle,
  preserving roster colors, existing procedural gestures/lean, runner/pedestrian
  appearance, detached skin/wreck lifecycle and all collision simulation.
- Added a bounded visual-only mesh intake and reproducible GLB/atlas export;
  kept sources outside runtime assets and registered provenance/hashes.
- GPU skin follows existing eleven visual joints with standing/seated bind
  surfaces. Recovery has three graphics-hook lines; normalized remaining source,
  body specifications, vehicle controller and lockfile match the baseline.
- Differential tests find zero maximum body position difference across eleven
  bodies/180 fixed ticks; full recovery and skin cleanup match. 125 tests pass
  (one optional soak ignored), format/Clippy/locked builds and audit/source tests
  pass. Native race/NPC crash/remount/audio and world inspection/survey pass;
  warmed 1080p city p95 12.90 ms on this host. See testing/MODELS.md.
- Corrected headless fixture resource requirements and runtime asset lookup found
  during verification; final checks were rerun. Added texture readiness polling
  and linear normal/roughness loading. Existing physics/camera/input behavior was
  not retuned. A legacy-visual environment switch permits direct comparison.
- ART-03 checked; broader ART-02 production acceptance stays open. Added ART-04
  animation/deformation polish and ART-05 LOD/more-hardware gates. No push/PR,
  platform certification or legal clearance claimed. ADR-027 records the choice.

### 2026-10-31 — V2 motion corrections and revised execution roadmap

- Fixed atomic retry across logical state, Bevy transforms/history and Rapier
  current/next poses. Native R-key regression restores all fifteen grid positions.
- Replaced saturated AI lane commands with proportionate gradual steering;
  preserved player handling. Mounted detachment uses pre-contact world momentum.
- Replaced disconnected visual pivots with original source-anatomy bind frames,
  fixed-length parent chains and bounded elbow/knee flex, without retuning the
  accepted eleven-body physical ragdoll. Exporter and intake validate bind metadata.
- Preserved all 294 historical tasks/statuses; introduced 54 V2 tasks for the new
  Parts 01–08 sequence, then original 09/10. Checked only four verified repairs.
  Vault/combat swerve and full animation/world production remain explicitly open.
- Screened five animation/ragdoll repositories and publisher Synty showcases;
  recorded pinned versions/license findings, no third-party code/media adopted.
  ADR-028–030 and testing/MOTION_REPAIRS.md document decisions and evidence.
- 130 Rust tests pass (one optional soak ignored), format/Clippy/locked builds and
  native race/actor smoke pass. No dependency migration or physics mass/joint/
  motor/damping retuning. Isolated body trace remains identical; full remount
  comparison allows documented 2 mm f32 projection tolerance.

- Final 15-section survey passes at 1080p warmed city p95 12.25 ms on this host;
  nine Python tests, progress sync and provenance audit pass.

### 2026-10-31 — Completed-first README and consolidated remaining backlog

- User explicitly requested all completed pieces earlier and all unfinished pieces
  later. Moved 128 verified requirements into Part 00; placed all 220 open tasks
  in Parts 01–16, following the requested animation/world/weapons/police sequence
  before original playtest/network gates. No status, ID or requirement changed.
- Brought all original unfinished tasks back into README. Historical P01–P08/BW
  file is now frozen context; README alone drives PROGRESS. Retained dependency,
  environment/audio inventories, budgets and rights prose in supporting contracts.
- Added machine-readable current placements and pre-move states, navigation and
  counts; two new regression checks protect ordering/coverage and summary totals.
  No new roadmap IDs, runtime changes, assets or external research. ADR-031 and
  testing/README_REORGANIZATION.md record verification and maintenance rules.

### 2026-10-31 — Dedicated full animation production section

- Expanded README into a prominent standalone animation section with eleven
  action families, original-sprite-to-semantic-keyframe mapping, numbered gallery
  deliverables, research candidates and two explicit production stages. Existing
  V2-02/V2-03 tasks own this coverage; no new IDs or completion changes.
- Included gradual AI banks, contact anchors, combat loss of balance, measured
  impact/vault poses, bounded ragdoll reactions, terrain gait, get-up/remount,
  weapon/police hooks and separate human/quadruped motion. Implementation remains
  open. No new source/media/dependency imported or newly investigated repository.

### 2026-10-31 — Collision yaw, velocity and energy corrections

- Published and merged the full animation roadmap and previously local motion/
  tracking changes in PR #8 (merge 1b600d4). Its tree matches the validated local
  snapshot. Continued new corrections on codex/collision-momentum-input-heading.
- Mounted yaw now follows fixed steering commands rather than contact spin.
  Implemented the user's explicit 120/40 -> 80 km/h arcade rule for both car
  directions, including AI, with no reversed entry and a dissipative mass/inertia/
  gravity crash envelope. No artificial height hops or body/wreck spawn spin;
  wrecks topple through gravity from an unstable lean. Civilian knockdowns remain.
- Added COLLISION-01/02/03 (Part 01), with reason/dependencies/acceptance and tests.
  Earlier clearance-jump task remains historical; full vault/swerve stays open.
  No dependency, original media or body/joint/motor/damping migration. ADR-032
  records the arcade policy and comparison limits.
- Corrected the soak's nonzero neutral-yaw expectation and insufficient fixture
  floor, retaining the full 600 s drive and numeric stability bounds. Added real
  car-contact tests and roadmap tests to CI. Receipts: testing/COLLISION_RESPONSE.md.

- Verified COLLISION-01/02/03 and moved them to Part 00; 131 verified tasks, 220
  open, all 351 stable IDs retained. 136 regular Rust tests, explicit ten-minute
  soak and eleven Python tests pass, plus native race/actor/audio and validators.
  Final city survey p95 12.79 ms. No broader V2 exit gates were checked.

## 2026-10-31 / 2026-11-01 — Animation A reference and static keyframe foundation

- Started the requested first animation stage on codex/animation-reference-keyframes,
  independently of the previous PR's CI/merge. No runtime physics/model changes.
- Preserved 130 old mounted IDs; authored 117 additional candidates for a total
  of 247 draft poses / 27 use cases, explicit contacts/confidence/unknown timing.
- Added reproducible Blender tooling, 247 editable single-frame pose Actions,
  independent round-trip verification, offline gallery, primary/supplemental views
  and original quadruped review proxy. No external source/media imported.
- Inventoried 326 rider / 45 police reference slots locally, reviewed source helper
  anchors and five pinned animation/IK/ragdoll repositories; recorded source and
  licensing limits, including missing native sequence timing and uncertain labels.
- Added rr_content schema/rig/contact validator and --validate-animation CLI;
  140 Rust and 14 Python tests, format, Clippy and build pass. All 247 Blender
  reloads pass; accepted model-body physics parity remains zero difference.
- ADR-033 separates static draft vocabulary from runtime playback. ANIM-01 and
  V2-02-004 verified; ANIM-02/03/04 added for hands, native sequence interpretation
  and support/specialist art review. README/progress preserve open approval gates.

## 2026-11-01 — Correct animation context before pose reconstruction

- User rejected V1 semantics; explicitly superseded inspection-helper action
  labels without erasing historical artifacts or claiming approval.
- Added independent bounded retail metadata reader and source-ordered catalog:
 104 bank/code slots,194 descriptors,371 sprite-slot usage entries, actual mirrors,
  controls/fallbacks/logical timing, repeated phases and explicit unknown callers.
- Cross-checked194 descriptors/rear selections against the separately compiled
  pinned reference library outside RoadDash. No C++/retail media imported; no
  native-original execution claim. Punches176–179, kicks225/226 and gesture44/45
  now have their correct selector contexts.
- Rebuilt isolated138 human poses /16 bike layouts using semantic phase names,
  original weapon proxies, authoring fist keys and complete assembly loader.
 138 static Actions round-trip; all art remains draft and runtime physics intact.
- ANIM-05 verified; ANIM-06 retains fidelity/unknown-binding/approval work. ADR-034,
  source/provenance/testing records and README/progress updated together.

## 2026-11-01 — Full-body corrections and first runtime clip integration

- Added all 154 own frame images to PR #10; merged at d9e791ee2987ed9ddc482e4e51d5339c721b46cd.
- Corrected pelvis/spine/chest/head motion for mounted attacks and tuck; kicks
  release the seat and raise hips while fitting retained grip/peg support.
  Regenerated 138 human poses, 16 bike layouts and private source comparisons.
- Added validated 36-key / 13-clip full hierarchy, reference-informed ride/tuck
  selection, fixed-step snapshots, turn/attack/recovery transitions, contact
  constraints and runner/bike coordinate handoff. Passive physics unchanged.
- Produced nine original animation GIFs and a separate editable Blender file with
  36 motion/bank/lift/fist Actions. These are authoring renders, not native footage.
- 150 Rust tests / 20 Python tests; Clippy, format, build and native race/recovery
  smoke pass. See testing/ANIMATION_B.md for receipts and verification limits.
- ADR-035; ANIM-07/08 verified and ANIM-09 acceptance work added. Existing full
  production animation and pose-approval gates remain open, preserving history.

## 2026-11-01 — Independent original-crash calculation comparison

- Created codex/original-crash-calculations from main 5fac329f; biomechanical
  codex/biomechanical-ragdoll / draft PR12 left separate and unmerged.
- Investigated arkits aa9cb6d3181e20736af71022cd4d6aac0e2788d1 and the user's
  hashed retail EXE. Native x86 signed +0x108 contact gate differs from the
  reconstruction car helper's scalar field; original deliberately adds boosts.
- Added independent rr_core contact/separation/damage/entry/drag rules, opt-in
  modern bike/bike and bike/car bridge, separate rider/wreck launch results,
  persistent comparison conditions with reset, and a headless numeric report.
  Preserved default collision policy, original eleven-body rig and recovery.
- Added optional bounded retail verifier (Unicorn2.1.4 only in disposable venv),
  23 retail checks, nine core rule tests, three adapter tests and contact-to-remount
  integration test. Full tests: 163 pass, one existing ignored soak. Formatting,
  clippy, build, 20 Python tests and audit pass. Default and opt-in native window/
  audio smoke pass. Evidence/limitations: docs/testing/ORIGINAL_CRASHES.md.
- Added CRASHREF-001–004: first three narrow deliverables verified; remaining
  caller/geometry/traffic/trajectory parity open. README/progress now 364 IDs,
  139 complete / 225 open; corrected inherited stale 357-ID progress prose.
  Recorded ADR037, source/license research and remaining acceptance. No assets,
  original source/media or new game dependencies incorporated; no merge performed.
