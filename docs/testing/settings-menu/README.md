# Functional settings menu — 2026-11-09

Branch `codex/settings-menu`, from59fe130. User requests settings setup. New Camera/Controls/Audio/Display categories reuse modern menu typography/palette and real preferences,not unsupported placeholders. Accepted rider/bike/ragdoll constraints/tuning,world,saves and underlying attack/control behavior remain unchanged.

## Features and files

- `src/settings.rs`: optional master_volume/audio_muted/fullscreen/vsync fields with safe legacy defaults,validation and existing backup/write protection. Same version1 reader accepts old files; no automatic rewrite. Old builds preserve extended files as read-only; restore backup when rolling back.
- `src/settings/view.rs`: full-window editor,two-column visible controls,category buttons,+/- changes,rebind prompts,status and explicit save/session-close. Reuses existing licensed Jost assets; no new raster art/dependency.
- Input through existing controls chain: visible-row navigation,categories PageUp/Down/RB,binding editor keyboard/controller Tab/Y,pickup/drop support,reserved/duplicate rejection and rebind cancellation. Gameplay edges remain suppressed while open; exact prior Racing/Countdown restored on close. Camera defaults do not reset bindings/audio/display. Save failure remains open/paused with error; Backspace/Select or Close without saving keeps session values/file unchanged.
- `src/sound.rs`: absolute master gain on existing loops and tagged one-shot base levels,and initial/persisted/session mute. New audio starts at the current gain; already playing effects avoid multiplicative decay. Default1 preserves prior mix. No GlobalVolume-only shortcut (that affects only new playback in installed Bevy).
- `src/frontend.rs`: opaque settings suspend hidden3D camera rendering and restore it after close. No simulation interpolation/physics change.
- Borderless fullscreen/current monitor,windowed-size restoration and native VSync mode update live;explicit --world-survey override preserved. These are driver requests,not a guarantee of hardware timing.

## Verification

413 Rust tests passed; one existing soak test remains ignored. Strict formatting, Clippy and build passed. Final commands/results: checks.json,fmt/clippy/test/build/audit logs. New tests cover legacy optional-field defaults/nondefault save/reload,invalid volume,category/audio/display real actions,windowed-size restore,failed save without overwrite,pause/dismiss and pickup binding/cancel. Absolute master gain/default/mute tests accompany existing PCM and physics/recovery/controls/profile regressions.

Opt-in native: `ROADDASH_VERIFY_SETTINGS=1 target/debug/road-dash --frontend-smoke` in an isolated working folder. It captures all4categories and720p controls,uses actual keyboard messages to change categories/volume/VSync,checks window API mode,saves via F2,reloads `settings-native.json`,and completes the normal22-stage/all16portrait/five-viewport/race/pause/results flow. Script writes only isolated preference evidence,no player profile. Native headless window mode assertions are not physical monitor proof; no human listening/driver latency/controller/Windows signoff.

Initial/local screenshots precede final palette/label alignment; historical receipts and JSON are retained, while redundant intermediate PNGs remain local. Final screenshots/report are under native/. Master shown at95% in settings-audio-adjusted.png. Final settings PNGs are versioned; broader duplicate frontend PNGs remain local. All actual host viewport sizes are retained in frontend-report.json.

## Remaining gates and provenance

SETTINGS-01 is narrow technical integration; UI-02/P15-006/P15-007/AUDIO-013 remain open for complete production art/accessibility,quality presets,independent audio buses/captions,physical controller/monitor/Windows and audible acceptance. Kick/mouse punch alternatives remain fixed and disclosed. Smaller-than720p layout is unverified. Session-close does not revert live changes;its label/status state that distinction. No60FPS claim.

Research: existing installed Bevy0.17.3 window/audio APIs (R08 dependency,MIT OR Apache-2.0);no upstream implementation/media or new repository adopted. Existing unmodified OFL1.1 Jost fonts reused. ASSETS.csv updates source hashes and registers original editor code. ADR104 documents alternatives/tradeoffs/rollback. README/progress/task placement,known issues/history and TEST_MATRIX update together. No files removed,no new Rust dependency/Cargo.lock change,no next major part,no push/merge.

README now records220 verified /245 open /465 stable task IDs. No new repositories, third-party media or dependencies; installed Bevy API review only. Local branch codex/settings-menu.
