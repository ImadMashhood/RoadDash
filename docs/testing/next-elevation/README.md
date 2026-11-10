# Moderate Metropolis elevation — 2026-11-09

The user requested moderately more pronounced elevation. Raise each longitudinal
knot's displacement from the existing 10 m datum by 25%; keep every knot station,
the quintic C2 blend and the 0.8 m lateral crossfall unchanged. The knot range is
10–75 m instead of 10–62 m (65 m relief instead of 52 m). Analytic maximum
longitudinal grade rises from 3.00% to 3.75%, below the existing 4% bound. Analytic
maximum longitudinal curvature rises from 0.0000739008/m to 0.0000923760/m;
slope and curvature remain zero at each knot. No added hills district, sharp
crest, banking or physical bike/rig tuning.

Both actual route sample ranges change from 12.596–62.482 m to 13.353–75.481 m;
maximum sampled grade rises from 2.9963% to 3.7459%. These extrema also occur
inside the new playable 500–11500 m window. Full authored progress remains
0–12000 m: trimming the race does not trim the map.

## Preservation and support

The approved `LAYOUT.json` is updated with the existing height-only authoring
script. The new `bake_metropolis.py --elevation-only --previous-layout PATH` mode
first requires the previous source's exact SHA to match the baked manifest,
then rejects all source changes except elevation and its derived measurements.
It changes only profile, source hash and route/traffic/control-point y values.
It cannot overwrite the final lane repair with a fresh planar geometry bake.
Unknown source metadata changes are rejected as well as x/z, IDs and progress.

[Reproducible measurements](measurements.json), produced by [measure.py](measure.py),
compare to immutable `b3f14b48`. The entire manifest is exactly equal after
normalizing only those allowed height/hash fields. This includes both paths and
all progress fractions, lane sections, 19,108 buildings and their footprints,
42 fields, 36,615 surface patches, 14 civic sites, 122 parked cars, 192 pedestrians,
1,521 street props, bounds, streets, junctions, track/grid/checkpoints and counts.
Neither legacy map nor any runtime Rust/physics code changed.

The existing runtime derives terrain, pavement, sidewalk, building foundations,
fields and civic grounds from `track.terrain_height`: see
`src/world_gen/metropolis/geometry.rs` (foundation maximum plus 0.12 m; envelope
bottoms extend 0.8 m below ground; pavement and sidewalk offsets retained),
`civic.rs` and `farms.rs`. Render/collider geometry continues to share those
heights. Sampled triangle interpolation errors against the analytic profile:

| Surface | Samples per profile | Before | After |
| --- | ---: | ---: | ---: |
| Pavement | 358,856 | 9.557 mm | 11.921 mm |
| Fields | 25,104 | 9.470 mm | 11.828 mm |

These are triangle midpoint/centroid samples, not a continuous proof or native
collision test. Both stay below the existing 12 mm gate without relaxing it;
remaining margin is narrow, so additional relief needs tessellation review.

## Verification and handoff

- Five Python elevation tests pass: authored/baked path parity, analytic grade and
  knot smoothness, exact field patch coverage, actual support interpolation,
  height-only preservation and six source-drift/stale-height rejection cases.
- The before/after audit asserts exact normalized manifest equality and current
  source hash. `git diff --check` passes. The planning profile PNG is refreshed.
- No dependency, external repository/code or media intake. Research consisted of
  existing approved source, authoring, shared runtime support and installed lockfile
  (Bevy 0.17.3, bevy_rapier3d 0.32.0). This is original project authoring.
- Cargo fmt/clippy/tests/build, repository provenance audit, both-branch native
  recovery and `--metro-survey` are deferred to the parent integration run, which
  owns the shared cargo/native slot and shared documentation. No visual feel,
  collision/hardware or performance acceptance is claimed here.

Reproduce using `/tmp/roaddash-layout-venv/bin/python` (Shapely/Pillow environment):

```sh
PYTHONPATH=tools /tmp/roaddash-layout-venv/bin/python -m unittest tools.test_metropolis_elevation
/tmp/roaddash-layout-venv/bin/python docs/testing/next-elevation/measure.py
```

Save the exact prior layout outside the repository before changing the profile;
run `tools/update_metropolis_elevation.py` and then the elevation-only bake with
that saved layout. `--no-chart` permits numeric-only source updates while native
capture owns the rendering slot. Never substitute a newly generated plan.

Suggested new requirement for the parent live checklist: **ELEVATION-01**, part04,
reason: user requests moderately stronger terrain relief; dependencies: METRO-01,
LAYOUT-03 and the existing shared elevation/support system; acceptance: bounded
20–25% greater longitudinal relief with smooth grades below4%, exact full-map
planar/identity/lane preservation and support/recovery on both branches;
verification: these authoring receipts plus parent native survey/recovery and user
feel review. Keep open until the remaining gates pass. Parent owns README,
PROGRESS, TASK_PLACEMENT, decisions, ASSETS and known-issue integration; no closure,
push or merge by this subtask.
