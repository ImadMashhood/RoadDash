# Rider/combat/handling/pursuit correction — 2026-10-31

Implemented the user's combined corrections:
- Kicks apply 8 m/s lateral velocity instead of 3, retaining signed nearest-side
  selection, leg pose and no punch-chain damage.
- Sidewalk trees use 44 m spacing instead of 22; solid trunks and clear walking
  strips/junctions are retained.
- Fifteen racers (YOU plus fourteen CITY names) use independent reference-inspired
  shirt, helmet, trouser and bike colors. Minimap dots still match shirts.
  Both tracks have fifteen non-overlapping grid positions; legacy eight-position
  authored tracks remain loadable with fallback rows for the extra riders.
- Every handling preset's turn response rises 50%, with corresponding lateral
  steering and yaw caps increased 50%. Visual bank and force buildup timing stay
  as authored. Dynamic players use their actual heading and no automatic
  curvature compensation. AI retains its road-following frame.
- Abandoned motorcycles roll physically after knockoff with a narrow profile
  collider; linear momentum is preserved and the rider gets no artificial launch.
- Pure rr_core pursuit selects nearby racers, leaves a staggered start grace,
  approaches alongside at 1.8 m spacing and matches speed with gap feedback.
  Some opponents aim at downed riders within 28 m, using runner/torso poses.
  Engagement windows create breaks; throttle/brake and attack cooldowns limit
  immediate pack pileups. Traffic avoidance remains ahead of pursuit steering.

Reference investigation and adaptation limits: ../research/RIDERS_AND_KICKS.md.
ADR-025 records alternatives, consequences and rollback. No dependencies, copied
reference source, proprietary models or media. Independently selected material
swatches use existing procedural geometry in all human lifecycle states.

Files changed: content catalog/handling/two tracks; rr_content catalog, track and
handling validation; rr_core pursuit module; game/combat/vehicle/recovery/human/
world/UI; streetscape spacing and tests; trackers, decisions, research, provenance
and this evidence. No files removed.

New roadmap tasks: COMBAT-02 (roster/kicks), CITY-04 (tree spacing), RIDE-02
(steering/wrecks), AI-01 (measured pursuit). Their reasons, parts, dependencies,
acceptance and verification are recorded in README and synchronized PROGRESS.
Future original handling/relationship-table parity and human difficulty/art
acceptance remain open. No new major development part was started.

## Verification

118 automated tests passed; one optional soak ignored. Workspace format, Clippy
with warnings denied, locked debug/release builds, four source-input checks and
provenance audit pass. Added neutral-heading and physical wreck-roll tests;
existing initial momentum, remount, terrain, direct-hit and camera regressions
remain passing. Deterministic pursuit checks cover launch grace, speed matching,
alongside spacing and selective downed hunts. Kick tests check both signed sides,
nearest target, stronger velocity, no punch damage and selected-leg animation.

Native crash → remount → city/hills → hill crash → remount → finish passes with
four audio sinks initialized. Reviewed road-height capture shows 15 named racers
on the leaderboard and reduced street tree density. Player pose capture shows
blue helmet, pale shirt and red trousers/bike. Survey and inspection evidence,
metrics and captures are saved under riders/. These are local Linux/Vulkan
scripted checks; Windows and human difficulty/handling acceptance remain open.
Benign Vulkan/device-mapping warnings remain in receipts.

No new dependency or GitHub code reuse. Source and original procedural material
provenance hashes updated. New task IDs are closed only for these bounded
acceptance criteria, not full original behavior or production-art parity.
Next step: human gameplay feedback on kick strength, cornering and pursuit
difficulty, before expanding AI personalities or another roadmap part.

Final city survey: **11.29 ms p95 at 1920×1080**, fifteen racers, all fifteen
world sections captured, within 16.7 ms target on this Linux/Vulkan host.
COMBAT-02, CITY-04, RIDE-02 and AI-01 acceptance is checked in both trackers.
Local commit is on codex/city-combat-followup; no push or PR update is claimed.
