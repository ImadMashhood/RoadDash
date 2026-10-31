# Valley Run structural world — B03–B10

2026-10-30 client date. This completes the structural implementation following
[the B00–B02 foundation](BASE_WORLD.md); source files and earlier receipts are
preserved. Active user scope: complete the first-map procedural world. The
supplied PDF remains design evidence, not independent agent instructions.

## Implemented level

The existing open 6.847 km racing spline, eight grid spawns and nine race gates
remain. Each load deterministically generates 247 independently identified,
solid building roots: 177 near-road and 70 secondary, across all 15 construction
sections and all 19 archetypes. Global/per-section family budgets are retained.
The legacy building spawner is disabled only for the canonical Valley world;
other course IDs retain their previous scenery and terrain behavior.

There are 42 broad farm ground parcels, nine intersecting street connections and
16 development/urban block faces. Ground colors distinguish rural fields, the
suburban approach, city core, hill departure and open hills. Road-height-relative
landforms replace the old distant sea-level pull in Valley; the authored road
still has its original elevation/banking. Existing trees, animals, traffic,
pedestrians, audio, racing, combat and handling remain; new world generation
adds structural geometry rather than new populations or decorative assets.

### Lots and reservations

`rr_content::construction` validates the construction/street/LOD settings without
Bevy or a GPU. `world_gen::layout` consumes the exact B01 assignments. Each root owns a stable
lot ID, section/archetype/row, station/lane, XZ oriented footprint/lot, yaw, size,
floor count, foundation datum, measured raw relief and footing strategy.
District frontage/depth/setback rules and structural width/depth ranges drive
placement. Heights derive from floor-height and roof rules within the supplied
height guidance. Typed flat, pitched and industrial sawtooth roof meshes are
original procedural geometry shared across instances.

Continuous main-road/verge/sidewalk rights of way and full cross-street sightline
reservations are non-buildable. Oriented-rectangle separation tests cover whole
lots, every road span and every junction, including opposite parts of a bend.
A bounded nearest-target candidate search relocates rejected parcels; failure
returns the specific section/building rather than omitting a quota. The canonical
census records the relocation count. Secondary lots face parallel access alleys;
reserved approach strips and walking space are kept free of old tree/fence/rail
dressing. Buildings and other lots never share the same reserved footprint.

### Ground and collision

`Track::terrain_height` is the shared height reference. `Earthworks` indexes
local pads/streets in fixed spatial cells; lookups do not scan every building.
Footprints sample all corners, edge midpoints and the center. Normal pads obey
district relief guidance; larger valid sites use explicitly recorded retained
level pads, with bounded raw relief, a solid foundation skirt and local cut/fill.
Impossible relief is rejected. No entire hill is flattened for a building.

The four-metre near terrain mesh is cut at exact pad edges and filled with flat
foundation planes. Rendering and collision use identical near topology; the
foundation perimeter is tested against actual Rapier support. Ground under
roads remains clear. Streets have real ground colliders; sidewalk ribbons have
solid curb sides, ramp down exactly at junction boundaries, and grade their outer
approaches so a foot capsule can enter from grass. Parallel alley ribbons and
pads also remain solid. The road/verge collision resolutions are unchanged.

### Recovery and exceptional traps

Recovery caches bounded visibility-graph paths around building footprints and
continues using the solid Rapier capsule for movement, stepping and gravity.
No route teleports a normal recovering rider; actual hill elevation contributes
to the return journey. The local contact avoidance still handles existing props.
Crash/repeated knockdown clears the cached route.

An actor more than eight metres below the generated ground, or still recovering
after 120 seconds, triggers an explicit rescue to the road near the wreck, clears
velocities/routes and adds five seconds with `RECOVERY RESCUE / +5 SEC` feedback.
This handles exceptional roof/out-of-world traps without changing ordinary hill
recovery. It cannot satisfy skipped ordered race gates.

