# Testing requirements and evidence matrix

Scope: Parts 00-01, 2026-10-30. Detailed receipts: PARTS_00_01.md. Never replace
unrun platform results with assumptions from a successful local build.

| ID | Scenario | Verification |
| --- | --- | --- |
| T-CORE | IDs, edge buffering, award rules | rr_core tests, no renderer/physics dependency |
| T-CONTENT | version, finite tuning, colors, references, duplicates | rr_content tests; --validate-content success/failure |
| T-INPUT | remapped/unknown/duplicate bindings; zero-tick frame press | controls tests and attack_press_survives_a_render_frame_without_a_fixed_tick |
| T-FIXED | identical two-second intent at 30/60/120 render FPS | fixed_simulation_matches_at_30_60_and_120_render_fps; exact 120 ticks and pose tolerances |
| T-RESET | 100 crash/pause/resume/reset cycles | frozen body positions while paused, no debris/joints/disabled colliders/entity leaks |
| T-STATES | Boot -> Loading -> MainMenu -> Race -> Results | lifecycle_visits_loading_menu_race_and_results |
| T-CRASH | momentum/separation and riderless wreck | crash_separates_rider_and_preserves_momentum_without_an_upward_boost; pre-solver spawn snapshot |
| T-REMOUNT | six fallen poses, approached from both sides | remount_reaches_a_fallen_bike_in_sideways_upside_down_and_vertical_poses; 8-second bound each |
| T-REGRESSION | controls/combat/traffic/ragdoll/runner/camera/finish | existing gameplay tests retained in game/track/world/sound |
| T-SAVE | versionless credit/time preservation, schema 1 roundtrip, corrupt/future rejection | legacy_profile_is_upgraded_without_losing_progress |
| T-AUDIT | dependency declarations, media rights/hash, 20+ observations | tools/audit.py plus tools/test_audit.py |
| T-NATIVE | real window, audio sinks, crash -> foot -> remount -> results | --smoke-test screenshots + wall/fixed diagnostics JSON; scripted finish fixture is not checkpoint/foot-finish proof |
| T-PANIC | local backtrace file and expected nonzero process | --verify-panic-report with temporary ROADDASH_LOG_DIR |
| T-FEATURES | minimal offline and debug collider builds | --no-default-features --features offline; --features debug_physics |
| T-WINDOWS | MSVC build/headless tests + native GPU/audio/input | CI matrix for former; separate real Windows session for latter (open) |
| T-NVIDIA | Fedora KDE Wayland/NVIDIA launch | target-specific hardware session required (open unless adapter evidence matches) |

Mandatory local commands:

```sh
cargo fmt --all --check
cargo clippy --workspace --locked --all-targets -- -D warnings
cargo test --workspace --locked
cargo build --locked
python3 tools/test_audit.py
python3 tools/audit.py
cargo run -- --validate-content content/catalog.json
cargo run -- --smoke-test
```

CI repeats the source/headless checks on Ubuntu and Windows. It cannot replace
real input-device, graphics, audio, clean-install, performance or rights review.

## Part 02 additions (2026-10-30)

Detailed receipt: PART_02.md. New checks: headless authored road imports including
open endpoints and invalid geometry/gates/spawns; render/collision ribbon winding;
240 terrain/road support rays across all zones; no-teleport complete fixed-controller
point-to-point race, skipped/reverse/outside gates and missed-finish retry; boost
release/exhaustion, four grass AI/player cases, cow lift, solid slope slide and
100 physical ragdoll drops with finite transforms/velocities, anchor and hinge-axis
checks. Native smoke includes both farm and hill recovery, city, hills and F4
inspector captures, active audio sinks and results. World-tour/result teleports
are presentation fixtures only; race validation is the separate controller test.

## Part 03 additions (2026-10-30)

Receipt: [PART_03.md](PART_03.md). Regular headless checks explicitly cover:
three preset acceleration/braking and wheel support; dynamic acceleration/braking
at 30/60/120 FPS; leaning/inverted self-righting; stationary and moving curb hits;
eight-rider grid; no lane-zero magnet; dry wreck mass and stationary remount;
direct combo timing/duplicates/geometry and matched-speed contact thresholds;
NPC topologies/budget/reset/occupied respawn; shirt-color minimap identities.
Explicit ignored soak runs 600 simulated seconds of alternating steering and
records numeric acceleration/braking/steady-turn data. CI runs it on Linux.
`--validate-handling content/handling.json` validates the pure authoring contract.
Native `--smoke-test` and `--actor-smoke-test` run serially with graphics/audio;
they exercise scripted scene/recovery and distinct ragdoll topologies. They do
not replace manual ten-ride feedback, beginner finish, tactile control testing,
free-form crowd collisions or actual Windows hardware runs.

## Part 04 additions (2026-10-30)

Receipt: PART_04.md. Camera continuity at 60/120/144 Hz, sphere-clearance wall,
warp snap, native comfort panel/city/hill/recovery captures; analog trigger values
reach chassis forces; gamepad steering/freelook/deadzone/disconnect and keyboard
fallback; modal countdown pause/restore; live keyboard/controller rebinding and
invalid/conflicting settings preservation. Real controller completion, tactile
input/comfort feedback and subjective high-refresh readability remain open.

Latest corrections: four human states share mesh/material/scale slots, steering
demand does not roll visual root, six civilian-kind/class contacts clear measured
height/pass obstacle with finite physics, and four cross-car cases wipe out (both
adapters, approaching a stopped car / moving car striking a stopped bike).

## Source-informed steering / camera correction

Receipt: HANDLING_CORRECTION.md. Core checks held-force buildup, return/reversal,
speed dependence/parked zero and timestep agreement; content accepts old profiles
and rejects invalid response. Dynamic integration verifies increasing turn force
and no parked strafe. Actual visual hierarchy banks bike/rider together while
removing a tilted road's roll at zero steering. Camera distance stays bounded in
27 mode/speed/render-rate scenarios with slow smoothing. Existing car/civilian,
recovery, controls, collision and race tests stay in the full suite. Numeric
original-game equivalence is not tested/claimed; P03-020 needs missing data.

