# Pass12 handoff — STOP after final requested pass

RoadDash-polish / codex/gameplay-graphics-polish only. Previous implementation
1ebf2b11dc2a60a706f8a8b9294c40dcad201850; parent handoff-only base
a2fb8b045153890b1fcfd80de65c235bf52355f9. Original RoadRash UI untouched; shared
build cache used with explicit CARGO_TARGET_DIR. No subagents or commits made.
Parent owns docs/testing/polish-12/final; exclude it from this pass's commit.

Added one replaceable race-clock pulse on confirmed mounted melee. Punch0.30s,
kick0.38s/50ms peak; signed victim-local direction, small spine/chest rotation,
head/shoulder response, smooth return. Upperbody only: no root/pelvis/feet or
physical writes, and held grips/pegs use unchanged IK. Active attack has priority
(no contact-key distortion or queued input); pulse ages under it. Third punch
clears ordinary pulse into original stagger/knockoff; kicks keep8m/s push and no
hit damage. Miss/LOS/immunity rejection cannot create pulse/audio/shake; no block
mechanic exists. Existing sound/shake remain confirmed-contact events unchanged.
Schedule clears expiry/recovery/finish/inactive/reset; pause freezes race clock.

Initial failed test caught10.83mm grip reach loss in acceleration caused by
backward punch torso pitch. Corrected to small forward pitch while retaining
lateral/head recoil; strict20µm held-contact tolerance remains. Initial exact
quaternion comparison changed to orientation-dot test for normalization noise.
Negative/diagnostic receipts retained; no failures concealed. New5tests plus
existing434 pass =439passed, one pre-existing ignored. Includes bothside/type
hit/miss/replacement/lifecycle/unchanged8m/s kick/directhit rules, active attack
key/authority neutrality and held chain contacts. Both model180tick trajectories
maxpositionerror0; all11model tests and full recovery differential pass.

Mandatory fmt, strict locked workspace/alltarget Clippy, locked workspace tests,
locked debug build, audit, five roadmap tests and progress all pass. Fresh release
58.08s. Six serialized native runs after all compilation: bothreaction smokes,
both dedicated recovery, both human/cow actor; all exit0,116nonblank images,
frozen hashes unchanged, no errors/panics. Eight actual confirmed melee events
produce40reaction captures. Native fixture uses real melee/LOS resolver but
invisible temporary logical attackers, bypassing windup; existing real keyboard
30/60/120/144Hz tests independently cover contact clock. Do not claim a human fight
or visible attacker/victim contact approval. Dedicated native recovery maintains
reverse−1.898m/s,bothpickup0.0000m,gridretry,5audio sinks,route0/1results. All18lab
cases finite; anatomy15crashessleep240/240. Only known Vulkan/gamepad warnings.

Reviewed native anatomy70offset0/4/25,110/150offset4 andbaseline190offset4: visible
mirrored head/chest response, connected hands/feet, return to riding pose. Fourth
hit overlaps existing steering and original kick push, demonstrating continuing
input; bank is not visual authority. Original clothing/faceted seams and wider
art/clearance/human sound/impact/balance/hardware/performance gates stay open.

CPU active-layer microbenchmark7×100k, optlevel1: median116.52ns,min113.26,max119.31.
Optional pulse12bytes/Rider; no new systems/draws/meshes/lights/uploads. Not ECS,
GPU or broad framebudget certification. All18registered physical/model/content/
Cargo files byte-identical to base; pass11ankle fix and previous passes preserved.

ADR116; README COMBAT-FEEL-01/P05-006 and generatedPROGRESS,KNOWN_ISSUES,history,
provenance updated. No newIDs/closures/moves/countchanges:220verified/245open/465.
TASK_PLACEMENT unchanged correctly. No external research/media/dependencies.
Changed runtime: crates/rr_core/src/motion.rs,src/game.rs,src/combat.rs,
src/animation.rs,newsrc/animation/reaction.rs; opt-in src/verification.rs fixture.
README/report/receipts in pass12; git-status.txt captures exact working state
including parent-owned final dir. No push/merge or next roadmap work.

Proposed commit: feat(combat): add bounded confirmed-hit rider reactions

Parent next: review actual diff/evidence, commit pass12, record SHA in root
handoff, run prepared final integrated regression. Stop after12 as requested.
