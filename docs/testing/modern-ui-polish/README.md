# Modern menu polish — 2026-11-09

Branch `codex/modern-ui-polish`, from d6bbf87. User rejected the previous UI as ugly/dated. ADR103 supersedes that literal-retro layout. Existing approved paintings,portraits and font binaries are byte-identical. No new image generation or gameplay/physics/profile changes.

## What changed

- Entry menus fill the actual window. Loaded image dimensions determine a proportional cover crop; no4:3 side bars or stretched faces. The explicitly requested4×4 portraits/full dossiers retain their4:3 stage/content.
- Main choices occupy the painting's quiet right side. Clean warm-white title/copy,small rust-orange eyebrow/focus and generous spacing replace the hard red/shadowed headline,yellow underline and black rails. Secondary pages use inset actions; existing disabled/navigation/quit/options/retry behavior remains.
- Native Bevy BackgroundGradient/LinearGradient provide continuous upper/lower/right readability shading. No new texture/shader or banded rectangle stack. Jost licenses/provenance retained from prior pass; gallery captions/titles/focus now share the quiet palette and shadow-free text.
- Font scale uses both window dimensions for entry pages; ultrawide does not inflate type. Cover/canvas/font values change only when their computed values differ,avoiding presentation-induced repeated layout writes. No whole-game performance improvement is claimed.

## Verification

Final fmt,strict workspace Clippy,locked workspace tests/build and asset audit are recorded in checks.json/logs. **408 Rust tests passed,1existing soak ignored**. The presentation geometry test verifies nonoverlapping in-bounds action areas; existing navigation/input/retry/profile and physical rig differential tests remain passing.

Native `--frontend-smoke` traverses22 stages,opens all16 portraits/biographies,checks five actual viewport sizes and button bounds,and launches/pauses/finishes/exits the real race. See native/receipt.json and screenshots/frontend-report.json. Actual host resize dimensions are retained rather than claiming requested sizes when the window manager substitutes one. Initial native images precede the final portrait palette alignment; final images are under native/. Main/menu paintings are unchanged.

Preview: [main](native/screenshots/frontend-02-main.png),[street](native/screenshots/frontend-04-street.png),[rider grid](native/screenshots/frontend-06-bar.png),[course](native/screenshots/frontend-09-course-1280x720.png).

## Tracking,rights and remaining work

New narrow UI-POLISH-01 (origin Part15) records technical presentation/verification; UI-FORMAT-01 remains historical and explicitly superseded. UI-02/UI-05 retain human visual/readability/crop acceptance,HUD/pause/settings formatting,full career/shop content,physical controller/Windows/accessibility and performance coverage. A technical pass does not establish that the user likes the design. No all-screen original parity or60FPS claim.

Source files: src/frontend.rs,frontend/layout.rs,frontend/gallery.rs. Documentation/progress/task placement/ADR103/known issues/development history/test matrix/native frontend docs and source intake updated together. No removed files,new assets,Rust dependencies or lock migration. Research: inspected installed Bevy0.17.3 gradient APIs/examples (existing R08 DEPENDENCY,MIT OR Apache-2.0); no upstream implementation copied,new repository adopted or media imported. Previous unmodified OFL1.1 Jost fonts remain licensed/shipped unchanged. Source hashes pin the actual final implementation.

Local branch only; no push/merge. Final commit reference is reported in chat. Next review is the actual rendered menu/readability; no next development part started automatically.
