# Shared Metropolis/Hills racing AI — 2026-11-12

User approves Hills and requests the same racing AI in Metropolis, followed by
commit/publish/merge. Base local843c80f5; ADR172 records the boundary.

## Implementation

`src/game.rs::ai_uses_local_flow` selects Hills AND Metropolis. Metropolis now
uses the same local same-direction forecast, physical steady-oncoming gaps,
centre/sidewalk choices, deduplication and ordered first-safe lane search. Main
car dimensions/centres come from actual colliders on both maps. Bike-class pace,
personalities, combat, launch lanes and queue backoff were already shared.
Retain route-relative coordinates, split/rejoin corridor checks, solid geometry
sweeps, full oncoming/crossing horizons, contacts, player handling, accepted rigs,
saves and content. Legacy Valley stays the comparison policy.

`src/racer_ai_tests.rs` shares a collision-ribbon/bike/solid-car fixture. Hills
retains its controlled CrossTraffic path; Metropolis uses production main-road
Traffic and marked street lanes at3250m city entry and10400m dense city. Its
final scenario starts the bike in the occupied incoming lane. No artificial
attack/contact/avoidance outcomes. `game::tests` verifies policy selection for
both modern maps and the preserved legacy Valley control.

## Verification

- `cargo fmt --all --check`: pass.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: pass;
  final source recheck also passes.
- `cargo test --workspace --locked`:479 pass, two known failures, one ignored;
  main-lib failures stop workspace traversal. Separate locked domain run:
  rr_core165 + rr_content59 pass, one ignored. Total703 Rust tests pass.
- Known failures remain
  `biomechanics::tests::wall_impact_and_high_speed_ejection_remain_bounded` and
  `world_gen::metropolis::geometry::tests::standalone_static_chunks_keep_contact_ccd_and_support_trajectories`.
  No solver/rig or tolerances changed to hide these.
- Both moving-car physical tests and policy selection pass in the workspace
  suite. [Dedicated final occupied-lane physical output](physical-final.txt): both
  tests pass, including both Metropolis stations (retained final speeds above
  25m/s, health100, Riding and actual steering).
- `cargo build --release --locked` and `cargo build --locked`: pass.
- Nine Python roadmap/native-harness tests and `python3 tools/audit.py`: pass.
- No new dependencies/assets/repositories investigated. No removals of content.

## Rejected scaffold and blocked native run

The first artificial dense-city fixture used CrossTraffic following the curved
race line instead of the real main-road controller/street reservation. It gained
projected lateral velocity and retained the wider crossing margin, then collided.
[Failed trace](physical-trace.txt) retained; it is not a passed main-car test.
[Production-main-car fixture](physical-main.txt) passes at both stations before
the final occupied-lane strengthening covered in the workspace suite.

[Native receipt](native-blocked/receipt.json) is **not passed**: requested7200
Metropolis frames, exit-15 after the own process was deliberately stopped during
countdown. [Native log](native-blocked/native.txt) explicitly reports software
rendering. [GPU diagnosis](gpu-environment.json): loaded NVIDIA kernel615.78.08,
NVML620.70; `nvidia-smi` fails with driver/library version mismatch. The render
schedule reaches seconds per frame; this is not usable hardware/FPS evidence.
No driver, display or system changes made. Both-map new native recovery checks
were not run after this blocker; previous843c80f5 checks remain historical only.

## Tracking and integration

AI-METRO-01 stays **unchecked**: shared policy/physical implementation is in place,
but native rendered traversal/recovery and user Metropolis review remain open.
Counts234 verified/351 open. Preserve AI-11, RFU-RACE-03, AI-ENGAGE-01 and
ADR171 sustained-performance/replay gates. No Metropolis FPS improvement claim.

Restore a matching GPU stack and repeat native Metropolis/Hills recovery and city
split traversal before closing the acceptance item. Do not automatically start
the next development part. Publication uses the connected GitHub app; original
Hills tree matches local843c80f5 exactly. Final publication checks tree identity
against the tested local commit, without force-pushing or rewriting main history.

[Source/binary identity](metro-flow-source.json) and locked-build/test logs
are retained beside this report. Verified release installed atomically for the
existing launcher; [installation receipt](installation.json).
