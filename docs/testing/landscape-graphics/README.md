# Farm landscape follow-up — 2026-11-09

User-requested graphical follow-up to the twelve-pass polish, on
`codex/landscape-graphics`, based on `f4086433a98e0c92bed48846d922d7dbe2486842`.
Parent reviewed and committed on the focused branch; no push/default-branch merge.

## Implementation

Original opaque pasture/grain/fallow grass with deterministic clustered density,
independent size variation, narrow geometric blades and32m batches. Native Bevy
visibility dither fades grass65–110m; no added shadow casters or transparent cards.
Dense0.6m candidate spacing within12m of the farm road uses the budget reclaimed
from reducing far-offroad coverage to34m. Shared original64px mipmapped flecks
add21,844texture bytes. Soil ramps meet the actual6.8m pavement envelope and blend
to the exact underlying ground material/UV/tint. Continuous spatial field tint
and irregular soil shoulder ribbons remain beyond
the grass range. Counters include actual generated grass and visible triangles.
Opt-in `ROADDASH_LANDSCAPE_VIEWS=1 --metro-survey` places the first two cameras
beside the shoulder for close inspection. Normal survey camera poses are retained.

Full blade footprints exclude pavement/access surfaces, parcel holes, buildings,
barns and fence margins. Terrain triangles, physics, support, layout, envelopes,
source content, facade materials/atlases and both legacy maps are unchanged.
The static caps are120,000tufts and600batches; exceeding them fails explicitly.
No silent end-of-course truncation. Grass has no collider or gameplay authority.

## Evidence and review

`before/` is the frozen baseline native survey with isolated saves/settings;
metro00 and metro01 were directly inspected and both render actual ground.
`rejected-leaves/` retains the first after trial: visually rejected despite clean
native exit because reversed lane-side winding darkened soil and broad triangular
blades looked like folded leaves. Subsequent implementation reverses shoulder
winding and uses five narrower independently varied blades. Numeric tests also
require sky-facing shoulder normals.

`checks.json` / individual logs record mandatory checks; `after/` and recovery
folders record commands, binary/content hashes and isolated native runs. Blank
image detection alone is not landscape acceptance. Exact final results follow in
HANDOFF.md; the existing cold-start readiness concern is not declared fixed.

## Scope, provenance and open gates

[Provenance](PROVENANCE.md) records all five supplied reference images and three
pinned grass-repository investigations. No external source/media/assets or new
dependency imported. ADR117 explains the chosen bounded native implementation.
No new roadmap IDs or closures: existing V2-06-005/006/007 and WORLDART-01 remain
partial, with220 verified /245 open /465 IDs. User visual acceptance, full-route
streaming/temporal and GPU/Windows/hardware/performance gates remain open.
Wind, new trees/stone walls and complete terrain silhouettes are outside this pass.
Wall-frame samples are not GPU measurements or causal speedup claims; the previous
16.7ms p95 gate was unmet and must remain open unless measured otherwise.

The sky, tree catalog/placement and terrain silhouette are unchanged by this
follow-up. This is not a claim to match the complete reference game. Two
provisional road/shoulder runs and the initial dark-ribbon trial are retained
with only their two farm images; their receipts describe original capture counts.
An independent Shapely audit found58 nominal-width soil quads overlapping authored
pavement/access; full-polygon clipping now omits such quads. The initial failure
is retained in `shoulder-footprints-initial.json`; do not substitute nominal road
width or allocation counts for actual footprint checks.

## Final measured scope

| Measurement | Baseline | Final |
| --- | ---: | ---: |
| Resident buildings / chunks |19,068 /3,067 |19,068 /3,067 |
| Collision triangles / static batches |2,613,119 /2,531 |2,613,119 /2,531 |
| Environment render triangles |2,909,113 |3,208,310 |
| Added grass tufts / batches / triangles |0 |57,197 /333 /285,985 |
| Added clipped soil quads / triangles |0 |6,606 /13,212 |
| Survey final wall average / p95 |19.040 /36.350ms |17.685 /34.765ms |
| Early farm-window weighted wall time |7.524ms |7.309ms |
| Early-window maximum observed process RSS |3,711,852kB |3,860,916kB |

At farm survey views00/01, **7/11grass batches and12,560/14,630grass triangles**
are submitted while in the visibility range, including fading geometry. Later
city views show zero grass batches. The final native grass/soil generation takes
337.042ms once at startup in this run. Added geometry vertex/index payload is
approximately60.2MB before CPU/backend duplication, not measured GPU allocation;
shared fleck texture adds21,844bytes. No per-frame mesh generation/upload occurs.

[Comparison](comparison.json) retains raw one-second early-farm windows, camera
positions, meshes and process memory. These windows include warmup and may span
camera transitions; there is no isolated900m cost or causal speedup attribution.
Final p95 comes from a downtown window after grass fades and still exceeds16.7ms.
Process RSS variation includes scene/loading/runtime allocations, not solely
this feature. No GPU timestamps, Windows/hardware or full driven-route acceptance.

[Preservation](preservation.json) records exact unchanged physics/track/content/
asset/dependency inputs. The new regression compares all6,276actual field
triangles' collision positions and winding before/after material/UV application.
Native collision census and resident envelope assertions also agree. Independent
polygon intersection found58bad soil candidates; runtime full-polygon clipping
retains exactly the independently verified6,606zero-overlap quads, with no source
layout mutation. [Footprint audit](shoulder-footprints.json).

## Final verification

[Mandatory receipts](checks.json): fmt, strict workspace/all-target clippy,
**443workspace tests passed, one existing ignored**, locked debug/release builds,
audit, progress sync and five roadmap tests all exited0.

[Native receipts](native-results.json): final road and shoulder surveys, route0
anatomical recovery/reverse, route1 baseline recovery/reverse, CityLOD and ValleyLOD
all exited0. Both recoveries report0.0000m pickup error. The18-case paired ragdoll
lab and explicit Metropolis/Valley inspections also exited0. The no-argument
legacy `--inspect-world` reports CityRun (977buildings); explicit Valley input
files were separately inspected. **121final native captures, zero blank-image
checks and zero ERROR/panic lines**; hashes stayed frozen for each run.

Valley world survey exited1 **solely because p9518.597080ms exceeds16.7ms**.
It completed15sections,657building roots,224dressing placements and29captures;
its overlap budget held. `survey.benchmark_passed` is exactly the p95 comparison
in `src/world_gen/survey.rs`; geometry/readiness assertions did not fail. This
existing performance gate remains open; it is not reported as a passing test.

Direct visual inspection covered baseline farm00/01, final farm00/01, final low
shoulder01, actual recovery racing02 and city03. These show real ground and placed
grass; visual acceptance of every capture or cold-start readiness is not claimed.

Review: [before farm](before/screenshots/metro-01.png),
[after farm](after/screenshots/metro-01.png),
[low shoulder](shoulder/screenshots/metro-01.png),
[actual racing](recovery/screenshots/02-racing.png).
Parent diff/evidence review and integration completed. User art and broader
performance/hardware/temporal gates remain open; do not automatically start the
next major roadmap part. No push/merge authorized by this handoff.
