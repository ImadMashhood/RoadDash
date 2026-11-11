# Development history

## 2026-11-11 — Bounded map atmosphere candidate (ADR156, issue #42)

Added original procedural transparent air cues on all four courses: terrain-seated feathered dust and rising heat bands on Hills, curb steam in Metropolis and City Run, and quiet Valley motes. A quality switch bounds the pools and disables them entirely at `ROADDASH_MAP_FX=0`; one immutable material per kind avoids per-frame uploads. The user rejected an initial thin sand stream, so the native-centered chase review drove a denser gray-brown translucent billow with the same pool count. The latest paired Hills view shows scenery through the dust; City Run shows two curb wisps after replacing an earlier round facade glow. Heat and Metropolis steam remain subtle. Fixed physics, Sand surface classification, saves and controls are unchanged. RFU-FX-01–04 and GPU/full-course/art acceptance remain open. See testing/issue-42-map-fx/REPORT.md for binary hashes, failed stages and final checks.

## 2026-11-10 — Metropolis atmosphere follow-up (ADR145, issue #30)

Reused the existing spatial clouds and Hills bird pool in Metropolis with six bounded flocks, independent quality controls and an opt-in moving native survey. Refined urban placement after visual review to use street approaches and authored civic openings. Preserved Hills dust and existing sun, physics, controls and recovery. The combined racer-AI/sand tree passed 661 workspace tests, strict checks/builds, final Metropolis visual proof and eight both-map quality runs. RFU-SKY-01..04 remain open for user visual, full-course/GPU/hardware and broader performance acceptance. No new media or dependency migration. Evidence: testing/metropolis-atmosphere/REPORT.md.

## 2026-11-08 — Random rival clubs and Part09 starter receipts

RIVAL-WEAPONS-01: seeded seven-of-fourteen starting rival clubs, validated configurable fraction, new-grid/retry-only assignment and player/police exclusion. Existing combat policy, launch grace, drop, visual sockets and physical rig retained. Native held/equipped checks and375 workspace tests passed. No new assets/dependencies; existing source/ability provenance hashes updated.

Started Part09 with an isolated CLI runner, binary/seed/status/timeout receipts and three failure/isolation tests. Real entry mode passed22 stages; full police/recovery suite, external sessions, fun metrics and frame-budget acceptance remain P09-AUTO-01/P09-001..007. Corrected obsolete P07-004 foot-finish status against existing foundations without closing the task. ADR092/093 and testing/random-rival-weapons / testing/part09-start retain details. Branch codex/random-rival-weapons; earlier frontend/art changes carried forward; uncommitted, no push/merge.

## 2026-11-08 — Native gritty entry UI (ADR091)

Integrated seven original generated scenes into Bevy: approved-style splash, main, street, shop and bar, plus new course-postcard and rider-introduction art. Independent original-inspired Thrash/course and Big Game/rider/street navigation now hand off to the existing race reset/countdown. Mouse/keyboard/controller edges are owned by the menu, settings stay connected, results retry or return without changing payouts. Existing physical rider/bike/crash/recovery remains unchanged. Pure rr_core flow and native regressions protect held-confirm, disabled navigation, profile continuity, launch/results and controller back. Native 22-stage flow passed after correcting result handoff and UI projection resize cache; saved actual viewport/bounds receipts and screenshots, including ultrawide/4K. Full campaign/shop/player-choice/art/accessibility/hardware acceptance remains UI-02/UI-05/P15-006/007; no new task IDs or broad checkboxes closed. No commit/push/merge.

## 2026-11-08 — Approved gritty UI style variations

User approved the corrected main-menu style. Generated four additional original scenes with built-in imagegen: biker bar, shady dealer, rival gang and crooked street, using the approved sample and private original screens as references. Saved unchanged PNGs, exact prompts, source IDs, hashes and a standalone gallery. No runtime changes or new task IDs; UI-05/UI-02 and individual art acceptance remain open. No commit/push/merge.

## 2026-11-08 — UI screen drafts and direction correction

Generated and placed 28 initial screen drafts with actual dimension/alpha/hash provenance, gallery and persistent review metadata. User rejected their polished 3D style. Studied private original splash/main/street/shop/bar/restroom screens, documented separate frontend direction, generated one original grotesque painted main-menu candidate, archived the first pass and disabled it by default. No runtime/physics changes, no art approval, no push/merge. UI-04 records technical historical draft availability; UI-05/UI-02 remain open. Evidence: docs/testing/generated-ui/README.md.


## 2026-11-08 — PERF-08 synchronization pass and UI-03 generation prompts

- Diagnose latest200-window local race capture;191 selected race windows show
  substantial physics/catch-up cost. Add separate backend sync/step/writeback,
  independent window mean and contact/body census with filtered/normalized CLI.
- Preserve exact static surfaces while removing immutable Fixed bodies and
  combining building collision by semantics rather than render material.
  2,920 source batches become2,529;2,606,411 collision triangles unchanged.
  Stable settled facade ranges/prop visibility/HUD text avoid redundant writes.
- Native survey median independent window mean16.83→14.67ms in the paired
  fixture; final rolling p9536.11ms remains over budget. Broad PERF-08 stays open.
  Default/control recoveries and18-case lab retain accepted gameplay/rigs.
- Add28 screen and142image prompts across66master families, deterministic
  generator/coverage tests, artist desk links and refreshed local ZIPs. No new
  art generated or integrated. UI-03 verified; UI-02/art remain open.
- ADR089; no dependency or external media changes.210verified/242open/452IDs.
  Full receipts/remaining gates: docs/testing/performance-ui-prompts/README.md.


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

## 2026-11-01 — First procedural facade authoring kit

- Created codex/procedural-facade-kit from merged original-crash main.
- Recorded inherited local 39-name city facade catalog; inspected three reference
  images and pinned R01 family/panel selection. No original pixels/source copied.
- Added editable three-family grammar and independent offline geometry exporter:
  12 width/height/corner variants, connected frontage, OBJ/MTL and preview.
- Added four authoring tests, source/provenance notes and ADR038. V2-04 tasks
  remain open/partial; this is not final production art or native integration.

## 2026-11-01 — Realistic building reference batch

- User requested ChatGPT image-generation edits across all 25 extracted sheets.
  Used built-in image generation once per sheet: 17 facade and eight structure/
  surface sheets covering 146 catalog IDs. Saved originals unchanged and all edits
  locally under art/buildings/realistic_sheets with prompts, source/output hashes
  and an unapproved-reference manifest. Packaged ZIP in Downloads.
- PNG decoding and expected filename coverage passed for 25/25; ZIP integrity
  passed. Generated signs/detail/layout may drift; precise UVs, material maps,
  modeling, production rights and art approval remain open. No gameplay changes,
  new dependencies, pushed assets or merged work. V2-04-002 remains unchecked.

## 2026-11-01 — Volumetric building concept batch

- Generated 25 three-quarter architectural concept sheets using built-in ChatGPT
  image generation, based on the realistic reference batch and a shared first-sheet
  style reference. Covers 146 catalog IDs, including surface/relief samples.
- Saved PNGs, exact prompts, source/output hashes and a row/column ID map under
  art/buildings/concept_3d. These are 2D concepts, not meshes; hidden geometry is
  inferred and some generated labels are absent. Art approval, production rights,
  modeling, UVs, LOD/collision and runtime gates remain open (V2-04-004).
- Verified all 25 PNGs decode and the download ZIP passes integrity testing.
  No gameplay changes; Rust/native tests not rerun for image-only work.

## 2026-11-01 — Procedural one-kilometre city study

- Created codex/city-kilometre-preview, preserving earlier uncommitted art sheets.
  Added editable seeded block/lot/facade generator and isolated native Bevy viewer
  (city_preview); default game and accepted ragdoll/physics remain unchanged.
- Generated 414 supported geometric building drafts from six concept-reference
  families in four touching rows along a 1 km avenue, ten blocks, parallel roads,
  rear courts, sidewalks, crosswalks, canopies/roofs, lamps and sparse trees.
  Geometry batches into 61 meshes; these are not final conversions of 146 designs.
- Added five authoring tests and one mesh winding/bounds test. Full workspace
  tests: 164 pass / one existing ignored; Python 29 pass; fmt, clippy, builds and
  asset audit pass. Captured and inspected four native views; explicit timing/
  smoke receipts in testing/city-preview. Production performance remains open.
- Recorded CITY-REF-01 (AGPL reference only, no copied source), authored generator
  provenance and ADR039; updated README/progress and known issues. No new task
  IDs or checked exit gates. No push, merge or game-map replacement performed.

## 2026-11-01 — Correct city preview to use the actual new facades

- User rejected the preceding palette-only result. Replaced generated window/sign
  fronts with the actual unchanged enhanced facade artwork: 100 IDs / 17 PNGs,
  source hashes, explicit body UV metadata and native texture loading. Existing
  images/prompts are preserved; no bitmap edit, new image generation or original
  retail texture import. All IDs are assigned before repeating the seeded catalog.
- Added continuous textured-relief surfaces, first-six detail profiles, roof/shell
  volume and end/rear returns in 416 buildings along the existing kilometre study.
  Added fifth native proof view, corrected mirrored lettering and tested source
  orientation on both road sides. Archived rejected generic captures.
- Full tests: 165 Rust pass, one prior soak ignored; 33 Python pass. Format,
  clippy, builds, audit and five native captures pass. Original new PNG hashes
  verified unchanged. See testing/city-preview for scope/timing and remaining gates.
- Recorded ADR040, local-only draft media provenance and limitations; updated
  README/progress without closing broad production-art/mesh/physics/gameplay gates.
  No new task IDs or repository investigation/dependency. Nothing pushed/merged.

## 2026-11-01 — Actual first-sheet building meshes

- Created codex/building-mesh-assets; preserved preceding uncommitted work.
  Authored six specific approved first-sheet designs in Blender4.5.1: actual
  masonry/openings/shopfronts/arches/canopy/bays/cornices/sides/roofs. Exported18
  embedded GLBs with3geometric LODs and retained editable source/materials.
- Independent GLB/clay checks caught buried brick geometry; corrected depth and
  re-exported. All six detailed assets now verify52.5mm projecting brickwork.
  No facade PNG/render card or original-game media is embedded in these meshes.
- Default city viewer now places actual shared GLB scenes, not earlier substitutes,
  with LOD/hysteresis and forced-high proof gallery. Native static scene import
  initially exposed missing reflection registrations; fixed in preview plugin
  without changing dependency features or default game/physics/ragdoll systems.
-167Rust tests pass /1prior soak ignored;36Python tests pass; format/clippy/build/
  audit and18GLB binary/material/geometry checks pass. Actual geometry, re-imported
  clay and five native views captured. Performance remains a preview-only sample.
- Added open MESH-01 (other94facades) and MESH-02 (46structures/surface references),
  synchronized README/progress/task placement/counts, recorded ADR041 and draft
  asset provenance. First-six material/fidelity review and production integration
  remain open. No new repository/source dependency or software source copied.


## 2026-11-01 — Full architectural draft catalog

Branch codex/all-building-models: showed existing kilometre map first, retained
six accepted models, then authored the remaining 140 IDs as actual geometric
facades/structures/surface modules. Added explicit classified parameters, bounded
sheet authoring, editable sources, GLB LODs, actual renders, reconciliation and
binary validation. Expanded isolated city preview to all 100 facades with lazy
high-detail loading and a 25-page catalog gallery. See
[verification](testing/building-catalog/README.md). Art approval and production
integration remain open; no racing/ragdoll source or dependency changes.


## 2026-11-01 — District and original-course catalog grouping

Branch codex/building-district-groups. Independently recorded all146 named CEL
memberships and five-course MSRR family load sets from user-local data (no pixel
extraction). Added13 authored district/support sets, five overlapping course
eligibility views, relative manifests/438 canonical GLB links, CSV and labeled
contact sheets. DISTRICT-01 verified; ADR043 separates research evidence from
inferred art direction. Current map selection/gameplay remains unchanged.
Evidence: [district tests](testing/building-districts/README.md).


## 2026-11-01 — Playable 5.3 km whole-building city

Branch codex/city-5300-buildings. User clarified actual playable race scope.
CITYGEN-01 adds977 complete exterior building lots,53 blocks/eight district zones,
85 original City-family-eligible facades,300 own-model geometric front LOD files,
shared roof/rear/end assembly and solid world support. New default City Run is
5,300m open/point-to-point; Valley remains selectable and unchanged. Added content
contract, deterministic bake/check, shared preview/game renderer, independent
colliders and foot routing. Existing crossing cars and physics are reused.
[Evidence](testing/city-5300/README.md) includes native retries/two recoveries/finish,
all-building roof rays and actual-game district survey. ADR044 records choices;
production art/platform/performance and fine collision gates remain open.

## 2026-11-01 — Downed rider/wreck contact routing

- On codex/original-contact-routing, replaced the unconditional foreign-body
  wipeout with separate downed/standing/mounted contact routing. Solid downed
  colliders remain; low geometry supports suspension and assisted clearance.
- Preserved car entry policy and accepted ragdoll body/joint configuration;
  standing runner knockdowns use current movement, parked cars do not repeatedly
  re-knock recovering runners. Added pure policy and runtime regression coverage.
- Studied pinned original race.cpp contact callbacks without copying source.
  ADR045 records modern calibration and remaining native parity limits.
- Verification receipt: docs/testing/contact-routing/README.md. Earlier city
  generation work in this checkout is preserved; no commit/push/merge performed.

## 2026-11-01 — Full world-prop source inventory and first concept sheet

- Created codex/prop-reference-catalog preserving all prior city/collision work.
- Scanned269 families,54 vehicle ANIM resources across five courses and global CELs;
  reconciled4172 CEL occurrences/1310 unique images into597 review banks,1063x2
  sheets, searchable local gallery and a369-entry prop rebuild queue. Eleven unknown
  groups remain visible. Architecture/material banks are explicit cross-references.
- Added independent bounds/coverage tests, local validated zip and metadata-only
  source inventory. Retail pixels remain in Downloads; no runtime source changed.
