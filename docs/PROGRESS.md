# Development progress

## Current live roadmap — completed first

README is the single live task source. Part 00 contains all 180 verified tasks;
Parts 01–16 contain all 223 unfinished/partial/unverified requirements. The reorganization preserved status; four later collision corrections were
verified and moved to Part 00 in the subsequent implementation session. Historical numbering in task IDs is stable;
current README section determines execution order, not the ID prefix.

The complete eleven-family animation coverage and two-stage deliverables are now
explicit in README's dedicated Animation production section. Existing V2-02/V2-03
IDs own the expanded coverage. Animation A source investigation and the first draft authoring deliverable are verified; full art approval remains open.

Current scope: user-requested roadside variety,35 roles/20 authored groups and independently seeded sides; DRESS-02 verified by native224-owner/680-mesh/34-role/20-group census on the existing local Part05 branch (ADR056, testing/roadside-variety/README.md). Earlier scope: Part05 semantic dressing/materials plus explicit soft-contact, facade-transition and cloud/daylight follow-ups (ADR054/055, testing/part05-world/README.md). PR16 is merged; follow-up branch remains local. COLLISION-07, LOD-01, DRESS-01, MAT-01 and SKY-01 verified; MAT-02 and broad art/performance gates remain open. Earlier scope: user-requested high-rise/infill/road detail, stable facade loading and five-second unseen rear NPC crash recovery, followed by commit/push/merge and explicitly authorized Part05 (ADR053, testing/world-polish/README.md). KIT-04/05, PERF-06 and ANIM-12 verified. Earlier scope: Part04 authored Valley storey kit and deterministic bounded geometry integration (ADR052, testing/architecture-valley/README.md). KIT-01/02 and V2-04-005 verified; Part04 art/role/infill and performance gates remain open. Earlier scope: camera/contact/gait/wrist/crash-style fixes and default Valley restoration (ADR051, testing/recovery-polish/README.md). Earlier scope: Part 03 animation/recovery and corrected tuck/attack sides; ADR050 and testing/animation-part03/README.md record implementation and remaining gates. Earlier user-authorized scope: integrate and merge the approved biomechanical ragdoll with the current game (2026-11-06, ADR049). Automatic startup, explicit baseline, original crash-direction policy and current prop NPC skins are retained. Earlier scope: apply selected 3D prop drafts in gameplay (PROP-03, ADR048). Traffic/scenery and human/cow visual bindings now use the existing physical owners; full art, category placement, bespoke audio and production acceptance remain open. Previous scope: author actual 3D prop drafts. PROP-02 covers 369 mesh IDs/1,107 GLBs and an isolated native viewer; faithful concept likeness, actor rigs and live replacement remain open. Previous scope: extend the building workflow to all world props and actors. PROP-01 verifies scoped extraction/review infrastructure, with a full rebuild queue and 369 source groups covered by 66 selected draft concept sheets. V2-05/V2-06 retain identity/art/mesh/placement/behavior acceptance. Prior scope: correct collisions with downed riders/bikes using original-style contact routing (COLLISION-04; ADR045). This preserves accepted ragdoll settings and existing car entry behavior. Prior scope: assemble complete buildings using original-game facade/roof/end/side-street principles and make the playable race a 5.3 km generated city. CITYGEN-01 verifies the new default City Run: 977 whole lots, 53 blocks, eight district zones, 85 measured City-eligible facades, actual 3D front/return modules, shared roofs and detailed rear walls. Physical collision, supported grid/roads/sidewalks, recovery paths, native retries and finish remain integrated. Original Valley content/quotas and the accepted physical rider/bike/ragdoll remain preserved. MESH/V2-04 retain final art, optimization and broader production gates. Earlier open work remains tracked:
interpolation/terrain gait/active recovery, buildings, props, NPCs followed
by landscape/sky, weapons, police, original Part 09 playtests and Part 10 network
spike. Later production/economy/online/QA requirements remain tracked.

All 403 task IDs are in README exactly once. This includes four new CRASHREF IDs;
the inherited 357-ID header was stale after the preceding animation additions. roadmap/BLUEPRINT_PARTS_01_08.md is frozen historical evidence;
sync_progress.py now reads README only. roadmap/TASK_PLACEMENT.json records the
current section for every ID; roadmap/README_REORGANIZATION_STATUS.json protects
all checkbox states at the time of this move. Acceptance details remain in
roadmap/EXECUTION_V2.md and roadmap/SUPPORTING_CONTRACTS.md.

The implementation notes below are historical receipts. Their Part numbers refer
to the original blueprint unless explicitly marked V2; consult README for current
placement. No new gameplay work, tests or milestone completion is implied by
moving an item. Existing partial work and unverified exit gates remain open.

## Current collision correction scope

COLLISION-01/02/03 originated in Part 01 and are now checked in Part 00 after
verification. They cover input-owned mounted heading, removal
of scripted collision launch/spin, and explicit arcade car-speed loss plus a
mass/inertia/gravity energy envelope. The new request supersedes P06-022's earlier
clearance jump. Full over-car choreography and combat swerve remain open V2 tasks.
The dedicated eleven-family animation plan is documentation, not completed clips.

## Animation A — current authoring session

247 numbered draft candidates, 27 mapped use cases, 247 primary renders and 57
supplemental views are available in art/animation. The editable Blender package
has 247 single-frame constant pose Actions. All 247 JSON poses round-trip with
maximum matrix difference 4.77e-7; no physics bodies are present in the library.
140 Rust and 14 Python tests pass; format, Clippy, build and authoring CLI pass.
See testing/ANIMATION_A.md and animation/README.md for receipts and open work.
ANIM-01 is verified infrastructure; ANIM-02/03/04 retain hand/sequence/support and
specialist-pose gaps. V2-02-004 source research is complete. No pose is approved,
no runtime playback is connected and the accepted passive ragdoll is unchanged.
The user handles the previous PR merge; no CI wait is a gate for this session.

## Animation context correction — 2026-11-01 (supersedes V1 semantics)

V1 guessed meanings from inspection-based helpers; its labels are rejected as
native reference authority. ANIM-05 records the corrected trace:104 bank/code
slots,194 ordered descriptors and371 sprite usages, with independent CANS/EXE
reader and source-selector evidence.194 descriptors/rear selections agree with
the separately compiled pinned reconstruction; native original execution remains
unverified. See animation/context/README.md and testing/ANIMATION_CONTEXT.md.

The new isolated art set has138 human /16 bike-only layouts, labeled by action
and phase. Police35 phases are metadata-only; code43's five empty source images
have no invented recreation. ANIM-06 remains open for silhouette/support/fist/
weapon/cloth review, unresolved callers, secondary views and approval. Passive
ragdoll and runtime art are unchanged. Old ANIM-01 counts document infrastructure,
not verified action meanings. Source timings are logical counters, not Bevy clips.

## Authored-model baseline (ART-03, preserved history)

User scope: replace character/bike on codex/seated-models-preserve-ragdoll while
preserving the accepted physical ragdoll exactly. The new meshes bind to existing
visual joints; no simulated skeleton migration is authorized. Part 11 prototype
integration only, with animation polish/LOD deferred under ART-04/ART-05. See
ADR-027, art/README.md and testing/MODELS.md for actual verification receipts.
ART-02 and all unrelated milestone exit gates remain open.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| ART-03 / 11 | Replace approved rider/bike visuals without losing accepted crashes | authored GLBs/pose; current body/visual separation | Four human states, riding/wreck geometry, unchanged body/joint descriptors and 180-tick trajectories, full recovery; automated and native evidence |
| ART-04 / 11 | Polish retarget, fingers and interpolated authored transitions | ART-03; static pose library | Contact-correct gestures/deformation and readable transitions; pose review and native clips with physics parity |
| ART-05 / 11 | Add art LOD and broader hardware evidence | ART-03; profiler | Collision-independent distance LOD and measured budget on supported hardware; native surveys and Windows review |

## Evidence and current boundaries

Part 00 artifacts: GDD.md, VERTICAL_SLICE.md, ORIGINAL_CONTENT_POLICY.md,
content/SHEETS.md, ../content/catalog.json, research/OBSERVATIONS.csv (23
immutable documentary observations), third_party/REPOSITORIES.md, ASSETS.csv and
license notices. Source/owner/confidence/use/follow-up are explicit; no historical
play session or rights grant is invented.

Part 01 implementation: ../crates/rr_core, ../crates/rr_content and the real
client subsystem plugins in ../src/runtime.rs, controls.rs, content.rs and
diagnostics.rs. ADR-001 through ADR-008 document scope/compatibility and rollback.
Test requirements and actual receipts: testing/TEST_MATRIX.md and
 testing/PARTS_00_01.md. Known limitations: KNOWN_ISSUES.md.

Full simulation extraction (P01-003/P01-004), complete authority-event contracts
(P01-006), a functional netplay mode (P01-007), and real Windows hardware
verification (P01-009/P01-010/P01-011) remain open. Subsystem plugins and pure
core/content crates are useful progress, not all eleven future crates or a server.

## Part 02 implementation and remaining gates

See tracks/AUTHORING.md and testing/PART_02.md. Authored 6.85 km Valley Run,
2 km handling fixture, 3D arc length/bank/width/surface/sector metadata, physical
ordered gates, separate collision ribbons, bank-safe terrain, F4/CLI inspection,
seeded farm/city/hill gameplay actors and actual elevated recovery are implemented.
Source files: ../src/track.rs, environment.rs, recovery.rs, game.rs and
../crates/rr_content/src/track.rs. Tests cover a complete no-teleport fixed-controller
race and one payout, endpoint/retry/shortcut rejection, 240 road collision rays,
boost/grass/cow/slope behavior and 100 high-speed ragdoll drops.

P02-004 remains partial only for optional branch/split/merge support. P02-010 has
both requested road lengths but representative busy-race pacing/fun measurement
is not established. P02-011 remains open until those requirements are resolved.
The user changed route order and closure; original task text remains in the index
as history, with the explicit current contract P02-012. P06-012 includes a broader
trap/traffic/time-loss audit than the new drop stress test, so it stays unchecked.
P07-003/P07-005 are partial (bike ordered gates now work; out-of-bounds/foot and
same-tick rider/bike outcomes remain). P07-007/P12-005 retain class/track save work.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P01-019 / 01 | Prior Windows CI audit failed decoding UTF-8 author metadata as cp1252 | metadata/text readers | Explicit UTF-8 and a simulated Windows code-page regression; Python tests; fresh remote matrix still required |
| P02-012 / 02 | User's point-to-point map correction | authored spline, gates | Separate endpoints, no return geometry/wrap; full fixed-controller race; missed gate stops/retry, ordered-gate tests |
| P02-013 / 02 | Environment affects gameplay, per user sketch | terrain, solid props, fixed patrols | Seeded farm gaps/fences/cows lift riders; city streets/solid crossing cars/pedestrians; elevated hills/guardrails; actor counts, cow test and native captures |
| P02-014 / 02 | Visual review found grass intruding on banked asphalt | terrain and separate road meshes | Terrain follows bank with clearance; first collision surface is road at 240 sampled positions; winding and resolution tests |
| P02-015 / 02 | Sea-level righting would break elevated recovery | terrain, runner/wreck recovery | Actual hill crash/get-up/run/remount completes at height; native fixture and screenshots |
| P03-012 / 03 | Boost release/exhaustion snapped from 270 to 198 km/h | speed controller | Excess speed decays by drag, not a target clamp; both input scenarios tested |
| P03-013 / 03 | Grass drag exceeded acceleration and drained health | surface metadata, AI | Four player/AI grass cases advance without health loss/knockoff; AI returns to road |
| P03-014 / 03 | User requests slightly tighter handling | validated bike tuning | About 20% greater lateral rates, old schema-1 catalogs load defaults, steering/frame-rate regressions pass |
| P06-017 / 06 | Uniform joint limits fold limbs implausibly | existing articulated bodies, Rapier motor API | Directional hinges, bounded twist and passive tissue resistance; 100 dynamic drops, anchor/axis/finite/ground checks, native crash captures |
| P06-018 / 06 | Wreck sticks to slopes; always-low friction drifts too far on flats | terrain ray normals, fixed physics | Low friction only on steep support; ramp regression and native hill/flat recovery |

## Part 03 implementation and remaining gates

The native default now uses the dynamic chassis adapter; the assisted controller
remains an explicit comparison option. See [contract](VEHICLE_AND_COMBAT.md),
[tests/measurements](testing/PART_03.md) and ADR-012–015. Pure direct-hit/contact
rules live in rr_core; versioned handling validation lives in rr_content. Runtime
vehicle, combat and NPC adapters are integrated into the fixed schedule.

P03-003–009 are verified by automated numeric/curb/lean/frame-rate tests and native
render/audio checks. P03-010 remains partial: a 600-second simulated aggressive
turning soak passes, but ten human rides/controller feedback are missing.
P03-011 stays open until distinct feel, beginner completion and fun are accepted.
The force model is road-assisted arcade physics, not a free-heading multibody
motorcycle. Existing Part 01/02 and foot-finish gates remain unchanged.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P02-016 / 02 | User requests fewer rails and richer scenery | seeded zones/solid terrain | Outer curve rails only; varied trees, detailed buildings/cross cars/pedestrians; native captures and source predicates |
| P03-015 / 03 | Strong road-centering undermined lane choice | dynamic forces | Reduced adjustable assist, no lane-zero magnet; offset-at-rest regression and F6 native overlay |
| P03-016 / 03 | Preserve AI avoidance when adding a new adapter | controller intent/fixed forces | Same AI/player intention path, launch grace; dynamic eight-rider grid test |
| P03-017 / 03 | Adapter exposed stale remount velocity and class mass loss | crash/remount, presets | Stationary protected remount and correct dry wreck mass for all three presets; regression tests |
| P04-010 / 04 | User clarifies colors mean racer shirts only | shared shirt palette/HUD | Eight racer dots match model colors, no actor-category markers; UI palette test and native image |
| P05-009 / 05 | Broad melee/start fights caused immediate knockouts | pure hit chain/LOS | Three direct hits, each gap <=3 s; reach/side/height/heading/occlusion and duplicate protection; core/game tests |
| P06-019 / 06 | Equal-speed contacts should transfer momentum | pre-solver velocities/contacts | Rear closing >=50 km/h or strong side impact wipes out; light taps nudge; threshold and game tests |
| P06-020 / 06 | Cows/pedestrians must react physically | articulated bodies/fixed messages | Distinct 10/11-body rigs, eight-ragdoll cap, reset and occupied-spawn-safe cleanup; headless/native actor checks |
| P06-021 / 06 | Sidewalk contact incorrectly caused crashes | support rays/Ground tagging | Low curb supports bike at elevation without knockoff; stationary and moving curb tests |
| P08-010 / 08 | User requests US traffic and safe grid start | AI intent/fixed patrols | Positive-right forward traffic, separated one-way cross lanes, no early AI fights/weave; source/native/grid tests |

