# Repository investigations

Date: 2026-10-30. Versions below are inspected snapshots, not assumptions from
the blueprint. Classification separates actual reuse from documentary research.
Dependency SPDX declarations are checked in CI; adoption is not commercial-release
legal signoff. Full asset rights remain separate.

## R01 - arkits/road-rash

- URL: https://github.com/arkits/road-rash
- Author/organization: arkits
- Revision/version inspected: `90f782b50c22f1a00db4ff8b4dc4e632d038dd12`
- Investigation date: 2026-10-30
- Classification: **REFERENCE ONLY**
- Purpose: C++/SDL historical behavior research
- RoadDash parts: P00, P02-P09, P12
- Relevant inspected material: README menu/career, varied courses, contacts, pursuit, controls, sound
- Verified license evidence: No explicit implementation reuse permission verified; https://github.com/arkits/road-rash/tree/90f782b50c22f1a00db4ff8b4dc4e632d038dd12
- Reuse/media boundary: No source reuse; media has independent EA rights
- Compatibility: Not a Rust/Bevy dependency
- Maintenance/risk: Active README/status observed; parity claims not independently reproduced
- Decision and actual incorporation: Study; no code or assets incorporated; independent original game only

## R02 - Bevy-Jam-5/bike-game

- URL: https://github.com/Bevy-Jam-5/bike-game
- Author/organization: Bevy Jam authors
- Revision/version inspected: `08658d123008fdc2896d20aa41b3e074b71fae5e`
- Investigation date: 2026-10-30
- Classification: **REFERENCE ONLY**
- Purpose: Bevy bicycle game design/example
- RoadDash parts: P00/P01; later P03/P11
- Relevant inspected material: README mechanics/rights; manifest dev features and scene workflow
- Verified license evidence: MIT OR Apache-2.0 declared in README; https://github.com/Bevy-Jam-5/bike-game/blob/08658d123008fdc2896d20aa41b3e074b71fae5e/README.md
- Reuse/media boundary: Code could be reused with notices; mixed city/music/media licenses need per-file review
- Compatibility: Bevy 0.14/Avian 0.1.1; not drop-in compatible
- Maintenance/risk: Jam project; inspected snapshot only; no maintenance SLA inferred
- Decision and actual incorporation: Study only; no implementation or media copied

## R08 - bevyengine/bevy

- URL: https://github.com/bevyengine/bevy
- Author/organization: Bevy contributors
- Revision/version inspected: `v0.17.3 / 706fdd9800c4bf335d67f127ff6e5a93deadabb9`
- Investigation date: 2026-10-30
- Classification: **DEPENDENCY**
- Purpose: Engine/ECS/render/audio/state/scheduling
- RoadDash parts: P01 throughout
- Relevant inspected material: Official fixed-step example, asset/font source, Camera/States APIs
- Verified license evidence: MIT OR Apache-2.0; https://github.com/bevyengine/bevy/blob/v0.17.3/LICENSE-MIT
- Reuse/media boundary: Permissive code with retained notices; Fira font separately OFL; no unverified LUT media
- Compatibility: Exact installed 0.17.3 pinned; 0.20.1 exists but migration not performed
- Maintenance/risk: Maintained upstream; old pinned line has known renderer ordering constraints
- Decision and actual incorporation: Retain dependency; independent fixed-step/plugin implementation; no example code copied verbatim

## R10 - dimforge/bevy_rapier

- URL: https://github.com/dimforge/bevy_rapier
- Author/organization: Dimforge contributors
- Revision/version inspected: `v0.32.0 / b4ad97f9fc6bb8950d9f793d4602efac834d010e`
- Investigation date: 2026-10-30
- Classification: **DEPENDENCY**
- Purpose: Bevy adapter to rigid bodies/CCD/joints/KCC
- RoadDash parts: P01/P03/P06
- Relevant inspected material: Installed plugin in_fixed_schedule, physics sets, collider/KCC APIs
- Verified license evidence: Apache-2.0; https://github.com/dimforge/bevy_rapier/blob/v0.32.0/LICENSE
- Reuse/media boundary: Retain Apache notices; no media imported
- Compatibility: Manifest pins Bevy 0.17.3; currently compatible
- Maintenance/risk: Maintained upstream; pinned compatible baseline, no lockstep guarantee
- Decision and actual incorporation: Retain; fixed scheduler uses actual adapter APIs

## R04 - avianphysics/avian

- URL: https://github.com/avianphysics/avian
- Author/organization: Avian contributors
- Revision/version inspected: `a693f2b7cec0955770fabfa45a8e0fae355c449f`
- Investigation date: 2026-10-30
- Classification: **REFERENCE ONLY**
- Purpose: ECS physics and alternative adapter
- RoadDash parts: P01 migration decision
- Relevant inspected material: crates/avian3d/Cargo.toml dependency and license fields
- Verified license evidence: MIT OR Apache-2.0 (manifest); https://github.com/avianphysics/avian/blob/a693f2b7cec0955770fabfa45a8e0fae355c449f/LICENSE-MIT
- Reuse/media boundary: No source/media imported; verify full selected package before adoption
- Compatibility: 0.8.0 requires Bevy 0.20; incompatible with current 0.17.3 baseline
- Maintenance/risk: Maintained upstream; snapshot inspected, migration/regression cost
- Decision and actual incorporation: Defer coordinated migration; no dependency added

## R06 - simgine/bevy_enhanced_input

- URL: https://github.com/simgine/bevy_enhanced_input
- Author/organization: Hennadii Chernyshchyk / Alice Cecile and contributors
- Revision/version inspected: `deaddeca88f2cab1ae249e74c4b5752ca3ede151`
- Investigation date: 2026-10-30
- Classification: **REFERENCE ONLY**
- Purpose: Action maps and binding contexts
- RoadDash parts: P01/P04
- Relevant inspected material: Cargo.toml dependency mapping and optional serialize/state features
- Verified license evidence: MIT OR Apache-2.0 (manifest); https://github.com/simgine/bevy_enhanced_input/blob/deaddeca88f2cab1ae249e74c4b5752ca3ede151/LICENSE-MIT
- Reuse/media boundary: No source/media imported
- Compatibility: 0.27.2 requires Bevy 0.20; not compatible with current engine
- Maintenance/risk: Maintained upstream; native controller validation still needed
- Decision and actual incorporation: Defer; small keyboard action/edge boundary uses Bevy built-ins

## R11 - RustAudio/cpal

- URL: https://github.com/RustAudio/cpal
- Author/organization: RustAudio contributors
- Revision/version inspected: `0.15.3 crates.io; Cargo.lock and vendor/cpal`
- Investigation date: 2026-10-30
- Classification: **ADAPTED SOURCE**
- Purpose: Cross-platform audio backend via Rodio/Bevy
- RoadDash parts: P01/P15
- Relevant inspected material: Existing ALSA poll underrun prepare/restart patch
- Verified license evidence: Apache-2.0; https://github.com/RustAudio/cpal/blob/v0.15.3/LICENSE
- Reuse/media boundary: Source reused with full vendor license; no assets
- Compatibility: Compatible with Bevy audio 0.17/Rodio 0.20; included patch local
- Maintenance/risk: Old pinned backend; unpatched PipeWire poll error loop observed in prior session
- Decision and actual incorporation: Retain existing adapted source and ROADDASH-PATCH.md; no new upstream code copied

## R12 - MrLixm/AgXc

- URL: https://github.com/MrLixm/AgXc
- Author/organization: MrLixm / referenced Troy Sobotka
- Revision/version inspected: `898198e0490b0551ed81412a0c22e0b72fffb7cd`
- Investigation date: 2026-10-30
- Classification: **REJECTED**
- Purpose: Referenced source of bundled AgX LUT
- RoadDash parts: P00 rights/P15 color
- Relevant inspected material: README and recursive tree at referenced revision
- Verified license evidence: UNVERIFIED: no license file or README grant located; https://github.com/MrLixm/AgXc/tree/898198e0490b0551ed81412a0c22e0b72fffb7cd
- Reuse/media boundary: No imported media permitted on this evidence
- Compatibility: Not a code dependency; LUT optional in Bevy
- Maintenance/risk: Historical snapshot; current license status not inferred
- Decision and actual incorporation: Disable tonemapping_luts and use mathematical Reinhard; no assets incorporated

## R13 - h3r2tic/tony-mc-mapface

