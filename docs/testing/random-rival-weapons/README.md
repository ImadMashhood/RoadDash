# Random starting rival weapons

2026-11-08, codex/random-rival-weapons. Seven of fourteen rival racers now start with the existing Club. The player and resident officer are excluded. A new grid/retry rerolls the subset; pausing/resuming, dropping and crashing do not replenish equipment. Remaining route pickup, dropped-club cap, actor meshes, physics and combat rules are unchanged. Armed racers only retaliate against their attackers; killers retain their existing policy and all attacks retain the eight-second opening grace.

Selection is based on a process-random session seed and race counter. Set `ROADDASH_RACE_SEED=1234 cargo run --release` for a reproducible sequence. Tune starting_armed_fraction in content/weapons.json (0..1); it is compiled content, so rebuild after edits. The default is exactly half, rounded for smaller fixtures. This is the existing first club, not a new complete arsenal.

Files: rr_core/src/weapons.rs pure seeded selection; rr_content/src/weapons.rs validated fraction; content/weapons.json; src/weapons.rs GridWeapons; src/game.rs reset hook; src/runtime.rs registration. Existing src/frontend/smoke.rs now checks default grid equipment and seven inherited held-root visibilities. New tests cover uniqueness/query order/seed variation, reset/retry and no pause rearm. Existing club contact/range/retaliation/drop/crash tests are retained.

Verification receipts are in this directory and ../part09-start/run-01. The native entry test requires seven equipped non-player/non-police riders and seven active held attachments. Inherited visibility is attachment-state evidence, not proof every distant club occupies pixels onscreen. All viewport/profile/physics guarantees and limits from the preceding native-frontend pass remain. Final test outcomes are appended after commands complete.

No new dependencies, source repositories or media were investigated or adopted for this change. Existing original club provenance rows are updated by hash. ADR092 records choices and rollback. Broad Part07 arsenal, art, combat feel/performance and exit acceptance remains open. Changes are uncommitted; no push/merge.

## Final results

`cargo fmt --all --check`, `cargo clippy --workspace --locked --all-targets -- -D warnings`, `cargo test --workspace --locked` and `cargo build --locked` passed. Workspace total: 375 passed, zero failed, one pre-existing ignored test. Release build passed. Three slice-runner Python tests passed. Audit/roadmap/progress checks are recorded alongside these receipts. No dependency version changes.

Actual isolated native run: ../part09-start/run-01/receipt.json — seed1234, exit0, 22 scripted stages in23.643s (whole process wall time, not FPS); default seven equipped rivals and seven inherited held-root visibilities asserted in the native fixture. Native report records the selected IDs, actual viewport/bounds and final return to garage. This is not a full human race or weapon-balance approval. RIVAL-WEAPONS-01 is narrowly verified; Part07's broader gates remain open.
