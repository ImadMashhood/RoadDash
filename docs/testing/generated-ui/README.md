# Generated UI evidence

2026-11-08, codex/generated-ui-screens. Authoring-only changes; no game/runtime code changed.

Initial pass: 28 saved PNG drafts with exact generation prompts, hashes, native dimensions and three real-alpha overlays. Browser evidence in browser-pages.json/browser-workflow.json records 28 decoded page images, gallery, real PNG download, synthetic review-note export and restored prior metadata. Screenshots record that initial pass, not the corrected direction. Mandatory Rust/check logs in this directory are from that pass (367 Rust tests and one pre-existing ignored test); no new native gameplay smoke was run.

User rejected the entire first-pass art direction. Those technical checks do not establish art acceptance. A separate main-gritty-v1.png was generated using privately inspected original splash/shop/street style references, copied unchanged and visually inspected. See docs/ui/UI_ART_DIRECTION.md and candidate manifest/prompt. Original pixels remain outside Git/runtime. Main preview selects the candidate; other first-pass pages default to wireframes and allow explicit rejected-art comparison. Corrected browser appearance has not been visually reverified; no claim of full corrected-screen QA. User approval, remaining screen regeneration, layered/high-resolution masters, native integration and performance gates remain open.

Post-correction authoring check outcomes are stored in revision-checks.txt. No new canonical tasks: this is UI-05 refinement; all art-review checkboxes remain open. No repositories/dependencies adopted; original media used only as local style references. No commits/push/merge performed for this pass.

JavaScript syntax CLI check was attempted but `node` is unavailable in the shell. Corrected preview browser validation remains open. An initial deterministic prompt test failed after manually adding supersession notices; generator was corrected to emit those notices and the focused nine-test suite passed before the full suite was rerun.