- Used imagegen skill for the first six new art-direction concepts matching building
  concepts; recorded IDs/hash/provenance, with concept/model states kept distinct.
- ADR046, PROP-01 and docs/testing/prop-catalog record verified extraction and open
  identity/model/material/placement/audio/native gates. No commit/push/merge.

## 2026-11-01 — Generated prop concepts, batch02

- User requested visible reference sheets and image generation. Showed six source
  selections, generated six corresponding3x2 concept sheets/36 subjects using the
  existing building concept style. Six imagegen calls plus two typography-cleanup
  edits; original drafts retained.39 unique IDs now have concepts including batch01.
- Cars, pedestrians, trees, utilities, street furniture and rural props are mapped
  by stable source ID/cell. Prompts, source-sheet hashes, output hashes, review and
  file mappings in art/props/concept_3d/BATCH_02.json / PROMPTS_BATCH_02.json.
- Corrected DEER2 to deer-crossing sign after visual inspection; BIGFNCN2 is a
  checked bollard, not a fence. Updated taxonomy/test and retained historical
  numbers/hashes used for generation. Active reference sheets106→105; original
  superseded sheet archived locally, no source-image coverage reduced.
- Saved all final sheets/project gallery and local comparison zip. No new meshes,
  rigs, runtime art, gameplay/physics, dependencies, commits, pushes or merges.
- Nine Python tests, formatting/Clippy/build/audit and176 workspace Rust tests
  pass (one existing soak ignored). Art approval/full-catalog/geometry/native gates
  stay open; no new checkbox completed or task ID introduced this session.


## 2026-11-01 — Full prop concept generation

Generated every remaining scoped prop/actor reference bank: 330 IDs, 59 sheets,
plus prior 39 IDs for 369/369 coverage across 66 selected sheets. Added resumable
manifest, source clarifications, serialized output receipts, searchable gallery,
provenance and portable zip. Retained three edited first drafts. Reconciled queue,
README/PROGRESS and stage separation; geometry/rig/runtime tasks remain open.
No gameplay/physics modifications, new dependencies or repository investigations.
Required Rust checks pass (176 tests; one existing ignored), nine Python and five
roadmap tests pass; coverage/hash/PNG/zip verification and audit pass. Browser
file-URL QA blocked by policy; native smoke not rerun. Branch remains
`codex/prop-reference-catalog`; no commit/push/merge requested or performed.


## 2026-11-01 — Draft 3D prop authoring and isolated inspection

Created codex/prop-mesh-catalog. Authored 369 source-mapped volumetric drafts using
85 semantic recipes, 1,107 self-contained GLBs/three LODs and 13 editable Blender
category scenes. Generated 66 proof sheets by re-importing exported GLBs and an
additional untextured parked-vehicle proof. Added native prop_preview inspector,
ID/LOD/material/binary validation, malformed-input regressions and packaging.
Fixed incorrect export colors, hidden reflector bands, a truck cabin gap and the
viewer's missing reflected scene types. No changes to race/physics/accepted rider.

PROP-02 verifies only the draft export/inspection infrastructure; V2-05-002 and
V2-06-001 retain faithful art, anatomy, skinning, colliders, placement and production
gates. Source recipes are rough interpretations, not finished copies of concepts.
No new dependencies/repository research/source copying; generated art registered
local-only with unresolved release-design review. Tests/evidence in
docs/testing/prop-meshes. No commit, push or merge requested/performed.


## 2026-11-01 — Apply draft props in gameplay (PROP-03)

- Added validated prop intake/asset source, bounds normalization and distance LODs;
  replaced selected traffic/scenery visuals while retaining physical owners.
- Added reserved city sidewalk/parallel-street fixtures with independent colliders.
- Generated 118 visual-only human/cow skins; existing walking and interpolated
  ragdoll frames drive them. Body construction, ejection and recovery remain intact.
- Added physical-spec/momentum/bounded-contact and socket-clearance regressions,
  independent binary skin checks and actual scene-selected ID receipts. Native
  city survey, Valley actor knockdowns and City crash/recovery/finish smoke pass.
- Recorded ADR048, limitations and development-only asset provenance. Final art,
  full placement/behavior/audio/performance/release approval remain open. No new
  dependency/repository research or source/media reuse. No push/merge performed.
- Full files, evidence, metrics and next steps: testing/props-in-game/README.md.

## 2026-11-01 — Optional biomechanical ragdoll comparison

- Merged animation PR #11 at 5fac329fce868e9e8db9fd5caa930624f84b2770;
  created codex/biomechanical-ragdoll for the explicitly unmerged comparison.
- Investigated actual code/licenses in all five user-nominated repositories;
  selected an original native Rapier integration, no package/source/media imports.
- Added 16-body anatomy, safe angular charts, hinge stops, passive/active capped
  motors, bind-calibrated pose release, pre-contact spin/COM handoff, muscle
  curves, collision-dependent decay and natural backend sleep. Original eleven
  bodies remain selectable with their original settings.
- Added validated central JSON, F7 diagnostics, nine headless fixtures in both
  modes and standalone native comparison/reset/impulse controls. Preserved race
  controls, existing collision/ejection rules, recovery, audio and saved progress.
- ADR-036 and RAG-01–06 distinguish verified implementation from remaining skin,
  native realism and production performance/user acceptance. Evidence, exact
  checks and remaining limitations: testing/RAGDOLL_BIOMECHANICS.md.


## 2026-11-02 — Run the comparison rig without B

The user requested automatic new-ragdoll selection. Shared game/lab launch parsing
now defaults to biomechanical on the isolated comparison branch, preserves explicit
baseline and CLI precedence, and rejects conflicting flags. B remains lab-only.
No physics values, assets, dependencies or main changes. Extended RAG-04 launch
coverage; no new roadmap scope or exit gates closed. Verification receipts:
testing/ragdoll/default-launch/.


## 2026-11-06 — Approved biomechanics integration and contact feedback

RAG-07 combines the reviewed16-body articulation with current city/props and crash
routing, including preservation of original allow-reverse envelopes. New physics
starts automatically; explicit baseline remains. RAG-08 adds mirrored hip medial
stops, stronger persistent hip/knee resistance and8 extra solver iterations.
COLLISION-05 adds bounded rear/downed ride-over hops, target knockdown/push and
cooldown while retaining head-on/car rules. User explicitly authorized merge.
No dependencies or new source/media research. README/progress and task placement
reconciled; detailed tests, known limits and files: testing/ragdoll/integration/README.md.


## 2026-11-06 — Part 03 animation/recovery and user pose corrections

- Branch codex/part-03-animation-recovery from merged main038d0b1. Corrected
  resolved punch/kick sides and inward tuck/lean elbows while preserving grips.
- Added pilot16-joint visual skin, independent distal articulation/weights,
  distance-driven gait, sole support/stance IK and physical-landing get-up blend.
  Body masses, joint limits, passive tuning, crash/contact policy unchanged.
- Added validated recovery tuning/pure foot priority, camera-relative WASD/stick,
  delayed/optional assist, safe manual interaction and shared once-only foot/bike
  finish/payout; actual-crash5m finish and reverse/skipped-gate tests pass.
- Added capped supported player bracing through existing native motors; no direct
  root impulses. Added native eight-motion/three-view animation review command.
- Updated current roadmap/evidence together;10 newly verified requirements moved
  to Part00, broad Part03 art/terrain/performance gates remain open. Added ANIM-10,
  ANIM-11, RAG-09 verified subset records and ANIM-12 default-stack loader issue.
- No new dependencies, third-party source/assets or engine migration. ADR050,
  testing/animation-part03/README.md record checks, limitations and next steps.
  This session leaves changes local; no push/merge is implied.

## 2026-11-06 — Camera/contact/gait/crash-style follow-up and straight wrists

- Implemented requested static-only camera routing with ceiling fallback and safe
  look orientation; mounted/on-foot/foreign-ragdoll cases at60/144Hz pass.
- Superseded upright rear hops with mass-weighted normal momentum sharing and
  60km/h closing-speed front knockoffs. Retained separate downed/head-on policies.
- Replaced old gait playback with continuous procedural locomotion, stable knee
  poles and lower held-arm elbows. Native review exposed distal neutral calibration
  mismatch; fixed parent-relative frames and removed standing wrist twist. Added
  neutral standing/seated wrist attachment/axis regression.
- Added supported energy-paid one-shot rolls and capped frontal hip/knee release;
  baseline/motor limits preserved. Native --crash-motion-demo supports comparison.
- Restored Valley default and explicit city alias. Added CAM-02, COLLISION-06,
  ANIM-13, RAG-10, MAP-01 verified implementation records; production gates remain
  open.167 checked/223 open,390 IDs. ADR051 and testing/recovery-polish record
  design, checks, screenshots, provenance and limitations. No new dependencies,
  imported source/media, push or merge.

## 2026-11-06 — Part 04 authored buildings on the original Valley map

- Branch codex/part-04-world-buildings from main 825a5a1 (merged PR15). Connected
  existing own-authored architecture to the restored default Valley map; no new
  gameplay, ragdoll/physics, track, save or dependency changes. Explicit City Run
  retained. Kept 247 canonical quotas  + 410 neighborhood lots, all 19 roles / 15 sections,
  stable IDs, geometry fingerprint, rights of way, level supports and colliders.
- Derived 128 design ground/upper/crown kits: 384 GLBs, 34 shared own-material PNGs.
  Source-generator floor cuts preserve storefronts/windows; real repeated upper
  floors fill tall lots. Exact vertex indexing/shared images reduce duplicate
  storage; original model hashes stay unchanged. Added independent binary intake,
  source/output manifests/checksums and provenance register entries.
- Added typed rr_content grammar validation, deterministic block/lot variants,
  bounded pending/active detail, LOD hysteresis/48-entry cache, coarse ornament
  shadow reduction and native actual-mesh/collider census. Survey now proves
  loaded geometry rather than only allocation metadata. Preserved AppExit errors
  as process status so failed native acceptance cannot silently return success.
- Mandatory fmt/strict clippy/231 workspace tests/locked build pass ; 1 existing ignored
  soak. Audit 519 declarations / 37 runtime records / 27 observations and2 audit / 5 roadmap tests
  pass. Full native route views and smoke preserve two crashes/remounts and finish,
  four audio sinks and default-stack Valley load. Native frame target remains
  unmet; see testing/architecture-valley for failed benchmark measurements.
- KIT-01/02 and V2-04-005 verified, KIT-03 newly open.170 checked / 223 open / 393 IDs.
  Partial grammar/art bible documented; final fidelity, infill/bespoke roles,
  broader art/performance/platform gates remain open. ADR052 records alternatives,
  tradeoffs and rollback. No new external repository/source/media copied. No
  commit/push/merge performed; Part05 not started.

## 2026-11-06 — User world detail, loading and unseen crash follow-up

- Improved fallback high-rises/party-wall infill with batched actual window relief,
  mullions/floor bands/roof trim and family masonry. Added original asphalt PBR
  aggregate/repair/wear/normal/roughness and CPU mipmaps with world-space road UVs.
  Suppress duplicate small-ornament shadows while retaining structural shadows.
- Resident coarse facades/masonry, spatial hysteresis and hidden scene preparation
  keep the old displayed facade until a new instance is ready. Cache, preparation
  and displayed geometry remain bounded. Existing structural IDs/colliders intact.
- Reproduced IO worker stack overflow with expanded loading; configured16MiB stacks
  only for asset IO workers, preserving default1–4 thread assignment. Default Valley
  and explicit City Run crash/recovery/finish smoke pass without RUST_MIN_STACK.
- User-requested unseen rear NPC crash path creates no ragdoll/wreck, disables the
  bike for300 active ticks, then restores lane0 at the recorded impact station.
  Pause freezes the timer, restart cancels it. Player/nearby/visible and already
  physical recovery remains unchanged; no mass/joint/motor retune.
- Mandatory checks:237 tests pass/1existingignored, strict clippy/fmt/build; native
  route and performance evidence alongside. New KIT-04/05 and PERF-06 verified;
  ANIM-12 resolved locally.174checked/222open/396IDs. Existing broad art/LOD/memory/
  performance/platform gates remain open. ADR053 records design and rollback.
- User authorizes commit/push/merge followed by a new Part05 branch. Integration
  references are reported after actual success; no new external source/media or
  dependencies. Existing research/provenance and original MIT asset registers updated.

## 2026-11-07 — Part05 dressing/materials and explicit contact/sky follow-ups

PR16 merged to main2a3ea003d59921dc3817251b69c4b894b9e6a17e. Started focused
codex/part-05-world-dressing from that merge. Added saved-entry soft-actor/wreck
contact response, runner knockdown without stopping the attacker, energy-paid
small ride-over, ready-scene temporal dither, typed semantic Valley prop dressing,
shared original PBR microdetail/tangents/concrete and original cloud/daylight sky.
Added COLLISION-07, LOD-01, DRESS-01, MAT-01, SKY-01 and open MAT-02; broad prior
production gates remain open. No new dependency/media or ragdoll articulation
change. ADR054/055 and testing/part05-world contain rationale and actual receipts.

## 2026-11-07 — Roadside composition variety

Expanded Part05 dressing from15 to35 original-catalog roles and20 authored groups.
Added near-road farm verges, service/loading clusters, civic/post/transit furniture
and hill pullouts. Seeded independent sides/model/spacing/heading/size replace the
mirrored cycle. Weighted per-zone shares preserve whole-route coverage within224
static owners. Existing clearance/support/solid collision rules retained; no trees,
new dependencies/media, rider/recovery or structural map changes. ADR056 and
[test evidence](testing/roadside-variety/README.md) record acceptance and limits.

## 2026-11-07 — Publish world changes; crash behavior research only

