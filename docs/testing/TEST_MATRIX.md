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
