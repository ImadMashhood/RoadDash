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

Actual image dimensions are recorded per file in NATIVE_ART_MANIFEST.json (~1672×941); no fake 4K upscale. The 16:9 native stage letterboxes on other aspect ratios. Seven PNGs are retained while running (~44 MiB decoded base RGBA; excludes GPU/backend overhead). 3D scene rendering is disabled in menus/results and restored for play; no measured FPS gain is claimed.

Use `cargo run --release -- --frontend-smoke` for the scripted native flow and screenshots in screenshots/frontend-*.png; use a test profile/settings location if overriding test defaults. Required automation and human-review limits: ../testing/native-frontend/README.md.

Approved portrait follow-up (ADR094): the Biker Bar now contains a sixteen-card 4×4 portrait picker with individual views and restored selection focus. See [controls, provenance and remaining gates](APPROVED_PORTRAITS.md). Portrait concepts do not replace simulation identities or make new profiles playable. Retained images now include sixteen 768×576 cards in addition to the seven backgrounds.

ADR095: portrait selection/details now use a classic 4:3 canvas, full-page approved paintings, a lower detailed biography strip and compact controls. The rest of the frontend retains 16:9 placement with Open Sans display/body typography. See [native captures and limits](../testing/portrait-dossier/README.md).
