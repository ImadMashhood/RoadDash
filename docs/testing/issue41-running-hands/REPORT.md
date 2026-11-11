# Running hand orientation — issue #41

Status: implementation in `codex/issue-41-running-hands`; broad performance
and human acceptance pending.

The user reports flat, rear-facing palms and thumbs that fail to point forward
while running. The 2026-11-10 urgent run already uses a distance-matched visual
gait. The standing key's local thumb axes are left +X and right -X. With the
existing upper/forearm swing, one hand's thumb can rotate behind the body at
each half-cycle. This change computes each current hand frame from the authored
chest/arm chain, then applies a bounded (at most 0.7 rad) world-forward wrist
flexion weighted by run speed. Stop/idle keeps the standing hand bind. It changes
only `Pose.rotations[9/13]`, so it does not write physical bodies, joint motors,
controller displacement, root motion, saves or controls. It adds a few quaternion
operations per moving character; measured runtime cost remains open.

The new full-cycle test checks both thumb vectors retain a forward component on
240 phase samples, while existing joint position/length and stop-continuity
checks remain active. The first test failed because the world-X correction had
the opposite sign; that failure was retained in tool output and corrected.
`cargo fmt --all --check`, strict workspace Clippy, all 617 workspace tests
(two ignored), `cargo build --locked`, and `python3 tools/audit.py` passed.

The native animation review ran on flat and uneven terrain with isolated saves
and settings. Both cases exited successfully and reported eleven motions in
three views; each produced 153 frames in ignored `output/testing/issue41-running-hands`.
Selected [flat side](flat-side.png), [flat front](flat-front.png), and
[uneven side](uneven-side.png) frames show the fist shape and forward thumb cue
without a visible wrist gap. These captures are samples, not full-cycle skin or
human-feel approval. The same frozen debug binary SHA256
`331ac0015bbc42eaa9ffc2cfbb90c7cb0ea696a4365e10b75bbdd6da6de1a388`
passed paired native manual-return/walk-crossing recovery fixtures with the
anatomical default and `--ragdoll-baseline`, including all fixed recovery stages
([receipt](recovery-receipt.json)). These are scripted Linux cases, not human
motion or hardware acceptance. Broad performance and human acceptance remain
open.
