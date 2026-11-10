# City-entry lane repair — 2026-11-09

The reported two-lane-looking city entry with four-lane traffic was real. The
planning source reserves an **8 m road at the urban end of `farm_approach`**.
The earlier baker appended a 13.6 m farm asphalt overlay *after* generating
sidewalks. That overlay extended underneath the raised sidewalks and into 20
existing building envelopes. Asphalt-only containment therefore passed while
outer-lane cars drove through visible sidewalks. Fine samples first reproduce
whole-car sidewalk intrusion at 2972 m; this is not an allocation-count finding.

The retained baseline receipt measures 1,934.214 m² of road/sidewalk overlap and
2,352 combined body/paint failures across 12,552 body and 4,892 paint samples.
The baseline has not been retested using the enlarged final sampling set; its
counts are not a same-denominator performance or failure-rate comparison.

## Implementation

- Keep the wide farm road and original grid. Both route profiles now have four
  lanes through 2800 m, a smooth 100 m four-to-two taper, two lanes through the
  physically narrow entry to 3350 m, then the existing three-lane branches and
  four-lane city after 5362.5 m. The latter transitions still blend over 100 m.
- Smoothly join the traffic/paint center to the reserved street center during
  2800–2900 m, before the narrow reservation starts around 2970 m. The old
  3150 m lateral switch is removed. Authoritative race paths remain identical.
- Clip **additional** farm asphalt outside the city envelope. The source urban
  road, sidewalk access, building identities/envelopes and collision stay intact.
  No buildings, traffic cars or pedestrians are hidden or removed to fit lanes.
- Paint uses the correct end cross-section through tapers/bends. Its 14 cm edge
  stripe is inset 25 cm, keeping its whole width and short bend chords on asphalt.
- `rr_core::streets::shares_lane_ahead(sections, route, station, outgoing, slot,
  other_slot)` checks current and 120 m ahead slot convergence. The traffic
  adapter follows the destination queue before physical bodies overlap and uses
  stable entity priority for an equal-progress side-by-side merge. This is a
  focused merge correction; broad queue overtaking remains a separate task.

No dependencies, imported art or third-party code were added. Inspected installed
versions are Bevy 0.17.3 / bevy_rapier3d 0.32.0. Source provenance remains the
approved `docs/planning/farms-long-city/LAYOUT.json`; regeneration uses
`tools/bake_metropolis.py` with the Shapely authoring environment.

## Evidence and verification

- [Final audit](after.json): **30,120 complete 1.9 × 3.9 m car footprints** and
  **17,828 complete paint quads**, every 4 m across both route records plus 1 m
  samples at the entry and all lane tapers. Zero body/paint failures. Full visible
  road/sidewalk intersection is 0.00000156 m² (triangulation precision); full
  road/building intersection is zero. The collider envelope also contains the fitted 1.85 × 3.8 m vehicle visuals
  (`src/props.rs`). Road containment tolerance is 2.5 cm;
  raised-sidewalk/building overlap is checked separately, never excused by that
  tolerance. This tests actual final surface polygons, not nominal widths.
- [Preservation receipt](preservation.json): all 19,108 building records, exact
  race paths, track/grid/gates, fields, bounds, junctions, streets, source hash,
  street props, pedestrians and civic sites are byte-equal to `ebc70d73`.
  Parked vehicles remain 122; moving traffic allocation is unchanged.
- [Authoring tests](authoring-tests.log): seven street-life/streetscape tests pass,
  including the new regression proving farm extensions cannot conceal urban
  sidewalks or buildings. [Baker receipt](bake.log).
- `cargo fmt --all --check` passes. `python3 tools/audit.py` was run and fails
  on `asset/source hash mismatch: original-cloud-daylight`, an untouched source
  in this branch; parent must reconcile provenance and rerun. See
  [format](fmt.log) and [repository audit](audit.log) logs.
- Added Rust tests `entry_car_bodies_and_paint_stay_on_visible_carriageway` and
  `entry_merging_slots_share_queue_before_bodies_touch` cover actual transformed
  body/paint support and a side-by-side pair forming longitudinal spacing before
  the taper. **Rust tests, clippy, build and native entry/recovery/survey captures
  are deferred to the parent integration run**, per its shared-build-slot
  instruction. No runtime or visual acceptance is claimed from authoring alone.

Reproduce the final geometric audit with:

```sh
/tmp/roaddash-layout-venv/bin/python tools/audit_metropolis_lanes.py --output docs/testing/next-city-lanes/after.json
/tmp/roaddash-layout-venv/bin/python -m unittest discover -s tools -p 'test_metropolis_street*.py'
```

Existing CITY-WALK-02 and WORLDART-01 remain open for human visual, route/access,
streaming and performance acceptance. No new requirement IDs or checklist
closures. Parent owns the shared README/PROGRESS/DECISIONS/ASSETS update and final
integrated validation. No push or merge performed by this task.
