# RACE-TRIM-01 — playable Metropolis bounds

The playable race uses absolute physical-route chainage 500–11500 m (11 km).
The full 12000 m route, both branches, terrain, buildings, field/crop geometry,
street content and world collision remain unchanged. Existing grid offsets move
forward exactly 500 m, preserving fifteen riders and relative spacing (212–492 m).
The authored manifest remains byte-for-byte unchanged; no content rebake is needed.

`rr_core::race::RaceInterval` owns race-distance/grid calculations. The adapter
uses the interval for initial/retry grid and camera, kinematic/dynamic endpoint
handling, exceptional foot rescue placement, course card and HUD. Absolute
chainage still drives standings, branch choice, traffic and physical sampling;
subtracting the same start from every racer does not alter rank. The full map
remains on the minimap; it has no endpoint markers to relocate.

Metropolis gate posts and their dedicated colliders, gate debug lines, counter
and instructions are removed. Legacy map gate presentation is unchanged. Ordered
physical route checks remain internal (including branch checks); the last check
moves to 11500 m. Ordinary progress, reverse travel, jumping directly to the
finish and crossing the opposite branch cannot grant a finish. Bike and foot
share the same route observer and one-time results path. An incomplete-route
message offers retry without displaying gate requirements.

Authoring validation rejects interior checks outside the playable interval and
grids that would fall outside the retained starting tail. No authored geometry,
physics rig, animation, controls, save format or dependencies change. There was
no external research or imported code/media.

Verification pending coordinated build slot: core interval test, full geometry/
both-route finish/false-win tests, shifted retry regression, content validation,
native smoke and parent integration's full required suite. Source-only manifest
comparison passed (both paths still12000 m); rustfmt and git diff whitespace check
passed. `tools/audit.py` currently reports the expected original-environment source
hash mismatch; parent updates the shared asset ledger after integration. This is
not a legal, performance, hardware or user art acceptance claim.
