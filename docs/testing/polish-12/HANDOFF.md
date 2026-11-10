# Sequential gameplay and graphics polish — 2026-11-09

Starting commit: b52ab5e (exact full SHA below). Branch: codex/gameplay-graphics-polish.
Worktree: /home/imad/Documents/ChatGPT/RoadDash-polish. Original repository and uncommitted UI work remain untouched. No push/default-branch merge authorized.

Exactly twelve fresh-context implementation agents, sequentially; parent review before each integration. User request: REQUEST.txt. Preserve accepted physics/content; human, hardware and art gates stay open.

| Pass | Scope | Status | Commit | Evidence / gates |
|---|---|---|---|---|
| 01 | Performance and visual stability | Integrated | 554956de7d02990dc02ffe19050d495e716581b3 | pass01/README.md; GPU/full-route/human gates open |
| 02 | Low-speed contacts and recovery | Integrated | 3c88ba368dbc50ec9d12e4274636666476285f4a | pass02/README.md; human/confined/hardware gates open |
| 03 | Elevation and terrain integration | Integrated | b26d7f59c34bf91f9362c4a1d9e5ccb247424b85 | pass03/README.md; performance/human/full-route gates open |
| 04 | Unified lighting and exposure | Integrated | e87ed2be8b6594d07ae7f923cd151698d391baee | pass04/README.md; art/performance/hardware open |
| 05 | Shadows and ground contact | Integrated | 9f89e6005805c213d228a50966ad6ca30466a745 | pass05/README.md; temporal/human/hardware open |
| 06 | Roads, curbs and sidewalks | Integrated | 3bdd9fb668718501cec6fd5e32cceb0dc7c77cf7 | pass06/README.md; modeled drainage/art/performance open |
| 07 | Building material consistency | Integrated | 669cb094f8d1930b81f783c350275f65bbeabac1 | pass07/README.md; art/temporal/performance open |
| 08 | Farm-to-city transition and district coherence | Integrated | 75ccf0dc270d1f7166b1e50b3c77cdc0a61f378f | pass08/README.md; human/art/streaming/performance open |
| 09 | Atmospheric depth | Integrated | 0a23465c406eeb8096ad2341a6de0efd165838fa | pass09/README.md; horizon seam to10, performance/human open |
| 10 | Skybox and clouds | Integrated | 7263342d182b1f2b7bf274e74b7c0000af1475d0 | pass10/README.md; independent cloud animation/art/performance open |
| 11 | Character movement and visual continuity | Integrated | 1ebf2b11dc2a60a706f8a8b9294c40dcad201850 | pass11/README.md; NPC seams/gait/art/hardware open |
| 12 | Punch/kick reactions and impact feedback | Integrated | ef7a4826ffa6de94ca49dab3f39c42a50a6be951 | pass12/README.md; human feel/art/hardware/performance open |

## Handoffs

Initial: Bevy 0.17.3, Rapier adapter 0.32.0 / Rapier 0.31.0, wgpu 26.0.1. Existing evidence points to physics catch-up and streaming spikes, not measured GPU timings. No active game/compiler process observed at startup.
b52ab5e11f4dc21815c1b8a64fc95558cde419af

### Pass01 reviewed and integrated
Retained-only facade priority and 2m plane hysteresis; parent inspected diff/tests and downtown native image. 415 tests, fmt/clippy/build/audit, paired surveys/recovery/NPC and 18-case labs passed. Controlled survey 16.69→16.43ms is not an established speedup. ADR105, no new IDs/closures. Release cache holds baseline comparison; rebuild before subsequent native release runs. Full handoff: pass01/HANDOFF.md.

### Pass02 reviewed and integrated
Same-time pre-solver downed target velocity corrects false/missed ride-over lift. Parent reviewed runtime diff/regression receipts/remount capture. 416 tests and mandatory checks pass; both native rigs recovery/reverse/NPC and 18-case lab pass. ADR106; no new IDs/closures. Release rebuilt to current implementation.

### Pass03 reviewed and integrated
Shared smooth authored heights, tiled apron/fields, traffic sampling and preserved planar layout. Parent reviewed code, coverage fixes, profile and city entry image.419tests/mandatory+20authoring checks pass; metro/recovery/NPC/lab pass. Valley survey geometry passes but exit1 for19.208ms p95. Metro isolated p9538.934ms; before timing contaminated by authoring, no causal comparison. ADR107; ENV-001 remains partial.