Committed the verified Part05/world/contacts/variety tree locally as3d3194f,
published identical tree b5c7c0a70a6ab5aed923f83d9c8563372250c27a through the connected
GitHub API at d74d82799056d0e7b28fab60ab456d2c826331c1, and merged PR17 to main
6aa3f17383c7773ebb81e09a8706128f873bfc08. Original local commit remains retained.
Started codex/crash-behavior-research from merged main. User then explicitly asked
for findings before implementation. No gameplay/physics/assets/dependencies changed.
RAG-11 verifies source-level investigation of current ejection/support/muscle/roll
behavior and five pinned repositories. Expanded the existing unfinished combat
sequence and active-recovery tasks; no implementation gate closed. Source hashes,
licenses, compatibility, recommendations and unverified runtime claims are in
research/crash-behavior. Research notes remain separate from the merged world PR.

## 2026-11-07 — Method B crash comparison implementation

Created codex/crash-velocity-muscles worktree from the common research baseline;
cherry-picked shared swerve/continuous-release/pair-grace/comparison controls and
pending-KO fixes from Method A. Added original full-tensor reciprocal angular
impulses, work/rate/torque caps, actual core ground-normal support, directional
bracing and finite supine recovery attempts, with no authored linear velocity or
body position. Native passive rig/constraints and baseline remain. No dependency
or media import. New RAG-B-01..03 retain native/user acceptance gates. ADR058-B and
docs/testing/crash-comparison/method-b record tuning/provenance/actual checks.

Final B verification:257workspace tests passed/1preexisting ignored, strict
Clippy/locked build/fmt/audit/5roadmap tests passed. Paired18-case lab finite;
nine baseline rows exactly equal excluding timing. Native passive-rest support
recognizes genuinely quiet distal ground support; active corrections expire
after4.95s to preserve240-body pileup sleep. Worst fully-prone8m/s attempt
rotated torso onlyabout7degrees; full roll-to-back and user acceptance stay open.

## 2026-11-07 — Select velocity and correct facade transitions

User chose Method B; based focused codex/velocity-popin on its clean candidate.
Retain passive anatomical joints, baseline, shared swerve/release/recovery and
unchanged map; no native-motor candidate integration. Correct facade handoff,
dither coverage, camera timing, readiness, prefetch and finite overlap/cache.
RAG-12/LOD-02 track verification; ADR060 records tradeoffs. No new media/dependency.

Final verification:258tests pass/1ignored; fmt/strictClippy/build/audit/5roadmap
pass; native recovery smoke pass;18finitepairedlab withbaselinepreserved. World
survey15sections657roots,9directcrossfades, boundsvalid; p95=16.94174ms misses
16.7ms (surveyexit1). RAG-12/LOD-02 completed; LOD-03 remains open.

## 2026-11-07 — Complete technical facade LOD follow-up

User chose facade LOD before Part05. Added128design/383band model-derived PBR
proxy atlas with original hashes/mips/guards, face-aware coverage and closest-face
distance. City Run/preview now prepare then crossfade ready scenes under48cache/
8transition limits. Source physics, world quotas, saves and selected velocity
remain untouched.263tests pass/1ignored; formatting/strictClippy/build and both
native moving-camera fixtures pass; city recovery and preview pass. World geometry
assertions pass but60FPS remains open. ADR061/LOD-04 capture implementation;
LOD-03 retains user visual/long-streaming signoff. Full receipts in facade-lod.

Final isolated facade LOD survey: p95=16.80066 ms against16.7 ms, so performance gate remains open (exit1); geometry/source/coverage assertions and all15sections pass. Final native Valley sweep54direct crossfades, City389; both recovery smokes and preview pass. Longer streaming/user approval remain open.

## 2026-11-07 — World repair planning and targeted regressions

Started `codex/world-repair-plan` from 894ed7a. User confirmed the 3D Come Bye art
reference. Generated street/property/current-height plans for 657 measured parcels,
with proposed owner groups/fences/access edges before city rearrangement. Corrected
scaled near/far backup-core occlusion, normal-speed static-wall crash entry,
visual pickup/running knee planes, out-of-reach kick-facing direction and faster
attack playback with separate kick sound/confirmed-hit feedback. Adapted roster
pace, doubled route traffic with downed-rider yielding/horns, narrowed rural roads
and allowed validated 0.8 m verges. Existing car-crash regressions remain covered.
Complete city texture/terrain/sky/NPC skin rebuilding and final visual/performance
acceptance remain open. ADR062 and testing/world-repair/README.md own evidence.

## 2026-11-07 — Dense city, layout only

User rejected the sparse plan and then explicitly limited work to layout, with
no 3D changes. Created `codex/dense-city-layout` from 149e355. Added a 2D proposal
with touching street walls, 12 m facade-to-facade width, rear courts/service wings,
parallel streets and two block bands per side: 752 concept parcels, 376 primary,
92.53 percent station frontage including cross-street breaks. LAYOUT-01 remains
open for user acceptance and later construction validation. No game source,
content, models, textures, rendering or physics changed. ADR063 and the planning
README record evidence and limitations.

## 2026-11-07 — City center with two split/rejoin sections

User rejected the road-shaped city and confirmed two choice sections. Created
`codex/city-center-layout` from 58eeabb. Produced a 2D city/route proposal around a
central square/city hall, market, neighborhoods and 108 blocks. Market/Station
branches rejoin downtown; after 300 m of shared avenue, Old Town/Civic branches
rejoin before exit. Four legal route combinations, equal nominal 2.464 km lengths,
2,992 clearance-filtered concept plots. Graph endpoints, DAG/paths, invalid-case
rejection, deterministic exports and layout-only scope verified. No game or 3D
changes. LAYOUT-02/ROUTE-SPLIT-01 remain open; ADR064 and planning/city-center own
acceptance, implementation and construction limits.

## 2026-11-07 — Full farm-to-long-city redraw and straight-footprint correction

User replaced the two-choice scheme with one early-city split, rejoining around
one quarter into a city that continues to the end. Draft ~12 km spine includes the
existing 3.15 km farm/transition approach and 8.85 km city; no hills district. Added the
entire-route map, split close-up, elevation profile/contours, fields, yards, parks,
city properties and two-path graph. Corrected an initial erroneous grid warp after
the user rejected curved building shapes: all retained plots now have straight
rectangular footprints. Tests reject bent plots and other brief violations.
LAYOUT-03, WORLD-EXTEND-01 and updated ROUTE-SPLIT-01 retain acceptance/future
integration gates. ADR065 and planning/farms-long-city own the proposal. No runtime
or 3D content changed; no push/merge claimed.

## 2026-11-07 — Street-first city layout correction (ADR066)

- User clarified the NYC/Flatiron approach after rejecting imposed rectangular
  blocks and dogleg junctions. Replaced those rules with road/sidewalk subtraction,
  actual polygon blocks, fitted frontage parcels and whole angled corner buildings.
- Regular grid pitch180×360m, farm approach, long city to finish, one first-quarter
  split/rejoin and elevation study retained. All outputs are offline planning.
- Added pinned Shapely authoring geometry helper/requirements and inspected its
  actual node/Voronoi APIs; recorded licenses and NYC primary reference. No copied
  upstream source/media, Cargo changes, runtime assets or physics edits.
- LAYOUT-03 remains open for user/layout/construction acceptance. No new task IDs;
  existing task requirements were corrected explicitly rather than falsely closed.
- Local branch codex/street-first-city. See planning/farms-long-city/README.md,
  VALIDATION.json and CHECKS.txt for actual verification/results and limits.

## 2026-11-07 — City height/detail plan before3D (ADR067 / LAYOUT-04)

- Added seeded downtown-to-outskirts height zoning, taller frontage accents,
  scattered interior towers, per-building floor/ground/roof data and road-distance
  authored detail tiers. Skyline towers retain at least skyline detail.
- Interior tower pads use unbuilt courts; explicit6m access easements cut clear
  openings through frontage and record affected proposed parcels/site assembly.
- Exported whole-city height/detail plans and downtown tower/access PNG/SVG views.
  No game source, runtime assets, models, physics, Cargo or playable map changes.
- Added LAYOUT-04 with reason/dependencies/acceptance/verification and synchronized
  README/progress/placement/counts. Gate stays open for review and later3D/LOD work.
- Existing licensed offline geometry stack retained; no new external repository,
  library or media adopted. Actual tests/validation in planning/farms-long-city/
  CHECKS.txt, VALIDATION.json and MASSING.md. Local codex/city-height-detail-plan.

## 2026-11-07 — Race-centered primary avenue plan (ADR068 / LAYOUT-05)

- Applied the user's supplied NYC-inspired reference to the2D plan: city rows are
  centered around the actual race paths, including the two alternatives, instead
  of a fixed world-origin footprint. Race/farm alignment and graph stay fixed.
- Primary avenue junctions take priority; omit complete competing parallel,
  fork-throat and redundant-close secondary segments without forced doglegs or
  warped buildings. Regenerate street-defined blocks/parcels and seeded massing.
- Added centering/junction/connectivity audits, four additional negative cases,
  explicit primary-junction data and whole-map/junction-detail exports.
- Concave merged blocks exposed a frontage clipping artifact: use native fixed-
  precision overlay and retain footprint inner rings throughout authoring/audits.
  No runtime geometry, engine, physics, assets, controls or saves were modified.
- New LAYOUT-05 remains open for review/later construction and traffic/native
  gates. README/progress/placement/counts synchronized; no historical tasks removed.
  Actual checks in planning/farms-long-city/CHECKS.txt and VALIDATION.json.

- LAYOUT-05 user screenshot follow-up: widened parallel-road exclusion to a local
 110m/48m-run/40degree rule, sampled every8m. The initial35m/60percent rule missed
 narrow strips and is superseded. Added four regression fixtures and an audit
 of all remaining secondary roads. Whole segments are omitted; cross streets
 and both legal race alternatives stay connected.

## 2026-11-07 — Approved Metropolis runtime construction

- Branch codex/build-approved-metropolis from64f3c82. Bake exact approved layout
  into separate12km course with19,068 closed buildings, both legal alternatives,
  graded roads/terrain/courts,42 fields, authored scenery and30 crossing cars.
- Branch-aware gameplay adapters preserve accepted physics/art/save formats;
  legacy Valley/City remain. New rr_core route rules and rr_content validation.
- Original facade atlas consolidated11,020→2,991 batches; full geometry/collision
  persists across camera moves. Actual native captures and limits recorded in
  testing/metropolis. Dev-build p9520.38ms misses target; broader gates stay open.
- New scoped METRO-01; source approval supersedes earlier layout-only restriction.
  No proprietary media/code imported and no dependency/physics migration.

## 2026-11-07 — Wider Metropolis sidewalks and curb furniture

- codex/metropolis-sidewalks: typical sidewalks2.4→3.4m by shifting city curbs1m
  inward; farm pavement, all19,068 buildings and both exact routes remain intact.
- Bake730 deterministic existing-model furniture owners, including205 lamps,
 70 benches and50 trees, with clear walking strips and priority-junction setbacks.
  Replace the earlier regular lamp/tree placement; add original slab paving.
- Authoring fixtures, strict content validation, preservation/native evidence and
  acceptance tracked as SIDEWALK-01; ADR070 records width/placement tradeoffs.
  No new dependency, proprietary media or accepted ragdoll/force tuning changes.

## 2026-11-07 — Original authored3D facade kit restored on Metropolis

- codex/metropolis-authored-facades: actual earlier ground/upper/crown GLB primitive
  handles repeated along accepted polygon edges and storeys. Twelve existing mapped
  families, resident prepared prototypes, finer street rows/coarser upper geometry,
  bounded smooth additive detail; retained solid buildings/sidewalks/furniture.
- Native review caught repeated RADIO signage; keep it once per frontage and use
  the matching original white-brick window module above. Rebuild the Metropolis
  coarse upper tile with explicit source identity; do not modify source models.
- AUTHORED-01 records scoped integration. New future PALETTE-01 records broader
  district/catalog adoption; no claim of all128 designs or completed art/performance.
  ADR071/source/mesh/cap/visual/recovery receipts accompany this change.

## 2026-11-07 — Lane-aware main streets and varied skyline

- codex/metropolis-street-life:96 initialized main-street cars, American direction
  centres,2/3/4 marked lane sections and100m tapers, following/yielding and fleet
  reset. Keep street widths/sidewalks/lot footprints and both exact race routes.
- Pavement audit caught full-path simplification disagreement; use original-edge
  street centres shared after merge.3,480 full-body samples now fit real pavement.
-995 original street furniture owners,65 checked parked cars and recovery obstacles;
  strong per-building2–32-floor height mix with existing interior towers retained.
- New STREET-LIFE-01, core lane/following rules, strict content config, authoring/
  footprint audits and native evidence. ADR072 records scope/arcade-width tradeoffs.
  No new dependency/source media or accepted ragdoll/control tuning changes.

## 2026-11-07 — Persistent Metropolis prop destruction

- User clarified destruction to props only, no cars. New focused branch
  codex/persistent-world-destruction from2cf3892; DESTRUCTION-01/ADR073.
- Register995 original sidewalk models/physical owners across13 types with validated
  per-role strength/mass and persistent collision/direct-melee durability.
- Keep original models solid while crushing/toppling; pin broken LOD, synthesize
  break audio, bound live prop debris to48, settle supported low-energy bodies and
  retain static ruins instead of timed disappearance. Navigation follows wreckage.
- Explicit start/retry restores health, model scale, collider, fixed-body/backend
  pose, LOD eligibility and navigation. Native testing caught and fixed backend
  writeback overwriting component-only restoration.
- Added domain/content/contact/melee/budget/settling/navigation/reset tests and a
  real-model native destruction fixture. Evidence: testing/prop-destruction/README.md.
- Added DESTRUCTION-02 for dedicated broken meshes/art; broad prop/category art,
  worst-case performance, legacy-map retrofit and platform gates remain open.
  No imported media/dependencies, car damage, building/road damage or ragdoll retune.

## 2026-11-07 — Racer/killer AI split

- AI-10/ADR074: seven racing/overtaking rivals and seven aggressive hunters,
  exact landed-hit attacker memory, identity-filtered auto-kicks/hit resolution,
  shorter killer cooldowns and bounded pursuit. Restore clean memory on restart.
