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

- R03 https://github.com/blyfoten/driftmaster - authors/license/version/maintenance
  not reverified; research-only candidate P03/P06/P08; no source or assets used.
- R05 https://github.com/cBournhonesque/lightyear - no selected revision; P10/P13
  authority/prediction investigation deferred; no networking code used.
- R07 https://github.com/djeedai/bevy_hanabi - no selected revision; optional P15
  effects/performance investigation deferred; no code/assets used.
- R09 https://github.com/NiklasEi/bevy_kira_audio - no selected revision; optional
  P15 mixing investigation deferred because current built-in audio works.

Do not convert these candidates to dependencies or assert file-level permission
without recording an exact inspected revision, license URL and compatibility.
