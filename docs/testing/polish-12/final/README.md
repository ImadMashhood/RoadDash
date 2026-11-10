# Integrated twelve-pass verification — 2026-11-09

Exactly twelve fresh-context implementation agents ran sequentially, with parent diff/evidence review and a separate Conventional Commit for every pass. No thirteenth implementation agent or next roadmap part was started. Integration branch: `codex/gameplay-graphics-polish`, worktree `/home/imad/Documents/ChatGPT/RoadDash-polish`; base `b52ab5e11f4dc21815c1b8a64fc95558cde419af`. Final runtime reviewed/tested at `51d3fd5` (implementation tip `ef7a4826ffa6de94ca49dab3f39c42a50a6be951`). Later final-report changes are documentation/evidence only.

## Implemented and reviewed

Every row has mandatory checks and native evidence in its linked pass record. Test counts are workspace tests at that pass, not new tests added. Human approval is not claimed.

| Pass | Implementation | Verification | Implementation commit | Remaining gates |
|---|---|---|---|---|
| [01](../pass01/README.md) | Retained facade selection and 2m plane hysteresis | 415 tests; paired surveys/recovery/NPC/lab | `554956de7d02990dc02ffe19050d495e716581b3` | Full route/GPU/performance |
| [02](../pass02/README.md) | Same-time pre-contact downed ride-over eligibility | 416 tests; both rigs/reverse/remount/lab | `3c88ba368dbc50ec9d12e4274636666476285f4a` | Confined recovery/human feel |
| [03](../pass03/README.md) | Shared authored smooth route elevation, tiled ground | 419 tests; 20 authoring checks; branch/recovery surveys | `b26d7f59c34bf91f9362c4a1d9e5ccb247424b85` | Full driven route/performance |
| [04](../pass04/README.md) | Warm sun, cool fill, unified exposure | 421 tests; matched native views | `e87ed2be8b6594d07ae7f923cd151698d391baee` | Art/hardware |
| [05](../pass05/README.md) | Contact shadow bias/cascade overlap | 421 tests; native comparisons; rejected bias retained | `9f89e6005805c213d228a50966ad6ca30466a745` | Temporal stability/hardware |
| [06](../pass06/README.md) | Asphalt PBR, repairs, faded paint, paving seams | 424 tests; metro/recovery; Valley geometry | `3bdd9fb668718501cec6fd5e32cceb0dc7c77cf7` | Modeled drainage/art/performance |
| [07](../pass07/README.md) | Coordinated near finishes and material-aware far atlas | 426 tests; 144 GLB fingerprints; 383 bands/15 mips; both LOD modes | `669cb094f8d1930b81f783c350275f65bbeabac1` | Art/temporal/performance |
| [08](../pass08/README.md) | Low-rise business/workshop/neighborhood blend | 427 tests; 1,338 style-only assignments; survey/recovery | `75ccf0dc270d1f7166b1e50b3c77cdc0a61f378f` | Human district/streaming review |
| [09](../pass09/README.md) | Economical readable exponential haze | 430 tests; matched native views; black HDR trial rejected | `0a23465c406eeb8096ad2341a6de0efd165838fa` | Broad atmosphere/performance |
| [10](../pass10/README.md) | Tone-matched sky, correct fixed sun, layered clouds | 432 tests; horizon/sun captures | `7263342d182b1f2b7bf274e74b7c0000af1475d0` | Independent cloud motion/art |
| [11](../pass11/README.md) | Authored visual ankle pivots and paired inverse binds | 434 tests; zero physical trajectory differences; paired poses/NPC/recovery | `1ebf2b11dc2a60a706f8a8b9294c40dcad201850` | NPC seams/hovering/full gait |
| [12](../pass12/README.md) | Bounded directional confirmed-hit punch/kick recoil | 439 tests; contact/priority/reset tests; paired rigs/native reactions/lab | `ef7a4826ffa6de94ca49dab3f39c42a50a6be951` | Human impact feel/art/balance |

## Final independent regression

[Command receipts](checks.json): fmt, strict workspace/all-target clippy, workspace locked tests (**439 passed, one existing ignored**), locked debug and release builds, audit, five roadmap tests, progress synchronization, six Metropolis authoring suites, facade atlas and material profile verifiers all exited zero. Differential rendering/physics/recovery tests are included in the workspace run.

