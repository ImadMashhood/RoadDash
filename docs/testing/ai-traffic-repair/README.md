# Predictive rival traffic repair — 2026-11-09

Scope: prevent recurrent avoidable AI/car impacts while retaining solid traffic collisions, accepted rider physics, responsive gradual steering and the existing racer/killer target/retaliation logic. No imported source, assets, dependency changes or physics tuning.

## Diagnosis

`game::drive` previously noticed only cars within 35m, reflected the car lane through the road center without checking the destination, and kept full racing/chase throttle. At 50m/s this gives less than one second for a stopped car and even less for oncoming traffic. The reflected destination could be inbound or occupied. AI traffic braking did not exist; the kinematic AI brake path also multiplied by the human brake strength (normally zero). Chase throttle independently reused the unbounded chase speed.

## Change

Pure `rr_core::traffic_avoidance` predicts longitudinal occupancy intervals and conservative lateral travel over a speed-dependent 3–7s horizon. Candidate passes check the whole lateral sweep, including alongside/rear-closing cars; deterministic selection is independent of traffic iteration order. If no pass is reachable, a stopping/following limit feeds the same dynamic/kinematic intention path and full ordinary service brakes. This is intent, not collision prevention by teleportation or immunity.

The adapter uses signed track gaps (nearest wrap only on closed courses), same-corridor routing, actual placed car transforms for road-relative lane coordinates and all nearby traffic regardless of rendered/collider interest. Outbound usable bounds honor legacy positive-lane traffic and Metropolis negative-lane traffic, including street offsets and marked dividers. Combat choices remain upstream; an unsafe car approach bounds their drive intent. Player controls, traffic contact rules, car colliders, accepted rig, recovery, saves and world content are unchanged.

The forecast is deliberately bounded and assumes constant car speed plus conservative rider lateral convergence. It does not promise avoidance of unavoidable close spawns, impacts caused by other racers, arbitrary cross traffic, or every turn/taper. Broad long-race/human gameplay and hardware/performance acceptance remain open.

## Verification

- Lockfile/source inspection: Bevy 0.17.3, Rapier 0.32.0; no dependency edits.
- Six pure planner regressions: high-speed early response, occupied/oncoming pass, blocked lateral sweep, unobstructed combat/race intent, stationary stop/resume, iteration determinism.
- Existing native-library traffic/contact regressions: 5 passed (`cargo test --lib --locked traffic_ -- --nocapture`), including physical cross-traffic wipeouts in both adapters and traffic crash/restart cleanup.
- `cargo test --lib --locked predictive_ai_traffic -- --nocapture`: passed, three actual dynamic Bevy/Rapier 600-frame scenarios; all retain health 100 and Riding. Narrow stationary car: rider stops at104.5m behind car110m, minimum center distance5.46754m. Narrow12m/s leader: rider218.3722m/car229.99817m, minimum11.555145m. Wide stationary car: rider passes to425.22864m, minimum2.0604954m. All exercised service braking.
- An initial version passed stopping/following but failed passing because it predicted instantaneous cruise velocity instead of the actual approach velocity. Corrected before final passing result; this failure is retained here as test evidence.
- Final pure-source `rustc --edition=2024 --test crates/rr_core/src/traffic_avoidance.rs` run:6/6 passed after the launch-hazard flag was added. This small flag preserves ordinary first3s straight launch while allowing a predicted traffic hazard to request evasive steering. The final adapter expression still needs the root integration compile/test gates.
- `cargo fmt --all --check`: passed. `python3 tools/audit.py`: passed (523 dependency declarations,131 runtime asset records,27 reference observations; declaration/hash scope only).
- Shared Cargo slot was released to other fixes after targeted physical tests. Full workspace clippy/test/build, differential models/recovery checks and windowed native race review are delegated to root integration; none are claimed here.

Proposed live requirement: **AI-12**, active Part 01 repair, depends on V2-01-002/AI-09/AI-10. Reason: observed repeated avoidable car impacts. Acceptance: predictive speed-aware passing/braking from real traffic with occupied/oncoming-lane checks; retains racer/killer combat and physical crashes. Verification: domain scenarios plus actual dynamic stop/follow/pass tests, existing contact/recovery regressions and broad native gameplay review. Broad gameplay acceptance remains open.

## Crossing-car follow-up