Base-world B00–B02: `rr_content::world` tests exact section/registry/row/family
counts, deterministic and seed-varying allocations with unique stable IDs,
invalid versions/topology/budgets/lot dimensions, boundary ownership and blend
continuity. `world_gen` tests baked-station projection/end snapping and mismatched
courses/seeds; report tests compare two actual legacy environment builds and
separate allocations from measured masses. `tools/test_world_inputs.py` compares
all source CSV/JSON section fields and converted archetypes/heights/quotas.
CI runs source checks and `--validate-world`; native evidence/F4 screenshots and
CLI failure probes: ../world/BASE_WORLD_TESTS.md. Spatial placement/ground/LOD
and final-world acceptance remain BW-B03–B10.

B03–B10: full spatial layout repeats exactly, corruption/duplicate junctions are
rejected, every footprint corner has physical foundation support, main-road
rays and crossing stubs remain solid, internal terrain seams have continuous
support, LOD children coexist with all 247 fixed colliders, and three real
capsules walk farm/city-alley/hill routes. The full existing suite and explicit
600-s simulated soak cover gameplay regressions. Native survey captures all 15
sections with actual resolution and 1080p warmed city metrics; normal race and
actor smoke remain separate. JSON/footprint SVG, root and geometry hashes and
full evidence: ../world/COMPLETION_TESTS.md. Hardware/subjective gates from other
parts are not silently closed.

### 2026-10-31 initial combat/camera follow-up

`cargo fmt --all --check`, workspace Clippy with `-D warnings`, workspace locked tests (107 passed, one soak ignored), and `python3 tools/audit.py` passed. Logs are local `/tmp/roaddash-followup-{clippy,tests-final,audit}.log`. Build completion is recorded separately in `/tmp/roaddash-followup-build.log`. No native playtest or city/lean acceptance was performed. Kick buffering has a new consumed-once/context-clear regression; actual kick contact, pose and camera crowd behavior remain unchecked.

### 2026-10-31 completed city/combat follow-up

Supersedes the initial partial entry above. See CITY_COMBAT.md and city-combat/ for 112 passing tests (one optional soak ignored), successful format/Clippy/build/audit, measured 657-root native survey, seven lean and two kick captures, original-disc proportions and successful farm/hill recovery-to-finish smoke. Camera crowd and yaw faults have focused regressions; human subjective handling and Windows hardware are not claimed.

### CITY-02 touching frontages — 2026-10-31

113 tests passed, optional soak ignored. Exact join edges, street reservations and physical roofs of all 347 infills checked. Updated exposed-perimeter route audit passes; existing physical capsule and gameplay regressions pass. Native city/grid screenshots reviewed after correcting smooth normals; 1080p p95 9.79 ms. Native crash→remount→city/hills→hill crash→remount→finish passed. See FRONTAGES.md and frontages/ for logs/captures.

### CITY-03 streetscape — 2026-10-31

115 tests pass (one optional soak ignored), including sidewalk flat normals,
every street-tree trunk's collision/clearance and taller floors with stable lots.
Format/Clippy/locked builds, source-input checks and provenance audit pass.
Native inspection, 15-section survey (1080p p95 9.66 ms), road-height visual
review and crash/remount/finish smoke pass. Receipts and limitations:
[STREETSCAPE.md](STREETSCAPE.md).

### Roster, kicks, handling and pursuit — 2026-10-31

118 tests pass; one optional soak ignored. Signed 8 m/s kick, shirt-colored dots
for 15 racers, reduced tree sites, neutral-heading independence, physical wreck
roll, conserved linear momentum/remount, deterministic pursuit grace/speed/spacing/
downed selection and existing regressions pass. Workspace format/Clippy/locked
builds, source-input checks/audit pass. Native 15-section survey: 11.29 ms p95
at 1080p; crash/remount/finish and audio initialization pass.
See [RIDERS.md](RIDERS.md) and riders/ receipts. Human tuning and Windows remain open.

### AI-02 personalities and rear grid — 2026-10-31

120 tests pass (one optional soak ignored). Profile validation, relationship
pass/follow/fight and downed filters, 15-spawn rear ordering/12 m gaps/negative
projection/terrain clearance/solid support plus prior regressions pass.
Format/Clippy/locked builds, metadata-reader reproduction, tracker/source checks
and provenance audit pass. Final native inspection/survey (1080p p95 14.74 ms),
racing visual review and crash/remount/finish with audio pass. Initial approach
terrain intrusion was fixed and all checks rerun. [Evidence](PERSONALITIES.md).
AI-03 full original parity, human acceptance and Windows remain open.

### ART-03 authored rider/bike, preserved ragdoll — 2026-10-31

125 workspace tests pass, one optional soak ignored. Model intake corruption/NaN/
joint-ID/physics-payload checks, four-state skin consistency and nonphysical mesh
components pass. Eleven-body differential trace: 180 ticks, zero maximum position
difference; descriptors/velocities/rotations unchanged within recorded tolerances.
Full crash/run/remount differential and detached-skin cleanup pass. Workspace
format/Clippy/locked debug+release builds, six Python tests, provenance/tracker
checks and world inspection pass. Native farm/hill recovery, finish, NPC crashes
and audio pass. All 15 survey sections: 1080p warmed city p95 12.90 ms on this
Linux/RTX 3080/Vulkan host. [Receipts and limits](MODELS.md). ART-04/05 and Windows/
human visual acceptance remain open. No body/joint/controller tuning changed.

### V2 motion/restart repairs — 2026-10-31

