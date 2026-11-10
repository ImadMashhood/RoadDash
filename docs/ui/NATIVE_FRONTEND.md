# Native RoadDash entry flow

Run from the repository: `cargo run --release`. This frontend is integrated into the native Bevy game; the HTML art desk remains a separate authoring tool.

- Splash → main menu.
- Thrash → current course → loading handoff → existing countdown/race.
- Big Game / Street → current rider introduction → street → current course → race.
- Street → bike shop / biker bar → back to street. These are visits; purchases, other playable profiles and conversations are explicitly coming later.
- Options opens existing persistent comfort/control settings; closing returns to the same page. F2 remains available.
- Mano a Mano is disabled: offline only. Quit asks for confirmation.
- Race retains existing HUD/pause/restart. Results shows real finish/arrest/credit values, retry or main menu.

Arrows / D-pad navigate; Enter / bound start / controller South confirms; Escape / controller East goes back; mouse activates buttons. Main-menu Escape opens quit confirmation. Existing start/controller remapping remains active. Menu activation cannot start a race except at the selected course/results, and held confirmation does not repeat.

The active course is selected at process startup as before (default Metropolis; ROADDASH_TRACK=valley or city for legacy maps). This pass does not reload world geometry or silently map unbuilt original courses to Metropolis. Big Game borrows the original navigation, not a completed career campaign. Current player, handling, racer identities and savings remain intact.

[Seven image records](NATIVE_ART_MANIFEST.json): existing approved-style splash/main/street/shop/bar scenes plus newly generated course postcard and rider introduction. New exact prompts/origins: generated/candidates/native-entry-jobs.json and course-native-v1.txt/rider-native-v1.txt. These are illustrative frontend portraits, not new playable character meshes or exact maps. Original media remains private reference only. Original compositions are declared under the project MIT terms; this is not a legal clearance or exclusive-rights assertion.

Actual image dimensions are recorded per file in NATIVE_ART_MANIFEST.json (~1672×941); no fake 4K upscale. ADR103 now fills the window for entry menus with proportional cover crop;the portrait grid/dossiers retain4:3. The earlier ADR102 all-menu4:3 treatment is historical and user-rejected. Seven PNGs are retained while running (~44 MiB decoded base RGBA; excludes GPU/backend overhead). 3D scene rendering is disabled in menus/results and restored for play; no measured FPS gain is claimed.

Use `cargo run --release -- --frontend-smoke` for the scripted native flow and screenshots in screenshots/frontend-*.png; use a test profile/settings location if overriding test defaults. Required automation and human-review limits: ../testing/native-frontend/README.md.

Approved portrait follow-up (ADR094): the Biker Bar now contains a sixteen-card 4×4 portrait picker with individual views and restored selection focus. See [controls, provenance and remaining gates](APPROVED_PORTRAITS.md). Portrait concepts do not replace simulation identities or make new profiles playable. Retained images now include sixteen 768×576 cards in addition to the seven backgrounds.

ADR095: portrait selection/details now use a classic 4:3 canvas, full-page approved paintings, a lower detailed biography strip and compact controls. That historical pass retained16:9/OpenSans elsewhere; ADR102 below supersedes that placement/type choice. See [native captures and limits](../testing/portrait-dossier/README.md).

ADR102 original-format pass: all entry menus use4:3; main has four160×60-equivalent bottom choices and street has three213/213/214×60-equivalent choices at y420/480. Large floating cards are removed; short lower copy and gold text/underline show live status/focus. Main options are Thrash,Big Game,disabled Mano a Mano and Restroom; quit via Esc/controllerEast from Main,options via Restroom or F2. Street is Biker Bar,Bike Shop,Race; back via Esc/East. Other supported pages adapt this rail rather than inventing unimplemented original content. Approved4×4 portraits/dossiers remain. Jost Book/Bold are unmodified OFL1.1 substitutes for geometric retail type, with license shipped; no retail fonts/media bundled. Full HUD/pause/settings visual parity and final art/readability/controller acceptance remain open. Evidence: ../testing/original-ui-format/README.md.

Current ADR103 polish supersedes literal retro formatting: full-window art,clean warm-white titles/copy,right-side main choices,inset secondary actions,soft native gradients and rust-orange focus. Existing art and licensed fonts unchanged; main options still dispatch Thrash/Street/disabled multiplayer/Settings,with Esc/back for quit and F2 options. No gameplay/HUD/physics change. Native source/viewport and acceptance evidence: ../testing/modern-ui-polish/README.md.

ADR104 settings: Main Settings or F2 opens grouped Camera/Controls/Audio/Display. PageUp/Down or RB changes category; arrows select/adjust; click +/- or a binding; Enter starts rebinding; Tab/Y changes binding editor device; Esc/Start cancels a rebind. F2/Esc or Save & close saves and returns,resuming the exact prior race/countdown state. Save failure remains visible; Close without saving keeps session changes without changing the file. Camera comfort,keyboard/controller and pickup/drop remapping,master volume/mute,desktop-resolution borderless fullscreen and VSync are functional. Fullscreen returns to the cached windowed size. Driver behavior/per-bus audio/quality presets and full hardware/accessibility remain open; see ../testing/settings-menu/README.md.
