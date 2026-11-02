# Playable City Run — 5.3 km (2026-11-01)

User clarified that the **playable race** should become5.3km. This is implemented
as the new default City Run, not only an isolated viewer. Branch:
codex/city-5300-buildings. Original Valley track/world data, exact quotas and IDs
are unchanged and selectable with ROADDASH_TRACK=valley.

## Implementation

-5,300m open/point-to-point race,53 reserved100m blocks, eight coherent district
zones,977 complete exterior lots and85 original-City-family-eligible facade IDs.
-Actual modeled3D facade fronts/returns:300 geometric modules derived from our own
100×3LOD catalog. Original438 GLBs/sources remain byte-identical. No old facade
sprites, concept PNG cards, imported proprietary textures or copied C++ code.
-One full-depth floor/roof per lot, real rear-wall apertures, recessed glazing,
frames/sills, service doors, pipes, chimney and shared rooftop plant. Returned
facade windows are not hidden by solid backing end walls.
-Independent building/road/sidewalk/trunk/lamp collision, supported negative-distance
starting-grid apron and finish runoff. Existing15-racer grid remains player-last.
-Existing crossing-car actor/physics is reused (20 cars); sidewalk pedestrians
reuse the accepted articulated model/knockdown. No rider/bike/ragdoll retuning.
-Merged row footprints feed the same bounded recovery navigation solver used by
Valley. Physics remains authoritative; no visual interpolation moves bodies.
-Deterministic offline bake and rr_content contract validate route length/seed,
finite geometry, complete districts/frontages and whole-lot street reservations.
Both native preview and game share assembly/LOD rendering in world_gen::city.

## Run / reproduce

```sh
cargo run --release
ROADDASH_TRACK=valley cargo run --release
python3 tools/generate_city_race.py --check
# Re-bake after changing config; --seed also saves the new canonical seed:
python3 tools/generate_city_race.py --seed 74219
# Only needed when deriving modules again from our existing full GLBs:
python3 tools/extract_front_modules.py
cargo run --locked -- --inspect-world --output city-report.json
cargo run --locked -- --world-survey
cargo run --locked -- --smoke-test
```

Recompile after baking: compiled track/plan/settings must match. Runtime resolves
architecture:// through art/buildings/mesh_assets in the checkout, or a packaged
assets/architecture directory beside the executable. Include front_modules/ and
materials/ when packaging; the isolated catalog gallery additionally needs the
original models/. Rider assets retain their existing root and lifecycle.

## Evidence

- rust-tests.txt: workspace regressions including unchanged physical rig/recovery
fixtures;977 actual Rapier building-roof rays; supported grid/road/sidewalk rays;
back-to-street foot routes across five districts; legitimate ordered checkpoint
crossings and player-last grid. rr_content rejects bad sites, district gaps,
missing frontage and invalid geometry.
- python-tests.txt:48 tests, including deterministic full geometry bake, exact
compiled-data equality, course/district pool filtering, reserved setbacks, one
shared roof/floor/service door per lot and all300 front GLB hashes/index bounds/
front bounds/LOD reduction. Original source GLBs remain unchanged.
- fmt.txt, clippy.txt, build.txt: required locked workspace checks.
- roadmap-tests.txt: five synchronized checklist/placement/history checks.
- reproduction.txt: single-config bake matches track, plan, settings and preview.
- inspect-world.json: validated authoring report; not alone proof of physical
placement. placed-census.json counts actual977 CityBuilding bodies in the world.
- native-smoke.txt and game-smoke/: real default game verifies all15 racers retry
to grid, two crash/get-up/run/remount sequences and results screen. The results
screen uses the existing completion fixture; ordered gates are separately tested.
- valley-regression-smoke.txt: unchanged Valley farm/city/hills crash/recovery/finish.
- native-survey.txt and survey/: nine1080p actual-game views of the full5.3km route,
residential/industrial/run-down/commercial/office-rear/Chinatown/finish/block areas.
Survey cameras are developer overrides; the race world and its physics are real.
- audit.json: provenance/declaration/hash audit only. Local development art is
explicitly cataloged as unshipped draft media; it is loaded from the named art
source, outside the production assets directory. This audit is not certification
of release rights or final art approval.

An initial survey attempted images before creating its output folder; creation
now precedes capture and the survey asserts all nine files exist before passing.
Initial Clippy failures (lazy Option expression/helper after test module) were
corrected. A Python reproducibility assertion initially compared tuples to JSON
lists; canonical serialization now checks actual data equality without a huge
irrelevant diff. Final evidence supersedes those failed attempts.

## Research and architecture

Pinned arkits/road-rash aa9cb6d3181e20736af71022cd4d6aac0e2788d1,
[building_panels.cpp](https://github.com/arkits/road-rash/blob/aa9cb6d3181e20736af71022cd4d6aac0e2788d1/src/building_panels.cpp):
facade, roof, exposed-end and side-street-return roles informed this independent
implementation. The old renderer projects panels; it does not supply full3D
buildings or a modern procedural city. Exact route/selector replication is not
claimed. R01 remains REFERENCE ONLY; no source/retail media copied, no dependency
or engine migration. ADR044 records design, tradeoffs and rollback.

## Checklist and limits

CITYGEN-01 is verified in Part00. V2-04-003/004/005/006/007 and MESH-01/02 retain
broader final art, site variety, fine collision, optimization and acceptance work.
The avenue is straight and point-to-point; side streets are environmental grid
connections rather than alternate checkpoint routes. Buildings are complete
exteriors with solid proxy collision, not enterable interiors. Ornamental stoops,
awnings and trim lack exact mesh collision. Production texture/weathering/roof
variety, streaming/cold-load/long-session/multi-crash performance and Windows/GPU
hardware review remain open. No60fps or broad-platform claim is made from the
bounded native samples. Existing audio, saves, controls and physics remain intact.

No commit/push/merge performed; prior uncommitted catalog work is preserved.

## Files changed

New: content/tracks/city5300.json, content/world/city5300.json and city_settings.json;
crates/rr_content/src/city.rs; src/world_gen/city/{mod,geometry,visuals,survey,tests}.rs;
tools/{city_buildings,extract_front_modules,generate_city_race,test_city_buildings}.py;
300 front_modules GLBs/manifest and this verification folder.

Modified: track/world/runtime/startup/recovery/main integration, shared navigation
adapter, factored existing crossing-car helper, world census, content module
registration, preview adapter/config/generator/tests, README/PROGRESS/checklist
placement, decisions/history/known issues/test matrix and provenance/research.
The original catalog meshes and Valley content files are not modified. No files
were removed. Prior uncommitted catalog/district changes are preserved separately
from the work described here.

Final verification.json records counts and receipt hashes. One existing600s
soak remains ignored by the standard Rust command. The terrain now extends450m
beyond the parallel-road offset, with500m longitudinal ground runoff; grid and
finish asphalt aprons remain separate. Cross-street sidewalks now span only the
reserved distance between main-road and parallel-road edges, with a contract/test
preventing carriageway intrusion.

Recommended next steps: review the nine district captures and ride the course;
then polish fine collision/stoops/awnings, roof/site variation and profile
streaming/cold-load/simultaneous-crash costs before closing V2-04 production gates.