130 workspace tests pass, one optional soak ignored. Format, warnings-denied
Clippy, locked debug/release builds pass. Native real-key retry restores all
fifteen racers; farm/hill recovery, finish/audio and actor collision smoke pass.
Connected endpoint/hinge/extreme-pose tests and isolated 180-tick eleven-body
comparison (zero maximum positional difference) pass. Full-cycle remount permits
2 mm f32 projection tolerance. [Evidence and limits](MOTION_REPAIRS.md).
Final 15-section survey passes: 1080p warmed city p95 12.25 ms on this Linux/RTX
3080/Vulkan host. Nine Python tests, tracker sync and provenance audit pass.
Over-car vault, melee swerve, full animation/IK, user acceptance and Windows stay open.

### Completed-first README reorganization — 2026-10-31

348 IDs/states/requirement texts/evidence links match pre-move task sources exactly.
All 128 checked tasks precede all 220 unfinished tasks; per-part placement and
summary counts match. Eleven Python tests, tracker sync, provenance audit, local
evidence-link validation and diff whitespace checks pass. Rust format, workspace
Clippy with warnings denied, 130 workspace tests (one ignored) and locked build
pass. No native rerun for documentation/tool-only changes.
[Evidence and maintenance rules](README_REORGANIZATION.md). No new completion gates closed.

### Input-owned heading and crash energy — 2026-10-31

136 regular Rust tests pass; the optional ten-minute/600 simulated second soak
was run separately and passes. Player/AI 120/40 -> 80 km/h entry, real rear and
oncoming contacts (~2.924 m forward travel/eight ticks), no spawn spin/launch,
energy/gravity/angular limits and 30/60/120 Hz neutral heading pass. Native race/
actor/audio/retry/recovery smoke and 15-section survey pass (1080p city p95
12.79 ms on this host). Format, Clippy, locked builds, eleven Python tests, audit,
tracker and content/world validators pass. [Receipts/limits](COLLISION_RESPONSE.md).
Isolated body parity remains zero; full-scene remount tolerance is now documented
2 cm for observed 6–8 mm contact/projection variance, with exact stage timing.
Full vault/swerve, new Windows runtime and human acceptance remain open.

### Animation A static authoring — 2026-10-31 / 2026-11-01

140 regular Rust tests (one optional soak ignored), 14 Python tests, format,
Clippy with warnings denied and locked build pass. The authoring CLI validates
247 draft poses / 27 use cases. Independent Blender reload checks all 247 poses,
247 single-frame constant Actions and 50,670 channels with maximum matrix error
4.77e-7 and no physics bodies. Offline gallery run filter/detail/side view was
checked visually. [Evidence, draft gaps and limits](ANIMATION_A.md). No runtime
playback, native smoke rerun, remote CI or art approval is claimed this session.

### Context-first animation correction — 2026-11-01

Five new Python tests (19 total), reproducible metadata and source semantic
expectations;194 descriptor/rear comparisons against pinned external reference.
138 human poses /41 use cases pass content validation;138 one-frame Actions and16
bike layouts pass independent Blender reload (max matrix error5.37e-7). Source
and new action-ordered strips reviewed as draft. Runtime art/passive physics
remain unchanged; no native-original/Windows/CI parity claim. Full evidence:
[ANIMATION_CONTEXT.md](ANIMATION_CONTEXT.md). ANIM-06 art approval remains open.

## Animation full-body/interpolation pass — 2026-11-01

ANIM-07/08: 150 regular Rust tests (one optional soak ignored), 20 animation-context
Python tests; format/workspace Clippy/build; 138 static Blender Action reloads and
36 motion Actions. Nine GIFs are authoring renders. Native Fedora/RTX3080/Vulkan
race smoke passed crash/get-up/run/remount twice and finish, 15-racer restart and
five audio sinks. Full attack/tuck native visual matrix, terrain-foot fidelity,
Windows and user approval remain ANIM-09. See [receipt](ANIMATION_B.md).

## Original-crash comparison — 2026-11-01

CRASHREF-001–003: native-unit integer rules, bounded retail x86 corroboration,
opt-in bike/car and bike/bike bridge, separate rider/wreck velocities, default
rig/control/recovery regression. Full command evidence and explicit parity limits:
[ORIGINAL_CRASHES.md](ORIGINAL_CRASHES.md). CRASHREF-004 remains open. Retail checks
need the user's original EXE plus optional Unicorn2.1.4 and are not represented
as ordinary CI coverage or original Windows-game execution.

## Facade authoring prototype — 2026-11-01

Four new offline authoring tests pass (24 total Python tests): constant bay/window
sizes and recessed glass, corner geometry/ground bounds, OBJ indices/material
references and invalid configuration. Full Rust regression: 163 pass, one existing
ignored soak; fmt, clippy, locked build, audit and progress sync pass. Preview
inspected. No runtime geometry changes; native city placement/performance and
complete catalog/art acceptance remain open under V2-04. Source and reproduction:
[facade prototype](../research/FACADE_PROTOTYPE.md).

## Playable City Run5300 — 2026-11-01

Current default: city-run-5300,5,300m open course. See
[full receipts](city-5300/README.md). Required checks cover the rr_content city
contract; deterministic source/compiled-data equality; City-family district
filtering; all300 derived-front GLB/source hashes; whole-lot/sidewalk reservations;
all977 actual Rapier building-roof rays; grid/road/sidewalk/runoff support; foot
routes from rear courts; ordered checkpoint crossings; all15 native restart
positions; two native crash/recovery cycles and results; nine actual-game district
survey views; unchanged Valley native smoke and existing rig differential tests.
171 Rust tests pass (one existing600s soak ignored by default),48 Python tests pass;
format/strict Clippy/build/audit/reproduction checks pass. Broader production,
ornamental collision, long-session/multi-crash/Windows/performance gates stay open.

| T-DOWNED-CONTACT / COLLISION-04 | Original-style downed/mounted/standing routing; solid low-wreck passage without forced wipeout | rr_core::contact tests; collision_tests::detached_body_contacts_never_script_a_new_mounted_wipeout (16 cases,11 contacts each); vehicle::tests::dynamic_bike_physically_traverses_a_solid_low_wreck; full car/runner/recovery regressions and native City Run smoke. [Receipt](contact-routing/README.md). Exact retail/SI parity and subjective pile-up approval remain open. |

