# Rival launch and rear visibility follow-up — 2026-11-11

Base: `eb1c9b705107fcb9e0e75c26561e86c40d5f13f3`, isolated Bevy 0.20.1 worktree. No external repository, code, asset, or license was imported; only the installed lockfile and local gameplay paths were investigated.

## Cause and change

The physical bike already has a bounded first-five-second AI torque bonus, but `src/game.rs` supplied a speed-error-dependent `throttle_amount`; that amount multiplies the bonus in `src/vehicle.rs`. The rival top-speed rule in `rr_core::pursuit` was player-relative and is used by both the AI target and physical bike. The camera overlap rule in `src/game.rs` hid an entire rival hierarchy within a 3 m camera-centered sphere, including followers visible behind the player.

Rivals now request full throttle in the first five racing seconds. Traffic/corner/brake/backing decisions still control whether throttle actually drives the bike. The pure rival ceiling is 155 km/h in both AI and physical paths. Police and player limits stay separate. Direct camera overlap hides a rival only within 1.4 m; recovery and combat/collision continue independently of presentation.

## Verification and limits

- Pure pace/launch and dynamic grid regression cover opening intent, 155 km/h cap, and no early melee knockouts.
- A camera-distance regression covers direct overlap and a visible close follower.
- Full native race balance, moving-follower visual approval, matched GPU/render frame cost and wider hardware remain open. The known Bevy/Rapier high-speed ragdoll and static-collider parity failures are unrelated and remain open.

## Checks completed on this isolated worktree

- `cargo test -p rr_core --locked pursuit::pace_tests`: 3 passed.
- `cargo test --locked --lib following_riders_remain_visible_outside_the_camera_contact_zone`: 1 passed using the shared Bevy 0.20 target.
- `cargo fmt --all --check`, `python3 tools/audit.py`, `python3 tools/sync_progress.py --check`, and `python3 tools/test_roadmap.py`: passed.
- The dynamic grid fixture is added but its focused run, strict Clippy, full workspace tests/build and native smoke/performance measurements remain for the integrated source. A queued grid run was canceled to release the shared Cargo build lock for integration. Existing engine parity failures remain known.

## Integrated queue regression (2026-11-11)

The first integrated workspace run found two additional rider queue failures with a 155 km/h target used for both motor limit and passing horizon. A controlled high-ceiling rerun passed the stationary-human chain. The final candidate keeps the 155 km/h physical motor and kinematic limit while allowing its planner to use the profile's uncapped clear-road target for anticipation; both `role_free_rider_chain_physically_passes_a_stationary_human_player` and `crawling_rider_queue_passes_without_a_car_trigger` then passed individually. The two pre-existing Bevy/Rapier parity tests remain separate failures; full suite rerun is pending.

The combined release binary passed both-map native smoke tests, but these scripted fixtures do not measure a full grid launch or full-race balance. Strict Clippy and locked release build pass; the full workspace run has only the two previously known Bevy/Rapier parity failures (467 road-dash library tests passed). Following-rider visual acceptance remains open.