## Part 04 — user-authorized follow-up to merged PR #3

Software implementation: native Gilrs gamepad input, analog force strengths,
rebindable keys/buttons, saved comfort panel, interpolated heading and camera
volume clearance, traction/speed cues and independent on-foot input context.
See [contract](CAMERA_AND_INPUT.md) and [receipt](testing/PART_04.md).
P04-005/006/007 are checked only for those implemented/automatically verified
capabilities. P04-004 is partial pending human high-refresh jitter/readability
review; P04-008/009 remain open pending real-controller finish and external tester
acceptance. The Part 03 human feedback gate is still open; no Part 05 start.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P04-011 / 04 | Numeric high-refresh camera and clipping/warp coverage was missing | interpolated pose, Rapier sphere sweep | Finite/continuous 60/120/144 Hz views; wall clearance and large-warp snapping; camera_tests.rs |
| P04-012 / 04 | Persisted/rebound input needs corruption and modal isolation protection | settings schema, live bindings | Roundtrip, future/corrupt file unchanged, conflict rejection, no hidden driving/countdown leakage; settings/controls tests |
| P04-014 / 04 | Native backend discovered an unmapped controller; do not infer real-device correctness | Gilrs/OS device access | Exercise real axes/buttons/default mapping and add an appropriately licensed mapping if needed; physical controller test (open) |
| P04-013 / 04 | Digital-only intentions would discard trigger pressure or latch disconnected input | native Gamepad, fixed intentions | Partial trigger yields lower real chassis acceleration; controller removal clears input and keyboard works; device/vehicle tests |

## Latest user corrections included with Part 04

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P04-015 / 04 | User finds character turn lean odd | rider/bike presentation | No cosmetic steering roll, wheels/suspension retained; neutral-root regression and native captures |
| P06-022 / 06 | Existing pedestrian path had no lift and fixed cow impulse varied by mass | collision event, pre-solver velocity, selected mass | Both civilian kinds ragdoll; each of three classes clears standing height and passes obstacle with finite physics; six numeric cases |
| P08-011 / 08 | User reports crossing cars not causing wipeouts | crossing actor contacts/classification | Explicit hazard tag, all-type/continuous contacts; moving/stationary car contacts wipe out in assisted/dynamic adapters; four physical cases |
| P11-011 / 11 | Human appearance changed on entering ragdoll | shared procedural segment definitions | Identical eleven mesh/material/size slots for seated/pedestrian/runner/ragdoll; four-state test, native actor/race captures |

Original Part 03 lean-clamp presentation target is superseded by the user's
explicit no-lean correction. Diagnostic lean demand/traction calculations remain.
The shared model is procedural graybox, not the future production animation rig.

## Handling / lean / camera correction after Part 04

Latest request: reference handling is the target; turn force strengthens while
held; bike and rider should bank together from input only, not road curvature;
chase views were too distant. See [source investigation](research/ROAD_RASH_HANDLING.md)
and [receipt](testing/HANDLING_CORRECTION.md). P04-015 is historical/superseded.
User confirmed they have no original files. P03-020 remains open/blocked on
verified authorized calibration and numeric parity; existing engine/brake/gears/
traction/nitro behavior is not misrepresented as a complete original clone.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P03-018 / 03 | User explicitly requests C++ handling investigation | exact source snapshot, rights check | Trace control/physics/motion and identify missing data; source-linked research and R01 update |
| P03-019 / 03 | Constant steering missed the reference's stored-force buildup | pure state, validated tuning, fixed input | Held force grows, release/reversal unwinds, speed dependence/no parked strafe; core and game tests; timings explicitly provisional |
| P03-021 / 03 | User asks for twice-fast full lean on both sides | authored steering response, visual bank | Halve rise/release timing in all classes; 16 Hz visual response; same force/angle caps; both signs attain full bank quickly and neutral stays upright on curves; core/vehicle tests |
| P03-022 / 03 | User asks for stronger turning at maximum lean | P03-019/021, handling profiles | Raise profile turn response by 25%; both adapters consume gain, full-left/right travel increases without larger visual angle; integration comparison and native smoke |
| P03-020 / 03 | Entire handling equivalence needs original SPEC/table inputs and replay comparison | authorized verified values, independent engine compatibility | Match throttle/brake/gear/nitro/steer/grip/motion across bikes/surfaces with recorded numeric tolerances; open, user has no files |
| P04-016 / 04 | User reverses no-lean feedback and forbids curve-driven lean | shared visuals, interpolated body pose | Bike/human same input bank, no roll from a banked road at zero input; actual presentation hierarchy test and native captures |
| P04-017 / 04 | Camera lag and overhead far view zoomed out too far | interpolated pose/velocity, collision sweep | Near/default/far distances <=3.93/5.20/6.68 m from focus in 27 scenarios, including slow smoothing and 80 m/s; camera tests/native review |

## Added requirements and rationale

| ID / part | Description and reason | Dependencies | Acceptance / verification |
| --- | --- | --- | --- |
| P01-012 / 01 | Frame-rate independence and reset soak; variable scheduling previously untested | fixed physics/input | Equal 120 ticks/poses over two seconds at 30/60/120 FPS; 100 crash/pause/resume/reset cycles, no actor leaks; automated game tests |
| P01-013 / 01 | Preserve button edges through zero/multiple fixed ticks | core intent buffer | A short press during a zero-tick render frame executes once at the next tick; core and game regression tests |
| P01-014 / 01 | Version legacy profiles without dropping credits or overwriting unknown data | core schema contract | Legacy roundtrip and malformed/future-version rejection/preservation; profile tests |
| P01-015 / 01 | Reproducible local panic reports and a headless content validator | logging/content validation | Validator success/exit-2 failure; intentional panic produces local backtrace and nonzero status; process probes |
| P01-016 / 01 | Prevent README/tracker checkbox drift | stable task IDs | Generated full index matches README; sync_progress.py --check in CI |
| P01-017 / 01 | Audit embedded font/render media beyond package SPDX declarations | source/license investigation | Runtime media hashes/notices checked; unverified LUT feature excluded; audit.py |
| P06-014 / 06 | Rider remains seated / artificial launch reported by user; separate collision-clear ragdoll and preserve measured momentum | existing crash bodies, physics queries | Initial rider/wreck velocities equal measured velocity, no injected Y; seated visuals absent from wreck; headless snapshot test + native crash capture |
| P06-015 / 06 | Root-pivot reach and runner impulse can prevent remount after a bike tumbles | bike OBB, runner controller | Reach all six representative poses from both sides within 8 seconds; dynamic native crash recovers, no leaked actors |
| P06-016 / 06 | Shoulder damage immediately after remount undermines recovery protection | existing immunity timer | End-to-end crash/recovery restores full health and immunity at handoff; gameplay test |
| P01-018 / 01 | Platform build is not a GPU/hardware acceptance receipt | CI and native installation | Record Linux adapter and real Windows/NVIDIA launch/input/audio receipts; hardware gates remain unchecked until run |

Future consequences already retained in the original backlog: P03 dynamic chassis
adapter, P04 gamepads/comfort and full rebinding UI, P05/P07 authoritative events,
P06 manual foot override, P07 foot finish/checkpoints, P10/P13 headless server and
transport, P12 career save migration/backups, P15 rendering/audio budgets, P16
hardware/clean-install/performance/release audit. No requirement was removed to
close this session.

## First-map procedural base-world foundation — 2026-10-30

Completion follow-up: BW-B00–B10 and BW-X01–X03 verified. Ubuntu/Windows CI
run 41166322394 passed implementation f245f2b; native 1080p survey p95 10.28 ms.
See [complete receipt](world/COMPLETION_TESTS.md). No future major part started.

User supplied a structural world PDF, JSON and section CSV and requested starting
first-map procedural generation. [Review, contracts and BW-B00–B10 dependency/
acceptance register](world/BASE_WORLD.md); [verification](world/BASE_WORLD_TESTS.md).
B00–B02 established the foundation; the user then explicitly requested completion.
B03–B10 structural construction is documented in [the implementation](world/BASE_WORLD_COMPLETE.md)
and [completion receipt](world/COMPLETION_TESTS.md). Current checkboxes below
reflect verified gates. The route/seed/gameplay are retained while canonical old
buildings are replaced by exact 247-root geometry. Source PDF handoff
language is design guidance; active user scope controls this session.

## City streetscape follow-up — 2026-10-31

CITY-03 adds flat concrete sidewalks and paving joints, original collidable curbside
trees, and an authored 1.6 urban floor multiplier. Canonical quotas/IDs and lot
footprints remain intact. See [verification](testing/STREETSCAPE.md) and ADR-024.
Original source height tables remain historical inputs, superseded at runtime by
the user's taller-city request.

## Rider, steering and pursuit follow-up — 2026-10-31

COMBAT-02, CITY-04, RIDE-02 and AI-01 address the user's combined corrections.
118 automated tests and native crash/remount/finish smoke pass. Fifteen-racer
city survey: 11.29 ms p95 at 1080p on this host. Reference parity, production
art and human difficulty acceptance remain open. [Evidence](testing/RIDERS.md).

## Individual rival personalities — 2026-10-31

AI-02 completes validated level-zero profiles, selective relationship-driven
race/follow/fight decisions, individual hit responses/stats and a supported rear
grid with 12 m gaps. 120 tests and final native inspection/survey/smoke pass.
Evidence: [PERSONALITIES.md](testing/PERSONALITIES.md). AI-03 retains full-career,
relationship-axis/event and random-stream parity gates.

<!-- GENERATED CHECKLIST INDEX -->

### Part 00 — Completed and verified foundations

- [x] **DRESS-02** Vary roadside scenery through biome-specific groups and independent seeded sides. Reason: repeated mirrored single-object placement; part: 05; dependencies: DRESS-01 and PROP-03; acceptance: at least28 actual roles and16 groups in all five zones, broad zone coverage, bounded224 owners, guarded road/access/support and unchanged tree counts; verification: deterministic/group validation tests, native mesh/role/zone census and farm/city/hill captures ([evidence](testing/roadside-variety/README.md)). Finished prop art and broad performance acceptance remain separate.

- [x] **COLLISION-07** Preserve forward motion through pedestrians, recovering riders and downed bikes. Reason: post-solver stopped velocity erased speed; part: 01; dependencies: COLLISION-04 and entry motion snapshots; acceptance: attacker remains mounted, soft target falls, downed-bike lift spends remaining linear kinetic energy, no backward response; verification: three-preset and pre-sync contact regressions plus native recovery smoke ([evidence](testing/part05-world/README.md)).

- [x] **LOD-01** Transition ready authored facade groups through the detailed fallback over 0.30s. Reason: abrupt scene replacement remained visible; part: 04; dependencies: KIT-05; acceptance: native shader dither, no blank replacement, one displayed group/lot, actual display and hidden budgets enforced; verification: opacity tests and native 15-section displayed-mesh census ([evidence](testing/part05-world/README.md)). Full silhouette morphing and long streaming QA stay open.

- [x] **DRESS-01** Place existing prop models in guarded Valley sidewalk, field-edge, rural and hill sockets. Reason: the restored map lacked semantic dressing; part: 05; dependencies: PROP-03 and structural layout; acceptance: deterministic IDs, whole-footprint road/access/building guards, solid supported owners and bounded counts; verification: placement tests and native 131-owner/376-mesh census ([evidence](testing/part05-world/README.md)). Full prop/category art and behavior gates remain open.

- [x] **MAT-01** Add shared original palette micro-normal/roughness maps, missing tangents and repeated concrete paving. Reason: flat surfaces and missing glTF tangents; part: 05; dependencies: KIT-02 and verified original palettes; acceptance:34 exact source profiles, bounded asynchronous derivation, shared maps, original colors/positions/skinning preserved; verification: all 34 source/decode hashes, PBR/tangent tests and native 934-material census ([evidence](testing/part05-world/README.md)). Bespoke albedo painting stays open.

- [x] **SKY-01** Add an original seamless cloud sky, daylight environment, warm sun, exposure and fog. Reason: user explicitly requested sky/cloud/lighting before the full landscape pass; part: 06; dependencies: current native renderer; acceptance: bounded typed settings, cube mips/seams, rendered drifting clouds and native recovery smoke; verification: cube/config tests and native screenshots ([evidence](testing/part05-world/README.md)). Volumetric weather and full landscape art are not included.

- [x] **KIT-04** Improve high-rise/infill and road surface detail. Reason: fallback towers/infill were visibly plain and roads were flat gray; part: 04; dependencies: KIT-02 and immutable structural geometry; acceptance: measured-scale batched window reveals/mullions/floor bands/cornices, owner-family infill masonry and original mipmapped asphalt albedo/normal/roughness across main/side roads without collider changes; verification: [relief/PBR tests, native street views and unchanged support/recovery](testing/world-polish/README.md). Full art approval and bespoke roles remain open.

- [x] **KIT-05** Keep facades visible while replacement LODs load and instantiate. Reason: asset/cache changes and early scene swaps caused missing-detail frames; part: 04; dependencies: KIT-02 and native scene intake; acceptance: finite coarse/masonry residency, bounded high-LOD cache and hidden preparation, old scene retained until every replacement instance is ready, spatial hysteresis and displayed-budget assertions; verification: [native instantiated-mesh/preparation census and full-route survey](testing/world-polish/README.md). Perfect geometry crossfades and production streaming stress remain open.

- [x] **PERF-06** Abstract unseen rear NPC crashes to a five-second delay. Reason: the user requests skipping ragdoll simulation for racers behind them; part: 01 performance/contact; dependencies: current crash entry, fixed simulation and reset; acceptance: distant rear/out-of-view mounted NPCs allocate no ragdoll/wreck, wait exactly300 active ticks and return at road centre at the recorded crash station, pause/restart work, player/visible/already-physical crashes unchanged; verification: [eligibility/no-body/timer/pause/return/reset tests and native recovery regression](testing/world-polish/README.md).

- [x] **ANIM-12** Resolve and verify default-stack asset loading. Reproduced the IO worker overflow, configured only asset IO workers with16MiB stacks while retaining Bevy's1–4 thread policy, and verified native Valley and City Run startup/crash/recovery/finish without RUST_MIN_STACK. [Before trace, design and native receipts](testing/world-polish/README.md). Broader platforms/loader memory optimization remain open; no upstream parser stack reduction is claimed.

