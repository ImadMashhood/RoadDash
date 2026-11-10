# Pass02 — low-speed contacts and recovery

Scope: integration branch `codex/gameplay-graphics-polish`, following pass01
`554956de7d02990dc02ffe19050d495e716581b3`. Focused V2-01-003 correction;
no new IDs, moved tasks or closed broad gates. README/PROGRESS synchronized;
TASK_PLACEMENT and checklist counts unchanged. UI artwork remains paused.

## Reproduced defect and correction

Downed wreck/ragdoll contacts compared mounted pre-solver motion against target
post-solver motion for the3m/s ride-over gate, despite using target pre-solver
motion in the energy calculation. The new actual impact-system fixture fails
against old source: bike2m/s, target pre2m/s / post−5m/s incorrectly gets lift.
The failure is retained in reproduction-before.txt. Fixed regression also tests
bike10m/s, target pre0 / post10m/s: the genuine approach retains bounded lift.
Both cases run for Wreck and Ragdoll; the no-lift case checks no scripted target
push or cooldown. Synthetic solver outcomes isolate sample selection; they do
not establish ordinary-game frequency or human acceptance.

Use the existing PreviousMotion target sample for eligibility and energy input
(ADR106). No new domain rule: rr_core contact thresholds, lift energy accounting,
cap and cooldown remain. Current-velocity fallback remains for missing history.
No changes to accepted handling, ragdoll specs/joints/motors/masses, velocity
muscles, baseline, fixed loop, controls, save/audio/progression or content.
No dependencies/assets/source imports; internal source and prior contact,
remount/reverse and pass01 evidence were the research sources.

## Audit coverage

| Area | Existing integrated regressions retained |
|---|---|
| Gentle cars/walls | parking_speed_vehicle_contacts_never_trigger_scripted_wipeouts; static_wall_uses_normal_closing_speed_instead_of_touch_or_total_speed; physical crossing-car cases |
| Rider momentum | rear_contact_threshold_and_mass_weighted_push_do_not_add_vertical_speed; both entity orders and head-on contacts; rr_core shared-normal momentum/energy tests |
| Downed body/wreck | new mixed-time sample fixture; actual_dynamic_wreck_contacts_hop_and_move_the_target_without_ejection; detached_body_contacts_never_script_a_new_mounted_wipeout; real low-wreck traversal |
| Energy | rr_core lift_and_target_push_stay_inside_pair_energy_and_forward_direction; crash_envelope_dissipates_solver_energy_and_reverse_com_without_launching |
| Reverse | dynamic solid-wall backout/forward return; assisted checkpoint preservation; core30/60/120Hz bounded reverse |
| Pickup/remount | live displaced elevated support in both adapters; stale backend/momentum rejection; fallen-pose approaches; model differential physics/recovery |

No additional reproduced defect was found in these covered fixtures. This does
not prove arbitrary trapped geometry, moving traffic or confined mount safety.
P06-009/P06-CONFINED, human accepted feel, full-route/hardware/Windows/controller
and performance gates remain open. No performance claim or facade/material
change; no source atlas or provenance hashes required refresh.

## Verification

`run_checks.py` and checks.json record mandatory commands and exact exit codes.
Workspace tests:416 passed,0 failed,1 existing ignored soak, including the new
regression and src/models/tests.rs physical/recovery differential checks.
fmt, strict workspace Clippy, locked debug/release builds, asset audit, five
roadmap tests and progress synchronization all passed. Four native processes
exited0: both recovery routes/rigs measured reverse−1.825m/s and two0.0000m
pickup errors each; each verified15-racer retry and initialized audio. Both actor
smokes passed with10 cow bodies and16 anatomy /11 baseline pedestrian bodies.
The paired18-case lab exited0, all finite including15 simultaneous rider crashes
per rig. Actual anatomy remount screenshot inspected; representative crash,
run, remount, reverse and actor PNGs retained, redundant menu/combat PNGs omitted.
Native Vulkan extension/gamepad mapping/suboptimal initial present warnings
remain known environment observations, with successful exits. These are no
performance or new-art approval claims. See HANDOFF.md for integration status.

`run_native.py` runs a freshly rebuilt release from this worktree, serially with
isolated save directories. Anatomy route0 and baseline route1 exercise the real
renderer/audio/roster, brake reverse, farm/city crashes, both remounts and results
fixture. Both modes also run NPC actor smoke. Headless lab checks both rigs.
These scripted fixtures include repositioning; they are not uninterrupted full
races, performance measurements or direct native reproduction of the synthetic
solver-time defect. Exact commands/exit codes, logs and native JSON are retained.