- URL: https://github.com/h3r2tic/tony-mc-mapface
- Author/organization: Tomasz Stachowiak
- Revision/version inspected: `909e51c8a74251fd828770248476cb084081e08c`
- Investigation date: 2026-10-30
- Classification: **REFERENCE ONLY**
- Purpose: Alternative LUT referenced by Bevy
- RoadDash parts: P00/P15
- Relevant inspected material: LICENSE-MIT and repository tree
- Verified license evidence: MIT (verified text); Apache alternative present; https://github.com/h3r2tic/tony-mc-mapface/blob/909e51c8a74251fd828770248476cb084081e08c/LICENSE-MIT
- Reuse/media boundary: MIT notice required if used; nothing imported in this update
- Compatibility: LUT disabled with optional Bevy LUT feature
- Maintenance/risk: Historical snapshot; verify future selected files separately
- Decision and actual incorporation: Defer; no separate LUT needed for current baseline

## Deferred candidates (UNVERIFIED; no new inspection/adoption)

- R05 https://github.com/cBournhonesque/lightyear - no selected revision; P10/P13
  authority/prediction investigation deferred; no networking code used.
- R07 https://github.com/djeedai/bevy_hanabi - no selected revision; optional P15
  effects/performance investigation deferred; no code/assets used.
- R09 https://github.com/NiklasEi/bevy_kira_audio - no selected revision; optional
  P15 mixing investigation deferred because current built-in audio works.

Do not convert these candidates to dependencies or assert file-level permission
without recording an exact inspected revision, license URL and compatibility.

## Part 02 follow-up inspection — 2026-10-30

No new repository/dependency adopted. Revisited the already recorded R08 Bevy
v0.17.3 and R10 bevy_rapier v0.32.0 snapshots and installed source: indexed mesh
construction/normals, triangle colliders, generic/revolute joint locked axes,
force-based spring motors, force bounds, ray normals and friction combine rules.
Official guide: https://rapier.rs/docs/user_guides/bevy_plugin/joints/ and
https://www.rapier.rs/docs/user_guides/bevy_plugin/colliders/ (current guides,
installed version source is the compatibility authority). Engine MIT/Apache and
adapter Apache notices remain unchanged. Decision: retain existing dependencies;
independently implement spline/terrain/gate rules and original graybox geometry.
No upstream example, game implementation or third-party media copied. Existing
pinned versions and later coordinated-migration risk remain as recorded above.

## R03 - blyfoten/driftmaster (Part 03 investigation, 2026-10-30)

- URL/author: https://github.com/blyfoten/driftmaster — blyfoten.
- Exact revision inspected: `a556ec8ade723c0af921c58859005f5e45a32fef` (HEAD
  resolved with git ls-remote); README/root listing inspected.
- Classification: **REFERENCE ONLY**; reuse permission **UNVERIFIED**.
- Purpose: Bevy vehicle demonstration; README describes four wheel raycasts,
  spring/damper suspension, grip-circle forces, torque/RPM/gears and telemetry.
  Relevant to P03 chassis/contact/diagnostic design.
- License evidence: no license grant located in the inspected root/README;
  https://github.com/blyfoten/driftmaster/tree/a556ec8ade723c0af921c58859005f5e45a32fef
  is evidence of the inspected snapshot, not a license URL/grant. Code reuse is
  not cleared. Assets have no independently verified permission; none imported.
- Compatibility/maintenance: README targets Bevy 0.14, whereas RoadDash is
  0.17.3/Rapier 0.32.0. Current API compatibility and maintenance are unproven;
  direct integration would require an audit/migration.
- Decision/actual use: study the high-level wheel-ray/grip-budget approach only.
  Independently implement a compact two-wheel arcade adapter on pinned APIs.
  No source, examples, algorithms expressed as code, or media copied/adapted.

Part 03 revisited installed R08/R10 APIs. Current official vehicle guide
https://rapier.rs/docs/user_guides/bevy_plugin/vehicle_controller/ describes a
controller not exposed by installed bevy_rapier3d 0.32.0's control module (which
contains the character controller). Installed source determines compatibility.
Use existing ray/force/torque APIs instead of changing the physics dependency.
Existing dependency licenses/notices remain unchanged; no new dependency adopted.

## R14 — Gilrs native gamepad backend

- URL: https://gitlab.com/gilrs-project/gilrs ; author: Mateusz Sieczko and Gilrs
  developers. Classification: **DEPENDENCY**, through Bevy's `bevy_gilrs` feature.
- Inspected 2026-10-30: locked gilrs 0.11.2, gilrs-core 0.6.8, both crate VCS
  metadata point to `07e286e24b046cf39e5c367daa2770b805a64692`.
- Purpose/parts: native discovery, connect/disconnect, normalized gamepad state,
  platform mapping; Part 04 input. Existing Bevy 0.17.3 API source inspected.
- License verified: MIT OR Apache-2.0; exact upstream texts:
  https://gitlab.com/gilrs-project/gilrs/-/blob/07e286e24b046cf39e5c367daa2770b805a64692/LICENSE-MIT
  and https://gitlab.com/gilrs-project/gilrs/-/blob/07e286e24b046cf39e5c367daa2770b805a64692/LICENSE-APACHE. Local copies in licenses/gilrs-*.
  Reuse permitted with notices; no examples/source adapted into RoadDash files.
- Assets: bundled SDL mapping database has separate Zlib notice (R15); no game art,
  sounds or example PNGs imported. The crate build filters mappings per platform.
- Compatibility/maintenance: selected by Bevy 0.17.3, verified locally against the
  locked API; pinned crate snapshot, not an assertion of latest release health.
  Linux adds libudev development dependency; physical device/OS permission and
  Windows hardware behavior need acceptance beyond synthetic component tests.
- Decision/actual use: adopt via compatible engine feature; no custom HID backend.
  Keep fixed-input buffering/normalized strengths in our adapter. R06's newer
  input dependency remains deferred; avoid a Bevy migration for this requirement.

## R15 — mdqinc/SDL_GameControllerDB

- URL: https://github.com/mdqinc/SDL_GameControllerDB ; author/organization:
  mdqinc/community; license credits Sam Lantinga. **DEPENDENCY** mapping data via
  Gilrs (not game media or adapted gameplay source).
- Inspected 2026-10-30 exact gitlink `cbdb3678270cf5bf14a127934fd7ab332ecd3cbc`,
  resolved from R14 repository tree; packaged gamecontrollerdb.txt byte-compared
  to that exact upstream revision. README/license/build filtering inspected.
- Purpose/parts: controller button/axis mappings across devices; P04 discovery.
- Verified license: Zlib; https://github.com/mdqinc/SDL_GameControllerDB/blob/cbdb3678270cf5bf14a127934fd7ab332ecd3cbc/LICENSE
  copied to licenses/SDL-GameControllerDB-Zlib.txt. Redistribution permitted with
  notice; do not misrepresent origin or changes. ASSETS.csv records source hash.
- Compatibility/maintenance: exact bundled data selected by Gilrs; no claim every
  listed physical device works on our host. Future database updates need hash/
  license review. No separate rights grant inferred for README example images.
- Actual incorporation: unmodified package mapping source, upstream build filters
  target-platform lines into its embedded database. No mappings manually copied
  into game content, and no source PNGs/media used. Adopt selected snapshot.

## R01 handling follow-up — 2026-10-30

- URL/author: https://github.com/arkits/road-rash — arkits (Archit Khode).
- Exact new snapshot: `4e052aca9552a5690c2b145b0f53856d5c31e980` (HEAD resolved
  through GitHub API). The earlier README-only inspection remains historical.
- Classification: **REFERENCE ONLY**. Investigated bike.cpp/bike.hpp, input.cpp,
  race.cpp/race.hpp, bike tests, README and REVERSE_ENGINEERING.md; exact source
  links/functions and findings in ../research/ROAD_RASH_HANDLING.md.
- Purpose/parts: persistent steering force/rates, drive/brake/gears/traction/nitro,
  speed and motion; P03/P04. Confirmed holding turn builds force and release/
  reversal unwinds it; bank is steering-derived, not directly road curvature.
- License evidence: no project-wide license file/grant found in inspected root or
  README. https://github.com/arkits/road-rash/tree/4e052aca9552a5690c2b145b0f53856d5c31e980
  documents the inspected tree, not permission. third_party/LICENSE is Sean
  Barrett's third-party notice and does not grant reuse of the port's own source.
  Source reuse permission remains UNVERIFIED. No C++ source/examples copied.
- Assets/data: actual tuning loads BIKESPEC.RSC and ROADRASH.EXE tables, absent
  from public repo and this workspace; user confirmed no original files. No disc,
  extracted table, media or protected data incorporated. Synthetic tables are
  explicitly tests, not original tuning; never present them as calibration.