- [x] **KIT-01** Compile and validate reusable authored facade storeys. Reason: Valley needs the existing modeled assets with real repeated floors rather than whole-building vertical stretching; part: 04; dependencies: original 100 facade/28 structure drafts and source provenance; acceptance: 128 designs, three ground/upper/crown GLB LODs per ID, shared own-textures, unchanged source hashes and independent binary geometry validation; verification: [384 GLBs, 34 shared textures, source/export hashes and band bounds](testing/architecture-valley/README.md). Final art approval remains MESH/V2-04 work.

- [x] **KIT-02** Integrate bounded authored geometry on every existing Valley building lot. Reason: the default Valley still used generic shells despite the modeled City Run assets; part: 04; dependencies: KIT-01 and existing supported lot generator; acceptance: all 657 existing lot IDs mapped through 19 role pools, deterministic in-envelope storey assembly, bounded pending/active scenes and cache, actual native GLB geometry with no added colliders; verification: [all-lot tests, native mesh census, 15-section survey and recovery smoke](testing/architecture-valley/README.md). The measured frame-time target is unmet and remains V2-04-006; this task does not certify production performance.

- [x] **V2-04-005** Retain supported ground, roads, collision and navigation. Verified canonical quotas/IDs/rights of way and geometry fingerprint unchanged; visual kit remains independent of colliders and pad support, with existing route/walk/recovery tests and native smoke passing. [Evidence](testing/architecture-valley/README.md). New building interiors/navigation remain ENV-005.

- [x] **BASE-003** Record dependency versions, purposes, licenses, and notices in `docs/third_party/REPOSITORIES.md`; record media provenance in `docs/third_party/ASSETS.csv`.

- [x] **CAM-02** Prevent actor proximity and low ceilings from collapsing/flipping the chase camera. Reason: reported nearby-body/under-object camera jumps; originating part: 01; dependencies: static obstruction cast and recovery focus; acceptance: moving/owned/NPC bodies do not shorten the boom, low-ceiling flat/lower candidates retain a usable chase direction and degenerate look vectors keep orientation; verification: mounted/on-foot ceiling plus foreign-ragdoll tests at 60/144 Hz, wall regression and native race in [receipt](testing/recovery-polish/README.md).

- [x] **COLLISION-06** Replace upright rear-bike hops with momentum sharing and a 60 km/h closing-speed knockoff. Reason: latest user supersedes the earlier rear-hop request; originating part: 01; dependencies: contact classification and pre-solver velocity snapshots; acceptance: mass-weighted normal-speed exchange, no scripted rear lift, front victim knocked down at threshold, side/head-on/downed routing retained; verification: pair momentum/energy, below/above threshold, both entity orders and real-contact regressions in [receipt](testing/recovery-polish/README.md).

- [x] **ANIM-13** Rewrite the gait and correct downward elbows and neutral wrist alignment. Reason: user rejected walking, raised tuck elbows and sideways wrists; originating part: 03; dependencies: ANIM-11, RAG-09 and shared FK/IK; acceptance: continuous opposing foot/arm cycle, stable forward knee poles, downward held-arm poles and parent-relative neutral distal binds with straight hanging wrists; verification: cycle continuity/chain tests, standing/seated wrist-axis tests and native eleven-motion/three-view review in [receipt](testing/recovery-polish/README.md). Full production/user motion acceptance remains ANIM-09.

- [x] **RAG-10** Add a bounded grounded roll and brief frontal leg-release prototype. Reason: requested side rolling, forward somersault tendency and kick-back variation; originating part: 03; dependencies: accepted biomechanical rig and crash energy guard; acceptance: one-shot support/speed-gated angular cue paid from at most 8% of current horizontal COM energy, direction follows slide, frontal hip/knee targets use capped motors and no direct ejection impulse, baseline/limits preserved; verification: forward/side energy and one-shot tests, unsupported/baseline rejection, motor-only velocity parity and native comparison demo in [receipt](testing/recovery-polish/README.md). Contacts may interrupt rolls; final feel review remains RAG-05/ANIM-09.

- [x] **MAP-01** Restore the original Valley route as the launch default while retaining City Run. Reason: user wants the original mixed-environment map instead of the city test route; originating part: 01; dependencies: both existing track definitions and structural generators; acceptance: bare launch selects Valley, explicit valley/city aliases work without altering geometry/quotas/IDs; verification: default native farm/city/hill recovery/results smoke and retained city tests in [receipt](testing/recovery-polish/README.md).

- [x] **FOOT-001** Crash 5 m before the finish; manually run forward across it -> finish once with valid prior checkpoints. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-002** Crash 5 m before the finish; run away from the bike across the line -> manual control wins and foot finish succeeds. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-004** Hold left/right/back after a crash -> independent chosen movement immediately; release -> assist resumes only after the configured neutral delay. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-005** Walk deliberately away near the bike -> no forced remount or teleport; camera follows rider and HUD shows bike arrow/distance. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-006** Bike and rider cross in the same tick -> exactly one racer-level finish event and payout. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-007** Walk backward through finish or skip a checkpoint -> reject until ordered forward progress is valid. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **P06-007** Add immediate camera-relative 360-degree WASD/stick manual walking override, including running away from the bike; resume assist only after 0.25-0.5 s neutral input. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **ANIM-10** Correct resolved punch/kick side and draw elbows inward during tuck/lean while retaining grips. Reason: user-reported side inversion and flared elbows; originating part: 03; dependencies: ANIM-08; acceptance: both attacks select the matching gameplay side and both elbows move inward without stretched chains or grip release; verification: four-side selector, endpoint/length tests and native rear/front/side review in [Part 03 receipt](testing/animation-part03/README.md).

- [x] **ANIM-11** Integrate a visual terrain-gait and physical-landing prototype. Reason: time-only running and a generic get-up start ignored actual displacement/landing; originating part: 03; dependencies: ANIM-08 and physical pose snapshots; acceptance: traveled-distance stride, stop blend, support-normal sole IK, stance locks and captured-pose get-up blend stay renderer-only; verification: slope/chain tests, paired physical parity and native recovery in [Part 03 receipt](testing/animation-part03/README.md). Production terrain/clearance metrics remain V2-03-003/006.

- [x] **RAG-09** Drive the pilot skin with independent wrist/foot/spine joints. Reason: sixteen physical bodies were presented through eleven skin joints; originating part: 03; dependencies: RAG-07 and ART-03; acceptance: sixteen normalized skin influences, independently rotated distal skin joints and unchanged no-input body simulation; verification: skin-weight, distal articulation and paired 180-tick physics tests in [Part 03 receipt](testing/animation-part03/README.md). Prop NPC skins/fingers/final art approval remain RAG-05/ANIM-09.

- [x] **P00-001** Document the original-successor approach and exclude the reference port's code, original game data, sprites, and soundtrack. Evidence: introduction and License sections of this README.

- [x] **P00-002** Establish a top-level production backlog from the blueprint. Evidence: this progress tracker.

- [x] **P00-003** Write a GDD v0.1 under 12 pages with P0/P1/P2 priorities, player fantasy, session length, fun metrics, and performance targets.

- [x] **P00-004** Lock the slice definition: one road, one bike plus two variants, at least three AI, punches/kicks, traffic, crash-to-foot control, valid foot finish, results, and retry.

- [x] **P00-005** Create at least 20 confidence-rated reference observations with source URL, build/version, date, recording description, and independent implementation notes.

- [x] **P00-006** Build bike, weapon, rider/outfit/palette, track, shop, progression, and UI content sheets; distinguish historical observation from new design.

- [x] **P00-007** Maintain research fields `source_id`, `observed_behavior`, `confidence`, `asset_owner`, `allowed_usage`, and `follow_up`.

- [x] **P00-008** Complete a rights register covering code, art, sound, fonts, contributors, attribution, and redistribution; remove unknown-license runtime content.

- [x] **P00-009** **Exit gate:** original-content policy, content schema, backlog, and one-page slice definition exist, with the reference and rights inventories reviewed.

- [x] **V2-01-001** Restart at the authored grid, including on a zero-physics-tick frame.

- [x] **V2-01-002** Give AI proportionate, gradual steering and lean.

- [x] **V2-01-003** Use pre-contact world momentum for physical knockoffs.

- [x] **V2-01-004** Repair visual skeleton attachments and anatomical bending. [Verification for corrections 001–004](testing/MOTION_REPAIRS.md).

- [x] **CRASHREF-001** Independently implement original integer contact, damage, entry and scalar drag calculations. Reason: compare actual original crash rules; part: 01; dependencies: pinned R01 and retail facts; acceptance: explicit fixed-point rounding, mass/order, signed velocity, damage scaling, caller-specific boosts and clamps; verification: nine domain tests and [calculation report](research/ORIGINAL_CRASH_CALCULATIONS.md).

- [x] **CRASHREF-002** Verify recovered arithmetic against the local retail executable. Reason: reconstruction comments can differ from native fields; part: 01; dependencies: CRASHREF-001 and user's hashed EXE; acceptance: compare Rust outputs to bounded native x86 execution without distributing original bytes; verification: 23 passing [retail checks](testing/ORIGINAL_CRASHES.md), signed-field discrepancy recorded.

- [x] **CRASHREF-003** Integrate an opt-in bike/bike and bike/car crash comparison while preserving the passive rig. Reason: compare independently of biomechanical PR12; part: 01; dependencies: CRASHREF-001/002 and existing collision/recovery; acceptance: separate rider/wreck entry velocities, persistent comparison condition, explicit SI/geometry adaptation, default behavior retained, remount resets state; verification: adapter tests, native-window smoke and fixed-simulation contact-to-remount regression in [test receipt](testing/ORIGINAL_CRASHES.md).

- [x] **P11-001** Provide placeholder road, trees, mountains, ocean, bikes/riders/cars, solid scenery proxies, and full-footprint road-clearance validation. Evidence: [world](../src/world.rs), [track](../src/track.rs).

- [x] **P11-011** Use one human mesh/proportion/outfit definition for seated bikers, pedestrians, recovery runners and ragdolls; change pose without swapping appearance. Evidence: [shared model and four-state test](../src/human.rs); the new authored skins use [four-state/physics parity tests](../src/models/tests.rs) and [native receipts](testing/MODELS.md).

- [x] **P12-001** Award and persist credits, wins, race count, and a best time; write profile updates through a temporary file and atomic rename in the user data directory. Evidence: [profile](../src/game.rs).

- [x] **P12-002** Test that a finished race pays once across subsequent updates. Evidence: `finish_awards_results_once_and_button_starts_next_race` in [game tests](../src/game.rs).

- [x] **P15-001** Provide baseline PBR lighting, shadows/fog, menus/HUD/results, engine/wind/nitro/braking loops, impacts/footsteps/countdown/finish cues, and mute. Evidence: [world](../src/world.rs), [UI](../src/ui.rs), [audio](../src/sound.rs).

- [x] **P15-002** Generate original PCM sounds in memory; verify valid non-silent WAV data and initialized/playing audio sinks in the native smoke check. Evidence: [sound test](../src/sound.rs), [render/audio check](../src/verification.rs).

- [x] **P16-001** Maintain functional regression tests for steering, circuit/projection, countdown/nitro/pause, combat, physical contacts, repeated knockdowns, camera tracking, full recovery, payouts, and scenery clearance. Evidence: tests in [game](../src/game.rs), [track](../src/track.rs), [world](../src/world.rs), [sound](../src/sound.rs).

- [x] **P16-002** Build and run a local Linux release; smoke-check garage -> riding -> crash -> get up -> run -> remount -> results with active audio. Evidence: [verification script](../src/verification.rs).

- [x] **AUDIO-014** Exclude proprietary original Road Rash music, voices, and sound files from the current build; current effects are synthesized in [sound](../src/sound.rs).

- [x] **CITY-01** Add side-road grids and denser urban blocks; compare road/sidewalk/frontage proportions against the verified original disc. Evidence: [city/combat verification](testing/CITY_COMBAT.md), 657 solid buildings, native city captures and 9.56 ms p95 survey.

- [x] **COMBAT-01** Space auto-side kick pushes bikes and animates the selected leg; mouse buttons punch left/right. Evidence: nearest-side/push/input tests and native left/right kick captures in [verification](testing/CITY_COMBAT.md).

- [x] **RIDE-01** Correct rival camera obstruction and supported contact-yaw instability; add symmetric input-only lean presentation. Evidence: crowd-camera/yaw tests, seven native bank captures and crash/recovery smoke in [verification](testing/CITY_COMBAT.md).

- [x] **CITY-02** Join neighboring city buildings into continuous street fronts. Reason: the user rejected detached six-metre gaps; part: world development; dependencies: CITY-01, solid lots and recovery routing; acceptance: shared-edge infill, clear streets, exposed-perimeter recovery paths and road-height screenshots; verification: 347 shared-edge sections, physics rays for every infill, 113 passing tests and native road-height survey; [evidence](testing/FRONTAGES.md).

- [x] **CITY-03** Improve sidewalks, plant solid street trees and raise urban buildings. Reason: user requested better pavement and a taller, greener city; part: world development; dependencies: CITY-01/02 and shared ground; acceptance: flat sidewalk shading, clear walking strip, solid trees outside junctions, taller whole-floor buildings without lot changes; verification: geometry/physics tests, native survey and crash/recovery smoke; [evidence](testing/STREETSCAPE.md).

- [x] **COMBAT-02** Strengthen kick displacement and adopt the fifteen-racer CITY roster with reference-inspired shirt, helmet and bike colors. Reason: user requested original roster/color schemes and stronger kicks; part: combat/world; dependencies: COMBAT-01 and shared human meshes; acceptance: 14 named rivals, consistent minimap/ragdoll colors and stronger signed physical shove; verification: combat/UI tests and native captures; [evidence](testing/RIDERS.md).

- [x] **CITY-04** Thin sidewalk trees from 22 to 44 m spacing. Reason: user found the avenue overplanted; part: world; dependencies: CITY-03; acceptance: roughly half the sites with existing clearances/collision; verification: placement tests and native survey.

- [x] **RIDE-02** Require player steering through curves, increase turn strength 50%, and topple abandoned bikes. Reason: user reported automatic driving and upright wrecks; part: Parts 03/04; dependencies: dynamic chassis and recovery; acceptance: neutral heading stays independent of road, stronger turn settings and physical roll without linear launch; verification: heading/roll/momentum/remount regressions and native smoke.