- Preserve authored roster stats, eight-second launch grace, traffic avoidance,
  strike geometry/stamina, accepted riding/ragdoll/crash/recovery and prop damage.
- Add R/K standings labels, deterministic domain/combat regressions and native
  roster census. AI-11 keeps full-race user balance feedback open. Evidence:
  testing/racer-killer-ai/README.md. No dependency/media import or engine migration.

## 2026-11-07 — Opening-world repairs and FPS logging

- REPAIR-01/ADR075: slight outward seated knee IK with retained pegs, paired fence
  posts with endpoint deduplication,10m farm pavement/handling and paved shoulders,
 64 main cars seeded700m ahead, gradual city-edge height caps from source layout.
- Share finite vehicle closing-speed gates, preserve solidity/high-speed ejection,
  orient horizontal normals and re-evaluate ongoing vehicle contacts. Correct old
  scrape/placement test assumptions rather than restore touch-triggered crashes.
- PERF-07: bounded asynchronous1Hz rotating local JSONL, all fixed-step costs,
  main/render extraction/render CPU-wait spans, scene counts and memory/paging;
  summary CLI and printed log path. PERF-08 retains actual bottleneck optimization.
- Existing anatomy/physics, racer/killer roles, city branches/parcels/IDs and legacy
  Valley content stay integrated. Evidence: testing/startup-world-diagnostics.

## 2026-11-07 — Default parallel physics

- PERF-09/ADR076: enable existing Rapier parallel feature via default project
  parallel_physics, retain explicit serial build. Keep engine versions, SIMD,
  fixed/substeps, masses/anatomy/muscles, constraints, crash/recovery and content.
- Lock four newly activated Rayon/Crossbeam packages; inspect actual package/VCS
  metadata, API source and dual licenses, archive notices and record repository
  decisions. No new media or copied implementation.
- Record compiled backend/CPU availability/requested thread environment in local
  logs. Preserve serial executable for identical paired lab/native comparisons;
  benchmark gains and limits belong to testing/parallel-physics, not assumptions
  from enabled features. PERF-08 retains overall slow-race/60FPS optimization.

## 2026-11-08 — Pickup location, reverse and Animation A audit

- RECOVERY-01/ADR077: transfer the live wreck pose/support into the riding
  chassis and synchronize backend current/next state and interpolation history.
  Preserve accepted ragdoll construction and collision rules.
- REVERSE-01: S/Down brakes forward then requests low-speed reverse; throttle
  returns forward. Both chassis adapters use one pure parking rule.
- ANIM-14: resume Part02 with a hash-bound context-to-runtime audit:41 sequences,
  138 draft poses,13 clips,104 unplayed context poses and two authored lean keys.
  No new pose approval or native timing is inferred. Broad gates stay open.
- Evidence and limitations: testing/remount-reverse/README.md and
  animation/NEXT_REVIEW.md. No new dependencies, media or external source reuse.

## 2026-11-08 — Animation A glove/contact refinement

- ANIM-15/ADR078: implement shared visual glove morphs, bounded finger-edge
  refinement, independent left/right grip/fist blending, relaxed hands and typed
  grip/weapon sockets on existing visual wrists. Preserve the physical rider.
- Add hand metadata validation and source-to-universal-grip checks, morphology,
  GPU data/geometry budgets and animation blend regressions. Keep original model
  bundle, GLBs and accepted ragdoll/control/recovery assets unchanged.
- Add exact-key native hand close-up review and numbered screenshots; ordinary
  animation review remains dynamic. Update Part02's partial hand acceptance and
  track ANIM-16 authoring round-trip, without closing art/user/production gates.

## 2026-11-08 — Part01 runtime development pass

- Audited all29 remaining Part01 requirements against actual code/CI/research;
  retained their unchecked acceptance gates and corrected stale proposed-only
  combat wording and missing-local-calibration claim.
- Added pure measured vehicle approach classification plus bounded adapter contact
  observations (tick/entities/world velocities/normal/closing/height/outcome).
- Existing combat stagger now releases on1m/s tall-solid/vehicle normal contact;
  ordinary riding and passive baseline thresholds/physics remain unchanged.
- Added both-side/phase interruption/no-launch regressions,100/60 directional and
  solid-height observations, and real solid-car height trajectory comparisons.
  ADR079 and testing/part01-runtime/README.md record exact receipts/limitations.
- No dependencies/assets/source imports, server/network migration, new task IDs,
  task moves, push or merge; Part02 is not started by this development pass.

## 2026-11-08 — Part02 shared-profile hand authoring development pass

- Replaced static Blender compression with runtime-profile grip/fist hinges,
  source-scale conversion, independent hand states and typed visual sockets.
- Corrected a measured10.467mm raw-landmark/source socket mismatch by fitting
  wrists from the calibrated socket with the unchanged fixed-length solver;
  declared held contacts reject errors over5mm.
- Attached original club/chain proxies to weapon sockets and authored handle axes;
  retained source phase/mirror/view/counter semantics and138human/16bike IDs.
- Rebuilt the complete numbered context gallery and editable one-frame Actions;
  added18source hand views and actual GLB target/skin/UV/socket export/reload checks.
- Authoring-state/contact validation belongs to rr_content; Python checks recorded
  artifacts and the actual pose loader. Rust compares Blender numeric fold/normal
  fixtures against the runtime recipe without modifying runtime behavior.
- ADR080 and testing/part02-authoring/README.md retain normal/tessellation parity,
  angular thumb/finger silhouettes, clearance, specialist/support and user approval
  gates. All15Part02 requirements remain unchecked; no new IDs or Part03 advance.
- No physical ragdoll, gameplay timing, controls, saves, model source GLBs, accepted
  velocity-muscle settings, map defaults, dependency migration, push or merge.

## 2026-11-08 — Part03 fixed contact timing and visual support lifecycle

- Separated input/AI attack intent from already-resolved contact; authored
  extension IDs/duration sums drive a single fixed clock shared by melee and
  visuals. Retained cooldown/stamina, three-direct-hit/3s, exact-attacker AI,
  range/LOS, kick push and existing knockoff outcomes.
- Froze kick leg selection at windup across eligible rivals/NPCs; duplicate
  windup intent cannot restart the clock. Pause freezes, interruption/recovery/
  reset/finish cancel. Added real buffered-input multi-rate contact regressions
  alongside the deliberately resolved-contact fixtures.
- Repaired never-zero visual plant release, old-stride reuse and world-fixed
  support locks; physics-local contacts follow kinematic movement and reject
  support loss. No authoritative physical transforms or rig settings changed.
- ADR081 and testing/part03-runtime retain quantitative synthetic/native receipts,
  all27 still-open Part03 requirements and separate held-pose/timing limits.
  Recaptured all33 native hand views with the current release and refreshed
  their actual source/image provenance after detecting stale source hashes.
  No task moves/new IDs, source assets, dependency changes, push, merge or Part04.

## 2026-11-08 — Part04 bounded district facade pass

- PALETTE-01 expands Metropolis12→24 original existing facade families, assigning
  district frontage pools and office-only tall/interior towers deterministically.
- Share full/coarse bay/storey fitting; derive matching crown/ground/upper tiles
  from original model atlas and retain BUSS4 once-only upper-sign exception.
- Preserve approved whole-map geometry, routes/heights, collision, Valley/City
  originals and accepted physical/gameplay systems. No dependencies/imports.
- Record actual placed source-mesh/triangle and residency evidence, native paired
  distance views, structural equality and source-band/mip checks in
  testing/part04-world/. All29 Part04 requirements stay unchecked; no newtaskIDs.

## 2026-11-08 — Part05 bounded furniture damage pass

- Added twelve deterministic offline GLB derivatives of original furniture: wall
  folds, dented panels, tilted lids and sagged roof/foliage. Preserved embedded
  material/UV/image identity, recomputed geometry/normals/bounds, and independently
  decoded finite triangle/collider-envelope checks. No external source/media.
- Prepared hidden damage scenes with hierarchy/material readiness and explicitly
  ordered distance/damage visibility; preserve fallback and immediate reset.
- Fixed Part03 contact-timing regression in prop melee by sharing resolved
  StrikeContact/frozen kick side; retained already-resolved test fixtures.
- Extended loading/reset/distance, keyboard windup/pause/cancel/single-contact
  and opposite-side kick tests plus native per-role capture and resident census.
  Detailed execution evidence and unmet gates: testing/part05-props/README.md.
  All ten Part05 boxes remain open; no Part06 work or pushed/merged commit.

## 2026-11-08 — Part06 bounded civilian skin continuity pass

- Replace rigid semantic-body weighting with a shared source-space four-bone field,
  continuous central pelvis ownership and normalized female-source sampling.
- Retain original neutral geometry/materials/UVs; one conforming shared-edge split
  per blended primitive samples the field without changing the neutral surface.
  Preserve8 cow GLBs byte-for-byte and all human/quadruped physical definitions.
- Add source/triangle/UV/budget checks, focused authoring regressions, complete
  synthetic paired stress measurements, real native driver receipts and family
  front/rear/side/impact views. Record individual regressions and increased cost.
- All11 Part06 requirements remain open. No new requirements, landscape work,
  dependencies, external repositories/media, push or merge. ADR084/evidence:
  testing/part06-npcs/README.md.

## 2026-11-08 — Part07 first held club development pass

- Independently authored first club/rules/ability content, buffered keyboard
  pickup/drop and bounded static pickups; shared rival timing/retaliation and
  captured fixed-contact reach for racers/NPCs/props. No physical rig/energy change.
- Appended six original authored keys/two clips with old calibrated data deeply
  preserved; original socket attachment/grip and selected-arm riding layering.
  Native recaptures and verification outcomes are in testing/part07-weapons/.
- R01 pinned reference-only investigation recorded; root license unknown, no
  imported source/media/dependency. No Part08, push, merge or broad gate closure.

- Final first-club verification:349Rust tests passed/1existing soak ignored;
  mandatory fmt/clippy/debug/release and source/asset/roadmap checks passed.
  All18paired lab cases finite; actual keyboard native club and Metro recovery/
  results passed; current-release hand/body recaptures packaged. Rolling wall
  frame19.811ms/p9522.633ms still misses16.7ms; no paired cost claim.
- P05-003 alone moved to Part00:206verified/242open of448stable requirements.
  Ten Part07 tasks remain open; raw grip/clearance/fidelity limitations retained.


## 2026-11-08 — Part08 resident police development pass (ADR086)

On codex/part08-police-loop from5dcc791, add one non-racing shared motorcycle
patrol, pure police states, validated original authored tuning, confirmed witnessed
violations,10Hz LOS/loss/search decisions and conditional60Hz safe arrest contact.
The officer shares handling/contact/three-hit physical recovery and can deliberately
park into the existing solid runner/navigation path. Busted has no finish/profile
award, and retry restores police identity/chassis. Existing15 racers, club input,
worlds/props, rigs, saves and controls remain integrated. Root R01 permission stays
UNVERIFIED; source inspection is reference-only and no media/code is imported.
P08-006 is verified in Part00 after state/integration/native evidence; the other9 Part08 requirements retain remaining broad art/human/performance/exit gates;
no Part09 or network work. Exact tests, failures/corrections and native receipts:
testing/part08-police/README.md. No new task IDs or automatic push/merge.

## 2026-11-08 — Rider schemes and wide race setup (ADR087)

On codex/rider-colors-wide-grid from432ab33, adjust original swatches/neutral
leather after local fifteen-scheme reference review, disable player nitrous
through authored handling configuration and both adapter guards, and update HUD.
Metropolis has four3m lanes before/after its fork, three on each branch,20m
successive grid gaps, player last at−288m and340m painted asphalt apron. Fix
spawn-width/depth validation and cached rear projection extent. Restore primary
road within the existing reservation, keep995props/13types, move larger furniture
and30parked cars to secondary streets. Paths/elevations/gates/all19,068buildings
are unchanged. RACE-SETUP-01 alone is verified intoPart00;208complete/241open,
449IDs. Native/test receipts and exact limitations: testing/race-setup/README.md.
Human art/race balance, performance/hardware gates remain open. No reference code
or media imported, no physical-rig change, no Part09/push/merge.

## 2026-11-08 — UI reference library and artist desk (ADR088)

On codex/ui-reference-kit from1a96667, independently catalog the user-local UI
images/PEicons, frontend table/popups, native dialog templates, font roles and
video clips after pinned R01/primary Win32 format research.142 UI records include
137 unique source hashes;12world-context images are separate.29 populated screen
entries/36slots,13 popups and7 unique native dialogs repeated in2EXEs are covered.
Deliver63editable geometry SVGs,28proposed RoadDash pages,66master family briefs,
142image/15component unchecked art tasks, dimensions/source/replacement CSVs,
external-only original reference gallery, video thumbnails and26contact sheets.
Preview own replacement art and import/export notes/statuses; refresh edited
plans without the game installation. Fix note saving to input events so export
retains the last edit. No code/media reused from R01; no raw media/font/EXE/AVI
bundles in Git/runtime. Original references remain in the user's separate folder.
UI-01 alone is checked; UI-02 added open;209complete/242open/451IDs.
17 Python tests, full Rust regressions and actual browser verification are recorded
in testing/ui-kit/README.md. No gameplay/UI/physics migration, push or merge.

## 2026-11-08 — Approved portrait UI pass

Added the exact ten selected portraits and six approved remakes to a sixteen-card native Biker Bar browser, with a 4×4 picker, live names/bios, individual detail views and restored grid focus. Packaged bounded UI derivatives with source hashes and provenance; retained all original approvals. Existing UI-02/UI-05 remain partial; game roster replacement, playable profiles, conversations, final typography and human/hardware review remain open. Verification receipts: docs/testing/approved-portrait-ui/README.md.

## 2026-11-08 — Full-screen rider dossiers

