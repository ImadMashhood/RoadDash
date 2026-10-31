# Runtime architecture and developer entry points

- Root package `road-dash`: the functioning native offline client library/binary.
- `rr_core`: no Bevy/Rapier; stable IDs, phase/intent contracts, buffered edges,
  reward rules and save/protocol version constants.
- `rr_content`: serde authoring validation and the built-in typed catalog.
- `tools/audit.py`: headless provenance, source hashes, declared-license and
  reference-field checks; no production dependency on developer tools.

Plugins: content -> lifecycle -> simulation/input -> world -> presentation/audio
and diagnostics. Startup sets order world/validation, racers, UI, audio. Boot ->
Loading validates built-in content -> MainMenu. Offline Phase controls gameplay;
AppScreen derives menu/race/results views without awarding outcomes.

PreUpdate captures remappable held actions and edge buffers, then applies menu,
pause/reset/camera/mute intentions. FixedPreUpdate remembers body poses. FixedUpdate
runs collision events, recovery, vehicle/AI/combat/race and transient cleanup,
then Rapier SyncBackend -> StepSimulation -> Writeback with 1/60 s and three
substeps. Post-writeback captures current poses and fixed/active tick counters.
Update interpolates *visual child roots*, animates, displays HUD and emits audio.
PostUpdate follows interpolated rider/torso focus before transform propagation.
No render interpolation is written into physical body transforms.

F3 toggles bounded diagnostic history (240 wall-frame samples), average/P95 frame
ms, measured fixed tick count, entity/collider counts and last gameplay/physics
cost. These CPU/wall metrics are not GPU timings or hardware acceptance claims.
`debug_physics` enables Rapier's collider visualization. `offline` and `devtools`
are default; `--no-default-features --features offline` omits the dev overlay.
There is no pretend netplay feature; server authority/transport is P10/P13.

Input bindings: `content/input.json`; override with `ROADDASH_BINDINGS=/path.json`.
Unknown actions/keys, duplicate keys, empty actions and schema mismatch are rejected
with a visible warning/fallback. Native gamepad/UI rebinding and settings persistence
remain P04 work. F3 is a developer control outside the gameplay action map.

Catalog: `content/catalog.json`, stable IDs and physics/cosmetic separation. Current
player tuning consumes its validated speed/acceleration/braking values; rider IDs
are attached at spawn. Remaining prototype constants move in their gameplay parts.
Profile schema 1 accepts previous versionless aggregate saves. Unsupported or
malformed existing profiles are preserved and new writes disabled; inspect/move
or restore the file explicitly before saving new progress.

Panic reports: default temp-dir `roaddash-crashes/`, override ROADDASH_LOG_DIR;
include local backtrace, OS and app version. No network telemetry. Test via
`--verify-panic-report` (intentional nonzero exit, never a gameplay run).

Future extraction: vehicle/combat/race/AI still share game.rs, physics types remain
in the client adapter, and server/network crates do not exist. Adding empty crates
would not establish authority. See ADR-003 and P01-SPLIT/P10 prerequisites.

## Part 02 additions

`rr_content::track` validates track authoring without Bevy. `track::Track` bakes
Catmull-Rom points, true 3D distances, local road frames and metadata. Valley Run
is an open point-to-point route; Track::default retains a closed handling fixture
for headless regression tests, while build_app selects Valley Run or ROADDASH_TRACK.
See [track contract](tracks/AUTHORING.md). Input never changes gate authority.
Rider owns GateProgress; fixed movement validates gates before one-time results.

`environment` builds independent road render/collision ribbons, terrain and
seeded zone dressing. Patrols and slope-dependent wreck friction run in the fixed
simulation chain. Terrain sampling accounts for bank and road clearance. Cameras
still follow physical rider/runner state; mounting uses actual terrain elevation.
F4 visualizes track geometry; headless --inspect-track/--validate-track validate
both supplied track files in CI. No new engine/physics dependency or external
asset pack is introduced. Surface friction/traction and steering tuning are typed.

## Part 03 additions

`rr_core::HitChain` and contact-severity/strike-geometry functions are pure rules.
`rr_content::handling` validates schema-1 three-preset handling catalogs.
`vehicle` owns the dynamic one-body/two-wheel-ray client adapter and `BikeState`;
`game::drive` still produces shared player/AI intentions and validates race gates.
`combat` resolves one direct/visible target per attack. `npc` owns bounded
pedestrian/cow articulated bodies and safe delayed restoration.

FixedPreUpdate attaches/configures the selected chassis before pose history.
FixedUpdate consumes collision messages, recovery, shared drive intentions,
melee/NPC hits, patrol/cleanup/slope friction and vehicle forces, then Rapier.
Post-writeback reads authoritative BikeState and pre-next-impact velocity cache,
records bounded tick telemetry, then captures interpolation poses/counters.
Update interpolates visual roots before wheel/lean presentation. Physics mass,
center of mass and inertia are independent from rider/wheel visual transforms.

Native defaults to Dynamic; `ROADDASH_VEHICLE=assisted` retains comparison and
legacy headless coverage. New adapter tests explicitly select Dynamic. F5 cycles
validated presets; F6 adjusts assistance; F3 displays RPM/gear/slip/support.
See [contract](VEHICLE_AND_COMBAT.md) for limits and rollback.

## Part 04 additions

Pure rr_core::comfort owns normalized deadzone, comfort ranges/FOV and input
contexts. VehicleIntent preserves digital fields and adds defaulted analog amounts.
controls captures one deterministic native Gamepad plus keys into the same fixed
buffer; context separates on-foot intent. settings owns validated local versioned
preferences and a modal settings/rebinding panel before gameplay input. Native
build_app loads preferences; tests/smoke do not touch real user settings.
Camera reads interpolated position and heading, with volume collision sweeps;
feedback reuses a bounded set of HUD entities. Bevy's pinned Gilrs feature adds
native discovery without migrating input/physics libraries. See CAMERA_AND_INPUT.md.

Handling correction: pure SteeringMemory in rr_core accumulates player turn force;
rr_content validates optional response settings with older schema-1 defaults.
Rider owns raw steering and resolved memory; drive consumes it at fixed ticks,
crash/reset clears it, and telemetry distinguishes raw/force values. Bike and
human share a visual bank driven only by steering; presentation removes chassis
road roll and keeps grade. No human-only counter-roll system remains. Chase
positions use velocity lead and bounded distance. Full original handling parity
is not established; missing reference tuning is tracked in P03-020.