- [x] **AI-01** Add measured rival pursuit, speed matching and selective attacks on downed riders. Reason: user requested reference-inspired fighting/chaos; part: AI follow-up; dependencies: relative contacts and crash recovery; acceptance: start grace, bounded approach, alongside spacing, gradual speed matching, limited downed hunts and attack cooldowns; verification: deterministic pursuit scenarios, regression suite and native survey. Original relationship-table parity remains future work.

- [x] **AI-02** Match individual level-zero rival pace/stats and relationship-driven race/follow/fight choices; start YOU last with 12 m gaps. Reason: user rejected uniform aggression and crowded/front starts; part: AI follow-up/track setup; dependencies: AI-01 and supported dynamic grid; acceptance: distinct validated profiles, selective hostility and hit reactions, per-rider pace/strength/stamina/weight, solid separated rear grid and retry; verification: profile/decision/physical-grid tests plus native smoke/survey; [evidence](testing/PERSONALITIES.md).

- [x] **ART-01** Research rigged character sources matching the user's faceted, natural-proportion references and biker gear. Reason: user requested a visual upgrade; part: character art; dependencies: reference images and shared human lifecycle; acceptance: sourced shortlist, format/rig claims and license review; verification: publisher/marketplace pages reviewed 2026-10-31; [findings](third_party/CHARACTER_CANDIDATES.md).

- [x] **ART-03** Integrate the approved authored rider and motorcycle as presentation-only replacements, retaining all accepted ragdoll bodies/joints and crash/recovery behavior. Reason: explicit user request; part: 11 art; dependencies: authored GLBs/seat pose and existing visual joints; acceptance: shared rider through riding/walking/ragdoll, bike/wreck replacement, unchanged physics trajectories, complete recovery, native visual check; verification: [tests](../src/models/tests.rs), [evidence](testing/MODELS.md).

- [x] **P01-001** Pin a Rust toolchain and dependencies; provide formatting, Clippy with warnings denied, and Linux headless tests in CI. Evidence: [toolchain](../rust-toolchain.toml), [manifest](../Cargo.toml), [CI](../.github/workflows/ci.yml).

- [x] **P01-002** Launch a native Fedora/Linux 3D build with ground, lighting, camera, keyboard input, menus, pause, and reset. Evidence: [app](../src/main.rs), [render smoke check](../src/verification.rs).

- [x] **P01-005** Implement Boot/MainMenu/Loading/Race/Results lifecycle and fixed 60 Hz gameplay/physics scheduling, with independent render interpolation.

- [x] **P01-008** Add FPS, average/P95 frame time, 60 Hz tick count, entity/collider counts, physics timing, toggleable debug UI, logs, and panic reports.

- [x] **P01-012** Verify 30/60/120 render-FPS simulation equality and 100 crash/pause/resume/reset cycles without actor leaks. Evidence: [game tests](../src/game.rs).

- [x] **P01-013** Preserve and consume short attack inputs once across zero/multiple fixed ticks. Evidence: [core buffer](../crates/rr_core/src/lib.rs), [input tests](../src/game.rs).

- [x] **P01-014** Version prototype profiles while retaining legacy progress and rejecting unsupported/corrupt data. Evidence: [profile tests](../src/game.rs).

- [x] **P01-015** Provide a headless content-validation CLI and local panic report process probe. Evidence: [entry point](../src/main.rs), [test receipts](testing/PARTS_00_01.md).

- [x] **P01-016** Keep the complete README/tracker task index synchronized by stable IDs. Evidence: [sync checker](../tools/sync_progress.py).

- [x] **P01-017** Audit declared dependency/media provenance, font notices, source hashes and excluded unverified embedded LUTs. Evidence: [audit](../tools/audit.py), [inventory](third_party/ASSETS.csv).

- [x] **P01-019** Decode Cargo metadata and repository text explicitly as UTF-8 across host locales; regress the Windows cp1252 audit failure. Evidence: [audit tests](../tools/test_audit.py), [CI follow-up](testing/PART_02.md).

- [x] **P02-001** Build a closed driveable circuit with lanes, shoulders, solid road surfaces, and canonical arc-length progress. Evidence: [track](../src/track.rs), [world](../src/world.rs).

- [x] **P02-002** Validate circuit closure, distance wrapping, camera-right lane direction, progress projection, and full-footprint scenery clearance in headless tests. Evidence: tests in [track](../src/track.rs) and [world](../src/world.rs).

- [x] **P02-003** Document meter units, +Y up, spline-tangent forward, origin, and any future origin-rebasing policy.

- [x] **P02-005** Define typed track IDs, control points, surfaces, ordered checkpoint IDs, prop placement, spawn points, and safe foot/verge regions.

- [x] **P02-006** Build simplified collision meshes separately from render meshes, initially sampling around 2-3 m; add curbs, guardrails, fencing, and reliable highway boundaries.

- [x] **P02-007** Implement ordered gates and forward crossing validation; validate checkpoint order, intersections, NaNs, negative widths, and spawn overlap.

- [x] **P02-008** Add asphalt/dirt/gravel/grass/water metadata and traction/friction modifiers, animal crossings, and cliff hazard zones.

- [x] **P02-009** Add a track inspector for spline normals, triangle count, driving line, bounds, and spawns; load authored tracks headlessly.

- [x] **P02-012** Build the user-directed point-to-point map with distinct start/finish, no endpoint wrapping, canonical route HUD and safe endpoint/retry behavior. Evidence: [track](../src/track.rs), [race tests](../src/game.rs).

- [x] **P02-013** Populate seeded farm openings/fences/cows, city buildings/cross traffic/walking pedestrians, and elevated hills with physical guardrails. Evidence: [environment](../src/environment.rs), [native captures](testing/PART_02.md).

- [x] **P02-014** Keep terrain below banked road lanes and verify route support, render/collision winding and independent collision resolution. Evidence: [geometry tests](../src/environment.rs).

- [x] **P02-015** Verify actual hill crash/get-up/run/remount at elevation without resetting the bike to sea level. Evidence: [recovery](../src/recovery.rs), [native receipt](testing/PART_02.md).

- [x] **P02-016** Keep hill guardrails on outer curves, open the straights, and add varied trees in all zones plus window/door/roof, cross-vehicle and pedestrian detail. Evidence: [environment](../src/environment.rs), [native captures](testing/PART_03.md).

- [x] **BW-B00** Capture existing track/world statistics, measured legacy building census, reproducibility and native baseline views.

- [x] **BW-B01** Validate 15 sections, 19 archetypes, lot rules and exact 247/177/70/family budgets; allocate stable seeded planning IDs headlessly.

- [x] **BW-B02** Bind districts to baked road stations, resolve/blend all boundaries and final endpoint, reject mismatched courses and expose F4 preview.

- [x] **BW-B03** Generate field parcels and farm/suburban/city/hill ground classification across all sections.

- [x] **BW-B04** Build at least seven reserved junctions, side-street foundations and continuous accessible curbs/sidewalks.

- [x] **BW-B05** Generate non-overlapping, reachable frontage/secondary lots outside every road/junction/gate/spawn reservation.

- [x] **BW-B06** Place exactly 247 structural building roots with mass/roof/collision and validated full-footprint clearance.

- [x] **BW-B07** Grade supported pads and hillside foundations with shared render/collision heights and local cut/fill.

- [x] **BW-B08** Prove farm/city-alley/hill on-foot recovery and escape paths without geometry traps.

- [x] **BW-B09** Implement spatial building LOD/chunks while preserving collision/IDs and measure representative 1080p performance.

- [x] **BW-B10** Complete the full-map survey, plan export, count/geometry hashes, provenance, regressions and all base-world exit criteria.

- [x] **BW-X01** Protect paired LOD origins and local terrain vertices against disappearing ground; verify all 160 tile pairs and building children plus the native atlas.

- [x] **BW-X02** Record actual survey resolution/focus and use continuous uncapped benchmark updates so background idle timers cannot masquerade as frame cost.

- [x] **BW-X03** Clear recovery motion/routes on exceptional rescue and apply one visible five-second penalty; verify normal capsule routes and rescue state transitions.

- [x] **P03-001** M0: reach the finish with a kinematic road-following arcade controller, throttle, braking, steering/lean visuals, and nitro. Evidence: [racing controller](../src/game.rs).

- [x] **P03-002** Integrate collision-driven knockoff, continuous movement checks against solid geometry, and impact launch/landing/bounce behavior. Evidence: [controller](../src/game.rs), [impacts](../src/recovery.rs), steering/collision/bounce tests. Rider-to-rider contact is now severity-based (P06-019); historical universal hops are superseded.

- [x] **P03-003** M1: implement a dynamic chassis with mass, center of mass/inertia, upright assist, and separate rider/wheel visuals behind `VehiclePhysicsAdapter`.

- [x] **P03-004** M2: add front/rear wheel contact rays, spring-damper suspension forces, engine braking, service braking, and a torque/RPM/redline/gear model.

- [x] **P03-005** M3: implement surface-dependent combined longitudinal/lateral traction budget, slip, controlled drift, speed-sensitive steering, aerodynamic drag proportional to speed squared, and low-speed self-righting.

- [x] **P03-006** Define a simulation-facing `BikeState` with pose/velocity, yaw/lean, slip, grounded wheels, actual RPM/gear, damage, and rider condition references.

- [x] **P03-007** Author three data-driven handling presets: light/agile, balanced, and heavy/powerful; initially target 150-290 kg dry plus rider, 0-100 km/h in 3.5-6.5 s, and a roughly 50-60 degree lean clamp.

- [x] **P03-008** Expose live tuning and per-tick telemetry: bike ID, tick/dt, pose, velocity, inputs, yaw rate, lean, wheel contacts, spring forces, slip, surface mu, damage, and crash state.

- [x] **P03-009** Benchmark acceleration, braking distance, steady turns, lean recovery, and curb hits; compare numeric results at 30/60/120 render FPS and collect 100 Hz traces where needed.

- [x] **P03-012** Decelerate smoothly when boost is released or exhausted; preserve excess speed instead of clamping to the ordinary top speed. Evidence: [regression](../src/game.rs).

- [x] **P03-013** Allow player/AI forward movement in grass, with reduced traction/speed and no grass-only health loss or knockoff; AI can return to the road. Evidence: [four-scenario regression](../src/game.rs).

- [x] **P03-014** Tighten lateral handling approximately 20% using validated authored steering rates, retaining older schema-1 catalogs with defaults. Evidence: [tuning](../content/catalog.json), [schema tests](../crates/rr_content/src/lib.rs).

- [x] **P03-015** Reduce road assistance without pulling the rider to lane zero; expose F6 adjustment and preserve free lane choice. Evidence: [stationary-offset regression](../src/vehicle_tests.rs).

- [x] **P03-016** Feed AI and human tick intentions through the same dynamic adapter, retaining AI traffic avoidance and launch grace. Evidence: [controller](../src/game.rs), [dynamic grid regression](../src/vehicle_tests.rs).

- [x] **P03-018** Investigate the C++ reference's steering, drive/braking, traction, gear, nitro and motion paths at an exact revision, separating source facts from missing calibration data. Evidence: [investigation](research/ROAD_RASH_HANDLING.md).

- [x] **P03-019** Accumulate player steering force while held, unwind on release/reversal, scale the target with speed, and prevent parked sideways driving. Evidence: [pure rules](../crates/rr_core/src/steering.rs), [integration tests](../src/vehicle_tests.rs). Timing values remain provisional RoadDash tuning.

- [x] **P03-021** Double full-lean response left/right by halving steering buildup/return times and doubling visual bank response, retaining angle/force limits. Evidence: [symmetric rate test](../crates/rr_core/src/steering.rs), [full lean/neutral test](../src/vehicle_tests.rs).

- [x] **P03-022** Increase full-lean turn strength by 25% in each handling class, with shared profile gain in both driving adapters and unchanged visual bank. Evidence: [left/right comparison](../src/vehicle_tests.rs), [verification receipt](testing/HANDLING_CORRECTION.md).

- [x] **P03-017** Preserve selected dry bike mass in abandoned wrecks and clear stale velocity on dynamic remount. Evidence: [mass/recovery regressions](../src/vehicle_tests.rs).

- [x] **P04-001** Provide smoothed follow/look-ahead, near/default/far chase views, speed-sensitive FOV, impact shake, and scenery collision avoidance. Evidence: [camera](../src/game.rs).

- [x] **P04-002** Provide keyboard throttle/brake/steering, pause/reset, and correct left/right steering in chase-camera space. Evidence: [input and regression tests](../src/game.rs).

- [x] **P04-003** Add speed-linked engine pitch and wind feedback. Evidence: [sound](../src/sound.rs).

- [x] **P04-005** Add gamepad stick steering/freelook, analog throttle/brake triggers, attack bindings, deadzones, and key/controller rebinding.

- [x] **P04-006** Provide persistent FOV/smoothing/anti-nausea settings; target configurable 60-95 degrees, about 75 degrees default, with optional reduced shake.

- [x] **P04-007** Add readable slip/traction feedback and optional speed lines; allocate the independent on-foot input context.

- [x] **P04-011** Verify camera continuity at 60/120/144 Hz, swept-volume obstacle clearance and safe large-warp snapping using automated scenarios. Evidence: [camera tests](../src/camera_tests.rs).

- [x] **P04-012** Preserve malformed/future settings files, reject conflicting bindings, and prevent modal settings input leaking into racing. Evidence: [settings](../src/settings.rs), [input tests](../src/controls.rs).

- [x] **P04-013** Carry analog triggers into chassis forces and clear input on controller disconnect while retaining keyboard fallback. Evidence: [adapter test](../src/vehicle_tests.rs), [device tests](../src/controls.rs).

- [x] **P04-015** Historical correction: keep the character upright during steering. This was verified in Part 04, then superseded by the user’s coordinated, input-only bike/rider lean request in P04-016. [Historical receipt](testing/PART_04.md).

- [x] **P04-016** Bank bike and rider together from accumulated steering input only; remove road/chassis bank from rendered roll and preserve the shared human model. Evidence: [bank/road-neutral test](../src/human.rs).

- [x] **P04-017** Keep near/default/far camera views close at high speed, eliminate follow-lag zoom-out, and reduce optional speed FOV to three degrees. Evidence: [27 camera scenarios](../src/camera_tests.rs), [native receipt](testing/HANDLING_CORRECTION.md).

- [x] **P04-010** Color only racer minimap dots by their shirt palette, sharing the model color source. Evidence: [palette test](../src/ui.rs), [shirt colors](../src/world.rs).

