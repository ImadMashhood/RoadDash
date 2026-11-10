# Original-format entry UI — 2026-11-09

Branch `codex/original-ui-format`, from d12947a. User requests original-game formatting. This is a presentation/layout pass: existing approved PNGs are byte-identical,16-card4×4 selection/dossiers retained, gameplay/save/input/retry/physics authority unchanged. No new image generation.

## Inspected evidence and implementation

Read the existing source-bound UI catalog, RR09/RR0F SVG geometry, SCREEN_MAP, fonts list, prior frontend/dossier records and private MAIN reference. Catalog pins reconstruction7aecf6044f2dd773b925b325823f30d648d61c89; stored geometry is not captured dynamic retail rendering. Main's four cells are x0/160/320/480,y420,w160,h60 on640×480. Street cells are0/213/426,w213/213/214,y420,h60. Native main/street now reproduce those proportions. Other supported pages adapt the same band to actual available actions; no invented playable courses or completed campaign/shop.

All entry pages use a4:3 stage with black letterboxing. Existing wide paintings centre-crop with aspect preserved, rather than stretching characters. Floating heading/body cards and inset red button tiles are replaced by direct title/copy, compact lower scrims and full-width bottom choices. Gold selected text/underline distinguish keyboard/controller focus; subtle white hover remains separate. Main's four options retain Thrash,Big Game,disabled Mano a Mano and Restroom/settings; Esc/controllerEast reaches quit and F2 opens options. Street has bar/shop/race,with back on Esc/East. Native16-card grid/full-page dossiers are intentional user overrides of the old character screen.

Jost3.7 static Book400/Bold700, exact upstream35f141c970538f1ed0f235789c19156b3ce2a762, provide a closer geometric substitute for the catalog's FUTR/FUTB roles. Source inspected README/AUTHORS/OFL; unmodified fonts/license bundled under SIL OFL1.1, with exact asset hashes/intake. See REPOSITORIES R-JOST and assets/ui/fonts/Jost-OFL.txt. No retail font DLLs,original logos/images or upstream code copied. Exact BADLOC/Futura glyph/color/handler parity is not claimed.

## Verification and limits

**408 Rust tests passed,1existing soak ignored**; fmt/strict Clippy/locked debug build and asset audit passed. Final native walkthrough exit0:22 stages,16 portraits,five reported viewport sizes.

Final commands/results in checks.json,audit.txt and native/receipt.json. Full workspace checks include physics/recovery differentials, buffered input, real Results/Busted R retry and source geometry tests. The native frontend smoke traverses22 stages,all16 portraits,course launch/pause/results/quit and five actual viewport sizes,with computed button bounds and portrait biography checks. Initial native run passed but visual review caught street credit/hint overlap; adjust the short copy band and keep initial-native as historical evidence. Final images show the correction. Native render/compiler work overlap is not performance evidence.

Remaining: final user typography/crop/readability approval; original HUD,pause and settings visual formatting; full dynamic menu/career/shop content; physical controller/Windows/hardware accessibility/performance coverage. Existing UI-02/UI-05 stay unchecked. The painting's edge crop can remove peripheral subjects; no fake art approval or all-screen parity. Jost weight/metrics differ from original. Retained OpenSans assets support rollback.

Files: src/frontend.rs and new src/frontend/layout.rs; three Jost files under assets/ui/fonts; README/progress/task placement; ADR102,known issues/history/test matrix; NATIVE_FRONTEND.md; ASSETS.csv and REPOSITORIES.md; this evidence. No files removed. No new Rust dependencies/Cargo.lock migration. New narrow technical task UI-FORMAT-01 belongs to origin Part15. No next development part or push/merge.

Native previews: [main](native/screenshots/frontend-02-main.png),[street](native/screenshots/frontend-04-street.png),[small course](native/screenshots/frontend-09-course-1280x720.png). The host reported one intermediate requested2560×1440 resize as1912×967; exact actual sizes are retained in frontend-report.json, not claimed as requested pixels. All reported layout bounds passed. UI-FORMAT-01 is narrowly checked;218verified/245open across463IDs. No push/merge.
