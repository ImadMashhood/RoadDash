# Track format and coordinate contract (schema 1)

Meters; +Y is up. Bike models face local +Z along the spline tangent. The
road frame includes grade and authored banking; positive gameplay lane is the
camera-right vector (negative local X). Width is half the carriageway, excluding
the separately authored safe verge. World origin is fixed at (0,0,0). Valley Run
starts at (0,0.1,-2940), in farmland, and finishes at (315,60,3003), in hills.
Its 6,847.3 m centerline spans approximately 112.6 m vertically. No rebasing is
needed for this bounded graybox; future worlds beyond the validated ±20 km
coordinate range must add atomic physics/render/navigation origin rebasing.

The default playable course is **point-to-point**, per the user's 2026-10-30
correction. No return road or final-to-first interpolation is generated.
`handling.json` remains a separate approximately 1.98 km closed *test fixture*;
it is not the playable Valley Run route. Old closed-circuit regressions remain
useful, but cannot establish open-endpoint correctness.

## Load and inspect

```sh
cargo run --locked -- --validate-track content/tracks/valley.json
cargo run --locked -- --inspect-track content/tracks/handling.json
ROADDASH_TRACK=content/tracks/valley.json cargo run --release
```

`--validate-track` / `--inspect-track` construct the sampled course headlessly,
print JSON length/elevation/sample/gate/spawn metadata and exit 0; invalid files
exit 2. A bad `ROADDASH_TRACK` fails visibly at startup. Built-ins require no
external download. F4 toggles centerline/driving-line, local normals, carriageway
bounds, world bounds, checkpoint lines and grid-spawn gizmos; it logs render and
collision triangle counts. F3 displays runtime tick/entity/collider/timing data.

## Authoring fields

See `crates/rr_content/src/track.rs` and the two JSON files for the exact schema.
Unknown fields and invalid versions are rejected.

- `id`, `name`, `closed`, `scenery_seed`, `verge_width`.
- `control_points`: position [x,y,z], positive half_width, bank_degrees (±15°),
  farms/city/hills sector, asphalt/dirt/gravel/grass/water surface.
- `checkpoints`: unique stable IDs and strictly increasing canonical fractions;
  start=0, finish=1 for an open course. At least three gates. No self-crossings.
- `spawns`: eight unique grid IDs, nonpositive distance and signed lane. Reject
  overlapping bikes, out-of-width spawns, excessive grade, duplicate points,
  NaNs, negative widths or unbounded coordinates.
- `props`: stable ID, archetype, fraction and lane. Farm, fence, cow, building,
  crossing, pedestrian and guardrail instantiate solid graybox objects.
  Cliff/animal_crossing create explicit hazard markers. Building footprints must
  clear the whole road. Decorative zone dressing also checks road clearance.

Uniform Catmull-Rom interpolation uses extrapolated endpoint handles on open
routes. A dense approximately 1.25 m polyline bakes true 3D arc length; runtime
sampling uses that distance, never raw spline parameter. Road render ribbons
sample at 1.25 m and independent collision ribbons at 2.5 m, chunked around 80 m.
A 10 m terrain grid in 200 m chunks shares an analytical height sampler with
verge/off-road movement. Terrain includes banking and clearance below asphalt;
240 collision rays verify that terrain does not cover road lanes. City's sidewalks
are solid step-height curbs; hill guardrails and intermittent farm fencing remain
physical. Surface traction modifies acceleration, surface friction modifies
contacts; grass slows riders without draining health or preventing forward motion.

Seeded farmland dressing leaves open grass between farms/fences/trees/cows.
City buildings flank the road, with gaps reserved for cross streets; civilian cars
move across those streets and pedestrians patrol sidewalks. Hills use actual
road/terrain elevation; falling objects and the recovering capsule travel through
that 3D space. Cow contact gives the existing bounded lift/landing behavior.
These are original procedural placeholders, not production animal/character art.

## Race gates and endpoint behavior

Each rider owns ordered gate progress. A legal crossing requires forward canonical
movement, crossing the expected physical plane inside the safe corridor/height
and a plausible per-tick displacement. Skipping gates, reverse crossings, a finish
teleport or a finish far outside the verge cannot award a race. Endpoints do not
wrap: unfinished racers stop at the finish. A missed gate displays R-to-retry;
retry resets every rider's gate state. Finished rivals no longer block the line.
Civilian road traffic exits beyond the endpoints and respawns outside the opposite
end's view; it never takes a fictional return road. Profile awards remain once.

Optional branch/split/merge authoring and production route editing are not yet
implemented. Current progress is one canonical path. Approximately three-minute
busy-race pacing is a target pending external playtests; the fixed controller
full-course test verifies completion, not fun or representative performance.
