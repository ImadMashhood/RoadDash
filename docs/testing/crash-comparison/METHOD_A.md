# Method A — native joint motors (unmerged candidate)

Branch: `codex/crash-native-motors`. User comparison/selection is open.

```sh
cd /home/imad/Documents/ChatGPT/RoadDash-crash-native
cargo run -- --crash-comparison
```

Enter starts the race. F10 cycles and sets 20/60/120/200 km/h; F8 starts a
combat swerve/countersteer release; F9 starts a frontal release. Only mounted
racing accepts these edges. Esc pauses; Enter restarts from results. The overlay
shows Method A, preset, physical reaction, ground support, attempts and remaining
**effort estimate**. Comparison disables profile writes. Normal launch naturally
uses the candidate; `--ragdoll-baseline` retains the accepted eleven-body control.
Run from the worktree: content is embedded where applicable, but normal runtime
content/asset paths still resolve relative to the repository.

## Architecture and preservation

The shared lifecycle is in COMMON.md. Three direct hits initiate mounted steering
away and physical countersteering feedback. Release waits for returning lateral
motion or a near-line/hard timeout; no lane snap. Continuous mounted release uses
the current seated frame instead of the old lateral offset. A short contact hook
filters only the released owner/wreck pair. The original hidden chassis collider
is stored/removed while dismounted, then restored exactly: ColliderDisabled alone
was insufficient to exclude it from the walking controller's casts. The visible
wreck and other obstacles remain solid. Baseline behavior is retained.

Method A changes existing Rapier native joint motor targets and capped gains only.
It does not author body transforms or velocities and adds no root torque guide.
Existing anatomy, masses, motor force ceilings, fixed timestep and energy envelope
remain intact. Pure state rules live in rr_core; typed authored parameters in
rr_content and content/native_crash.json. Supported torso/pelvis contact requires
an upward fixed-world contact normal; wall, dynamic-bike and airborne contacts do
not enable a roll. Near-ground descending casts enable a brief directional brace.
Finite asymmetric limb/spine poses attempt a roll; lack of improvement, loss of
support, excessive motion, exhausted effort or timeout yields to passive settling.
This is an attempt, not a guaranteed forced roll. Supine support permits low-tone
slide; arms/head brace according to body-local travel direction.

Defaults: roll below 16 m/s and 6 rad/s; at most two 1.4 s attempts; protect/roll/
slide activation 0.45/0.34/0.08. Effort starts at 180 J-estimate and spends an upper
motor torque times observed relative angular speed estimate. This is **not exact
measured solver work** and is not numerically comparable to Method B's measured
impulse-work budget. Existing measured kinetic/gravity envelope also reduces gains.
High speed may retain a brief weak brace while disallowing roll. Supported low
motion only rests immediately if already supine, so prone bodies may still attempt.

## Verification and remaining gates

Automated checks cover continuous seated root/anchors across normal/tuck/attack
frames, unchanged transforms/velocities when updating native targets, wall-only
support rejection in the actual backend, bounded pure state/work rules, real
three-hit mounted stagger, pause/reset and exact chassis restoration, and manual
foot finish after a crash five metres before the line in baseline/new rig modes.
Full command results are recorded in TEST_MATRIX.md after execution. Native feel,
Windows/hardware and multi-rider performance remain open. In particular strict
core contact can terminate an attempt when arms/boots support the torso just above
the road; do not claim guaranteed supine settling or measured motor work.

No dependency or media imported. The pinned reference audit in
../../research/crash-behavior/README.md supplies architecture inspiration only.

Parent-coordinated native default-game smoke passed farm crash/remount, city/hills,
second hill crash/remount and finish; four audio sinks initialized. Runtime evidence
is native-game-smoke.txt and native-world-survey.json beside this document. Baseline
smoke and manual comparison review are separate, not inferred from this pass.

Final parent verification also passed native `--smoke-test --ragdoll-baseline` and
`--smoke-test --crash-comparison`: see native-baseline-smoke.txt and
native-comparison-smoke.txt. The latter verifies comparison-plugin/HUD startup and
scripted recovery lifecycle, **not** F8/F9/F10 keyboard delivery or subjective feel.
Attempted X11 focus/image capture on the Wayland/Xwayland window returned BadMatch,
so injected comparison keys were not delivered. Manual keyboard and A/B feel review
remain for the user. Headless actual-third-punch regression passed independently.
