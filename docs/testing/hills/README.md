# Hills — independent course (2026-11-10)

User-directed original12.5km course, with the supplied diagram's town-exit merge
changed to an **in-town merge** as explicitly requested. Run:

```sh
ROADDASH_TRACK=hills cargo run --release --locked
cargo run --locked -- --inspect-hills
cargo run --release --locked -- --hills-survey
```

Metropolis remains the default; Valley and City remain selectable. The online
map picker also recognizes Hills. Course name, race bounds, minimap, branch
choice and checkpoint validation use the selected course.

| Logical station | Setting |
| --- | --- |
|0–1.5km|City streets tapering into the countryside|
|1.5–5.5km|Winding climbs and descents with solid guardrails|
|5.5–7.4km|Two sandy western-town streets|
|7.4–8.5km|Rejoined main street continuing through town|
|8.5–12.5km|Rolling dune terrain and finish|

Route source: `tools/bake_hills.py`; two251-knot authoring documents in
`content/tracks/hills*.json`. The shared logical stations keep the two town
routes comparable for racing. Render/ground collider geometry shares topology;
resident building, rail and prop collision remains independent of visibility.
No dependency, controls, save format or accepted physical-rig tuning changes.
Road elevation spans7.91–217.96m; maximum sampled grade26.69%.

Original western storefronts have false fronts, timber porches, board seams,
window frames and sign panels. Cows and residents reuse existing NPC interfaces;
cowboy hats are original additions. Saddled horses are original **static solid
scenery**, without horse gait or a new horse ragdoll. Trucks reuse the existing
original TRUCK2 work-truck model. New structures remain draft procedural art. Four original barn/pasture sites,
existing conifer/broadleaf trees and bounded grass batches fill the countryside.
The shoulder fades from gravel to grass, then back to sand approaching town.

## Sources and rights

The user-supplied Photo1 diagram informed only the sequence and broad layout.
Inspected existing `content/props.json` and `art/props/mesh_assets/README.md`;
reused original cow meshes (COWA2/COWB2/COWC2 family), existing human skins and
`PROP-532EF1E6365B`/TRUCK2, TREE4A2/TREEH2 and BRNGRN2. Existing original-game research/catalog names identify
references, not newly imported retail bytes. Missing storefront/horse/hat
geometry is independently authored in `src/world_gen/hills/props.rs`.
`docs/third_party/ASSETS.csv` records source hashes and original MIT provenance.
No new external repository, dependency or proprietary media intake.

## Verification and remaining gates

Final workspace regression: **536 tests passed**, one existing ignored soak.
`cargo fmt --all --check`, strict workspace/all-target Clippy, locked build,
asset/dependency declaration audit and five roadmap tests pass. Receipts:
[tests](tests-full.txt), [Clippy](clippy.txt), [build](build.txt), [audit](audit.txt).

Final native survey passes11 completed screenshots,1506 physical road probes
and retained solid geometry. Maximum road-support error is0.025898m against a
0.05m gate. [Survey report](survey/screenshots/hills-survey.json),
[green hills](survey/screenshots/hills-02.png),
[western street](survey/screenshots/hills-06.png),
[in-town merge/shared street](survey/screenshots/hills-07.png),
[dunes](survey/screenshots/hills-09.png).

Native census:239city/town buildings,950guardrail segments,737trees,4farms,
27cows (12farm cattle),15static horses,21cowboy residents,8parked trucks and
40,035grass tufts in69batches. The existing24moving traffic vehicles and15racer
roster remain active. These counts are verified with actual spawned geometry,
collider presence, both-road footprint clearance and native captures; they do
not establish final art quality or a performance budget.

All **five final native cases pass**. Every tested remount reports0.0000m pickup error.
The serial native recovery batch is recorded in [native checks](native-checks.json)
with a [frozen binary receipt](native-binary.json). Those cases exercise both town
branches, steep-hill recovery and the retained baseline rig, followed by a legacy
Metropolis recovery regression. See each case's native log and screenshots for
its actual completion status. Native logs are retained as `native.txt` alongside
the screenshots (`native.log` is ignored by the repository).
HILLS-01/02/03 remain open for human art/feel/full-course
acceptance. Survey captures pause racers: their timings cannot certify a driven
race, GPU timings, Windows or target-hardware performance. Native checks used
the optimized debug build; compilation/testing overlapped part of the batch,
so captured timing fields are not an isolated benchmark. Static horses,
procedural architecture, branch fairness and steep-hill feel need user review.