Retained the sixteen approved portraits and 4×4 selection. Character clicks now open a 4:3 full-page painting with a bottom biography strip and compact Back control. Restored detailed personality/food/work/racing descriptions as paraphrases under the user's later non-racist direction. Compact portrait-first grid and licensed Open Sans typography bring menus closer to the original composition language. No generated replacement art, race roster/profile or physics changes. UI-02/UI-05 remain partial. Evidence: docs/testing/portrait-dossier/README.md; decision ADR095.

## 2026-11-09 — Fresh-agent traffic, weapon-model and walking repairs

Three brand-new agents started with no inherited conversation context in isolated branches/worktrees. Integrated predictive main/cross-traffic and queue-safe AI intent, independently authored chain/whip/rolling-pin/bat source/LOD assets with native preview/equipped proof, and actual-road paint plus authoritative-grounded foot adhesion. Root reviews caught hazard-free police-goal regression, unoccupied-pass assumptions, headless renderer requirements and an invalid native building-interior test setup; final scoped fixes preserve player handling, physical crash consequences and accepted rigs.

New technical tasks AI-12, ROAD-WALK-01 and WEAPON-MODELS-01 track verification; FOOT-SPAWN-01 records the deferred inspected get-up-clearance issue. Root combined validation/evidence: testing/traffic-weapons-road-fixes/README.md. Human art/race/performance/platform and broader weapon-family gates remain open. No external source/assets/crates adopted and no next major part started.

## 2026-11-09 — Fresh Part07/08 agents and downtown street life

User requested two brand-new fresh-context subagents and downtown trees/parked cars/walking pedestrians. Created isolated codex/part07-arsenal-fresh and codex/part08-police-fresh worktrees from d5db26f; integrated on codex/combat-police-downtown. Original five-family gameplay/inventory/model skins and bounded police last-seen search/readable feedback accompany deterministic all-city curb/tree/patrol authoring and local side-road parking bays. Shared accepted rig/physical tuning and approved4/3/4 race geometry preserved. Final verification and open acceptance details live in testing/combat-police-downtown/README.md; no next Part09, push or merge is implied.

Final integration:407 Rust tests passed,1existing soak ignored; strict Clippy/fmt/locked build/audit and authoring/model/roadmap checks passed. Native14-view survey and two weapon/recovery route/rig modes passed; police native exposed Results swallowing R, now fixed and verified through real Starting/Countdown with no payout. DOWNTOWN-01/UI-RETRY-01 checked narrowly;217verified/245open across462IDs. Native city wall p95target missed; no hardware/performance or broad Part07/08 exit closure. Source/binary/failed-attempt evidence retained.

## 2026-11-09 — Original-format native frontend

User requested original game UI formatting. codex/original-ui-format replaces wide floating cards with4:3 letterboxing, full-width stored-proportion main/street choice bands, compact art-led copy and gold focus; retains approved paintings,4×4 portraits and real controls/results retry. Adopted unmodified OFL1.1 Jost Book/Bold from pinned upstream commit; no retail font/media import. Verification and exact limits: testing/original-ui-format/README.md. Broad UI-02/UI-05 gates remain open.

## 2026-11-09 — Modern menu presentation after user rejection

User rejected the literal original-format pass as ugly/dated. codex/modern-ui-polish replaces hard red/shadowed headings, bars, stretched aspect assumptions and yellow rail with full-window proportional art, warm-white hierarchy,right-side main choices,native soft gradients and rust-orange accents. Preserves approved artwork bytes,portrait4×4/dossiers,menus/guarded input/retry/profile and gameplay. Exact source-table layout retained as historical evidence, explicitly superseded by ADR103; no new major part started. Verification/limitations: testing/modern-ui-polish/README.md.

Final ADR103 verification:408Rust tests passed,1existing soak ignored; fmt/strict Clippy/locked build/audit and final native22-stage/all16portrait/five-viewport flow passed. Proportional full-window layout verified against actual reported window sizes. UI-POLISH-01 narrowly checked;219verified/245open across464IDs; broader user acceptance remains open.

## 2026-11-09 — Functional grouped settings editor

User requested settings setup after menu polish. codex/settings-menu replaces the flat comfort/binding list with Camera/Controls/Audio/Display tabs,keyboard/mouse/controller navigation,optional pickup/drop bindings,live absolute master gain/mute,borderless fullscreen with windowed-size restoration,and VSync. Optional preference fields load old files without rewriting; invalid/future files remain preserved. Save errors remain visible; explicit session-only close is available. Settings pause/resume/suppress input and stop hidden3D rendering; accepted rigs/gameplay unchanged. New native opt-in settings traversal uses real keyboard category/adjust/save and temp-file reload; scoped results/evidence in testing/settings-menu/README.md. Broad quality/audio-bus/human/hardware gates remain open.

## 2026-11-09 — Sequential polish pass01

PERF-08: corrected facade detail selection hysteresis around tangent camera views and retiring groups. Resident collision/envelopes, source meshes/atlas and physical settings preserved. Paired native evidence and mandatory check receipts: testing/polish-12/pass01/README.md. No new IDs or task moves; broader performance/visual/hardware gates remain open.


## 2026-11-09 — Sequential polish pass02: low-speed contacts and recovery

Audited gentle car/wall contacts, rear momentum, downed body/wreck lift, parking
reverse and live pickup. Reproduced false lift from mixed pre/post-solver target
velocity; aligned eligibility with existing pre-solver energy input (ADR106).
Added actual impact-system regression with both owned-body kinds and opposing
solver outcomes. Existing thresholds, rig/handling, collision and recovery
parameters remain. No new IDs/assets/research repositories; V2-01-003 evidence
extended, checklist counts/placement unchanged. Verification and limitations
are recorded in testing/polish-12/pass02/README.md.

## 2026-11-09 — Sequential polish03: authored elevation and terrain support

Inspected existing farm/city sine-based grades before changing them. ADR107
replaces duplicated runtime/planner formulas with a serialized C2 knot profile,
shared across baked routes, terrain, props and navigation. Preserved approved
horizontal source geometry, IDs/rights of way, field boundaries, building shells
and legacy maps. Frozen progress prevents height edits reshuffling authoring.
Retiled long start apron and farm surfaces; fixed repair-before-clipping for
self-intersecting source fields after strict coverage tests exposed the issue.
Final pavement/field samples stay below10mm; both routes' actual marked lanes
and grid verified.419 tests plus mandatory checks pass. Native survey/cost and
paired recovery evidence in docs/testing/polish-12/pass03; no broad milestone
closure, physics tuning, external code/media or UI artwork. Current isolated
survey misses16.7ms p95; human/full-route/hardware gates remain open.

## 2026-11-09 — Sequential polish04: unified lighting and exposure

Inspected existing environment, sky, world/camera adapters and Bevy0.17.3's
installed analytic tonemapping implementation. No external repositories/media
imported. Added bounded colors/direction/operator authoring and one complete
camera/light installation; warmer lower sun and stronger cool fill balance
EV12.7 with explicit Reinhard. No facade/atlas, physics or dependency changes.
Existing SKY-01 evidence extended, no new task IDs/count/placement changes.
Mandatory checks, matched captures and remaining gates: testing/polish-12/pass04.

## 2026-11-09 — Sequential polish05 shadow contact (ADR109)

Investigated pinned Bevy default receiver bias/cascades, existing texel snapping,
coarse-facade/ornament exclusions and matched native farm/city/recovery captures.
Reduced normal bias1.8→1.5,retained depth bias0.02,4×2048 shadow maps,10m first
split/150m reach; widened cascade blend20→25%. Shared world installation assertions
protect bounded configuration and solid scenery. No lighting/exposure/material/
atlas/physics or content/dependency changes. P15-004 evidence extended without
new IDs, placement changes or broad closures. Source provenance updated; no
external assets/code imported. Native and mandatory receipts: polish-12/pass05.
Parent reviews/integrates; no commit/push/merge by implementation agent.


## 2026-11-09 — Sequential polish06: roads and paving

Inspected legacy UV/PBR and default Metropolis batches; found the default map
never received the asphalt installer. Connected shared original procedural
albedo/normal/roughness, measured world UVs/tangents and quieter repair patches.
Weathered existing paving seams and added opaque faded paint without new meshes,
overlays, collision or facade changes. ADR110 extends P11-004 evidence; no task
IDs moved/closed, no dependency or imported asset. Native/check results and
remaining temporal/art/drainage/hardware gates: testing/polish-12/pass06.


## 2026-11-09 — Polish07 building material consistency

ADR111: coordinate the24 placed facade families in the offline front compiler,
preserve original source models/textures and all derived geometry bytes, refresh
72front/72storey material headers, rebake383model-derived bands and their complete
mip chains/Metropolis crops. Correct far roughness to consume the same provenance
profiles and quantized micro-map as runtime. Metropolis consumes packed linear
roughness/metallic factors in one additional existing-material texture sample;
no additional meshes/draw groups, physical changes or media imports.
Shared actual-map fixture and source/geometry/hash verification accompany native
Metropolis, both LOD modes and recovery evidence in testing/polish-12/pass07.
MAT-03 remains partial for human/streaming/hardware acceptance. No new IDs, task
moves or closures; original dirty RoadRash/UI work untouched. No commit/push/merge.

## 2026-11-09 — Sequential polish08 / approach district use (ADR112)

Preserve the approved farm-to-city geometry and existing height ramp. Blend
low-rise business/workshop/neighborhood facade pools before1800m; fix capped
interior-tower office use only inside this scope. Taller and distant identities,
all roads/ROW/fields/fences/vegetation/closed collisions and simulation remain.
Original catalog reused with matching near/far style IDs; no external intake.
PALETTE-01/V2-04-003 evidence extended, no new IDs or closures. Verification and
remaining art/hardware/performance gates: testing/polish-12/pass08/README.md.

## 2026-11-09 — Sequential polish09 / atmospheric depth (ADR113)

Replaced fully opaque1050m linear fog with bounded exponential-squared haze;
shared unchanged horizon palette and actual sky exposure scale remove the
separate dark fog target. Retain non-HDR after an HDR trial rendered black;
record remaining sky/PBR tonemap mismatch for pass10 (V2-06-007). Preserve near traffic contrast, resident geometry,
collision, facades/atlas lifecycle, all physical rigs and gameplay. No external
media, dependencies or UI art changes. SKY-01 evidence extends without ID/state/
placement/count changes. Native paired views, cost and mandatory receipts:
testing/polish-12/pass09. Sky/cloud work remains pass10; broad gates stay open.


## 2026-11-09 — Sequential polish10 sky presentation

ADR114 separates displayed sky from unchanged lighting radiance, applies matching
analytic exposure/tonemap, adds original layered cloud shading and corrects mirrored
solar direction. Removes whole-sky/environment wind rotation; independent cloud
motion remains open. No geometry/physics/facade/rig or UI changes. Tests and native
paired captures/cost receipts: testing/polish-12/pass10. Parent reviews/integrates;
no commit/push/merge by implementation agent. No IDs, moves or gate closures.

## 2026-11-09 — Sequential polish11 character articulation

ADR115 replaces the render-only0.50m ankle pivot approximation with the existing
0.558046m authoring endpoint. Joint inverse binds update with it; original mesh,
weights, proportions, neutral wrists and physics remain unchanged. Negative
reproduction and contact/neutral-surface/model parity tests cover the correction;
native paired pose/recovery/NPC/lab receipts live in testing/polish-12/pass11.
ART-04 remains partial; no IDs, moves, count changes, next-part advance or UI art.
Full deformation/gait/civilian seams and art/hardware/performance gates stay open.
Parent reviews/integrates; implementation agent makes no commit/push/merge.

## 2026-11-09 — Sequential polish12: confirmed mounted impact reactions

Final of the requested12 fresh sequential implementation agents. Added one bounded
replaceable presentation pulse per confirmed mounted hit, with mirrored punch
head/shoulder recoil and kick trunk fold, fixed race-clock recovery, attack/knockoff
priority and no input lock. Existing grip/peg fitting, kick push, three-hit knockoff,
velocity muscles/11body control, physical authority, audio/camera contact timing,
controls/saves/race and prior eleven polish passes retained. Initial acceleration
grip reach regression was reproduced and corrected by changing visual punch pitch;
strict contact tests retained. ADR116 and testing/polish-12/pass12 record exact
receipts/provenance/limits. COMBAT-FEEL-01/P05-006 remain partial; no IDs/moves/count
changes, no art approval, no push/merge and no automatic next-major-part work.

## 2026-11-09 — Parent final twelve-pass integration verification

Exactly12 fresh-context agents completed sequentially with reviewed Conventional Commits on codex/gameplay-graphics-polish. Final439 workspace tests (one existing ignored), mandatory lint/build/audit, authoring/atlas/progress checks, ten native cases and18-case paired lab passed. Parent visual review rejected the first groundless farm survey capture; same-binary repeat rendered it correctly, and cold-start capture readiness remains PERF-08. Both runs retained; p9539.747/33.901ms miss16.7ms. README/PROGRESS remain220 verified/245 open with no newIDs/closures. Full files, decisions, provenance, images and limits: testing/polish-12/final/README.md. No push/default merge, no next-part work. Original checkout independently advanced to clean main2a3770e; unrelated work not absorbed.

## 2026-11-09 — User-authorized landscape follow-up after twelve-pass polish

Started codex/landscape-graphics from f4086433a98e0c92bed48846d922d7dbe2486842.
Implemented original deterministic farm grass, native dither distance handling,
coherent field tint and irregular soil shoulder geometry. Added actual visible
batch/triangle survey counters and opt-in shoulder camera. Field holes, paved
access, buildings, barns and fence margins constrain placement. No physics,
collision/layout/source content/dependencies/facade materials or legacy maps
changed. ADR117 and testing/landscape-graphics preserve references, rejected
incompatible plugin intake and exact check/native receipts. Existing
V2-06-005/006/007 and WORLDART-01 extended; no new IDs, closures or task moves.
User art, full-map/temporal/hardware/performance acceptance remain open. Parent
review/commit remains outstanding; no push or merge performed.

