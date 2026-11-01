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
