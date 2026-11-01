# Original-crash comparison verification — 2026-11-01

Branch: codex/original-crash-calculations; base main 5fac329f. Independent of
unmerged biomechanical PR12. No physics-engine or dependency-lock change.

## Automated evidence

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: passed.
- `cargo test --workspace --locked`: 163 passed (client 114, rr_content 19,
  rr_core 30), one existing optional ten-minute soak ignored.
- `cargo build --locked`: passed.
- `python3 -m unittest discover -s tools -p 'test_*.py'`: 20 passed.
- `python3 tools/audit.py`: passed (519 declarations, 28 runtime asset records,
  27 historical reference observations; declaration/hash audit, not legal clearance).
- `python3 tools/sync_progress.py --check`: README/progress match; 364 IDs,
  139 complete / 225 open. CRASHREF-001–003 are verified narrow deliverables;
  CRASHREF-004 retains the remaining native-trajectory/caller work.

Native Rust compile/check commands used
`PKG_CONFIG_PATH=/tmp/roaddash-native/lib/pkgconfig` to installed system runtime
libraries (existing local development metadata shim; no system install).

New domain tests cover static vs moving integer order, mass validation,
reciprocal sign, unscaled damage, stamina floor and reason multipliers,
6000/6001 boundary, damage/speed-gap distinction, glance direction and speed loss,
rider copy-before-bike-transfer, class-1 vs class-2, clamps, all five entry pairs,
signed oncoming speed and ground/air drag through stopping. Adapter tests cover
separate axis scales, upright-frame conversion, matched-speed contact and the
signed native gate. The integration regression injects an oncoming car contact
with healthy/depleted conditions, checks all eleven original body velocities and
the separate wreck velocity BEFORE solver stepping, then physically simulates
through get-up/run/remount and verifies condition reset. Existing default crash,
energy, controls, combat, camera, animation/rig and recovery tests still pass.

## Retail x86 evidence

[`retail-verification.json`](original-crashes/retail-verification.json): **23 checks
passed** using optional Unicorn2.1.4 and user's hashed retail EXE. Rust outputs
come from [`calculations.json`](original-crashes/calculations.json), not a second
Python rewrite. Checks: four signed reason-scale cases, two default/punch pairs,
one contact pair, three signed gap boundaries, twelve full scalar drag cases,
one object-template data check. Surface classification alone is stubbed to a
selected coefficient. Instruction budgets and the exact EXE hash bound execution.

This executes actual retail instruction blocks, not the entire original game.
It does not prove full native collision routing, EXE wall-clock timing, source
license permission, original Windows launch or full replay parity. The report
records that distinction. No retail bytes are committed.

## Native 3D smoke

Default and `--original-crashes` both ran a real native window/audio on Fedora,
RTX3080, NVIDIA615.78.08, Vulkan, with `ROADDASH_VERIFY_REPAIRS=1` and isolated
`ROADDASH_SAVE_DIR` paths. Both passed farm crash→get-up→run→remount, city/hills,
hill crash→run→remount, results, and retry restoring all fifteen racers to the
authored starting grid. Five audio sinks initialized. Receipts:
[baseline](original-crashes/baseline-smoke.json),
[comparison](original-crashes/comparison-smoke.json).

The window smoke's scripted knockdowns primarily exercise regression/recovery;
it is NOT exhaustive visual acceptance of each newly calculated car-contact path.
Those entry calculations are covered separately by the fixed-simulation test.
Local screenshots 03-crash and 06-remounting were inspected: rider detached from
bike, shared skin present, recovery returned to seated state. Screenshots remain
local under screenshots/; no protected original screenshots/media are published.
The short wall-clock samples in receipts are not a production performance claim.

Runtime warnings: unavailable optional Vulkan validation/DRM extensions, skipped
unused dzn adapter, and an unmapped physical gamepad UUID. The selected NVIDIA
Vulkan device and scripted audio initialized successfully. No real-controller,
Windows-hardware, prolonged multi-rider soak, or user feel acceptance is claimed.

## Remaining gates

CRASHREF-004 retains full original low-object/static/wall/attack caller routing,
condition/regeneration/personality fidelity, original pair ordering/reciprocal
traffic response and replay trajectory comparison. Original vertical/terrain/
landing math is documented; Rapier still owns 3D tumble and settling. The accepted
passive rig is unchanged; no biomechanics motors are included. Test and review
this comparison before selecting any new default or combining the two branches.