Landscape final verification:443tests (one ignored), mandatory fmt/clippy/locked
builds/audit/progress/roadmap checks passed. Final grass57,197tufts/333batches;
58overlapping soil candidates rejected by actual polygon intersection,6,606quads
retained with independent Shapely parity. Six native survey/recovery/LOD cases,
18-case paired lab and explicit inspections passed;121native captures, no blanks
or logged errors. Valley world-survey completed geometry but exited1 solely for
p9518.597ms; Metropolis p9534.765ms also misses16.7. Retain performance/user-art/
hardware gates; no newID/closure/commit/push/merge by implementation agent.

Parent landscape integration review: accepted final road/shoulder/gameplay evidence after correcting sparse tufts, shoulder winding/material mismatch and actual-polygon overlaps. Independently checked audit/progress/diff; committed on codex/landscape-graphics. Existing performance and user art gates remain open; no push/default merge.

## 2026-11-09 — Farm gravel verge, cattle clearance and original prop reuse

User follow-up requests a less abrupt road edge, correction of cows on fences,
and existing3D farmland props. ADR118 adds a broad gravel/mown verge, moves whole
cow footprints inside fields, and places16existing green barn/windmill/hay models
with fixed collision and access/boundary clearance. No physical rig or source
layout changes. README/PROGRESS retain220verified/245open and no new taskIDs.
Evidence and verification: testing/farm-verges; no push/default merge.

Farm follow-up verified:444tests (one existing ignored), mandatory lint/build/audit,
roadmap checks, five native road/shoulder/recovery/actor cases all passed. Independent
Shapely audit confirms32occupants, minimum4.533m boundary margin and no access
intersection. All16reused prop scenes ready; actual road/shoulder views reviewed.
Metropolis p9533.464ms still misses16.7ms; no performance/user approval claim.

## 2026-11-09 — Helmeted stylized humans and wind grass

User selected helmets with stylized clothing after supplying pastoral references.
ADR119 adds matte faceted clothing/human skins and original curved wind grass,
retaining physical rigs, authored surfaces, farm exclusions and existing content.
First native clothing trial exposed mixed jacket/leg source regions; canonical
height partition corrected them. First broad blades were narrowed after native
review. Rejected evidence is retained. Existing checklist IDs remain partial;
no new IDs, task moves or closures. Verification: testing/stylized-humans-grass.

ADR119 verified:445workspace tests (one existing ignored), strict fmt/clippy,
locked debug/release builds, asset audit and roadmap checks passed. Both model
physics trajectories remain exactly unchanged over180ticks. Seven serial native
runs/124captures and18paired lab cases passed; independent farm clearance audit
still passes. Native p9535.341ms misses16.7ms; resident grass payload estimate
115.46MB versus49.62MB parent. Original silhouette and NPC waist seams remain
open; no claim of full reference-look or user art acceptance. No push/main merge.

## 2026-11-09 — Portrait cast AI and living city (ADR120)

User requested major AI/cast/loadout changes, urban-only density, crossing-speed
repair, reverse recovery, approach infill, City Hall, fountain park, cafes,
sidewalk expansion and crosswalks. New four scoped IDs added with acceptance and
verification; README220verified/249open. Stable content IDs/save format retained.
Original procedural civic geometry and existing assets; no external intake.
Initial reverse fixture incorrectly inspected unsigned HUD speed; corrected to
measure signed physical velocity. Actual reverse and140m crossing-wrap physics
fixtures pass. Wider authoring exposed the old1500furniture cap at1519; explicit
new1600cap retains a bounded budget. Geometric clipping uses existing micrometre
quantization with10micrometre audit tolerance; meaningful overlaps remain errors.
Independent8572whole-car samples and20Metropolis authoring tests passed. Full
Rust/native results will be recorded in testing/living-city-ai; no next major
part or main merge/push.

- User correction during ADR120: the first civic-site proposal used small labelled
  public lots and was rejected. Explicit selection is park3 LC-S0152 for a grand
  City Hall, arches facing west into the city, and park5 LC-S0246 for the detailed
  fountain park. Replaced the proposal; preserved the other green blocks.

- Final ADR120 validation:450workspace tests passed (1existing ignored),20authoring
  tests,fmt,strict Clippy,debug/release builds,provenance and roadmap audits.
  Five final serial native runs passed with nonblank/hash-stable captures;
  both rigs/routes reversed at-1.898m/s and remounted with0pickup error; NPC
  contacts passed.18paired lab cases finite. Final city census96main/58crossing
  cars/192pedestrians/1521street props. Native bounded wall18.55ms average and
  23.24ms p95 leaves the16.7ms target open. All requested implementation items
  are enumerated in testing/living-city-ai/README.md; art/balance/hardware
  acceptance remains separate, no next major part or main merge/push.

- Final user correction supersedes west/city-centre frontage: point the arches
  at the actual driving street. Heading now uses the nearest race traffic path
  (alignment dot1.0); park5 turf covers its full irregular block minus pavement
  (60,964.44m²), with26trees,11beds,15benches,8lamps and pergola. Grounds are
  clipped and triangulated by the baker, validated by rr_content and checked
  independently for complete block coverage; paths/gardens extend into the
  formerly empty eastern portion. Superseded visual iterations retained.

- Final corrected frontage/full-block park build re-passed450workspace tests,
  20authoring tests,strict Clippy,fmt,debug/release builds and audits. Five fresh
  native runs all exited0 with stable frozen hashes/nonblank images; both
  recovery rigs/routes and actors passed;18paired lab cases finite. Final
  bounded wall19.91ms average/22.65ms p95 still exceeds16.7ms. Complete requested
  implementation checklist and remaining review gates recorded in the report.

## 2026-11-09 — Civic access and both AI escape paths (ADR121)

User rejected empty City Hall surroundings, requested nearer road placement and
sidewalk-connected park paths, and confirmed both mounted/walking AI traps.
Added31context buildings/ten trees and grass, moved hall24m, baked six connected
park entrances with a fountain ring. Added bounded pure grid planning with live
physics sweeps, mounted road-rejoin steering and stale walking-route replanning.
Physical mounted and walking concave-enclosure regressions pass without a rescue
or teleport. Existing four living-city IDs stay open; no new IDs or next part.
Final integrated evidence is recorded in testing/civic-access-ai.

Final ADR121 verification: 457 workspace tests passed (one existing ignored),
20 authoring tests, strict Clippy, fmt, locked debug/release builds, geometry,
provenance and roadmap audits. Five serial native runs passed with stable input
hashes and nonblank images; both rigs/routes remounted twice at zero pickup
error, NPC contact checks passed, and all 18 paired lab cases remained finite.
City Hall/park captures were visually inspected. Route survey wall average
17.32ms / p95 22.26ms leaves the 16.7ms gate open; art/balance and full-race escape
coverage also remain open. Original checkout unchanged; no push or merge.

## 2026-11-09 — Civic greenery and city-width proposal (ADR122)

Added denser hall/park trees and 5,346 civic grass tufts using existing batched
wind rendering and clear-access exclusions. Hall has 120 trees (previously 10),
all context buildings and both civic sites stay intact. User requested a map of
removing about half of non-frontage blocks: CITY-WIDTH-01 records a preview of
136 removed / 194 retained blocks and 7,833 fewer buildings; no live city deletion.
Generated full before/after map and character gallery (16 portraits, all ten
exported catalog sheets and native rider capture), explicitly separating art
from unique 3D models. No external asset import or dependency migration.
458 workspace tests, 20 authoring tests, fmt, strict Clippy, debug/release builds,
provenance/roadmap and whole-car geometry audits pass. Native survey p95 22.99ms
leaves the 16.7ms performance gate open. Final native evidence: testing/civic-greenery.
Five final serial native checks passed; both rigs/routes reversed and remounted
with zero pickup error, actor contact checks passed, frozen inputs stayed stable
and images were nonblank. No push/merge; original checkout unchanged.

## 2026-11-09 — User-authorized main integration

Merged codex/living-city-ai into main (repository has no master branch) and created
codex/city-next-pass. Resolved overlapping-history conflicts using verified source
while retaining main-only historical evidence/artwork. Source/dependency/runtime
asset comparison matches 2bbf731c exactly. Recorded run/model-view commands and
actual biker captures in testing/main-merge-2026-11-09. No remote push, history
rewrite or requirement closure. Narrow-city map remains a proposal only.
Merged checkout passed 458 tests, fmt, strict Clippy, locked debug/release builds,
provenance/roadmap audits and a fresh release native recovery smoke (both farm/city
remounts at zero pickup error). Source parity receipt preserves exact comparison.

## 2026-11-09 — Fresh-agent city and gameplay follow-up begins

User requests separate contextless GPT-6 Astra medium agents for ten focused
changes: town hall; original biker outfit restoration; lighting research;
500m race-end trims/no gates; city lane consistency; traffic queue overtaking;
texture fidelity; persistent crop deformation; moderately stronger elevation;
and police uniform/custom bike. Added three stable IDs and acceptance gates;
no task is complete merely from delegation. Work is isolated and reviewed before
integration on codex/city-next-pass. Map-width reduction remains preview-only.

## 2026-11-09 — Fresh-agent follow-up integrated and verified (ADR123–127)

Completed ten separate contextless GPT-6 Astra/medium worker tasks, reviewed and
integrated on codex/city-next-pass: detailed Hall stonework/PBR, original biker
outfit restore, researched prefiltered sky IBL,500m race trims/no visible gates,
entry carriageway alignment, safe queued-rider passing, persistent animated wheat
trails,25%longitudinal relief and dedicated officer uniform/patrol motorcycle.
All buildings/planar map/legacy maps and accepted physical rigs remain; no city
width deletion, dependency migration or new external media/code intake.

Parent clean-workspace run passed481tests (one existing600-second turning soak
ignored),fmt,strict all-target Clippy,locked debug/release builds,audit and progress
checks.23Metropolis authoring and5roadmap tests pass;30,120whole-car/17,828paint
samples have no failures. Corrected readiness timing in the new retry fixture
and froze the historical negative-control rim's old elevation/road frame without
weakening current-map support tests. Native review also caught and corrected a
worker GPU-buffer binding defect before integration acceptance.

Twelve serial release native cases pass, plus18finite paired ragdoll lab cases.
Both Metropolis branches/rigs, both police rigs, original model poses, pedestrian
solids, legacy City/Valley and crop persistence/pause/reset are covered. An
interrupted police batch and a wrapper screenshot-path failure are retained and
excluded; repeated standalone police cases pass. Hall, officer, original outfit,
road entry and crop art were visually reviewed. Route wall p9522.376ms remains
above16.7ms; single-pair crop wall samples do not establish a GPU speedup or globally
fastest method. Art/feel, full-course performance and wider hardware stay open.

Four new IDs tracked: RACE-TRIM-01 verified and moved toPart00; CROP-TRAIL-01,
ELEVATION-01 and POLICE-ART-01 remain open for remaining acceptance.221verified /
253open, README/PROGRESS/TASK_PLACEMENT synchronized. Shared provenance refreshed;
source/binary hashes and full report: testing/next-pass. Local Conventional
Commits only; no new merge/push or next major part.

## 2026-11-09 — Multiplayer/UI/walking follow-up under integration

User requested actual multiplayer and subagents, clarified walking animation
rather than controls, and selected player hosting across different internet
connections: Quick Play joins an available <=250ms host or becomes host. They
rejected operating a separate directory. Two fresh contextless Astra/medium
workers own network/UI; the existing Astra walking worker was reused when the
agent limit prevented another fresh worker. Root coordinates and reviews.

Added ONLINE-PLAY-01 and UI-RETRO-BG-01; no completion yet (221verified/255open).
UI source and visual gait checkpoint integrated first. Initial UI12tests and
10-state native captures passed before later overlay/status changes; walking
22animation/12model/10recovery tests passed before final native after-review.
Network12focused tests passed, including isolated DHT, lease ownership/handoff,
probe validation, interpolation and offline cleanup; actual multi-process race
and final combined checks are pending. Native UI fixtures are not online proof.

Review caught launch input consumed by menu, missing in-race Leave, stale
post-reload menu state and incomplete guest/offline cleanup; fixes are being
integrated and require reruns. Public Mainline discovery and temporary UPnP
mapping avoid a user-operated directory but do not guarantee NAT reachability.
No automated public DHT announcements or live router changes are required for
fixtures. Dependency declaration review found attohttpc0.30.1 MPL-2.0; recorded
its exact source/license notice and scoped audit intake after reading upstream
license/FAQ. All580 declarations then pass; complete integrated audit remains.

### 2026-11-09 — Integrated online evidence and city-activity follow-up

Final rider gait capture152nonblank frames and original/fallback11-state UI plus
22-stage offline flow pass; last online-overlay HUD mask recapture remains.
Rendered Metropolis host/guest matches598snapshot hashes and observes movement,
attack, crash, foot recovery and remount. Compression/750m interest reduces measured
encoded payload from45.6Mbps JSON to6.32Mbps per guest, still above production
bandwidth target. Full508-test checkpoint passed; later compression/EOF changes
await final integrated rerun. Failed native checkpoint/Valley short-window receipts
remain evidence, not overwritten successes. Separate-network/NAT is unverified.

User adds multi-rider queue escapes, more city traffic, doubled city pedestrians
and occasional crossings. Added AI-QUEUE-02 (Part01) and CITY-ACTIVITY-02 (Part06),
221verified/257open. Separate workers own queue behavior and city activity; rural
counts and accepted physics remain unchanged. Implementation/verification pending.

### 2026-11-09 — Multiplayer/walking/city follow-up verified locally

Implemented Host/Join/Quick Play with15-second automatic start, private/public
lobbies/map selection, bot takeover/fallback, original local UI backgrounds and
travel-matched rider gait. Review corrected remote-human obstacle holds/camera/
footsteps, results-clock regression, buffered EOF delivery, downed-NPC offline
cleanup and Quick Play losing automatic Start after a legacy-map relaunch.