- Compatibility/maintenance: C++20/SDL3, not a Rust/Bevy dependency. Recent source
  changes observed; repository parity/maintenance claims not independently run.
- Decision/actual use: independently implement functional normalized steering
  memory and record precise incomplete equivalence gate. Keep modern 3D physics,
  collision/recovery and current longitudinal profiles; no claim they now match
  the entire original. No new dependency, adapted source or reference media.

### 2026-10-30 base-world foundation follow-up

No additional repository was investigated or adopted for B00–B02. The supplied
world plan's Road Rash/source links are research suggestions, not newly verified
findings or a license grant. Current local serde/Bevy/Rapier infrastructure was
sufficient for schema, census and district preview. No external code/assets or
new dependencies were incorporated. User-provided design input provenance is
recorded in docs/world/source/MANIFEST.json and docs/world/BASE_WORLD.md.

### R08 follow-up — structural world visibility, 2026-10-30

Inspected the installed Bevy 0.17.3 source corresponding to existing R08
v0.17.3 / 706fdd9800c4bf335d67f127ff6e5a93deadabb9:
`bevy_camera/src/visibility/range.rs` (VisibilityRange, paired margins,
use_aabb) and the existing mesh/visibility APIs. MIT OR Apache-2.0; existing
license URLs/notices and dependency audit apply. Classification: DEPENDENCY.
Adopted its API for near/far building and terrain visibility; no example code or
media copied, no dependency/version change. Current engine supports it directly;
no external procedural-city/navigation library was necessary. Risk: visual
ranges must not remove collision; root ownership and native visible-part counts
are verified in world/COMPLETION_TESTS.md. A future engine migration must recheck
range/shadow behavior and rendering order rather than assume compatibility.

B09 verification also inspected the installed bevy_winit 0.17.3
`winit_config.rs`: unfocused game windows default to a 60 Hz reactive idle timer.
The native survey uses Continuous updates only for benchmarking and records
actual focus/resolution. Normal play retains its existing power policy. Paired
VisibilityRange meshes now use the exact same local origin (including terrain
tiles); a native atlas exposed the prior world-origin error and a regression
asserts normalized vertices and paired transforms. No upstream example copied.

### R01 city-spacing follow-up — 2026-10-31

Revisited arkits/road-rash (author arkits), exact commit
`4e052aca9552a5690c2b145b0f53856d5c31e980`: resource, course, road, terrain and
course_scene readers. REFERENCE ONLY. Verified the supplied disc against the
upstream importer table (blob `7ab9f9524c1281fe6b4e040e04e4744e671ae732`, 341/341)
and independently measured City resource lane/shoulder endpoints. See
../research/CITY_PROPORTIONS.md. No C++ copied, no dependency added, no assets
incorporated. The earlier unresolved project-wide source license remains
UNVERIFIED; public source availability does not establish reuse permission.
Native C++/SDL is not directly compatible with Rust/Bevy; only format/behavior
research is used. This snapshot is historical, not a claim of current maintenance
or complete game parity. Proprietary disc/media remain local and outside Git.

CITY-02, 2026-10-31: reused the above R01 course_scene facade-strip findings for
touching city fronts; independent shared-edge 3D infill, no additional repository,
C++ source or original media incorporated. Evidence: ../testing/FRONTAGES.md.

### R01 rider/combat/pursuit follow-up — 2026-10-31

Revisited arkits/road-rash, author arkits, exact commit
4e052aca9552a5690c2b145b0f53856d5c31e980. REFERENCE ONLY. Reviewed race.cpp combat
and pursuit, race_setup.cpp names/bike slots, bob.cpp remaps, rider_sprites.cpp
and pe_strings.cpp. License/reuse permission remains UNVERIFIED, per existing R01;
https://github.com/arkits/road-rash/tree/4e052aca9552a5690c2b145b0f53856d5c31e980.
No code/assets incorporated. Native C++ is not a Rust/Bevy dependency; independent
rules and procedural materials avoid compatibility issues. Local pinned snapshot
does not establish current upstream maintenance. Proprietary disc assets retain
separate restrictions and remain outside runtime/repository. Findings, decisions
and limits: research/RIDERS_AND_KICKS.md. Adopt behavior reference only.

### R01 individual-personality follow-up — 2026-10-31

Same pinned arkits/road-rash commit 4e052aca9552a5690c2b145b0f53856d5c31e980,
author arkits. Inspected race.cpp personality/stat readers, relationship scores,
reactions, threshold state changes and attack selection. REFERENCE ONLY; no source
copied, no Bevy dependency. Existing license remains UNVERIFIED for implementation
reuse; license evidence URL and maintenance/compatibility limitations remain as
recorded above. Local numeric observations inform independently authored profiles;
no executable/media shipped. Adopt behavior research; defer all-career/random/
three-axis parity to AI-03. Evidence: testing/PERSONALITIES.md and ADR-026.

### R08 authored skin integration — 2026-10-31

Consulted the installed Bevy **0.17.3** mesh/skinning types, StandardMaterial image
settings and custom_skinned_mesh example API. Repository: bevyengine/bevy,
https://github.com/bevyengine/bevy/tree/v0.17.3; Bevy contributors. Existing
DEPENDENCY classification and dual MIT/Apache-2.0 licenses remain unchanged:
https://github.com/bevyengine/bevy/blob/v0.17.3/LICENSE-MIT and LICENSE-APACHE.
No new dependency or source/media import. The adapter is independently written;
it uses existing API types rather than copying the tutorial mesh. Compatibility
is verified against the pinned lockfile, not a new upstream version. Existing
shader/renderer and platform risks apply. Adopt existing GPU skinning; keep
Rapier bodies and joint/motor settings unchanged. The model assets themselves
come from the earlier independent authoring step, not the Bevy repository.

### Motion/active-ragdoll comparison — 2026-10-31

#### sergioabreu-g/active-ragdolls

- URL/version: https://github.com/sergioabreu-g/active-ragdolls/tree/459cbab9489c7481e65bf16ac93fcfff98e6a594; exact inspected commit `459cbab9489c7481e65bf16ac93fcfff98e6a594`; author/organization `sergioabreu-g`.
- Investigation: 2026-10-31, README/license/API declarations; no cross-engine runtime benchmark claimed.
- Classification: **REFERENCE ONLY**. License: Apache-2.0 verified from LICENSE. Evidence: https://github.com/sergioabreu-g/active-ragdolls/blob/459cbab9489c7481e65bf16ac93fcfff98e6a594/LICENSE.
- Permission/actual incorporation: Conceptual reuse permitted under Apache-2.0 with required notices; no source reused. No assets incorporated; demo/sample/media rights are separate and unverified.
- Purpose/useful systems: Study modular target/physical-body pose following; compare capped torque, not wholesale rig replacement.
- Compatibility/risk: Unity 2020.1.9f1, no direct Bevy compatibility; old sample.
- Maintenance snapshot: archived=False; last push observed `2021-02-02T14:51:32Z`. This is a dated observation, not a maintenance guarantee.
- Decision: study/evaluate only; implementation prototypes, compatible lockfile tests and visual/physics/performance comparison are V2-02/V2-03 prerequisites.

#### hairibar/Hairibar.Ragdoll

- URL/version: https://github.com/hairibar/Hairibar.Ragdoll/tree/ba7488ad7abe86dc94ae10c5e8e25dc9c1688366; exact inspected commit `ba7488ad7abe86dc94ae10c5e8e25dc9c1688366`; author/organization `hairibar`.
- Investigation: 2026-10-31, README/license/API declarations; no cross-engine runtime benchmark claimed.
- Classification: **REFERENCE ONLY**. License: MIT verified from LICENSE.md. Evidence: https://github.com/hairibar/Hairibar.Ragdoll/blob/ba7488ad7abe86dc94ae10c5e8e25dc9c1688366/LICENSE.md.
- Permission/actual incorporation: Code reuse permitted with MIT notice; no source reused. No assets incorporated; demo/sample/media rights are separate and unverified.
- Purpose/useful systems: Study separate animation targets and physical bones, position/rotation matching and safe blending; avoid unbounded position drives.
- Compatibility/risk: Unity/C#, no direct Bevy compatibility; historical package.
- Maintenance snapshot: archived=False; last push observed `2021-04-21T07:19:57Z`. This is a dated observation, not a maintenance guarantee.
- Decision: study/evaluate only; implementation prototypes, compatible lockfile tests and visual/physics/performance comparison are V2-02/V2-03 prerequisites.

#### jrouwe/JoltPhysics