| T-PROP-CATALOG / PROP-01 | Complete scoped local reference extraction, hierarchy/frame aliases,3×2 sheet review, stage/provenance separation | Nine malformed-input/alias/frame/grid/queue/output-boundary tests;4172 source CELs reconciled;1310 images validated;597 unique review groups in105 sheets; zip CRC and browser filter/search/view checks. [Receipt](prop-catalog/README.md). Actual new geometry, semantic placement, art/physics/audio/native acceptance remain V2-05/V2-06. |


### Full prop concept authoring coverage (2026-11-01)

- 369/369 queue IDs covered; 59 additional source-mapped sheets, 66 selected total.
- Evidence: `prop-catalog/all-concepts.json` and `prop-catalog/all-concepts/`.
- PNG/hash/slot/queue-stage/archive validation passed; nine Python catalog tests
  and five roadmap tests passed; required fmt, Clippy, test/build and audit passed.
- All new output sheets visually inspected; three targeted v2 edits retained.
- Browser file URL blocked by policy; interactive UI and native integration
  unverified. Bitmap coverage does not close mesh/rig/art-approval gates.


### Draft prop GLB catalog (PROP-02, 2026-11-01)

- All 369 IDs, three indexed-triangle LODs each, embedded original 128px material
  tiles, valid normals/UVs/indices, positive-volume extents, reduced triangle counts
  and exact model/source/render hashes checked independently of Blender.
- Every ID has a proof sheet rendered after re-import; parked-vehicle clay renders
  remove material imagery. Selected sheets inspected visually; this is not final
  art review of every asset.
- Isolated Bevy GLB native captures: fire engine, cow and human. First importer
  failure fixed through existing explicit scene-type registration. No racing-body
  replacement; full Rust regressions include accepted-model physics checks.
- Six mesh-authoring tests (including malformed binaries and semantic exceptions),
  nine reference-catalog tests, roadmap synchronization and asset declaration audit.
- Evidence: prop-meshes/README.md and adjacent command logs.


### 2026-11-01 — PROP-03 draft props in gameplay

[Integration evidence](props-in-game/README.md): strict locked Rust checks,
179tests passing/one ignored, prop authoring/reference tests, independent118skin
GLB checks and1,107static GLB checks. Socket tests preserve crossings, road and
pedestrian lane; NPC tests preserve physical definitions/ejection velocity and
bound180tick simulations. Actual City nine-view survey, Valley actor knockdown
smoke and City two-crash/recovery/finish smoke pass onVulkan/RTX3080. Scene receipts
identify selected IDs, not approved art or all catalog entries placed. Performance,
platform breadth, deformation quality and production release remain open.

## Biomechanical ragdoll comparison — 2026-11-01

RAG-01–04: domain activation/zero-control, anatomical hard limits and passive
motors; source-bind calibration; pose-hold/external torque; coherent COM/spin
release; drop/limb-force/torso-push/hinge/8 and75 m/s ejection/road-tumble/wall/
fifteen-rider scenarios; both baseline and opt-in biomechanical mode. Native
comparison scene, full race/recovery and actor smoke. Format/workspace locked
Clippy/tests/build and provenance audit required. Full skin/native realism and
production p95/Windows/user approval remain RAG-05/06. Exact receipts:
[ragdoll report](RAGDOLL_BIOMECHANICS.md).


2026-11-02 RAG-04 usability: verify a bare launch selects biomechanical, baseline
CLI/environment remains available, explicit CLI wins, conflicting flags fail,
and game/lab share mode selection. Run default and explicit-baseline native race
smoke plus the two-mode standalone view. Evidence: ragdoll/default-launch/.


2026-11-06 RAG-07/08 and COLLISION-05: integration suite has200passing Rust tests,
one preexisting ignored soak; equal-load knee deflection, paired inward hip stress,
original direction-policy handoff, rear entity ordering and actual dynamic ride-over
contacts included. Native default/baseline City race recovery, Valley NPCs and
paired lab view pass;18headless cases finite. Full receipts: ragdoll/integration/.
New per-joint resistance does not close distal-art or broad performance gates.

## V2 Part 03 animation/recovery (2026-11-06)

[Receipt and native review](animation-part03/README.md), ADR050. Corrected mirrored
punch/kick selection and tuck/lean elbow poles;16-joint pilot visual articulation,
physical landing blend, distance/velocity gait and support/stance feet; manual
camera-relative WASD/stick with delayed optional assist, safe interaction, shared
foot/bike payout and supported capped bracing.217 workspace tests pass (one
preexisting ignored soak), strict Clippy/fmt/build, audits and roadmap checks pass.
Native motion viewer, city default/baseline recovery and Valley human/cow smoke
pass on Linux/NVIDIA using the documented large-stack workaround for city load.
All18 paired headless lab scenarios finite;240 pileup bodies sleep. Neither this
nor overlapping scripted smoke constitutes RAG-06 production profiling. Native
art/Windows/terrain clearance, confined traps, active-work budgets and default
asset-loader stack diagnosis remain open. Failed launch/actor precondition logs
are retained, not omitted from acceptance evidence.

## User camera/contact/locomotion follow-up (2026-11-06)

[Receipt and corrected wrist render](recovery-polish/README.md), ADR051. Added
rear-contact momentum/energy and60km/h threshold checks with no upright hop;
static wall plus low-ceiling/foreign-ragdoll camera tests in mounted/on-foot
contexts at60/144Hz; recovery-heading continuity across road tangent changes;
continuous procedural gait/forward-knee invariants, downward held elbows and
standing/seated parent-relative distal wrist axes/attachment; supported one-shot
forward/side roll energy conversion, unsupported/baseline rejection and capped
motor-only frontal release without direct velocity impulses. Native eleven-motion
review, optional crash-style comparison, Valley bare/default/baseline recovery and
NPC actors provide local rendering/physics evidence. Production subjective motion,
full terrain/clearance, Windows and15-racer profiling gates remain open.

## Part04 Valley architecture — 2026-11-06