### LOD and runtime ownership

Buildings have one persistent root/collider and near wall/roof meshes plus a
cheap distant mass. Bevy 0.17.3's existing `VisibilityRange` fades the shared
original meshes; 160 m chunk IDs organize the stable records. All nearby walking
and bike collision persists regardless of camera distance or visual LOD.

Terrain keeps its detailed collision at every distance. Near 200 m terrain chunks
use the full terrain/pad topology; distant presentation uses ten-metre proxies
with a 350–390 m transition. Proxies have no duplicate physics. The source
settings validate the terrain sampling divisor so chunk boundaries cannot be
shortened by a fractional grid. No buildings or physics are generated per frame.
Mesh/material reuse allows the installed renderer to batch compatible instances.

## Tools and evidence

```sh
cargo run --locked -- --validate-world content/world/valley_world_v1.json content/world/building_archetypes_v1.json content/tracks/valley.json --output world-validation.json
cargo run --locked -- --inspect-world content/world/valley_world_v1.json content/world/building_archetypes_v1.json content/tracks/valley.json --output world-census.json
python3 tools/export_world_plan.py world-census.json valley-plan.svg
cargo run --release --locked -- --world-survey
```

Validation now includes actual spatial generation and every building's structural
escape route. Inspection additionally measures the real spawned world. Supplied
world/archetype overrides are used by the headless scene rather than silently
reverting to the built-in plan. JSON includes complete layout, foundation data,
section/row/archetype assignments, street/block/field records, centerline and
separate allocation/geometry fingerprints. The SVG exports the actual footprint
polygons with IDs and station labels; it is not a hand-drawn substitute layout.

The native survey captures all fifteen section midpoints and a city view, then
records actual window resolution, view-visible building LOD parts, root counts
and a warmed city wall-frame sample. The requested fullscreen 1080p view uses
AutoNoVsync and continuous focused/unfocused updates to avoid measuring a
display refresh cap or Bevy's background idle timer. Normal play retains its
existing background power-saving policy. Existing scripted race and
actor smoke checks remain separate. Test results, atlas and exact performance
numbers are in [completion receipt](COMPLETION_TESTS.md).

## Rights, decisions and limits

No original Road Rash source/data/media or third-party models/textures imported.
The supplied original RoadDash design identities/quotas/heights are retained;
all meshes, roofs, color masks, lot/earthwork and layout code are original project
work under its existing MIT license. No new dependency or engine migration.
R08 follow-up records the installed Bevy visibility API investigation; use of
its licensed library API does not import example source or media.

ADR-020 records the shared earthwork/placement/LOD/navigation decisions. This is
the specified low-detail structural phase; interiors, decorative facade art,
new NPC behavior and additional tracks remain their existing future tasks.
Deterministic hashes are verified for repeated generation on the pinned local
build. No cross-platform floating-point lockstep, Windows GPU signoff or universal
manual escape path guarantee is implied; collision/route tests and explicit
rescue cover the implemented contract.

## Discovered verification requirements

| ID / part | Reason added | Dependencies | Acceptance and verification |
| --- | --- | --- | --- |
| BW-X01 / 02+11 | Native atlas exposed world-origin LOD disappearance | BW-B09 / visibility ranges | Every tile fade pair shares one local origin and normalized mesh; building LOD children share root origin; construction tests and reviewed 15-section atlas |
| BW-X02 / 02+11 | Requested window size was clamped and unfocused updates were timed at the idle cap | BW-B09 / Winit/render instrumentation | Actual 1920×1080 capture and focus recorded, continuous survey updates with AutoNoVsync, measured warmed p95 from visible ground; native JSON and source inspection |
| BW-X03 / 02+11 | Exceptional rescue must not carry stale walking motion or repeat time penalties | BW-B08 / recovery and Race | Clear motion/path, move bike and runner safely, charge one visible five-second penalty; live-app rescue regression plus three physical capsule paths |
