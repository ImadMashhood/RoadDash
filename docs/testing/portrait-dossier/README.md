# Full-page character biographies — 2026-11-08

Implemented on codex/approved-portrait-ui over the preceding uncommitted portrait/frontend and unrelated existing work. No commit, push or merge performed.

The 4×4 picker is retained with larger portrait-first tiles, compact name tags, vivid focus, and a 4:3 canvas. Selecting a card opens its painting across the full page, with its complete expanded biography in a lower dialogue strip and a compact Back control. Escape/controller East and Back return to the selected card. Live text remains separate from approved image pixels. All sixteen source paintings and 768×576 derivatives are unchanged. Fonts across the native frontend now use licensed Open Sans Regular/ExtraBold 1.10 with black shadows.

Source investigation: inspected private original RR-0E/RR-1A composition previews, UI-IMG-0032 and UI-IMG-0068 plus docs/ui/catalog.json table geometry. Previews do not capture retail dynamic fonts/labels; exact parity is not claimed. No original game media shipped or source copied. Open Sans came from installed Fedora open-sans-fonts-1.10-25.fc44.noarch, with Apache 2.0 confirmed by packaged LICENSE.txt and font metadata. License and unmodified file hashes are in docs/third_party/licenses and ASSETS.csv. No GitHub repositories or new crate dependencies adopted in this follow-up. ADR095 records alternatives, tradeoffs and rollback.

The bios are adaptations of earlier user character briefs under their later instruction to remove racist material. They restore food/work/personal jokes, appearance and racing personality; they do not implement new character stats, weapons, models or playable profiles. UI-02/UI-05 remain partial in README and the synchronized progress index. No new roadmap task IDs, task moves or completed milestones.

Verification actually run:

- cargo fmt --all --check: passed.
- cargo clippy --workspace --locked --all-targets -- -D warnings: passed.
- TMPDIR set to workspace output/testing/portrait-ui/tmp, cargo test --workspace --locked: passed, existing ignored test retained. Includes navigation/focus, game lifecycle, visual-model differential physics and recovery checks. /tmp still has an environment quota problem from the previous pass; no production code/test was changed to hide it.
- cargo build --locked and cargo build --release --locked: passed.
- python3 tools/audit.py: passed (523 dependency declarations, 131 runtime asset records, 27 observations). Declared licenses/hashes are not legal clearance or security certification.
- python3 tools/sync_progress.py --check and git diff --check: passed; CSV normalization warning only.
- Native debug binary --frontend-smoke: exit 0. All sixteen grid cards opened and returned; every full biography is found as actual live Text with nonzero bounds and is checked clear of bottom controls at 1280×720. Retained 22 entry/race/pause/results/quit stages and five course viewport checks. Screen captures inspected: picker, Esteban detail and Dolph detail. Report is copied here. Optional Vulkan/driver-layer and Gilrs mapping warnings remain environmental.

Remaining: human approval of menu typography/layout and biography editing, physical controller/Windows coverage, larger portrait masters, playable profile integration and broad UI/art/performance gates. No measured performance or exact retail UI parity claim.
