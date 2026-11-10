# Hills revision — 2026-11-10

## Implementation and scope

Hills now starts with the final kilometre of the existing Metropolis physical street (10,500–11,500 m), including unchanged building footprints, facade families, pavement, sidewalks, parked cars and street furniture. Its source street and elevation align with the playable road; the following 500 m blends into the hills. Metropolis remains the default map; `cargo run --release --locked -- --hills` selects Hills.

Hill bends are tighter and reparameterized to keep physical travel consistent with station distance. Guardrails protect curves instead of enclosing every straight. Six developed hill farms reusing actual Metropolis barn/windmill/hay/machinery assets and textured crop/soil materials, more trees and batched grass, giant boulders, closer western frontage and the existing mounted cowboys/atmosphere enrich the course. Market stalls, blacksmith shelters, freight yards, wells, corrals and open-access backlots fill space between and behind the streets. A final overhead review prompted 42 compact median courtyards (11 in the 6650–6950 m review band) and 54 broad irregular crop/orchard parcels beyond the yards. Exact-coordinate terrain caches and a conservative whole-field road bound avoid repeated authoring queries without changing the resulting layout. The two town roads still rejoin inside the town, before dunes.

An original suspension bridge crosses a physical canyon about 82 m below the deck. The latest user follow-up removes all low side railings and their colliders. Vertical hangers are now 0.65 m square steel beams with matching collision; the deck edges between supports remain open. Traffic adds eight articulated eighteen-wheelers and six tractors alongside 24 cars, with matching compound colliders, full-length avoidance, curve speed limits and guest trailer presentation.

Immutable Hills solids retain their exact collider, entity, transform and role while omitting redundant fixed rigid-body state. Moving objects and breakables are excluded. `ROADDASH_HILLS_FIXED_BODIES=1` restores the fixed-body control for same-binary measurement. No physical rider/rig tuning, dependency changes, default-map change or save migration.

Eight fixed volumetric cloud banks use one original 48³ density texture and an original maximum-12-step shader. No extra shadow maps or depth prepass are added. `ROADDASH_HILLS_CLOUDS=0` restores the baked-cloud fallback. Volumetrics are a visual feature, not an assumed speedup.

## Files and decisions

Route: `tools/bake_hills.py`, both Hills track JSON files and `src/track/hills.rs`. World: `src/world_gen/hills/{city_slice,bridge,countryside,dressing,props,geometry,static_collision}.rs`, shared Metropolis helper refactors and world dispatch. Traffic: `src/heavy_traffic.rs`, `src/game.rs`, runtime scheduling and `crates/rr_core/src/traffic.rs`. Clouds: `src/world_gen/hills/clouds.rs`, `assets/shaders/hills_clouds.wgsl` and `src/sky.rs`. Material-event scan optimization: `src/tangent_detail.rs`. Evidence harness: `src/hills_revision_survey.rs`, `src/hills_bridge_exit_tests.rs`, recovery fixture and `tools/run_hills_revision_checks.py`. ADR137 records the decisions.

New stable requirements HILLS-07/08/09/10 remain open for full-course human acceptance; existing PERF-08/P16-003 cover performance. README remains the canonical checklist: 222 verified / 266 open. No next major development part started.

## Research and provenance

Three fresh agents implemented route/canyon, scenery and heavy traffic. The later visual follow-up used four new agents for bridge, farms, town and clouds. Existing Metropolis authoring, the earlier farm planning concept and repository-owned catalog assets were inspected/reused. Bridge and heavy-vehicle geometry are original procedural work. Pinned local Bevy 0.17.3 APIs/shaders were inspected for renderer integration. No external repository, imported media or retail Road Rash files were brought in. `docs/third_party/ASSETS.csv` records source hashes and declarations; audit is not legal clearance.

## Verification