- [x] **P05-001** Implement left/right punches, directional range checks, damage, cooldown, simple arm animation, hit sound, and HUD feedback. Evidence: [combat](../src/combat.rs), [animation](../src/world.rs), [sound](../src/sound.rs).

- [x] **P05-002** Test directional damage and a single takedown reward. Evidence: `directional_melee_damages_the_target_and_awards_one_takedown` in [game tests](../src/game.rs).

- [x] **P05-009** Require three direct side-by-side hits with no more than three seconds between hits, with reach/side/height/heading/occlusion checks, one target per attack and duplicate-hit protection. Evidence: [core rules](../crates/rr_core/src/lib.rs), [melee](../src/combat.rs), [regressions](../src/game.rs).

- [x] **P06-001** Trigger physical knockoff from attacks, vehicle contacts, and solid roadside obstacles; maintain rider health and crash state. Evidence: [impacts](../src/recovery.rs), [controller](../src/game.rs).

- [x] **P06-002** Use separate owner-linked ragdoll, abandoned bike, and on-foot actors; implement falling -> getting up -> assisted run -> right bike -> remount. Evidence: [recovery](../src/recovery.rs).

- [x] **P06-003** Provide an 11-body jointed ragdoll, CCD, friction/restitution, collision filtering for the hidden riding body, and a solid on-foot capsule with step/slope handling. Evidence: [recovery](../src/recovery.rs).

- [x] **P06-004** Follow the physical torso and on-foot rider with the camera; allow traffic/rivals to knock a recovering rider down again without duplicate ragdolls. Evidence: [camera](../src/game.rs), recovery-camera and repeat-knockdown tests.

- [x] **P06-005** Provide impact/footstep cues and verify the rendered crash/get-up/run/remount sequence. Evidence: [sound](../src/sound.rs), [render check](../src/verification.rs).

- [x] **P06-014** Preserve current crash momentum without synthetic upward launch and visibly separate the rider from the riderless wreck. Evidence: [crash tests](../src/game.rs), [recovery](../src/recovery.rs), [native receipt](testing/PARTS_00_01.md).

- [x] **P06-015** Remount against the actual fallen bike collision shape, including sideways, upside-down and vertical poses, without the runner pushing it away. Evidence: twelve automated approaches in [game tests](../src/game.rs) and native recovery smoke.

- [x] **P06-016** Apply post-remount protection to shoulder damage so recovery does not immediately drain health. Evidence: end-to-end recovery regression in [game tests](../src/game.rs). Historical behavior: P03-013 now removes grass-only damage entirely.

- [x] **P06-017** Replace universal ball-joint limits with anatomical stops, directional hinges and bounded passive tissue resistance; verify 100 high-speed drops without detached joints or twisted hinge axes. Evidence: [articulation](../src/recovery.rs), [stress test](../src/game.rs).

- [x] **P06-018** Make fallen bikes slide on steep solid terrain while retaining flat-road stopping friction. Evidence: [slope contact friction](../src/recovery.rs), [ramp regression](../src/game.rs).

- [x] **P06-019** Classify motorcycle wipeouts by relative closing speed: >=50 km/h longitudinal or substantial side impact; lighter contacts nudge without universal launch/knockoff. Evidence: [severity rules](../crates/rr_core/src/lib.rs), [contact tests](../src/game.rs).

- [x] **P06-020** Add distinct pedestrian/cow ragdolls with direct-hit and solid-impact paths, an eight-actor budget, safe delayed standing and complete reset cleanup. Evidence: [NPC systems/tests](../src/npc.rs), [native actor capture](testing/PART_03.md).

- [x] **P06-022** Launch a mounted biker above cows/pedestrians using obstacle-height and mass-aware clearance impulses; preserve horizontal approach momentum and civilian ragdolls. Evidence: [six clearance cases](../src/vehicle_tests.rs), [measurements](testing/part-04/hazard-clearance.json).

- [x] **P06-021** Treat sidewalks as rideable support so low curb contact raises the bike instead of causing an obstacle wipeout. Evidence: [suspension](../src/vehicle.rs), [curb-transition test](../src/vehicle_tests.rs).

- [x] **P07-001** Implement countdown, laps/progress standings, race clock, speed/armor/nitro readouts, minimap, results/payouts, and restart. Evidence: [race](../src/game.rs), [HUD](../src/ui.rs).

- [x] **P07-002** Persist basic race count, wins, credits, and personal-best time; test one-time finish rewards and restarting. Evidence: [profile and finish tests](../src/game.rs).

- [x] **P08-001** Populate seven rival racers with curve-speed planning, lane changes, traffic avoidance, attacks, and recovery through the shared recovery system. Evidence: [AI controller](../src/game.rs), [recovery](../src/recovery.rs).

- [x] **P08-002** Populate two-way civilian traffic with solid vehicle colliders. Evidence: [traffic setup/update](../src/game.rs).

- [x] **P08-011** Treat crossing traffic as an explicit wipeout hazard with continuous/all-type contacts, including a moving car striking a stopped bike. Evidence: [four physical collision cases](../src/vehicle_tests.rs).

- [x] **P08-010** Use American right-hand traffic lanes and separate one-way cross-traffic lanes; stop automatic AI fights/weaving during grid launch. Evidence: [traffic](../src/game.rs), [cross streets](../src/environment.rs), [grid tests](../src/vehicle_tests.rs).

- [x] **COLLISION-01** Keep mounted heading input-owned through contacts. Reason: contact spin steers the bike; part: 01; dependencies: dynamic chassis and fixed input; acceptance: neutral input cannot acquire collision yaw, gradual intended steering and recovery still work; verification: repeated spin at 30/60/120 Hz, handling/camera regressions and native smoke.

- [x] **COLLISION-02** Remove scripted collision launches and spawn spin. Reason: latest request supersedes civilian clearance jumps; part: 01; dependencies: separated rider/wreck and civilian knockdowns; acceptance: zero added upward/spin velocity at entry, gravity-led wreck toppling, civilian momentum sharing and working recovery; verification: six civilian/bike-class cases, entry snapshots, slope/toppling tests and native actor/race smoke.

- [x] **COLLISION-03** Apply measured arcade vehicle speed loss and a dissipative crash envelope. Reason: player/AI crashes reverse or gain energy; part: 01; dependencies: pre-contact samples and physical body mass/inertia; acceptance: 120 km/h rider versus either direction of 40 km/h car seeds 80 km/h forward, never negative retained speed, no added aggregate energy beyond gravity or reverse crash centre-of-mass motion; verification: player/AI directional cases, kinetic/potential/rotation budget checks, cross-traffic contacts and native recovery.

- [x] **COLLISION-04** Separate fallen rider/wreck ride-over contacts from mounted wipeouts. Reason: any detached limb or bike triggered another crash; part: 01; dependencies: separated bodies and contact events; acceptance: solid downed contacts do not script another wipeout, low shapes support passage, mounted glances nudge, standing runners and car crashes remain functional; verification: player/AI and own/foreign body matrix in both crash modes, physical wreck traversal, full regressions and native crash/recovery smoke. [Evidence](testing/contact-routing/README.md).

- [x] **PROP-01** Deliver exhaustive scoped world-prop reference extraction and review infrastructure. Reason: the building workflow must extend to all roadside props and actors without hidden exclusions; originating part: 05; dependencies: local source resources and building catalog; acceptance: every indexed CEL in all families, five vehicle libraries and global CELs is accounted for, stable IDs/aliases/views retained, labeled 3×2 sheets/searchable gallery and explicit rebuild stages; verification: [4,172 source CELs, 1,310 images, 597 review banks, 105 sheets, independent coverage/image/zip validation and browser checks](testing/prop-catalog/README.md). This verifies extraction, not completed prop models or production art.

- [x] **PROP-02** Deliver source-mapped draft volumetric prop exports and an isolated GLB inspector. Reason: move the full bitmap catalog into editable 3D authoring; originating part: 05; dependencies: PROP-01 and the 369-ID concept queue; acceptance: all IDs have actual volumes, three reduced geometry LODs, original self-contained materials, editable category sources, independently re-imported render proofs and a working native viewer; verification: [binary/LOD/material/ID checks, authoring regressions and native smoke](testing/prop-meshes/README.md). This closes draft export infrastructure only; V2-05-002 retains concept fidelity, finished art, rigs, collider/placement and performance gates.

- [x] **PROP-03** Apply selected draft props in the playable maps. Reason: connect the 3D catalog to gameplay; originating part: 05; dependencies: PROP-02, existing traffic/NPC/physics and city reservations; acceptance: working moving/cross traffic, trees/lamps, clear sidewalk/parallel-street sockets, consistent human/cow visuals through knockdown, independent preserved body definitions and momentum, validated LOD skins, native survey and recovery smoke; verification: [integration tests, actual-game screenshots and selected-ID receipts](testing/props-in-game/README.md). This verifies development integration, not all 369 banks placed or final art/performance/release approval.

- [x] **V2-02-004** Investigate animation, IK and active-ragdoll repositories. Evidence: [five pinned source comparisons, licenses and compatibility](research/ANIMATION_AND_RAGDOLL.md); no library or source imported.

- [x] **ANIM-01** Deliver the first reviewable static pose catalog and authoring validator. Reason: Animation A needs reproducible review assets before playback; originating part: 02; dependencies: existing authored rider/bike/contact rig; acceptance: 247 numbered draft candidates across 27 use cases, offline gallery, editable single-frame pose assets and independent connected-rig/contact validation; verification: [140 Rust / 14 Python tests, all-pose Blender round trip and gallery review](testing/ANIMATION_A.md). This completes the review infrastructure, not pose approval or final animation coverage. V1 semantic labels were rejected and are superseded by the context-first trace below.

- [x] **ANIM-05** Recover source-ordered action/phase/view context before reconstruction. Reason: V1 misclassified inspection-helper slots; originating part: 02; dependencies: supplied retail build and pinned reference selectors; acceptance: all 104 bank/code slots, 194 ordered descriptors and 371 sprite-slot usages include semantic IDs, mirror/view variants, controls, timing counters and explicit fallbacks/unknowns; verification: independent decoder versus separately compiled reference (194 descriptor/rear matches), reproducible metadata and semantic/bounds tests. [Corrected catalog and limits](animation/context/README.md).

- [x] **ANIM-07** Correct torso and support motion in the source-linked mounted action keys. Reason: limb-only poses missed the original silhouettes; originating part: 02; dependencies: ANIM-05 and shared authoring contacts; acceptance: punch lean/target look, raised kick hips/released seat, full-body tuck and mirrored phases, with retained fixed-length support contacts; verification: 138 static Action reloads, 20 Python tests and regenerated labeled review renders. [Evidence and art limits](testing/ANIMATION_B.md).

- [x] **ANIM-08** Integrate the first full-body normal/tuck/turn/combat/recovery clip set. Reason: static keys need continuous presentation; originating part: 03; dependencies: ANIM-07 and existing visual skin adapter; acceptance: 36 keys / 13 clips, current-pose action entry/exit, steering-only turns, reference-informed tuck selection, held-contact constraints, pause/crash interruption and pickup space handoff without physical writes; verification: Rust sampler/state/contact/parity tests, nine authoring GIFs and native crash/remount/finish smoke. [Evidence and remaining production gates](testing/ANIMATION_B.md).

- [x] **RAG-01** Audit the current rig and five requested biomechanics references. Reason: the existing system already had limits and must be diagnosed before replacement; originating part: 03; dependencies: current source/lockfile and reference source access; acceptance: actual joint/CCD/mass/bind/control causes, five pinned source and license decisions, no unlicensed imports; verification: [investigation](research/RAGDOLL_BIOMECHANICS.md), repository register and ADR-036.

- [x] **RAG-02** Implement the 16-body anatomical articulation and passive resistance. Reason: wrists/ankles/split spine and bounded resistance were missing; originating part: 03; dependencies: RAG-01 and native Rapier; acceptance: calibrated local frames, elbow/knee hinges, bounded neck/spine/shoulders/hips/wrists/ankles, safe quaternion chart, 70 kg distribution/shape inertia, persistent passive motors and predictive collision; verification: native joint/torque tests and [nine dual-mode fixtures](testing/ragdoll/lab-results.json). Original eleven-body control remains selectable.

- [x] **RAG-03** Integrate capped muscles and coherent crash release. Reason: crash states lacked smooth loss of strength and animation/spin handoff; originating part: 03; dependencies: RAG-02 and animation bind frames; acceptance: configurable native PD pose tracking, bracing/ejection/tumble/rest curves, bind-calibrated initial pose with coincident anchors, COM/pre-impact spin preservation, unchanged ejection events, no forced upright motion and natural sleep; verification: pose-hold/external-force/state/bind/momentum tests, bounded energy and 15-rider settling results in [report](testing/RAGDOLL_BIOMECHANICS.md).

- [x] **RAG-04** Deliver the reproducible ragdoll comparison and tuning tools. Reason: the overhaul must be reviewable before merging; originating part: 03; dependencies: RAG-02/03; acceptance: central validated JSON, F7 shape/axis/limit/motor diagnostics, nine headless and native scenarios, B/reset baseline comparison, native clips and recovery/actor regressions; [automatic launch selection](testing/ragdoll/default-launch/README.md) with explicit baseline override (2026-11-02); verification: 164 Rust tests, 20 tooling tests, native comparison/race/actor smoke, build/Clippy/format/audit [receipts](testing/ragdoll/receipt.json).

- [x] **RAG-07** Integrate the approved biomechanical rig with current City Run, props and contact routing. Reason: comparison branch predates the live race; part: 03; dependencies: RAG-04, CITYGEN-01, PROP-03, COLLISION-04; acceptance: automatic new mode, explicit original baseline, current human/cow skins and recovery, original-crash direction policy retained; verification: [200 Rust tests, native paired race/actor/view and 18 lab runs](testing/ragdoll/integration/README.md). User authorized merge on2026-11-06; distal art and broad performance gates remain open.

- [x] **RAG-08** Strengthen passive hip/knee resistance and constrain inward leg crossing. Reason: user reported crossing knees and loose legs; part: 03; dependencies: RAG-02/07; acceptance: mirrored hip adduction/twist limits, persistent stronger leg motors, knees remain separated under paired inward torque, improved equal-load deflection and bounded high-speed settling; verification: [physical knee-load/separation tests and native settled capture](testing/ragdoll/integration/README.md). Baseline control articulation remains selectable.

