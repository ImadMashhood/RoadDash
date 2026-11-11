# Issue #43 — Metropolis city-life authoring

Branch `codex/issue-43-city-life` from `main` at `e2a31918`, with scope records
cherry-picked from `36246757` as `cf8f4ad9`. No external media, reference code,
runtime crate or save-format change. The city candidate was subsequently
integrated with main through guardrail merge `9932b53b`. Performance and
human full-course acceptance remain open.

## Baked changes

`tools/bake_metropolis.py` uses the approved `LAYOUT.json` and the revised
`living_city.json` config. The city sidewalk area is 1,459,930.200m² versus
1,167,944.103m² before landward widening (ratio 1.25000006). The computed
landward offset is 0.873978m, versus 0.688827m at the previous 1.20 target.
The 12m primary carriageway and 4/3/4 race lanes remain in the existing source;
all 19,108 building identities and 122 parked cars remain. The baker's polygon
audit checks no building/walk or carriageway/walk overlap and a complete
physical footprint on pavement for every curb prop.
An independent parsed comparison against `main` confirms exact equality of
track, both paths, fields, streets, junctions, bounds, traffic paths, street
life, parked cars and source-layout hash. Every building retains its non-shape
fields (ID, height, floors, tier and style); only the landward frontage shape
is clipped by the new pavement.

| Placed census | Previous | Candidate |
| --- | ---: | ---: |
| Urban pedestrians | 384 | 480 |
| Clear sidewalk-to-sidewalk crossing routes | 10 | 13 |
| Alternate-branch pedestrians | 0 | 96 |
| Alternate-branch crossing routes | 0 | 3 |
| Street props | 1,521 | 1,552 |
| Broadleaf street trees | 211 | 257 |
| Distinct placed original prop model IDs | 32 | 79 |
| Main cars | 144 | 144 |
| Parked cars | 122 | 122 |

Pedestrian patrol authoring checks a full capsule corridor and spaces accepted
origins by at least 8m. The former source-order allocation filled all 480
places on the long route. The revised baker reserves 96 clear places for the
short branch, leaving 384 on the main route. The crossing baker checks both sidewalk endpoints, a
capsule-wide supported road/sidewalk corridor, prop avoidance, at least 6m of
road traversal and 180m route separation. Fixed-step host admission retains a
two-active cap and predictive traffic gaps; release spacing changes from 12s to
9s. These counts do not prove crossings completed during native play.

Main-avenue tree frequency increases within the checked curb band. Secondary
streets now use source-route district pools of existing original props. The
same owner collider, destruction, navigation obstacle and 32/95/500m prop LOD
path handles all placements. Metropolis now selects catalog models with the
full stable placement index instead of folding every seed into four values.
The catalog has 51 original broadleaf models; placed model IDs rise from 32 to
79 across the same 13 semantic roles. Tree destruction uses the existing topple
profile, so no new damaged mesh or tuning was introduced. No street model,
facade material or far atlas was altered. A 1.25-area bake initially exposed a 2.88e-7m² boundary triangle
with zero exterior area; `triangulate` now checks exterior area below 1e-6m²
and retains total-area conservation.

## Verification and remaining gates

- Full final bake succeeded: 19,108 buildings, 36,619 surface patches, 11,219,624
  manifest bytes. Generated SHA-256: `3b7e9485f7317b95ef4b24793cef44de14cf419fb8944455655ece9046775150`.
- Nine `test_metropolis_population` and `test_metropolis_streetscape` authoring
  tests passed, including a two-path allocation fixture. `sync_progress.py
  --check` passed; README and PROGRESS keep all four RFU-CITY IDs open.
- On the guardrail-integrated tree, `cargo fmt --all --check`, strict locked
  workspace Clippy, `cargo test --workspace --locked` (623 passed, two ignored,
  zero failed), `cargo build --locked`, `python3 tools/audit.py` (580 dependency
  declarations, 199 runtime asset records, 28 reference observations), and
  the nine Python authoring tests all passed. The physical crossing test now
  exercises both routes.