- Workspace tests: 575 passed, one existing ignored soak (`tests.txt`). Includes both-rig differential checks, city street coverage, physical bridge open edges/supports, heavy traffic geometry/following and standalone-collider contact/knockoff equivalence.
- Strict workspace Clippy and formatting pass. Locked debug/release builds and asset audit are recorded beside this report.
- Native city/course and west/east recovery, baseline-rig and Metropolis controls pass. Both town-route smokes report two remounts with zero exceptional rescues. Receipts record the exact binary per case; the optional native harness and final town additions were subsequently verified separately. The final active check on the benchmark binary preserves the same 42 courtyard/54 field counts and completes Assisted fall/remount without rescue. The preceding Dynamic/census run differs only by the later exact-preserving generation cache.
- The updated scene confirms 3,006 road probes (maximum error 0.02584 m), 17 captures per active run, zero bridge rail segments, 42 square hangers, eight moving semis/six tractors, 88 backlots/five central ranches/118 reused props/32 roadside groups, 42 median courtyards and 54 outer fields (15,325 cells), plus the eight-bank cloud budget.
- Assisted lateral departure: actual ragdoll descent 82.6 m, then Riding, zero exceptional rescues. Native-default Dynamic departure: authoritative chassis descent 82.7 m, peak downward speed 39.2 m/s, Riding and zero exceptional rescues. Dynamic remains mounted in this trajectory; the result proves physical descent, not an immediate combat-triggered ragdoll or an automatic return to the bridge road.

The initial active fall fixture incorrectly assigned sideways velocity to the optional assisted chassis, which follows its controller rather than integrating that velocity. Review then found that the assisted sampler could hover beyond the deck and snap to the floor. A Hills-only unsupported-deck handoff now checks real ground support after solid sweeps and enters existing physical recovery with preserved momentum. Four new production-controller tests pass: steering and actual combat kicks on both sides, supported hops/blocked moves, and other-map isolation. The final native fixture applies lateral controller momentum and requires observed Falling, eleven bodies/wreck and actual body descent. It never directly calls crash. The `active` case explicitly selects `ROADDASH_VEHICLE=assisted`; `active-dynamic` separately measures authoritative chassis descent and downward velocity. The native application defaults to Dynamic, which remains selected for all other native cases and timing. A failed intermediate native fixture had selected Dynamic while asserting Assisted-specific immediate ragdoll behavior; this is recorded as a fixture mismatch, not a passing run. No rig/mass/joint tuning. Historical `before-performance.json` is a prior optimized-debug build, so it is context only, not a valid paired speedup baseline.

## Performance

The same release binary (`215cc272371945362b35eb3e8c7f35c61f287afb5af1406810b16f3215166977`) was tested with native-default Dynamic handling, identical dense scenery, and fixed-body/bodyless and cloud/fallback switches. Average of the two per-run frame-window medians: **21.805 → 20.765 ms (4.8% lower)** with redundant fixed rigid bodies removed. Average weighted physics cost: **8.342 → 7.369 ms/fixed step (11.7% lower)**. These are window-summary CPU/wall measurements, not raw-frame percentiles or GPU timestamps. Stable 60 FPS remains unverified and the measured window medians exceed 16.7 ms.

The approximately 0.20 ms cloud-on/fallback difference is smaller than run-to-run variation; no reliable cloud speedup or precise cloud cost is claimed. One candidate run overlapped a newly opened game and is archived/excluded. The valid control preceded that interruption; the remaining five cases were rerun cleanly on the same binary. Background indexing was temporarily suspended and restored. Detailed samples, method, hardware limits and receipts: [performance evidence](PERFORMANCE.md).

## Native views

- [Open suspension bridge](active/screenshots/revision-suspension-canyon.png)
- [Imported city street](active/screenshots/revision-city-driving.png)
- [Sharper hill turns](active/screenshots/revision-tight-curves.png)
- [Metropolis-style farmyard](active/screenshots/revision-farmyard-detail.png)
- [Western frontage](active/screenshots/revision-western-west.png) and [backlots](active/screenshots/revision-western-backlots.png)
- [Moving eighteen-wheeler](active/screenshots/revision-moving-eighteen-wheeler.png)

## Outstanding gates

Human full-course art/feel, broad recovery balance, mounted-canyon return to the road, a native combat-triggered kick through a bridge gap, multiplayer session/hardware coverage and GPU/target-frame-budget certification remain open. Static tests prove physical openings and collision continuity; do not equate them with every gameplay or hardware scenario. Local branch is `codex/hills-map`; no push or merge is claimed. Concurrent civilian traffic-stall work in this checkout is preserved and tracked separately; the final workspace regression count includes those tests.
