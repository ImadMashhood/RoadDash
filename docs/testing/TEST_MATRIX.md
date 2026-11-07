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