- The exact-tree native `--metro-survey` exited zero using debug binary SHA-256
  `ff04d63b880afba5d86829738df1bf1d1e41d97044a37fd7c4ef23c99b4cf33d`:
  480 walkers spawned, 464 moved over 0.5m, four crossing starts and four
  completions, with two completed on each route (matched to baked crossing
  origins within 0.0001m), maximum two active, 144 main cars and 1,552 street
  props. All 14 captures were nonblank and no error lines appeared. The
  alternate-branch riding-height view at station 4100 recorded 14 walkers
  within 80m, all moving; main-route view four recorded 21 within 80m.
  [Alternate branch](evidence/alternate-branch.png),
  [main avenue](evidence/main-street.png) and
  [curb props and trees](evidence/curb-props-trees.png) show the live result.
  The sampled alternate view has visible people on both sidewalks and some
  trees, lights and furniture; trees still look sparse in this view.
  [Crossing counters](evidence/pedestrian-crossings.json),
  [walker census](evidence/metro-pedestrians.json) and
  [survey receipt](evidence/survey-receipt.json) preserve the measured values.
- The same binary's `--smoke-test` recovery exited zero with 21 nonblank
  captures, unchanged frozen inputs and no error lines
  ([receipt](evidence/recovery-receipt.json)). This is a scripted smoke, not
  a human full-course review.
- The debug native wall sample averaged 27.429ms and had 31.056ms p95, above
  the 16.7ms target. It has no matched baseline and no GPU timestamp. A valid
  cost comparison should run baseline and candidate release binaries with the
  same renderer, resolution, settings, scripted camera path and quiet host;
  compare median/p95 wall time, fixed/physics time, entities and draw budgets
  across repeated paired runs. [Sample](evidence/metro-performance.json).
  RFU-CITY-01..04 and RFU-PERF-06 stay open for this comparison, broader
  visual density, collision/human traversal and hardware review.

## Development history

1. Inspected the baseline: 384 walkers, 10 crossing routes, 1,521 props and
   211 street trees. Identified the existing capped baker, content schema,
   host fixed-step crossing admission and original prop LOD path.
2. Tried a 1.25 sidewalk-area target. The first bake stopped on frontage
   `LC-S0085-F000-0` because Shapely rejected a microscopic triangle's
   representative point even though the triangle had zero area outside the
   clipped building. A 1.21 probe baked, then the full 1.25 target succeeded
   after the area-based numerical check.
3. Rebalanced source-route curb roles, raised the bounded city population and
   rebaked; actual counts above replaced budget-only assumptions.
4. Compared placed semantic kinds and catalog model IDs. Kinds stayed at 13,
   so the Metropolis model seed now uses the complete stable index and places
   79 project-original models instead of 32 without widening collision bounds.
5. The first native survey exposed zero walkers on route 1. The baker had
   exhausted its budget from route-0 candidates before considering route 1.
   Reserving a 96-person branch allocation and authoring crossing separation
   per route produced 96 branch walkers and three branch crossing corridors.
   First-candidate native timing is retained only as an unpaired control;
   final route-balanced native evidence was then obtained. The first route-1
   camera at station 4300 landed in an empty local gap; shifting the survey-only
   camera to station 4100 exposed 14 nearby moving walkers at riding height.
6. Integrated the guardrail mainline, reran all strict checks and the exact
   binary's native survey/recovery. No city authored IDs or approved road lanes
   changed during integration.

## Superseded first-candidate native control

The initial 480-route-0 bake ran a functional survey, a quiet repeat, and a
Metropolis recovery smoke. Both survey runs exited cleanly with 14 nonblank
captures, 461 walkers moving over 0.5m, three crossing starts and two completed
crossings; the recovery smoke exited cleanly with 21 nonblank captures. The
first survey overlapped a brief root worktree operation during startup and is
functional evidence only. The quiet repeat measured 27.237ms average and
31.814ms p95 wall frame time in a debug build, above the 16.7ms target. It is
an unpaired sample and is superseded for density/branch review by the final
route-balanced manifest. It did expose the route-1 absence that prompted the
authoring correction.