[Native integrated receipts](native-integrated-results.json): all ten serial runs exited zero: Metropolis survey; default and baseline recovery/reverse on separate branches; combat/club; isolated confirmed reactions; default and baseline NPC actors; City and Valley LOD surveys; frontend/settings. The paired headless ragdoll lab exited zero with **18 cases**. There were **172 captures**, zero blank-image checks and zero logged ERROR/panic lines. Executable and selected asset hashes remained frozen; runs used isolated saves/settings and process guards. Nonblank checking is not scene-completeness proof: parent inspection found the first metro farm capture missing ground, documented below. Other final images inspected include city03/city09, gameplay racing and reaction70 peak. Do not interpret automated success as every screenshot having been visually approved.

## Performance and limitations

Final bounded Metropolis wall-frame sample: **18.616 ms average, 39.747 ms p95**, target16.7ms **not met**. The survey simulates fifteen racers near the farm while the render camera ends downtown. It is not GPU timestamp attribution, a complete driven route, simultaneous-crash hardware qualification or Windows/controller acceptance. Census: 19,068 buildings, 3,067 chunks, 2,613,119 collision triangles, 2,909,113 render triangles, two physical routes; resident envelope/collision assertions passed.

Per-pass paired observations are retained, not aggregated into an invented overall speedup. Controlled pass01 median16.69→16.43ms did not establish causality. Pass03's original before timing was contaminated by authoring; replacement after p9538.934ms. Pass06 and07 p95 increased in their bounded comparisons (33.708→37.740ms and39.788→41.238ms). Pass10 adds2,097,144 bytes and measured120.588ms one-time sky generation; pass07 adds6,285,312 texture bytes. Pass12's116.52ns active-reaction microbenchmark is not whole-game frame cost. Earlier Valley world-survey p95 failures remain in pass03/06 evidence; final LOD-survey success does not supersede those performance gates.

Final metro00 contains riders/sky but no ground; later city views and the actual racing smoke frame render ground correctly. The survey uses a fixed frame-count capture without render-pipeline readiness gating. Treat first-view readiness as unresolved under PERF-08, retain the failed visual evidence, and do not use metro00 as proof of farm appearance. The same-binary repeat exited zero and rendered ground correctly in metro00 (parent inspected); its14 additional captures and receipts are retained in `metro-repeat/` and `native-repeat-results.json`. Repeat wall-frame average17.696ms/p9533.901ms still misses target. This is consistent with a readiness issue but does not establish its cause. Full temporal scene readiness needs further investigation; entity census alone cannot certify placed/rendered geometry.

## Files, decisions and research

[Changed files](changed-files.txt) inventories all source/generated/evidence files through the integrated runtime; [commits](commits.txt) records integration history. Principal changes cover `src/world_gen/metropolis`, lighting/sky/material/shadow rendering, `src/models`, combat/game adapters, pure `rr_core` rules, `rr_content` validation, authoring tools, authored world/facade data and matching atlases. Final documentation adds this folder and README/PROGRESS/history/handoff updates.

ADR105–116 in `docs/DECISIONS.md` explain choices and rejected alternatives. Repository investigations, asset provenance/licenses and known issues are recorded in existing corresponding docs and each pass. No proprietary media/reference code was imported, no UI artwork generated, no dependency migration. Sky analytic tone mapping attribution to Tomasz Stachowiak/Bevy (MIT/Apache) is recorded; this is provenance documentation, not blanket legal clearance.

README remains the live checklist: **220 verified /245 open /465 IDs**. **No new roadmap IDs, moves or closures**; scoped evidence extends existing requirements, including PERF-08, ENV-001, P11-004, MAT-03, PALETTE-01, SKY-01, ART-04 and COMBAT-FEEL-01/P05-006. Human art/feel, performance, hardware, Windows/controllers, drainage geometry, animated clouds and NPC seam/gait gates stay open. No milestone acceptance or new user approval is claimed.

## Review images and git

- [Actual farm gameplay](anatomy/screenshots/02-racing.png)
- [City street](metro/screenshots/metro-03.png) and [downtown](metro/screenshots/metro-09.png)
- [Confirmed punch peak](reactions/screenshots/reaction-70-04.png)
- [Rejected first survey view](metro/screenshots/metro-00.png)

Final evidence is committed on the dedicated integration branch; exact final HEAD is supplied in the user report. No push/default-branch merge was performed. Original checkout independently became clean `main` at `2a3770e10c33fe8d31d3125dca340d0a93799542`; those unrelated art/archive changes were not absorbed. Its recorded status/head are included here. Stop after pass12; future work requires a subsequent scoped request.