Receipt: [architecture-valley/README.md](architecture-valley/README.md).
231workspace tests pass (1existing ignored soak), including deterministic657lot
role binding, allband/LOD transformed envelopes, real repeated floors, malformed
pools/budgets/paths/hashes/bounds. Independent binary check covers384GLBs/34PNGs
and original source SHA256s. --inspect-world retains canonical quotas, support/
walk audits and unchanged geometry fingerprint. --world-survey captures15sections,
counts actual Mesh3d descendants, rejects decorative colliders and asserts pending/
active streaming limits. Native smoke covers farm/hill crash/remount, city and
finish/audio on the default stack. Native frame p95 target is still unmet; this
is a failed acceptance measurement, not a fully passing Part04 exit gate. Native
AppExit errors now propagate to process status. Art/user, seam/stream stress,
Windows and broad hardware performance certification remain open.

## Surface/loading/offscreen crash follow-up — 2026-11-06

Receipt: [world-polish/README.md](world-polish/README.md).237 workspace tests pass,
1existing ignored soak. Added relief bounds/winding, original mipmapped asphalt
PBR validation, distant-rear/camera eligibility and integrated no-body/five-second/
pause/road-centre/reset tests. Native default Valley and explicit City Run smoke
pass without a stack environment override after configuring asset IO workers.
World survey checks actual displayed geometry, bounded hidden preparations/cache
and unchanged657 physical lots; frame target and broader art/hardware gates stay
separate. Part05 is explicitly requested after commit/push/merge, not auto-advanced.

## 2026-11-07 — Part05 world/contact follow-up

Receipt: [part05-world/README.md](part05-world/README.md). Soft actors and wrecks
use saved entry motion; test all three handling presets, stopped solver velocity,
mounted attacker, knocked target and bounded pair energy/lift. Existing complete
model/physics/recovery/control/reset/save regressions remain in workspace tests.
New tests validate deterministic footprint-guarded dressing,34 provenance/decode
profiles, mip/PBR channels, tangent position preservation, cube edges/config and
dither weights. Native smoke covers two crashes/remounts/audio/finish. Full-route
survey counts actual loaded prop meshes/maps/tangents and unchanged657 building
owners, separately from bounded frame-time and broader art/hardware acceptance.

## 2026-11-07 — Roadside variety

DRESS-02 receipt: [roadside-variety/README.md](roadside-variety/README.md).
Seeded35-role/20-group grammar validated against the original prop catalog;
compare complete transforms/size/model seeds, change scenery seed, check zone
start/end coverage, fair weighted budgets, road/access/support clearance and
malformed/missing references. Native census must show at least28 roles/16 groups,
all five zones and actual descendant meshes/solid owners. Extended world survey
adds close views of actual farm pickup, city transit and hill picnic groups after
the existing benchmark/pose stages, without modifying measured simulation timing.

## 2026-11-07 — Crash behavior research only

[RAG-11 source audit](../research/crash-behavior/README.md): five exact upstream
commits/file hashes and licenses inspected; current Bevy/Rapier/GLAM versions read
from manifest/lockfile. Original animation selector/catalog and current combat,
detached origin, support detection, muscle targets, roll and energy guard inspected.
No external demos, retail replay or new crash feature was run; gameplay/native
checks from PR17 remain historical baseline evidence. This change alters docs
only. Verify roadmap synchronization/tests, formatting, declaration audit and a
zero runtime/content/Cargo diff from merged main; proposed behavior acceptance
remains unchecked. User receives findings before any implementation.

## Method B — velocity muscle comparison

Focused pure/backend cases prove real active angular changes, reciprocal tensor
momentum, exact unchanged linear velocities, bounded measured work/power/impulses,
invalid/zero budget handling, actual wall contacts rejected as support, finite
retries/high-motion yield/rest and eleven-body baseline exclusion. Full workspace,
headless lab, native smoke and common swerve/release/recovery regressions are
recorded in crash-comparison/method-b after execution; no visual/user gate inferred.

## 2026-11-07 — Velocity selection and facade pop-in

RAG-12/LOD-02 verified on codex/velocity-popin; detailed commands/logs/screenshots
in velocity-popin/README.md.258workspace tests pass,1ignored; strict Clippy/fmt/
build/audit/roadmap pass. Paired18finite ragdoll cases preserve9baseline rows.
Native crash/recovery/remount/route/finish/audio smoke passes. Native world survey
validates15sections657roots,9actualdirectcrossfades and bounded displayed/cache
counts, but deliberately exits1: p95=16.94174ms exceeds16.7ms. Productionperf,
LOD-03 distant/alternate-map art and full roll recovery remain open.

## 2026-11-07 — Model-derived facade identity / City Run ready handoff

LOD-04 technical integration:263workspace tests pass/1ignored; fmt/strictClippy/
locked build pass.128designs383bands and15mips have source/output hash, guard,
coverage and bounds verification. All old/new face masks have complementary pixel
coverage. Native Valley/City continuous near/far/reverse/orbit fixtures pass, with
53/389initialdirectcrossfades and48cache/8transition maxima forCity. Native Valley
andCity crash/recovery/finish smokes and standalone preview pass. Actualworld
15section/657root census validates2628proxy meshes/226956triangles/134086656bytes.
Initialworld p95=18.45385ms fails16.7ms; repeat performance receipts and exclusions
are recorded in facade-lod/README.md. LOD-03 user visual/long-streaming and broad
production/hardware performance gates remain open. No physics tuning or Part05.

Final isolated facade LOD survey: p95=16.80066 ms against16.7 ms, so performance gate remains open (exit1); geometry/source/coverage assertions and all15sections pass. Final native Valley sweep54direct crossfades, City389; both recovery smokes and preview pass. Longer streaming/user approval remain open.

## World repair (2026-11-07)