- URL/version: https://github.com/jrouwe/JoltPhysics/tree/ea4599d5044d92ba91b18664ec1df96255d5eb31; exact inspected commit `ea4599d5044d92ba91b18664ec1df96255d5eb31`; author/organization `jrouwe`.
- Investigation: 2026-10-31, README/license/API declarations; no cross-engine runtime benchmark claimed.
- Classification: **REFERENCE ONLY**. License: MIT verified from LICENSE. Evidence: https://github.com/jrouwe/JoltPhysics/blob/ea4599d5044d92ba91b18664ec1df96255d5eb31/LICENSE.
- Permission/actual incorporation: Core code reuse permitted with MIT notice; no source reused. No assets incorporated; demo/sample/media rights are separate and unverified.
- Purpose/useful systems: Study powered-rig constraints/motors and stability cases; defer backend adoption and retain current Rapier.
- Compatibility/risk: C++ physics engine, not a Bevy/Rapier plugin; FFI/migration cost.
- Maintenance snapshot: archived=False; last push observed `2026-10-31T20:58:23Z`. This is a dated observation, not a maintenance guarantee.
- Decision: study/evaluate only; implementation prototypes, compatible lockfile tests and visual/physics/performance comparison are V2-02/V2-03 prerequisites.

#### Kurble/bevy_mod_inverse_kinematics

- URL/version: https://github.com/Kurble/bevy_mod_inverse_kinematics/tree/5c037a33b55e8056ec24222d2968972f59e009d5; exact inspected commit `5c037a33b55e8056ec24222d2968972f59e009d5`; author/organization `Kurble`.
- Investigation: 2026-10-31, README/license/API declarations; no cross-engine runtime benchmark claimed.
- Classification: **UNVERIFIED**. License: README and Cargo declare MIT OR Apache-2.0; root license files/API lookup unresolved. Evidence: https://github.com/Kurble/bevy_mod_inverse_kinematics/tree/5c037a33b55e8056ec24222d2968972f59e009d5/README.md.
- Permission/actual incorporation: Do not incorporate until compatible release grant/archive verified. No assets incorporated; demo/sample/media rights are separate and unverified.
- Purpose/useful systems: Evaluate compatible positional/pole IK for terrain feet/bike contacts later; no dependency installed.
- Compatibility/risk: Inspected HEAD is 0.12 / Bevy 0.19; README maps Bevy 0.17 to 0.10, not yet build-tested.
- Maintenance snapshot: archived=False; last push observed `2026-09-01T11:26:27Z`. This is a dated observation, not a maintenance guarantee.
- Decision: study/evaluate only; implementation prototypes, compatible lockfile tests and visual/physics/performance comparison are V2-02/V2-03 prerequisites.

#### guillaumeblanc/ozz-animation

- URL/version: https://github.com/guillaumeblanc/ozz-animation/tree/744eb9d99f606eda849acb0b1204f7a3dc20bca1; exact inspected commit `744eb9d99f606eda849acb0b1204f7a3dc20bca1`; author/organization `guillaumeblanc`.
- Investigation: 2026-10-31, README/license/API declarations; no cross-engine runtime benchmark claimed.
- Classification: **REFERENCE ONLY**. License: MIT core grant manually verified in LICENSE.md (GitHub classifier NOASSERTION). Evidence: https://github.com/guillaumeblanc/ozz-animation/blob/744eb9d99f606eda849acb0b1204f7a3dc20bca1/LICENSE.md.
- Permission/actual incorporation: Core code reuse permitted under read MIT grant; no source reused. No assets incorporated; demo/sample/media rights are separate and unverified.
- Purpose/useful systems: Study blending/IK/compression patterns; defer FFI/runtime adoption in favor of current engine tools.
- Compatibility/risk: C++ animation sampling/blending/offline toolchain; not a drop-in Bevy dependency.
- Maintenance snapshot: archived=False; last push observed `2026-08-01T21:34:54Z`. This is a dated observation, not a maintenance guarantee.
- Decision: study/evaluate only; implementation prototypes, compatible lockfile tests and visual/physics/performance comparison are V2-02/V2-03 prerequisites.

R01 follow-up: local arkits/road-rash `cd5504caea48aee59806dbfd11c920f33df21200`, race.cpp traffic separation/contact-car path inspected for both-object velocity/state handling. REFERENCE ONLY; no source copied and existing unresolved reuse permission remains. Original units do not establish modern SI crash equivalence. See research/ANIMATION_AND_RAGDOLL.md.

### Animation A source follow-up — 2026-10-31

