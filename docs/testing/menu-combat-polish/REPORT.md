# Menu and riding-combat polish — 2026-11-12

Later same-day follow-up: [whole-body lean and restored native GPU checks](BODY_LEAN.md).
The blocked native attempts and debug-only delivery below describe the earlier pass.

User request: live tinted city menu, turn weight/knee movement, visible high-speed
AI turning, attack-only weapons, clearer hits, forgiving unarmed/downed contact,
and twice the existing downed-object hop height. Branch: codex/menu-rider-combat-polish.

Implementation: the main/splash/street/course menu renders the existing map with
a slow city camera and three nonphysical display motorcycles. Portrait/gallery
and other specialized pages retain their artwork. The comfort anti-nausea option
freezes the tour. Menu props never acquire Rider/collider components or change
race progress. Existing camera, world streaming, original models and UI fonts
are reused; no external media or reference code is imported.

Both assisted and dynamic rival presentation includes bounded road-curvature
bank, while player bank remains steering-controlled. The rider shifts 7.5 cm
inward at full bank; the inner knee opens through its IK pole while grips/pegs
and chain lengths remain fitted. Inventory no longer overrides the riding pose;
held weapon meshes appear during weapon strikes and hide for idle/kicks/recovery.
Confirmed hits use stronger transient flinches and existing punch/kick audio,
shake and attacker confirmation, with explicit HIT/KICKED feedback for the victim.

Unarmed target-centre envelope expands lateral 0.65–1.9 → 0.30–2.25 m,
longitudinal 0.9 → 1.3 m, height 0.9 → 1.15 m. This approximates body volume;
it does not authorize arbitrary whole-bike overlap hits. Side, facing, occlusion,
authoritative contact time, one-target and three-hit rules remain in force.
Downed bodies/wrecks gain a swept 25 cm side/end assistance margin, preserving
physical collision shapes, own-body exclusion, scenery blocking and cooldown.
Their vertical velocity budget scales by sqrt(2), giving twice the old ballistic
rise when energy/clearance permits. Energy conservation can cap a real contact;
pedestrian hop and upright rear-bike knockoff remain separate. Ride-over emits
a landing thump and player feedback once per accepted contact/cooldown.

Validation results are recorded below as checks complete. Native GPU/subjective
review is blocked: nvidia-smi reports driver/library mismatch, NVML620.70.
No GPU performance or visual acceptance claim. The known high-speed ragdoll and
static-collider trajectory parity failures remain separate unresolved gates.

## Final verification

- `cargo fmt --all --check`: pass.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: pass.
- `cargo test --workspace --locked --no-fail-fast`: 711 pass, 2 fail, 2 ignored.
  Main library: 483 pass, 2 fail, 1 ignored. Content: 59 pass. Core: 168 pass,
  1 ignored. City preview: 1 pass. The two failures are the existing
  `biomechanics::tests::wall_impact_and_high_speed_ejection_remain_bounded`
  (high-speed maximum joint-anchor error 0.16624 m) and
  `world_gen::metropolis::geometry::tests::standalone_static_chunks_keep_contact_ccd_and_support_trajectories`
  (translation divergence 1.611353 m). Their controls/thresholds were not relaxed.
  Model physics/recovery differential tests, player handling, current/baseline
  recovery, NPC/combat, menu lifecycle, grip/knee and weapon tests pass.
- `cargo build --locked`: pass. This rebuilds the debug executable; the release
  launcher binary was not rebuilt in this task.
- Final `python3 tools/audit.py`: pass (575 dependency declarations, 205 runtime
  asset/source records, 30 reference observations). Three registered source hashes
  refreshed; no new media/dependencies or external repository research/intake.
- Progress index and five roadmap tests: pass; 234 verified /356 open.
- Final debug executable `--ragdoll-lab`: all 18 paired baseline/biomechanical
  cases finite. This does not close the known high-speed joint-error gate.
- Final debug executable `--frontend-smoke`, separate temporary save/settings:
  timed out at 45 seconds (exit124). Bevy explicitly selected software rendering.
  Loaded NVIDIA kernel615.78.08 versus NVML620.70 confirmed. No completed menu
  navigation/capture, native recovery, GPU/performance or subjective art/feel claim.
  Both-map/current-baseline native recovery and visual review must be repeated
  once a matching GPU stack is available. No driver changes made.

The initial stronger flinch exceeded tucked hand reach by 1.54 cm. The final
layer bounds trunk movement by tuck while retaining stronger head/neck motion;
the original contact/chain-length assertions now pass. The initial margin test
queried paused Garage physics; its final fixture warms the real physics world
in Countdown before testing the production contact system. Old reach/hop boundary
assertions were updated to the requested new envelope/height; remote misses,
own-body exclusion, timing, geometry and physical rig controls remain tested.

## Delivery / next verification

ADR173 records the implementation boundary and tradeoffs. Five open IDs:
UI-CITY-TOUR-01 (Part15), FEEL-LEAN-01 (Part03), FEEL-WEAPON-01 and FEEL-HIT-01
(Part07), FEEL-DOWNED-01 (Part01). README/progress/task-placement/history and
known issues are synchronized. Human art, combat feel, native viewport review,
menu-streaming cost and broad hardware coverage stay open.

Primary files: `src/frontend/cinematic.rs`, `src/frontend.rs`,
`src/frontend/layout.rs`, `src/animation.rs`, `src/animation/reaction.rs`,
`src/weapons.rs`, `src/combat.rs`, `src/recovery.rs`, `src/game.rs`,
`src/vehicle.rs`, core contact/steering/strike rules and focused tests.

Changes remain uncommitted on `codex/menu-rider-combat-polish`; nothing pushed
or merged. After repairing the GPU stack, use `cargo run --locked` to review this
debug build, then rebuild release before using `play-bevy20.sh` for release checks.
No next major development part was started.
