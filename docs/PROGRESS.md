# Development progress

Issue26 junction-queue follow-up (2026-11-11, ADR151): a bounded native trace
at Metropolis station 3,040 identified a real oncoming car, then rear-occupied
outbound merge space, not a static rejection, at the former queue head. A pure
outbound-refuge candidate moves the denied inbound racing line toward the legal
outbound centre. Matched 7,500-frame native windows reduced sampled Riding
zero-speed/zero-plan points 20→14, with rider 13 clearing the window but rider
1/11 stops remaining. The final wildlife/FX-integrated source passed 670 locked
workspace tests, strict code/build/audit gates and both native recovery rigs.
This is partial evidence; RFU-AI-01/02/06/08, full-race and quiet cost gates
remain open. Receipts:
testing/issue26-racer-ai/native-queue-3040-before and -after.

Issue32 wildlife (2026-11-11, ADR147): six original off-road scenic herds and 48 animals are integrated with main 99b697d3. Five RFU-WILD requirements added (Parts05/06); 228 verified /320 open. Final-source 667 workspace tests (two ignored), strict checks, locked builds, audit and roadmap tests pass. Native close bison/horse survey and four matched enabled/disabled release cost runs pass; frame differences overlap variation and no 60 FPS claim is made. Full-course movement, both-rig native recovery, user art and broad hardware/GPU acceptance remain open. [Evidence](testing/wildlife/REPORT.md).

Issue29 sand follow-up (2026-11-11, ADR144): explicit shared Sand rules and Hills biome agreement are integrated with current racer AI/city/guardrails. Final source passed 656 locked workspace tests, strict format/Clippy, debug/release builds, audit and roadmap checks; four native cases passed on one release binary, including both physical recovery rigs. RFU-SURF-01/02/03 remain open for player feel, steep-uphill reverse and broader acceptance. Current checklist: 228 verified / 304 open. Evidence: testing/sand-speed/REPORT.md.

Issue26 racer/killer follow-up (2026-11-11, ADR141): RFU-AI-01–08 add forward combat and bounded opposite-road passing. Four narrow policy/trace tasks are verified. The branch integrated guardrails and denser city content, passed 647 workspace tests, strict checks, locked debug/release builds and audit. Its unteleported release Metropolis race finished slot zero at 525.05 simulated seconds with nine riders finished; both anatomy and baseline native recovery passed. The real trace still has avoidance-driven sampled stops up to 33 seconds around station 3,040, so queue balance, broad lane behavior and quiet planner cost remain open. Evidence: testing/issue26-racer-ai/REPORT.md. No physical rig/dependency change.

Issue30 atmosphere follow-up (2026-11-10, ADR145): shared Metropolis bird pool, bounded cloud quality switches and native comparison fixture pass on sand/racer-AI integrated source. The 661-test workspace suite, strict checks/builds/audit, final Metropolis visual and eight both-map quality runs pass; local timing differences are mixed and Metropolis remains above 16.7ms. RFU-SKY-01..04 remain open for user art, full-course GPU/hardware and broader performance acceptance. 228 verified / 308 open. No dependency or physics migration.

Issue #40 four-candidate checkpoint (2026-11-11, ADR150): CPU, facade, loading and GPU branches have reviewable draft PRs and frozen baseline receipts. CPU and GPU warm scores differ by less than 0.1 percentage point; final-source/cold and combined-content gates do not establish a repeatable winner. No candidate is merged; RFU-PERF-06/07 and the 16.7 ms target remain open. Evidence: testing/issue40-selection/REPORT.md.