Review identified that `game::Traffic` is not the full vehicle population. Legacy street crossing cars carry `environment::CrossTraffic`/`Patrol`; Metropolis crossing cars additionally carry `world_gen::metropolis::Crossing`. The follow-up reads these placed bodies without changing either mover or its collision policy. World velocity comes from actual fixed Crossing displacement (including a held junction car), or the legacy patrol's signed analytic travel velocity. Disabled legacy Patrol movers are forecast stationary; Crossing uses its measured displacement, matching its separate mover lifecycle. Orientation projects the existing0.95×1.95m half-footprint into the rider's local road frame; lateral velocity is forecast over the same collision interval as forward motion.

The pure forecast now checks relative lateral extrema and the initial steering-delay kink as well as interval endpoints, so a car moving across the entire lane cannot disappear between two point samples. Candidate generation uses a lazy iterator instead of another per-plan allocation. Cross-car scans use a350m world-distance/4m height filter and constant-size per-car calculations, with one bounded actor-population vector shared with the main-car candidates for that rider. No full-route nearest-segment search is added per crossing car. This is a local tangent forecast, so broad sharp-curve/overpass/taper, large population timing and human gameplay acceptance remain separate gates.

Final standalone domain suite:9/9 passed, adding a moving crossing that must yield despite its current outside-road position, eventual clear/resume, an alongside swept conflict (including separated endpoints with an intervening nonlinear overlap), and an occupied upright-rider pass lane.

`cargo test --lib --locked predictive_ai_ -- --nocapture` passed both tests/five scenarios before the field follow-up: the existing main-car stop/follow/pass cases remain unchanged; legacy crossing brakes20→11.359757m/s, minimum4.861951m, then resumes after actual car clearance. The real Metropolis path/markings/negative outbound case brakes slightly (minimum19.98063m/s), passes with minimum2.9675968m separation and resumes after actual car clearance. Both keep health100/Riding for420 frames. The Metropolis fixture uses local flat physical support beneath the production path to isolate traffic intent; it is not a baked-world native race certificate.

A first shared-target compile picked stale rr_core metadata from another worktree and reported a missing module. Touching only owned source timestamps forced rr_core/rr_content/road-dash recompilation; the results above are from that forced source build. Root integration still requires a project-package clean/rebuild.

Field follow-up adds upright rider occupancy/following only while a real car hazard is predicted, leaving ordinary side-by-side combat and downed-body/wreck/hunter policy alone. A four-rider/two-racer/two-killer physical queue fixture compares actual avoidance with traffic-unaware straight intentions through identical solid collision/crash systems. Final result: traffic-unaware control3/4 riders impacted (minimum pair distance0.108322494m); actual predictive queue0/4 impacted (minimum pair distance4.5671587m), all stopped below1m/s before106m behind the110m car. This control is not a reimplementation of historical AI or proof of whole-race impact rates. No external sources/assets/dependencies were introduced; AI-12 remains the proposed requirement.


Final follow-up source verification:

- Restored hazard-free police/combat lane and throttle behavior: legacy requested width remains unchanged; Metropolis retains its previous full-road clamp. Outbound-only bounds and extra service braking apply during an actual car hazard. The9-test pure suite explicitly checks an unobstructed chase across the outbound divider. Rider clearance uses existing motion projected into each rider's own road frame, avoiding false lateral motion from a curved road tangent.
- An interim restoration exposed a1.995m pass-clearance result (health remained100), below the test's2m gate. Added0.15m to the forecast's lateral safety allowance only; final wide pass minimum2.2001035m. No collider, contact, rig, handling preset or crash threshold changed.
- Final `cargo test --lib --locked predictive_ai_ -- --nocapture`:3/3 pass, covering all queue, main-car and crossing cases. Legacy crossing20→11.248473m/s, minimum5.0456343m; real Metropolis crossing minimum2.963711m; both brake and resume after measured car clearance with health100/Riding. Main stopped/following results remain5.46754m/11.555145m minimum separation.
- Final `cargo test --lib --locked police::tests -- --nocapture`:7/7 pass, including arrest/retry, escape, visibility/falling interruption, paused/restarted/finished priority, officer impact interruption and payout exclusion.
- Final standalone pure-source suite:9/9 pass. Final formatting and diff checks pass. Parent project-package clean plus full workspace/model/recovery/native gates remain required, because cross-worktree Cargo metadata contamination was reproduced twice during earlier overlapping builds. The final successful runs used forced owned-source timestamps under an exclusive shared-target slot.