### Pass04 reviewed and integrated
Validated shared warm34.7degree sun/cool fill/EV12.7/Reinhard profile. Parent reviewed adapters/tests and matched city-entry image.421tests+mandatory/native paired recovery/NPC/lab pass. Geometry unchanged; avg19.313→19.664ms,p9539.287→39.191ms no speedup claim. ADR108; sky-disc drift reserved for10; no new IDs/closures.

### Pass05 reviewed and integrated
Normal bias1.8→1.5,overlap20→25%,retained maps/range/casters. Rejected1.0 field-pattern candidate archived. Parent reviewed diff/final remount.421tests/mandatory/native survey+recovery pass. avg19.459→19.073,p9537.082→36.298ms no causal GPU/speedup claim. ADR109, no new IDs/closures.

### Pass06 reviewed and integrated
Metropolis asphalt PBR/world-phaseUVs,opaque faded markings,paving seam dirt; no geometry/collision change. Removed unused duplicate texture allocation after parent review. Dark candidate rejected. Parent reviewed final street view.424tests+mandatory pass; metro/recovery pass; Valley geometry passes exit1p9519.938ms. Metro avg19.606→19.972,p9533.708→37.740ms, no causal GPU claim. ADR110/P11-004 partial,no IDs closed.

### Pass07 reviewed and integrated
24families source finishes;144GLB geometry fingerprints preserved;383far bands/15mips rebaked with effective runtime roughness. Metro packed material crop fixes constant far roughness/metallic,extra6285312bytes no draws. Parent reviewed source/shader/parity/native09.426tests+mandatory+7authoring groups/native metro/recovery/both LOD pass. Intermediate old-binary/new-shader failure retained/resolved. avg19.925→20.062,p9539.788→41.238ms no performance claim. ADR111/MAT-03 partial.

### Pass08 reviewed and integrated
1338low-rise approach style assignments; parent review bounded changes to2–5floors,z<1800,nonstyle world/layout/atlas identical. Reviewed entry03 capture.427tests mandatory/native metro+recovery pass. Entry detail2708→4862meshes,1495690→1298748triangles within caps. avg19.029→19.288,p9536.966→34.572ms not causal speedup. ADR112,no IDs/closures.

### Pass09 reviewed and integrated
Gradual exponential-squared haze,>99%transmission100m,shared exposed horizon radiance; retained more far geometry. Parent verified installed exposure ordering and native aerial. HDR trial black despite no shader errors rejected; nonHDR sky tonemap mismatch passed to10.430tests+mandatory/native survey/recovery pass. avg18.984→18.236,p9535.355→32.766ms no causal claim. ADR113,no IDs/closures.

### Pass10 reviewed and integrated
Separate display-tonemapped procedural cube resolves nonHDR horizon and mirrored/drifting sun; shaded clouds stationary,lighting maps preserved. Parent reviewed tone math/provenance and native farm00/aerial10.432tests+mandatory/native survey/recovery pass. Extra2097144bytes,120.588ms startup; avg18.238→18.493,p9532.108→33.548ms no causal claim. ADR114,independent cloud motion open,no IDs/closures.

### Pass11 reviewed and integrated
Two visual ankle pivots now authored0.5580462m; source mesh/weights/neutral wrist/physical rigs preserved. Sole drift69.278mm→0.000603mm; physical differential zero. Parent reviewed code/metrics/running image.434tests mandatory/native paired rigs recovery/NPC+18case lab pass;38matched pose views each (first normal/front omitted by fixture). ADR115; NPC source seams/hovering open,no IDs/closures.

### Pass12 reviewed and integrated
Confirmed contact replaces single bounded directional punch/kick recoil; attacks/stagger priority,IK contacts,physical rules preserved. Parent reviewed runtime/tests and before/peak reaction capture; rejected backward-fold grip reach failure corrected without relaxing20µm limit.439tests+mandatory/model zero-trajectory diffs+6native runs/18case lab pass.116nonblank images incl40reaction frames;116.52ns active-layer microbenchmark not whole-frame guarantee. ADR116,no IDs/closures. Exactly12fresh-context agents completed sequentially; final integrated regression follows without further agents/roadmap work.

### Final parent regression and stop
All mandatory checks,439 tests (one ignored), six authoring suites/atlas/profiles/roadmap, ten native cases and18-case lab pass.172 main-run captures plus14 repeat captures retained. First metro00 ground missing: rejected visually despite census/nonblank success; same-binary repeat and actual gameplay show ground. Cold-start capture readiness remains PERF-08. Final avg18.616/p9539.747ms; repeat17.696/33.901ms; target unmet. Final report: final/README.md. No newIDs/checklist closures;220verified/245open. Original checkout independently cleanmain2a3770e; no unrelated absorption/push/defaultmerge. Exactly12 agents; stopped.