| ID | Scenario | Evidence |
| --- | --- | --- |
| T-CORE-DEPTH | Every lot/face/LOD band stays outside both inset backup cores | architecture::tests::both_backup_shells_fit_behind_every_scaled_facade_band |
| T-WALL-TOUCH | 1km/h wall touch, fast parallel scrape and hard direct entry | collision_tests::static_wall_uses_normal_closing_speed_instead_of_touch_or_total_speed |
| T-KNEE-PLANE |100samples each of pickup/mirrored/run/getup | animation::correction_tests::pickup_and_running_keep_ankles_on_their_own_side_and_knees_in_a_bending_plane |
| T-KICK-SIDE | Closest rider outside reach selects leg but receives no remote hit | combat::tests::kick_faces_nearest_out_of_reach_opponent_without_a_remote_hit |
| T-TRAFFIC-QUEUE | Brake/stop/horn cooldown/resume | game::tests::traffic_brakes_and_rate_limits_horns_then_resumes_when_rider_clears |
| T-WORLD-REPAIR | Large facade sweep, route, recovery, actors and performance | world-repair/README.md; performance gate explicitly unmet |

| T-DENSE-LAYOUT |2Donly: touching frontages, finite non-self-crossing outlines, deterministic exports, unchanged runtime | tools/plan_dense_city.py + tools/verify_dense_city_layout.py; planning/dense-city-layout/VALIDATION.json and CHECKS.txt; user layout approval and future construction gates remain open |

| T-CITY-CENTER | Two split/rejoin graph, four legal paths, equal nominal branch lengths, plot/race clearance, invalid graph rejection and unchanged runtime scope | tools/plan_city_center.py --verify; planning/city-center/VALIDATION.json and CHECKS.txt. Layout review and playable integration remain open. |

| T-FARM-LONG-CITY | Whole route, exactly one split/two paths, first-quarter rejoin, urban finish, elevation/grades and rigid rectangular building plots | tools/plan_farms_long_city.py REPORT --verify; planning/farms-long-city/VALIDATION.json and CHECKS.txt. Deliberately bent plots, a second split, hills phase, grade jump and out-of-city finish are rejected. No playable-map or performance acceptance implied. |

| T-STREET-FIRST | Real road/sidewalk reservations define valid nonoverlapping blocks; ownership partitions cover land; angled corner site permitted; buildings stay within sites; graph crossings noded; regular pitch180×360m; one race split/two paths | tools/street_first_layout.py + plan_farms_long_city.py --verify; planning/farms-long-city/VALIDATION.json and CHECKS.txt. Supersedes T-FARM-LONG-CITY's rectangle-only constraint. User/construction/playable gates remain open. |

| T-CITY-MASSING | Downtown height exceeds outskirts; seeded taller frontage/interior sites; valid per-building height/detail association; actual distance to either race route; protected distant skyline; clear connected6m tower access and no building overlap; invalid height/tier/missing/blocked access rejected | tools/city_massing.py + plan_farms_long_city.py --verify; planning/farms-long-city/MASSING.md, VALIDATION.json and CHECKS.txt. Static planning only; native3D/performance acceptance deferred. |

| T-RACE-FIRST-CITY | City sections centered on actual race paths within half a grid cell; both-sided development; primary-junction spacing/angle/incidence and protected fork throats; omitted competing secondaries; connected street graph; race/farms identical to de20a70; concave footprint overlay/inner rings; retained massing/geometry checks | tools/race_first_layout.py + plan_farms_long_city.py --verify; planning/farms-long-city/RACE_FIRST.md, VALIDATION.json and CHECKS.txt. False centering/off-track junction/restored omitted street/disconnected fragment rejected. No3D/runtime traffic/performance claim. |

| T-NO-CLOSE-PARALLEL | Within110m,48m sustained parallel run at less than40degrees removes the whole secondary segment; partial run missed by old60percent rule is caught; perpendicular crossings and180m-spaced avenues remain; both race branches protected | tools/test_race_first_layout.py (4 fixtures), race_first_layout.audit_race_first checks all retained segments; planning/farms-long-city/VALIDATION.json. Authoring only. |

## Metropolis — 2026-11-07

| ID | Scenario | Evidence |
| --- | --- | --- |
| T-METRO-CONTENT | strict schema/finite geometry/style/path rejection and deterministic bake | rr_content::metropolis tests; testing/metropolis/bake-determinism.json |
| T-METRO-ROUTE | both paths projection, choice hysteresis/lock, ordered physical gates; wrong branch/large jump rejection | rr_core::route and track::metropolis tests |
| T-METRO-NAV | exact diagonal polygon boundaries preserve primary corridors | world_gen::metropolis::navigation tests |
| T-METRO-NATIVE | both branch recovery fixtures, restart grid, audio, results; no full driven race claim | testing/metropolis native logs/receipts |
| T-METRO-RENDER | farm/both branches/downtown/north/finish and constant resident meshes | testing/metropolis/metro-00..06.png and metro-survey.json |
| T-METRO-PERF | bounded1440×900 RTX3080 Vulkan dev sample | metro-performance.json; p9520.38ms FAIL16.7ms target; full-course/platform certification open |

## SIDEWALK-01 — 2026-11-07

- Authoring fixtures: width increases2.4→3.4m, street reservation unchanged,
  farm pavement unchanged; deterministic props, conservative complete footprints,
 2m building-side paved walk and20m junction setbacks.
- Content fixtures: unknown roles, duplicate IDs and nonfinite furniture sizes
  rejected. Preservation receipt compares every building, path, field, street,
  junction, bounds, spawn and gate with the previous committed runtime manifest.
- Native survey/smoke evidence: testing/metropolis-sidewalks; original ragdoll,
  recovery and legacy content regression suite remains required. Full traffic,
  subjective art/platform and production performance gates stay open.

## AUTHORED-01 — 2026-11-07

- Repeat original kit bands/bays on straight/diagonal polygon edges at6/32/110m;
  finite positive transforms, width/height fit, restrained relief and once-only
  unique signage. Preserve source GLBs and physical map records.
- Verify existing kit and far atlas hashes; record the neutral BUSS4 upper remap
  in the Metropolis derivative source ledger. No runtime-only identity override.
