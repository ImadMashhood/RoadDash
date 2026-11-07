# Method B — velocity muscles

Branch `codex/crash-velocity-muscles`; worktree
`/home/imad/Documents/ChatGPT/RoadDash-crash-velocity`.
User-authorized comparison prototype, not a main/default merge or final feel signoff.

```
cd /home/imad/Documents/ChatGPT/RoadDash-crash-velocity
cargo run --release -- --crash-comparison
```

Start the race normally. F10 cycles/sets20/60/120/200km/h while mounted; F8 tests
combat stagger/countersteer/release; F9 tests frontal release. Both F8/F9 use current
motion. R resets, Esc pauses, Enter remounts as before. The overlay labels Method B
and shows actual pilot phase, positive work and attempt count. Comparison mode
sets Persistence(false), so trials do not write progression. Natural gameplay
also uses the new behavior. Existing `--ragdoll-baseline` bypasses it; existing
`--ragdoll-lab-view` and headless `--ragdoll-lab --output ...` remain available.

## Architecture

Shared A/B code handles the three-direct-hit KO, mounted away-hit steer then
countersteer with physical tire forces, obstacle interruption, continuous release
from the seated pose, and short pair-specific own-wreck collision grace ending on
clearance or0.65s. Re-knocked runners keep their own body origin. No lane snapping
or crash launch is introduced. Common source/acceptance: ../COMMON.md.

B's active controller uses reciprocal angular impulses rather than native active
motor springs. Pure rr_core rules calculate full-tensor pair corrections, exact
positive work and finite recovery phases. rr_content validates separate tuning in
content/velocity_muscles.json. Existing native anatomical constraints/passive
resistance and accepted masses remain; their active drive is disabled to avoid
competing controllers. Desired arm reach/head protection/supine roll targets are
anatomically clamped. Linear velocity/COM and body positions are never authored by
this controller. The existing mechanical-energy guard stays authoritative.

Default limits:12/s response coefficient;40Nm active joint torque (also bounded by each native joint cap);6rad/s desired relative rate;5rad/s maximum
feedback correction per tick;180W positive-work rate,180J total. Initial brace up to0.55s (shorter with higher speed), roll only
with torso/pelvis/spine fixed-surface support(normalY>=0.65),speed<=16m/s and
spin<=6rad/s; two attempts of at most1.15s each, early yield after0.85s without
progress,0.55s retry pause. A supine body protects its head/limbs while friction
slows it. Severe motion yields; no guaranteed roll or somersault. Quiet rest stops
active writes, preserving sleep. No world-position follower or body-position pin is used. A separately labeled,
bounded fixed-support angular guide may help the torso roll; it never acts airborne. Standalone16-body lab owners participate in B, not only racers.

## References and provenance

Original implementation studied the pinned MIT Godot directional-brace/roll state
ideas and MIT/Apache Rust follower concepts already recorded in
research/crash-behavior/sources.json. No code or media copied; no new engine,
crate, dependency migration or asset. Unlike Godot's scalar inertia/direct roll
velocity writes, B uses full tensor reciprocal impulses and measured work. Unlike
the Rust follower, there is no per-limb linear animation-position chase.

## Evidence and open gates

Focused tests exercise a real Rapier fixture and prove the controller actually
changes angular motion while preserving each exact linear velocity and total
pair angular momentum; work/power/torque caps, no positive-work damping, invalid
input, real wall-only contact rejection, high-motion/airborne yield, retry limit,
quiet rest and eleven-body baseline exclusion. Full required workspace/native
results are recorded alongside after they run; do not infer success from code.

User feel/visual review, comparative recovery success rates, long performance/
Windows hardware and broad production acceptance remain open. Muscles may lose
against contacts or the existing global energy guard; a failed recovery is
intentional rather than hidden with a teleport. Parked props, world quota/layout,
saves/audio/traffic and unrelated motorcycle policy are unchanged.

The shared roll eligibility gates match Method A (16m/s COM speed,6rad/s spin).
B accounts exact positive angular impulse work; A uses a conservative motor work
estimate, so their numeric budget values are not directly equivalent.

Method B now includes an explicit fixed-support reaction guide (120Nm aggregate torque cap,
3.5rad/s target rate) for the roll attempt. Its positive work shares the same180J/180W
ledger and existing energy ceiling. It never writes linear motion/body poses.
The guide approximates reaction against the fixed supporting world; unlike the
internal joint impulse pairs it does not conserve closed-ragdoll angular momentum.
Upward core support, speed/spin eligibility, next-rotation obstruction checks,
timeouts and failed-progress yield govern it. HUD labels this support guide and
reports guide count/obstruction. This deliberate reference-style assistance is
the meaningful alternative to Method A strict native motors, not a hidden claim
of perfect anatomical actuation.

Measured limitation: in the reproducible fully-prone8m/s fixture, the final
controller started supported attempts and352 distributed guide corrections,
spent180J and improved torso supine score only from-1 to-0.9932 (about7degrees).
It did not roll onto its back. This prototype exposes a bounded attempt rather
than guaranteeing recovery; RAG-B-01 remains open for visual/user acceptance.
The support proxy requires real upward fixed-body contact plus core proximity
within8cm of the supporting surface; a wall or airborne ray alone cannot qualify.
Core spin controls eligibility; all limbs must quiet before resting.

Active corrections expire after the configured brace plus both attempt/retry
windows and1s (4.95s with defaults), including cheap slide damping. This returns
control to native passive tissue/sleep rather than repeatedly waking limbs.
Native recovery/bracing consumes the previous fixed tick's verified ground
support proxy; support clears on the next sample after contact loss.

For native recovery-state bookkeeping only, an otherwise motionless body may
rest on any actual upward fixed-ground segment contact (all parts below0.05m/s
and0.1rad/s). This does not enable an active roll on distal support alone.

Final automated verification: `cargo fmt --all --check`, strict workspace
Clippy, workspace tests (257passed,0failed,1pre-existing ignored), locked build,
`tools/audit.py`, roadmap sync and5roadmap tests passed. The paired headless lab
produced18finite rows; all9eleven-body control rows exactly match the research
baseline excluding wall time. B's maximum anchor error was0.003921m; all single
ragdolls slept16bodies/rested1 and fifteen-crash slept240/rested15. Raw receipts
are adjacent. Existing controls/crash/momentum/foot-finish/remount/reset tests
remain unchanged in their acceptance; actual three-hit/hidden-chassis fixes are
shared with A. F8/F9/F10 delivery and subjective gameplay remain user gates.

Final native smoke: normal, `--ragdoll-baseline`, and `--crash-comparison`
all exited0 after farm crash/remount, city/hills traversal, hill crash/remount
and race finish with four audio sinks initialized. Final logs/survey are adjacent.
Direct F8/F9/F10 delivery and subjective appearance were not verified: Wayland
X11 focus injection returned BadMatch. Automated state/backend tests provide
functional evidence; user side-by-side comparison remains required.
