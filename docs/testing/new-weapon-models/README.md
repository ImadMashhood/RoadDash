# Original weapon models — 2026-11-09

The requested chain, whip, Luigi pizza rolling pin and Abeyomi baseball bat are actual original meter-scale 3D meshes. They use faceted turned silhouettes, oxide leather/wrapped grips, warm maple, gunmetal and worn brass details consistent with the existing stylized biker kit. No external repository, stock geometry, proprietary media, raster concept art, fonts or texture images were imported. Project MIT declarations apply; no legal clearance is claimed.

`tools/build_weapon_models.py` builds four editable Blender sources and twelve runtime GLBs. `art/weapons/PROVENANCE.json` records Blender version, source and authoring hashes; `assets/weapons/catalog.json` records each GLB hash, actual indexed triangles and Y-up bounds. Each export contains one mesh with bounded material slots. Grip is exactly origin zero, long axis +Y, matching existing club hand sockets; physical size is baked in meters, with no runtime scale hack. Front/rear PNGs here are Cycles renders of the actual geometry, not generated concept pictures.

| Model | LOD0 / LOD1 / LOD2 triangles | Runtime height including grip |
|---|---|---|
| Chain | 6216 / 3088 / 1556 | approximately 0.93m |
| Whip | 3804 / 1804 / 984 | approximately 1.26m |
| Rolling pin | 1108 / 724 / 364 | approximately 0.72m |
| Baseball bat | 1628 / 736 / 396 | approximately 0.85m |

Run `cargo run --locked --bin weapon_preview -- --id chain` to inspect real GLBs. Arrows change weapon, 1/2/3 change LOD, Q/E orbit and +/- zoom. `--capture PATH` saves a native screenshot and exits after readiness. All three LODs are authored and selectable in the viewer; automatic distance LOD for held assets is not implemented. Existing bounded club lifecycle limits model instances; LOD0 is used for optional equipped review.

Run `ROADDASH_WEAPON_MODEL=chain cargo run --locked --bin weapon_preview -- --equipped --capture PATH` to review the actual existing animated rider/hand sockets. Use `chain`, `whip`, `rolling_pin` or `baseball_bat`. The same opt-in environment variable works in the native game for held and ground inventory visuals. A replacement attaches during Update, retains existing three-part club fallback until the scene is fully instantiated, and inherits the established root visibility/drop/recovery lifecycle. Unknown IDs retain the original club. Without the variable, the working club remains unchanged.

These are presentation skins of the existing club ability: contact remains the existing analytical melee rule, no weapon colliders/rigid bodies are added, no damage/reach/timing/saves/controls/rider physics are changed. The chain and whip have rigid authored rest silhouettes; flexible rope motion, family-specific attacks and contact tuning are deferred and must not be inferred from these models. Character-specific automatic loadouts, final grip/art/user approval and broad performance/hardware measurements also remain open.

Validation: `python3 tools/validate_weapon_models.py` checks actual GLB hashes, indexed geometry triangle totals, POSITION bounds and absence of skin/animation data for all twelve exports. `rr_content::weapon_models` validates exact IDs, safe paths, origin/axis, finite bounded dimensions and decreasing LOD budgets. Native and Cargo receipts are recorded separately when run.

Final evidence: all18 native cases passed (`native-run.txt`, `native/receipts.json`): twelve GLB/LOD captures, four equipped models, original-club baseline and invalid-ID fallback. All four equipped views were visually inspected. Initial equipped review exposed the lab's missing static glTF reflection registration; the fixture now installs the project's existing `BuildingSceneTypesPlugin`, and the complete native run passes. The initial failure receipt is retained for history.

The shared Cargo target initially reused a sibling worktree's rr_content artifact; explicitly refreshing local crate-root mtimes forced a correct rebuild. `interrupted-workspace-test-build.txt` is an intentionally interrupted early full-suite compilation to yield the shared build slot, not a test result. The primary agent owns final integrated full-workspace test/build/audit gates; this worktree records focused catalog/model/weapon regressions and the preview build. No production FPS, Windows hardware, flexible-weapon physics or human art approval is claimed.

Focused regression receipts: two catalog tests, nine `src/models/tests.rs` tests (both 180-tick parity variants report maximum position error0m, full recovery fixture passes), and ten existing weapon tests pass. The fresh rebuild initially exposed the optional skin systems requiring `AssetServer` in renderer-free model tests; making that resource optional preserves the existing primitive/headless path, and all nine rerun tests pass (`initial-headless-assetserver-failure.txt` retains the investigation). No body/constraint/motor/damping/recovery values changed.