- Native survey queries actual Mesh3d handles against original loaded GLB
  primitives, counts actual triangle indices, rejects decorative colliders and
  enforces displayed+retiring budgets. Resident base chunks remain unchanged.
- Native both-branch crash/recovery/restart/audio results fixtures and bounded
  performance evidence: testing/metropolis-authored. Art/palette/full-course and
  platform/performance exit gates remain open.

## STREET-LIFE-01 — 2026-11-07

- rr_core street profiles: American centres, continuous merges, body clearance,
  stopped-leader queue/brake/resume. rr_content rejects profile gaps/invalid counts/
  over-wide layouts and retains existing strict geometry/route checks.
- Authoring: deterministic per-building contrast/interior towers, parked body
  support/junction setbacks, reserved street centres without race edits; updated
  furniture fixtures protect walking strips and all role coverage.
- Audit3,480 complete vehicle rectangles against actual pavement through tapers;
  compare preserved footprints/IDs/paths/gates/streets/sidewalks with8f83245.
- Native survey queries main fleet/directions/lane counts/parked/furniture and
  actual height histogram, plus original facade draw/collision budgets. Native
  both-branch crash/recovery/restart/audio and performance evidence alongside
  testing/metropolis-street-life; full manual traffic/platform/art gates remain open.

## Metropolis prop destruction — 2026-11-07

DESTRUCTION-01, ADR073, detailed evidence: [prop-destruction/README.md](prop-destruction/README.md).

| Check | Evidence |
| --- | --- |
| All13 placed kinds registered | Production factory995 owners +13 native fixture models |
| Genuine collision and no artificial prop launch | `real_contact_breaks_releases_and_reset_restores_backend_pose`, native real-sphere rams |
| Restart clears persistent damage | headless backend reset and native before/damaged/reset poses |
| No slow/paused damage | `slow_contacts_and_paused_damage_are_ignored` |
| Direct melee and obstruction | `two_direct_punches_break_a_bin_but_a_wall_blocks_the_same_strike` |
| Bounded persistent debris | `budget_keeps_additional_damage_as_persistent_static_wreckage` |
| Supported settling | `supported_slow_debris_settles_and_stays_present` |
| Recovery paths move/reset | `moved_wreckage_updates_only_its_spatial_cells_and_reset_restores_footprints` |
| Config/domain bounds | rr_content13-role/finite/budget coverage; rr_core durability/energy/reset |
| Runtime regressions | both native branch recovery smoke fixtures; receipt limitations apply |

Coarse damage art is not complete fracture-art approval. Worst-case damaged-course
performance, broader legacy-map scope and Windows hardware verification are not claimed.

## Racer/killer roles — 2026-11-07

AI-10/ADR074, evidence: [racer-killer-ai/README.md](racer-killer-ai/README.md).
Seven/seven role assignment, unprovoked racer suppression, exact-ID retaliation
including shared personality slots, reset, launch grace, bounded killer pursuit,
cooldowns, passing and downed exclusions have domain scenarios. Integrated melee
checks actual provocation and refusal to kick a nearer innocent bystander. Native
smoke asserts real roster counts and both-route crash/recovery/grid restart.
AI-11 retains human full-race aggression balance; physics and art gates stay open.

## Opening-world and local performance logging — 2026-11-07

REPAIR-01/PERF-07, ADR075, [evidence](startup-world-diagnostics/README.md).
Check held seated outward knee bend/peg contact and unchanged unheld chains;
both fence posts; matching widened farm support; seeded reduced traffic;
monotonic city-edge tower/frontage cap;0.5km/h main/cross/parked vehicle touches,
solid gentle scrapes, persistent-contact high-speed ejection and recovery.
Native release survey covers farm/entry/skyline; both routes retain roster/restart/
recovery. Verify actual parseable background JSONL including render counters and
memory/paging; true GPU attribution and target hardware budget remain PERF-08.

## Parallel physics — 2026-11-07

PERF-09, ADR076, [evidence](parallel-physics/README.md). Confirm actual Cargo
feature graph activates rapier3d/parallel plus SIMD under defaults and omits it in
serial fallback. Run full physical/momentum/anatomy/model/recovery tests under
parallel, compare18 paired headless900-tick lab cases against preserved serial
binary, verify native both-route roster/crash/getup/run/remount/restart and backend
log metadata. Compare measured timings without promising linear scaling, true
GPU timing or bitwise parallel determinism. PERF-08/platform gates remain open.

## Live pickup and reverse — 2026-11-08

RECOVERY-01, REVERSE-01, ANIM-14, ADR077: [evidence](remount-reverse/README.md).
Test a displaced elevated wreck against stale owner/backend poses in both
adapters, then retain the dynamic pose across later ticks. Test brake-before-
reverse at30/60/120 Hz, a solid wall backout, throttle return and assisted reverse
without false finish. Run full workspace/physics/model/recovery regression,
required audit/build/clippy/fmt, native Metropolis retry/audio/farm/city recovery,
reverse and exact pickup-error assertions. Animation coverage report must be
current and six context tests pass; this does not close pose/animation approval.

## Animation A glove contacts — 2026-11-08

ANIM-15/16, ADR078, testing/hand-contacts/README.md. Verify four independent
hand shapes blend without overshoot, invalid socket/quaternion/hinge/key metadata
is rejected, seated sockets match universal grips, wrists/non-hand vertices stay
unchanged, finite GPU deltas and shared image budgets. Run original model/body/
180-tick physics and full crash/recovery differential regressions, workspace
fmt/clippy/test/build, native hand close-ups and game/restart/reverse/recovery.
Native hand review must observe relaxed, both-grip, left-fist/right-grip and
right-fist/left-grip states and package all33 labeled views. Exact held poses
must not be confused with dynamic clip snapshots. User art approval, Blender
round-trip, weapon gameplay, Windows and production performance remain open.