- [x] **COLLISION-05** Hop over rear/downed rider and bike contacts without ejecting the attacking rider. Reason: user requested ramp-like arcade contacts; part: 01; dependencies: COLLISION-04; acceptance: bounded speed-gated hop, fast rear hit knocks target down, downed/runner impacts push or knock target over, own bodies and tiny taps excluded, repeated limb hits deduplicated by cooldown, head-on/car behavior retained; verification: [both entity orders, actual dynamic contacts, no duplicate bodies and native recovery tests](testing/ragdoll/integration/README.md). This user-requested hop supersedes the earlier no-launch policy for these contact classes only. Upright rear hops are superseded by COLLISION-06; downed-body/wreck hops remain.

- [x] **DISTRICT-01** Organize all 146 modeled IDs into district/support sets and original-course eligibility indexes. Reason: procedural scenery needs coherent local pools instead of a global random mix; originating part: 04; dependencies: complete draft mesh catalog and local resource research; acceptance: exactly one primary group per ID, five measured course-family views, labeled previews, canonical mesh links and explicit evidence/inference separation; verification: [146 name matches, 13 groups, 438 unchanged linked LODs, parser/coverage tests](testing/building-districts/README.md). District placement and transitions remain under V2-04-003.

- [x] **CITYGEN-01** Assemble full-depth buildings from the modeled facades and generate the playable 5.3 km City Run. Reason: district catalogs need a real race-world implementation rather than another isolated preview; originating part: 04; dependencies: DISTRICT-01, geometric facade assets, shared physics/recovery; acceptance: complete facade/roof/rear/end assemblies, coherent districts, exact 5,300 m open race, solid support/buildings, all 15 grid starts, working recovery/restart/checkpoints/finish, preserved Valley fallback; verification: [deterministic bake, per-building Rapier rays, foot routes, all gates, native city and Valley smoke, nine actual-game survey views](testing/city-5300/README.md). Production fidelity, fine collision/art review and broad performance/platform acceptance remain V2-04 gates.

### Part 01 — Remaining handling, crash response and runtime corrections

- [ ] **CRASHREF-004** Extend original-crash coverage and establish trajectory parity before changing the default. Reason: native 2.5D geometry, other callers and post-entry laws exceed the first 3D comparison; part: 01; dependencies: CRASHREF-001–003, retail trajectory fixtures and explicit gameplay review; acceptance: low/static object, combat, downed-body/traction/wall caller routing, original condition/regeneration tables, pair ordering/reciprocal traffic response, and justified native-vs-Rapier bounce/drag trajectories documented and tested; verification: reproducible original-vs-modern traces, multi-rider contact/energy/recovery scenes and user comparison. Current comparison boundaries remain explicit in [research](research/ORIGINAL_CRASH_CALCULATIONS.md).

- [ ] **V2-01-005** Design and implement measured vehicle-impact and over-car crash responses.

- [ ] **V2-01-006** Add a combat knockoff loss-of-balance sequence.

- [ ] **V2-01-007** Compare the reference crash/control code and evaluate bounded physics changes.

- [ ] **V2-01-008** Exit gate: accept corrections before authoring motion.

- [ ] **ARCH-001** Isolate `client`, pure `sim`, `physics`, authoritative `server`, typed `content`, and developer `tools` responsibilities; clients never adjudicate hits/results from reported transforms.

- [ ] **ARCH-002** Define stable bike/rider/weapon/track IDs, cosmetic-versus-physics IDs, schema versions, validators, and migration-safe saves/protocols.

- [ ] **ARCH-003** Define `AttackIntent`, `AttackResolved`, `CrashStarted`, `RecoveryCompleted`, racer-level `RaceFinished`, and `PoliceBusted` contracts with IDs/ticks and single-result semantics.

- [ ] **ARCH-007** Tick/control simulation at 60 Hz, AI perception at 5-15 Hz, interpolate independently, and use explicit adapter-observed components/events rather than renderer or animation authority.

- [ ] **RIGHTS-006** Sessions 01-02: workspace, CI, debug metrics, clean Fedora scene.

- [ ] **RIGHTS-008** Sessions 05-08: acceleration/brakes, two wheel rays, stable camera/turns.

- [ ] **RIGHTS-009** Sessions 09-10: lean, surface grip, three distinct presets.

- [ ] **P01-003** Split the single-crate prototype into client/server/core/vehicle/combat/race/AI/content/network/tools responsibilities, with a plugin per subsystem and conventional commit rules.

- [ ] **P01-004** Keep pure simulation independent of UI/rendering; isolate the physics adapter, presentation, typed content, dev tools, and later server authority.

- [ ] **P01-006** Add action mapping/input contexts, stable content IDs, schema versions, and the intent -> simulation -> authoritative event -> presentation contract.

- [ ] **P01-007** Add `devtools`, `debug_physics`, `offline`, and `netplay` feature boundaries and reproducible quality modes.

- [ ] **P01-009** Extend CI to a validated Windows build plus content/license audits and workspace-wide checks; verify the dependency migration separately.

- [ ] **P01-010** Validate a native Fedora/NVIDIA launch, Windows launch, and 100 pause/reset cycles without state corruption.

- [ ] **P01-011** **Exit gate:** independent 60 Hz ticks are measured, both platform launches are verified, and the required CI matrix passes.

- [ ] **P01-018** Obtain actual Windows and Fedora/NVIDIA native launch/input/audio receipts in addition to the CI build matrix; record the measured adapter and keep unmet hardware gates open.

- [ ] **P03-010** Run a ten-minute aggressive-turning soak without NaNs/tunneling/unexplained flips; record at least ten test rides and controller feedback.

- [ ] **P03-011** **Exit gate:** three presets feel distinct, a beginner can finish, and handling is fun and stable with placeholder art.

- [ ] **P03-020** Reproduce and numerically verify the entire reference handling, including original bike specs, drive/brake ramps, gear thresholds, nitro charges, surface tables and fixed-point effects. Original calibration files are unavailable and the user has none; source reuse permission is unresolved. See [precise remaining work](research/ROAD_RASH_HANDLING.md).

- [ ] **P04-004** Drive camera motion from interpolated fixed-step poses; verify no jitter at 120-144 Hz and readability through tight turns, impacts, and jumps.

- [ ] **P04-008** Validate camera clipping, both input devices, comfort-setting persistence, and responsive handling with an external tester.

- [ ] **P04-009** **Exit gate:** riding is fast, comfortable, responsive, and finishable with keyboard and gamepad without unintended auto steering.

- [ ] **P04-014** Verify the default mapping for the OS-discovered controller that Gilrs reports as unmapped; exercise real buttons/axes and add a licensed mapping only if needed. Evidence: [native receipt](testing/PART_04.md); physical mapping acceptance remains open.

- [ ] **P06-006** Add independent stamina, bike durability, balance/wobble, police-hit and bad-landing crash thresholds, and one authoritative `CrashStarted` event.

### Required animation coverage

### Research, labels and deliverables

### Part 02 — Animation A — reference study, labeled keyframes and pose approval

- [ ] **V2-02-001** Build a complete animation/use-case catalog.

- [ ] **V2-02-002** Inspect all original rider animation categories.

- [ ] **V2-02-003** Research Synty/POLYGON-style games and authentic locomotion.

- [ ] **V2-02-005** Author and label the full original 3D keyframe set.

- [ ] **V2-02-006** Cover running, on-foot recovery and non-rider characters.

- [ ] **V2-02-007** Publish a numbered pose gallery with anatomical review.

- [ ] **V2-02-008** Exit gate: keyframes and catalog approved.

- [ ] **ANIM-02** Refine fingers, grips, fists and weapon sockets. Reason: the current twenty-bone rig has rigid hands; part: 02; dependencies: rider mesh/rig and contact catalog; acceptance: believable closed fists, wrapped grips and compatible held-weapon points without changing passive physics; verification: front/rear/side hand close-ups and rig/contact validation.

- [ ] **ANIM-03** Resolve reference sequence linkage, timing and ambiguous labels. Reason: sprite slots include view/depth variants and do not establish timing; part: 02; dependencies: local reference metadata and CANS/RRAN/EXE research; acceptance: evidence-backed phase/side/weapon labels and timing or explicit unknowns for every reference family; verification: source-linked sequence audit, no guessed native durations.

- [ ] **ANIM-04** Complete and review recovery/support and specialist poses. Reason: get-up kneel/sit candidates are preliminary crouches and arrest/equip/drop coverage is missing; part: 02; dependencies: catalog and hand refinement; acceptance: supported get-ups/lifts/mounts, reviewed vault/knockoff proposals, weapon equip/drop, police restraint and refined quadruped anatomy with clothing/body/bike clearance; verification: multi-angle art review, endpoint/chain checks and user pose approval before Part 03.

- [ ] **ANIM-06** Review context-first pose fidelity and finish unresolved action bindings. Reason: correct sequence metadata alone does not prove faithful 3D silhouettes; part: 02; dependencies: ANIM-05 and new source-linked draft poses; acceptance: review every action/phase, source mirroring, fixed grip/fist/weapon placement, grounded get-up/mount support, crash/bike silhouettes, additional camera views and unknown caller aliases; verification: labeled source/recreation strips, joint/contact tests, multi-angle art review and explicit approval before interpolation.

- [ ] **P11-003** Establish tested Blender -> glTF/GLB scale, axes, rig/bone names, PBR materials, collision naming, LODs, and source/export folder separation.

- [ ] **P11-006** Introduce asset validation for triangles, skeleton paths, textures/material references, ownership metadata, release clearance, and content hashes.

- [ ] **ARCH-006** Define evidence/classic-reference/new-design/rights/QA catalog fields; keep original-source observations distinct from new stats/unlocks/visuals.

### Part 03 — Animation B — interpolation, terrain locomotion and active recovery

- [ ] **V2-03-001** Implement a data-driven animation state/blend graph. Initial riding/attack/recovery graph verified under ANIM-08; full weapon/police/locomotion coverage and approval remain open.

- [ ] **V2-03-002** Interpolate approved keyframes and combat/recovery transitions.

- [ ] **V2-03-003** Make walking/running match velocity and terrain. Partial: distance-driven stride, velocity blend, support-ray sole alignment and stance locking are integrated; pelvis/stair/turn-in-place and measured terrain slip acceptance remain open.

- [ ] **V2-03-004** Keep hands and feet on bike contacts when required. Initial four-contact render solver passes fixed-length/endpoint tests; full runtime hand/foot deformation and clearance approval remain open.

- [ ] **V2-03-005** Prototype bounded active ragdoll and player attempts to recover. Partial: supported player input raises only existing capped joint motors to 0.12 activation; neutral restores the passive crash curve. Controlled work/energy, conscious/unconscious policy and user feel approval remain open.

- [ ] **V2-03-006** Blend physical landings into terrain-aware get-up and remount. Partial: capture the settled physical pose and blend into get-up, followed by terrain-aware feet and existing mirrored remount. Confined clearance, recovery direction selection and full terrain approval remain open.

- [ ] **V2-03-007** Add motion/audio polish without changing gameplay outcomes. Partial: footsteps follow measured travel instead of elapsed recovery time. Contact-event timing, breathing/look and broader motion/audio polish remain open.

- [ ] **ANIM-09** Finish native full-body animation acceptance. Reason: authoring renders and numeric constraints do not certify every runtime deformation/transition; part: 03; dependencies: ANIM-08, ANIM-02/04/06 and terrain locomotion; acceptance: native multi-angle normal/tuck-to-all-attacks, mirrored pickup and physical-pose get-up review, fingers/feet and cloth/bike clearance, Windows hardware and user approval; verification: labeled native video matrix, contact/deformation checks and regression parity.

- [ ] **RAG-05** Approve native biomechanics silhouettes and finish distal skin articulation. Reason: pilot distal skin articulation and impact peaks require human review; prop NPC skins still use eleven joints; part: 03; dependencies: RAG-02/03, ART-04 and ANIM-02/09; acceptance: independently visible wrists/feet/spine, reviewed source-pose release and multi-angle drop/car/terrain/knockoff clearance, acceptable distal impact velocities and remaining multi-angle art approval; the user approved default integration on2026-11-06; verification: native videos, deformation/contact metrics and explicit user review. Partial: pilot wrists/feet/spine now use sixteen independent visual joints. Prop pedestrian skins, finger detail, all impact-camera views and final art approval remain open.

- [ ] **RAG-06** Profile and approve worst-case race ragdoll performance. Reason: additional bodies/solver work cost more than the baseline and instrumented headless timing is not race FPS; part: 03; dependencies: RAG-04 and representative traffic/environment load; acceptance: compare frame/physics p95 and allocations across15 racers plus bounded NPC crashes, sleeping/wakeup, Windows hardware and quality presets without regressions; verification: repeatable native profiling on target hardware and user acceptance, recording remaining production performance acceptance separately from the2026-11-06 default integration approval.

- [ ] **V2-03-008** Exit gate: motion is responsive, coherent and physically readable.

- [ ] **P11-008** Animate riding, fighting, crash, walking, and remount through a production rig; add lightweight animal animation/AI and safe cliff recovery.

- [ ] **FOOT-003** Crash with neutral input -> seek bike around a known obstacle; verify safe proximity/facing/low-speed remount.

- [ ] **FOOT-008** Run off a cliff or become trapped between a fence and car -> recover without an invisible-wall/terrain softlock.

- [ ] **RIGHTS-012** Sessions 16-18: manual foot movement, optional assist, foot finish, remount, no softlocks.

- [ ] **ART-02** Acquire a licensed pilot rider and integrate one shared skinned avatar across riding, fighting, walking and ragdoll. Original authored pilot integration is tracked under ART-03; complete production deformation/animation/LOD acceptance remains open. Reason: researched assets are not yet working models; part: character art/animation; dependencies: ART-01, model selection/acquisition and exact archive license; acceptance: biker outfit/color zones, coherent rig and recovery, unchanged physics/controls, 15-racer performance/LOD acceptance; verification: deformation/animation/ragdoll tests, native visual checks and profiling.

- [ ] **ART-04** Polish the visual retarget and authored animation transitions without changing simulation. Reason: authoring rig has 20 bones and the current fingers/pose drivers are prototypes; part: 11 art/animation; dependencies: ART-03 and static pose library; acceptance: clean grip/fist/deformation and contact-aware transitions at gameplay speeds; verification: pose review, native clips and repeated physics parity tests.

- [ ] **P05-006** Implement idle/lean/punch/kick/hit/stagger animation states, input buffering/priorities, anti-spam, and gameplay-owned timing.

- [ ] **P06-008** Separate OnFootAutoSeekBike and OnFootManual states; support FinishedOnFoot and BustedOrRecover outcomes. Auto/manual/FinishedOnFoot are implemented; police BustedOrRecover remains deferred to Part 08.

