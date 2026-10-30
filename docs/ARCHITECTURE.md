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
