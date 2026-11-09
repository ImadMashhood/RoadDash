# Part07 first held club development pass

2026-11-08 · `codex/part07-held-club` · reviewed parent `d825305`.
This pass implements one functioning original training club in the race. Broader
weapon families, steal/backhand, art, balance and exit gates remain open; Part08
has not started. ADR085 and the R01 follow-up record scope and research.

Players press **E** near an unobstructed club to equip it, use the existing
mouse/J/K side attacks to swing, and press **Q** to drop. Keyboard actions are
remappable in the binding file; old configurations retain their bindings and
receive unused fallback keys if E/Q are occupied. The current settings panel and
physical-controller pickup/drop workflow remain future work. Space kicks keep
their frozen leg selection and do not transfer the held club. A pickup edge arriving during pending contact is rejected; press again after
contact. A drop edge queues once until contact completes without changing the
strike snapshot. Crash/reset clears that queue. Crash/stagger/finish cancels the strike and drops the club.

Six route pickups reset with the race. Players and rivals share pickup distance,
solid occlusion, ability reach, stamina, cooldown and visible windup/contact.
Racers retain attacker-specific retaliation; killers retain their existing policy.
Each contact still contributes one direct hit to the existing three-hit chain.
No added knockoff energy, car damage, destructible bodies or physical-rig tuning.
Prop/NPC/rider strikes keep the shared fixed event path. The weapon is immediately
resident procedural geometry, so there is no unloaded invisible hit advantage.

The club is an independently authored three-piece dark training baton with a
metal end cap on the existing typed +Y hand socket. It reuses resident cube/dark /
metal assets. Both hands have prepared hidden render attachments; only the
selected held side inherits the rider's visibility. Only the selected arm/hand
is layered over the current normal/tuck/turn base between attacks. Full authored
club attacks return to that latest base. Grip morph weights are active around the socket; the angular glove and
protruding fingers do not yet establish convincing wrap or clearance. There is
no punch-fist substitution, standing wrist twist or physical driver writes.
Instant side-to-side handoff, finished silhouettes and moving clearance are
explicit prototype art limits.

`motion-preservation.json` proves the append-only conversion:36 old keys,
13 old clips and every old field/hierarchy/offset/limb remain deeply equal;
six actual original club keys and two clips are added. The conversion script
executes only the existing frame-conversion prefix, never the unsafe full export.
The contact source key is2162/2172 at.32s; gameplay uses the retained.055s entry
and1.8 playback. These are authored RoadDash timings, not native-game parity.

## Verification and limits

See RESULTS.md for actual commands, native outcomes, costs and outstanding gates.
The matched-speed harness exercises real input capture, pickup, drive, scheduler,
authored contact pose and melee at30/60/120/144Hz, including zero-fixed-tick render
frames. It is deterministic fixture evidence, not human difficulty/fun approval.
Tests also cover LOS/immunity/side/range, mutable tuning and captured abilities,
low stamina, interrupted/drop/reset, fair rival pickup/tells/retaliation,
under-roof/elevated support, unchanged lean/tuck and hidden-owner inheritance.

Ground drops preserve liveX/Z and raycast downward from the live drop point.
If fixed support cannot be found within12m, clamped terrain height is the fallback.
Clubs are static pickup markers, not simulated thrown weapons. New drops are
discarded when the12-root ground cap is full. Six initial
pickups and at most12 ground roots are bounded independently of rider equipment.
Fifteen riders have at most30 prepared held roots/90 part instances; only one
held root per equipped rider is shown. Maximum roots/children:168 extra entities,
126 mesh instances; maximum visible geometry:27 clubs/81 part instances/972
triangles, using existing shared24-vertex/12-triangle cuboids. No new GPU mesh /
material assets or physics bodies. Hidden resident entities still cost memory /
traversal; this is not a performance-free change. Native counts and frame samples
are reported separately; a short scripted smoke does not establish60FPS.

## Per-task dispositions

| ID | Disposition |
| --- | --- |
| V2-07-001 | Partial: pinned behavioral study and original first-club data/asset; full weapon families remain open. |
| V2-07-002 | Partial: club pickup/drop/attack/contact/outcome/interruption integrated; holster/equip choreography, bespoke audio and broad family/clearance acceptance remain open. |
| V2-07-003 | Partial: shared club pickup/timing/reach/cost and exact retaliation tested; defense/personality tuning and human difficulty review remain open. |
| V2-07-004 | Open: no broad family/traffic/slope/performance/human exit approval. |
| ARCH-004 | Partial: first club authored; complete weapon schema/catalog and bike data work remain open. |
| RIGHTS-011 | Partial: scoped reference/provenance/timing/range tests; full session/deliverable acceptance remains open. |
| P05-003 | Verified in Part00: left/right kicks plus first held club, actual native keyboard pickup/contact/drop and fixed input/range/AI/crash/reset regressions. This closes only the narrow first-weapon requirement. |
| P05-004 | Partial: source-contact/reach/stamina/cooldown/pickup authored; complete active/recovery/damage/stagger/impulse ability schema remains open. |
| P05-005 | Open: existing single-side volume/contact remains; swept sensors and rich result records are not implemented. |
| P05-007 | Partial: moving side-by-side/input/pose/whiff/tuning regressions; full combat harness presentation and exhaustive boundaries remain open. |
| P05-008 | Open: three-minute human unarmed enjoyment/feel gate was not run. |

No new task IDs, external media, dependency migrations, push or merge.