Revisited the five pinned animation/ragdoll candidates above, with unchanged SHAs,
licenses and adoption classifications. Inspected active-ragdolls AnimationModule,
PhysicsModule and joint-target conversion; Hairibar AnimationMatching and target/
profile hooks; Jolt PoweredRigTest; Bevy IK lib/solver; ozz foot_ik and two-bone
runtime/sample sources. Concrete comparison and source links are in
[ANIMATION_AND_RAGDOLL.md](../research/ANIMATION_AND_RAGDOLL.md#part-02-source-level-comparison--2026-10-31).
No source or sample media incorporated; no backend or dependency adopted. There
are no compiled comparative performance results. Per-joint motor cost, iteration
budgets, probe counts and energy/contact behavior require the Part 03 prototypes.
The unresolved Bevy IK license-file evidence and HEAD incompatibility remain.

R01 local snapshot cd5504caea48aee59806dbfd11c920f33df21200 was revisited for
sprite/helper anchors, view/descriptor separation and crash/get-up transitions.
All 326 HIBOB and 45 LOCOP slots were inventoried from the local disc as numeric
metadata. No source or proprietary pixels were imported. See animation reference
review. Native CANS/RRAN/EXE timing/sequence linkage remains unverified.

### R01 context correction follow-up — 2026-11-01

arkits/road-rash, arkits, https://github.com/arkits/road-rash, pinned
cd5504caea48aee59806dbfd11c920f33df21200, REFERENCE ONLY / incorporation rights
UNVERIFIED (no root license grant verified; https://github.com/arkits/road-rash/tree/cd5504caea48aee59806dbfd11c920f33df21200).
Read actor_descriptors.cpp/.hpp, actor_animation.cpp/.hpp, actor_nodes.cpp,
actor_render.cpp, animation.cpp, resource.cpp, exe_tables.cpp, race_setup.cpp,
race.cpp, crash.cpp and descriptor tests. Its stand-in attack_frame helper is
explicitly inspection-based and superseded as an action authority. Actual
EXE selector arrays, relocated descriptors, CANS groups, RRAN/BOB index mapping,
view flags/mirroring and playback controls were traced instead.

The library was compiled temporarily outside the project to cross-check our new
independent Python metadata reader: 194 descriptors and rear selections agree.
No C++/media copied into RoadDash, no dependency/engine adoption. Numeric binary
layout facts and observed mappings are recorded; no measured native Windows parity
is claimed. Runtime/Bevy compatibility is irrelevant to the offline research
executable. Maintenance snapshot unchanged; original files are user-supplied local
research only and not redistributed. Full corrected findings: animation/context.

### R01 full-body/tuck follow-up — 2026-11-01

Same arkits/road-rash commit cd5504caea48aee59806dbfd11c920f33df21200
(arkits; https://github.com/arkits/road-rash). Inspected animation selector and
hud.cpp numeric_speed for low-speed/acceleration/tuck boundaries and the unrounded
41/4096 mph coefficient. Source ordered attack phases remain cataloged separately.
REFERENCE ONLY; no source/media incorporated. No root license grant verified;
source reuse remains UNVERIFIED, public availability is not permission. Existing
license/maintenance risks unchanged. Native C++ is not a Rust/Bevy dependency;
independently tested domain rules are compatible with this pinned baseline.
Decision: study behavior, original visual transforms/clock/contact math in RoadDash;
defer exact legacy mode/boost flag mapping and original-executable timing evidence.

### R01 original-crash comparison — 2026-11-01

- Repository/author: arkits/road-rash, https://github.com/arkits/road-rash.
- Exact inspected commit: aa9cb6d3181e20736af71022cd4d6aac0e2788d1,
  dated 2026-10-31, 687 commits at investigation. Active reconstruction snapshot;
  some synthetic helpers differ from native motion fields, so comments alone
  are not executable evidence.
- Classification **REFERENCE ONLY**, source reuse **UNVERIFIED**. No root license
  grant verified (root README still leaves licensing undecided); third-party
  subdirectory licenses do not license the reconstruction. License URL: none
  verified. Assets are original-game media with separate unresolved redistribution
  rights; none incorporated. No C++ copied and no source dependency adopted.
- Studied Race separation/contact/knock_off/vertical, crash entry/recovery,
  bike drag and object templates. Useful for original crash mechanics only;
  C++/SDL is not a Rust/Bevy-compatible plugin. Independently modeled mathematical
  rules in rr_core; native EXE hash/address facts and numeric tests corroborate
  selected behavior. No full port/original gameplay parity claim.
- Decision: study behavior; follow retail signed velocity over scalar car helper;
  opt-in 3D adapter with explicit remaining parity gates. Exact formulas and
  code/retail provenance: docs/research/ORIGINAL_CRASH_CALCULATIONS.md.

### R24 — Unicorn optional verification tool — 2026-11-01

- URL/organization: https://github.com/unicorn-engine/unicorn, Unicorn Engine.
- Inspected release 2.1.4, tag commit 8028ec436f2d9376525352dd38ed9ed6b9f6be10.
- Classification **DEPENDENCY (optional research tool only)**. Python package
  unicorn==2.1.4 installed into a disposable /tmp venv; no runtime Cargo dependency,
  source copy, vendored binary, media or assets incorporated into RoadDash builds.
- Purpose/actual use: isolated x86 instruction execution against the user's local
  retail EXE to verify Rust crash arithmetic; 23 bounded checks. Techniques:
  PE mapping, instruction budgets, register/memory setup and one surface-selector
  stub. No full-game emulation or proprietary payload redistribution.
- License verified: engine COPYING GPL-2.0,
  https://github.com/unicorn-engine/unicorn/blob/2.1.4/COPYING. Python package
  metadata says BSD License; that is not a GPL waiver for the native engine.
  Reuse/distribution must follow the applicable component license; no source reused
  or runtime redistribution attempted. No associated media used.
- Compatibility: external Python3 tool, independent of Rust1.94/Bevy0.17.3/Rapier;
  selected wheel works on this Linux host. Release source is maintained; risks are
  optional installation, architecture support, precise EXE address/hash dependency
  and interpreting bounded results as whole-game parity.
- Decision: adopt only for explicitly invoked local research verification; retain
  dependency-free Rust rules/tests for normal builds and CI.

### R01 facade kit follow-up — 2026-11-01

Inspected same pinned aa9cb6d3181e20736af71022cd4d6aac0e2788d1 (arkits,
https://github.com/arkits/road-rash), family.cpp, building_panels.cpp and
course_scene.cpp building-group resolution. REFERENCE ONLY, root reuse license
still UNVERIFIED; no source code or proprietary images incorporated. Useful
concepts: selector family/category mapping, size variants, separate facade/roof/
end/side-street construction. C++/SDL compatibility is irrelevant to our offline
independent Python grammar; Bevy dependencies unchanged. Existing maintenance and
license risks remain. Local prior extraction supplies 39 inherited named city
records, whose completeness is unverified. Decision: implement original geometric
kit with explicit dimensions; defer texture authoring and runtime replacement.

### CITY-REF-01 — phiresky/procedural-cities (REFERENCE ONLY, 2026-11-01)

- URL: https://github.com/phiresky/procedural-cities ; author: phiresky (Robin).
  Inspected exact commit f0c2c86e4289cea63e05fb55a78994c5a8ef99f5.
- Read paper.md, implementation/src/mapgen.ts and config.ts, README and license.
  Demonstrates road growth, population-driven branching, priority queues,
  quadtree intersection tests and road snapping; paper contrasts street growth
  with sequential building subdivision. Useful for future irregular districts.
- Verified root LICENSE.md: GNU AGPL v3 default license,
  https://github.com/phiresky/procedural-cities/blob/f0c2c86e4289cea63e05fb55a78994c5a8ef99f5/LICENSE.md .
  Reuse requires applicable AGPL terms; not adopted into MIT runtime. mapgen.ts
  also notes adaptation from tmwhere.com; upstream provenance needs review before
  any reuse. Separate image/media restrictions not verified; no assets copied.
- Compatibility: TypeScript/browser demo, no Rust/Bevy integration. Source includes
  old webpack/yarn setup and explicit TODOs; current build not tested. Maintenance
  and algorithm completeness are not presumed. No code or dependencies adopted.
- Decision: study only; a controlled grid and exact lot partition are simpler
  for this fixed avenue. Preserve road-right-of-way logic in our own generator.

### Building-model tooling / existing Bevy follow-up — 2026-11-01

No new repository source adopted. Used existing local Blender4.5.1 LTS binary,
build b0a72b245dcf (2025-07-29); inspected bundled copyright.txt (GPL2-or-later,
with individual component license inventory), bpy/runtime version and stock
glTF export/import through execution. Official source: https://projects.blender.org/blender/blender/src/tag/v4.5.1 ;
license: https://www.blender.org/about/license/ . Tool is external, not shipped
with game; no Blender/Cycles/add-on source or bundled media copied. Own original
Python generator/geometry/material code and standard GLB output only.

Inspected installed Bevy0.17.3 scene_spawner.rs static SceneRoot change/despawn
behavior and glTF loader components plus reflection traits. Existing dependency
licenses MIT/Apache2.0 unchanged; standard API use only, no source copied.
Explicit reflection registration in isolated preview resolves omitted automatic
registration without a dependency migration. Stock static GLBs tested natively.


### R01 district/location follow-up — 2026-11-01

Revisited arkits/road-rash, exact aa9cb6d3181e20736af71022cd4d6aac0e2788d1:
https://github.com/arkits/road-rash/tree/aa9cb6d3181e20736af71022cd4d6aac0e2788d1 .
Read course.cpp MSRR events, course_scene.cpp family residency/horizon names,
family.cpp named texture groups, resource.cpp directory layout and explode.cpp
resource envelope. REFERENCE ONLY; existing root license/reuse permission remains
UNVERIFIED (no root license file in inspected snapshot), third_party/LICENSE is
not blanket project clearance. No code or media copied/adopted. No C++ dependency
or Rust/Bevy compatibility change. Maintenance assessment remains the pinned
snapshot; do not assume game-wide parity from family-reader findings.

Used the existing user-local city_source.py decompressor as an external research
tool, with its SHA in research/building-districts/source-associations.json; not
vendored or a build dependency. Independent RoadDash metadata scanner records
146 named CEL/family memberships and five-course load eligibility. Course family
loads are not proof of texture placement. Decision: use evidence for asset filters,
keep authored13 district names/roles explicit as interpretation. Original game
files remain local; no sprites, textures or binary resources redistributed.


### R01 whole-building assembly follow-up — 2026-11-01

Same arkits/road-rash aa9cb6d3181e20736af71022cd4d6aac0e2788d1;
https://github.com/arkits/road-rash/blob/aa9cb6d3181e20736af71022cd4d6aac0e2788d1/src/building_panels.cpp .
Inspected building_panels.cpp/.hpp facade/flat-roof/exposed-end logic, side-facade
runs and side-street curb/shoulder roles, plus course_scene family selection.
REFERENCE ONLY; existing unresolved root reuse license remains unchanged, no source
copied. Original is a2.5D projected panel renderer, not a complete3D city library.
Independently adapted the roles into our geometric front runs, shared whole-lot
roofs/rears/ends and reserved side-street returns. Exact original coordinates,
selector interleaving or visual parity are not claimed. Source media stays local;
our derivative front modules use only our own original generated GLBs and material
tiles. No new dependency/version/physics migration. Compatibility: current pinned
Bevy0.17.3/Rapier0.32.0; existing maintenance/license risks remain.

### R01 downed-contact follow-up — 2026-11-01

Inspected arkits/road-rash (arkits), commit
`aa9cb6d3181e20736af71022cd4d6aac0e2788d1`,
[race.cpp](https://github.com/arkits/road-rash/blob/aa9cb6d3181e20736af71022cd4d6aac0e2788d1/src/race.cpp):
Race::contact low/containment/bike/tall/glance routing, contact_static,
contact_half_static, and collide detached-actor dispatch. Low objects avoid the
normal knock_off path; detached actors have separate collision callbacks.
Decision: REFERENCE ONLY for independently written contact policy (ADR045).
Existing R01 license/reuse permission remains UNVERIFIED; no source or retail
assets incorporated and no permission to redistribute inferred. This old C++
reconstruction is not a Bevy/Rapier dependency; no Rust compatibility or retail
parity is claimed. Exact pinned source and uncertain reconstruction completeness
matter more than current maintenance activity. Existing native6001/damage and
height-unit comparison gates remain open.

### R01 all-prop reference follow-up — 2026-11-01

arkits/road-rash (arkits), aa9cb6d3181e20736af71022cd4d6aac0e2788d1:
[family.cpp](https://github.com/arkits/road-rash/blob/aa9cb6d3181e20736af71022cd4d6aac0e2788d1/src/family.cpp),
[course_scene.cpp](https://github.com/arkits/road-rash/blob/aa9cb6d3181e20736af71022cd4d6aac0e2788d1/src/course_scene.cpp),
animation.cpp and traffic.cpp inspected. Category/selector hierarchy, scenery RRAN,
actor category2, stored view/size frame bodies and separate course vehicle ANIM/CANS
resources inform independently written metadata-only extraction/review tools.
REFERENCE ONLY; no C++ copied, no dependency or original media incorporated.
Existing unresolved root reuse/license status stays UNVERIFIED. The user's external
local decoder is called for decompression, not vendored; its hash is recorded.
These are data-layout observations, not guaranteed reconstruction/retail parity.
No Rust/Bevy compatibility claim needed for an offline source-reference investigation.
See ADR046 and docs/research/props/README.md; maintenance/completeness risk remains.


2026-11-01 PROP-03: reused existing RoadDash original draft geometry, Bevy0.17.3
scene/skin loading and current Rapier0.32.0 owners. No new repository inspected,
dependency adopted, reference source copied or original retail media incorporated.
See ADR048 and testing/props-in-game/README.md; existing research decisions stand.

## Biomechanical ragdoll investigation — 2026-11-01

Actual source and license files inspected for all five requested repositories;
[full source/version/decision register](../research/RAGDOLL_BIOMECHANICS.md).
All classified REFERENCE ONLY, with no code or assets incorporated:

- bhubbard/active-ragdoll-rs (Brandon Hubbard), a2fc2f1ad8c08590c4571d5b731ac478f1a29e78,
  MIT OR Apache-2.0. Binding/target/control architecture and Rapier adapter studied;
  Rapier 0.36/glam 0.33 do not match installed Rapier 0.31/Bevy 0.17.3.
- ashleve/ActiveRagdoll (ashleve; license copyright Lucas Z.),
  749cd80dbccd73dffb7bb1d3827ad0e719b84f1c, MIT. Strength states and configurable
  joint drives studied; Unity APIs not directly compatible. Last commit 2023-03-05.
- sergioabreu-g/active-ragdolls (Sergio Abreu),
  459cbab9489c7481e65bf16ac93fcfff98e6a594, Apache-2.0. Joint-space conversion and
  modular target/physics separation studied; Unity-only example, last commit 2021-02-02.
- AshimPoudel7/ProceduralLocomotion (AshimPoudel7),
  8c866b44671732faa57a6df2c6cd0a52a1bc81d1. License UNVERIFIED (no license file at
  inspected root). Behavioral study only; torque/perturbation source not copied.
- pemmyz/js_rapier_ragdoll_v1 (pemmyz),
  7b84ece887a0fcf2644fa1949f486fdcfe7da7f0, MIT. Capsule/hinge construction studied;
  unconstrained spherical shoulders rejected for this requirement. Browser JS,
  not a Rust dependency; last commit 2026-04-13.

Exact URLs, license URLs, editor/library versions, maintenance dates, risks and
actual source paths are in the linked register. All third-party media excluded;
source permission is not a blanket asset grant. Selected an original native Rapier
integration to preserve existing crash/animation/recovery rather than adopting a
new physics wrapper or copying Unity controllers. No package additions.


2026-11-06 integration: reused the five previously pinned ragdoll investigations
and existing Bevy/Rapier dependencies. No new repository inspected or source/assets
copied. ADR049 and testing/ragdoll/integration/README.md record current integration.


2026-11-06 Part03: consulted existing pinned animation/ragdoll investigations
(recorded source-level findings in research/ANIMATION_AND_RAGDOLL.md); no new
repository investigation, dependency, copied source or media. Existing native
FK/two-link fitting and capped Rapier motors were extended in original code;
ADR050 records why no incompatible IK/animation wrapper was introduced.

2026-11-06 camera/contact/gait follow-up (ADR051): no new repository researched
or third-party source/media incorporated. Extended existing original FK/IK,
pre-contact response and Rapier motors; no dependency migration. Normal-speed
sharing and kinetic roll budgeting are original elementary mechanics code.

2026-11-06 Part04 Valley architecture (ADR052): reused the project's existing
original mesh/material generators and previously recorded Bevy 0.17.3/glTF
integration. Consulted installed Bevy scene-ready event source for correct API
use. No new repository investigated, dependency adopted, third-party algorithm
source copied or new external media imported. The storey compiler, clipping,
manifest validation, fixed-width variant selection and renderer are original
RoadDash code. Existing catalogs retain their own research/license records.

2026-11-06 surface/loading/performance follow-up (ADR053): inspected installed
Bevy0.17.3 task_pool_plugin.rs (default25%,1–4 IO assignment and get_or_init) and
bevy_tasks TaskPoolBuilder::stack_size, plus existing scene readiness APIs.
Existing pinned Bevy repository/license record applies; no new repository/version,
code copying, dependency or third-party media adoption. Original procedural asphalt,
elevation meshes, scene staging and user-requested crash approximation implemented.

### 2026-11-07 — Part05 follow-up: existing pinned Bevy APIs

No new repository or dependency adopted. Inspected installed Bevy0.17.3 source:
visibility-range dither, SceneInstanceReady, image cube views/mip layout,
EnvironmentMapLight/Skybox, Mesh::generate_tangents and async task polling. Existing
Bevy MIT/Apache-2.0 dependency/license record applies. Application code is original;
no upstream code or external media copied. Palette profiles refer only to the34
already registered original building textures. Existing draft prop asset licenses
are inherited, with no proprietary reference pixels imported into runtime.

### 2026-11-07 — Roadside variety follow-up

Inspected the existing local369-record original prop catalog and semantic placement
source. No new GitHub repository, dependency or media researched/adopted. Reused
registered draft models and existing source/license records; grouped placement and
seeded variation are original project code. Parked vehicles are scenery only.

## Crash behavior source audit — 2026-11-07 (research only)

The user requested findings before any implementation. Full comparison, exact
source hashes and no-adoption evidence: [crash-behavior](../research/crash-behavior/README.md)
and [sources.json](../research/crash-behavior/sources.json). Nothing copied into
runtime, no new dependencies, no third-party assets imported or external demos run.

| Repository / author | Inspected commit / last commit date | License and reuse | Findings / compatibility / decision |
| --- | --- | --- | --- |
| [arkits/road-rash](https://github.com/arkits/road-rash), arkits |40829dbb2d30c5961cfd46c4fcf6373247126125 /2026-11-05 | Root code grant not found; upstream third_party notices do not license the main source. UNVERIFIED permission; behavioral research only, no copying. Retail media has separate restrictions. | C++/SDL3 reverse-engineered port. actor_animation.cpp, crash.cpp, race.cpp and bike.cpp: speed/height-selected crash cycles, motion integration, immediate decisive knock-off versus surviving steering reactions. REFERENCE ONLY; no Bevy dependency. No fresh retail parity replay; do not treat simplified paths or rendered sprites as a biomechanical simulation. |
| [bhubbard/active-ragdoll-rs](https://github.com/bhubbard/active-ragdoll-rs), Brandon Hubbard |f9fb31c4c1d705bbd6b48ed9447ed6563066c2b5 /2026-11-01 | [MIT](https://github.com/bhubbard/active-ragdoll-rs/blob/f9fb31c4c1d705bbd6b48ed9447ed6563066c2b5/LICENSE-MIT) OR [Apache-2.0](https://github.com/bhubbard/active-ragdoll-rs/blob/f9fb31c4c1d705bbd6b48ed9447ed6563066c2b5/LICENSE-APACHE), grants read; licensed source reuse possible with notice, none adopted. No media imported. |0.1.1, Rapier0.36/glam0.33.10 versus RoadDash Rapier0.31/Bevy glam0.30.10. Recent development, not production certification. Follower/PD/adapter inspected; linear animation pursuit, direct torque and zero limp drives conflict with current policies. REFERENCE ONLY; defer dependency, retain native constraint motors. |
| [DanilKhmyrov/godot-active-ragdoll](https://github.com/DanilKhmyrov/godot-active-ragdoll), Danil Khmyrov |bd212867145484aafb9393a0ccc2bfc3ec0befae /2026-11-05 | [MIT](https://github.com/DanilKhmyrov/godot-active-ragdoll/blob/bd212867145484aafb9393a0ccc2bfc3ec0befae/LICENSE), root/addon grants read; source reuse possible with notice, none adopted. Demo mannequin separately declares Quaternius CC0; other bundled media not comprehensively cleared, no assets imported. | Godot4/GDScript/Jolt-or-Godot, no native Bevy integration. Strongest behavioral reference: roll_over, orientation tests, directional braces, per-group tone and get-up alignment. Direct velocity/pin/transform corrections and friction changes require redesign for our momentum/work policy. Recent repo, unbenchmarked here. REFERENCE ONLY, study behavior rather than copy the solver adapter. |
| [sergioabreu-g/active-ragdolls](https://github.com/sergioabreu-g/active-ragdolls), Sergio Abreu García; joint helper credits Michael Stevenson |459cbab9489c7481e65bf16ac93fcfff98e6a594 /2021-02-02 | [Apache-2.0](https://github.com/sergioabreu-g/active-ragdolls/blob/459cbab9489c7481e65bf16ac93fcfff98e6a594/LICENSE), verified; source reuse possible subject to notices, none adopted. Sample assets require separate review, excluded. | Unity C#, no Bevy compatibility; older example. AnimationModule/PhysicsModule/ConfigurableJointExtensions studied. Useful target/physics separation and joint frames; stabilizer/frozen-rotation modes are unsuitable for uncontrolled crashes. REFERENCE ONLY, no engine substitution. |
| [ashleve/ActiveRagdoll](https://github.com/ashleve/ActiveRagdoll), ashleve; source license Lucas Z. |749cd80dbccd73dffb7bb1d3827ad0e719b84f1c /2023-03-05 | [MIT](https://github.com/ashleve/ActiveRagdoll/blob/749cd80dbccd73dffb7bb1d3827ad0e719b84f1c/LICENSE), verified; source reuse possible with notice, none adopted. No asset clearance inferred, none imported. | Unity C#, older example; SlaveController and AnimationFollowing inspected. Reuse strength transition concepts; reject world-position velocity chasing and disabling passive resistance for our crashes. REFERENCE ONLY. |

Maintenance descriptions above are dated source observations, not assurances of
upstream support. No license permission is inferred from GitHub visibility.

2026-11-07 Method B comparison: original tensor-aware reciprocal angular-impulse
controller and pose/state rules inspired by the five pinned source observations
above. No external source/media copied and no package adopted. Godot/Rust
repositories remain REFERENCE ONLY; native Rapier anatomy/passive drives retained.
Full world inverse inertia and exact per-pair positive work replace scalar/directed
velocity heuristics; no animation-position follower. See ADR058-B and Method B QA.

2026-11-07 facade follow-up: inspected installed bevy_camera/bevy_pbr0.17.3
visibility/range.rs, render/mesh_functions.wgsl and pbr_functions.wgsl (Cargo.lock
pinned crates, bevyengine/bevy, MIT OR Apache-2.0; existing DEPENDENCY). Confirmed
start/end complementary dither and origin semantics; no source copied or new
dependency/media. ADR060 and velocity-popin evidence record actual use.

### 2026-11-07 facade LOD authoring/runtime follow-up

Existing external Blender build tool:4.5.9 LTS, build8bf95cbd38d1 (2026-04-21),
from the previously acquired local blender-4.5.9-linux-x64 archive. Verified runtime
version and bundled license files. Official source/tag:
https://projects.blender.org/blender/blender/src/tag/v4.5.9 ; license:
https://www.blender.org/about/license/ (GPL; external tool, not shipped). Used
bundled Python/numpy and image IO for our original triangle/UV/depth projection
script. No Blender source/add-on/media imported; existing original model provenance
continues through the atlas manifest. No new game dependency.

Existing Bevy0.17.3 DEPENDENCY: SceneSpawner readiness, Assets/Images mip data,
SceneRoot observers and VisibilityRange temporal masks. Installed pinned source
informed API use; no source copied. Scene/material/tangent readiness and actual
native captures verified. Optional city rendering is bounded independently of
physics. No new GitHub project adopted; ADR061 records alternatives/tradeoffs.

### 2026-11-07 installed-source follow-up (existing dependencies)

- **DEPENDENCY**, Bevy / bevyengine, https://github.com/bevyengine/bevy,
  exact installed0.17.3. Inspected PBR mesh visibility/dither shader and visibility
  origin handling for the disappearance investigation. MIT OR Apache-2.0 as declared
  by the installed crate; [license](https://github.com/bevyengine/bevy/blob/v0.17.3/LICENSE-MIT).
  Existing dependency retained, no source copied or new media incorporated. GPU
  cube probe did not reproduce the degenerate-range disappearance hypothesis.
- **DEPENDENCY**, bevy_rapier / Dimforge, https://github.com/dimforge/bevy_rapier,
  exact installed0.32.0. Inspected ContactPairView/ContactManifoldView normal order
  and Collider::project_point for the normal-closing-speed static-wall adapter.
  Installed manifest declares Apache-2.0; [license](https://github.com/dimforge/bevy_rapier/blob/v0.32.0/LICENSE).
  Existing compatible engine integration retained, no copied upstream implementation
  or assets. Contact APIs are used directly; no new dependency/migration. This is
  a pinned-source investigation, not a claim about latest upstream maintenance.

## 2026-11-07 — Street-first offline layout geometry (ADR066)

- **DEPENDENCY (authoring only)** — Shapely / Shapely contributors,
  https://github.com/shapely/shapely, tag2.1.2, commit
  `5fb639d1056888d135fe56bfaf750c9648addeec` (resolved annotated tag).
  Installed2.1.2 actual Python source inspected: constructive.py buffer/node/
  voronoi_polygons and set_operations.py union_all. These wrap native GEOS
  operations: noded crossings, buffered rights of way, union/difference and
  frontage-seeded partitions. Pin in tools/requirements-layout.txt; decision:
  adopt its APIs for offline2D authoring rather than fragile custom polygon math.
  BSD-3-Clause verified at https://github.com/shapely/shapely/blob/2.1.2/LICENSE.txt .
  Reuse permitted subject to notices; no source adapted/copied, assets none.
  Python3.14.8/Linux wheel tested; no Bevy/Rapier integration or runtime shipment.
  Risk: native GEOS and version-sensitive geometric output; pin versions and
  validate topology/coverage, not just record counts. Maintained2.1 release line
  inspected; not a claim that this is the newest release.
- **DEPENDENCY (indirect authoring only)** — GEOS / OSGeo contributors,
  https://github.com/libgeos/geos, installed bundled3.13.1, tag3.13.1 at
  `431568d6e311e0bbfb057b4ec3d44d0d3ba3335f`.
  Geometry engine used through Shapely, no direct C++ code incorporation.
  LGPL2.1 license inspected at https://github.com/libgeos/geos/blob/3.13.1/COPYING .
  Local wheel dependency only; source/build redistribution duties apply if later
  redistributing this toolchain. No GEOS binary/assets committed or game dependency.
  Compatibility verified through actual authoring tests; no Rust API dependency.
  Risk: numeric robustness/reproducibility across native versions; exact wheel
  version recorded in exported JSON. Decision: use through pinned Shapely.
- **DEPENDENCY (indirect authoring only)** — NumPy / NumPy developers,
  https://github.com/numpy/numpy, version2.3.4. Installed to satisfy Shapely's
  array API; no direct algorithm/source/media imported. BSD-3-Clause core license
  verified https://github.com/numpy/numpy/blob/v2.3.4/LICENSE.txt . Wheel carries
  its own bundled-component notices; wheel not redistributed. Python3.14 Linux
  compatibility verified. Decision: pinned indirect authoring dependency; no
  Bevy compatibility concern. Version pin limits numeric/tool drift, not a latest
  maintenance claim.
- **DEPENDENCY (existing image-authoring tool, now pinned for this generator)** —
  Pillow / python-pillow contributors, https://github.com/python-pillow/Pillow,
  version12.0.0, MIT-CMU verified at
  https://github.com/python-pillow/Pillow/blob/12.0.0/LICENSE . Used existing
  Image/ImageDraw APIs to render original map art, no copied source/third-party
  image assets. Local Python3.14 Linux tested; no game dependency. Decision:
  retain authoring renderer. Risk: font/renderer versions affect bitmap hashes;
  exact Python/package environment and font helper documented.

**REFERENCE ONLY — primary NYC design evidence, no repository/assets adopted:**
NYC Landmarks Preservation Commission Flatiron designation LP-0219 (1966),
https://s-media.nyc.gov/agencies/lpc/lp/0219.pdf . Actual document read and rendered
for inspection: triangular site at Broadway/Fifth/23rd determines building form.
Principle studied, architecture/map/media not copied. Separate NYC Plaza Map
https://www.nyc.gov/assets/cecm/downloads/plaza-maps/MN-Flatiron-Plazas.pdf exceeded
web viewer limits and returned403 to direct download; it is not claimed as
visually inspected. The rendered designation report, not that unavailable map,
is the inspected primary evidence. NYC media rights not investigated because
none are redistributed. Full RoadDash geometry and drawings are original work.

### 2026-11-07 — Existing Shapely authoring follow-up, ADR068

Same pinned Shapely2.1.2 / GEOS3.13.1 and previously verified licenses retained.
Inspected installed set_operations.py `intersection` implementation and its
`grid_size`/native intersection_prec path to resolve near-coincident frontage
clipping on a concave regenerated block. Adopted the public API with a1e-9m
snap-rounding overlay; exported footprints retain inner rings and strict overlap/
containment checks. No upstream source copied, no dependency/version change or
new external repository adopted. User-supplied race-first illustration is visual
planning guidance only; no image media was imported into the repo/game.

## 2026-11-07 — Metropolis API/source follow-up (existing dependencies)

- Bevy, https://github.com/bevyengine/bevy/tree/v0.17.3, bevyengine;
  **DEPENDENCY**, exact installed0.17.3. Read local bevy_pbr ExtendedMaterial,
  pbr.wgsl, extended_material example and ImageSampler/VisibilityRange APIs.
  MIT OR Apache-2.0: https://github.com/bevyengine/bevy/blob/v0.17.3/LICENSE-MIT
  and LICENSE-APACHE in that tag. API-based original shader, no example/media
  imported; compatible with locked0.17.3; no upgrade. Study/adopt existing API;
  engine API churn and shader/render-backend testing remain risks.
- Shapely, https://github.com/shapely/shapely/tree/2.1.2, shapely contributors;
  **DEPENDENCY** (offline authoring only), installed2.1.2 with GEOS3.13.1.
  Read installed constrained_delaunay_triangles/make_valid/set_precision APIs.
  BSD-3-Clause: https://github.com/shapely/shapely/blob/2.1.2/LICENSE.txt;
  GEOS LGPL-2.1-or-later remains authoring tool, not redistributed with game.
  No source/assets copied; own compiler calls the public APIs. No Rust/Bevy
  runtime dependency. Existing maintained toolchain retained/pinned in evidence;
  precision/topology changes require rebake/coverage validation before upgrades.

2026-11-07 SIDEWALK-01: reused the existing original RoadDash prop catalog roles
and installed Bevy0.17.3/Shapely2.1.2 APIs already investigated above. No additional
repository investigated, dependency introduced, upstream code copied or media
imported. Street furniture uses the same original project-MIT model files;
physics owns independent conservative colliders. Paving pixels are original
procedural output. Existing asset and model source hashes remain unchanged.

2026-11-07 AUTHORED-01: existing Bevy0.17.3 dependency source follow-up. Inspected
installed bevy_gltf Gltf/GltfMesh/GltfPrimitive APIs, asset dependency readiness,
Mesh indices/tangents and standard PBR VisibilityRange. Same upstream Bevy version
and MIT OR Apache-2.0 license already recorded above; no new dependency or upstream
implementation copied. Actual project-original storey GLBs and shared materials
are reused by handle; no original-game media or new downloaded assets. The local
project's existing fade-range principle is reused with final propagated camera
origins. Palette expansion and final art/performance remain separate.

2026-11-07 STREET-LIFE-01: existing project-original car/furniture/facade catalog
and previously investigated Bevy0.17.3/Shapely2.1.2 APIs reused. No additional
repository investigated, upstream source copied, downloaded media or dependency
adopted. Domain lane/following formulas, authoring seed rules and layout audits are
original RoadDash code. Existing project MIT asset notices/provenance still apply.

2026-11-07 DESTRUCTION-01: inspected the installed existing bevy_rapier3d0.32.0
context shape-cast/contact-pair APIs, backend rigid-body reset and existing Bevy0.17.3
scene hierarchy/LOD interfaces. These are the already adopted dependencies above;
no additional repository/library investigated or imported. Actual backend is
rapier3d0.31.0 per Cargo.lock. Own durability/controller/render integration calls
public APIs; no external fracture implementation copied. Existing project-MIT
prop GLBs/materials are reused; break PCM is original synthesis. Native joint,
velocity-muscle and accepted rider physics remain unchanged. Full fracture would
need dedicated broken art and budget review, so no new engine/dependency is justified.

2026-11-07 AI-10: reused this project's existing normalized roster observations
and installed Bevy0.17.3 ECS APIs. No new repository investigated, dependency
adopted, external implementation copied or media imported. Exact-ID grudge and
racer/killer logic are original RoadDash code for the user's requested behavior;
this supersedes historical personality-hostility decisions without claiming
new original-game behavior research or exact parity. Physics pins are unchanged.

2026-11-07 REPAIR-01/PERF-07: existing installed Bevy0.17.3 SubApp take_extract/
set_extract and RenderSystems schedule source inspected for an additive timing
wrapper; existing bevy_rapier3d0.32.0 contact-pair/manifold/shape projection APIs
inspected for gated persistent contacts. Previously recorded dependency licenses
and versions apply; no upstream implementation copied or new dependency/media.
Shapely2.1.2 offline authoring rebakes the approved original layout/height changes;
source footprints/paths stay unchanged. No new original-game research/parity claim.

## PERF-09 — Parallel physics dependency follow-up (2026-11-07)

- **DEPENDENCY**, existing bevy_rapier3d0.32.0 / rapier3d0.31.0: inspected installed
  Cargo feature forwarding and rapier3d `src/pipeline/physics_pipeline.rs`253–327.
  Native `parallel` activates independent-island Rayon solving; intra-island
  parallelism is explicitly unavailable in this version. Original engine pins and
  previously recorded Apache-2.0/MIT license evidence remain. No copied upstream
  implementation or media; actual use is the existing public Cargo feature.
  Limits: scheduling overhead, connected-island scaling and future determinism.
- **DEPENDENCY**, rayon-rs/rayon, https://github.com/rayon-rs/rayon (Rayon project).
  Locked rayon1.12.0, registry VCS commitc9ced185ae3508246a9eb70c8407a1199bb1b77f;
  rayon-core1.13.0, commit7af20d7692d5decbcb4adcaa079cd607e3e50814.
  Installed manifests, `.cargo_vcs_info.json`, dual license files and rayon-core
  thread-count selection source inspected. Work-stealing parallel iterator/runtime
  supports Rapier's existing island loop. MIT OR Apache-2.0; exact license links:
  https://github.com/rayon-rs/rayon/blob/c9ced185ae3508246a9eb70c8407a1199bb1b77f/LICENSE-MIT
  and https://github.com/rayon-rs/rayon/blob/c9ced185ae3508246a9eb70c8407a1199bb1b77f/LICENSE-APACHE.
  Source reuse permitted with notices; no source copied or assets supplied. Minimum
  Rust1.80 is compatible with project1.94; no direct Bevy API coupling. Adopted
  transitively by pinned Rapier. Installed published snapshots verified; no claim
  of a fresh upstream maintenance-health audit. Risks: extra worker-pool contention
  and small-island overhead; RAYON_NUM_THREADS permits controlled comparisons.
- **DEPENDENCY**, crossbeam-rs/crossbeam, https://github.com/crossbeam-rs/crossbeam
  (Crossbeam project). Locked crossbeam-deque0.8.8 and crossbeam-epoch0.9.21,
  both registry VCS commit2920c984290229ab4e0ca0452ef09e48a82063f3. Installed manifests,
  VCS metadata and MIT/Apache license copies inspected. Rayon-core uses these
  work-stealing deque/epoch synchronization libraries. MIT OR Apache-2.0:
  https://github.com/crossbeam-rs/crossbeam/blob/2920c984290229ab4e0ca0452ef09e48a82063f3/LICENSE-MIT
  and https://github.com/crossbeam-rs/crossbeam/blob/2920c984290229ab4e0ca0452ef09e48a82063f3/LICENSE-APACHE.
  Source reuse permitted with notices; no assets/implementation copied. Rust1.61
  minimum fits1.94; no direct Bevy coupling. Adopt transitive package versions
  through lockfile; existing crossbeam-utils version remains unchanged. Published
  local source snapshot reviewed, not a maintenance SLA. Risks follow the worker
  runtime; audit/test under the actual dependency graph before upgrades.

Actual incorporation is linked library code via Rapier's feature, not adapted
source or downloaded media. ADR076 records rollback and unchanged physical tuning.