2026-11-08 Part02 authoring development: [evidence](part02-authoring/README.md).
Blender4.5.9 full138-action/16-bike FK reload, independent grip/fist GLB surface
reload, typed sockets and actual pose-loader resets;18source hand views and33native
hand views. rr_content validates complete per-pose hand weights/held contacts;
Rust checks Blender fold position/normal fixtures. Full catalog/art/normal parity,
clearance and user approval remain open; no Part03 interpolation or physical tuning.

## ADR089 — performance synchronization / UI prompt pass (2026-11-08)

- Exact immutable triangle-soup and gameplay-role preservation; explicit versus
  standalone static drop/wall/CCD/ray trajectories; stable settled/fading facade
  render inputs: workspace Rust tests.
- Actual13-view metro survey, default route0 and baseline route1 crash/recovery /
  audio/results plus an explicit reverse/club flag fixture; paired18-case lab; native census/logs/screenshots:
  [performance/UI evidence](performance-ui-prompts/README.md).
- Nine summary/prompt tests and17 existing UI-kit tests cover all28 draft screen
  /142image IDs, exact source aspects, deterministic regeneration and catch-up /
  malformed/paused/zero-tick log interpretation.
- Full driven-route60FPS, streaming spikes, GPU timestamps and hardware coverage
  remain open under PERF-08. Prompts do not close artwork or native UI approval.

| T-UI-DIRECTION | Original-screen study, preserve rejected batch, separate corrected sample | docs/ui/UI_ART_DIRECTION.md; generated-ui/README.md. Candidate inspected as an image; revised browser visual approval remains open. |

| T-NATIVE-FRONTEND | Offline original-inspired navigation, no held-confirm skip, controller/mouse/keyboard, launch/reset/results/settings, image readiness and viewport bounds | native-frontend/README.md, Rust domain/integration tests, --frontend-smoke actual window/layout receipts. Human controller/accessibility/production art acceptance stays open. |

| T-RIVAL-GRID | Seeded half-grid clubs; skip player/police; new-start/retry only; retain policy/drop | random-rival-weapons/README.md;375 Rust tests and part09-start/run-01 native held census. |
| T-P09-RUNNER | Isolated receipt directories/saves/settings, pinned seed, failed/timeout process and entry report validation | part09-start/README.md; three Python tests and actual entry run. Full suite/human gates remain open. |

## 2026-11-08 — Approved portrait picker (ADR094)

UI-02/UI-05 remain partial. Verify sixteen approved source/runtime hashes, four-column up/down navigation, selection bounds, return-to-card focus, unchanged profile/phase, every native card/detail and retained street/race/pause/results/quit. Evidence and environment limitations: [approved portrait UI](approved-portrait-ui/README.md).

## 2026-11-08 — Full-page portrait biographies (ADR095)

All sixteen live bios at 1280×720, lower-strip bounds clear of controls, 4×4 card navigation/focus, retained native menu/race/results flow, font readiness/provenance and workspace checks: [dossier evidence](portrait-dossier/README.md). Human art/profile/hardware gates remain open under UI-02/UI-05.

## 2026-11-09 — Combined traffic / weapon models / road walking

AI-12: pure motion/occupancy bounds and real dynamic stop/follow/pass/crossing/4-rider queue with car collisions retained, plus police/role regressions. ROAD-WALK-01: old-input stall control vs24 authored support traversals, solid-wall rejection, actual public-ground/sidewalk native walk/remount on both routes and rig modes, and four/three/four native lane views. WEAPON-MODELS-01: source/12GLB hashes/geometry/materials/origins,18 native LOD/equipped/fallback cases, optional headless path and exact model/rig/recovery differential checks. Root clean rebuild, full workspace checks and combined native skin+walking modes: [integration evidence](traffic-weapons-road-fixes/README.md). FOOT-SPAWN-01 remains unverified/deferred.

## Fresh arsenal/police/downtown pass (2026-11-09)

See [combined evidence](combat-police-downtown/README.md), [downtown geometry](downtown-street-life/coverage.json) and per-agent Part07/08 evidence. Require full locked workspace/fmt/Clippy/build/audit, all-family real buffered input/contact/drop tests across render rates, police memory/search/HUD regressions, source-preserved4/3/4 and whole sidewalk patrol/bay checks. Native survey must count actual96 walkers, observe movement and show downtown props/parking; normal and baseline recovery plus real arsenal pickup/attack/drop and police smoke must remain functional. Human art/fun/fairness, flexible weapons, moving capture and hardware60FPS stay open.

## Original-format entry frontend (2026-11-09)

[Evidence](original-ui-format/README.md): exact stored main/street control proportions,4:3 stage and aspect-preserved background crop;22-stage native flow,all16 dossier biographies and five viewport sizes; disabled choices/keyboard/Esc/F2 and retained Results/Busted R tests. Strict workspace/fmt/build/audit and font OFL/hash intake mandatory. User readability/art/crop,HUD/pause/settings parity,hardware/controller/performance remain open.

## Modern entry menu polish (2026-11-09)

[ADR103 evidence](modern-ui-polish/README.md): full-window proportional cover,smaller-dimension typography scaling,native gradients and nonoverlapping action safe areas. Existing22-stage/five-viewport/all16portrait native flow and whole-workspace rig/input/retry regression required. Retain asset hashes and licensed fonts; human art/crop/contrast,HUD/pause/settings,physical controller/Windows/accessibility/performance remain open. No automatic next part.

## Grouped settings editor (2026-11-09)

[ADR104 settings evidence](settings-menu/README.md): legacy-file defaults/nondefault persistence and invalid-file preservation; real panel category/audio/window actions,windowed-size restoration,modal pause/input suppression,failed-save/dismiss and pickup rebind/cancel; absolute master mix/default invariance. Native opt-in frontend run traverses all4settings categories with720p controls,real keyboard master/VSync/save and temp-file reload,then completes existing22-stage/16portrait/race/results flow. Human audible/fullscreen-monitor/controller/Windows,per-bus audio/quality/accessibility/smaller viewports remain open. Rig/controls/save/profile regression and strict workspace checks mandatory.