- [ ] **P06-009** Make remount safe: proximity, facing/interaction option, low bike speed, animation/collision checks, and no unwanted snap while deliberately walking away. Partial: proximity, facing, low-speed and explicit interaction gates are tested; final animation/obstruction clearance acceptance remains open.

- [ ] **P06-010** Tune control handoff toward 1-3 s after a crash; add optional delayed stuck recovery with penalties and no automatic finish-line teleport.

- [ ] **P06-012** Run 100 high-speed crashes without frozen physics/terrain clipping; audit trapping around fences, buildings, curbs, cliffs, and traffic, and tune repeated-hit protection/time loss.

- [ ] **P06-013** **Exit gate:** knockoff, manual foot movement, optional assist, valid foot finish, and safe remount all work smoothly.

### Part 04 — World A — facade study, original textures and procedural buildings

- [ ] **V2-04-001** Catalog every reference building/facade family locally.

- [ ] **V2-04-002** Create original facade textures in the RoadDash style. A first realistic AI reference batch covers 25 sheets / 146 source IDs; [manifest](../art/buildings/realistic_sheets/MANIFEST.json) and [prompts](../art/buildings/realistic_sheets/PROMPTS.json). Production textures, exact layout/text cleanup and art/rights acceptance remain open.

- [ ] **V2-04-003** Plan a data-driven city/world construction grammar. Valley now maps all 657 unchanged lots through 19 role pools and actual ground/repeated-upper/crown geometry; [grammar and limits](art/BUILDING_GRAMMAR.md). Broader acceptance stays open. Earlier partial: the [playable 5.3 km City Run](testing/city-5300/README.md) now builds 977 complete exterior lots across 53 blocks and eight district zones using 85 City-eligible geometric facade designs, shared closed roofs, recessed rear openings and cross-street facade returns. Canonical Valley quotas/IDs remain unchanged. Broader terrain grammar, final transitions/site variety and full world acceptance remain open.

- [ ] **V2-04-004** Generate detailed building archetypes and whole blocks. The default Valley now uses the derived storey kit with bounded streaming (KIT-01/02); final infill/role fidelity and material/user approval remain open. [Complete draft model catalog](../art/buildings/mesh_assets/README.md) now covers 100 facades, 28 structures and 18 surface modules with editable Blender sources and 438 embedded GLB LOD files, real apertures/storefronts/canopies/bays/roof geometry, independent binary validation and native placement in the 5.3 km playable city. Earlier palette/flat-facade previews were rejected and do not satisfy this task. Draft mesh coverage and city runtime assembly are implemented; full material/art fidelity, user review, optimization and production acceptance remain open.

- [ ] **V2-04-006** Validate city life, visual density and performance. The latest Valley native survey meets its local 16.7 ms wall-frame p95 target at15.83 ms after ornament-shadow trimming; [measurement and limits](testing/world-polish/README.md). Broader racing/memory/platform acceptance remains open. [Native playable-city survey and crash/recovery/retry evidence](testing/city-5300/README.md) available; production art/LOD, encounter tuning and broad racing performance acceptance remain open.

- [ ] **V2-04-007** Exit gate: coherent procedural buildings are accepted.

- [ ] **KIT-03** Finish Valley infill skins and bespoke role geometry. Reason: native views expose plain party-wall infill and parking/service roles using generic industrial cladding; part: 04; dependencies: KIT-02, immutable lot/rights-of-way constraints and material/art review; acceptance: infill joins visually match neighbors without street intrusion, and parking/service/farm roles have appropriate full exterior geometry rather than misleading reused fronts; verification: join-envelope tests, per-role coverage and native multi-angle riding/on-foot review. Performance remains V2-04-006.

- [ ] **MESH-01** Model the remaining 94 facade designs from the approved three-quarter renders. Reason: the first sheet supplies six genuine meshes, not the full catalog; part: 04; dependencies: six-model kit, per-design reference review and material authoring; acceptance: every remaining facade ID maps to a distinct editable 3D model with actual openings/details/roof/sides and verified GLB LODs; verification: catalog coverage, mesh/material/bounds validation, native multi-angle renders and user art review. Partial: all 94 additional facade drafts exported with editable sources and three LODs; artist/user fidelity approval remains open.

- [ ] **MESH-02** Author the 46 structure/surface references appropriately. Reason: barns, churches, tanks and surface samples must not be miscounted as completed facade buildings; part: 04; dependencies: structure classification and reference/model grammar; acceptance: each ID becomes its appropriate volumetric structure or usable material asset with scale, provenance and export metadata; verification: type-aware coverage, geometry/material intake and native visual review. Partial: 28 standalone structures and 18 volumetric surface modules exported; final material/art and production intake gates remain open.

- [ ] **P11-002** Create a road-world art bible for distinct town, farm, cliff, coast, and woodland identities before polishing bike/rider art. Partial: [Valley architecture vocabulary and construction contract](art/BUILDING_GRAMMAR.md); cliff/coast/woodland and full art direction remain open.

- [ ] **P11-004** Author modular roads/intersections/sidewalks, storefronts/homes, barns/silos/fields/fences, cattle/tractors, cliffs/rocks/barriers, signs, poles, and landmarks.

- [ ] **P11-005** Build the route-zone/lot/terrain/encounter pipeline and foot-accessible collision/navigation surfaces described in the environment checklist below.

- [ ] **ENV-001** E0: author spline, widths, shoulders/sidewalks, vertical curves, checkpoint and finish volumes.

- [ ] **ENV-002** E1: author distance-station zones (example: town 0-2 km, farm 2-5 km, cliffs 5-8 km) with smooth art/audio transitions.

- [ ] **ENV-003** E2: stamp roadside lots/terrain cells; reserve 10-30 m of critical visibility at bends and intersections.

- [ ] **ENV-005** E4: validate bike and on-foot collision/navigation through curbs, barns, fences, buildings, and cliffs; remove trapping/invisible barriers.

- [ ] **ENV-007** Define `WorldZone` ID/from/to/biome/road style/speed profile/scenery/audio/music/encounters/hazards/safe-spawn fields and corresponding schema validation.

- [ ] **ARCH-005** Author track bundles containing road, ordered checkpoints, spawn groups, props/LOD, original source art, runtime GLB, and separate collider GLB.

- [ ] **RIGHTS-007** Sessions 03-04: spline mesh/collision road and valid start/finish/checkpoint gates.

- [ ] **P02-004** Replace the generated flat polyline with an authorable Catmull-Rom/Bezier centerline supporting widths, banking, elevation, sectors, and optional route splits/merges.

### Part 05 — World B — props, materials and procedural dressing

- [ ] **MAT-02** Author bespoke detailed albedo/PBR texture families for remaining buildings, vehicles, props and riders. Reason: shared microdetail does not finish every requested texture; part: 05; dependencies: MAT-01, source provenance and approved art direction; acceptance: per-family approved source art, consistent texel density, mip/normal/roughness review, native close/far views with no missing maps; verification: asset hash/license audit, contact sheets and in-game visual approval.

- [ ] **V2-05-001** Inventory reference prop categories and gameplay roles. [Scoped source catalog and rebuild queue](research/props/README.md) extracted across all five courses; category/role proposals and eleven visually reviewed unresolved banks remain subject to identity/archetype/source-use review. Includes pedestrians, animals, traffic/parked vehicles, lamps/utilities, trees/plants, signs, furniture, fences, rocks and ground hazards. Source view/color/size banks are not unique finished models.

- [ ] **V2-05-002** Author original prop meshes and texture families. [Actual draft mesh catalog](../art/props/mesh_assets/README.md): 369 source IDs, 1,107 GLBs, 13 editable Blender scenes and exported-mesh proof renders. These are rough shared-archetype geometry drafts, not faithful finished reproductions of the [66 concept sheets](../art/props/concept_3d/ALL_CONCEPTS.html). Significant silhouette/material/pictogram/actor refinement, finished deformation rigs, collider refinement, wider placement and final art approval remain open. Selected drafts now run in game through PROP-03. Inspect with `cargo run --locked --bin prop_preview`; accepted race physics remain unchanged.

- [ ] **V2-05-003** Place props through semantic rules. PROP-03 adds reserved city sockets, existing tree/lamp replacements and traffic/NPC attachment; DRESS-01 adds131 guarded Valley props across15 roles. Full category placement, variety and encounter review remain open.

- [ ] **V2-05-004** Give gameplay props reliable physical/audio behavior. PROP-03 preserves existing moving-owner and ragdoll collisions and adds solid furniture; category-specific audio, breakables and comprehensive behavior QA remain open.

- [ ] **V2-05-005** Exit gate: dressed world remains playable.

- [ ] **ENV-004** E3: place memorable landmarks plus interactive fences, cows, cars, rocks, and tractors; make encounters readable rather than unavoidable surprise spawns.

### Part 06 — World C — NPCs first, then sky/terrain/landscape art

- [ ] **V2-06-001** Build NPC reference catalog and original character variants. Pedestrian/animal source banks are included in the [shared prop catalog](research/props/README.md); PROP-03 binds draft human/cow appearances to existing walking and ragdoll frames; identity/species review, finished deformation and animations remain open. Preserve the accepted human/quadruped physical rigs when replacing visuals.

- [ ] **V2-06-002** Create animal-specific art, animation and physical responses.

- [ ] **V2-06-003** Populate the world with believable NPC activity.

- [ ] **V2-06-004** NPC exit gate before the landscape pass.

- [ ] **V2-06-005** Study skyboxes, atmosphere and landscape references.

- [ ] **V2-06-006** Author original sky and landscape textures/materials.

- [ ] **V2-06-007** Integrate landscape art with the playable world.

- [ ] **V2-06-008** Landscape exit gate: the complete first-map art pass is accepted.

- [ ] **ENV-008** Implement cow grazing/startled/turn/cross/react states with simple critical collision proxies; animal ragdolls remain optional cosmetics.

- [ ] **P08-005** Add traffic gap keeping, braking, lane changes/intersections, and obstacle rerouting.

### Part 07 — Weapons and complete riding combat

- [ ] **V2-07-001** Research weapon behavior and define original data/assets.

- [ ] **V2-07-002** Integrate weapon animations, contacts and outcomes.

- [ ] **V2-07-003** Teach rivals fair weapon use and tune combat.

- [ ] **V2-07-004** Exit gate: weapons improve the race loop.

- [ ] **ARCH-004** Define weapon timing/range/stamina/damage/stagger/knockback/pickup data and bike class/mass/top speed/torque/grip/lean/durability/visual-asset data as authored files.

- [ ] **RIGHTS-011** Sessions 13-15: punches/kicks, timing/range/stagger validation.

- [ ] **P05-003** Add left/right kicks, then one simple held weapon and pickup rules.

- [ ] **P05-004** Define typed ability data for startup/active/recovery, range, stamina cost, impulse, damage, stagger, and cooldown; initial punch timing 0.15/0.12/0.30 s and kick timing 0.24/0.16/0.50 s are tuning proposals.

- [ ] **P05-005** Use stable side/front cones and transient swept sensor volumes; resolve each attack once with attacker/target/attack ID/contact point/relative speed/tick/result fields.

- [ ] **P05-007** Build a matched-speed side-by-side combat harness; test no double-hit, misses 0.5 m beyond range, and cooldown/stamina/immunity boundaries.

- [ ] **P05-008** **Exit gate:** three minutes of enjoyable punch/kick combat without weapons, with timing and hit validation evidence.

### Part 08 — Police, pursuit and arrest/escape loop

- [ ] **V2-08-001** Research original police behavior and author police content.

- [ ] **V2-08-002** Implement bounded police pursuit and interceptions.

- [ ] **V2-08-003** Integrate rider knockoff, on-foot pursuit and arrest/escape.

- [ ] **V2-08-004** Tune police feedback and fairness.

- [ ] **V2-08-005** Validate the complete offline loop before Part 09.

- [ ] **V2-08-006** Exit gate: proceed to existing Part 09, then Part 10.

- [ ] **RIGHTS-014** Sessions 23-24: police, town/farm/cliff audio zones, placeholder soundscape.

- [ ] **P08-004** Implement Cruise/SeekOvertake/Defend/Attack/Recover/Pursued/Arrested states with a blackboard and aggressive/cautious/opportunistic/reckless weights.

- [ ] **P08-006** Add police patrol, violation assessment, chase/intercept/bust, loss of target, and return-to-patrol states with testable transitions.

- [ ] **P08-009** **Exit gate:** fair, repeatable four-racer race with traffic, police, and melee, including opponent overtakes, hits, and recovery.

### Part 09 — Vertical-slice playtest gate

- [ ] **P09-001** Consolidate the three-minute town/farm/cliff route with player plus three AI, traffic/police, punch/kick, manual foot control, foot finish, checkpoints, results, and retry.

- [ ] **P09-002** Run at least ten external play sessions with recordings, handling A/B comparisons, perceived-speed/crash feedback, and failure/input notes.

- [ ] **P09-003** Measure finish rate, retries, voluntary replay, attack/hit counts, collisions, and time outside player control; record the five biggest frustrations.

- [ ] **P09-004** Test the proposed fun targets: 80% finish within three tries, over 50% voluntary replay, and time outside control below 12% of a session.

- [ ] **P09-005** Measure a 1080p Fedora/RTX 3080 slice with all P0 systems active against a stable 60 FPS baseline; store P95 frame-time evidence.

- [ ] **P09-006** Record keep/change/remove decisions and freeze the minimal controller/combat/race interfaces for the network spike.

- [ ] **P09-007** **Exit gate:** core loops need no debug intervention and external playtest evidence supports replaying the graybox; retune before expanding content otherwise.

- [ ] **ENV-009** Verify a distinct silhouette/encounter every 10-25 s at 100-220 km/h and recognizable locations without reading the HUD; record the on-foot hazard audit and frame times.

- [ ] **RIGHTS-010** Sessions 11-12: timer, standings, results, complete retry loop and evidence.

- [ ] **RIGHTS-013** Sessions 19-22: AI and traffic budgets, repeatable full race.

- [ ] **RIGHTS-015** Sessions 25-27: ten external playtests, metrics, tuning, frustration fixes.

- [ ] **P02-010** Build a short approximately 2 km handling road, then a roughly 6-9 km town -> farm -> cliff corridor tuned for a three-minute race, with straight, turns, hairpin, and hazards.

- [ ] **P02-011** **Exit gate:** an authored graybox supports a complete race with valid gates, stable collision, clear progress, and no terrain fall-through.

