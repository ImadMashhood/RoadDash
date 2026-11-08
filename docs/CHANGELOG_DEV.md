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