2026-11-10 expanded user scope (ADR150, issues #40–#43): eleven new open IDs track four-way measured performance selection, running hand orientation, bounded Hills/city effects, and denser safer Metropolis sidewalks/crossings/props/trees. The combined issue26/guardrail checklist has 228 verified / 301 open. All new acceptance gates remain open; prior completed CITY-ACTIVITY-02 and RFU-MOVE-03/04 retain their original scope.

Issue28 / ADR143 adds urgent recovery run presentation, terrain-aware swing clearance and signed reverse heading. RFU-MOVE-03/04 passed headless and ten native cases; RFU-MOVE-01/02 retain user terrain/feel acceptance. The civilian-integrated source passed 617 workspace tests and strict checks. Evidence: testing/issue-28-movement/README.md. No movement-speed, rig or dependency migration.

Civilian overtaking (issue27/ADR142): RFU-TRAF-01–04 add slow-leader passing, opposing motor-vehicle horizon/merge checks, deliberate upright-biker non-veto and competing-pass reservations. The integrated traffic branch passed its 615-test and native gates; broad balance, long-race and hardware/performance acceptance remain open. Evidence: testing/civilian-overtaking/REPORT.md.

Issue25 / ADR140 selects exact terrain reuse, single fixed dispatch and eight loading-only material jobs from controlled warm/cold trials. Readiness improved about 37–39% and traversal p95 about 13–23% in those trials; the 16.7 ms, combined-content and hardware gates remain open. Evidence: testing/issue25-performance/REPORT.md.

Guardrail follow-up (2026-11-11, ADR148/issue33): RFU-RAIL-01..05 add turn-side policy, independently grounded posts, bounded runs/re-entry gaps, solid terminals and bounded automatic walking re-entry. Physical gap/midrun recovery, 623 locked workspace tests, strict Clippy, format, debug/release builds, audit, roadmap, 18 paired lab cases and seven native functional checks pass on integrated main. Human full-course and broad hardware/performance acceptance remain open. 224 verified / 297 open; no requirement closure. Evidence/status: testing/guardrails/REPORT.md.
Issue31 / ADR146 (2026-11-11): original batched distant woodland, wider irregular grass, western mesas, four shop-front display families, three guarded street-front groups and outer-lot shrubs/cacti; canyon bed/water raised together. Seven RFU-WORLD IDs remain open; 228 verified / 315 open. Integrated source passes 663 tests, strict checks, audit, six native Hills/Metropolis cases and a bounded noncausal CPU comparison. Valley world-survey still exits1 at 17.94ms p95 against the open 16.7ms gate. See testing/issue31-world/REPORT.md; no full-course art/GPU/hardware acceptance.

Hills rear-grid correction (2026-11-10, ADR139/HILLS-01): reproduced the rear spawn off the imported street. Extend the exact source-city approach through the negative-station grid apron; verify all grid slots and launch corridors against actual road triangles and building footprints. 594 workspace tests and strict lint/build/audit checks pass. Shared Metropolis fixes normalize AI planning coordinates across bends and close the 8cm soil/asphalt seam; Metropolis native seam views and both Hills routes/current-baseline recovery controls pass with zero exceptional rescues. Evidence: testing/hills-grid/REPORT.md. No new IDs or closures.

Current delivery follow-up (2026-11-10, ADR138): native course selection, connected bridge supports/river, gentler farm parcels, separate spatial clouds/distinct sun and extended city shadows. Four new IDs; 222 verified / 270 open. Implementation passes 590 workspace tests, strict checks/builds/audit and seven native scenarios. Four matched shadow runs pass: 150→500 m median window means21.848→21.773ms and P95windowmeans23.544→23.814ms; differences remain within variation, with no60FPS claim. Art/full-course/hardware/performance acceptance stays open. Evidence: testing/hills-delivery/REPORT.md. User explicitly authorizes merge/commit/push to the repository default main branch; publication requires local Git authentication.

Current traffic follow-up (2026-11-10): AI-QUEUE-02/P08-005 addresses the user’s stopped-city-car report. Reproduced a three-car circular wait in core and production fixed-update tests; strict leader ordering implemented. Isolated codex/traffic-stall-fix passes 551 tests, strict lint/format, debug/release builds and audit. Both-direction rendered queue and shared-checkout traffic regressions pass; active and baseline native recovery checks both pass. Evidence: testing/traffic-stall. No population/physics changes, new IDs or checklist closures.

Current revision (2026-11-10): Hills reuses the final Metropolis kilometre, tighter hill bends/curve-only rails, developed Metropolis-style farms, dense western backlots/central ranches, an unrailed canyon suspension bridge with square supports, heavy traffic and bounded volumetric clouds. HILLS-07/08/09/10 and ADR137 track the work; PERF-08/P16-003 retain measured performance gates. Latest combined workspace: 575 tests and strict checks/builds pass; native/performance receipts are recorded at testing/hills-revision. Paired release controls show 4.8% lower frame-window medians with collider-only static scenery; cloud differences remain within run variation, and stable 60 FPS is not established. 222 verified / 266 open; human art/full-course/hardware gates remain open.

Current polish (2026-11-10): HILLS-04/05/06 and ADR136 add user-authorized atmosphere, mounted cowboys and richer original scenery. Three agents completed distinct scopes; 548 workspace tests, strict checks/build/audit and six native cases pass. West/east guardrail traps use the explicitly verified 120-second/+5-second rescue; baseline and Metropolis remain unassisted. Evidence: testing/hills-polish. Human art/full-course/performance gates remain open; 222 verified / 262 open.

Current scope (2026-11-10): independent Hills course under HILLS-01/02/03 and ADR135. City → winding elevated hills → western town with an in-town branch rejoin → dunes. Existing maps/default and physical rigs retained. 536workspace tests and strict checks/build/audit pass; native Hills survey passes1506road probes. Final recovery receipts: testing/hills. Human art/feel/full-course and hardware gates remain open; no new task closures.222verified/259open.

Current scope: user-authorized performance work under PERF-08/P16-003, after fast-forwarding local main to the verified multiplayer/city pass. Remote push is waiting for Git authentication. Release comparisons show lower gameplay CPU work from skipping ineligible crossing forecasts; total-frame gains overlap run variation and the 16.7ms target stays open. Rejected worker/prop-body/damage-prefilter experiments are not shipped. Final529tests/strict checks/builds/audit and native destruction/both-rig recovery/city survey pass; no new IDs or checklist closures.

Current follow-up: AI-QUEUE-02 and CITY-ACTIVITY-02 track multi-rider queue escapes,50%more city main traffic, doubled city pedestrians and paced crossings. No rural increase. Implemented with529passing workspace tests; native city/reload checks pass. CITY-ACTIVITY-02 verified/moved to Part00; broader networking/AI review remains open;222verified/256open.

Completed implementation/integration of all ten fresh contextless GPT-6 Astra/medium tasks: Hall detail/PBR, original outfit, researched lighting, shorter race/no gates, lane alignment, queued-rider passing, persistent crops, moderate relief and police art.481tests, strict checks/builds/audit,28Python tests,12release native cases and18paired lab cases pass. RACE-TRIM-01 verified/moved toPart00; CROP-TRAIL-01/ELEVATION-01/POLICE-ART-01 and broad art/feel/performance/hardware acceptance remain open.221verified/253open. Route wall p9522.38ms misses16.7ms. Evidence: testing/next-pass. No city-width removal, remote push, new main merge or next major part.

User-authorized local integration: merged living-city-ai into main, with a fresh codex/city-next-pass branch. Runtime source/assets match the verified 2bbf731c implementation; earlier main integration evidence/art retained. No requirement closures or remote push. Verification and biker images: testing/main-merge-2026-11-09.

Current follow-up: denser trees and batched civic grass under CITY-CIVIC-01; CITY-WIDTH-01 previews removal of 136/273 non-frontage blocks while preserving both race branches and all six parks. No city removal applied. Character gallery distinguishes approved portrait art from exported meshes. 458 workspace tests, 20 authoring tests and five native checks pass; wall p95 22.99ms still exceeds 16.7ms. Evidence: testing/civic-greenery.

Current follow-up: ADR121 connects park 5 paths to six sidewalks, brings City Hall 24m toward its driving street with 31 surrounding buildings and greenery, and adds live obstacle-aware mounted and walking AI escapes. Uses existing CAST-AI-01/CITY-CIVIC-01; no new IDs or closures. All 457 workspace tests and 20 authoring tests pass, including physical concave-trap fixtures without rescue/teleport. Final route survey wall p95 is 22.26ms, above the 16.7ms target. Evidence and remaining art/balance/full-race gates: testing/civic-access-ai.

Current scope: user-requested cast AI, bounded reverse recovery, urban-only population, traffic recycle fixes, City Hall/fountain park/cafes, gradual rural infill and wider sidewalks (ADR120). New IDs CAST-AI-01/CITY-CIVIC-01/CITY-WALK-02/CITY-LIFE-02 have implemented, integrated scope with450passing workspace tests,20authoring tests,5native runs and18paired lab cases; remain open for art/balance/performance acceptance. User-selected park3 City Hall and park5 detailed park replace the rejected small-lot proposal. Wall p95 22.65ms exceeds16.7ms. No next major part. Evidence: testing/living-city-ai.

Current scope: helmeted stylized clothing, matte human skins and curved wind grass (ADR119). Existing ART-04/V2-06-007/WORLDART-01 remain partial; no new IDs or closures. Evidence: testing/stylized-humans-grass.

Current scope: user-requested gravel verges, fence-clear cows and existing farm props (ADR118). Validation/evidence: testing/farm-verges. No new IDs or closures.

Current scope: user-authorized landscape follow-up after the twelve polish passes: original bounded grass, field material variation and actual-footprint-clipped soil shoulders (ADR117). Existing V2-06-005/006/007 and WORLDART-01 stay partial; no new IDs, closures or count changes. Evidence: testing/landscape-graphics.

Current scope: All twelve sequential gameplay/graphics polish passes are integrated and parent-reviewed; final439-test regression, ten native cases and18-case paired lab passed. The16.7ms frame target, first-survey capture readiness, art/feel and hardware gates remain open. No new IDs or checklist closures (220 verified/245 open). Evidence: testing/polish-12/final/README.md.

Current scope: sequential polish12 adds confirmed-contact directional punch/kick recoil, bounded replacement/recovery and held-contact/attack priority (ADR116). Existing kick push, three-hit knockoff, physics, sound and camera policy remain intact. COMBAT-FEEL-01/P05-006 remain partial; human art/feel/balance/hardware/performance acceptance stays open. No IDs, moves or count changes; stop after12. Evidence: testing/polish-12/pass12.

Current scope: sequential polish11 corrects render-only ankle pivots to match the original authored chain (ADR115). Sole/grip landmarks, neutral surface and both physical rigs are regression checked; full gait/civilian seams/art/performance/hardware gates remain open. No task IDs, moves or count changes. Evidence: testing/polish-12/pass11.

Current scope: sequential polish10 uses a separate display cube with matching exposure/analytic tonemap, layered cloud banks and a fixed correctly oriented sun (ADR114). Lighting radiance maps remain unchanged. Clouds are stationary; independent cloud motion and full landscape/hardware/performance/art acceptance remain open. No task moves, IDs or count changes. Evidence: testing/polish-12/pass10.

Current scope: sequential polish09 replaces opaque linear distance fog with bounded exponential-squared haze and a shared exposed horizon colour (ADR113). The non-HDR sky/fog tonemap mismatch remains V2-06-007 after a rejected black HDR trial. Geometry/LOD and warm lighting stay unchanged; SKY-01 evidence extended, broader V2-06 art/hardware/performance gates remain open. Evidence: testing/polish-12/pass09.

Current scope: sequential polish08 blends low-rise approach facade use across existing business/workshop/residential families (ADR112). Approved heights, footprints, setbacks, vegetation and collision remain unchanged. PALETTE-01/V2-04-003 remain open; evidence: testing/polish-12/pass08.

Current scope: sequential polish06 connects original asphalt PBR to Metropolis and refines road/paint/paving scale (ADR110). Existing road geometry/contact and facades unchanged; P11-004 remains partial. Evidence: testing/polish-12/pass06.

Current scope: sequential polish05 tunes shadow contact bias and cascade overlap without adding maps/casters (ADR109). Evidence: testing/polish-12/pass05. P15-004 remains open for scalable presets, temporal/hardware and broad performance acceptance.

Current scope: sequential polish04 unifies late-afternoon lighting settings (ADR108), with native before/after evidence in testing/polish-12/pass04. Existing SKY-01 baseline evidence extended; broad lighting/shadow/art/hardware gates remain open.

Current scope: sequential gameplay/graphics polish pass02: downed-contact pre-solver eligibility correction and low-speed contact/reverse/remount audit under V2-01-003. Existing human/confined-recovery/hardware gates remain open. Evidence: testing/polish-12/pass02/README.md.

Current scope: sequential gameplay/graphics polish pass01 (PERF-08): facade selection stability and paired native profiling. Full route/GPU/frame-budget and human/hardware acceptance remain open. Evidence: testing/polish-12/pass01/README.md.

Current scope: SETTINGS-01 grouped native editor (ADR104), persisted camera/controls/audio/display with safe save failure/session close. Broader quality, audio buses and hardware/art gates remain open. Evidence: testing/settings-menu/README.md.

Current scope: UI-POLISH-01 modern full-window entry menus (ADR103) supersedes the user-rejected literal-retro ADR102. Native22-stage/all16portrait/five-viewport and408-test verification complete; broader UI art/readability/HUD/settings/hardware/performance gates remain open. Branch codex/modern-ui-polish; testing/modern-ui-polish/README.md.

Current scope: UI-FORMAT-01 source-table4:3 entry layout, compact controls and licensed Jost font substitute (ADR102), verified native22-stage/five-viewport/16portrait pass; UI-02/UI-05 keep full HUD/pause/settings, font/art/crop/human/hardware acceptance open. Branch codex/original-ui-format; testing/original-ui-format/README.md.

Current scope: fresh-context Part07 arsenal and Part08 police agents plus downtown street population on codex/combat-police-downtown. Five-family gameplay and police last-seen/search/feedback pass are integrated; broad Part07/08 art/fairness/exit gates remain open. DOWNTOWN-01 and UI-RETRY-01 are technically verified; evidence is recorded in testing/combat-police-downtown/README.md. No Part09 advance.

Current UI follow-up (2026-11-08): approved sixteen-portrait 4×4 picker and full-page 4:3 biographies (ADR094/095), preserving current race identities and physics; UI-02/UI-05 remain partial. Evidence: testing/approved-portrait-ui/README.md.

Current scope: random starting rival clubs (ADR092), then Part09 isolated scripted slice receipt infrastructure (ADR093). Earlier native frontend/art work remains on this branch. Broad Part07/09 and external/hardware/performance gates remain open. See testing/random-rival-weapons/README.md and testing/part09-start/README.md.

Current scope: UI-02 first native frontend pass — approved gritty images, original-inspired Thrash/course and Big Game/rider/street navigation, native launch/settings/results and two new art compositions. Full art/career/controller/hardware acceptance remains open. See testing/native-frontend/README.md and ADR091.

Current scope: UI-04 — 28 ChatGPT built-in imagegen drafts saved and applied to the artist preview; 25 backgrounds / 3 real-alpha overlays. Live labels/buttons, gallery/downloads and review metadata verified; native runtime unchanged. All generated art review rows remain open, with UI-05 for approved layered/high-resolution masters and UI-02 for Bevy integration. Evidence: testing/generated-ui/README.md (ADR090).

Current scope: PERF-08 focused optimization and UI-03 artist prompts (ADR089). Preserve physical settings and exact city collision surfaces; measure backend phases/catch-up, avoid redundant render writes and batch immutable solids independently of materials. UI-03 covers 28 draft screens / 142 UI image IDs / 66 families; art/Bevy integration and broad city 60 FPS remain open. Evidence: testing/performance-ui-prompts/README.md.

Current scope: UI-01 — source-bound UI/media catalog and independent artist skeleton; original pictures remain external. 142 UI resources/29 populated screen entries/13 popups/7 unique Windows dialogs,63 geometry SVGs and28 draft pages are verified. All 142 image/15 component art boxes remain open; UI-02/P15-006/007 retain production art, native hooks and approval. No game UI/runtime changes. See ui/README.md and testing/ui-kit/README.md (ADR088).

Current scope: RACE-SETUP-01 — reference-informed default rider schemes/readable leather, disabled player nitrous in both adapters,20m staggered grid and four/three/four-lane Metropolis. Geometry, paths, rigs and recovery are regression verified; human art/race balance and performance remain open. See testing/race-setup/README.md and ADR087. No Part09 work.

Current scope: Part08 resident police motorcycle — one non-racing shared Rider, authored validated warning/LOS pursuit/search/return, live foot targets, deliberate dismount, safe arrest/escape and no-payout busted retry. P08-006 is verified in Part00; the other9 Part08 requirements remain open pending full art, discretionary targeting, human fairness, performance/hardware and exit acceptance. See testing/part08-police/README.md and ADR086. No Part09 work.

Current scope: Part07 first held club — original procedural training baton, append-only authored side attacks, validated ability/snapshot, buffered pickup/drop, shared rival and prop/NPC contact integration. P05-003 is verified and moved to Part00; the other ten Part07 requirements remain open. Evidence: testing/part07-weapons/README.md (ADR085). Broad arsenal/art/balance/human/performance gates remain open; no Part08 work.

Current scope: Part06 NPC-SEAMS-01 — shared original civilian source-space weights, continuous pelvic ownership and one-level conforming blend refinement across55 families/twoLODs. Original surfaces/materials and physical drivers are retained; cows byte-identical. Synthetic/native per-attachment evidence and increased geometry/residency costs are explicit; some torso/shoulder samples still worsen. All11 Part06 tasks remain open; no landscape work. See testing/part06-npcs/README.md and ADR084.

Current scope: Part05 DESTRUCTION-02 — twelve offline original-source buckled variants for six crush furniture roles, prepared hidden scenes and immediate intact reset. Prop punches/kicks now consume the shared fixed contact event/frozen kick side after Part03 timing integration. No new bodies or racer tuning; all ten Part05 boxes remain open. See testing/part05-props/README.md and ADR083.

Current scope: Part04 PALETTE-01 — expand Metropolis from12 to24 original facade families using deterministic district/height roles. Detailed and coarse source identity now share bay/storey fitting and include model-derived crown tiles. Approved world geometry and independent collision remain unchanged; all29 Part04 requirements remain open. See testing/part04-world/README.md and ADR082.

Current scope: Part03 runtime contact timing and terrain foot-lock lifecycle (ADR081). Buffered melee intent now resolves at explicit extended-contact clip keys; fixed clocks, frozen kick side and interrupt/pause/reset rules are shared with rendering. Physics-local support locks finitely release and reacquire each stance. All27 remaining Part03 tasks stay open; final motion/terrain/art, active recovery work limits, hardware/performance and user approval remain open. See testing/part03-runtime/README.md.

Previous scope: Part02 ANIM-16 — shared runtime-profile Blender grip/fist authoring, typed socket props and complete assembly hand states; actual4.5.9 GLB reload and original multi-angle review. All15 remaining Part02 requirements stay open, including source/runtime silhouette/normal parity and user approval. See testing/part02-authoring/README.md (ADR080). No Part03 advance or physical/gameplay/timing changes.

Current scope: ANIM-15 — Animation A visual glove shapes, typed sockets and native close-up review. ANIM-02 keeps final hand art/clearance approval open; ANIM-16 tracks static authoring round-trip. No physical ragdoll or gameplay animation timing changes. See testing/hand-contacts/README.md.

Current scope: RECOVERY-01 / REVERSE-01 — live pickup physics handoff and low-speed brake-to-reverse. ANIM-14 resumes Part02 with a hash-bound source-to-runtime coverage audit; no pose approval or new interpolation is implied. See testing/remount-reverse/README.md and animation/NEXT_REVIEW.md.

Current scope: PERF-09 — default Rapier parallel-island feature, audited runtime dependencies, serial fallback and controlled paired crash/native measurements. 307 tests,18 matched lab cases, both native routes and repeat comparisons verified; broad low-FPS optimization remains PERF-08.

Current scope: REPAIR-01 and PERF-07 — seated stance, fence/start/height/contact fixes and local performance evidence. Technical tests/native integration verified; full art/balance and60FPS remain open; see testing/startup-world-diagnostics/README.md. PERF-08 retains measured optimization.

Current scope: AI-10 — seven racers and seven killers, exact landed-hit retaliation, passing and more aggressive killer pursuit. See testing/racer-killer-ai/README.md; AI-11 keeps human race balance open.

Current scope: DESTRUCTION-01 — persistent Metropolis props only; no car damage, building collapse or road damage. Evidence: testing/prop-destruction/README.md. DESTRUCTION-02 retains dedicated broken-art refinement; legacy-map retrofit and broad art/performance gates are not closed.

## Current live roadmap — completed first

README is the single live task source. Part 00 contains all 220 verified tasks;
Parts 01–16 contain all 245 unfinished/partial/unverified requirements. The reorganization preserved status; four later collision corrections were
verified and moved to Part 00 in the subsequent implementation session. Historical numbering in task IDs is stable;
current README section determines execution order, not the ID prefix.

The complete eleven-family animation coverage and two-stage deliverables are now
explicit in README's dedicated Animation production section. Existing V2-02/V2-03
IDs own the expanded coverage. Animation A source investigation and the first draft authoring deliverable are verified; full art approval remains open.

Current scope: STREET-LIFE-01, denser lane-aware main-road traffic,2/3/4 marked lanes, checked street activity and per-building height contrast (ADR072). See testing/metropolis-street-life/README.md. Previous scope: AUTHORED-01, reuse earlier3D GLB facades on Metropolis with resident bounded detail; PALETTE-01 tracks wider catalog/district curation. See testing/metropolis-authored/README.md (ADR071). Previous scope: SIDEWALK-01, wider Metropolis walks and checked curb furniture, following runtime construction (ADR070). See testing/metropolis-sidewalks/README.md. Previous layout-only scope: LAYOUT-05 race-centered city envelope and track-first primary junctions; preserve race/farm geometry, street-first blocks and height/detail rules, no 3D. See planning/farms-long-city/RACE_FIRST.md. Previous scope: LAYOUT-04 offline height zoning, seeded frontage/interior accents, six-metre service access and race-distance authored detail; no 3D rendering. See planning/farms-long-city/MASSING.md. Previous scope: LAYOUT-03 street-first NYC/Flatiron correction: curved/diagonal roads define polygonal blocks and fitted straight-edged parcels, 180 × 360 m regular grid, farms → extended city → urban finish, one fork and elevation; layout-only. All-rectangle/forced right-angle rules are superseded. See planning/farms-long-city/README.md. Previous scope: LAYOUT-02, city center with two confirmed split-and-rejoin sections; layout-only, no 3D changes. See planning/city-center/README.md. Previous scope: LAYOUT-01, dense road-hugging city planning only; user explicitly excludes 3D changes. See planning/dense-city-layout/README.md. Previous scope: user-requested world repair and 3D Come Bye art direction; see planning/world-redesign/README.md and testing/world-repair/README.md. Eight focused technical requirements verified; complete art/NPC seams/contact-timed feel remain open. Previous scope: finish facade LOD before Part05 textures on codex/facade-lod-identity. LOD-04 technical integration is verified; LOD-03 user visual/long-streaming acceptance remains open. Previous scope: select velocity muscle controller and correct severe facade pop-in on codex/velocity-popin. Retain passive anatomical joints; native-motor candidate is excluded. RAG-12/LOD-02 own verification. Previous scope: user-authorized Method B velocity/impulse muscle comparison on codex/crash-velocity-muscles. Shared lifecycle from Method A retained; no automatic merge. RAG-B-01..03 track implementation/QA/user choice. Earlier scope: research-only crash/ragdoll audit before implementation, per latest user steering (RAG-11, research/crash-behavior/README.md). PR17 is pushed/merged at6aa3f17383c7773ebb81e09a8706128f873bfc08; new codex/crash-behavior-research branch changes documentation only. V2-01-006 and V2-03-005 implementation remains open. Earlier scope: user-requested roadside variety,35 roles/20 authored groups and independently seeded sides; DRESS-02 verified by native224-owner/680-mesh/34-role/20-group census on the existing local Part05 branch (ADR056, testing/roadside-variety/README.md). Earlier scope: Part05 semantic dressing/materials plus explicit soft-contact, facade-transition and cloud/daylight follow-ups (ADR054/055, testing/part05-world/README.md). PR16 is merged; follow-up branch remains local. COLLISION-07, LOD-01, DRESS-01, MAT-01 and SKY-01 verified; MAT-02 and broad art/performance gates remain open. Earlier scope: user-requested high-rise/infill/road detail, stable facade loading and five-second unseen rear NPC crash recovery, followed by commit/push/merge and explicitly authorized Part05 (ADR053, testing/world-polish/README.md). KIT-04/05, PERF-06 and ANIM-12 verified. Earlier scope: Part04 authored Valley storey kit and deterministic bounded geometry integration (ADR052, testing/architecture-valley/README.md). KIT-01/02 and V2-04-005 verified; Part04 art/role/infill and performance gates remain open. Earlier scope: camera/contact/gait/wrist/crash-style fixes and default Valley restoration (ADR051, testing/recovery-polish/README.md). Earlier scope: Part 03 animation/recovery and corrected tuck/attack sides; ADR050 and testing/animation-part03/README.md record implementation and remaining gates. Earlier user-authorized scope: integrate and merge the approved biomechanical ragdoll with the current game (2026-11-06, ADR049). Automatic startup, explicit baseline, original crash-direction policy and current prop NPC skins are retained. Earlier scope: apply selected 3D prop drafts in gameplay (PROP-03, ADR048). Traffic/scenery and human/cow visual bindings now use the existing physical owners; full art, category placement, bespoke audio and production acceptance remain open. Previous scope: author actual 3D prop drafts. PROP-02 covers 369 mesh IDs/1,107 GLBs and an isolated native viewer; faithful concept likeness, actor rigs and live replacement remain open. Previous scope: extend the building workflow to all world props and actors. PROP-01 verifies scoped extraction/review infrastructure, with a full rebuild queue and 369 source groups covered by 66 selected draft concept sheets. V2-05/V2-06 retain identity/art/mesh/placement/behavior acceptance. Prior scope: correct collisions with downed riders/bikes using original-style contact routing (COLLISION-04; ADR045). This preserves accepted ragdoll settings and existing car entry behavior. Prior scope: assemble complete buildings using original-game facade/roof/end/side-street principles and make the playable race a 5.3 km generated city. CITYGEN-01 verifies the new default City Run: 977 whole lots, 53 blocks, eight district zones, 85 measured City-eligible facades, actual 3D front/return modules, shared roofs and detailed rear walls. Physical collision, supported grid/roads/sidewalks, recovery paths, native retries and finish remain integrated. Original Valley content/quotas and the accepted physical rider/bike/ragdoll remain preserved. MESH/V2-04 retain final art, optimization and broader production gates. Earlier open work remains tracked:
interpolation/terrain gait/active recovery, buildings, props, NPCs followed
by landscape/sky, weapons, police, original Part 09 playtests and Part 10 network
spike. Later production/economy/online/QA requirements remain tracked.

All 465 task IDs are in README exactly once. This includes four new CRASHREF IDs;
the inherited 357-ID header was stale after the preceding animation additions. roadmap/BLUEPRINT_PARTS_01_08.md is frozen historical evidence;
sync_progress.py now reads README only. roadmap/TASK_PLACEMENT.json records the
current section for every ID; roadmap/README_REORGANIZATION_STATUS.json protects
all checkbox states at the time of this move. Acceptance details remain in
roadmap/EXECUTION_V2.md and roadmap/SUPPORTING_CONTRACTS.md.

The implementation notes below are historical receipts. Their Part numbers refer
to the original blueprint unless explicitly marked V2; consult README for current
placement. No new gameplay work, tests or milestone completion is implied by
moving an item. Existing partial work and unverified exit gates remain open.

## Approved metropolis construction — 2026-11-07

User approved building the race-first farm-to-city source. Metropolis is now a separate default runtime course; legacy Valley/City remain selectable with their original quotas/IDs. See [native evidence and remaining gates](testing/metropolis/README.md). METRO-01 records the scoped integration; broader layout/art/performance/traffic exit gates remain open.

## Current collision correction scope

COLLISION-01/02/03 originated in Part 01 and are now checked in Part 00 after
verification. They cover input-owned mounted heading, removal
of scripted collision launch/spin, and explicit arcade car-speed loss plus a
mass/inertia/gravity energy envelope. The new request supersedes P06-022's earlier
clearance jump. Full over-car choreography and combat swerve remain open V2 tasks.
The dedicated eleven-family animation plan is documentation, not completed clips.

## Animation A — current authoring session

247 numbered draft candidates, 27 mapped use cases, 247 primary renders and 57
supplemental views are available in art/animation. The editable Blender package
has 247 single-frame constant pose Actions. All 247 JSON poses round-trip with
maximum matrix difference 4.77e-7; no physics bodies are present in the library.
140 Rust and 14 Python tests pass; format, Clippy, build and authoring CLI pass.
See testing/ANIMATION_A.md and animation/README.md for receipts and open work.
ANIM-01 is verified infrastructure; ANIM-02/03/04 retain hand/sequence/support and
specialist-pose gaps. V2-02-004 source research is complete. No pose is approved,
no runtime playback is connected and the accepted passive ragdoll is unchanged.
The user handles the previous PR merge; no CI wait is a gate for this session.

## Animation context correction — 2026-11-01 (supersedes V1 semantics)

V1 guessed meanings from inspection-based helpers; its labels are rejected as
native reference authority. ANIM-05 records the corrected trace:104 bank/code
slots,194 ordered descriptors and371 sprite usages, with independent CANS/EXE
reader and source-selector evidence.194 descriptors/rear selections agree with
the separately compiled pinned reconstruction; native original execution remains
unverified. See animation/context/README.md and testing/ANIMATION_CONTEXT.md.

The new isolated art set has138 human /16 bike-only layouts, labeled by action
and phase. Police35 phases are metadata-only; code43's five empty source images
have no invented recreation. ANIM-06 remains open for silhouette/support/fist/
weapon/cloth review, unresolved callers, secondary views and approval. Passive
ragdoll and runtime art are unchanged. Old ANIM-01 counts document infrastructure,
not verified action meanings. Source timings are logical counters, not Bevy clips.

## Authored-model baseline (ART-03, preserved history)

User scope: replace character/bike on codex/seated-models-preserve-ragdoll while
preserving the accepted physical ragdoll exactly. The new meshes bind to existing
visual joints; no simulated skeleton migration is authorized. Part 11 prototype
integration only, with animation polish/LOD deferred under ART-04/ART-05. See
ADR-027, art/README.md and testing/MODELS.md for actual verification receipts.
ART-02 and all unrelated milestone exit gates remain open.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| ART-03 / 11 | Replace approved rider/bike visuals without losing accepted crashes | authored GLBs/pose; current body/visual separation | Four human states, riding/wreck geometry, unchanged body/joint descriptors and 180-tick trajectories, full recovery; automated and native evidence |
| ART-04 / 11 | Polish retarget, fingers and interpolated authored transitions | ART-03; static pose library | Contact-correct gestures/deformation and readable transitions; pose review and native clips with physics parity |
| ART-05 / 11 | Add art LOD and broader hardware evidence | ART-03; profiler | Collision-independent distance LOD and measured budget on supported hardware; native surveys and Windows review |

## Evidence and current boundaries

Part 00 artifacts: GDD.md, VERTICAL_SLICE.md, ORIGINAL_CONTENT_POLICY.md,
content/SHEETS.md, ../content/catalog.json, research/OBSERVATIONS.csv (23
immutable documentary observations), third_party/REPOSITORIES.md, ASSETS.csv and
license notices. Source/owner/confidence/use/follow-up are explicit; no historical
play session or rights grant is invented.

Part 01 implementation: ../crates/rr_core, ../crates/rr_content and the real
client subsystem plugins in ../src/runtime.rs, controls.rs, content.rs and
diagnostics.rs. ADR-001 through ADR-008 document scope/compatibility and rollback.
Test requirements and actual receipts: testing/TEST_MATRIX.md and
 testing/PARTS_00_01.md. Known limitations: KNOWN_ISSUES.md.

Full simulation extraction (P01-003/P01-004), complete authority-event contracts
(P01-006), a functional netplay mode (P01-007), and real Windows hardware
verification (P01-009/P01-010/P01-011) remain open. Subsystem plugins and pure
core/content crates are useful progress, not all eleven future crates or a server.

## Part 02 implementation and remaining gates

See tracks/AUTHORING.md and testing/PART_02.md. Authored 6.85 km Valley Run,
2 km handling fixture, 3D arc length/bank/width/surface/sector metadata, physical
ordered gates, separate collision ribbons, bank-safe terrain, F4/CLI inspection,
seeded farm/city/hill gameplay actors and actual elevated recovery are implemented.
Source files: ../src/track.rs, environment.rs, recovery.rs, game.rs and
../crates/rr_content/src/track.rs. Tests cover a complete no-teleport fixed-controller
race and one payout, endpoint/retry/shortcut rejection, 240 road collision rays,
boost/grass/cow/slope behavior and 100 high-speed ragdoll drops.

P02-004 remains partial only for optional branch/split/merge support. P02-010 has
both requested road lengths but representative busy-race pacing/fun measurement
is not established. P02-011 remains open until those requirements are resolved.
The user changed route order and closure; original task text remains in the index
as history, with the explicit current contract P02-012. P06-012 includes a broader
trap/traffic/time-loss audit than the new drop stress test, so it stays unchecked.
P07-003/P07-005 are partial (bike ordered gates now work; out-of-bounds/foot and
same-tick rider/bike outcomes remain). P07-007/P12-005 retain class/track save work.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P01-019 / 01 | Prior Windows CI audit failed decoding UTF-8 author metadata as cp1252 | metadata/text readers | Explicit UTF-8 and a simulated Windows code-page regression; Python tests; fresh remote matrix still required |
| P02-012 / 02 | User's point-to-point map correction | authored spline, gates | Separate endpoints, no return geometry/wrap; full fixed-controller race; missed gate stops/retry, ordered-gate tests |
| P02-013 / 02 | Environment affects gameplay, per user sketch | terrain, solid props, fixed patrols | Seeded farm gaps/fences/cows lift riders; city streets/solid crossing cars/pedestrians; elevated hills/guardrails; actor counts, cow test and native captures |
| P02-014 / 02 | Visual review found grass intruding on banked asphalt | terrain and separate road meshes | Terrain follows bank with clearance; first collision surface is road at 240 sampled positions; winding and resolution tests |
| P02-015 / 02 | Sea-level righting would break elevated recovery | terrain, runner/wreck recovery | Actual hill crash/get-up/run/remount completes at height; native fixture and screenshots |
| P03-012 / 03 | Boost release/exhaustion snapped from 270 to 198 km/h | speed controller | Excess speed decays by drag, not a target clamp; both input scenarios tested |
| P03-013 / 03 | Grass drag exceeded acceleration and drained health | surface metadata, AI | Four player/AI grass cases advance without health loss/knockoff; AI returns to road |
| P03-014 / 03 | User requests slightly tighter handling | validated bike tuning | About 20% greater lateral rates, old schema-1 catalogs load defaults, steering/frame-rate regressions pass |
| P06-017 / 06 | Uniform joint limits fold limbs implausibly | existing articulated bodies, Rapier motor API | Directional hinges, bounded twist and passive tissue resistance; 100 dynamic drops, anchor/axis/finite/ground checks, native crash captures |
| P06-018 / 06 | Wreck sticks to slopes; always-low friction drifts too far on flats | terrain ray normals, fixed physics | Low friction only on steep support; ramp regression and native hill/flat recovery |

## Part 03 implementation and remaining gates

The native default now uses the dynamic chassis adapter; the assisted controller
remains an explicit comparison option. See [contract](VEHICLE_AND_COMBAT.md),
[tests/measurements](testing/PART_03.md) and ADR-012–015. Pure direct-hit/contact
rules live in rr_core; versioned handling validation lives in rr_content. Runtime
vehicle, combat and NPC adapters are integrated into the fixed schedule.

P03-003–009 are verified by automated numeric/curb/lean/frame-rate tests and native
render/audio checks. P03-010 remains partial: a 600-second simulated aggressive
turning soak passes, but ten human rides/controller feedback are missing.
P03-011 stays open until distinct feel, beginner completion and fun are accepted.
The force model is road-assisted arcade physics, not a free-heading multibody
motorcycle. Existing Part 01/02 and foot-finish gates remain unchanged.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P02-016 / 02 | User requests fewer rails and richer scenery | seeded zones/solid terrain | Outer curve rails only; varied trees, detailed buildings/cross cars/pedestrians; native captures and source predicates |
| P03-015 / 03 | Strong road-centering undermined lane choice | dynamic forces | Reduced adjustable assist, no lane-zero magnet; offset-at-rest regression and F6 native overlay |
| P03-016 / 03 | Preserve AI avoidance when adding a new adapter | controller intent/fixed forces | Same AI/player intention path, launch grace; dynamic eight-rider grid test |
| P03-017 / 03 | Adapter exposed stale remount velocity and class mass loss | crash/remount, presets | Stationary protected remount and correct dry wreck mass for all three presets; regression tests |
| P04-010 / 04 | User clarifies colors mean racer shirts only | shared shirt palette/HUD | Eight racer dots match model colors, no actor-category markers; UI palette test and native image |
| P05-009 / 05 | Broad melee/start fights caused immediate knockouts | pure hit chain/LOS | Three direct hits, each gap <=3 s; reach/side/height/heading/occlusion and duplicate protection; core/game tests |
| P06-019 / 06 | Equal-speed contacts should transfer momentum | pre-solver velocities/contacts | Rear closing >=50 km/h or strong side impact wipes out; light taps nudge; threshold and game tests |
| P06-020 / 06 | Cows/pedestrians must react physically | articulated bodies/fixed messages | Distinct 10/11-body rigs, eight-ragdoll cap, reset and occupied-spawn-safe cleanup; headless/native actor checks |
| P06-021 / 06 | Sidewalk contact incorrectly caused crashes | support rays/Ground tagging | Low curb supports bike at elevation without knockoff; stationary and moving curb tests |
| P08-010 / 08 | User requests US traffic and safe grid start | AI intent/fixed patrols | Positive-right forward traffic, separated one-way cross lanes, no early AI fights/weave; source/native/grid tests |

## Part 04 — user-authorized follow-up to merged PR #3

Software implementation: native Gilrs gamepad input, analog force strengths,
rebindable keys/buttons, saved comfort panel, interpolated heading and camera
volume clearance, traction/speed cues and independent on-foot input context.
See [contract](CAMERA_AND_INPUT.md) and [receipt](testing/PART_04.md).
P04-005/006/007 are checked only for those implemented/automatically verified
capabilities. P04-004 is partial pending human high-refresh jitter/readability
review; P04-008/009 remain open pending real-controller finish and external tester
acceptance. The Part 03 human feedback gate is still open; no Part 05 start.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P04-011 / 04 | Numeric high-refresh camera and clipping/warp coverage was missing | interpolated pose, Rapier sphere sweep | Finite/continuous 60/120/144 Hz views; wall clearance and large-warp snapping; camera_tests.rs |
| P04-012 / 04 | Persisted/rebound input needs corruption and modal isolation protection | settings schema, live bindings | Roundtrip, future/corrupt file unchanged, conflict rejection, no hidden driving/countdown leakage; settings/controls tests |
| P04-014 / 04 | Native backend discovered an unmapped controller; do not infer real-device correctness | Gilrs/OS device access | Exercise real axes/buttons/default mapping and add an appropriately licensed mapping if needed; physical controller test (open) |
| P04-013 / 04 | Digital-only intentions would discard trigger pressure or latch disconnected input | native Gamepad, fixed intentions | Partial trigger yields lower real chassis acceleration; controller removal clears input and keyboard works; device/vehicle tests |

## Latest user corrections included with Part 04

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P04-015 / 04 | User finds character turn lean odd | rider/bike presentation | No cosmetic steering roll, wheels/suspension retained; neutral-root regression and native captures |
| P06-022 / 06 | Existing pedestrian path had no lift and fixed cow impulse varied by mass | collision event, pre-solver velocity, selected mass | Both civilian kinds ragdoll; each of three classes clears standing height and passes obstacle with finite physics; six numeric cases |
| P08-011 / 08 | User reports crossing cars not causing wipeouts | crossing actor contacts/classification | Explicit hazard tag, all-type/continuous contacts; moving/stationary car contacts wipe out in assisted/dynamic adapters; four physical cases |
| P11-011 / 11 | Human appearance changed on entering ragdoll | shared procedural segment definitions | Identical eleven mesh/material/size slots for seated/pedestrian/runner/ragdoll; four-state test, native actor/race captures |

Original Part 03 lean-clamp presentation target is superseded by the user's
explicit no-lean correction. Diagnostic lean demand/traction calculations remain.
The shared model is procedural graybox, not the future production animation rig.

## Handling / lean / camera correction after Part 04

Latest request: reference handling is the target; turn force strengthens while
held; bike and rider should bank together from input only, not road curvature;
chase views were too distant. See [source investigation](research/ROAD_RASH_HANDLING.md)
and [receipt](testing/HANDLING_CORRECTION.md). P04-015 is historical/superseded.
User confirmed they have no original files. P03-020 remains open/blocked on
verified authorized calibration and numeric parity; existing engine/brake/gears/
traction/nitro behavior is not misrepresented as a complete original clone.

| New ID / part | Description / reason | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| P03-018 / 03 | User explicitly requests C++ handling investigation | exact source snapshot, rights check | Trace control/physics/motion and identify missing data; source-linked research and R01 update |
| P03-019 / 03 | Constant steering missed the reference's stored-force buildup | pure state, validated tuning, fixed input | Held force grows, release/reversal unwinds, speed dependence/no parked strafe; core and game tests; timings explicitly provisional |
| P03-021 / 03 | User asks for twice-fast full lean on both sides | authored steering response, visual bank | Halve rise/release timing in all classes; 16 Hz visual response; same force/angle caps; both signs attain full bank quickly and neutral stays upright on curves; core/vehicle tests |
| P03-022 / 03 | User asks for stronger turning at maximum lean | P03-019/021, handling profiles | Raise profile turn response by 25%; both adapters consume gain, full-left/right travel increases without larger visual angle; integration comparison and native smoke |
| P03-020 / 03 | Entire handling equivalence needs original SPEC/table inputs and replay comparison | authorized verified values, independent engine compatibility | Match throttle/brake/gear/nitro/steer/grip/motion across bikes/surfaces with recorded numeric tolerances; open, user has no files |
| P04-016 / 04 | User reverses no-lean feedback and forbids curve-driven lean | shared visuals, interpolated body pose | Bike/human same input bank, no roll from a banked road at zero input; actual presentation hierarchy test and native captures |
| P04-017 / 04 | Camera lag and overhead far view zoomed out too far | interpolated pose/velocity, collision sweep | Near/default/far distances <=3.93/5.20/6.68 m from focus in 27 scenarios, including slow smoothing and 80 m/s; camera tests/native review |

## Added requirements and rationale

| ID / part | Description and reason | Dependencies | Acceptance / verification |
| --- | --- | --- | --- |
| P01-012 / 01 | Frame-rate independence and reset soak; variable scheduling previously untested | fixed physics/input | Equal 120 ticks/poses over two seconds at 30/60/120 FPS; 100 crash/pause/resume/reset cycles, no actor leaks; automated game tests |
| P01-013 / 01 | Preserve button edges through zero/multiple fixed ticks | core intent buffer | A short press during a zero-tick render frame executes once at the next tick; core and game regression tests |
| P01-014 / 01 | Version legacy profiles without dropping credits or overwriting unknown data | core schema contract | Legacy roundtrip and malformed/future-version rejection/preservation; profile tests |
| P01-015 / 01 | Reproducible local panic reports and a headless content validator | logging/content validation | Validator success/exit-2 failure; intentional panic produces local backtrace and nonzero status; process probes |
| P01-016 / 01 | Prevent README/tracker checkbox drift | stable task IDs | Generated full index matches README; sync_progress.py --check in CI |
| P01-017 / 01 | Audit embedded font/render media beyond package SPDX declarations | source/license investigation | Runtime media hashes/notices checked; unverified LUT feature excluded; audit.py |
| P06-014 / 06 | Rider remains seated / artificial launch reported by user; separate collision-clear ragdoll and preserve measured momentum | existing crash bodies, physics queries | Initial rider/wreck velocities equal measured velocity, no injected Y; seated visuals absent from wreck; headless snapshot test + native crash capture |
| P06-015 / 06 | Root-pivot reach and runner impulse can prevent remount after a bike tumbles | bike OBB, runner controller | Reach all six representative poses from both sides within 8 seconds; dynamic native crash recovers, no leaked actors |
| P06-016 / 06 | Shoulder damage immediately after remount undermines recovery protection | existing immunity timer | End-to-end crash/recovery restores full health and immunity at handoff; gameplay test |
| P01-018 / 01 | Platform build is not a GPU/hardware acceptance receipt | CI and native installation | Record Linux adapter and real Windows/NVIDIA launch/input/audio receipts; hardware gates remain unchecked until run |

Future consequences already retained in the original backlog: P03 dynamic chassis
adapter, P04 gamepads/comfort and full rebinding UI, P05/P07 authoritative events,
P06 manual foot override, P07 foot finish/checkpoints, P10/P13 headless server and
transport, P12 career save migration/backups, P15 rendering/audio budgets, P16
hardware/clean-install/performance/release audit. No requirement was removed to
close this session.

## First-map procedural base-world foundation — 2026-10-30

Completion follow-up: BW-B00–B10 and BW-X01–X03 verified. Ubuntu/Windows CI
run 41166322394 passed implementation f245f2b; native 1080p survey p95 10.28 ms.
See [complete receipt](world/COMPLETION_TESTS.md). No future major part started.

User supplied a structural world PDF, JSON and section CSV and requested starting
first-map procedural generation. [Review, contracts and BW-B00–B10 dependency/
acceptance register](world/BASE_WORLD.md); [verification](world/BASE_WORLD_TESTS.md).
B00–B02 established the foundation; the user then explicitly requested completion.
B03–B10 structural construction is documented in [the implementation](world/BASE_WORLD_COMPLETE.md)
and [completion receipt](world/COMPLETION_TESTS.md). Current checkboxes below
reflect verified gates. The route/seed/gameplay are retained while canonical old
buildings are replaced by exact 247-root geometry. Source PDF handoff
language is design guidance; active user scope controls this session.

## City streetscape follow-up — 2026-10-31

CITY-03 adds flat concrete sidewalks and paving joints, original collidable curbside
trees, and an authored 1.6 urban floor multiplier. Canonical quotas/IDs and lot
footprints remain intact. See [verification](testing/STREETSCAPE.md) and ADR-024.
Original source height tables remain historical inputs, superseded at runtime by
the user's taller-city request.

## Rider, steering and pursuit follow-up — 2026-10-31

COMBAT-02, CITY-04, RIDE-02 and AI-01 address the user's combined corrections.
118 automated tests and native crash/remount/finish smoke pass. Fifteen-racer
city survey: 11.29 ms p95 at 1080p on this host. Reference parity, production
art and human difficulty acceptance remain open. [Evidence](testing/RIDERS.md).

## Individual rival personalities — 2026-10-31

AI-02 completes validated level-zero profiles, selective relationship-driven
race/follow/fight decisions, individual hit responses/stats and a supported rear
grid with 12 m gaps. 120 tests and final native inspection/survey/smoke pass.
Evidence: [PERSONALITIES.md](testing/PERSONALITIES.md). AI-03 retains full-career,
relationship-axis/event and random-stream parity gates.

## Part01 runtime development pass — 2026-11-08

Measured vehicle-contact observations and bounded combat obstacle interruption are integrated under ADR079. Default velocity-muscle combat swerve already existed; old proposed-only wording is superseded. All29 Part01 task boxes remain open because broad acceptance is incomplete; the task-by-task audit and exact verification are in testing/part01-runtime/README.md. Verified local-disc calibration tables exist, but original numerical handling/crash parity, source reuse permission, user playtests, Windows hardware and physical-controller gates remain open. No new task IDs or task moves.

## 2026-11-09 — Player-hosted multiplayer, UI and walking follow-up (in progress)

The user authorized ONLINE-PLAY-01 and UI-RETRO-BG-01; walking animation continues under V2-03-003. Quick Play must join an available host with measured RTT <=250ms or become host, fill with bots and count down 15seconds. Players use different internet connections and the user does not want to operate a directory. Discovery and NAT reachability remain distinct unresolved integration gates; no live internet matchmaking claim. Native host authority, UI and visual gait work proceed in isolated worktrees. No task is closed by this entry.

<!-- GENERATED CHECKLIST INDEX -->

### Part 00 — Completed and verified foundations

- [x] **RFU-MOVE-03** Reverse steering turns heading left for D and right for A. Reason: issue #28 reports wrong reverse heading; part: 01; dependencies: REVERSE-01/P03-019; acceptance: both Dynamic and Assisted paths, symmetric inputs, continuous zero crossing and unchanged forward handling; verification: integrated vehicle tests and native reverse/forward review. [Evidence](testing/issue-28-movement/README.md).

- [x] **RFU-MOVE-04** Keep run cadence matched to measured movement. Reason: issue #28 forbids silent recovery-speed tuning; part: 03; dependencies: RFU-MOVE-01; acceptance: measured-distance phase, continuous speed changes, unchanged 6m/s controller and physical recovery; verification: gait trajectories, models differential checks and paired lab/native recovery. [Evidence](testing/issue-28-movement/README.md).

- [x] **CITY-ACTIVITY-02** Increase urban main traffic by50%, double urban pedestrians and add occasional sidewalk-to-sidewalk crossings. Reason: explicit user city-activity request; part: 06; dependencies: approved Metropolis baker, NPC patrols and host authority; acceptance:144 main traffic vehicles and384 city pedestrians, rural counts unchanged, paced grounded crossings and preserved collision; verification: authoring/runtime tests, generated census and native Metropolis survey. Verified:144main cars/384urban pedestrians,10clear crossing routes, two completed native crossings, rural/structural preservation and full integrated checks. [Evidence](testing/city-activity-02/README.md). Performance remains a separate open gate.

- [x] **RACE-TRIM-01** Move Metropolis start forward and finish backward by approximately 500m each, preserving the full map and removing visible race gates. Reason: user wants a tighter race without rebuilding the world; originating part: 01; dependencies: METRO-01, RACE-SETUP-01; acceptance: shifted starting grid and finish, correct race-relative HUD/results/retry/AI, no gate structures or gate HUD, both branches and physical world preserved; verification: start/finish/progress/retry/branch tests and native race/recovery captures. Verified:481-test workspace suite, interval/branch/false-win/retry tests and both-route release recovery/results captures; [integrated evidence](testing/next-pass/README.md). Full-race user balance/performance remain separate gates.

- [x] **SETTINGS-01** Integrate grouped Camera, Controls, Audio and Display settings. Reason: replace the flat editor with usable persisted settings; originating part: 15; dependencies: UI-POLISH-01 and existing input/preferences; acceptance: four categories, live master volume/mute/window controls, safe legacy loading, rebinding, save failure handling and session-only close, pause/input preservation; verification: [automated checks and native keyboard/save/reload walkthrough](testing/settings-menu/README.md). Independent audio buses, quality presets and human/controller/display acceptance remain open under UI-02/P15-006/P15-007/AUDIO-013.

- [x] **UI-POLISH-01** Replace the rejected literal-retro entry UI with modern art-led presentation. Reason: user rejected hard red/shadowed headings,black strips,letterboxing and yellow rails as dated; originating part: 15; dependencies: UI-RETRY-01 and existing approved art/font intake; acceptance: full-window proportional image cover/height-aware type,right-side main choices/inset secondary actions,native soft gradients/quiet focus,preserved4×4/dossiers and navigation/retry/physics; verification: [408tests,strict checks and native22-stage/all16portrait/five-viewport review](testing/modern-ui-polish/README.md). UI-FORMAT-01 is retained as superseded history; human styling/crop approval and broad UI-02/UI-05/HUD/settings/hardware/performance gates stay open.

- [x] **UI-FORMAT-01** Apply original-table proportions to the native entry frontend. Historical verified pass; the user rejected its dated styling and ADR103/UI-POLISH-01 supersedes its presentation. Reason: wide cards and an inset action strip did not match the original UI formatting; originating part: 15; dependencies: UI-01/UI-RETRY-01 and existing approved entry/portrait art; acceptance: all-entry4:3 stage, source RR09/RR0F bottom-cell proportions, compact live copy/focus and licensed geometric typography, preserved16-card picker/dossiers/actions/retry and no retail media/font import; verification: [408 tests, native22-stage/all16portrait/five-viewport review and font intake](testing/original-ui-format/README.md). User art/font/crop approval and full HUD/pause/settings/dynamic-screen parity remain UI-02/UI-05 gates.

- [x] **UI-RETRY-01** Preserve captured R retry from Finished/Busted through the real frontend. Reason: native police integration exposed the results menu clearing restart before game authority could consume it; originating part: 15 (police integration08); dependencies: UI-02/P08-006 and buffered input; acceptance: one edge dispatches normal Ride/Starting with readiness guards, no repeated held-key/garage/settings launch, cleared arrest with retained roster/profile/no payout; verification: two actual-frontend input/context regressions, [407-test suite and final native police arrest/R retry](testing/combat-police-downtown/README.md). Broader UI art/controller/viewport and police fairness gates remain open.

- [x] **DOWNTOWN-01** Populate the complete urban race corridor with trees, parked side-street cars and physically supported walking pedestrians. Reason: no avenue trees, sparse parking coverage and unchecked walker patrols left downtown empty; originating part: 05 (NPC support06); dependencies: STREET-LIFE-01/SIDEWALK-01/DESTRUCTION-01; acceptance: deterministic bounded district allocation, retained main4/3/4 race lanes/all building footprints, original-road curb bays with1.5m clear outer walks, whole capsule/furniture-clear authored patrols and native movement/residency evidence; verification: [coverage, authoring tests and native survey](testing/downtown-street-life/README.md), [combined407-test/native pass](testing/combat-police-downtown/README.md). Final density/art, crowd animation and user-hardware performance remain open.

- [x] **AI-12** Predict car approaches, swept crossing conflicts and occupied passes before braking or steering. Reason: rivals repeatedly drive into vehicles; part: 01; dependencies: AI-09/AI-10 and shared traffic/vehicle adapters; acceptance: actual car motion and map-correct outbound bounds, speed-aware yielding/following/passing and hazard-only rider occupancy, unchanged player/crash/contact/role rules; verification: deterministic pure and real dynamic stop/follow/crossing/field scenarios plus integrated native/recovery regressions in [AI repair evidence](testing/ai-traffic-repair/README.md). Broad race balance/human review remains AI-11.

- [x] **ROAD-WALK-01** Make the approved four/three/four roads legible and restore physical road/sidewalk access on foot. Reason: city entry appears two-lane and grounded walkers stick at support edges; part: 04 (recovery support 01); dependencies: RACE-SETUP-01/METRO-01 and shared foot recovery; acceptance: continuous divider/edge paint, regular lane dashes with real pavement support, historical four-lane entry proof (superseded by ADR123’s verified4→2entry alignment), grounded adhesion fix with original capsule/step/rig/collisions retained, actual two-way terrain/sidewalk crossings and solid-wall rejection; verification: authoring audit, real-source Rapier crossing fixtures and both-route native walk/remount evidence in [road/walkability evidence](testing/roads-walkability/README.md). Full map art/hardware gates remain open.

- [x] **WEAPON-MODELS-01** Author original chain, whip, pizza rolling pin and baseball bat models for the existing hand sockets. Reason: user requests actual 3D assets for the new weapon concepts; part: 07; dependencies: P05-003/ANIM-15; acceptance: four editable sources, twelve meter-scale GLBs across three bounded LODs, validated source/binary geometry and grip frames, native viewer/equipped proof and safe optional held/pickup skin lifecycle with working club/physics retained; verification: [model catalog, native renders, hashes and regression evidence](testing/new-weapon-models/README.md). Flexible chain/whip mechanics, distinct family attacks, character loadouts, automatic runtime LOD and final art/performance approval remain V2-07 gates.

- [x] **ANIM-15** Integrate visual glove grip/fist profiles and typed hand sockets. Reason: rigid open fingers ignored punch/grip contexts; originating part: 02; dependencies: ANIM-14, existing model/contact rig; acceptance: independent side-specific shapes, interpolated weights, relaxed fallback, shared bounded glove-only GPU targets, validated grip/weapon frames and unchanged physical rig; verification: socket/mesh/shape/blend/content tests, physics differential checks and [native review](testing/hand-contacts/README.md). This completes technical integration only; final art and authoring round-trip stay open under ANIM-02/16.

- [x] **RECOVERY-01** Remount at the live wreck pickup position and support height. Reason: the riding chassis snapped away after pickup; originating part: 01; dependencies: existing recovery/physics adapters; acceptance: live wreck coordinates, backend current/next pose, velocity/state/history synchronized without stale crash restoration; verification: displaced/elevated-wreck tests in both adapters, repeated dynamic ticks and two native zero-error remounts in [receipt](testing/remount-reverse/README.md). Full obstruction/art acceptance remains P06-009.

- [x] **REVERSE-01** Back out with the existing brake input after stopping. Reason: a solid obstacle could trap a mounted player; originating part: 01; dependencies: existing drive intent/tire force; acceptance: forward braking before bounded parking reverse, no throttle/reverse conflict, throttle returns forward, solid-wall backout and both adapters retain recovery/checkpoints; verification: 30/60/120 Hz domain tests, dynamic wall and assisted integration, native reverse in [receipt](testing/remount-reverse/README.md).

- [x] **ANIM-14** Reconcile context-first draft sequences with shipped runtime clips. Reason: next animation work needs an explicit source-to-runtime gap inventory; originating part: 02; dependencies: ANIM-05/08 and existing catalogs; acceptance: hash-bound sequence/pose/contact/entry/exit/interrupt mapping, unmatched keys and absent clips explicit, no invented native timing or pose approval; verification: reproducible [coverage report](animation/RUNTIME_COVERAGE.md), --check and six context tests; [next review](animation/NEXT_REVIEW.md). V2-02 and ANIM-02/03/04/06 approval gates remain open.

- [x] **PERF-09** Enable supported multicore physics with a serial fallback and measured comparisons. Reason: user requests parallel physics after logged simulation slowdown; part: 11; dependencies: PERF-07 and pinned Rapier feature/license review; acceptance: default parallel_physics forwards native island parallelism, logs identify compiled backend, unchanged physical tuning passes regressions, exact serial/parallel lab/native comparisons are recorded and new transitives audited; verification: [feature graphs,307 physical tests, paired crash labs and native logs](testing/parallel-physics/README.md). This does not close PERF-08's full-race frame budget.

- [x] **RACE-SETUP-01** Align default rider color schemes, disable player nitrous and widen the starting route/grid. Reason: user requests original-style colors, no player boost, more starting clearance and four/three/four lanes; originating part: 01 (world support04/05); dependencies: REPAIR-01, STREET-LIFE-01 and shared authored rider materials; acceptance: reference-informed fifteen-slot swatches and readable neutral leather, boost disabled in both adapters,20m station gaps/player last, supported extended grid, four3m lanes before/after the fork and three on each branch with matched pavement/traffic/paint, preserved paths/buildings/rigs; verification: [364tests, clearance audit, native road views and recovery](testing/race-setup/README.md). Technical integration verified; full race balance and color/art/performance approval remain open.

- [x] **REPAIR-01** Correct seated knee stance, fence ends, opening road/traffic, skyline transition and low-speed car wipeouts. Reason: user reports inward knees, missing posts, start pileups, abrupt city and touch-triggered ejection; part: 01 (world support04/05); dependencies: mounted IK, approved Metropolis compiler and contact policy; acceptance: outward seated bend with held pegs and unchanged unheld anatomy, both fence ends, matching10m farm pavement/handling with shoulders,64 delayed-start cars, gradual city-edge heights preserving paths/footprints, gentle solid contacts without ejection and preserved high-speed crash/recovery; verification: anatomy/contact regressions, authoring/clearance audits and native both-route/survey views in [evidence](testing/startup-world-diagnostics/README.md). Full art/balance remains open.

- [x] **PERF-07** Capture local bounded frame and scene diagnostics. Reason: low FPS is unexplained by aggregate CPU/GPU utilization; part: 01; dependencies: existing fixed diagnostics and Bevy render app; acceptance: nonblocking1Hz rotated JSONL with wall/main/gameplay/physics/render extraction/render CPU-wait, visibility/assets/bodies and Linux memory/paging, printed path, disable/directory controls and a summary tool, without false GPU attribution; verification: bounded producer/source tests, native real-file parse/timing samples and build/regression receipts in [evidence](testing/startup-world-diagnostics/README.md).

- [x] **AI-10** Split rivals evenly into racers and killers with exact-attacker retaliation. Reason: user requests passive overtaking racers and aggressive hunters instead of shared personality hostility; part: 01; dependencies: roster pursuit, direct combat and existing steering; acceptance: seven/seven roles, racers pass without initiating fights and only retaliate against actual attackers, killers actively pursue and fight, auto-kick respects permissions, restart clears memory, existing handling/crash/recovery retained; verification: [domain identity/role/passing tests, integrated hit/innocent-bystander regression and both native routes](testing/racer-killer-ai/README.md). Human balance remains AI-11.

- [x] **DESTRUCTION-01** Make every placed Metropolis sidewalk prop persistently destructible. Reason: impacts should leave the race visibly damaged; part: 05; dependencies: STREET-LIFE-01 and original prop/physics/navigation systems; acceptance: all995 props/13 types accumulate collision or direct melee damage, original models topple/crumple with solid matching geometry, bounded active debris persists until restart, poses/colliders/health/navigation reset, cars remain intact and rider/ragdoll gameplay passes regressions; verification: [real-model collision/reset views, headless contact/melee/budget/settling/navigation tests and native recovery](testing/prop-destruction/README.md). Scope excludes cars, buildings, roads and legacy-map retrofit; coarse damage is not finished fracture art.

- [x] **STREET-LIFE-01** Populate main streets with lane-aware traffic, parking/furniture and distinct roof heights. Reason: sparse main-road activity, uniform lane count and block-level heights miss the requested living city; part: 05 (road/height integration04); dependencies: METRO-01, SIDEWALK-01, AUTHORED-01 and existing traffic/prop assets; acceptance: actual96-car American main-street flow, data-driven2/3/4-lane sections with100m tapers and following/yielding, racer-centred170/220m collision interest, full vehicle-pavement clearance, more checked furniture/parking, pronounced per-building height variety and intact footprints/race/sidewalks/accepted physics; verification: [domain/schema/authoring tests,3,480-point pavement audit, native flow/height census, views and recovery](testing/metropolis-street-life/README.md). Verified:288 Rust tests, five authoring fixtures,3,480 chassis-clearance samples, native lane/fleet/height census and both release recovery branches; full-course art/platform/performance acceptance remains open.

- [x] **AUTHORED-01** Use the earlier detailed3D facade kit on source-mapped Metropolis polygon faces. Reason: flat source-derived atlas walls omitted the authored storefront/window/crown meshes; part: 04; dependencies: METRO-01, KIT-01 and SIDEWALK-01; acceptance: actual original GLB primitives on nearby faces, repeated ground/upper/crown bays, unique signs not repeated by floor, fully prepared resident sources, smooth bounded additive detail with retained physical envelopes and recovery; verification: [source/atlas audit, edge/height/sign tests, native draw-handle/cap census and visual/recovery views](testing/metropolis-authored/README.md). Verified:283 Rust tests, original kit/atlas audits, native original-mesh/cap census and both-branch recovery/restart; full art/palette/performance acceptance remains open.

- [x] **SIDEWALK-01** Widen Metropolis sidewalks and populate curb bands with reusable street furniture. Reason: user requests more sidewalk space and a populated streetscape; part: 05; dependencies: METRO-01 and existing original prop catalog; acceptance: typical3.4m walks, unchanged building IDs/routes/farms, physical furniture on pavement,2m building-side walk,20m priority-junction setbacks, original slab paving, preserved recovery and bounded static placement; verification: [authoring geometry fixtures, preservation manifest, schema tests and native captures](testing/metropolis-sidewalks/README.md). Verified:281 Rust tests, two authoring fixtures, identical rebake, preservation checks and native survey/recovery/restart on both branches; production performance and subjective art approval stay open.

- [x] **METRO-01** Integrate the approved metropolis as a separately selectable playable course. Reason: user approved construction of the race-first layout; part: 04; dependencies: LAYOUT-03/04/05 approved source, existing facade/prop art and fixed gameplay; acceptance: exact source building IDs/heights, full solid envelopes, both routes with ordered gates and branch-aware AI/camera/recovery/HUD, native restart/crash/remount/results and unchanged legacy content; verification: [bake identity, tests, native receipts and screenshots](testing/metropolis/README.md). Verified on both native branch recovery fixtures, fifteen-rider grid restart and automated gate/schema/navigation regressions. Broader art/performance and production playtest gates remain separate.

- [x] **WORLDPLAN-01** Export measured street/property/current-height plans before city rearrangement. Reason: the city needs an explicit layout; part: 04; dependencies: generated world inspection; acceptance: all 657 stable parcels, entrances, proposed owner/fence/access identities, full-route sheets and readable block study; verification: [plans and source fingerprint](planning/world-redesign/README.md). Fence placement, new heights and city rearrangement are not implied.

- [x] **LOD-05** Keep both opaque backup cores behind every scaled facade band. Reason: full-size far shells and fixed-depth near insets could hide authored faces; part: 04; dependencies: LOD-04; acceptance: cross-LOD depth after bay scaling, static near/far insets, no collider changes; verification: all-lot depth regression and 79 native large-facade transitions in [repair evidence](testing/world-repair/README.md). Broad pop-in acceptance remains LOD-03.

- [x] **ANIM-KNEE-01** Correct visual knee bend planes and same-side ankles during pickup and locomotion. Reason: sideways knees/crossed pickup feet; part: 03; dependencies: existing contact IK; acceptance: stable forward pole, unchanged segment lengths, same-side unheld ankle targets; verification: 100 samples each of pickup/mirrored/run/getup plus held-contact/length regressions in [evidence](testing/world-repair/README.md). Physical ragdoll extremes remain separate.

- [x] **COLLISION-08** Gate static-wall wipeouts by normal closing speed. Reason: parking touches caused crashes; part: 01; dependencies: existing contact classification; acceptance: 1 km/h touches and fast parallel scrapes stay mounted, hard direct hits detach, existing car policies preserved; verification: integrated contact tests, car regressions and native recovery smoke in [evidence](testing/world-repair/README.md).

- [x] **AI-09** Adapt ranked roster pace to the modern player bike class. Reason: all original-unit cruise targets were slower than normal player speed; part: 01; dependencies: authored personality profiles; acceptance: shipped roster 94–106 percent of unboosted player top speed without positional rubber-banding or aggression changes; verification: pure roster-pace test and native racing smoke in [evidence](testing/world-repair/README.md). Human race-balance approval remains open.

- [x] **COMBAT-09** Resolve kick-facing outside hit reach and sharpen attack presentation. Reason: wrong-side misses and slow clips; part: 03; dependencies: animation clips/direct-strike geometry; acceptance: nearest opponent within 12 m selects visual leg while unreachable targets take no hit, 1.8× clip playback, distinct original kick transient and confirmed-hit feedback; verification: kick-side/reach and tuck/return tests, PCM validation and native audio/recovery smoke in [evidence](testing/world-repair/README.md). Contact timing/subjective feel remains COMBAT-FEEL-01.

- [x] **ROAD-02** Narrow rural carriageways and verges while retaining the route and lots. Reason: back roads looked like broad urban pavement; part: 04; dependencies: validated track profiles; acceptance: 7.2 m farms / 6.8 m hills carriageway, 0.8 m verges, wider urban road, proportional traffic lane placement and unchanged lot IDs/quotas; verification: width validation, actual-width road-support rays, inspect-world and native route smoke in [evidence](testing/world-repair/README.md).

- [x] **TRAFFIC-04** Double route traffic and yield/honk for a downed rider in lane. Reason: requested traffic density and recovery interaction; part: 06; dependencies: projected recovery targets and existing traffic/audio; acceptance: 24 cars, bounded braking to 5 m clearance, 3 s horn cooldown, resume on clearance and signed actual speed retained for collisions; verification: ten-second integrated queue/horn/resume test, pure braking rule and native 24-car census in [evidence](testing/world-repair/README.md). General intersection and car-to-car queuing remain future work.

- [x] **LOD-04** Integrate model-derived distant facade identity and bounded City Run scene crossfades. Reason: generic far windows changed facade identity and the alternate map still swapped scenes; part: 04; dependencies: LOD-02 and immutable authored source catalog; acceptance: all 128 designs/383 bands, provenance/mip guards, bounded resident proxies/cache/overlap, face-aware coverage, actual native transitions and preserved world/physics; verification: [source, unit, native and census evidence](testing/facade-lod/README.md). User visual/long-streaming approval remains LOD-03.

- [x] **RAG-12** Adopt the user-selected velocity muscles as the default without the native-motor candidate. Reason: user selected velocity after A/B comparison; part: 03; dependencies: shared continuous release and Method B; acceptance: normal launch uses velocity, anatomical constraints/passive resistance and baseline remain intact, paired lab and crash/recovery pass; verification: [selection evidence](testing/velocity-popin/README.md).

- [x] **LOD-02** Remove fallback flashes and incorrect dither during authored facade replacements. Reason: severe user-reported texture pop-in; part: 04; dependencies: LOD-01 and KIT-05; acceptance: direct ready-to-ready transitions, complementary pixel coverage, final-camera range synchronization, material/tangent readiness, early requests and bounded overlap/cache; verification: shader-equivalent regression, native route survey/census and recovery smoke in [evidence](testing/velocity-popin/README.md).

- [x] **RAG-11** Audit original crash animation/control and open-source active-ragdoll approaches before changing gameplay. Reason: requested realistic roll-to-back, protective arms and continuous combat ejection, then findings before implementation; originating part: 03; dependencies: RAG-10, V2-01-006 and existing physical rig; acceptance: identify current teleport/support/roll gaps, pin actual source/licenses/compatibility, distinguish original behavior from proposals, adopt no runtime changes; verification: [source audit and file hashes](research/crash-behavior/README.md). This completes research only; V2-01-006 and V2-03-005 remain open.

- [x] **DRESS-02** Vary roadside scenery through biome-specific groups and independent seeded sides. Reason: repeated mirrored single-object placement; part: 05; dependencies: DRESS-01 and PROP-03; acceptance: at least28 actual roles and16 groups in all five zones, broad zone coverage, bounded224 owners, guarded road/access/support and unchanged tree counts; verification: deterministic/group validation tests, native mesh/role/zone census and farm/city/hill captures ([evidence](testing/roadside-variety/README.md)). Finished prop art and broad performance acceptance remain separate.

- [x] **COLLISION-07** Preserve forward motion through pedestrians, recovering riders and downed bikes. Reason: post-solver stopped velocity erased speed; part: 01; dependencies: COLLISION-04 and entry motion snapshots; acceptance: attacker remains mounted, soft target falls, downed-bike lift spends remaining linear kinetic energy, no backward response; verification: three-preset and pre-sync contact regressions plus native recovery smoke ([evidence](testing/part05-world/README.md)).

- [x] **LOD-01** Transition ready authored facade groups through the detailed fallback over 0.30s. Reason: abrupt scene replacement remained visible; part: 04; dependencies: KIT-05; acceptance: native shader dither, no blank replacement, one displayed group/lot, actual display and hidden budgets enforced; verification: opacity tests and native 15-section displayed-mesh census ([evidence](testing/part05-world/README.md)). Full silhouette morphing and long streaming QA stay open.

- [x] **DRESS-01** Place existing prop models in guarded Valley sidewalk, field-edge, rural and hill sockets. Reason: the restored map lacked semantic dressing; part: 05; dependencies: PROP-03 and structural layout; acceptance: deterministic IDs, whole-footprint road/access/building guards, solid supported owners and bounded counts; verification: placement tests and native 131-owner/376-mesh census ([evidence](testing/part05-world/README.md)). Full prop/category art and behavior gates remain open.

- [x] **MAT-01** Add shared original palette micro-normal/roughness maps, missing tangents and repeated concrete paving. Reason: flat surfaces and missing glTF tangents; part: 05; dependencies: KIT-02 and verified original palettes; acceptance:34 exact source profiles, bounded asynchronous derivation, shared maps, original colors/positions/skinning preserved; verification: all 34 source/decode hashes, PBR/tangent tests and native 934-material census ([evidence](testing/part05-world/README.md)). Bespoke albedo painting stays open.

- [x] **SKY-01** Add an original seamless cloud sky, daylight environment, warm sun, exposure and fog. Reason: user explicitly requested sky/cloud/lighting before the full landscape pass; part: 06; dependencies: current native renderer; acceptance: bounded typed settings, cube mips/seams, rendered drifting clouds and native recovery smoke; verification: cube/config tests and native screenshots ([evidence](testing/part05-world/README.md)). Volumetric weather and full landscape art are not included. [Polish04](testing/polish-12/pass04/README.md) coordinates warm late-afternoon sunlight, cool fill and explicit exposure/tonemapping; human art and hardware acceptance remain open. [Polish09](testing/polish-12/pass09/README.md) adds bounded near-readable aerial haze and shared exposed horizon colour without changing geometry or LOD; [Polish10](testing/polish-12/pass10/README.md) matches the supported non-HDR display horizon, adds layered cloud shading and fixes world sun direction. Clouds are stationary pending independent motion; full landscape/art/hardware acceptance remains V2-06-007.

- [x] **KIT-04** Improve high-rise/infill and road surface detail. Reason: fallback towers/infill were visibly plain and roads were flat gray; part: 04; dependencies: KIT-02 and immutable structural geometry; acceptance: measured-scale batched window reveals/mullions/floor bands/cornices, owner-family infill masonry and original mipmapped asphalt albedo/normal/roughness across main/side roads without collider changes; verification: [relief/PBR tests, native street views and unchanged support/recovery](testing/world-polish/README.md). Full art approval and bespoke roles remain open.

- [x] **KIT-05** Keep facades visible while replacement LODs load and instantiate. Reason: asset/cache changes and early scene swaps caused missing-detail frames; part: 04; dependencies: KIT-02 and native scene intake; acceptance: finite coarse/masonry residency, bounded high-LOD cache and hidden preparation, old scene retained until every replacement instance is ready, spatial hysteresis and displayed-budget assertions; verification: [native instantiated-mesh/preparation census and full-route survey](testing/world-polish/README.md). Perfect geometry crossfades and production streaming stress remain open.

- [x] **PERF-06** Abstract unseen rear NPC crashes to a five-second delay. Reason: the user requests skipping ragdoll simulation for racers behind them; part: 01 performance/contact; dependencies: current crash entry, fixed simulation and reset; acceptance: distant rear/out-of-view mounted NPCs allocate no ragdoll/wreck, wait exactly300 active ticks and return at road centre at the recorded crash station, pause/restart work, player/visible/already-physical crashes unchanged; verification: [eligibility/no-body/timer/pause/return/reset tests and native recovery regression](testing/world-polish/README.md).

- [x] **ANIM-12** Resolve and verify default-stack asset loading. Reproduced the IO worker overflow, configured only asset IO workers with16MiB stacks while retaining Bevy's1–4 thread policy, and verified native Valley and City Run startup/crash/recovery/finish without RUST_MIN_STACK. [Before trace, design and native receipts](testing/world-polish/README.md). Broader platforms/loader memory optimization remain open; no upstream parser stack reduction is claimed.

- [x] **KIT-01** Compile and validate reusable authored facade storeys. Reason: Valley needs the existing modeled assets with real repeated floors rather than whole-building vertical stretching; part: 04; dependencies: original 100 facade/28 structure drafts and source provenance; acceptance: 128 designs, three ground/upper/crown GLB LODs per ID, shared own-textures, unchanged source hashes and independent binary geometry validation; verification: [384 GLBs, 34 shared textures, source/export hashes and band bounds](testing/architecture-valley/README.md). Final art approval remains MESH/V2-04 work.

- [x] **KIT-02** Integrate bounded authored geometry on every existing Valley building lot. Reason: the default Valley still used generic shells despite the modeled City Run assets; part: 04; dependencies: KIT-01 and existing supported lot generator; acceptance: all 657 existing lot IDs mapped through 19 role pools, deterministic in-envelope storey assembly, bounded pending/active scenes and cache, actual native GLB geometry with no added colliders; verification: [all-lot tests, native mesh census, 15-section survey and recovery smoke](testing/architecture-valley/README.md). The measured frame-time target is unmet and remains V2-04-006; this task does not certify production performance.

- [x] **V2-04-005** Retain supported ground, roads, collision and navigation. Verified canonical quotas/IDs/rights of way and geometry fingerprint unchanged; visual kit remains independent of colliders and pad support, with existing route/walk/recovery tests and native smoke passing. [Evidence](testing/architecture-valley/README.md). New building interiors/navigation remain ENV-005.

- [x] **BASE-003** Record dependency versions, purposes, licenses, and notices in `docs/third_party/REPOSITORIES.md`; record media provenance in `docs/third_party/ASSETS.csv`.

- [x] **CAM-02** Prevent actor proximity and low ceilings from collapsing/flipping the chase camera. Reason: reported nearby-body/under-object camera jumps; originating part: 01; dependencies: static obstruction cast and recovery focus; acceptance: moving/owned/NPC bodies do not shorten the boom, low-ceiling flat/lower candidates retain a usable chase direction and degenerate look vectors keep orientation; verification: mounted/on-foot ceiling plus foreign-ragdoll tests at 60/144 Hz, wall regression and native race in [receipt](testing/recovery-polish/README.md).

- [x] **COLLISION-06** Replace upright rear-bike hops with momentum sharing and a 60 km/h closing-speed knockoff. Reason: latest user supersedes the earlier rear-hop request; originating part: 01; dependencies: contact classification and pre-solver velocity snapshots; acceptance: mass-weighted normal-speed exchange, no scripted rear lift, front victim knocked down at threshold, side/head-on/downed routing retained; verification: pair momentum/energy, below/above threshold, both entity orders and real-contact regressions in [receipt](testing/recovery-polish/README.md).

- [x] **ANIM-13** Rewrite the gait and correct downward elbows and neutral wrist alignment. Reason: user rejected walking, raised tuck elbows and sideways wrists; originating part: 03; dependencies: ANIM-11, RAG-09 and shared FK/IK; acceptance: continuous opposing foot/arm cycle, stable forward knee poles, downward held-arm poles and parent-relative neutral distal binds with straight hanging wrists; verification: cycle continuity/chain tests, standing/seated wrist-axis tests and native eleven-motion/three-view review in [receipt](testing/recovery-polish/README.md). Full production/user motion acceptance remains ANIM-09.

- [x] **RAG-10** Add a bounded grounded roll and brief frontal leg-release prototype. Reason: requested side rolling, forward somersault tendency and kick-back variation; originating part: 03; dependencies: accepted biomechanical rig and crash energy guard; acceptance: one-shot support/speed-gated angular cue paid from at most 8% of current horizontal COM energy, direction follows slide, frontal hip/knee targets use capped motors and no direct ejection impulse, baseline/limits preserved; verification: forward/side energy and one-shot tests, unsupported/baseline rejection, motor-only velocity parity and native comparison demo in [receipt](testing/recovery-polish/README.md). Contacts may interrupt rolls; final feel review remains RAG-05/ANIM-09.

- [x] **MAP-01** Restore the original Valley route as the launch default while retaining City Run. Reason: user wants the original mixed-environment map instead of the city test route; originating part: 01; dependencies: both existing track definitions and structural generators; acceptance: bare launch selects Valley, explicit valley/city aliases work without altering geometry/quotas/IDs; verification: default native farm/city/hill recovery/results smoke and retained city tests in [receipt](testing/recovery-polish/README.md).

- [x] **FOOT-001** Crash 5 m before the finish; manually run forward across it -> finish once with valid prior checkpoints. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-002** Crash 5 m before the finish; run away from the bike across the line -> manual control wins and foot finish succeeds. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-004** Hold left/right/back after a crash -> independent chosen movement immediately; release -> assist resumes only after the configured neutral delay. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-005** Walk deliberately away near the bike -> no forced remount or teleport; camera follows rider and HUD shows bike arrow/distance. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-006** Bike and rider cross in the same tick -> exactly one racer-level finish event and payout. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **FOOT-007** Walk backward through finish or skip a checkpoint -> reject until ordered forward progress is valid. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **P06-007** Add immediate camera-relative 360-degree WASD/stick manual walking override, including running away from the bike; resume assist only after 0.25-0.5 s neutral input. Evidence: [Part 03 receipt](testing/animation-part03/README.md), integrated fixed-loop tests and native recovery/HUD review.

- [x] **ANIM-10** Correct resolved punch/kick side and draw elbows inward during tuck/lean while retaining grips. Reason: user-reported side inversion and flared elbows; originating part: 03; dependencies: ANIM-08; acceptance: both attacks select the matching gameplay side and both elbows move inward without stretched chains or grip release; verification: four-side selector, endpoint/length tests and native rear/front/side review in [Part 03 receipt](testing/animation-part03/README.md).

- [x] **ANIM-11** Integrate a visual terrain-gait and physical-landing prototype. Reason: time-only running and a generic get-up start ignored actual displacement/landing; originating part: 03; dependencies: ANIM-08 and physical pose snapshots; acceptance: traveled-distance stride, stop blend, support-normal sole IK, stance locks and captured-pose get-up blend stay renderer-only; verification: slope/chain tests, paired physical parity and native recovery in [Part 03 receipt](testing/animation-part03/README.md). Production terrain/clearance metrics remain V2-03-003/006.

- [x] **RAG-09** Drive the pilot skin with independent wrist/foot/spine joints. Reason: sixteen physical bodies were presented through eleven skin joints; originating part: 03; dependencies: RAG-07 and ART-03; acceptance: sixteen normalized skin influences, independently rotated distal skin joints and unchanged no-input body simulation; verification: skin-weight, distal articulation and paired 180-tick physics tests in [Part 03 receipt](testing/animation-part03/README.md). Prop NPC skins/fingers/final art approval remain RAG-05/ANIM-09.

- [x] **P00-001** Document the original-successor approach and exclude the reference port's code, original game data, sprites, and soundtrack. Evidence: introduction and License sections of this README.

- [x] **P00-002** Establish a top-level production backlog from the blueprint. Evidence: this progress tracker.

- [x] **P00-003** Write a GDD v0.1 under 12 pages with P0/P1/P2 priorities, player fantasy, session length, fun metrics, and performance targets.

- [x] **P00-004** Lock the slice definition: one road, one bike plus two variants, at least three AI, punches/kicks, traffic, crash-to-foot control, valid foot finish, results, and retry.

- [x] **P00-005** Create at least 20 confidence-rated reference observations with source URL, build/version, date, recording description, and independent implementation notes.

- [x] **P00-006** Build bike, weapon, rider/outfit/palette, track, shop, progression, and UI content sheets; distinguish historical observation from new design.

- [x] **P00-007** Maintain research fields `source_id`, `observed_behavior`, `confidence`, `asset_owner`, `allowed_usage`, and `follow_up`.

- [x] **P00-008** Complete a rights register covering code, art, sound, fonts, contributors, attribution, and redistribution; remove unknown-license runtime content.

- [x] **P00-009** **Exit gate:** original-content policy, content schema, backlog, and one-page slice definition exist, with the reference and rights inventories reviewed.

- [x] **V2-01-001** Restart at the authored grid, including on a zero-physics-tick frame.

- [x] **V2-01-002** Give AI proportionate, gradual steering and lean.

- [x] **V2-01-003** Use pre-contact world momentum for physical knockoffs. Pass02 also aligns downed-body/wreck lift eligibility to paired pre-solver samples; [focused contact/recovery audit](testing/polish-12/pass02/README.md).

- [x] **V2-01-004** Repair visual skeleton attachments and anatomical bending. [Verification for corrections 001–004](testing/MOTION_REPAIRS.md).

- [x] **CRASHREF-001** Independently implement original integer contact, damage, entry and scalar drag calculations. Reason: compare actual original crash rules; part: 01; dependencies: pinned R01 and retail facts; acceptance: explicit fixed-point rounding, mass/order, signed velocity, damage scaling, caller-specific boosts and clamps; verification: nine domain tests and [calculation report](research/ORIGINAL_CRASH_CALCULATIONS.md).

- [x] **CRASHREF-002** Verify recovered arithmetic against the local retail executable. Reason: reconstruction comments can differ from native fields; part: 01; dependencies: CRASHREF-001 and user's hashed EXE; acceptance: compare Rust outputs to bounded native x86 execution without distributing original bytes; verification: 23 passing [retail checks](testing/ORIGINAL_CRASHES.md), signed-field discrepancy recorded.

- [x] **CRASHREF-003** Integrate an opt-in bike/bike and bike/car crash comparison while preserving the passive rig. Reason: compare independently of biomechanical PR12; part: 01; dependencies: CRASHREF-001/002 and existing collision/recovery; acceptance: separate rider/wreck entry velocities, persistent comparison condition, explicit SI/geometry adaptation, default behavior retained, remount resets state; verification: adapter tests, native-window smoke and fixed-simulation contact-to-remount regression in [test receipt](testing/ORIGINAL_CRASHES.md).

- [x] **P11-001** Provide placeholder road, trees, mountains, ocean, bikes/riders/cars, solid scenery proxies, and full-footprint road-clearance validation. Evidence: [world](../src/world.rs), [track](../src/track.rs).

- [x] **P11-011** Use one human mesh/proportion/outfit definition for seated bikers, pedestrians, recovery runners and ragdolls; change pose without swapping appearance. Evidence: [shared model and four-state test](../src/human.rs); the new authored skins use [four-state/physics parity tests](../src/models/tests.rs) and [native receipts](testing/MODELS.md).

- [x] **P12-001** Award and persist credits, wins, race count, and a best time; write profile updates through a temporary file and atomic rename in the user data directory. Evidence: [profile](../src/game.rs).

- [x] **P12-002** Test that a finished race pays once across subsequent updates. Evidence: `finish_awards_results_once_and_button_starts_next_race` in [game tests](../src/game.rs).

- [x] **UI-01** Catalog original UI/media and deliver an editable artist skeleton. Reason: user will create replacement art and needs all source references, IDs and screen context first; originating part: 15; dependencies: P00-006 and the user-supplied local game/reference R01; acceptance: complete supplied IMAGES/embedded-icon inventory, original screen/popup/native-dialog geometry, external-only reference gallery, frozen IDs/dimensions/replacement briefs and functioning standalone preview/progress workflow; verification: [142 UI resources,29 populated screen entries,13 popups,7 unique Windows dialogs,28 draft pages,63 SVGs,17 Python tests and browser checks](ui/README.md). No original media shipped and no game UI replaced; artwork/Bevy integration remain UI-02/P15-006.

- [x] **UI-03** Provide original-art generation prompts for every UI screen and source-image replacement. Reason: user will generate the art independently; originating part: 15; dependencies: UI-01 catalog and current rider/bike references; acceptance: all 28 proposed screens and 142 UI image IDs / 66 master families have specific subject, composition, quiet UI areas, aspect/export and identity/variant guidance without baked text or premature approval; verification: [screen and per-image prompts](ui/SCREEN_GENERATION_PROMPTS.md), machine-readable coverage and deterministic rebuild tests in [performance/UI evidence](testing/performance-ui-prompts/README.md). All production art and UI-02 integration gates remain open.

- [x] **UI-04** Generate and place all 28 proposed screen drafts in the artist preview. Reason: user requests ChatGPT image generation and usable screen placement; originating part: 15; dependencies: UI-03 prompts and existing project model views; acceptance: one saved traceable PNG per screen, real alpha for HUD/pause/confirmation, persistent matching-page art, live separate labels/buttons, gallery/download/review metadata and retained wireframe/source-resource workflows; verification: [28 decoded browser pages, PNG/alpha/hash checks and review/download workflow](testing/generated-ui/README.md). The first batch’s art direction was subsequently rejected; the user approved the grotesque painted style and four additional original scene variations are saved for individual review (docs/ui/UI_ART_DIRECTION.md). This verifies historical draft availability only; user art approval, layered/high-resolution masters and native Bevy integration remain UI-05/UI-02.

- [x] **P15-001** Provide baseline PBR lighting, shadows/fog, menus/HUD/results, engine/wind/nitro/braking loops, impacts/footsteps/countdown/finish cues, and mute. Evidence: [world](../src/world.rs), [UI](../src/ui.rs), [audio](../src/sound.rs).

- [x] **P15-002** Generate original PCM sounds in memory; verify valid non-silent WAV data and initialized/playing audio sinks in the native smoke check. Evidence: [sound test](../src/sound.rs), [render/audio check](../src/verification.rs).

- [x] **P16-001** Maintain functional regression tests for steering, circuit/projection, countdown/nitro/pause, combat, physical contacts, repeated knockdowns, camera tracking, full recovery, payouts, and scenery clearance. Evidence: tests in [game](../src/game.rs), [track](../src/track.rs), [world](../src/world.rs), [sound](../src/sound.rs).

- [x] **P16-002** Build and run a local Linux release; smoke-check garage -> riding -> crash -> get up -> run -> remount -> results with active audio. Evidence: [verification script](../src/verification.rs).

- [x] **AUDIO-014** Exclude proprietary original Road Rash music, voices, and sound files from the current build; current effects are synthesized in [sound](../src/sound.rs).

- [x] **CITY-01** Add side-road grids and denser urban blocks; compare road/sidewalk/frontage proportions against the verified original disc. Evidence: [city/combat verification](testing/CITY_COMBAT.md), 657 solid buildings, native city captures and 9.56 ms p95 survey.

- [x] **COMBAT-01** Space auto-side kick pushes bikes and animates the selected leg; mouse buttons punch left/right. Evidence: nearest-side/push/input tests and native left/right kick captures in [verification](testing/CITY_COMBAT.md).

- [x] **RIDE-01** Correct rival camera obstruction and supported contact-yaw instability; add symmetric input-only lean presentation. Evidence: crowd-camera/yaw tests, seven native bank captures and crash/recovery smoke in [verification](testing/CITY_COMBAT.md).

- [x] **CITY-02** Join neighboring city buildings into continuous street fronts. Reason: the user rejected detached six-metre gaps; part: world development; dependencies: CITY-01, solid lots and recovery routing; acceptance: shared-edge infill, clear streets, exposed-perimeter recovery paths and road-height screenshots; verification: 347 shared-edge sections, physics rays for every infill, 113 passing tests and native road-height survey; [evidence](testing/FRONTAGES.md).

- [x] **CITY-03** Improve sidewalks, plant solid street trees and raise urban buildings. Reason: user requested better pavement and a taller, greener city; part: world development; dependencies: CITY-01/02 and shared ground; acceptance: flat sidewalk shading, clear walking strip, solid trees outside junctions, taller whole-floor buildings without lot changes; verification: geometry/physics tests, native survey and crash/recovery smoke; [evidence](testing/STREETSCAPE.md).

- [x] **COMBAT-02** Strengthen kick displacement and adopt the fifteen-racer CITY roster with reference-inspired shirt, helmet and bike colors. Reason: user requested original roster/color schemes and stronger kicks; part: combat/world; dependencies: COMBAT-01 and shared human meshes; acceptance: 14 named rivals, consistent minimap/ragdoll colors and stronger signed physical shove; verification: combat/UI tests and native captures; [evidence](testing/RIDERS.md).

- [x] **CITY-04** Thin sidewalk trees from 22 to 44 m spacing. Reason: user found the avenue overplanted; part: world; dependencies: CITY-03; acceptance: roughly half the sites with existing clearances/collision; verification: placement tests and native survey.

- [x] **RIDE-02** Require player steering through curves, increase turn strength 50%, and topple abandoned bikes. Reason: user reported automatic driving and upright wrecks; part: Parts 03/04; dependencies: dynamic chassis and recovery; acceptance: neutral heading stays independent of road, stronger turn settings and physical roll without linear launch; verification: heading/roll/momentum/remount regressions and native smoke.

- [x] **AI-01** Add measured rival pursuit, speed matching and selective attacks on downed riders. Reason: user requested reference-inspired fighting/chaos; part: AI follow-up; dependencies: relative contacts and crash recovery; acceptance: start grace, bounded approach, alongside spacing, gradual speed matching, limited downed hunts and attack cooldowns; verification: deterministic pursuit scenarios, regression suite and native survey. Original relationship-table parity remains future work.

- [x] **AI-02** Match individual level-zero rival pace/stats and relationship-driven race/follow/fight choices; start YOU last with 12 m gaps. Reason: user rejected uniform aggression and crowded/front starts; part: AI follow-up/track setup; dependencies: AI-01 and supported dynamic grid; acceptance: distinct validated profiles, selective hostility and hit reactions, per-rider pace/strength/stamina/weight, solid separated rear grid and retry; verification: profile/decision/physical-grid tests plus native smoke/survey; [evidence](testing/PERSONALITIES.md).

- [x] **ART-01** Research rigged character sources matching the user's faceted, natural-proportion references and biker gear. Reason: user requested a visual upgrade; part: character art; dependencies: reference images and shared human lifecycle; acceptance: sourced shortlist, format/rig claims and license review; verification: publisher/marketplace pages reviewed 2026-10-31; [findings](third_party/CHARACTER_CANDIDATES.md).

- [x] **ART-03** Integrate the approved authored rider and motorcycle as presentation-only replacements, retaining all accepted ragdoll bodies/joints and crash/recovery behavior. Reason: explicit user request; part: 11 art; dependencies: authored GLBs/seat pose and existing visual joints; acceptance: shared rider through riding/walking/ragdoll, bike/wreck replacement, unchanged physics trajectories, complete recovery, native visual check; verification: [tests](../src/models/tests.rs), [evidence](testing/MODELS.md).

- [x] **P01-001** Pin a Rust toolchain and dependencies; provide formatting, Clippy with warnings denied, and Linux headless tests in CI. Evidence: [toolchain](../rust-toolchain.toml), [manifest](../Cargo.toml), [CI](../.github/workflows/ci.yml).

- [x] **P01-002** Launch a native Fedora/Linux 3D build with ground, lighting, camera, keyboard input, menus, pause, and reset. Evidence: [app](../src/main.rs), [render smoke check](../src/verification.rs).

- [x] **P01-005** Implement Boot/MainMenu/Loading/Race/Results lifecycle and fixed 60 Hz gameplay/physics scheduling, with independent render interpolation.

- [x] **P01-008** Add FPS, average/P95 frame time, 60 Hz tick count, entity/collider counts, physics timing, toggleable debug UI, logs, and panic reports.

- [x] **P01-012** Verify 30/60/120 render-FPS simulation equality and 100 crash/pause/resume/reset cycles without actor leaks. Evidence: [game tests](../src/game.rs).

- [x] **P01-013** Preserve and consume short attack inputs once across zero/multiple fixed ticks. Evidence: [core buffer](../crates/rr_core/src/lib.rs), [input tests](../src/game.rs).

- [x] **P01-014** Version prototype profiles while retaining legacy progress and rejecting unsupported/corrupt data. Evidence: [profile tests](../src/game.rs).

- [x] **P01-015** Provide a headless content-validation CLI and local panic report process probe. Evidence: [entry point](../src/main.rs), [test receipts](testing/PARTS_00_01.md).

- [x] **P01-016** Keep the complete README/tracker task index synchronized by stable IDs. Evidence: [sync checker](../tools/sync_progress.py).

- [x] **P01-017** Audit declared dependency/media provenance, font notices, source hashes and excluded unverified embedded LUTs. Evidence: [audit](../tools/audit.py), [inventory](third_party/ASSETS.csv).

- [x] **P01-019** Decode Cargo metadata and repository text explicitly as UTF-8 across host locales; regress the Windows cp1252 audit failure. Evidence: [audit tests](../tools/test_audit.py), [CI follow-up](testing/PART_02.md).

- [x] **P02-001** Build a closed driveable circuit with lanes, shoulders, solid road surfaces, and canonical arc-length progress. Evidence: [track](../src/track.rs), [world](../src/world.rs).

- [x] **P02-002** Validate circuit closure, distance wrapping, camera-right lane direction, progress projection, and full-footprint scenery clearance in headless tests. Evidence: tests in [track](../src/track.rs) and [world](../src/world.rs).

- [x] **P02-003** Document meter units, +Y up, spline-tangent forward, origin, and any future origin-rebasing policy.

- [x] **P02-005** Define typed track IDs, control points, surfaces, ordered checkpoint IDs, prop placement, spawn points, and safe foot/verge regions.

- [x] **P02-006** Build simplified collision meshes separately from render meshes, initially sampling around 2-3 m; add curbs, guardrails, fencing, and reliable highway boundaries.

- [x] **P02-007** Implement ordered gates and forward crossing validation; validate checkpoint order, intersections, NaNs, negative widths, and spawn overlap.

- [x] **P02-008** Add asphalt/dirt/gravel/grass/water metadata and traction/friction modifiers, animal crossings, and cliff hazard zones.

- [x] **P02-009** Add a track inspector for spline normals, triangle count, driving line, bounds, and spawns; load authored tracks headlessly.

- [x] **P02-012** Build the user-directed point-to-point map with distinct start/finish, no endpoint wrapping, canonical route HUD and safe endpoint/retry behavior. Evidence: [track](../src/track.rs), [race tests](../src/game.rs).

- [x] **P02-013** Populate seeded farm openings/fences/cows, city buildings/cross traffic/walking pedestrians, and elevated hills with physical guardrails. Evidence: [environment](../src/environment.rs), [native captures](testing/PART_02.md).

- [x] **P02-014** Keep terrain below banked road lanes and verify route support, render/collision winding and independent collision resolution. Evidence: [geometry tests](../src/environment.rs).

- [x] **P02-015** Verify actual hill crash/get-up/run/remount at elevation without resetting the bike to sea level. Evidence: [recovery](../src/recovery.rs), [native receipt](testing/PART_02.md).

- [x] **P02-016** Keep hill guardrails on outer curves, open the straights, and add varied trees in all zones plus window/door/roof, cross-vehicle and pedestrian detail. Evidence: [environment](../src/environment.rs), [native captures](testing/PART_03.md).

- [x] **BW-B00** Capture existing track/world statistics, measured legacy building census, reproducibility and native baseline views.

- [x] **BW-B01** Validate 15 sections, 19 archetypes, lot rules and exact 247/177/70/family budgets; allocate stable seeded planning IDs headlessly.

- [x] **BW-B02** Bind districts to baked road stations, resolve/blend all boundaries and final endpoint, reject mismatched courses and expose F4 preview.

- [x] **BW-B03** Generate field parcels and farm/suburban/city/hill ground classification across all sections.

- [x] **BW-B04** Build at least seven reserved junctions, side-street foundations and continuous accessible curbs/sidewalks.

- [x] **BW-B05** Generate non-overlapping, reachable frontage/secondary lots outside every road/junction/gate/spawn reservation.

- [x] **BW-B06** Place exactly 247 structural building roots with mass/roof/collision and validated full-footprint clearance.

- [x] **BW-B07** Grade supported pads and hillside foundations with shared render/collision heights and local cut/fill.

- [x] **BW-B08** Prove farm/city-alley/hill on-foot recovery and escape paths without geometry traps.

- [x] **BW-B09** Implement spatial building LOD/chunks while preserving collision/IDs and measure representative 1080p performance.

- [x] **BW-B10** Complete the full-map survey, plan export, count/geometry hashes, provenance, regressions and all base-world exit criteria.

- [x] **BW-X01** Protect paired LOD origins and local terrain vertices against disappearing ground; verify all 160 tile pairs and building children plus the native atlas.

- [x] **BW-X02** Record actual survey resolution/focus and use continuous uncapped benchmark updates so background idle timers cannot masquerade as frame cost.

- [x] **BW-X03** Clear recovery motion/routes on exceptional rescue and apply one visible five-second penalty; verify normal capsule routes and rescue state transitions.

- [x] **P03-001** M0: reach the finish with a kinematic road-following arcade controller, throttle, braking, steering/lean visuals, and nitro. Evidence: [racing controller](../src/game.rs).

- [x] **P03-002** Integrate collision-driven knockoff, continuous movement checks against solid geometry, and impact launch/landing/bounce behavior. Evidence: [controller](../src/game.rs), [impacts](../src/recovery.rs), steering/collision/bounce tests. Rider-to-rider contact is now severity-based (P06-019); historical universal hops are superseded.

- [x] **P03-003** M1: implement a dynamic chassis with mass, center of mass/inertia, upright assist, and separate rider/wheel visuals behind `VehiclePhysicsAdapter`.

- [x] **P03-004** M2: add front/rear wheel contact rays, spring-damper suspension forces, engine braking, service braking, and a torque/RPM/redline/gear model.

- [x] **P03-005** M3: implement surface-dependent combined longitudinal/lateral traction budget, slip, controlled drift, speed-sensitive steering, aerodynamic drag proportional to speed squared, and low-speed self-righting.

- [x] **P03-006** Define a simulation-facing `BikeState` with pose/velocity, yaw/lean, slip, grounded wheels, actual RPM/gear, damage, and rider condition references.

- [x] **P03-007** Author three data-driven handling presets: light/agile, balanced, and heavy/powerful; initially target 150-290 kg dry plus rider, 0-100 km/h in 3.5-6.5 s, and a roughly 50-60 degree lean clamp.

- [x] **P03-008** Expose live tuning and per-tick telemetry: bike ID, tick/dt, pose, velocity, inputs, yaw rate, lean, wheel contacts, spring forces, slip, surface mu, damage, and crash state.

- [x] **P03-009** Benchmark acceleration, braking distance, steady turns, lean recovery, and curb hits; compare numeric results at 30/60/120 render FPS and collect 100 Hz traces where needed.

- [x] **P03-012** Decelerate smoothly when boost is released or exhausted; preserve excess speed instead of clamping to the ordinary top speed. Evidence: [regression](../src/game.rs).

- [x] **P03-013** Allow player/AI forward movement in grass, with reduced traction/speed and no grass-only health loss or knockoff; AI can return to the road. Evidence: [four-scenario regression](../src/game.rs).

- [x] **P03-014** Tighten lateral handling approximately 20% using validated authored steering rates, retaining older schema-1 catalogs with defaults. Evidence: [tuning](../content/catalog.json), [schema tests](../crates/rr_content/src/lib.rs).

- [x] **P03-015** Reduce road assistance without pulling the rider to lane zero; expose F6 adjustment and preserve free lane choice. Evidence: [stationary-offset regression](../src/vehicle_tests.rs).

- [x] **P03-016** Feed AI and human tick intentions through the same dynamic adapter, retaining AI traffic avoidance and launch grace. Evidence: [controller](../src/game.rs), [dynamic grid regression](../src/vehicle_tests.rs).

- [x] **P03-018** Investigate the C++ reference's steering, drive/braking, traction, gear, nitro and motion paths at an exact revision, separating source facts from missing calibration data. Evidence: [investigation](research/ROAD_RASH_HANDLING.md).

- [x] **P03-019** Accumulate player steering force while held, unwind on release/reversal, scale the target with speed, and prevent parked sideways driving. Evidence: [pure rules](../crates/rr_core/src/steering.rs), [integration tests](../src/vehicle_tests.rs). Timing values remain provisional RoadDash tuning.

- [x] **P03-021** Double full-lean response left/right by halving steering buildup/return times and doubling visual bank response, retaining angle/force limits. Evidence: [symmetric rate test](../crates/rr_core/src/steering.rs), [full lean/neutral test](../src/vehicle_tests.rs).

- [x] **P03-022** Increase full-lean turn strength by 25% in each handling class, with shared profile gain in both driving adapters and unchanged visual bank. Evidence: [left/right comparison](../src/vehicle_tests.rs), [verification receipt](testing/HANDLING_CORRECTION.md).

- [x] **P03-017** Preserve selected dry bike mass in abandoned wrecks and clear stale velocity on dynamic remount. Evidence: [mass/recovery regressions](../src/vehicle_tests.rs).

- [x] **P04-001** Provide smoothed follow/look-ahead, near/default/far chase views, speed-sensitive FOV, impact shake, and scenery collision avoidance. Evidence: [camera](../src/game.rs).

- [x] **P04-002** Provide keyboard throttle/brake/steering, pause/reset, and correct left/right steering in chase-camera space. Evidence: [input and regression tests](../src/game.rs).

- [x] **P04-003** Add speed-linked engine pitch and wind feedback. Evidence: [sound](../src/sound.rs).

- [x] **P04-005** Add gamepad stick steering/freelook, analog throttle/brake triggers, attack bindings, deadzones, and key/controller rebinding.

- [x] **P04-006** Provide persistent FOV/smoothing/anti-nausea settings; target configurable 60-95 degrees, about 75 degrees default, with optional reduced shake.

- [x] **P04-007** Add readable slip/traction feedback and optional speed lines; allocate the independent on-foot input context.

- [x] **P04-011** Verify camera continuity at 60/120/144 Hz, swept-volume obstacle clearance and safe large-warp snapping using automated scenarios. Evidence: [camera tests](../src/camera_tests.rs).

- [x] **P04-012** Preserve malformed/future settings files, reject conflicting bindings, and prevent modal settings input leaking into racing. Evidence: [settings](../src/settings.rs), [input tests](../src/controls.rs).

- [x] **P04-013** Carry analog triggers into chassis forces and clear input on controller disconnect while retaining keyboard fallback. Evidence: [adapter test](../src/vehicle_tests.rs), [device tests](../src/controls.rs).

- [x] **P04-015** Historical correction: keep the character upright during steering. This was verified in Part 04, then superseded by the user’s coordinated, input-only bike/rider lean request in P04-016. [Historical receipt](testing/PART_04.md).

- [x] **P04-016** Bank bike and rider together from accumulated steering input only; remove road/chassis bank from rendered roll and preserve the shared human model. Evidence: [bank/road-neutral test](../src/human.rs).

- [x] **P04-017** Keep near/default/far camera views close at high speed, eliminate follow-lag zoom-out, and reduce optional speed FOV to three degrees. Evidence: [27 camera scenarios](../src/camera_tests.rs), [native receipt](testing/HANDLING_CORRECTION.md).

- [x] **P04-010** Color only racer minimap dots by their shirt palette, sharing the model color source. Evidence: [palette test](../src/ui.rs), [shirt colors](../src/world.rs).

- [x] **RIVAL-WEAPONS-01** Equip a seeded random subset of rivals at race start/retry. Reason: user requests starting weapons on random riders; originating part: 07; dependencies: P05-003, validated weapon content and existing reset/held-socket paths; acceptance: default seven of fourteen rivals armed, player/police excluded, reproducible seed/order-independent selection, reroll only on new grids, retained AI grace/retaliation, drop/recovery and bounded resident visuals; verification: [375 workspace tests, seeded native equipped/held census and isolated entry receipt](testing/random-rival-weapons/README.md). Full arsenal and human balance gates remain open.

- [x] **P05-001** Implement left/right punches, directional range checks, damage, cooldown, simple arm animation, hit sound, and HUD feedback. Evidence: [combat](../src/combat.rs), [animation](../src/world.rs), [sound](../src/sound.rs).

- [x] **P05-002** Test directional damage and a single takedown reward. Evidence: `directional_melee_damages_the_target_and_awards_one_takedown` in [game tests](../src/game.rs).

- [x] **P05-003** Add left/right kicks, then one simple held weapon and pickup rules. Evidence: [first-club implementation and actual native keyboard pickup/contact/drop](testing/part07-weapons/README.md), fixed moving contact/LOS/range/AI/drop/crash/reset tests; broad arsenal, grip art, balance and exit gates remain open.

- [x] **P05-009** Require three direct side-by-side hits with no more than three seconds between hits, with reach/side/height/heading/occlusion checks, one target per attack and duplicate-hit protection. Evidence: [core rules](../crates/rr_core/src/lib.rs), [melee](../src/combat.rs), [regressions](../src/game.rs).

- [x] **P06-001** Trigger physical knockoff from attacks, vehicle contacts, and solid roadside obstacles; maintain rider health and crash state. Evidence: [impacts](../src/recovery.rs), [controller](../src/game.rs).

- [x] **P06-002** Use separate owner-linked ragdoll, abandoned bike, and on-foot actors; implement falling -> getting up -> assisted run -> right bike -> remount. Evidence: [recovery](../src/recovery.rs).

- [x] **P06-003** Provide an 11-body jointed ragdoll, CCD, friction/restitution, collision filtering for the hidden riding body, and a solid on-foot capsule with step/slope handling. Evidence: [recovery](../src/recovery.rs).

- [x] **P06-004** Follow the physical torso and on-foot rider with the camera; allow traffic/rivals to knock a recovering rider down again without duplicate ragdolls. Evidence: [camera](../src/game.rs), recovery-camera and repeat-knockdown tests.

- [x] **P06-005** Provide impact/footstep cues and verify the rendered crash/get-up/run/remount sequence. Evidence: [sound](../src/sound.rs), [render check](../src/verification.rs).

- [x] **P06-014** Preserve current crash momentum without synthetic upward launch and visibly separate the rider from the riderless wreck. Evidence: [crash tests](../src/game.rs), [recovery](../src/recovery.rs), [native receipt](testing/PARTS_00_01.md).

- [x] **P06-015** Remount against the actual fallen bike collision shape, including sideways, upside-down and vertical poses, without the runner pushing it away. Evidence: twelve automated approaches in [game tests](../src/game.rs) and native recovery smoke.

- [x] **P06-016** Apply post-remount protection to shoulder damage so recovery does not immediately drain health. Evidence: end-to-end recovery regression in [game tests](../src/game.rs). Historical behavior: P03-013 now removes grass-only damage entirely.

- [x] **P06-017** Replace universal ball-joint limits with anatomical stops, directional hinges and bounded passive tissue resistance; verify 100 high-speed drops without detached joints or twisted hinge axes. Evidence: [articulation](../src/recovery.rs), [stress test](../src/game.rs).

- [x] **P06-018** Make fallen bikes slide on steep solid terrain while retaining flat-road stopping friction. Evidence: [slope contact friction](../src/recovery.rs), [ramp regression](../src/game.rs).

- [x] **P06-019** Classify motorcycle wipeouts by relative closing speed: >=50 km/h longitudinal or substantial side impact; lighter contacts nudge without universal launch/knockoff. Evidence: [severity rules](../crates/rr_core/src/lib.rs), [contact tests](../src/game.rs).

- [x] **P06-020** Add distinct pedestrian/cow ragdolls with direct-hit and solid-impact paths, an eight-actor budget, safe delayed standing and complete reset cleanup. Evidence: [NPC systems/tests](../src/npc.rs), [native actor capture](testing/PART_03.md).

- [x] **P06-022** Launch a mounted biker above cows/pedestrians using obstacle-height and mass-aware clearance impulses; preserve horizontal approach momentum and civilian ragdolls. Evidence: [six clearance cases](../src/vehicle_tests.rs), [measurements](testing/part-04/hazard-clearance.json).

- [x] **P06-021** Treat sidewalks as rideable support so low curb contact raises the bike instead of causing an obstacle wipeout. Evidence: [suspension](../src/vehicle.rs), [curb-transition test](../src/vehicle_tests.rs).

- [x] **P07-001** Implement countdown, laps/progress standings, race clock, speed/armor/nitro readouts, minimap, results/payouts, and restart. Evidence: [race](../src/game.rs), [HUD](../src/ui.rs).

- [x] **P07-002** Persist basic race count, wins, credits, and personal-best time; test one-time finish rewards and restarting. Evidence: [profile and finish tests](../src/game.rs).

- [x] **P08-001** Populate seven rival racers with curve-speed planning, lane changes, traffic avoidance, attacks, and recovery through the shared recovery system. Evidence: [AI controller](../src/game.rs), [recovery](../src/recovery.rs).

- [x] **P08-002** Populate two-way civilian traffic with solid vehicle colliders. Evidence: [traffic setup/update](../src/game.rs).

- [x] **P08-006** Add police patrol, violation assessment, chase/intercept/bust, loss of target, and return-to-patrol states with testable transitions. Evidence: [resident police implementation, deterministic transitions, safe contact/priority and native all-state bust/escape/retry](testing/part08-police/README.md). This verifies the narrow police-state requirement; bespoke art, moving-foot capture, discretionary encounters, human fairness and full offline/performance exit gates remain open.

- [x] **P08-011** Treat crossing traffic as an explicit wipeout hazard with continuous/all-type contacts, including a moving car striking a stopped bike. Evidence: [four physical collision cases](../src/vehicle_tests.rs).

- [x] **P08-010** Use American right-hand traffic lanes and separate one-way cross-traffic lanes; stop automatic AI fights/weaving during grid launch. Evidence: [traffic](../src/game.rs), [cross streets](../src/environment.rs), [grid tests](../src/vehicle_tests.rs).

- [x] **COLLISION-01** Keep mounted heading input-owned through contacts. Reason: contact spin steers the bike; part: 01; dependencies: dynamic chassis and fixed input; acceptance: neutral input cannot acquire collision yaw, gradual intended steering and recovery still work; verification: repeated spin at 30/60/120 Hz, handling/camera regressions and native smoke.

- [x] **COLLISION-02** Remove scripted collision launches and spawn spin. Reason: latest request supersedes civilian clearance jumps; part: 01; dependencies: separated rider/wreck and civilian knockdowns; acceptance: zero added upward/spin velocity at entry, gravity-led wreck toppling, civilian momentum sharing and working recovery; verification: six civilian/bike-class cases, entry snapshots, slope/toppling tests and native actor/race smoke.

- [x] **COLLISION-03** Apply measured arcade vehicle speed loss and a dissipative crash envelope. Reason: player/AI crashes reverse or gain energy; part: 01; dependencies: pre-contact samples and physical body mass/inertia; acceptance: 120 km/h rider versus either direction of 40 km/h car seeds 80 km/h forward, never negative retained speed, no added aggregate energy beyond gravity or reverse crash centre-of-mass motion; verification: player/AI directional cases, kinetic/potential/rotation budget checks, cross-traffic contacts and native recovery.

- [x] **COLLISION-04** Separate fallen rider/wreck ride-over contacts from mounted wipeouts. Reason: any detached limb or bike triggered another crash; part: 01; dependencies: separated bodies and contact events; acceptance: solid downed contacts do not script another wipeout, low shapes support passage, mounted glances nudge, standing runners and car crashes remain functional; verification: player/AI and own/foreign body matrix in both crash modes, physical wreck traversal, full regressions and native crash/recovery smoke. [Evidence](testing/contact-routing/README.md).

- [x] **PROP-01** Deliver exhaustive scoped world-prop reference extraction and review infrastructure. Reason: the building workflow must extend to all roadside props and actors without hidden exclusions; originating part: 05; dependencies: local source resources and building catalog; acceptance: every indexed CEL in all families, five vehicle libraries and global CELs is accounted for, stable IDs/aliases/views retained, labeled 3×2 sheets/searchable gallery and explicit rebuild stages; verification: [4,172 source CELs, 1,310 images, 597 review banks, 105 sheets, independent coverage/image/zip validation and browser checks](testing/prop-catalog/README.md). This verifies extraction, not completed prop models or production art.

- [x] **PROP-02** Deliver source-mapped draft volumetric prop exports and an isolated GLB inspector. Reason: move the full bitmap catalog into editable 3D authoring; originating part: 05; dependencies: PROP-01 and the 369-ID concept queue; acceptance: all IDs have actual volumes, three reduced geometry LODs, original self-contained materials, editable category sources, independently re-imported render proofs and a working native viewer; verification: [binary/LOD/material/ID checks, authoring regressions and native smoke](testing/prop-meshes/README.md). This closes draft export infrastructure only; V2-05-002 retains concept fidelity, finished art, rigs, collider/placement and performance gates.

- [x] **PROP-03** Apply selected draft props in the playable maps. Reason: connect the 3D catalog to gameplay; originating part: 05; dependencies: PROP-02, existing traffic/NPC/physics and city reservations; acceptance: working moving/cross traffic, trees/lamps, clear sidewalk/parallel-street sockets, consistent human/cow visuals through knockdown, independent preserved body definitions and momentum, validated LOD skins, native survey and recovery smoke; verification: [integration tests, actual-game screenshots and selected-ID receipts](testing/props-in-game/README.md). This verifies development integration, not all 369 banks placed or final art/performance/release approval.

- [x] **V2-02-004** Investigate animation, IK and active-ragdoll repositories. Evidence: [five pinned source comparisons, licenses and compatibility](research/ANIMATION_AND_RAGDOLL.md); no library or source imported.

- [x] **ANIM-01** Deliver the first reviewable static pose catalog and authoring validator. Reason: Animation A needs reproducible review assets before playback; originating part: 02; dependencies: existing authored rider/bike/contact rig; acceptance: 247 numbered draft candidates across 27 use cases, offline gallery, editable single-frame pose assets and independent connected-rig/contact validation; verification: [140 Rust / 14 Python tests, all-pose Blender round trip and gallery review](testing/ANIMATION_A.md). This completes the review infrastructure, not pose approval or final animation coverage. V1 semantic labels were rejected and are superseded by the context-first trace below.

- [x] **ANIM-05** Recover source-ordered action/phase/view context before reconstruction. Reason: V1 misclassified inspection-helper slots; originating part: 02; dependencies: supplied retail build and pinned reference selectors; acceptance: all 104 bank/code slots, 194 ordered descriptors and 371 sprite-slot usages include semantic IDs, mirror/view variants, controls, timing counters and explicit fallbacks/unknowns; verification: independent decoder versus separately compiled reference (194 descriptor/rear matches), reproducible metadata and semantic/bounds tests. [Corrected catalog and limits](animation/context/README.md).

- [x] **ANIM-07** Correct torso and support motion in the source-linked mounted action keys. Reason: limb-only poses missed the original silhouettes; originating part: 02; dependencies: ANIM-05 and shared authoring contacts; acceptance: punch lean/target look, raised kick hips/released seat, full-body tuck and mirrored phases, with retained fixed-length support contacts; verification: 138 static Action reloads, 20 Python tests and regenerated labeled review renders. [Evidence and art limits](testing/ANIMATION_B.md).

- [x] **ANIM-08** Integrate the first full-body normal/tuck/turn/combat/recovery clip set. Reason: static keys need continuous presentation; originating part: 03; dependencies: ANIM-07 and existing visual skin adapter; acceptance: 36 keys / 13 clips, current-pose action entry/exit, steering-only turns, reference-informed tuck selection, held-contact constraints, pause/crash interruption and pickup space handoff without physical writes; verification: Rust sampler/state/contact/parity tests, nine authoring GIFs and native crash/remount/finish smoke. [Evidence and remaining production gates](testing/ANIMATION_B.md).

- [x] **RAG-01** Audit the current rig and five requested biomechanics references. Reason: the existing system already had limits and must be diagnosed before replacement; originating part: 03; dependencies: current source/lockfile and reference source access; acceptance: actual joint/CCD/mass/bind/control causes, five pinned source and license decisions, no unlicensed imports; verification: [investigation](research/RAGDOLL_BIOMECHANICS.md), repository register and ADR-036.

- [x] **RAG-02** Implement the 16-body anatomical articulation and passive resistance. Reason: wrists/ankles/split spine and bounded resistance were missing; originating part: 03; dependencies: RAG-01 and native Rapier; acceptance: calibrated local frames, elbow/knee hinges, bounded neck/spine/shoulders/hips/wrists/ankles, safe quaternion chart, 70 kg distribution/shape inertia, persistent passive motors and predictive collision; verification: native joint/torque tests and [nine dual-mode fixtures](testing/ragdoll/lab-results.json). Original eleven-body control remains selectable.

- [x] **RAG-03** Integrate capped muscles and coherent crash release. Reason: crash states lacked smooth loss of strength and animation/spin handoff; originating part: 03; dependencies: RAG-02 and animation bind frames; acceptance: configurable native PD pose tracking, bracing/ejection/tumble/rest curves, bind-calibrated initial pose with coincident anchors, COM/pre-impact spin preservation, unchanged ejection events, no forced upright motion and natural sleep; verification: pose-hold/external-force/state/bind/momentum tests, bounded energy and 15-rider settling results in [report](testing/RAGDOLL_BIOMECHANICS.md).

- [x] **RAG-04** Deliver the reproducible ragdoll comparison and tuning tools. Reason: the overhaul must be reviewable before merging; originating part: 03; dependencies: RAG-02/03; acceptance: central validated JSON, F7 shape/axis/limit/motor diagnostics, nine headless and native scenarios, B/reset baseline comparison, native clips and recovery/actor regressions; [automatic launch selection](testing/ragdoll/default-launch/README.md) with explicit baseline override (2026-11-02); verification: 164 Rust tests, 20 tooling tests, native comparison/race/actor smoke, build/Clippy/format/audit [receipts](testing/ragdoll/receipt.json).

- [x] **RAG-07** Integrate the approved biomechanical rig with current City Run, props and contact routing. Reason: comparison branch predates the live race; part: 03; dependencies: RAG-04, CITYGEN-01, PROP-03, COLLISION-04; acceptance: automatic new mode, explicit original baseline, current human/cow skins and recovery, original-crash direction policy retained; verification: [200 Rust tests, native paired race/actor/view and 18 lab runs](testing/ragdoll/integration/README.md). User authorized merge on2026-11-06; distal art and broad performance gates remain open.

- [x] **RAG-08** Strengthen passive hip/knee resistance and constrain inward leg crossing. Reason: user reported crossing knees and loose legs; part: 03; dependencies: RAG-02/07; acceptance: mirrored hip adduction/twist limits, persistent stronger leg motors, knees remain separated under paired inward torque, improved equal-load deflection and bounded high-speed settling; verification: [physical knee-load/separation tests and native settled capture](testing/ragdoll/integration/README.md). Baseline control articulation remains selectable.

- [x] **COLLISION-05** Hop over rear/downed rider and bike contacts without ejecting the attacking rider. Reason: user requested ramp-like arcade contacts; part: 01; dependencies: COLLISION-04; acceptance: bounded speed-gated hop, fast rear hit knocks target down, downed/runner impacts push or knock target over, own bodies and tiny taps excluded, repeated limb hits deduplicated by cooldown, head-on/car behavior retained; verification: [both entity orders, actual dynamic contacts, no duplicate bodies and native recovery tests](testing/ragdoll/integration/README.md). This user-requested hop supersedes the earlier no-launch policy for these contact classes only. Upright rear hops are superseded by COLLISION-06; downed-body/wreck hops remain.

- [x] **DISTRICT-01** Organize all 146 modeled IDs into district/support sets and original-course eligibility indexes. Reason: procedural scenery needs coherent local pools instead of a global random mix; originating part: 04; dependencies: complete draft mesh catalog and local resource research; acceptance: exactly one primary group per ID, five measured course-family views, labeled previews, canonical mesh links and explicit evidence/inference separation; verification: [146 name matches, 13 groups, 438 unchanged linked LODs, parser/coverage tests](testing/building-districts/README.md). District placement and transitions remain under V2-04-003.

- [x] **CITYGEN-01** Assemble full-depth buildings from the modeled facades and generate the playable 5.3 km City Run. Reason: district catalogs need a real race-world implementation rather than another isolated preview; originating part: 04; dependencies: DISTRICT-01, geometric facade assets, shared physics/recovery; acceptance: complete facade/roof/rear/end assemblies, coherent districts, exact 5,300 m open race, solid support/buildings, all 15 grid starts, working recovery/restart/checkpoints/finish, preserved Valley fallback; verification: [deterministic bake, per-building Rapier rays, foot routes, all gates, native city and Valley smoke, nine actual-game survey views](testing/city-5300/README.md). Production fidelity, fine collision/art review and broad performance/platform acceptance remain V2-04 gates.

- [x] **RFU-AI-03** Target the closest rider ahead, pursue beyond10m and seek moving side-by-side combat inside10m. Reason: issue26 follow-up reports stopped racer/killer queues; part: 01 (NPC/combat06/07 integration); dependencies: AI-10, AI-QUEUE-02, shared vehicle controls and performance baseline; acceptance: eligible targets, stable side and bounded speed matching; verification: [rules, physical regressions and native decision receipts](testing/issue26-racer-ai/REPORT.md). Verified: deterministic targeting/hysteresis, equal-station ordering, moving-speed intent and physical pair/chain regressions; 647 integrated workspace tests and strict code/build/audit gates pass. Broader gameplay balance remains RFU-AI-01/02/06 and AI-11.

- [x] **RFU-AI-04** Race cleanly while leading until another rider approaches within10m. Reason: issue26 follow-up reports stopped racer/killer queues; part: 01 (NPC/combat06/07 integration); dependencies: AI-10, AI-QUEUE-02, shared vehicle controls and performance baseline; acceptance: rear engagement never creates braking feedback; verification: [rules, physical regressions and native decision receipts](testing/issue26-racer-ai/REPORT.md). Verified: deterministic targeting/hysteresis, equal-station ordering, moving-speed intent and physical pair/chain regressions; 647 integrated workspace tests and strict code/build/audit gates pass. Broader gameplay balance remains RFU-AI-01/02/06 and AI-11.

- [x] **RFU-AI-05** Trace leading-killer/follower braking through target, pace and avoidance. Reason: issue26 follow-up reports stopped racer/killer queues; part: 01 (NPC/combat06/07 integration); dependencies: AI-10, AI-QUEUE-02, shared vehicle controls and performance baseline; acceptance: reproducible diagnostics distinguish combat matching from obstacle stops; verification: [rules, physical regressions and native decision receipts](testing/issue26-racer-ai/REPORT.md). Verified: deterministic targeting/hysteresis, equal-station ordering, moving-speed intent and physical pair/chain regressions; 647 integrated workspace tests and strict code/build/audit gates pass. Broader gameplay balance remains RFU-AI-01/02/06 and AI-11.

- [x] **RFU-AI-07** Define combat, passing, avoidance and race-progress priorities. Reason: issue26 follow-up reports stopped racer/killer queues; part: 01 (NPC/combat06/07 integration); dependencies: AI-10, AI-QUEUE-02, shared vehicle controls and performance baseline; acceptance: explicit radius, eligibility, target/side hysteresis and forward preference; verification: [rules, physical regressions and native decision receipts](testing/issue26-racer-ai/REPORT.md). Verified: deterministic targeting/hysteresis, equal-station ordering, moving-speed intent and physical pair/chain regressions; 647 integrated workspace tests and strict code/build/audit gates pass. Broader gameplay balance remains RFU-AI-01/02/06 and AI-11.

### Part 01 — Remaining handling, crash response and runtime corrections

- [ ] **RFU-SURF-01** Give sand the strongest driveable-surface speed penalty. Reason: issue29 reports insufficient sand slowdown; part: 01; dependencies: shared handling/Surface contract; acceptance: lower matched-grade acceleration and sustained speed than asphalt, dirt, gravel and grass, with stronger coasting slowdown; verification: both physical adapters, same bike/input/grade, native authored terrain and player-feel review. [Implementation/evidence](testing/sand-speed/REPORT.md). Integrated flat-grade adapter and exact-binary native functional checks pass; human feel and broader hardware/performance acceptance remain open.

- [ ] **RFU-SURF-02** Align sand classification with authored ground on Hills and Metropolis. Reason: rendered sand was Dirt and lane coordinates can differ from pavement; part: 01; dependencies: RFU-SURF-01, terrain authoring; acceptance: sand only where authored, asphalt precedence at blended edges and both branches, no texture-pixel inference; verification: surface/schema tests and native surface/road support captures. [Evidence](testing/sand-speed/REPORT.md). Integrated schema, branch-road and both-map native classification checks pass; human terrain-edge review remains open.

- [ ] **RFU-SURF-03** Preserve predictable sand entry, exit and reverse. Reason: a hard speed clamp or excessive resistance can trap riders; part: 01; dependencies: RFU-SURF-01/02; acceptance: gradual energy loss, recoverable road pace, forward/reverse and AI traversal with accepted rigs unchanged; verification: adapter entry/exit/reverse regressions, native both-rig recovery and player-feel review. [Evidence](testing/sand-speed/REPORT.md). Integrated adapter reverse/entry/exit/AI and both-rig native recovery checks pass; steep-uphill parking reverse and human feel remain open.

- [ ] **RFU-RAIL-01** Separate guardrail road-relative height from post grounding. Reason: user reports low rails and floating feet; dependencies:HILLS-01 and resident shoulder/terrain collision; acceptance: raised readable rails and every support reaches actual ground across crests, dips and cross-slopes; verification: all-post ground rays, geometry measurements and native close views. Part:01. Physical, integrated and seven native functional checks pass; human full-course and broad hardware/performance acceptance remain open: [guardrail work](testing/guardrails/REPORT.md).

- [ ] **RFU-RAIL-02** Provide deliberate practical re-entry openings in long guardrail runs. Reason: riders trapped outside barriers; dependencies:RFU-RAIL-01 and existing recovery; acceptance: full-width openings traversable on foot and by intended bike footprint without exceptional rescue; verification: physical controller return, bidirectional shape sweeps and native traversal. Part:01. Physical, integrated and seven native functional checks pass; human full-course and broad hardware/performance acceptance remain open: [guardrail work](testing/guardrails/REPORT.md).

- [ ] **RFU-PERF-01** Measure and reduce Hills and Metropolis loading. Reason: issue25 reports excessive startup; part:01; dependencies: existing course construction/readiness; acceptance: repeated cold/warm samples separating generation, assets, collision, construction and playable readiness, fewer measured seconds with unchanged content/collision; verification: stage profiles, matched native runs and geometry/recovery regressions. Controlled cold-asset/warm repeats and both-rig/map recovery pass; see docs/testing/issue25-performance/REPORT.md. Partial: [Hills projection index](testing/issue25-projection/REPORT.md) reduces matched warm readiness 25.86→8.25s with identical structural survey fields; final combined native surveys pass. Full cold-boot/presentation timing, combined-source recovery and broader hardware remain open.

- [ ] **RFU-PERF-04** Evaluate offline baking/caching, streaming, instancing, culling and bounded simulation against measured bottlenecks. Reason: issue25 requests substantial performance improvements; part:01; dependencies: RFU-PERF-01/02 and preserved world/physics contracts; acceptance: measured gains without removing resident hazards, source identities or required content; verification: matched profiles, deterministic geometry/collision and long-route recovery/streaming checks. Partial: [bounded Hills route index](testing/issue25-projection/REPORT.md) is a measured spatial-pruning step; broader offline-bake/streaming/instancing and long-route acceptance remain open.

- [ ] **RFU-AI-01** Seek overtaking opportunities and race progress around slow/stopped riders. Reason: issue26 follow-up reports stopped racer/killer queues; part: 01 (NPC/combat06/07 integration); dependencies: AI-10, AI-QUEUE-02, shared vehicle controls and performance baseline; acceptance: stopped-player followers must pass when a reachable route exists; verification: [rules, physical regressions and native decision receipts](testing/issue26-racer-ai/REPORT.md). The city/guardrail-integrated branch passes 647 tests, strict checks, locked builds and a release Metropolis race with both recovery rigs. Avoidance-driven stops up to 33 sampled seconds, all-map lane behavior, quiet cost and human balance remain open. A bounded station-3040 follow-up traced a real oncoming car and an occupied rear merge; an outbound-refuge candidate reduced sampled Riding zero-plan stops 20→14 over matched 7,500-frame native windows, while rider 1/11 queues still occurred. Keep this requirement open; [paired receipts](testing/issue26-racer-ai/native-queue-3040-after/README.md) remain partial.

- [ ] **RFU-AI-02** Let racers and killers choose any physically drivable lane, including a clear opposite lane for passing, then merge back after a committed pass. Reason: issue26 follow-up reports stopped racer/killer queues; part: 01 (NPC/combat06/07 integration); dependencies: AI-10, AI-QUEUE-02, shared vehicle controls and performance baseline; acceptance: predict outbound/return clearance, yield to oncoming/rear obstacles and preserve collision; verification: [rules, physical regressions and native decision receipts](testing/issue26-racer-ai/REPORT.md). The city/guardrail-integrated branch passes 647 tests, strict checks, locked builds and a release Metropolis race with both recovery rigs. Avoidance-driven stops up to 33 sampled seconds, all-map lane behavior, quiet cost and human balance remain open. A bounded station-3040 follow-up traced a real oncoming car and an occupied rear merge; an outbound-refuge candidate reduced sampled Riding zero-plan stops 20→14 over matched 7,500-frame native windows, while rider 1/11 queues still occurred. Keep this requirement open; [paired receipts](testing/issue26-racer-ai/native-queue-3040-after/README.md) remain partial.

- [ ] **RFU-AI-06** Remove mutual-killer braking and multi-rider combat queues. Reason: issue26 follow-up reports stopped racer/killer queues; part: 01 (NPC/combat06/07 integration); dependencies: AI-10, AI-QUEUE-02, shared vehicle controls and performance baseline; acceptance: bounded progress in passable scenarios and legitimate yielding in sealed corridors; verification: [rules, physical regressions and native decision receipts](testing/issue26-racer-ai/REPORT.md). The city/guardrail-integrated branch passes 647 tests, strict checks, locked builds and a release Metropolis race with both recovery rigs. Avoidance-driven stops up to 33 sampled seconds, all-map lane behavior, quiet cost and human balance remain open. A bounded station-3040 follow-up traced a real oncoming car and an occupied rear merge; an outbound-refuge candidate reduced sampled Riding zero-plan stops 20→14 over matched 7,500-frame native windows, while rider 1/11 queues still occurred. Keep this requirement open; [paired receipts](testing/issue26-racer-ai/native-queue-3040-after/README.md) remain partial.

- [ ] **RFU-AI-08** Add reproducible scenario and decision receipts. Reason: issue26 follow-up reports stopped racer/killer queues; part: 01 (NPC/combat06/07 integration); dependencies: AI-10, AI-QUEUE-02, shared vehicle controls and performance baseline; acceptance: headless rules/physical cases plus native full-race and measured cost evidence; verification: [rules, physical regressions and native decision receipts](testing/issue26-racer-ai/REPORT.md). The city/guardrail-integrated branch passes 647 tests, strict checks, locked builds and a release Metropolis race with both recovery rigs. Avoidance-driven stops up to 33 sampled seconds, all-map lane behavior, quiet cost and human balance remain open. A bounded station-3040 follow-up traced a real oncoming car and an occupied rear merge; an outbound-refuge candidate reduced sampled Riding zero-plan stops 20→14 over matched 7,500-frame native windows, while rider 1/11 queues still occurred. Keep this requirement open; [paired receipts](testing/issue26-racer-ai/native-queue-3040-after/README.md) remain partial.

- [ ] **AI-QUEUE-02** Resolve multi-rider slow queues with safe committed overtakes and escape from blocked approaches. Reason: user observes racers piling up despite the earlier single-queue fix; part: 01; dependencies: traffic avoidance and shared vehicle controls; acceptance: multiple rivals make progress around slower traffic when a reachable gap exists, without entering sidewalks or bypassing collision; verification: deterministic multi-rider physical regression and native queue observation. Implemented; four-rider physical regressions pass, dedicated rendered queue/human balance review remains open. Civilian circular-wait correction and verification are tracked in [traffic stall follow-up](testing/traffic-stall/README.md); legitimate obstacle yielding is retained.

- [ ] **CAST-AI-01** Use the approved portrait cast for race names, description-based equipment and distinct driving/combat traits. Reason: dossiers and race opponents were disconnected; part: 01; dependencies: AI-12, WEAPON-MODELS-01; acceptance: stable save IDs, fourteen distinct active opponents with matching dossier names/loadouts, bounded lane decisions, safe reverse recovery and obstacle-aware mounted/on-foot escapes; verification: cast/core tests, real dynamic reverse fixtures and native races; human balance remains open.

- [ ] **FOOT-SPAWN-01** Validate collision-clear get-up origins without moving legitimate rooftop/car-supported riders. Reason: production GettingUp uses a centre support ray without building-interior/capsule-volume validation; an invalid native setup demonstrated a fully enclosed start, but ordinary-crash incidence is unverified; part: 01; dependencies: shared recovery, world navigation and FOOT-004; acceptance: reproduce actual invalid production placement, retain safe original origins, bound any nearby stand-up search, preserve roof/car support and solid walls, and explicitly handle deep invalid states without silent wreck teleport or indefinite recovery; verification: real-world closed-envelope/near-wall/roof/car and crash/run/remount tests. Deferred inspection finding from [road/walkability evidence](testing/roads-walkability/README.md); no placement/rescue policy changed in this pass.

- [ ] **AI-11** Review full-race racer/killer balance with human playtests. Reason: deterministic roles and combat tests do not establish fun/fair aggression; part: 01; dependencies: AI-10; acceptance: completed races on both branches with visible overtaking racers, readable killer threats and practical escapes/recovery, no unavoidable spawn pileup, user-approved difficulty; verification: recorded human races and tuning feedback with crash/attack counts.

- [ ] **CRASHREF-004** Extend original-crash coverage and establish trajectory parity before changing the default. Reason: native 2.5D geometry, other callers and post-entry laws exceed the first 3D comparison; part: 01; dependencies: CRASHREF-001–003, retail trajectory fixtures and explicit gameplay review; acceptance: low/static object, combat, downed-body/traction/wall caller routing, original condition/regeneration tables, pair ordering/reciprocal traffic response, and justified native-vs-Rapier bounce/drag trajectories documented and tested; verification: reproducible original-vs-modern traces, multi-rider contact/energy/recovery scenes and user comparison. Current comparison boundaries remain explicit in [research](research/ORIGINAL_CRASH_CALCULATIONS.md).

- [ ] **V2-01-005** Design and implement measured vehicle-impact and over-car crash responses. Partial: bounded tick/entity contact observations record both pre-solver world velocities, normal closing speed, rear/oncoming/side/grazing/stopped approach and solid collider height. Existing dissipative entry/CCD remain; measured over-car trajectories, bounded nose-down/vault response and user acceptance remain open.

- [ ] **V2-01-006** Add a combat knockoff loss-of-balance sequence. Partial: the default velocity-muscle path already performs mounted away-from-hit swerve/countersteer, three-hit release, seated-pose continuity and protective arms. Solid wall/vehicle contacts at ≥1m/s normal closing now interrupt either mounted phase without added launch or a new ordinary-riding threshold; both sides/phases and baseline are regression-tested. Full obstacle/terrain/multi-rider matrix, release velocity/pose trajectory measurements and recorded user gameplay approval remain open. [Audit and evidence](testing/part01-runtime/README.md).

- [ ] **V2-01-007** Compare the reference crash/control code and evaluate bounded physics changes.

- [ ] **V2-01-008** Exit gate: accept corrections before authoring motion.

- [ ] **ARCH-001** Isolate `client`, pure `sim`, `physics`, authoritative `server`, typed `content`, and developer `tools` responsibilities; clients never adjudicate hits/results from reported transforms.

- [ ] **ARCH-002** Define stable bike/rider/weapon/track IDs, cosmetic-versus-physics IDs, schema versions, validators, and migration-safe saves/protocols.

- [ ] **ARCH-003** Define `AttackIntent`, `AttackResolved`, `CrashStarted`, `RecoveryCompleted`, racer-level `RaceFinished`, and `PoliceBusted` contracts with IDs/ticks and single-result semantics.

- [ ] **ARCH-007** Tick/control simulation at 60 Hz, AI perception at 5-15 Hz, interpolate independently, and use explicit adapter-observed components/events rather than renderer or animation authority.

- [ ] **RIGHTS-006** Sessions 01-02: workspace, CI, debug metrics, clean Fedora scene.

- [ ] **RIGHTS-008** Sessions 05-08: acceleration/brakes, two wheel rays, stable camera/turns.

- [ ] **RIGHTS-009** Sessions 09-10: lean, surface grip, three distinct presets.

- [ ] **P01-003** Split the single-crate prototype into client/server/core/vehicle/combat/race/AI/content/network/tools responsibilities, with a plugin per subsystem and conventional commit rules.

- [ ] **P01-004** Keep pure simulation independent of UI/rendering; isolate the physics adapter, presentation, typed content, dev tools, and later server authority.

- [ ] **P01-006** Add action mapping/input contexts, stable content IDs, schema versions, and the intent -> simulation -> authoritative event -> presentation contract.

- [ ] **P01-007** Add `devtools`, `debug_physics`, `offline`, and `netplay` feature boundaries and reproducible quality modes.

- [ ] **P01-009** Extend CI to a validated Windows build plus content/license audits and workspace-wide checks; verify the dependency migration separately.

- [ ] **P01-010** Validate a native Fedora/NVIDIA launch, Windows launch, and 100 pause/reset cycles without state corruption.

- [ ] **P01-011** **Exit gate:** independent 60 Hz ticks are measured, both platform launches are verified, and the required CI matrix passes.

- [ ] **P01-018** Obtain actual Windows and Fedora/NVIDIA native launch/input/audio receipts in addition to the CI build matrix; record the measured adapter and keep unmet hardware gates open.

- [ ] **P03-010** Run a ten-minute aggressive-turning soak without NaNs/tunneling/unexplained flips; record at least ten test rides and controller feedback.

- [ ] **P03-011** **Exit gate:** three presets feel distinct, a beginner can finish, and handling is fun and stable with placeholder art.

- [ ] **P03-020** Reproduce and numerically verify the entire reference handling, including original bike specs, drive/brake ramps, gear thresholds, nitro charges, surface tables and fixed-point effects. Verified local-disc calibration tables are available; complete numerical calibration and source reuse permission remain unresolved. See [precise remaining work](research/ROAD_RASH_HANDLING.md).

- [ ] **P04-004** Drive camera motion from interpolated fixed-step poses; verify no jitter at 120-144 Hz and readability through tight turns, impacts, and jumps.

- [ ] **P04-008** Validate camera clipping, both input devices, comfort-setting persistence, and responsive handling with an external tester.

- [ ] **P04-009** **Exit gate:** riding is fast, comfortable, responsive, and finishable with keyboard and gamepad without unintended auto steering.

- [ ] **P04-014** Verify the default mapping for the OS-discovered controller that Gilrs reports as unmapped; exercise real buttons/axes and add a licensed mapping only if needed. Evidence: [native receipt](testing/PART_04.md); physical mapping acceptance remains open.

- [ ] **P06-006** Add independent stamina, bike durability, balance/wobble, police-hit and bad-landing crash thresholds, and one authoritative `CrashStarted` event.

### Required animation coverage

### Research, labels and deliverables

### Part 02 — Animation A — reference study, labeled keyframes and pose approval

- [ ] **V2-02-001** Build a complete animation/use-case catalog. ANIM-14 resumes source-to-runtime coverage review; unresolved caller/timing and full pose approval remain open. See [next review](animation/NEXT_REVIEW.md).

- [ ] **V2-02-002** Inspect all original rider animation categories.

- [ ] **V2-02-003** Research Synty/POLYGON-style games and authentic locomotion.

- [ ] **V2-02-005** Author and label the full original 3D keyframe set.

- [ ] **V2-02-006** Cover running, on-foot recovery and non-rider characters.

- [ ] **V2-02-007** Publish a numbered pose gallery with anatomical review.

- [ ] **V2-02-008** Exit gate: keyframes and catalog approved.

- [ ] **ANIM-02** Refine fingers, grips, fists and weapon sockets. Runtime glove shapes and visual sockets are integrated under ANIM-15; shared authoring profiles and GLB reload are implemented in the [Part02 pass](testing/part02-authoring/README.md); final thumb/finger silhouettes, reviewed clearance and exact source/runtime normal/tessellation parity remain open. Reason: the current twenty-bone rig has rigid hands; part: 02; dependencies: rider mesh/rig and contact catalog; acceptance: believable closed fists, wrapped grips and compatible held-weapon points without changing passive physics; verification: front/rear/side hand close-ups and rig/contact validation.

- [ ] **ANIM-03** Resolve reference sequence linkage, timing and ambiguous labels. Reason: sprite slots include view/depth variants and do not establish timing; part: 02; dependencies: local reference metadata and CANS/RRAN/EXE research; acceptance: evidence-backed phase/side/weapon labels and timing or explicit unknowns for every reference family; verification: source-linked sequence audit, no guessed native durations.

- [ ] **ANIM-04** Complete and review recovery/support and specialist poses. Reason: get-up kneel/sit candidates are preliminary crouches and arrest/equip/drop coverage is missing; part: 02; dependencies: catalog and hand refinement; acceptance: supported get-ups/lifts/mounts, reviewed vault/knockoff proposals, weapon equip/drop, police restraint and refined quadruped anatomy with clothing/body/bike clearance; verification: multi-angle art review, endpoint/chain checks and user pose approval before Part 03.

- [ ] **ANIM-06** Review context-first pose fidelity and finish unresolved action bindings. Reason: correct sequence metadata alone does not prove faithful 3D silhouettes; part: 02; dependencies: ANIM-05 and new source-linked draft poses; acceptance: review every action/phase, source mirroring, fixed grip/fist/weapon placement, grounded get-up/mount support, crash/bike silhouettes, additional camera views and unknown caller aliases; verification: labeled source/recreation strips, joint/contact tests, multi-angle art review and explicit approval before interpolation.

- [ ] **ANIM-16** Round-trip the glove profiles and sockets through the static authoring library. Shared profile/assembly/socket authoring and actual Blender export/reload are implemented; exact source/runtime silhouettes/normals and user approval remain open (ADR080). Reason: existing Blender Context_Fist drafts use a different compression recipe from the new runtime hinge shapes; part: 02; dependencies: ANIM-15, original source rig and context poses; acceptance: one profile source for open/grip/fist and sockets, preserved UVs/normals/contacts, matching source/runtime silhouettes and user review; verification: Blender export/reload, source/hash audits, per-hand multi-angle comparisons and unchanged physical trajectories.

- [ ] **P11-003** Establish tested Blender -> glTF/GLB scale, axes, rig/bone names, PBR materials, collision naming, LODs, and source/export folder separation.

- [ ] **P11-006** Introduce asset validation for triangles, skeleton paths, textures/material references, ownership metadata, release clearance, and content hashes.

- [ ] **ARCH-006** Define evidence/classic-reference/new-design/rights/QA catalog fields; keep original-source observations distinct from new stats/unlocks/visuals.

### Part 03 — Animation B — interpolation, terrain locomotion and active recovery

- [ ] **RFU-MOVE-05** Set the urgent run hand roll so both thumbs face the travel direction and palms do not stay flat toward the rear. Reason: user visual playtest of issue #28; part:03; dependencies: RFU-MOVE-01 and authoring bind; acceptance: anatomically readable bilateral swing/fist orientation at run speed while stopped pose remains neutral; verification: local hand-axis pose assertions and close native front/side/rear gait frames on flat and uneven ground. Visual pose and scripted captures pass; human approval remains open. [Evidence](testing/issue41-running-hands/REPORT.md), [issue](https://github.com/ImadMashhood/RoadDash/issues/41).

- [ ] **RFU-MOVE-06** Keep the running hand correction through terrain IK and recovery transitions. Reason: user reports the motion looks awkward on changed elevation; part:03; dependencies: RFU-MOVE-02/05; acceptance: no wrist pops, hand/body disconnection or physics/control change across run, stop, slope and remount; verification: pose continuity, differential rig/recovery tests and native gait review. Scripted uneven animation and paired native physical recovery pass; broad terrain and human review remain open. [Evidence](testing/issue41-running-hands/REPORT.md), [issue](https://github.com/ImadMashhood/RoadDash/issues/41).

- [ ] **RFU-MOVE-02** Align recovery feet/body over slopes, crests and uneven elevations. Reason: issue #28 terrain-contact defects; part: 03; dependencies: RFU-MOVE-01 and existing physical support locks; acceptance: bounded swing clearance, stable stance, continuous transitions and no physical rig changes; verification: translated up/down/cross-slope contact tests, native uneven-ground/manual/automatic/remount captures and user review. [Evidence](testing/issue-28-movement/README.md).

- [ ] **RFU-MOVE-01** Present urgent bike-return running. Reason: issue #28 reports a slow-looking recovery walk; part: 03; dependencies: V2-03-003/ADR130; acceptance: readable run at unchanged actual movement speed with user visual review; verification: distance-matched gait tests and native flat/slope sequences. Implementation pending acceptance; [evidence](testing/issue-28-movement/README.md).

- [ ] **COMBAT-FEEL-01** Synchronize strikes to visible contact keyframes and review impact feel/audio. Partial: fixed-clock windup/contact and interruption/pause/reset handling are integrated in the [Part03 pass](testing/part03-runtime/README.md). [Polish12](testing/polish-12/pass12/README.md) adds bounded confirmed-hit directional punch/kick recoil with held contacts and attack priority; subjective impact/audio, full geometry and user playtest approval remain open. Reason: faster playback alone does not align input-tick hit resolution; part: 03; dependencies: COMBAT-09 and fixed input buffering; acceptance: one strike at contact, interruption/pause safe, strong readable left/right impacts and approved sound/feel; verification: timed contact/miss/recovery tests, native recording and user playtest.

- [ ] **RAG-B-01** Implement and verify reciprocal angular-impulse muscle control on the existing constrained rig. Reason: user requested a second method for comparison; part: 03; dependencies: RAG-11 and shared crash lifecycle; acceptance: tensor reciprocal momentum, no authored linear/position correction, torque/rate/work caps, real support, finite attempts and baseline/sleep preservation; verification: focused/backend/full regression and native evidence in [Method B](testing/crash-comparison/method-b/README.md). Automated subset implemented; native/user gates remain open.

- [ ] **RAG-B-02** Verify complete Method B combat/crash/recovery integration. Reason: compare both methods using identical triggers; part: 03; dependencies: RAG-B-01 and shared A/B lifecycle; acceptance: real three-hit swerve/countersteer, continuous release, obstacle interruption, safe own-wreck grace, manual foot finish/remount/reset/pause, comparison saves disabled; verification: headless integration, native smoke and F8/F9/F10 review.

- [ ] **RAG-B-03** Choose the preferred crash behavior after A/B gameplay review. Reason: the user will select the winner; part: 03; dependencies: RAG-B-01/02 and Method A; acceptance: identical20/60/120/200km/h cases, documented keep/change decisions, approved feel and no critical regressions; verification: user playtest and recorded comparison. Velocity was selected by the user on 2026-11-07; prototype B acceptance and broad tuning gates remain open. The native comparison candidate is not integrated. User selection is now recorded: velocity on 2026-11-07 (RAG-12); remaining integration/tuning acceptance depends on RAG-B-01/02, not another selection question.

- [ ] **V2-03-001** Implement a data-driven animation state/blend graph. Initial riding/attack/recovery graph verified under ANIM-08; fixed contact scheduling now shares the authored clip clock and validates explicit contact keys (ADR081); full weapon/police/locomotion coverage and approval remain open.

- [ ] **V2-03-002** Interpolate approved keyframes and combat/recovery transitions.

- [ ] **V2-03-003** Make walking/running match velocity and terrain. Partial: distance-driven stride, velocity blend, support-ray sole alignment, finite swing release, fresh stance capture and physics-local moving-support locks are integrated; pelvis/stair/turn-in-place and measured terrain slip acceptance remain open.

- [ ] **V2-03-004** Keep hands and feet on bike contacts when required. Initial four-contact render solver passes fixed-length/endpoint tests; full runtime hand/foot deformation and clearance approval remain open.

- [ ] **V2-03-005** Prototype bounded active ragdoll and player attempts to recover. Partial: supported player input raises only existing capped joint motors to 0.12 activation; neutral restores the passive crash curve. Controlled work/energy, conscious/unconscious policy and user feel approval remain open. RAG-11 proposes true ground-normal support checks, posture-aware directional bracing, bounded attempts to roll supine, physical back sliding, timeout/yield under speed/spin/obstruction and work-budget integration with the existing energy guard. No such changes are implemented; retain passive fallback and verify high-speed/airborne/wall contacts, failed attempts, momentum, joint integrity and recovery.

- [ ] **V2-03-006** Blend physical landings into terrain-aware get-up and remount. Partial: capture the settled physical pose and blend into get-up, followed by terrain-aware feet and existing mirrored remount. Confined clearance, recovery direction selection and full terrain approval remain open.

- [ ] **V2-03-007** Add motion/audio polish without changing gameplay outcomes. Partial: footsteps follow measured travel instead of elapsed recovery time. Contact-event timing, breathing/look and broader motion/audio polish remain open.

- [ ] **ANIM-09** Finish native full-body animation acceptance. Reason: authoring renders and numeric constraints do not certify every runtime deformation/transition; part: 03; dependencies: ANIM-08, ANIM-02/04/06 and terrain locomotion; acceptance: native multi-angle normal/tuck-to-all-attacks, mirrored pickup and physical-pose get-up review, fingers/feet and cloth/bike clearance, Windows hardware and user approval; verification: labeled native video matrix, contact/deformation checks and regression parity.

- [ ] **RAG-05** Approve native biomechanics silhouettes and finish distal skin articulation. Reason: pilot distal skin articulation and impact peaks require human review; prop NPC skins still use eleven joints; part: 03; dependencies: RAG-02/03, ART-04 and ANIM-02/09; acceptance: independently visible wrists/feet/spine, reviewed source-pose release and multi-angle drop/car/terrain/knockoff clearance, acceptable distal impact velocities and remaining multi-angle art approval; the user approved default integration on2026-11-06; verification: native videos, deformation/contact metrics and explicit user review. Partial: pilot wrists/feet/spine now use sixteen independent visual joints. Prop pedestrian skins, finger detail, all impact-camera views and final art approval remain open.

- [ ] **RAG-06** Profile and approve worst-case race ragdoll performance. Reason: additional bodies/solver work cost more than the baseline and instrumented headless timing is not race FPS; part: 03; dependencies: RAG-04 and representative traffic/environment load; acceptance: compare frame/physics p95 and allocations across15 racers plus bounded NPC crashes, sleeping/wakeup, Windows hardware and quality presets without regressions; verification: repeatable native profiling on target hardware and user acceptance, recording remaining production performance acceptance separately from the2026-11-06 default integration approval.

- [ ] **V2-03-008** Exit gate: motion is responsive, coherent and physically readable.

- [ ] **P11-008** Animate riding, fighting, crash, walking, and remount through a production rig; add lightweight animal animation/AI and safe cliff recovery.

- [ ] **FOOT-003** Crash with neutral input -> seek bike around a known obstacle; verify safe proximity/facing/low-speed remount.

- [ ] **FOOT-008** Run off a cliff or become trapped between a fence and car -> recover without an invisible-wall/terrain softlock.

- [ ] **RIGHTS-012** Sessions 16-18: manual foot movement, optional assist, foot finish, remount, no softlocks.

- [ ] **ART-02** Acquire a licensed pilot rider and integrate one shared skinned avatar across riding, fighting, walking and ragdoll. Original authored pilot integration is tracked under ART-03; complete production deformation/animation/LOD acceptance remains open. Reason: researched assets are not yet working models; part: character art/animation; dependencies: ART-01, model selection/acquisition and exact archive license; acceptance: biker outfit/color zones, coherent rig and recovery, unchanged physics/controls, 15-racer performance/LOD acceptance; verification: deformation/animation/ragdoll tests, native visual checks and profiling.

- [ ] **ART-04** Polish the visual retarget and authored animation transitions without changing simulation. Partial ADR119: [helmeted matte clothing](testing/stylized-humans-grass/README.md) preserves authored surfaces, skin influences and physical rigs; user chose helmets, while final appearance remains for review. Partial: [polish11 ankle pivot correction](testing/polish-12/pass11/README.md) aligns boot articulation with the authored chain and preserves source geometry/weights, neutral wrists and both physical rigs; full deformation, civilian seams, motion/art and hardware acceptance remain open. Reason: authoring rig has 20 bones and the current fingers/pose drivers are prototypes; part: 11 art/animation; dependencies: ART-03 and static pose library; acceptance: clean grip/fist/deformation and contact-aware transitions at gameplay speeds; verification: pose review, native clips and repeated physics parity tests.

- [ ] **P05-006** Implement idle/lean/punch/kick/hit/stagger animation states, input buffering/priorities, anti-spam, and gameplay-owned timing. Partial: buffered intent and one fixed contact per attack share the visible clip clock; polish12 adds replaceable confirmed-hit presentation pulses with attack/knockoff priority and no input lock. Complete priority/state coverage and user acceptance remain open.

- [ ] **P06-008** Separate OnFootAutoSeekBike and OnFootManual states; support FinishedOnFoot and BustedOrRecover outcomes. Auto/manual/FinishedOnFoot and the Part08 busted/no-payout/retry outcome are implemented; explicit production outcome/state separation and broader manual/human acceptance remain open.

- [ ] **P06-009** Make remount safe: proximity, facing/interaction option, low bike speed, animation/collision checks, and no unwanted snap while deliberately walking away. Partial: proximity, facing, low-speed and explicit interaction gates are tested; final animation/obstruction clearance acceptance remains open.

- [ ] **P06-010** Tune control handoff toward 1-3 s after a crash; add optional delayed stuck recovery with penalties and no automatic finish-line teleport.

- [ ] **P06-012** Run 100 high-speed crashes without frozen physics/terrain clipping; audit trapping around fences, buildings, curbs, cliffs, and traffic, and tune repeated-hit protection/time loss.

- [ ] **P06-013** **Exit gate:** knockoff, manual foot movement, optional assist, valid foot finish, and safe remount all work smoothly.

### Part 04 — World A — facade study, original textures and procedural buildings

- [ ] **ELEVATION-01** Make Metropolis elevation moderately more pronounced. Reason: user requests stronger terrain relief; part: 04; dependencies: METRO-01, LAYOUT-03 and shared elevation/support; acceptance: 20–25% greater longitudinal relief, smooth grades below4%, exact planar/identity/lane preservation, support and recovery on both branches; verification: [authoring/preservation/support receipts](testing/next-elevation/README.md), integrated native survey/recovery and user feel review.

- [ ] **CITY-WIDTH-01** Preview a narrower Metropolis retaining all race-road frontage and removing about half of non-frontage blocks. Reason: the city is overbuilt laterally; part: 04; dependencies: METRO-01, CITY-CIVIC-01; acceptance: actual-source before/after map, explicit whole-block IDs/counts, both race branches and civic sites retained; runtime deletion is pending map selection; verification: read-only geometry/count audit, then source bake/navigation/recovery and measured native performance if implemented. [Proposal](testing/civic-greenery/README.md).

- [ ] **CITY-CIVIC-01** Build original City Hall, fountain park, covered cafe frontages and a gradual farm-to-city approach. Reason: reserved city sites and rural entry feel empty; part: 04; dependencies: METRO-01, WORLDART-01; acceptance: grand City Hall in park3 near the driving street with arches facing it and surrounding buildings, dense trees and grass, detailed full-block fountain park in park5 with sidewalk-connected paths, increasing standalone frontage density, safe access and persistent collision; verification: authoring geometry audits, native civic/route captures and recovery.

- [ ] **CITY-WALK-02** Expand urban sidewalks by approximately20% while preserving declared lanes and car/pavement fit. Reason: narrow walks and lane/traffic mismatch; part: 04; dependencies: ROAD-WALK-01, METRO-01; acceptance: measured expansion, retained building IDs/closed envelopes and carriageway, legible lane paint and pedestrian crosswalks; verification: baked geometry/whole-car audits and native street views.

- [ ] **PALETTE-01** Extend Metropolis beyond its twelve source-mapped facade families with curated city-district pools. Reason: reusing the currently mapped3D assets does not integrate every eligible earlier facade draft; part: 04; dependencies: AUTHORED-01, MESH-01 and MAT-03; acceptance: type-appropriate expanded catalog coverage, deterministic district/role assignment, matching full/coarse source identity, unique signage rules and retained ready/collision/performance budgets; verification: catalog/role/atlas hashes and coverage tests, close/far native district views and measured budgets. Partial: [Part04 district pass](testing/part04-world/README.md) expands12→24 source families with deterministic district/tower pools and matching three-band coarse/detail fitting. [Polish08](testing/polish-12/pass08/README.md) adds a height-aware roadside/workshop/neighborhood blend within existing approach envelopes, with unchanged taller/distant assignments and access geometry. Final palette/signage/art, extended streaming and performance acceptance remain open; no claim that all128 designs are instantiated in Metropolis.

- [ ] **LAYOUT-05** Review the race-centered city and primary-avenue intersection plan. Reason: user reference requires the whole city to be built around the race line, with intersections track first; part: 04; dependencies: LAYOUT-03 street-first parcels and LAYOUT-04 height/detail/access rules; acceptance: balanced whole-block development around both legal race alternatives, unchanged race/farm alignment, primary-avenue junction priority and protected fork throats, no competing secondary runs within110m for48m at under40degrees, connected secondary grid, fitted straight-edged buildings and retained height/detail hierarchy; verification: [centered whole map, junction close-up, centering/connectivity/junction/geometry/negative-case and deterministic checks](planning/farms-long-city/RACE_FIRST.md), user review and later construction/traffic/native verification. User approved construction; runtime implementation and current limitations: [Metropolis evidence](testing/metropolis/README.md). Full construction/traffic/performance acceptance remains open.

- [ ] **LAYOUT-04** Finalize height zoning, scattered taller buildings and route-distance detail before 3D rendering. Reason: the street-first plan lacked a skyline/detail hierarchy and interior buildings; part: 04; dependencies: LAYOUT-03 street/property plan, access reservations and existing facade identity rules; acceptance: taller central Market Quarter tapering to outskirts, seed-stable frontage accents/interior towers, clear six-metre access with affected parcels recorded, per-building heights and detail tiers measured from both race alternatives, complete distant silhouettes and user-reviewed maps before 3D; verification: [height/detail/downtown plans, overlap/access/zone/tier/negative-case and deterministic checks](planning/farms-long-city/MASSING.md), user review and later native LOD/construction validation. Approved heights/detail tiers are baked into Metropolis; wider native construction/art acceptance remains open.

- [ ] **LAYOUT-03** Review the entire farm-to-extended-city route with one split, elevation and street-derived polygon blocks. Reason: the user replaces the hills ending/two-fork concept and clarifies the NYC/Flatiron principle: roads define land, and straight-edged buildings fit its shape; part: 04; dependencies: current farm trace, density/city-center goals and layout-only scope; acceptance: recognizable farm approach, longer city through urban finish, one early split rejoining near the first city quarter, 180 × 360 m regular grid, curved/diagonal streets cutting triangular/trapezoidal blocks, fitted parcels/corner buildings, detailed property/landmark plan and elevation study; verification: [whole map, corner/property close-up, profile and geometry/graph/grade/determinism checks](planning/farms-long-city/README.md), user review and later construction validation. Approved ~12 km spine / 8.85 km city is now integrated as Metropolis; final construction acceptance remains open. The earlier all-rectangles rule and forced right-angle junction approach are superseded.

- [ ] **WORLD-EXTEND-01** Bake an accepted extended farm-to-city map in a later implementation step. Reason: the current game still uses the earlier hills-ended world; part: 04, deferred beyond layout; dependencies: LAYOUT-03, LAYOUT-04, LAYOUT-05, explicit validated ID/quota migration, terrain/rights-of-way and rendering budgets; acceptance: farm start, continuous enlarged city to finish, supported graded roads/sidewalks/lots, straight undeformed building geometry and preserved gameplay/physics/saves; verification: content/geometry/access tests, inspect-world, world survey, crash/recovery/finish native checks and performance review. Metropolis runtime now implements the accepted source without migrating legacy quotas; performance/support review remains open.

- [ ] **LAYOUT-02** Review the city-center footprint and connected neighborhoods. Reason: user rejected a corridor-shaped city; part: 04; dependencies: LAYOUT-01 density goals; acceptance: accepted downtown/civic places, connected districts, close rigid street fronts and a validated city/route/access plan; verification: [earlier city-center study](planning/city-center/README.md) and latest LAYOUT-03 review. Historical two-split topology is superseded by the user’s one-split, farm-to-long-city brief; city-center goals carry forward. No runtime or 3D changes.

- [ ] **ROUTE-SPLIT-01** Integrate the accepted single split/rejoin into the extended city in a later implementation step. Reason: the current single-centerline game cannot load the proposal; part: 04, deferred beyond layout; dependencies: LAYOUT-03, WORLD-EXTEND-01, P02-004 and validated junction/terrain authoring; acceptance: both legal alternatives, ordered branch gates/shared progress, AI choices, branch-aware recovery/camera/HUD, safe merges and no shortcut finish credit while preserving controls/physics/saves; verification: two deterministic race/foot/recovery scenarios, schema/transition/traffic tests and native review. Latest one-split brief explicitly supersedes the earlier two-split/four-combination target. Implemented in Metropolis; full production merge/traffic/playtest acceptance remains open.

- [ ] **LAYOUT-01** Review the dense road-hugging city layout before any 3D changes. Reason: user rejected sparse/dispersed placement and explicitly restricted this step to layout; part: 04; dependencies: measured race centerline and WORLDPLAN-01; acceptance: touching primary frontages, narrow sidewalks/minimal setbacks, rear courts/service access, dense secondary blocks and accepted street/property plan; verification: [2D plans, dimensions, parcel continuity checks and limits](planning/dense-city-layout/README.md), user layout review and later corner/overlap/access validation. Earlier draft: 752 concept parcels and close frontage; user rejected the corridor-only approach. Density goals carry forward into LAYOUT-02. No runtime geometry or asset changes.

- [ ] **LOD-03** Match distant facade identity and extend direct fades to optional City Run. Reason: initial generic-to-authored geometry still differs and City Run uses a separate scene-swap renderer; part: 04; dependencies: LOD-02 and approved facade families; acceptance: matching window/trim/material layout across distance levels, ready complementary transitions on both maps, prolonged high-speed and reverse-camera review without visible facade identity changes; verification: repeated native moving-camera captures, stream/cache census and user visual review. Technical implementation is now integrated under LOD-04, including matching far proxies and optional City Run. This checkbox remains open for user visual acceptance and extended streaming review.

- [ ] **V2-04-001** Catalog every reference building/facade family locally.

- [ ] **V2-04-002** Create original facade textures in the RoadDash style. A first realistic AI reference batch covers 25 sheets / 146 source IDs; [manifest](../art/buildings/realistic_sheets/MANIFEST.json) and [prompts](../art/buildings/realistic_sheets/PROMPTS.json). Production textures, exact layout/text cleanup and art/rights acceptance remain open.

- [ ] **V2-04-003** Plan a data-driven city/world construction grammar. Valley now maps all 657 unchanged lots through 19 role pools and actual ground/repeated-upper/crown geometry; [grammar and limits](art/BUILDING_GRAMMAR.md). Broader acceptance stays open. Earlier partial: the [playable 5.3 km City Run](testing/city-5300/README.md) now builds 977 complete exterior lots across 53 blocks and eight district zones using 85 City-eligible geometric facade designs, shared closed roofs, recessed rear openings and cross-street facade returns. Canonical Valley quotas/IDs remain unchanged. Broader terrain grammar, final transitions/site variety and full world acceptance remain open.

- [ ] **V2-04-004** Generate detailed building archetypes and whole blocks. The default Valley now uses the derived storey kit with bounded streaming (KIT-01/02); final infill/role fidelity and material/user approval remain open. [Complete draft model catalog](../art/buildings/mesh_assets/README.md) now covers 100 facades, 28 structures and 18 surface modules with editable Blender sources and 438 embedded GLB LOD files, real apertures/storefronts/canopies/bays/roof geometry, independent binary validation and native placement in the 5.3 km playable city. Earlier palette/flat-facade previews were rejected and do not satisfy this task. Draft mesh coverage and city runtime assembly are implemented; full material/art fidelity, user review, optimization and production acceptance remain open.

- [ ] **V2-04-006** Validate city life, visual density and performance. The latest Valley native survey meets its local 16.7 ms wall-frame p95 target at15.83 ms after ornament-shadow trimming; [measurement and limits](testing/world-polish/README.md). Broader racing/memory/platform acceptance remains open. [Native playable-city survey and crash/recovery/retry evidence](testing/city-5300/README.md) available; production art/LOD, encounter tuning and broad racing performance acceptance remain open.

- [ ] **V2-04-007** Exit gate: coherent procedural buildings are accepted.

- [ ] **KIT-03** Finish Valley infill skins and bespoke role geometry. Reason: native views expose plain party-wall infill and parking/service roles using generic industrial cladding; part: 04; dependencies: KIT-02, immutable lot/rights-of-way constraints and material/art review; acceptance: infill joins visually match neighbors without street intrusion, and parking/service/farm roles have appropriate full exterior geometry rather than misleading reused fronts; verification: join-envelope tests, per-role coverage and native multi-angle riding/on-foot review. Performance remains V2-04-006.

- [ ] **MESH-01** Model the remaining 94 facade designs from the approved three-quarter renders. Reason: the first sheet supplies six genuine meshes, not the full catalog; part: 04; dependencies: six-model kit, per-design reference review and material authoring; acceptance: every remaining facade ID maps to a distinct editable 3D model with actual openings/details/roof/sides and verified GLB LODs; verification: catalog coverage, mesh/material/bounds validation, native multi-angle renders and user art review. Partial: all 94 additional facade drafts exported with editable sources and three LODs; artist/user fidelity approval remains open.

- [ ] **MESH-02** Author the 46 structure/surface references appropriately. Reason: barns, churches, tanks and surface samples must not be miscounted as completed facade buildings; part: 04; dependencies: structure classification and reference/model grammar; acceptance: each ID becomes its appropriate volumetric structure or usable material asset with scale, provenance and export metadata; verification: type-aware coverage, geometry/material intake and native visual review. Partial: 28 standalone structures and 18 volumetric surface modules exported; final material/art and production intake gates remain open.

- [ ] **P11-002** Create a road-world art bible for distinct town, farm, cliff, coast, and woodland identities before polishing bike/rider art. Partial: [Valley architecture vocabulary and construction contract](art/BUILDING_GRAMMAR.md); cliff/coast/woodland and full art direction remain open.

- [ ] **P11-004** Author modular roads/intersections/sidewalks, storefronts/homes, barns/silos/fields/fences, cattle/tractors, cliffs/rocks/barriers, signs, poles, and landmarks. Partial: [polish06 road/paving materials](testing/polish-12/pass06/README.md) connects original mipmapped asphalt to Metropolis, with restrained repairs, opaque faded paint and weathered paving seams; full modular content, authored drainage and human art/temporal acceptance remain open.

- [ ] **P11-005** Build the route-zone/lot/terrain/encounter pipeline and foot-accessible collision/navigation surfaces described in the environment checklist below.

- [ ] **ENV-001** E0: author spline, widths, shoulders/sidewalks, vertical curves, checkpoint and finish volumes. Partial: [polish pass03](testing/polish-12/pass03/README.md) adds a shared authored C2 Metropolis profile, tiled apron/fields and both-route support checks; full-course human/terrain acceptance remains open.

- [ ] **ENV-002** E1: author distance-station zones (example: town 0-2 km, farm 2-5 km, cliffs 5-8 km) with smooth art/audio transitions.

- [ ] **ENV-003** E2: stamp roadside lots/terrain cells; reserve 10-30 m of critical visibility at bends and intersections.

- [ ] **ENV-005** E4: validate bike and on-foot collision/navigation through curbs, barns, fences, buildings, and cliffs; remove trapping/invisible barriers.

- [ ] **ENV-007** Define `WorldZone` ID/from/to/biome/road style/speed profile/scenery/audio/music/encounters/hazards/safe-spawn fields and corresponding schema validation.

- [ ] **ARCH-005** Author track bundles containing road, ordered checkpoints, spawn groups, props/LOD, original source art, runtime GLB, and separate collider GLB.

- [ ] **RIGHTS-007** Sessions 03-04: spline mesh/collision road and valid start/finish/checkpoint gates.

- [ ] **P02-004** Replace the generated flat polyline with an authorable Catmull-Rom/Bezier centerline supporting widths, banking, elevation, sectors, and optional route splits/merges.

### Part 05 — World B — props, materials and procedural dressing

- [ ] **RFU-WILD-05** Keep wild herds distinct from mounted cowboys/static animals and record original asset provenance. Reason: issue32 adds wildlife without replacing existing content; part:05; dependencies:HILLS-03/HILLS-05; acceptance: original recognizable horses/bison, no saddles/riders, retained existing animals and complete source/license records; verification: model/asset audit, native close views and user art review. Partial integrated evidence: [issue #32 wildlife report](testing/wildlife/REPORT.md); remaining acceptance gates stay open.

- [ ] **RFU-RAIL-03** Place ordinary guardrails on the requested turn side. Reason: explicit right-turn/right-side and left-turn/left-side request; dependencies:HILLS-01; acceptance: stable horizontal signed turns through grades, open straights/opposite side and separate authored bridge policy; verification: signed-yaw rules and placed collider survey. Part:05. Physical, integrated and seven native functional checks pass; human full-course and broad hardware/performance acceptance remain open: [guardrail work](testing/guardrails/REPORT.md).

- [ ] **RFU-RAIL-04** Terminate every guardrail run and gap end with solid end hardware. Reason: exposed thin beam ends; dependencies:RFU-RAIL-02/03; acceptance: readable flared terminals with matching collision and useful gap clearance; verification: terminal census/contact tests, native close views and traversal. Part:05. Physical, integrated and seven native functional checks pass; human full-course and broad hardware/performance acceptance remain open: [guardrail work](testing/guardrails/REPORT.md).

- [ ] **RFU-WORLD-04** Add substantial appropriate western settlement props. Reason: user reports barren Hills/western surroundings and low river in issue31; part:05; dependencies:HILLS-06 and HILLS-07; acceptance:supported shop goods, lights and outskirts scenery with clear roads and resident solids; verification:placed-geometry/clearance tests and close/wide native views. Partial shop-front and outer-lot dressing with clearance checks; remaining gates: [world follow-up](testing/issue31-world/REPORT.md).

- [ ] **DESTRUCTION-02** Author dedicated broken prop variants and review damage readability. Reason: runtime crush/topple retains original meshes but is coarse damage art; part: 05; dependencies: DESTRUCTION-01, V2-05-002 and approved prop art; acceptance: original severed/broken/deformed variants for all13 Metropolis types with matching solid colliders, clear before/after silhouettes and budgeted debris, preserving racer physics and restart; verification: per-role native contact sheets, collider/reset tests and measured damaged-street performance. Partial: [Part05 furniture pass](testing/part05-props/README.md) adds12 original-source buckled variants for the six crush roles with ready/material-gated presentation, preserved solid bounds and contact-timed melee. Seven topple roles, finished art approval and full damaged-street performance remain open.

- [ ] **WORLDART-01** Rebuild the world art around the confirmed 3D Come Bye direction and measured property plan. Reason: current city/terrain/sky assets and placement remain inconsistent; part: 05/06; dependencies: WORLDPLAN-01, LAYOUT-01, LAYOUT-02, LAYOUT-03, MAT-02/MAT-03, stable LOD and performance budget; acceptance: coherent properties/field borders/access/fences, intentional building height groups,2/3/4lane transitions, consistent building textures, improved terrain/clouds/sky and readable RoadDash identity; verification: plan-to-geometry/access audits, close/far and rural/city native views, measured performance and user visual acceptance. [Direction and plans](planning/world-redesign/README.md).

- [ ] **MAT-03** Keep distant facade proxies synchronized with new texture families. Reason: runtime-only palette or PBR changes would reintroduce close/far identity shifts; part: 05; dependencies: LOD-04 and MAT-02; acceptance: rebake against delivered close material data, update source/output hashes and mip guards, preserve matching close/far appearance; verification: source/hash audit, material coverage and paired native distance views. Partial: [polish07 source material pass](testing/polish-12/pass07/README.md) coordinates24 placed families, preserves geometry, rebakes all383source bands/mips and aligns effective roughness with runtime profiles. Human near/far art, extended streaming and hardware/performance acceptance remain open.

- [ ] **MAT-02** Author bespoke detailed albedo/PBR texture families for remaining buildings, vehicles, props and riders. Reason: shared microdetail does not finish every requested texture; part: 05; dependencies: MAT-01, source provenance and approved art direction; acceptance: per-family approved source art, consistent texel density, mip/normal/roughness review, native close/far views with no missing maps; verification: asset hash/license audit, contact sheets and in-game visual approval.

- [ ] **V2-05-001** Inventory reference prop categories and gameplay roles. [Scoped source catalog and rebuild queue](research/props/README.md) extracted across all five courses; category/role proposals and eleven visually reviewed unresolved banks remain subject to identity/archetype/source-use review. Includes pedestrians, animals, traffic/parked vehicles, lamps/utilities, trees/plants, signs, furniture, fences, rocks and ground hazards. Source view/color/size banks are not unique finished models.

- [ ] **V2-05-002** Author original prop meshes and texture families. [Actual draft mesh catalog](../art/props/mesh_assets/README.md): 369 source IDs, 1,107 GLBs, 13 editable Blender scenes and exported-mesh proof renders. These are rough shared-archetype geometry drafts, not faithful finished reproductions of the [66 concept sheets](../art/props/concept_3d/ALL_CONCEPTS.html). Significant silhouette/material/pictogram/actor refinement, finished deformation rigs, collider refinement, wider placement and final art approval remain open. Selected drafts now run in game through PROP-03. Inspect with `cargo run --locked --bin prop_preview`; accepted race physics remain unchanged.

- [ ] **V2-05-003** Place props through semantic rules. PROP-03 adds reserved city sockets, existing tree/lamp replacements and traffic/NPC attachment; DRESS-01 adds131 guarded Valley props across15 roles. Full category placement, variety and encounter review remain open.

- [ ] **V2-05-004** Give gameplay props reliable physical/audio behavior. PROP-03 preserves existing moving-owner and ragdoll collisions and adds solid furniture; Metropolis breakables and synthesized break cues are verified in DESTRUCTION-01; other category audio and comprehensive behavior/art QA remain open.

- [ ] **V2-05-005** Exit gate: dressed world remains playable.

- [ ] **ENV-004** E3: place memorable landmarks plus interactive fences, cows, cars, rocks, and tractors; make encounters readable rather than unavoidable surprise spawns.

### Part 06 — World C — NPCs first, then sky/terrain/landscape art

- [ ] **RFU-WILD-01** Add coherent moving wild horse herds to Hills. Reason: issue32 requests life beyond traffic; part:06; dependencies:HILLS-01, RFU-WILD-03/04; acceptance: distinct off-road regional herds, terrain-grounded continuous gait/group movement and no nearby frozen actors or population popping; verification: whole-path clearance/coherence tests and native near/mid/far/traversal motion captures. Partial integrated evidence: [issue #32 wildlife report](testing/wildlife/REPORT.md); remaining acceptance gates stay open.

- [ ] **RFU-WILD-02** Add varied bounded bison herds to Hills. Reason: issue32 requests regional herd variety; part:06; dependencies:RFU-WILD-01/03/04; acceptance: readable original bison, varied group sizes, coherent motion and suitable mountain/arid-transition placement; verification: population/path/gait bounds and native motion/art review. Partial integrated evidence: [issue #32 wildlife report](testing/wildlife/REPORT.md); remaining acceptance gates stay open.

- [ ] **RFU-WILD-03** Bound wildlife populations, distance work and LOD cost. Reason: performance is the highest priority; part:06; dependencies:PERF-08 and RFU-WILD-01/02; acceptance: explicit population/animation/render budgets, no distant per-animal terrain/path work, matched release enabled/disabled loading/frame/memory measurements with representative traffic; verification: runtime counters, matched quiet performance repeats and native distance traversal. Broad hardware acceptance remains open. Partial integrated evidence: [issue #32 wildlife report](testing/wildlife/REPORT.md); remaining acceptance gates stay open.

- [ ] **RFU-WILD-04** Specify and verify wild-herd road/contact policy. Reason: issue32 did not authorize new animal hazards; part:06; dependencies:existing collision/recovery and HILLS-01; acceptance: off-road scenic-only herds, no road crossing/colliders/damage, full swept road/fence/prop clearance and retained existing animal contacts; verification: authoring/placed-geometry tests, native contact inventory and both-rig recovery controls. Partial integrated evidence: [issue #32 wildlife report](testing/wildlife/REPORT.md); remaining acceptance gates stay open.

- [ ] **RFU-RAIL-05** Validate guardrail gap spacing, terrain access and bridge exceptions. Reason: openings must work in the actual landscape; dependencies:RFU-RAIL-01/02/03/04 and HILLS-08; acceptance: bounded long runs, practically traversable gaps, preserved open suspension edges/thick supports, both branch/rig recovery and human full-course acceptance; verification: native gap/route/recovery receipts, collision tests and full-course review; hardware/performance acceptance remains open. Part:06. Physical, integrated and seven native functional checks pass; human full-course and broad hardware/performance acceptance remain open: [guardrail work](testing/guardrails/REPORT.md).

- [ ] **RFU-CITY-01** Increase Metropolis urban pedestrian population and purposeful crosswalk use. Reason: city still feels sparse; part:06; dependencies: CITY-ACTIVITY-02, patrol authoring and RFU-PERF-06; acceptance: measured higher resident count than384, more completed grounded crossings without crowd jams, rural counts and solid interactions preserved; verification: generated census, crossing progress tests, both-route native surveys and matched frame profiles. [Issue](https://github.com/ImadMashhood/RoadDash/issues/43). Authoring and scripted both-route native evidence: [issue #43 city-life report](testing/issue-43-city-life/REPORT.md); matched frame and human review gates remain open.

- [ ] **RFU-CITY-02** Widen usable Metropolis sidewalks without shrinking the approved roadway. Reason: current walking bands look narrow; part:06 coordinating04; dependencies: ROAD-WALK-01, SIDEWALK-01 and RFU-CITY-01; acceptance: continuous capsule-clear walking routes, retained4/3/4 lanes and closed building envelopes/parking; verification: authored right-of-way geometry, physical two-way foot crossings and native street views. [Issue](https://github.com/ImadMashhood/RoadDash/issues/43). Authoring and scripted both-route native evidence: [issue #43 city-life report](testing/issue-43-city-life/REPORT.md); matched frame and human review gates remain open.

- [ ] **RFU-CITY-03** Line suitable Metropolis sidewalks with more trees. Reason: avenues remain bare; part:06 coordinating05; dependencies: RFU-CITY-02 and existing vegetation/collision; acceptance: varied bounded supported tree placement with pedestrian clearance, no doorway/crosswalk blockage and distance LOD; verification: placement/collision census, close/far native views and render cost. [Issue](https://github.com/ImadMashhood/RoadDash/issues/43). Authoring and scripted both-route native evidence: [issue #43 city-life report](testing/issue-43-city-life/REPORT.md); matched frame and human review gates remain open.

- [ ] **RFU-CITY-04** Broaden Metropolis curb and district prop variety. Reason: repeated furniture makes western/city blocks look plain; part:06 coordinating05; dependencies: RFU-CITY-02/03, prop catalog and RFU-PERF-06; acceptance: original varied district-specific props with solid/support-aware placement and readable crossing/doorway space; verification: source/rights audit, placed-geometry census, native views and frame cost. [Issue](https://github.com/ImadMashhood/RoadDash/issues/43). Authoring and scripted both-route native evidence: [issue #43 city-life report](testing/issue-43-city-life/REPORT.md); matched frame and human review gates remain open.

- [ ] **RFU-TRAF-01** Pass sustained slower civilian leaders. Reason: issue27 reports indefinite tractor/truck queues; part:06; dependencies:P08-005 and shared traffic support; acceptance: tractors remain slowest, semis next, cars faster, with eventual passing in an opening; verification: deterministic core/native multi-follower runs on Hills/Metropolis. Evidence: [civilian overtaking](testing/civilian-overtaking/REPORT.md): native selected passes, full-fleet progress and corrected endpoint admission verified; human full-course acceptance remains open.

- [ ] **RFU-TRAF-02** Reserve opposing-lane passes using vehicle length, relative speed and merge room. Reason: opposing large traffic must prevent unsafe completion; part:06; dependencies:RFU-TRAF-01; acceptance: both directions reject blocked horizons and retain terrain/bridge support; verification: oncoming-car/long-trailer fixtures and native motion/support checks.

- [ ] **RFU-TRAF-03** Allow civilian passing despite an upright oncoming biker. Reason: explicitly requested gameplay hazard; part:06; dependencies:RFU-TRAF-02 and physical contact/recovery; acceptance: biker presence alone does not veto a pass, no teleports/collision exemptions, downed-person yielding retained; verification: paired biker/no-biker native decision fixture and collision/recovery regressions.

- [ ] **RFU-TRAF-04** Resolve competing civilian passes and new obstructions. Reason: prevent overlapping claims and perpetual queues; part:06; dependencies:RFU-TRAF-01/02; acceptance: deterministic reservation, bounded abort/merge and eventual progress with room; verification: multi-follower and mid-pass obstacle tests, measured planner cost and native motion; long-race/hardware performance acceptance remains open.

- [ ] **RFU-SKY-01** Verify convincing spatial clouds in Metropolis. Reason: issue30 user visual request; part:06; dependencies:SKY-SUN-01 and HILLS-10; acceptance: depth/parallax in native driving and distant views using the shared bounded renderer; verification: cloud density/budget tests and moving native captures, separate user art approval.

- [ ] **RFU-SKY-02** Add visible natural bird groups to Metropolis. Reason: issue30 finds Hills-only bird spawning; part:06; dependencies:RFU-SKY-01 and existing atmosphere lifecycle; acceptance: bounded coherent flocks, inexpensive gradual distance behavior, pause/reset safety; verification: installed pool/flock/lifecycle tests and moving native rural/city views.

- [ ] **RFU-SKY-03** Preserve coherent sunlight, final render-camera culling and Hills atmosphere. Reason: issue30 shared-renderer integration; part:06; dependencies:SKY-01 and HILLS-04; acceptance: one light direction/exposure profile, propagated-camera fade, unchanged physical rigs/content and Hills dust; verification: both-map install/lighting tests and native controls.

- [ ] **RFU-SKY-04** Expose independent cloud/bird quality controls and measure their cost. Reason: issue30 prioritizes performance; part:06/16; dependencies:PERF-08 and RFU-SKY-01/02; acceptance: bounded full/low/fallback clouds and bird switch, matched release wall-frame/load/memory comparisons on both maps; verification: quality tests and independent-toggle native runs, GPU/broad hardware gates remain separate.

- [ ] **RFU-WORLD-01** Extend hillside woodland onto distant slopes. Reason: user reports barren Hills/western surroundings and low river in issue31; part:06; dependencies:HILLS-03 and PERF-08; acceptance:supported clustered trees at near and distant driving views, bounded spatial meshes and resident collision; verification:deterministic placement/road/support tests, native route views and measured render cost. Partial implementation and remaining gates: [world follow-up](testing/issue31-world/REPORT.md).

- [ ] **RFU-WORLD-02** Vary and widen under-dressed grass and vegetation. Reason: user reports barren Hills/western surroundings and low river in issue31; part:06; dependencies:HILLS-03 and RFU-WORLD-01; acceptance:irregular meadow coverage and clear farms/roads, bounded rendering; verification:placement tests and matched driving views. Partial implementation and remaining gates: [world follow-up](testing/issue31-world/REPORT.md).

- [ ] **RFU-WORLD-03** Add western plateaus, mountains and varied landforms. Reason: user reports barren Hills/western surroundings and low river in issue31; part:06; dependencies:HILLS-02 and HILLS-07; acceptance:continuous remote relief outside settlement lots and unchanged road profiles; verification:terrain continuity/support tests and both-branch native captures. Partial implementation and remaining gates: [world follow-up](testing/issue31-world/REPORT.md).

- [ ] **RFU-WORLD-05** Enrich desert Hills through landforms, scenery and wildlife coordination. Reason: user reports barren Hills/western surroundings and low river in issue31; part:06; dependencies:RFU-WORLD-03/04 and issue32 wildlife; acceptance:distinct populated desert vistas without additional traffic budgets; verification:full-route views, wildlife integration and combined cost measurement. Partial implementation and remaining gates: [world follow-up](testing/issue31-world/REPORT.md).

- [ ] **RFU-WORLD-06** Raise the bridge river with coherent banks and support. Reason: user reports barren Hills/western surroundings and low river in issue31; part:06; dependencies:HILLS-08 and HILLS-11; acceptance:water remains seated on raised resident canyon bed, open bridge edges and deep physical falls retained; verification:exact terrain/water tests and native bank/bridge/fall/recovery cases. Partial implementation and remaining gates: [world follow-up](testing/issue31-world/REPORT.md).

- [ ] **RFU-WORLD-07** Blend road, sand and grass without hiding physical classification. Reason: user reports barren Hills/western surroundings and low river in issue31; part:06; dependencies:issue29 sand boundary and HILLS-03; acceptance:consistent per-vertex material transition and physical biome classification; verification:boundary assertions and matched native road/grass/sand captures. Partial implementation and remaining gates: [world follow-up](testing/issue31-world/REPORT.md).

- [ ] **HILLS-11** Add a visible river beneath the Hills suspension bridge. Reason: user requests a river in the canyon; part:06; dependencies:HILLS-08 and resident canyon terrain; acceptance: continuous original water surface seated in the canyon with readable banks and bounded presentation, no invisible bridge support or changes to accepted rider physics; verification: terrain/water bounds tests and native bridge/bank views.

- [ ] **SKY-SUN-01** Make the visible sun distinct and coherent with separate volumetric clouds. Reason: user cannot see a real sun and rejects flat skybox-like clouds; part:06; dependencies:SKY-01 and HILLS-10; acceptance: clearly readable solar disc aligned with the directional light, spatial cloud depth separate from the display sky with bounded cost, consistent fog/exposure and other-map behavior; verification: sun-direction/cloud-volume tests and native sun/cloud views.

- [ ] **HILLS-10** Add bounded volumetric clouds to Hills with measured cost. Reason: user requests volumetric clouds while prioritizing speed; part:06; dependencies:HILLS-04 and PERF-08; acceptance: visible spatial cloud depth, coherent daylight, fixed resource/step budgets and a measured low-cost option, no unverified FPS claim; verification: native cloud views, fixed-budget tests and matched release performance comparison.

- [ ] **HILLS-07** Replace the plain Hills composition with a clearly urban opening, developed rolling-hill farms and giant rocks, and dense close western frontage. Reason: user rejected the sparse result and requested the earlier farm concept; part:06; dependencies:HILLS-01/03/06; acceptance: city before hills, no opening farms, supported farm parcels on hills, visibly denser buildings close to both town roads without blocking roads or mounted paths; verification: actual footprint/support tests, driving-height city/farm/town captures and full-course review. Partial implementation and current checks: [revision evidence](testing/hills-revision/REPORT.md); full acceptance remains open.

- [ ] **HILLS-08** Add a suspension bridge over a physical canyon with open edges and thick square supports. Reason: user requests a memorable bridge where riders can fall or be kicked off; part:06; dependencies:HILLS-01 and existing collision/recovery; acceptance: supported suspended deck, deep canyon below, original towers/cables, open deck edges with no low railing or hidden blocking collider, thicker square vertical supports and working fall/recovery; verification: physical deck/open-edge/support/fall tests and native bridge views and fall fixture, preserving accepted rigs. Partial implementation and current checks: [revision evidence](testing/hills-revision/REPORT.md); full acceptance remains open.

- [ ] **HILLS-09** Add moving eighteen-wheel semitrailers and agricultural tractors to Hills traffic. Reason: user requests richer vehicle traffic; part:06; dependencies:HILLS-01/02/08 and existing traffic/network interfaces; acceptance: recognizable original/reused vehicles, full-size collision and articulated turning, safe supported course/branch traversal, consistent traffic avoidance and guest presentation, no other-map regression; verification: wheel/dimension/path tests, native moving-traffic views, bridge/town traversal and recovery controls. Partial implementation and current checks: [revision evidence](testing/hills-revision/REPORT.md); full acceptance remains open.

- [ ] **HILLS-04** Polish Hills atmosphere with grounded dust, flying birds, coherent clouds and improved lighting. Reason: explicit extra polish request; part:06; dependencies:HILLS-01/03 and existing sky/presentation systems; acceptance: bounded effects, dust tied to actual moving ground contacts/surfaces, pause/reset safety, coherent sun/fog/IBL, no other-map or physics regression; verification: effect budgets/state tests, native active views and recovery/legacy controls. Human art and full-course performance remain open. Partial: implemented and technically checked in [Hills polish evidence](testing/hills-polish/README.md); final human acceptance remains open.

- [ ] **HILLS-05** Add cowboys riding moving horses around Hills. Reason: explicit mounted-life request; part:06; dependencies:HILLS-03 and authoritative world-actor interfaces; acceptance: recognizable riders, horse gait, supported clear patrols, matched physical bodies, pause/restart and host/guest consistency without player-rig tuning; verification: path/body/animation tests, native movement/contact/recovery evidence. Full animal behavior/art remains open. Partial: implemented and technically checked in [Hills polish evidence](testing/hills-polish/README.md); final human acceptance remains open.

- [ ] **HILLS-06** Enrich Hills with original cacti, tree variation, rocks/deadwood and western street details. Reason: user grants agents room for environmental polish; part:06; dependencies:HILLS-03; acceptance: biome-specific original/reused assets, road-clear supported placement, bounded detail independent of solid collision and readable town/dunes; verification: actual-placement/support tests, provenance and native close/wide views. Final art and target-hardware performance remain open. Partial: implemented and technically checked in [Hills polish evidence](testing/hills-polish/README.md); final human acceptance remains open.

- [ ] **HILLS-01** Add a separate12.5km Hills course with city opening, winding elevated hills, flat western town and dune finish. Reason: user map request/reference on2026-11-10; part:06; dependencies: existing track, road/terrain collision and race systems; acceptance: selectable course, strong supported relief/guardrails, preserved legacy maps/default, complete race/checkpoints/retry and both physical rigs; verification: route/content tests, locked regression suite and native Hills survey/recovery. Human full-course feel and hardware review remain open.

- [ ] **HILLS-02** Split Hills at town entry and rejoin inside the town before its exit. Reason: user's explicit correction to the reference diagram; part:06; dependencies:HILLS-01 and existing branch navigation; acceptance: fork around5.5km, merge around7.4km with town continuing to8.5km, two supported race corridors, correct AI/traffic/minimap/checkpoints and no shortcut wins; verification: route projection/choice/gate tests and native both-branch coverage.

- [ ] **HILLS-03** Populate Hills with original western storefronts, cattle, horses, cowboys and trucks, plus trees, grass, farms and city/hill/dune scenery. Reason: requested setting, asset reuse and follow-up for a livelier countryside; part:06; dependencies:HILLS-01/HILLS-02 and original prop catalog; acceptance: buildings on both sides of both town branches and after merge, road-clear trees/grass/pastures/farms, solid supported props/railings, distinct requested silhouettes and retained collision/recovery; verification: asset provenance, placed-geometry tests and native visual review. Final art, animal animation and broad performance approval remain open.

- [ ] **CROP-TRAIL-01** Add farm crops with animated persistent crash, walking and driving deformation. Reason: user wants visible interaction trails in fields; part: 06; dependencies: WORLDART-01, current fixed simulation and vegetation renderer; acceptance: actual field crops, contact-only continuous footprints and trails, smoothly animated bends retained through the race and cleared on restart, bounded state/uploads/geometry; verification: contact/reset/sweep tests, native visual sequences and measured memory/CPU/render cost with documented method comparison.

- [ ] **CITY-LIFE-02** Increase only urban traffic/pedestrian density and eliminate kinematic recycle-speed spikes. Reason: city is underpopulated and crossing cars occasionally jump at extreme speed; part: 06; dependencies: DOWNTOWN-01, AI-12; acceptance: extra cars stay in city, increased capsule-clear urban patrols, backend-safe recycling, preserved farm density and legacy maps; verification: fixed-step recycle regression, authoring counts and native actor/traffic surveys; broad hardware performance remains open.

- [ ] **NPC-SEAMS-01** Repair civilian source binds/weights and visible segment seams. Reason: user reports floating NPC pieces; part: 06; dependencies: current actor skin catalog and preserved physical drivers; acceptance: connected-looking torso/limbs throughout walking, impact and ragdoll poses without replacing physics; verification: per-family bind/weight/attachment tests and close-up native visual review. Actor lifecycle smoke alone is insufficient. Partial: [Part06 civilian weight pass](testing/part06-npcs/README.md) adds shared source-space multi-bone fields, a continuous pelvic midline and bounded conforming neutral-surface refinement across55 families/two LODs; exact physics/cow assets remain unchanged. Mean posed attachments improve, but some coarse torso/shoulder samples worsen, original volume seams/art approval and full performance remain open.

- [ ] **V2-06-001** Build NPC reference catalog and original character variants. Pedestrian/animal source banks are included in the [shared prop catalog](research/props/README.md); PROP-03 binds draft human/cow appearances to existing walking and ragdoll frames; identity/species review, finished deformation and animations remain open. Preserve the accepted human/quadruped physical rigs when replacing visuals.

- [ ] **V2-06-002** Create animal-specific art, animation and physical responses.

- [ ] **V2-06-003** Populate the world with believable NPC activity.

- [ ] **V2-06-004** NPC exit gate before the landscape pass.

- [ ] **V2-06-005** Study skyboxes, atmosphere and landscape references. Partial: [landscape source/compatibility review](testing/landscape-graphics/PROVENANCE.md) records three pinned open-source Bevy grass implementations and the supplied pastoral references; no external code/media imported.

- [ ] **V2-06-006** Author original sky and landscape textures/materials. Partial: [landscape follow-up](testing/landscape-graphics/README.md) adds shared original mipmapped ground flecks, coherent pasture/grain/fallow tint and material-matched soil edges; full landscape art acceptance remains open.

- [ ] **V2-06-007** Integrate landscape art with the playable world. Partial: [bounded farm landscape pass](testing/landscape-graphics/README.md) adds original deterministic grass, coherent field variation and irregular soil shoulders; full-map art, temporal/hardware/performance and user acceptance remain open. Follow-up ADR118 adds a gravel-to-grass verge, fence-clear cow placement and16 existing farm props; see [farm verge evidence](testing/farm-verges/README.md). ADR119 adds thinner curved grass with bounded GPU wind; see [grass and clothing evidence](testing/stylized-humans-grass/README.md).

- [ ] **V2-06-008** Landscape exit gate: the complete first-map art pass is accepted.

- [ ] **ENV-008** Implement cow grazing/startled/turn/cross/react states with simple critical collision proxies; animal ragdolls remain optional cosmetics.

- [ ] **P08-005** Add traffic gap keeping, braking, lane changes/intersections, and obstacle rerouting.

### Part 07 — Weapons and complete riding combat

- [ ] **V2-07-001** Research weapon behavior and define original data/assets. Partial: pinned R01 study, original Club/Chain/Whip/RollingPin/BaseballBat sources/GLBs and validated schema2 family reach/stamina/cooldown, with schema1 compatibility (ADR085/100); full specialist behavior/art and acceptance remain open.

- [ ] **V2-07-002** Integrate weapon animations, contacts and outcomes. Partial: five typed inventory/pickup/drop families, frozen family contact rules and ready model fallback now integrate with existing side clips/recovery (ADR100); distinct/flexible attack motion, holster/equip art and acceptance remain open.

- [ ] **V2-07-003** Teach rivals fair weapon use and tune combat. Partial: stable seeded family loadouts, per-family reach/cost/cooldown, shared pickup/LOS/tells and exact attacker retaliation; defense/personality tuning and human balance remain open. See [fresh arsenal evidence](testing/part07-arsenal-fresh/README.md).

- [ ] **V2-07-004** Exit gate: weapons improve the race loop.

- [ ] **ARCH-004** Define weapon timing/range/stamina/damage/stagger/knockback/pickup data and bike class/mass/top speed/torque/grip/lean/durability/visual-asset data as authored files.

- [ ] **RIGHTS-011** Sessions 13-15: punches/kicks, timing/range/stagger validation.

- [ ] **P05-004** Define typed ability data for startup/active/recovery, range, stamina cost, impulse, damage, stagger, and cooldown; initial punch timing 0.15/0.12/0.30 s and kick timing 0.24/0.16/0.50 s are tuning proposals.

- [ ] **P05-005** Use stable side/front cones and transient swept sensor volumes; resolve each attack once with attacker/target/attack ID/contact point/relative speed/tick/result fields.

- [ ] **P05-007** Build a matched-speed side-by-side combat harness; test no double-hit, misses 0.5 m beyond range, and cooldown/stamina/immunity boundaries. Partial: first-club real-input moving fixtures at30/60/120/144Hz cover contact poses, misses/LOS/side/immunity, data changes and interruption; full harness presentation and exhaustive boundary matrix remain open.

- [ ] **P05-008** **Exit gate:** three minutes of enjoyable punch/kick combat without weapons, with timing and hit validation evidence.

### Part 08 — Police, pursuit and arrest/escape loop

- [ ] **POLICE-ART-01** Give the officer a distinct uniform texture and custom police motorcycle. Reason: user wants identifiable police presentation; part: 08; dependencies: COP-01, current rider/bike assets; acceptance: original readable officer/bike identity at gameplay distance, preserved rider/bike physical rigs and pursuit/arrest/recovery behavior; verification: model parity, police native captures and pursuit/recovery tests.

- [ ] **V2-08-001** Research original police behavior and author police content.

- [ ] **V2-08-002** Implement bounded police pursuit and interceptions.

- [ ] **V2-08-003** Integrate rider knockoff, on-foot pursuit and arrest/escape.

- [ ] **V2-08-004** Tune police feedback and fairness.

- [ ] **V2-08-005** Validate the complete offline loop before Part 09.

- [ ] **V2-08-006** Exit gate: proceed to existing Part 09, then Part 10.

- [ ] **RIGHTS-014** Sessions 23-24: police, town/farm/cliff audio zones, placeholder soundscape.

- [ ] **P08-004** Implement Cruise/SeekOvertake/Defend/Attack/Recover/Pursued/Arrested states with a blackboard and aggressive/cautious/opportunistic/reckless weights.

- [ ] **P08-009** **Exit gate:** fair, repeatable four-racer race with traffic, police, and melee, including opponent overtakes, hits, and recovery.

### Part 09 — Vertical-slice playtest gate

- [ ] **P09-AUTO-01** Verify an isolated current-course scripted slice suite. Reason: reproducible failure-reporting is needed before external playtests; part: 09; dependencies: native frontend, club grid, police and recovery fixtures; acceptance: protected new receipt directories, isolated saves/preferences, pinned seeds, process failure/timeout reporting and actual native entry/police/recovery passes on the approved map; verification: CLI tests plus per-case native logs/reports. Partial: entry mode is implemented; the combined full suite remains unverified.

- [ ] **P09-001** Consolidate the three-minute town/farm/cliff route with player plus three AI, traffic/police, punch/kick, manual foot control, foot finish, checkpoints, results, and retry.

- [ ] **P09-002** Run at least ten external play sessions with recordings, handling A/B comparisons, perceived-speed/crash feedback, and failure/input notes.

- [ ] **P09-003** Measure finish rate, retries, voluntary replay, attack/hit counts, collisions, and time outside player control; record the five biggest frustrations.

- [ ] **P09-004** Test the proposed fun targets: 80% finish within three tries, over 50% voluntary replay, and time outside control below 12% of a session.

- [ ] **P09-005** Measure a 1080p Fedora/RTX 3080 slice with all P0 systems active against a stable 60 FPS baseline; store P95 frame-time evidence.

- [ ] **P09-006** Record keep/change/remove decisions and freeze the minimal controller/combat/race interfaces for the network spike.

- [ ] **P09-007** **Exit gate:** core loops need no debug intervention and external playtest evidence supports replaying the graybox; retune before expanding content otherwise.

- [ ] **ENV-009** Verify a distinct silhouette/encounter every 10-25 s at 100-220 km/h and recognizable locations without reading the HUD; record the on-foot hazard audit and frame times.

- [ ] **RIGHTS-010** Sessions 11-12: timer, standings, results, complete retry loop and evidence.

- [ ] **RIGHTS-013** Sessions 19-22: AI and traffic budgets, repeatable full race.

- [ ] **RIGHTS-015** Sessions 25-27: ten external playtests, metrics, tuning, frustration fixes.

- [ ] **P02-010** Build a short approximately 2 km handling road, then a roughly 6-9 km town -> farm -> cliff corridor tuned for a three-minute race, with straight, turns, hairpin, and hazards.

- [ ] **P02-011** **Exit gate:** an authored graybox supports a complete race with valid gates, stable collision, clear progress, and no terrain fall-through.

- [ ] **P07-003** Add seeded starts, ordered forward-crossed checkpoints, route segments, canonical racer-level progress, and out-of-bounds recovery.

- [ ] **P07-004** Accept a valid forward finish from either rider or bike with shared racer identity; record source OnFoot/Bike and a single finish tick/event/payout. **Correction:** bike/foot finish foundations already exist (FOOT-001/002/006); full Part09 race/edge-case acceptance remains open.

- [ ] **P07-005** Reject backward crossings, skipped gates, shortcuts, and finish teleports; handle same-tick rider/bike crossing exactly once.

- [ ] **P07-006** Add distance/progress, bike damage, weapon state, and on-foot bike-direction/distance HUD; retain stable placement on split routes and off-road movement.

- [ ] **P07-008** Verify retry retains settings while resetting all live race state, including traffic and recovery actors.

- [ ] **P07-009** **Exit gate:** repeatable approximately three-minute start-to-results-to-retry race with stable, checkpoint-validated bike and foot finishes.

- [ ] **P08-003** Route AI through the same `BikeControl`/vehicle physics interface as human input; remove hidden grip/acceleration differences unless explicitly presented as difficulty assists.

- [ ] **P08-008** Add AI debug goals, paths, desired speeds, and headless tournaments; complete 100 races without deadlock and test human/AI foot recovery separately.

### Part 10 — Networking feasibility spike

- [ ] **P10-001** Introduce Lightyear only after the offline slice gate; run a headless authoritative server with two LAN clients on one graybox track.

- [ ] **P10-002** Send tick/sequence-numbered input batches covering bike controls, attacks, manual foot movement, and remount; server owns simulation, hits, rankings, and rewards.

- [ ] **P10-003** Predict local movement, buffer/interpolate remote snapshots, reconcile errors, and deduplicate confirmed combat/crash/result events.

- [ ] **P10-004** Test 2 -> 4 players, high-speed opposing collisions, and punch adjudication; consider server-owned simplified/cosmetic client ragdolls.

- [ ] **P10-005** Measure 60 Hz server simulation, 20-30 Hz snapshots, and an initial bandwidth target below 100 kbps/client for four players.

- [ ] **P10-006** Exercise 0/50/100/150 ms RTT, jitter, and 0/1/5% packet loss; record correction distances, response, recoverability, and consistent hit outcomes.

- [ ] **P10-007** **Exit gate:** two clients race at 100 ms simulated RTT, loss does not create a desync storm, and the authority/physics approach is documented with measurements rather than deterministic-lockstep assumptions.

- [ ] **ARCH-008** Document snapshot/prediction/reconciliation assumptions and clock offsets; do not require cross-machine bit-identical physics or replicate individual cosmetic fragments.

- [ ] **RIGHTS-016** Sessions 28-30: reproducible two-client/server Lightyear spike and 100 ms test report.

- [ ] **P06-011** Separate simulation-critical colliders from cosmetic debris; budget ragdoll bodies/joints and avoid per-fragment replication.

### Part 11 — Remaining production-art integration and acceptance

- [ ] **PERF-08** Profile and optimize the measured low-FPS bottleneck on the user's hardware. Reason: instrumentation and aggregate utilization alone do not establish root cause or60FPS; part: 11; dependencies: PERF-07, reproducible farm/edge/downtown and damaged-course logs; acceptance: identify dominant main/render/simulation/paging spans, capture GPU timestamps when needed, make justified focused optimizations and demonstrate target P95 frame budget without removing authoritative hazards; verification: before/after native logs, GPU profiler receipts when applicable and user hardware confirmation. Current user-authorized follow-up: controlled release profiling, collision-pair classification and cheaper pedestrian crossing admission on `codex/performance-profile-pass`; evidence and rejected experiments are recorded in [performance pass](testing/performance-pass/README.md). Partial: [ADR089 performance pass](testing/performance-ui-prompts/README.md) adds separate sync/step/writeback and catch-up diagnostics, stable render updates and exact semantic static-collider batching; full target-hardware city/crash frame-budget acceptance remains open. Sequential polish pass01 adds retained-only facade selection hysteresis and paired native profiling; see [pass01 evidence](testing/polish-12/pass01/README.md).

- [ ] **P11-009** Validate readable scenery at racing speed, zero missing material/texture references, provenance/credits, and asset swaps without gameplay changes.

- [ ] **P11-010** **Exit gate:** the original town/farm/cliff art replaces graybox assets without changing physics/race rules and fits measured budgets.

### Part 12 — Garage, economy, progression and saves

- [ ] **P12-003** Add real garage selection/ownership, prices, upgrade tiers, stat comparisons, repair costs, and cosmetic/loadout unlocks. The current garage is a start menu and credits cannot be spent.

- [ ] **P12-004** Define typed RON/TOML/JSON bike/rider/weapon/track/loadout/economy data; validate on load without recompiling tunables.

- [ ] **P12-005** Extend saves to owned bikes, finances, class/track records, unlocks, and options with explicit schema versions, migration fixtures, checksums, and rotating backups.

- [ ] **P12-006** Test interrupted writes, crash/quit/restart double-credit prevention, and save/load equality over 100 write cycles; keep absolute asset paths out of saves.

- [ ] **P12-007** Build an economy/progression simulator and play through starter -> upgraded bike without softlocks; keep offline/online trust models separate.

- [ ] **P12-008** **Exit gate:** purchasing a better bike changes play, saves/migrations are reliable, and repeated races have a working reward loop.

- [ ] **P07-007** Add penalties, practice/free-ride, best records per class/track, and data-driven race length/AI/traffic/police/difficulty settings.

### Part 13 — Authoritative online races and lobbies

- [ ] **ONLINE-PLAY-01** Implement player-hosted races with Host/Join codes, public/private custom lobbies and map selection, and Quick Play joining an available measured RTT <=250ms lobby or becoming host otherwise. Reason: explicit user request for real multiplayer across different internet connections without operating a separate lobby server; part: 13; dependencies: fixed input/authority separation, discovery and host reachability; acceptance: 15-second Quick Play countdown, randomized grid, bots replaced by joining humans, synchronized movement/combat/foot/recovery/results and honest disconnect/error handling; verification: domain/protocol tests, multi-process native races, separate-network discovery/connectivity and latency tests. In progress; LAN discovery alone does not satisfy internet acceptance.

- [ ] **P13-001** Add join/lobby/ready/bike selection, synchronized starts, disconnect handling, reconnect policy, and consistent results.

- [ ] **P13-002** Define unreliable-sequenced redundant input batches and delta snapshots; use reliable/bounded or ID-deduplicated combat/crash/pickup/bust/finish events and idempotent rewards.

- [ ] **P13-003** Validate input magnitude/rate, selected bike, weapon ownership, reach/cooldown, damage, transforms, checkpoint order, and foot-or-bike finish on the server.

- [ ] **P13-004** Implement interest culling, interpolation buffers, prediction smoothing, replication priority, CPU/egress/RTT telemetry, and lag-compensated melee history.

- [ ] **P13-005** Replicate rider-foot and abandoned-bike states separately; keep particles/cosmetic ragdolls separate from authoritative crash/recovery state.

- [ ] **P13-006** Add authenticated identity, abuse/rate limits, instance IDs, heartbeat/logs, replay/incident dumps, and a 30-60 s reconnect/spectator policy without retroactive placement.

- [ ] **P13-007** Validate four-player races at 150 ms RTT / 1% loss with identical results and no duplicated finish/prize or bypassed attack limits.

- [ ] **P13-008** Measure eight-player stress before enabling it: 60 Hz tick budget 16.67 ms, target server P95 below 8 ms, and bounded bandwidth/corrections.

- [ ] **P13-009** **Exit gate:** stable private four-player races; eight players remain a stretch goal until measured headroom exists.

- [ ] **FOOT-009** Disconnect while on foot -> reconnect/AI timeout preserves authoritative rider/bike state and results.

### Part 14 — Content expansion and modes

- [ ] **P14-001** Add at least three further distinct original tracks spanning city, farm/livestock, coast/cliffs, and optional mountain woodland with unique hazards and passing opportunities.

- [ ] **P14-002** Build lightweight/balanced/touring/superbike/specialty classes, rider archetypes, cosmetics, loadouts, weapon pickups, and police variants.

- [ ] **P14-003** Add classic sprint, career/tournament, time trial, private multiplayer, and optional combat arena modes.

- [ ] **P14-004** Provide accessible difficulty controls and progression/unlock tests; only surface intentional AI assists explicitly.

- [ ] **P14-005** Use one versioned, validated content format across player/AI/network; separate cosmetics from physics and load new bikes/tracks without core-code edits.

- [ ] **P14-006** Add track/spawn editing tools when multiple contributors need them; verify packaged content hashes.

- [ ] **P14-007** Balance classes with 1,000 automated races plus human playtests; prevent career softlocks and regressions against the original slice gate.

- [ ] **P14-008** **Exit gate:** several unique tracks/bikes work through the same data pipeline without breaking the quality gate.

- [ ] **AI-03** Extend original AI parity to all career levels, full three-axis relationships/events, seeded career speed jitter and rubber-band tables. Reason: normalized current-level rules do not prove full original behavior; part: future AI/career; dependencies: AI-02 and career/difficulty authoring; acceptance: independently implemented per-level scenarios and documented unit/random calibration; verification: pinned-reference scenario comparisons and human difficulty tests.

### Part 15 — Presentation, audio, UI and accessibility

- [ ] **RFU-FX-01** Add localized Hills western-town/desert heat shimmer. Reason: requested biome identity; part:15; dependencies: Hills world content, RFU-PERF-06 and Bevy migration decision; acceptance: bounded effect near hot ground without distorting UI/traffic silhouettes or visibility; verification: native town/desert/other-biome comparisons and supported GPU/frame measures. Partial: original ground-heat band approximation implemented; native visual/refraction/GPU review remains open. [Issue](https://github.com/ImadMashhood/RoadDash/issues/42).

- [ ] **RFU-FX-02** Add deterministic wind-driven blowing sand in Hills desert and western approaches. Reason: requested environmental movement; part:15; dependencies: RFU-FX-01 and authored sand surface; acceptance: readable bounded gusts, appropriate quality control, no physics/material classification change or late mesh spawn; verification: native temporal captures, source/rights and quality/performance receipts. Partial: terrain-seated feathered dust pool implemented; centered-chase visual and performance review remain open. [Issue](https://github.com/ImadMashhood/RoadDash/issues/42).

- [ ] **RFU-FX-03** Add map-appropriate city shader/effects such as subtle vent steam, depth haze or a wet-road variation. Reason: requested city visual character; part:15; dependencies: RFU-PERF-06, RFU-SKY-01 and MAT-03; acceptance: coherent Metropolis/legacy-city presentation with restrained legibility, matching near/far material identity and opt-in quality budgets; verification: native views, source hash/provenance, GPU/frame comparison and visual review. Partial: bounded curb-steam pool implemented; native/GPU review remains open. [Issue](https://github.com/ImadMashhood/RoadDash/issues/42).

- [ ] **RFU-FX-04** Add a restrained original effect to Valley so every existing map has distinct air movement. Reason: issue #42 requests visual effects on every map, including Valley; part:15; dependencies: RFU-FX-01–03 and Valley route/content; acceptance: quality-controlled, off-road air motes remain legible and presentation-only across the full route; verification: native Valley before/after and temporal views, original source audit and CPU/GPU frame comparison. Partial: fixed-budget motes are implemented; native visual and performance review remain open. [Issue](https://github.com/ImadMashhood/RoadDash/issues/42).

- [ ] **UI-COURSE-01** Select Metropolis and Hills from the native menu. Reason: user needs level access without command-line flags; part:15; dependencies:UI-02 and HILLS-01; acceptance: two prominent course choices, preserved legacy maps, keyboard/controller/mouse navigation, selected course launches/retries correctly and preserves profiles/settings, online map changes not overridden by inherited CLI flags; verification: frontend/state/launch tests and native selection-to-race walkthrough.

- [ ] **SHADOW-RANGE-01** Extend readable city shadow coverage. Reason: user requests shadows farther through the city; part:15; dependencies:P15-004 and existing facade lifecycle; acceptance: broader bounded cascade coverage with preserved close contacts, no collision or facade-LOD regression, measured cost and a baseline comparison; verification: cascade policy tests, native city near/far views and matched timing.

- [ ] **UI-RETRO-BG-01** Clean the entry/lobby UI and use the user-provided original-game background pictures locally. Reason: explicit user art/UI direction; part: 15; dependencies: UI-02, ONLINE-PLAY-01 and the external reference kit; acceptance: readable keyboard/controller/mouse flows, correctly fitted original backgrounds with project-art fallback, functioning multiplayer actions and no retail image redistribution; verification: native screen captures and actual host/join/lobby navigation. In progress.

- [ ] **UI-02** Create/approve the replacement UI art and integrate the approved skeleton into Bevy. Partial: native entry flow, current-course launch, street visits and settings/results hooks now exist (ADR091); the approved sixteen-portrait Biker Bar browser has a 4×4 picker and full-page 4:3 biographies with retro typography (ADR094/095; docs/ui/APPROVED_PORTRAITS.md), with native visual acceptance pending; ADR102 source-table styling was rejected; ADR103 now uses full-window art/clean typography/soft gradients ([polish evidence](testing/modern-ui-polish/README.md)); grouped functional settings now integrate via ADR104 ([settings evidence](testing/settings-menu/README.md)); broader required art/HUD/pause/accessibility and hardware acceptance remain open. Reason: UI-01 is a reference/authoring kit, not production UI; part: 15; dependencies: UI-01, artist-approved required master families/components and existing lifecycle/settings/profile contracts; acceptance: original cleared art with correct slot identity, scalable typography/widgets, real garage/HUD/pause/busted/results and setting/input hooks, inactive career/network features clearly deferred, retained gameplay/saves/recovery, readable focus/accessibility and viewport behavior; verification: art review against the asset checklist, native keyboard/controller flows, saved-setting/error/retry cases and 1080p/1440p/4K/wide screenshots.

- [ ] **UI-05** Review/refine generated screen art into production masters and reusable layers. Reason: UI-04 outputs native ~1672×941 full-screen drafts rather than requested 3840×2160 layered masters or every source-resource variant; part: 15; dependencies: UI-04 and user art feedback; acceptance: approved model/outfit/bike identity, anatomy/composition, clean alpha edges, adequate 1080p/1440p/4K texel density, separate required portraits/bikes/frames and deliberate mapping to needed source variants/components without claiming a resize adds detail; verification: [28 screen review rows](ui/SCREEN_ART_REVIEW.md), master dimensions/layers/edges, small/large viewport comparisons and human art signoff before UI-02 native integration.

- [ ] **P15-003** Implement the licensed free-audio catalog, actual RPM/load-driven engine-band crossfades, spatial emitters/listener, surface sounds, location ambience, music buses, and audio QA below.

- [ ] **P15-004** Add scalable lighting/shadows, sensible optional SSAO/SSR/reflections, decals, anti-aliasing controls, and optional motion blur; do not require grain/blur/heavy shake. [Pass05 bounded contact-bias/cascade evidence](testing/polish-12/pass05/README.md); presets, temporal/hardware and broad performance acceptance remain open.

- [ ] **P15-005** Evaluate optional Hanabi particles for dust/exhaust/skids/sparks and cosmetic debris; cap emitters, GPU cost, and overdraw per quality setting.

- [ ] **P15-006** Polish garage/race/results UI, subtitles/non-speech captions, colorblind affordances, persistent input/audio/comfort settings, and rebinding.

- [ ] **P15-007** Add quality presets; test HUD at 1080p/1440p/4K and wide/ultrawide displays; never rely on color alone for critical state.

- [ ] **P15-008** Measure stable 1080p medium 60 FPS with four players/busy traffic and the aspirational 1080p high 120 FPS RTX 3080 target; add replay/highlight tools only if performance permits.

- [ ] **P15-009** **Exit gate:** presentation and audio are trailer-ready while meeting performance, legibility, accessibility, provenance, and saved-setting checks.

- [ ] **AUDIO-001** Create `content/audio_manifest.ron` / the asset ledger: path, author, exact source URL/license copy, acquisition date, hash, edits, redistribution permissions, attribution, and approval timestamp.

- [ ] **AUDIO-002** Motorbike inventory (~10 clips): idle, two mid bands, high band, rev hit, shifts up/down, two pass-bys, starter.

- [ ] **AUDIO-003** Surface inventory (~6): asphalt, gravel, dirt, grass, skid, wind; drive from wheel slip and `SurfaceType`.

- [ ] **AUDIO-004** Combat inventory (~5): light/heavy punch, kick, weapon, grunt; choose 3-5 licensed variants where appropriate from validated hit events.

- [ ] **AUDIO-005** Crash inventory (~5): body fall, metal hit, scrape, fence break, glass/crumple; layer by contact severity once per confirmed event.

- [ ] **AUDIO-006** World inventory (~6): city bed, farm insects/wind, cow, tractor, cliff wind, coast/surf; tie localized sources and ambience blends to zones.

- [ ] **AUDIO-007** Police/UI inventory (~4): near/far sirens, checkpoint, menu/finish cues; keep audible threat/readability priorities.

- [ ] **AUDIO-008** Implement an `AudioService` boundary with 3-4 seamless engine RPM/load bands, envelope crossfades, actual gear/transmission transients, and modest pitch changes.

- [ ] **AUDIO-009** Put the listener at the chase camera and emitters on bikes, police, cattle, and landmarks; test stereo/rolloff, opponent minimum volume, and distinct distance/gear cues.

- [ ] **AUDIO-010** Blend town/farm/cliff/coast ambience; visible livestock/tractors must match their sounds; keep music on a separate licensed, duckable, streamer-safe bus.

- [ ] **AUDIO-011** Evaluate Kira only if needed; if chosen, avoid running conflicting default Bevy audio and Kira backends.

- [ ] **AUDIO-012** Validate no loop clicks/gaps/clipping from idle to redline, distinguishable location ambiences, and no doubled combat/crash sound after prediction correction.

- [ ] **AUDIO-013** Persist master/engine/SFX/ambience/music sliders and input/audio options; caption critical non-speech cues and validate source release permissions. Partial: ADR104 persists master volume/mute with absolute live loop/one-shot gain and existing bindings/comfort; independent buses, captions and full audible/hardware acceptance remain open.

### Part 16 — QA, performance, platform, dependency and release gates

- [ ] **RFU-PERF-06** Compare four independent performance implementations on identical Hills/Metropolis baselines. Reason: user requests four fresh agents and measured selection; part:16; dependencies: RFU-PERF-02 and content integration; acceptance: same source/assets/seed/settings/hardware, clean repeated cold/warm runs, raw p50/p95/p99/max, readiness, memory and supported GPU samples, predeclared score and visual/correctness veto; verification: frozen binary provenance, contaminant-free receipts and four candidate reports. Four warm drafts are measured; final-source, full cold and combined-content comparisons remain open. [Evidence](testing/issue40-selection/REPORT.md), [issue](https://github.com/ImadMashhood/RoadDash/issues/40).

- [ ] **RFU-PERF-07** Integrate only the strongest verified performance candidate. Reason: user requests the winning gain; part:16; dependencies: RFU-PERF-06; acceptance: winner rerun on combined content, no repeatable >5% p95 regression on either map, preserved gameplay/collision/visuals and documented losers; verification: required Rust checks/audit, both-map native surveys/recovery and matched before/after profiles. No winner is merged; the nominal warm lead is within variation and integrated retest remains open. [Evidence](testing/issue40-selection/REPORT.md), [issue](https://github.com/ImadMashhood/RoadDash/issues/40).

- [ ] **RFU-PERF-02** Profile runtime CPU/GPU, raw frame-time distributions, stalls and memory on both courses. Reason: issue25 reports poor runtime; part:16; dependencies: PERF-07 and RFU-PERF-01; acceptance: populated areas, live traffic/combat and route traversal with p50/p95/p99/max, supported GPU timestamps, physics/AI/render attribution and measured improvements; verification: repeated release runs with pinned seed/settings/hardware, raw logs, recovery and full-route controls. Existing60FPS and broader hardware gates remain open.

- [ ] **RFU-PERF-03** Evaluate baked/hybrid stationary lighting. Reason: issue25 proposes baking as a candidate; part:16; dependencies: RFU-PERF-02 and MAT-03; acceptance: compare shadow cost, lightmap memory/load, visual quality and procedural-content compatibility before migration; verification: primary API/source review, measured GPU passes and documented tradeoffs, with no assumed FPS benefit or unapproved content replacement.

- [ ] **RFU-PERF-05** Establish explicit content budgets and quality/density controls. Reason: issue25 requires headroom before clouds/birds/vegetation/props/herds grow; part:16 coordinating05/06; dependencies: RFU-PERF-02 and content-family follow-ups; acceptance: bounded counts/draws/memory/work, appropriate presentation-only quality controls and preserved authoritative populations/collision; verification: budget tests, native census and matched per-quality profiles.

- [ ] **BASE-001** Verify the proposed dependency combination with locked Fedora/Linux and Windows builds; record the selected versions and migration decision.

- [ ] **BASE-002** Preserve a working baseline while migrating; keep engine, physics, input, audio, and networking adapters independently testable.

- [ ] **P11-007** Give every scenery family LOD, collision, and audio metadata; reveal recognizable landmarks or encounters every 10-25 s without hidden active hazards/pop-in.

- [ ] **P16-003** Profile CPU/GPU/render stages, draw calls, animation, contacts, networking, server ticks, and repeatable 1080p/1440p frame-time percentiles.

- [ ] **P16-004** Add golden state/replay checks, reproducible crash reports, a bug template, automated 30-minute eight-bot soak, and high-latency/loss multiplayer soak.

- [ ] **P16-005** Verify clean Fedora KDE Wayland/NVIDIA and Windows NVIDIA/AMD installations, controller/reconnect matrix, ultrawide HUD, save migration, and network ports.

- [ ] **P16-006** Add dependency/content/license audit and compatible `cargo audit`/`deny` checks to CI; generate current SBOM/attributions and resolve all P0/P1 bugs.

- [ ] **P16-007** Review localization, accessibility, opt-in telemetry/crash reporting, privacy/moderation policy, source-asset permissions, and final public-title/trademark clearance.

- [ ] **P16-008** Package/install-test distributions; add Steam depots/achievements/cloud saves only if selected; rehearse hotfix/support and stage closed test -> demo -> launch.

- [ ] **P16-009** Verify beta crash-free sessions above 99%, 60 FPS preset P95 frame time <=16.7 ms, and server P95 tick <=8 ms on recorded target hardware.

- [ ] **P16-010** **Exit gate:** functional, platform, performance, stability, rights, clean-install, support, and release-signoff evidence is complete; a successful build alone is not release approval.

- [ ] **ENV-006** E5: stream/LOD by visibility/distance without unloading or hiding authoritative hazards.

- [ ] **NFR-001** Store repeatable benchmark scenarios and results against every mandatory row above; distinguish requirements from aspirational targets and record intentional scope cuts.

- [ ] **NFR-002** Validate adaptive scenery budgets: modular building boxes/low LOD, near-corridor tree/rock proxies, lightweight distant animals, stable cliff collision, and batched road furniture.

- [ ] **RIGHTS-001** Obtain written commercial-use permission for acquired assets/contributions and proof for soundtrack, voices, logos, and any real motorcycle brands.

- [ ] **RIGHTS-002** Review the original-successor title/marketing and any close reproduction of protected expression; assume no permission for EA branding, exact tracks/characters/art, original retail files, or extracted media.

- [ ] **RIGHTS-003** Generate attribution/SBOM notices from the actual lockfile and asset manifests; audit installers, screenshots, trailers, and website material.

- [ ] **RIGHTS-004** Archive selected source/license versions for R01-R09 and the audio/art catalogs; research-only code stays research-only unless permission is established.

- [ ] **RIGHTS-005** After each part, attach a tagged build, test results, recording, performance metrics, architectural decision, explicit remaining bugs, and source/rights notes.

- [ ] **ART-05** Author distance LODs and validate the detailed rider/bike budget on more hardware. Reason: LOD0 replaces cheap primitives across 15 racers; part: 11 optimization; dependencies: ART-03 and profiler; acceptance: readable silhouettes, stable collision-independent swaps and measured frame budget; verification: native distance/performance surveys and Windows hardware review.

- [ ] **P08-007** Budget active racers/cars/police by visibility and distance; perception 5-15 Hz, controls 60 Hz, safe spawn/despawn outside view.