- [ ] **P07-003** Add seeded starts, ordered forward-crossed checkpoints, route segments, canonical racer-level progress, and out-of-bounds recovery.

- [ ] **P07-004** Accept a valid forward finish from either rider or bike with shared racer identity; record source OnFoot/Bike and a single finish tick/event/payout. **Finishing while on foot is not implemented.**

- [ ] **P07-005** Reject backward crossings, skipped gates, shortcuts, and finish teleports; handle same-tick rider/bike crossing exactly once.

- [ ] **P07-006** Add distance/progress, bike damage, weapon state, and on-foot bike-direction/distance HUD; retain stable placement on split routes and off-road movement.

- [ ] **P07-008** Verify retry retains settings while resetting all live race state, including traffic and recovery actors.

- [ ] **P07-009** **Exit gate:** repeatable approximately three-minute start-to-results-to-retry race with stable, checkpoint-validated bike and foot finishes.

- [ ] **P08-003** Route AI through the same `BikeControl`/vehicle physics interface as human input; remove hidden grip/acceleration differences unless explicitly presented as difficulty assists.

- [ ] **P08-008** Add AI debug goals, paths, desired speeds, and headless tournaments; complete 100 races without deadlock and test human/AI foot recovery separately.

### Part 10 — Networking feasibility spike

- [ ] **P10-001** Introduce Lightyear only after the offline slice gate; run a headless authoritative server with two LAN clients on one graybox track.

- [ ] **P10-002** Send tick/sequence-numbered input batches covering bike controls, attacks, manual foot movement, and remount; server owns simulation, hits, rankings, and rewards.

- [ ] **P10-003** Predict local movement, buffer/interpolate remote snapshots, reconcile errors, and deduplicate confirmed combat/crash/result events.

- [ ] **P10-004** Test 2 -> 4 players, high-speed opposing collisions, and punch adjudication; consider server-owned simplified/cosmetic client ragdolls.

- [ ] **P10-005** Measure 60 Hz server simulation, 20-30 Hz snapshots, and an initial bandwidth target below 100 kbps/client for four players.

- [ ] **P10-006** Exercise 0/50/100/150 ms RTT, jitter, and 0/1/5% packet loss; record correction distances, response, recoverability, and consistent hit outcomes.

- [ ] **P10-007** **Exit gate:** two clients race at 100 ms simulated RTT, loss does not create a desync storm, and the authority/physics approach is documented with measurements rather than deterministic-lockstep assumptions.

- [ ] **ARCH-008** Document snapshot/prediction/reconciliation assumptions and clock offsets; do not require cross-machine bit-identical physics or replicate individual cosmetic fragments.

- [ ] **RIGHTS-016** Sessions 28-30: reproducible two-client/server Lightyear spike and 100 ms test report.

- [ ] **P06-011** Separate simulation-critical colliders from cosmetic debris; budget ragdoll bodies/joints and avoid per-fragment replication.

### Part 11 — Remaining production-art integration and acceptance

- [ ] **P11-009** Validate readable scenery at racing speed, zero missing material/texture references, provenance/credits, and asset swaps without gameplay changes.

- [ ] **P11-010** **Exit gate:** the original town/farm/cliff art replaces graybox assets without changing physics/race rules and fits measured budgets.

### Part 12 — Garage, economy, progression and saves

- [ ] **P12-003** Add real garage selection/ownership, prices, upgrade tiers, stat comparisons, repair costs, and cosmetic/loadout unlocks. The current garage is a start menu and credits cannot be spent.

- [ ] **P12-004** Define typed RON/TOML/JSON bike/rider/weapon/track/loadout/economy data; validate on load without recompiling tunables.

- [ ] **P12-005** Extend saves to owned bikes, finances, class/track records, unlocks, and options with explicit schema versions, migration fixtures, checksums, and rotating backups.

- [ ] **P12-006** Test interrupted writes, crash/quit/restart double-credit prevention, and save/load equality over 100 write cycles; keep absolute asset paths out of saves.

- [ ] **P12-007** Build an economy/progression simulator and play through starter -> upgraded bike without softlocks; keep offline/online trust models separate.

- [ ] **P12-008** **Exit gate:** purchasing a better bike changes play, saves/migrations are reliable, and repeated races have a working reward loop.

- [ ] **P07-007** Add penalties, practice/free-ride, best records per class/track, and data-driven race length/AI/traffic/police/difficulty settings.

### Part 13 — Authoritative online races and lobbies

- [ ] **P13-001** Add join/lobby/ready/bike selection, synchronized starts, disconnect handling, reconnect policy, and consistent results.

- [ ] **P13-002** Define unreliable-sequenced redundant input batches and delta snapshots; use reliable/bounded or ID-deduplicated combat/crash/pickup/bust/finish events and idempotent rewards.

- [ ] **P13-003** Validate input magnitude/rate, selected bike, weapon ownership, reach/cooldown, damage, transforms, checkpoint order, and foot-or-bike finish on the server.

- [ ] **P13-004** Implement interest culling, interpolation buffers, prediction smoothing, replication priority, CPU/egress/RTT telemetry, and lag-compensated melee history.

- [ ] **P13-005** Replicate rider-foot and abandoned-bike states separately; keep particles/cosmetic ragdolls separate from authoritative crash/recovery state.

- [ ] **P13-006** Add authenticated identity, abuse/rate limits, instance IDs, heartbeat/logs, replay/incident dumps, and a 30-60 s reconnect/spectator policy without retroactive placement.

- [ ] **P13-007** Validate four-player races at 150 ms RTT / 1% loss with identical results and no duplicated finish/prize or bypassed attack limits.

- [ ] **P13-008** Measure eight-player stress before enabling it: 60 Hz tick budget 16.67 ms, target server P95 below 8 ms, and bounded bandwidth/corrections.

- [ ] **P13-009** **Exit gate:** stable private four-player races; eight players remain a stretch goal until measured headroom exists.

- [ ] **FOOT-009** Disconnect while on foot -> reconnect/AI timeout preserves authoritative rider/bike state and results.

### Part 14 — Content expansion and modes

- [ ] **P14-001** Add at least three further distinct original tracks spanning city, farm/livestock, coast/cliffs, and optional mountain woodland with unique hazards and passing opportunities.

- [ ] **P14-002** Build lightweight/balanced/touring/superbike/specialty classes, rider archetypes, cosmetics, loadouts, weapon pickups, and police variants.

- [ ] **P14-003** Add classic sprint, career/tournament, time trial, private multiplayer, and optional combat arena modes.

- [ ] **P14-004** Provide accessible difficulty controls and progression/unlock tests; only surface intentional AI assists explicitly.

- [ ] **P14-005** Use one versioned, validated content format across player/AI/network; separate cosmetics from physics and load new bikes/tracks without core-code edits.

- [ ] **P14-006** Add track/spawn editing tools when multiple contributors need them; verify packaged content hashes.

- [ ] **P14-007** Balance classes with 1,000 automated races plus human playtests; prevent career softlocks and regressions against the original slice gate.

- [ ] **P14-008** **Exit gate:** several unique tracks/bikes work through the same data pipeline without breaking the quality gate.

- [ ] **AI-03** Extend original AI parity to all career levels, full three-axis relationships/events, seeded career speed jitter and rubber-band tables. Reason: normalized current-level rules do not prove full original behavior; part: future AI/career; dependencies: AI-02 and career/difficulty authoring; acceptance: independently implemented per-level scenarios and documented unit/random calibration; verification: pinned-reference scenario comparisons and human difficulty tests.

### Part 15 — Presentation, audio, UI and accessibility

- [ ] **P15-003** Implement the licensed free-audio catalog, actual RPM/load-driven engine-band crossfades, spatial emitters/listener, surface sounds, location ambience, music buses, and audio QA below.

- [ ] **P15-004** Add scalable lighting/shadows, sensible optional SSAO/SSR/reflections, decals, anti-aliasing controls, and optional motion blur; do not require grain/blur/heavy shake.

- [ ] **P15-005** Evaluate optional Hanabi particles for dust/exhaust/skids/sparks and cosmetic debris; cap emitters, GPU cost, and overdraw per quality setting.

- [ ] **P15-006** Polish garage/race/results UI, subtitles/non-speech captions, colorblind affordances, persistent input/audio/comfort settings, and rebinding.

- [ ] **P15-007** Add quality presets; test HUD at 1080p/1440p/4K and wide/ultrawide displays; never rely on color alone for critical state.

- [ ] **P15-008** Measure stable 1080p medium 60 FPS with four players/busy traffic and the aspirational 1080p high 120 FPS RTX 3080 target; add replay/highlight tools only if performance permits.

- [ ] **P15-009** **Exit gate:** presentation and audio are trailer-ready while meeting performance, legibility, accessibility, provenance, and saved-setting checks.

- [ ] **AUDIO-001** Create `content/audio_manifest.ron` / the asset ledger: path, author, exact source URL/license copy, acquisition date, hash, edits, redistribution permissions, attribution, and approval timestamp.

- [ ] **AUDIO-002** Motorbike inventory (~10 clips): idle, two mid bands, high band, rev hit, shifts up/down, two pass-bys, starter.

- [ ] **AUDIO-003** Surface inventory (~6): asphalt, gravel, dirt, grass, skid, wind; drive from wheel slip and `SurfaceType`.

- [ ] **AUDIO-004** Combat inventory (~5): light/heavy punch, kick, weapon, grunt; choose 3-5 licensed variants where appropriate from validated hit events.

- [ ] **AUDIO-005** Crash inventory (~5): body fall, metal hit, scrape, fence break, glass/crumple; layer by contact severity once per confirmed event.

- [ ] **AUDIO-006** World inventory (~6): city bed, farm insects/wind, cow, tractor, cliff wind, coast/surf; tie localized sources and ambience blends to zones.

- [ ] **AUDIO-007** Police/UI inventory (~4): near/far sirens, checkpoint, menu/finish cues; keep audible threat/readability priorities.

- [ ] **AUDIO-008** Implement an `AudioService` boundary with 3-4 seamless engine RPM/load bands, envelope crossfades, actual gear/transmission transients, and modest pitch changes.

- [ ] **AUDIO-009** Put the listener at the chase camera and emitters on bikes, police, cattle, and landmarks; test stereo/rolloff, opponent minimum volume, and distinct distance/gear cues.

- [ ] **AUDIO-010** Blend town/farm/cliff/coast ambience; visible livestock/tractors must match their sounds; keep music on a separate licensed, duckable, streamer-safe bus.

- [ ] **AUDIO-011** Evaluate Kira only if needed; if chosen, avoid running conflicting default Bevy audio and Kira backends.

- [ ] **AUDIO-012** Validate no loop clicks/gaps/clipping from idle to redline, distinguishable location ambiences, and no doubled combat/crash sound after prediction correction.

- [ ] **AUDIO-013** Persist master/engine/SFX/ambience/music sliders and input/audio options; caption critical non-speech cues and validate source release permissions.

### Part 16 — QA, performance, platform, dependency and release gates

- [ ] **BASE-001** Verify the proposed dependency combination with locked Fedora/Linux and Windows builds; record the selected versions and migration decision.

- [ ] **BASE-002** Preserve a working baseline while migrating; keep engine, physics, input, audio, and networking adapters independently testable.

- [ ] **P11-007** Give every scenery family LOD, collision, and audio metadata; reveal recognizable landmarks or encounters every 10-25 s without hidden active hazards/pop-in.

- [ ] **P16-003** Profile CPU/GPU/render stages, draw calls, animation, contacts, networking, server ticks, and repeatable 1080p/1440p frame-time percentiles.

- [ ] **P16-004** Add golden state/replay checks, reproducible crash reports, a bug template, automated 30-minute eight-bot soak, and high-latency/loss multiplayer soak.

- [ ] **P16-005** Verify clean Fedora KDE Wayland/NVIDIA and Windows NVIDIA/AMD installations, controller/reconnect matrix, ultrawide HUD, save migration, and network ports.

- [ ] **P16-006** Add dependency/content/license audit and compatible `cargo audit`/`deny` checks to CI; generate current SBOM/attributions and resolve all P0/P1 bugs.

- [ ] **P16-007** Review localization, accessibility, opt-in telemetry/crash reporting, privacy/moderation policy, source-asset permissions, and final public-title/trademark clearance.

- [ ] **P16-008** Package/install-test distributions; add Steam depots/achievements/cloud saves only if selected; rehearse hotfix/support and stage closed test -> demo -> launch.

- [ ] **P16-009** Verify beta crash-free sessions above 99%, 60 FPS preset P95 frame time <=16.7 ms, and server P95 tick <=8 ms on recorded target hardware.

- [ ] **P16-010** **Exit gate:** functional, platform, performance, stability, rights, clean-install, support, and release-signoff evidence is complete; a successful build alone is not release approval.

- [ ] **ENV-006** E5: stream/LOD by visibility/distance without unloading or hiding authoritative hazards.

- [ ] **NFR-001** Store repeatable benchmark scenarios and results against every mandatory row above; distinguish requirements from aspirational targets and record intentional scope cuts.

- [ ] **NFR-002** Validate adaptive scenery budgets: modular building boxes/low LOD, near-corridor tree/rock proxies, lightweight distant animals, stable cliff collision, and batched road furniture.

- [ ] **RIGHTS-001** Obtain written commercial-use permission for acquired assets/contributions and proof for soundtrack, voices, logos, and any real motorcycle brands.

- [ ] **RIGHTS-002** Review the original-successor title/marketing and any close reproduction of protected expression; assume no permission for EA branding, exact tracks/characters/art, original retail files, or extracted media.

- [ ] **RIGHTS-003** Generate attribution/SBOM notices from the actual lockfile and asset manifests; audit installers, screenshots, trailers, and website material.

- [ ] **RIGHTS-004** Archive selected source/license versions for R01-R09 and the audio/art catalogs; research-only code stays research-only unless permission is established.

- [ ] **RIGHTS-005** After each part, attach a tagged build, test results, recording, performance metrics, architectural decision, explicit remaining bugs, and source/rights notes.

- [ ] **ART-05** Author distance LODs and validate the detailed rider/bike budget on more hardware. Reason: LOD0 replaces cheap primitives across 15 racers; part: 11 optimization; dependencies: ART-03 and profiler; acceptance: readable silhouettes, stable collision-independent swaps and measured frame budget; verification: native distance/performance surveys and Windows hardware review.

- [ ] **P08-007** Budget active racers/cars/police by visibility and distance; perception 5-15 Hz, controls 60 Hz, safe spawn/despawn outside view.
