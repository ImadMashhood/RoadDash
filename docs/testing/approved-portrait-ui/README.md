# Approved portrait UI — 2026-11-08

Branch: codex/approved-portrait-ui. Local uncommitted implementation over inherited UI/weapon/slice changes; no commit, push or merge performed. UI-02/UI-05 remain partial, with no new task IDs and no future milestone closed.

Sixteen exact approved images are preserved in output/portraits/approved-roster; bounded native derivatives live in assets/ui/portraits. The 4×4 picker uses keyboard/D-pad row navigation, mouse activation, individual portraits/bios and restored focus on return. It selects a portrait to inspect; it does not change the race roster, player model, saves, handling or accepted ragdoll. rr_core owns the catalog and valid navigation rules. Bevy owns images, widgets and focus. No dependencies or external repositories were introduced. ADR094 records the choice, memory tradeoff and rollback.

Verification performed:

- cargo fmt --all --check: passed.
- cargo clippy --workspace --locked --all-targets -- -D warnings: passed after final grid correction.
- cargo test --workspace --locked: passed using TMPDIR in the workspace (existing ignored test retained); model differential physics/recovery checks included. The first default /tmp run failed existing save/settings tests with QuotaExceeded; moving temporary files resolved that environment issue without changing those tests.
- Final cargo test --workspace --locked frontend: nine frontend tests passed, including pure selection bounds and four-column navigation, focus restoration and retained profile/phase.
- cargo build --locked: passed. Final cargo build --release --locked also passed; the corrected layout was verified in the debug native binary.
- python3 tools/audit.py: passed, 523 dependency declarations / 129 runtime asset records / 27 observations. Declarations/hash verification is not legal clearance.
- python3 tools/sync_progress.py --check and git diff --check: passed (CSV line-ending normalization warning only).
- All sixteen runtime PNGs match manifest hashes and 768×576 dimensions.
- Native --frontend-smoke: exit 0. Saved report records all sixteen cards opened and returned to the 4×4 grid, 22 entry/race/pause/results/quit stages, 23 images and final garage state. The fixture asserts every grid card has positive bounded dimensions; initial visual inspection exposed intrinsic-image grid expansion, corrected using zero-minimum fractional tracks and absolute child placement. Final grid/detail screenshots inspected.

Evidence: frontend-06-bar.png (4×4 picker), frontend-portrait-01.png, frontend-portrait-16.png and frontend-report.json. Other individual captures remain in screenshots/. The fixture's final grid shot has the last browsed card focused.

Remaining: human menu usability and art/typography acceptance, physical controller and Windows coverage, GPU memory/performance measurements, full UI-02/UI-05 production gates, playable identity selection and game integration. No measured FPS claim. Native warnings include missing optional Vulkan DRM/validation support and an OS controller without a specific Gilrs mapping.
