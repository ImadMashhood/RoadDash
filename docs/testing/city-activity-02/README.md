# CITY-ACTIVITY-02 — City density and occasional crossings

Implementation commit `17d8dbc` on `codex/city-activity` from `035968c` was
integrated by root as `1c5230f`, with combined verification fixes in `8c1ab49`.
Root owns shared roadmap, decisions, source-hash ledger and integration.

Approved `living_city.json` requests 144 main cars and 384 city pedestrians.
The baker reads these values, regenerates `metropolis.json`, and selects 10
full-capsule-clear sidewalk-to-sidewalk crossing routes under a 24-route cap.
Actual road/sidewalk union, endpoint sidewalks, cafe/furniture footprints and
240 m station spacing are checked. All other generated keys including building
envelopes, routes, surfaces, parked cars and fields are unchanged; Valley source
is unchanged. Existing first 64 shared/rural cars retain their spawn formula;
all 48 added main cars are city-only (now 80 city-only total).

Crossing admission is host-only in the existing fixed Gameplay schedule:
maximum 2 active, 12 s minimum release separation, 45 s rest, 300 m human/camera
proximity and relative-velocity prediction over the entire crossing. Main
Traffic has no Velocity, so signed car.speed uses the actual route street
heading. Recovering hidden Rider roots are ignored; physical owned
ragdolls/wrecks remain blockers. Crossing walkers publish actual Velocity
(zero while waiting) ahead of Patrol's legacy velocity fallback. Ground rays
sample actual fixed road/sidewalk colliders; capsule/rig/mass/joint/recovery/
controls are unchanged. NPC authored-origin replication keys and existing
4096-actor/750 m interest budgets are unchanged; no serialization format change.
More interested actors can increase payload; networking evidence is owned by root.

Five population/crossing authoring tests pass with
`/tmp/roaddash-layout-venv/bin/python -m unittest discover -s tools -p test_metropolis_population.py`.
Bake receipt and exact generated-field preservation are retained. Root reports
528 combined workspace tests passing, including predictive gaps, paced
admission, main-car velocity, actual-source support/completion/zero wait
velocity/retry, and schema budgets/endpoints. Root also reports fmt, strict
clippy, locked build and audit passing; its logs are the full-check authority.

Native evidence uses the frozen combined **debug** functional binary
`/home/imad/roaddash-online-city-final/road-dash`, SHA-256
`81c6c8c9d3e56e4f5998f18b093127a857120b1fc6b88dfac46256b4255dfc50`.
`run_native.py` runs `ROADDASH_CROSSING_SURVEY=1 --metro-survey` with isolated
settings/save/log paths, 300 frames/view, a process guard and binary hash check.
The run exited 0 with 14 captures and no errors/panics; the hash stayed unchanged.
Actual census: 144 main cars (72 per direction), 384 walking pedestrians,
122 parked cars. Of 10 authored crossing routes, 2 started and completed;
both walkers ended at the opposite sidewalk's exact XZ endpoint with nonzero
actual support height. Full records are in `native/screenshots/pedestrian-crossings.json`.
All 14 images were reviewed in `native/contact-sheet.jpg`, with full-size
inspection of metro-00/07/09: actual ground, road, sidewalks and envelopes are
visible. Metro-09 shows sidewalk walkers and cars; metro-07 shows cross traffic.
Static screenshots alone do not prove traversal; runtime counters and endpoint
positions supply that evidence alongside the fixed-step movement test.

This debug functional run measured 29.36 ms average and **32.73 ms p95**, missing
the 16.7 ms target. It is not comparable to older release route benchmarks and
is not GPU timing or a speedup claim. Art, full-route traffic-gap fairness,
hardware/frame-budget and multiplayer acceptance remain separate gates.
Generic sidewalk patrol support and existing NPC art are unchanged by the
crossing-only support fix. No external assets, dependencies or reference code
were introduced. No push was performed.