AI-QUEUE-02 now activates the existing predictive passing planner for crawling
rider queues; four-rider physical bike/car queues drain safely with sustained
passing. CITY-ACTIVITY-02 adds48city-only cars and doubles urban pedestrians to384,
with10authored crossing routes and bounded gap-aware admission. Generated geometry
and64shared/rural cars remain unchanged. Native survey exits0 with14reviewed views,
144main cars,384walkers and two completed opposite-side crossings. CITY-ACTIVITY-02
moves toPart00:222verified/256open. Dedicated rendered queue/full-course human
balance and broad UI/network acceptance remain open.

Final529workspace tests pass (1existing long soak ignored), strict clippy/fmt,
locked build and580dependency/180asset declaration/hash audit pass. Five population,
three audit and five roadmap Python tests pass. Native private Metropolis→Valley
reload matches1497snapshots; final loaded-Metropolis pair matches1495, with all
movement/attack/crash/foot/remount states and noerrors. Admission/late takeover/
fullcapacity/hostloss and150ms loopback-delay fixtures pass. No separate-WAN/NAT,
packet-loss,15scattered-human or production performance claim: observed farm-area
wire payload remains6.41Mbps/guest and debugcity wallp9532.73ms exceeds16.7ms.
Final Quick Play map-selection change has the final regression/build evidence;
preceding native captures retain their exact binary identity. No new engine/rig
migration, public deployment, push, main merge or automatic next major part.

## 2026-11-10 — Controlled performance follow-up (ADR134)

User requested commit/push/merge followed by optimization. Fast-forwarded local
main to d8824bf and created codex/performance-profile-pass. The GitHub push is
blocked on the desktop credential prompt; origin/main remains2665e73.

PERF-08/P16-003: skip traffic forecasts while pedestrian crossing admission is
impossible, add physical pair-type diagnostics and fix a stale native destruction
fixture count against the expanded world. Preserve all geometry, city population,
physics/recovery and damage behavior. Rejected worker-count changes (no total
frame gain), standalone breakable bodies (settling regression) and damage-sweep
prefilter (no consistent measured benefit). Retain their evidence, not their code.

Final529workspace tests (one existing soak ignored), fmt, strict clippy, locked
debug/release builds and audit pass. Native13-kind destruction/reset and both-rig
recovery pass. Controlled release surveys show lower gameplay CPU work but total
frame gains overlap run variation; no16.7ms/GPU/full-race/hardware acceptance.
Evidence: testing/performance-pass. No new task IDs/closures;222verified/256open.

## 2026-11-10 — Hills course (ADR135, HILLS-01/02/03)

User supplied a12.5km city/hills/western-town/dunes diagram, explicitly revised
the town merge to occur inside town, then reinforced strong relief/turns and
added trees/grass/farms/liveliness. Three requested subagents split route/content,
world/scenery and selection/survey integration. Original procedural additions
reuse existing cow/human/truck assets; no retail media or dependency migration.
Parent reviews source, collision, actual placements and native images; early
535-test run passed, later actual-placement test exposed overconservative stock
reservations and initial survey exposed a never-stepped query pipeline. Final
receipts and remaining gates are tracked in testing/hills/README.md.

Hills final:536workspace tests (one existing ignored soak), format/strict Clippy/
locked build/audit and five roadmap tests pass. All five native cases pass:
11-view Hills survey with1506road rays (max0.025898m error), west/east town plus
hill crash/recovery, baseline rig recovery and Metropolis regression. Every
tested remount has0.0000m pickup error. Final census includes737trees,4farms,
27cows,15static horses,21cowboy residents,8parked trucks and40,035grass tufts.
No new task closures:222verified/259open. User art/feel, full driven-race and
hardware/performance gates remain open; no next major part started.

## 2026-11-10 — Extra Hills polish (ADR136)

User authorized agent discretion for dust/birds/clouds/lighting/cacti/tree props
and mounted cowboys. Three agents added bounded atmosphere, five analytic
fixed-step horse patrols, original desert/forest props and western detail.
Parent integrated runtime/native fixtures, corrected effect scheduling in
headless fixtures and final-camera ordering, and strengthened dynamic-wheel
dust eligibility/native mounted collider verification. Smaller stone variants
restore steep-shelf coverage without weakening footprint guards.
HILLS-04/05/06 remain open for human art/full-course/hardware acceptance.
Final receipts are maintained in testing/hills-polish; no next major part begun.

Native close-up review found and corrected mirrored storefront lettering with
a front-view glyph regression. Initial west/east smoke then exposed an existing
Hills-only recovery gap: a wreck outside continuous rails left automatic foot
recovery oscillating inside. Retained diagnostics distinguish cumulative walking
from real displacement. The follow-up extends exceptional delayed rescue to
Hills and reports it explicitly, preserving rails and accepted physical rigs.

Final polish verification: 548 workspace tests (one existing ignored soak),
strict Clippy, formatting, locked build, asset audit and five roadmap tests pass.
All six native cases pass: active effects/mount collision/pause, 1,506-probe
course survey, west/east extended recovery, baseline-rig and Metropolis controls.
West/east each record [0,1] rescues after 120.011 stationary seconds with +5s;
baseline/Metropolis each record [0,0]. No requirement closures: 222 verified /
262 open. Source/art is original or reused from the existing catalog; no new
external media or dependency. Human full-course art/feel and hardware/performance
acceptance remain open.

## 2026-11-10 — Civilian circular-wait correction

AI-QUEUE-02/P08-005: reproduced three closely spaced cars selecting a circular chain of leaders and remaining stationary. Replace fuzzy station ties with strict progress ordering and identity only at exact ties. Preserve normal yielding, speeds, physical rigs and populations. Add both-direction production regressions, a two-minute production-fleet check and an opt-in rendered queue survey. Evidence and final verification: testing/traffic-stall. No external code/assets or new task IDs; broader traffic acceptance remains open.

## 2026-11-10 — Hills city, canyon and lived-in countryside revision

HILLS-07/08/09/10: reuse the final kilometre of Metropolis street and scenery,
blend into sharper elevated hills, limit guardrails to curves, and cross an
original suspension bridge above a physical canyon. Follow-up removes all low
bridge rails and replaces thin round hangers with square 0.65 m supports.
Optional Assisted control now hands unsupported bridge departures to existing
physical recovery; steering and actual combat kicks are covered on both sides.
No accepted rig/mass/joint tuning or default-map change.

Fresh agents added actual Metropolis farm catalog props/materials, denser
western frontage, central ranches, coherent dirt backlots and roadside clutter,
eight articulated eighteen-wheelers, six tractors and bounded original volumetric
clouds. Static Hills solids preserve collision while omitting redundant fixed
body state; normal-map material event filtering avoids unnecessary whole-world
mesh scans. Same-binary controls separate collision-state and cloud cost.
Evidence, final verification and remaining performance/art gates are maintained
in testing/hills-revision. Requirements stay open for full-course human and
hardware acceptance; no next major development part begun. Concurrent civilian
queue work is preserved separately.

Final overhead review required a further median/outer-field pass: 42 compact
courtyards and 54 broad crop/orchard parcels make shared town space visible.
Exact-coordinate terrain caches and a conservative road-distance bound retain
layout while avoiding repeated generation queries. Native bridge evidence is
adapter-specific: Assisted enters physical ragdoll recovery; native-default
Dynamic physically falls while mounted in the tested lateral departure. Assisted
remounts without exceptional rescue; Dynamic stays mounted in this trajectory.
Actual combat-triggered native races
remain a separate acceptance gate.

Final checks: 575 workspace tests pass (one existing ignored soak), strict
Clippy/format, locked debug/release builds, asset audit and roadmap checks pass.
Native scene/road probes, both adapter departures and west/east/baseline/Metropolis
controls are recorded with per-case hashes. Paired final-release timing averages
21.805 versus20.765ms frame-window medians (4.8% lower) and8.342 versus7.369ms
weighted physics/fixedstep (11.7% lower) for fixed-body versus collider-only
scenery. Cloud differences are below run variation; no cloud speedup or60FPS
certification. One competing-game sample is explicitly excluded. Indexing was
temporarily suspended for timing and restored. README remains222verified/266open.

## 2026-11-10 — Selectable Hills and landscape delivery (ADR138)

Fresh agents implement seven requested refinements: native Metropolis/Hills
course cards with legacy access and preserved session/profile handoffs
(UI-COURSE-01), regular bridge hangers attached to girders/outriggers (HILLS-08),
whole-parcel gentle farm selection retaining six Metropolis asset farms
(HILLS-07), an original canyon river (HILLS-11), separate volumetric cumulus
(HILLS-10), a light-aligned solar disc (SKY-SUN-01), and bounded 500 m city/Hills
shadows preserving the near cascades (SHADOW-RANGE-01). Accepted rigs, strong
road relief, open bridge edges and existing course content remain integrated.

Locked debug/release builds, strict Clippy, formatting, 590 workspace tests
(one existing ignored soak), declaration audit and five roadmap tests pass.
All seven native cases pass across the recorded binary hashes. Menu receipts
verify Metropolis → Hills → Metropolis, preserved credits/live preferences,
return and same-course retry. Assisted falls 82.54 m and recovers; Dynamic chassis
descends 82.677 m and remains mounted, both without exceptional rescue. West,
east, baseline-rig and Metropolis controls each record zero rescues. Six farms
passed visual review. Final active replaces the initial active evidence after
river texture/camera polish: 26 captures show supported water and softened
ripples. The first east attempt was interrupted and is excluded; its final rerun
passes. Four clean final-binary ABBA shadow 150/500 m runs with clouds constant
measure pooled median window means 21.848 → 21.773 ms and P95 window means
23.544 → 23.814 ms; differences remain within variation. No speedup, raw-frame
P95, GPU timing or 60 FPS claim. Full-city/hardware acceptance remains open;
`testing/hills-delivery/PERFORMANCE.md` records scope and controls.
Evidence and exact binary hashes: `testing/hills-delivery/REPORT.md`. No new
dependencies or retail media; original procedural/reused assets are recorded in
the provenance registry. Full-course art/feel, sustained performance and hardware
acceptance remain separate. Integration owns final merge/commit/push receipts.


### 2026-11-10 — Hills grid, stopped rivals and road-edge seam (ADR139)

Reproduced the off-street rear Hills slot and 23 false-empty Metropolis planning
envelopes (first2940m, matching the user screenshot). Preserve the curved city
apron through -300m and normalize AI bounds, proxies and passing memory to the
street centre. Close the8cm exposed grass seam with a bounded soil underlap.
All594workspace tests, strict lint/build/audit and five roadmap checks pass.
Native Metropolis seam views and both Hills current/baseline recovery runs pass
with no exceptional rescues. No physical rig/handling/dependency changes or new
IDs/checklist closures. Full evidence: testing/hills-grid/REPORT.md.

## 2026-11-10 — Issue29 sand follow-up (ADR144)

Investigated missing Sand classification and asymmetric adapter penalties;
implemented shared sand power/rolling rules and authored Hills biome agreement,
with route-aware pavement protection. RFU-SURF-01/02/03 added to Part01.
Verification/acceptance tracked in testing/sand-speed/REPORT.md; no gate closed.

## 2026-11-10 — Guardrail issue33 source checkpoint (ADR148)

Prepared original turn-side barriers with separate road-height/ground-foot calculations, 96m maximum runs, 16m gaps, solid flared ends, physical gap/support/terminal tests and a native bidirectional walking fixture. Added RFU-RAIL-01..05 as open requirements. Compilation and native execution are deferred for the coordinated performance window; no test completion, performance gain, push or merge is claimed.
## 2026-11-10 — Issue #28 recovery run and reverse heading (ADR143)

Isolated codex/issue-28-movement from754f9ddb after fetching main. Confirmed6m/s
recovery controller and fully blended run above4.5m/s. Refined visual silhouette,
added swing terrain probes and signed reverse heading. Added RFU-MOVE-01..04 with
acceptance retained. Source/tests and native evidence tracked separately under
testing/issue-28-movement; no physical rig, recovery policy, media or dependency
changes. Commit/push/merge recorded only once performed.

Issue #28 headless checkpoint:596workspace tests passed,1existing opt-in soak
ignored; strict Clippy, fmt, build, audit and5roadmap tests pass. Eight translated
terrain cases keep established sole slip below0.688mm/tick. Frozen native binary
and draft checkpoint prepared for the parent's coordinated GPU window; native
visual/recovery evidence and user/performance/hardware gates remain open.

## 2026-11-10 — Issue25 profiling and exact generation reuse (ADR140)

Added opt-in loading spans, raw race-frame evidence, render-pass diagnostics and
an isolated paired survey harness. Profiled both courses before optimization;
recorded loading/CPU/GPU boundaries, sampling policy limits, cache provenance and
lighting/budget research. Introduced scoped exact terrain reuse and duplicate
Metropolis corner-color reuse with original geometry/collision preserved;
loading-only job admission and fixed-schedule executor remain controlled
experiments pending measurements. RFU-PERF-01..05 are new stable open requirements.
Pinned dependencies remain unchanged; parent coordinates the requested0.20
migration separately. Verification and paired results are recorded in
`testing/issue25-performance/REPORT.md`; broad performance gates remain open.

## 2026-11-10 — CI diagnostics and explicit roadmap encoding (ADR140)

Preserve the900-update/240sleeping-body gate while adding failure-only per-owner
and1800-update observations for a pre-existing Ubuntu CI failure not reproduced
locally. No rig or physics changes. Fix Windows locale-dependent roadmap reads
with explicit UTF-8 and correct issue25 Part01/16 summary counts exposed by the
complete roadmap test. All five checks pass under an ASCII locale; Rust diagnostic
compilation/CI and root-cause resolution remain pending.

## 2026-11-11 — Controlled repeats, defaults and bounded natural-sleep verification (ADR140)

Detected a replacement indexer, rejected two attempts and downgraded earlier
pairs to exploratory evidence. Verified/pidfd-paused parent2060 for eight fresh
cold-asset/reverse-warm runs, then restored it in finally. Loading/frame gains
repeat with exact census/support parity; retain the terminal audit61ms sample
and separate it from a common gameplay cohort. Both rigs recover on both maps.
Select measured single-dispatch/eight-loading-job defaults with controls intact;
add actual lab executor reporting and terminal verification spans. CI establishes
bounded late natural sleep, so preserve15s stability and require all240sleeping
by30s on the same fixture for both executors, without physical tuning. Final
rebuilt checks and merge remain pending.

