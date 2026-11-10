# Pass12 — Confirmed punch/kick reactions

Final of exactly12 sequential implementation agents. Worktree RoadDash-polish,
branch codex/gameplay-graphics-polish; previous implementation1ebf2b11dc2a60a706f8a8b9294c40dcad201850.
Parent handoff-only recording commit/base HEAD:a2fb8b045153890b1fcfd80de65c235bf52355f9.
Original RoadRash UI changes untouched. Parent reviews/commits; no push/merge.

## Change and boundaries

Previously successful punches1/2 had sound/camera/HUD feedback but no victim
pose response, while kicks only applied their established lateral push. Confirmed
mounted melee now records one fixed race-clock pulse: punch0.30s, kick0.38s,
50ms peak, smooth return. Victim-local direction mirrors the spine/chest and
head response; shoulders inherit the chest chain. Kick folds the trunk more,
punch emphasizes head recoil. No additive stack, animation lock or hit-stop.

The rider's current base pose continues updating; active strikes have priority
so flinch never moves their contact key. The pulse keeps aging during that attack.
Held hands/feet use existing fixed-length fitting; root/pelvis/feet and physical
transform/velocity/control are untouched. Third punch removes this ordinary pulse
and follows existing stagger/knockoff; kick still pushes8m/s without hit damage.
Recovery/finish/inactive race/reset clear the pulse and pause freezes its clock.
Misses/obstruction/immunity produce no pulse, hit audio or hit camera cue. No block
mechanic exists, so none is invented. Sound and restrained camera amplitudes
remain the existing confirmed-contact values.

## Verification

All mandatory checks pass: fmt, strict locked workspace/all-target Clippy,
439tests (one pre-existing ignored), locked debug build, audit, five roadmap tests
and progress synchronization. Eleven model tests include both180tick differential
trajectories with maximum position difference0 and full crash/run/remount cleanup.
Focused reaction tests cover both sides/types, one-shot replacement, miss
silence/camera, existing kick impulse/punch counts, pause/recovery/reset, exact
active attack key priority, unchanged physical transform/velocity, held contacts
and chain lengths. Fresh release build58.08s. All six serial native runs exit0 with116nonblank
screenshots, frozen-input hashes unchanged and no errors/panics. Both rigs confirm
all four hit directions/types (40reaction frames), then complete both route
recoveries/results. Dedicated recovery runs retain reverse−1.898m/s, both pickup
errors0.0000m, grid retry and5audio sinks. Both human/cow actor smokes pass.
All18paired lab cases remain finite; biomechanical15crashes sleep240/240bodies.
See [native summary](final-summary.json) and [commands](native-results.json).
Initial failed tests are retained: backward punch torso pitch caused10.83mm grip
reach loss specifically in the acceleration pose. A small forward pitch preserves
lateral/head recoil within reach; the final test retains20µm contact tolerance.
The other initial failure compared quaternion bits; equivalent normalized rotations
now use dot-product comparison, as existing contact tests do.

Native reactions use the real melee resolver with temporary logical attackers;
this isolates victim output and bypasses windup. Existing real buffered keyboard
fixtures verify contact timing at30/60/120/144Hz. Do not confuse scripted captures
with a human fight or a full visible attacker-victim geometry approval.

## Cost

An isolated opt-level1 CPU test sampled100000 active layers seven times:
median116.52ns (113.26–119.31ns). The optional pulse occupies12bytes per Rider;
no new systems, meshes, materials, lights, uploads or draw calls. This does not
measure GPU time, full ECS/cache cost or a crowd frame budget. [Receipt](cost.json).

## Tracking and gates

ADR116; COMBAT-FEEL-01/P05-006 partial evidence extended. No new IDs, task moves,
closures or count changes:220verified/245open/465IDs; TASK_PLACEMENT unchanged.
Full contact/body clearance, human impact/audio/balance review, weapons/foot-state
coverage, production distal skin, crowd/GPU/frame-budget, Windows/controller and
hardware gates remain open. Existing sky cloud-motion/civilian seams/LOD art gates
remain open from prior passes. No automatic roadmap advance after12.

[Provenance](PROVENANCE.md). No assets, audio PCM, physical rigs, dependencies,
collision, recovery, controls, save or race policy changed.

## Native visual review

Actual gameplay captures show left/right head/chest response with the rider's
feet/grips still connected. Reviewed anatomy reaction70 offset0/4/25 (neutral,
peak and returned pose),110/150 offset4, and baseline190 offset4. The fourth
fixture coincides with existing held steering and kick's established lateral
push; that bank is not visual recoil steering the bike. Captures are not matched
static background frames because the race and camera continue moving.
Each rig captures four real confirmed impacts at five render offsets (40total).
No visible attacker is fabricated: temporary logical attacker use is disclosed.
Native feeling/audio correctness still needs a human fight and listening review.