## 2026-11-11 — Final executor parity and source-frozen verification (ADR140)

All18lab scenarios match exactly across explicit single/multi executors after
excluding only executor label/wall time. Final598workspace tests, strict Clippy,
format, provenance audit and Python evidence/roadmap checks pass. Same-fixture
pileups first naturally sleep all240bodies at update877 in both executors.
Terminal survey markers now flush partial raw windows and normal shutdown drains
the bounded writer; final release is frozen with exact source/tree/binary hashes.
Default-only four recovery/two survey checks pass with zero dropped logs and
retained terminal audits; final publication/merge follows required checks.
Broad RFU-PERF gates remain open; no physics/dependency migration is included.

Issue29 follow-up verification: 603 pre-integration tests and four native cases
pass. Corrected only native fixture broadphase readiness after its first failed
run; preserved that failure. Integrated PR35's measured defaults/exact terrain
cache while retaining shared sand tint. Human feel/steep-uphill reverse remain
open; final integrated checks and publication are tracked in the sand report.

### 2026-11-10 — Racer/killer follow-up issue26 (ADR141)

Traced rear-target speed subtraction, combat-capped queue detection and outbound-only
passing bounds. Implement forward-priority/hysteretic moving combat, race-progress
fallback for stopped targets, and bounded opposite-road/return checks. Added
RFU-AI-01–08 and diagnostic native race fixture;604workspace tests and strict
code/build/audit gates pass. RFU-AI-03/04/05/07 verified; four gameplay/native
gates remain open, with no hardware/frame-budget claim. See testing/issue26-racer-ai/REPORT.md.

### 2026-11-10 — Issue26 native completion and crossing review

Corrected the native survey completion predicate (605 tests). The frozen binary
completed the unteleported race in 435.238 simulation seconds; current and
baseline recovery runs passed with two remounts each. Retained the earlier
incomplete receipt and the late-section sampled stop rather than treating course
completion as blanket queue acceptance. Review found an unrelated-crossing veto;
restricted it to predicted home-corridor/time conflicts, with 20 focused rule
tests passing. Added opt-in synthetic segment/proxy/static-hit diagnostics; full
checks and native validation of this refinement are pending the measurement slot.

### 2026-11-10 — Issue26 stationary-human acceptance and bounded reposition

The full crossing refinement passed 608 tests. A new real Dynamic stationary
human fixture then exposed a narrow-road killer queue: the fallback consumed its
launch gap. Reserving maneuver space fixes the approaching queue. A separately
compressed queue exposed off-road A* escape and unsigned reverse-speed feedback.
Retain signed motion with a tested resting-jitter deadband and use a rear-swept,
live-gap reverse goal. One bounded pulse per rider clears the compressed fixture
in 22.82 simulation seconds, with health, road bounds and clearance preserved.
Retain the earlier failures and slower four-pulse result; 130 core tests pass.
Final combined gates and civilian lane-change interaction verification remain
pending integration. No vehicle/ragdoll physics or dependency changes.


## 2026-11-10 — Civilian overtaking (issue27 / ADR142)

- Read the current checklist/decisions/issues, installed0.17.3/0.32.0 lockfile,
  AI-QUEUE-02 evidence and traffic/racer adapters before implementing.
- Added RFU-TRAF-01–04 as open requirements in Part06, retaining prior tasks.
- Implemented a pure civilian pass/abort/merge FSM, deterministic local claims,
  continuous sampler-grounded lane changes, live large-vehicle/merge checks,
  width-derived lateral clearance and a bounded static collider sweep.
- Upright oncoming bikers intentionally do not veto passes; solid contacts,
  downed-person yielding, accepted rigs and speed hierarchy remain integrated.
- Added core motion/obstruction tests, production-course support/biker/static
  fixtures and opt-in native --civilian-survey. Full verification receipts and
  limitations are recorded in testing/civilian-overtaking/REPORT.md.

Issue27 validation follow-up: the first full suite exposed transient same-class
launch queues initiating passes from compressed envelopes. Preserve strict queue
order, reset impediment memory while a leader accelerates, require clear initial
body spacing, and still allow a genuinely stalled same-class motor after six
stable seconds. Integrate acceleration into the passing horizon and projected
merge point. Final609 workspace tests, strict lint/format, debug/release builds, audit and
roadmap checks pass; native timing/motion/recovery remain scheduled separately.

Issue27 native follow-up: frozen release passed eight selected civilian passes
(Hills real tractor/semi and Metropolis car-body speed fixtures, both directions),
with completed home returns, zero selected collider intersections and sampler
support errors. Active/baseline Hills recovery both remained unassisted. Captures
and receipts reviewed; concurrent compilation makes these functional evidence
only. Normal-fleet opportunities and quiet CPU measurements remain pending.

Issue27 full-fleet follow-up: quiet120s Hills38 actors completed11/11 passes;
Metropolis144 actors completed1/3 with two active. Extended240s diagnostics
exposed a73s stationary endpoint abort, preserved as failed behavioral evidence.
Use actual predicted pass+return distance for open-road admission; both-direction
core/adapter regression passes. Corrected240s Metro completed9/10 passes with
one healthy final merge, no aborted returns. Original quiet planner P95 was
0.176ms Hills/1.031ms Metro; whole-adapter P950.274/2.603ms. These are CPU costs,
not GPU/frame-budget acceptance or a speedup. All RFU-TRAF IDs remain open.

Issue27 integration: merged PR35 main76d4fe into traffic branch, preserving
all tasks/ADR140+142. Combined615 tests, strict lint/format, debug/release and
audit pass. Integrated native Hills4heavy overtakes and Metro240s144vehicle
census pass;9complete/1healthy ongoing merge, no aborts. Source/binary hashes
and loaded-functional limitations are retained in civilian-overtaking evidence.

## 2026-11-11 — Issue28 native matrix and performance integration

The capture-drain correction passes all ten native cases, including both adapters,
both rigs and18paired lab scenarios. Preserve the initial shutdown failure.
Visual spot review finds readable running/terrain contact; user motion acceptance
remains open. RFU-MOVE-03/04 move to verified;01/02 stay open. Merge measured
performance main76d4fe15 without changing movement source. Final strict checks
and publication follow; no runtime timing or hardware certificate.

Issue28 final integrated source passes600workspace tests,1existing ignored soak,
strict all-target Clippy, locked build, fmt, audit and roadmap checks. Grouping
the native preview queries in a named SystemParam resolves lint without changing
query behavior. Native10/10 evidence retains its exact earlier executable hash.

### 2026-11-11 — Issue26 combined civilian and reverse regression

Integrated measured performance defaults, civilian overtaking and published
reverse-heading corrections. Cache measured civilian lateral motion once per
car/tick and cap forecasts at committed lane endpoints, including return paths.
Corrected a test-only legacy-map mismatch; actual Metropolis incoming overtaking
now passes with2.632m minimum car clearance. All four physical racer tests pass;
compressed queue retains22.82s/one pulse under the reverse correction. Full
combined gates and final native segment are running; no frame-time claim.

## 2026-11-11 — Issue28 integrates civilian overtaking

Merge main7a63ef18 and preserve both opt-in smoke dispatch entries, all IDs and
both sets of evidence. Remove inherited PROGRESS conflict markers without losing
its civilian/performance summaries. Combined strict checks and paired native
manual/automatic recovery are required before publication; terrain/human-motion
approval and broad hardware/performance remain open.

Issue28 combined final:617tests pass,2explicit benchmarks ignored; strict checks
and paired native recovery2/2 pass. Exact binary hash and separate original
ten-case receipts retained. Final GitHub Linux/Windows CI remains asynchronous;
no CI success or hardware acceptance is inferred from local checks.

## 2026-11-10 — Issue41 running hand follow-up

The user found rear-facing flat palms during the new urgent run. Add a bounded
visual wrist correction from the authored arm chain, with bilateral thumb-axis
checks through 240 phases. Flat and uneven native animation review cases passed
and sample front/side frames were retained. Full workspace checks and paired
anatomical/baseline native recovery passed; integrated performance and human
motion review remain open under RFU-MOVE-05/06.

## 2026-11-10 — Issue26 full-road racer lane follow-up

User playtesting showed riders choosing only the traffic side. Remove the
outbound-only default and allow intentional opposite-side racing lines through
the existing oncoming/return and static-body forecast. Preserve civilian lane
rules, physical collision, committed pass return and the bounded legacy roam
needed by trap escape. The first full run exposed a positive-lane-only fixture,
a pass-return regression and a wider-roam trap regression; all were resolved
without disabling collision or escape. Final merged source passes 641 workspace
tests, strict Clippy, format/build, roadmap and audit; native full-race and quiet
planner-cost review remain open.

## 2026-11-11 — Issue40 four-candidate selection checkpoint

Four fresh agents measured CPU traffic snapshot reuse, facade visibility sync,
loading admission/cache and grass-wind shader arithmetic against the frozen
e2a31918 Hills/Metropolis baseline. Draft PRs #44, #49, #46 and #45 retain raw
receipts and failed or mixed trials. CPU has the smallest nominal warm weighted
ratio (~0.9886), narrowly ahead of GPU (~0.98955), but its final source was not
reprofiled and cold Hills p95 regressed. Facade Hills p95/tail regressed and the
loading cache was reverted. None is merged; final-source, full cold and
combined-content reruns remain RFU-PERF-06/07 gates. See
testing/issue40-selection/REPORT.md.

## 2026-11-11 — Guardrail issue #33 integrated functional verification

Merged current docs-only main after the source/test integration. Corrected the
headless fixture to start from resident shoulder support and added bounded
collision-grounded automatic end/gap guidance for trapped riders. 623 locked
workspace tests, strict Clippy/format, locked debug/release builds, audit,
roadmap checks,18 paired lab cases and seven native Hills/Valley functional
scenarios pass. Both turn-side runs and baseline/current recovery have zero
exceptional rescues in the native fixtures. Eight close captures and their
visual limits are recorded at testing/guardrails/REPORT.md. RFU-RAIL-01..05
remain open for human full-course and broad hardware/performance acceptance.
Draft PR #51 records publication; merge is tracked separately.

## 2026-11-11 — Racer lane follow-up on merged city and guardrail content

Removed the unconditional right-side free-racing target and allowed a safe
opposite-pavement racing line through the existing oncoming/return forecast.
Integrated PR51 guardrails and PR53 city activity. The combined source passed
647 workspace tests (two ignored), strict Clippy/format, locked debug/release
builds, audit, roadmap and progress checks. An unteleported release Metropolis
race finished slot zero at frame31,689; current and baseline recovery smokes
passed. The race still sampled avoidance-driven stops up to33 seconds around
station3,040, so broad queue behavior and frame-cost acceptance remain open.
See testing/issue26-racer-ai/REPORT.md and ADR141. No rig/physics or dependency
migration was made here.

## 2026-11-11 — Sand issue #29 final integrated verification

Merged guardrail, city activity and racer AI main into shared sand handling.
Resolved additive README/progress/history conflicts without closing RFU-SURF-01..03.
The final runtime source at 3e22ae81 passed 656 locked workspace tests (two
existing ignored), strict formatting/Clippy, locked debug/release builds,
audit and roadmap checks. Four native authored-ground and both-rig recovery
cases passed on one release binary SHA256 2eeea11c7edb3cb2f5bd90a66c36d8e4bb5afe2c1b2ffcc4b5d41f266dd1b462.
The reproducible runner and compact receipts are in testing/sand-speed/REPORT.md.
Human feel, steep-uphill reverse and broad hardware/performance gates remain open.
No imported code/media or dependency migration was added.
## 2026-11-10 — Issue31 original Hills world enrichment (ADR146)

Add deterministic spatially batched distant conifers with resident trunks,
wider irregular grass, remote western mesas, storefront goods/lanterns and
four larger shop-front display variants, plus outskirts cacti/rocks. Raise
canyon ground and its seated river together12m.
Riding-height follow-up: guarded striped stalls, handcarts and hitch/barrel
groups share storefront meshes and compound collision; backlots gain two low
shrubs/cacti each. Preserve route clearance and existing entity budgets.
Integrated with e776605f: 663 locked tests, strict format/Clippy, debug/release
builds, audit and roadmap checks pass. Six native Hills/Metropolis functional
cases pass; Valley world-survey writes15sections/657roots but exits1 at17.94ms
p95 against the pre-existing16.7ms gate. CPU sample comparison is confounded by
the concurrent atmosphere merge. Art/full-course/GPU/hardware gates stay open.
Preserve roads, legacy maps, dependency pins and accepted rigs. Seven RFU-WORLD
IDs remain open. Strict/native/combined performance verification in progress;
see testing/issue31-world/REPORT.md. No publication or broad acceptance claim.

## 2026-11-10 — Issue32 bounded scenic herds (ADR147)

Added RFU-WILD-01..05 to the live checklist. Implementing original wild horses
and bison with bounded deterministic groups, swept off-road reservations, exact
resident-terrain path samples and shared near/far meshes. Explicit no-crossing,
no-contact policy retains existing cows/cowboys and accepted physical rigs.
Technical/native/performance verification is pending, not a completion claim.

## 2026-11-11 — Issue #32 wildlife integrated verification

Integrated original scenic horse/bison herds with PR55 Hills terrain, woodland,
frontage and guardrail content. Kept five RFU-WILD tasks open. Final source passed
667 workspace tests (two ignored), strict formatting/Clippy, locked debug/release
builds, audit and roadmap checks. One native 1,100-frame visual survey and four
alternating release enabled/disabled cost runs completed. Wildlife system CPU
median was 8.6–9.0 µs in the near-herd fixture; total frame medians overlap run
variation. Full-course/native both-rig, art, GPU and broad hardware gates remain
open. No new collider, save field, fixed-loop physics or imported media.
