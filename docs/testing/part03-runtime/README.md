# Part03 — runtime contact timing and support lifecycle

Development pass2026-11-08; ADR081. All27 unfinished Part03 requirements stay
unchecked. No new requirement IDs, task moves, dependency/runtime-asset changes, physical
rig tuning, push or merge. This is an integrated improvement, not full Animation B
or user motion/feel approval. Part04 is not started.

## Implemented behavior

Live player/AI drive emits AttackIntent after its existing cooldown/stamina check.
The fixed scheduler owns a Rider-local clock and emits one StrikeContact. Direct
AttackAttempt fixtures intentionally bypass windup to isolate geometry/rules;
live input does not use that bypass. Pending duplicate/replacement intents are
ignored. Pausing does not age the clock; stagger, recovery, finish, inactive race
phases and rider reconstruction clear pending strikes. Finished riders cannot
attack or receive new live melee contacts.

The contact IDs are explicit, validated against the shipped runtime clips:

| Action | Runtime extended/contact key | Source time | Entry/playback | Contact threshold | First fixed contact |
| --- | --- | --- | --- | --- | --- |
| Left/right punch |2222/2232, index2 |.26s |.055s /1.8x |.199444s |tick12,.200000s |
| Left/right kick |2261/2271, index1 |.24s |.055s /1.8x |.188333s |tick12,.200000s |

Punch index1 is the chamber, not the extended contact. rr_content derives the
threshold from prior phase durations and rejects missing contact IDs or unequal
mirrored timing. These are RoadDash authored timings, not newly recovered original
logical-frame durations. The renderer samples the same fixed age; render
interpolation remains one fixed snapshot behind at alpha0 and does not write
physical transforms. Audio/shake remain confirmed-hit effects. Current range,
LOS, immunity and eligible targets are evaluated at contact, so a target escaping
during windup legitimately misses. Three direct hits with at most3s between
successive hits, racer retaliation against the actual attacker, killer/NPC rules,
stamina, cooldown and kick push are preserved.

Kick windup selects one leg using the nearest eligible rival or NPC within12m,
with deterministic entity tie-break. That side stays fixed through contact; a
rival crossing through the rider cannot flip the animated leg or be hit from the
opposite-side strike volume. Left/right no-target fallback remains existing policy.

Visual foot plants now retain collider-local points/normals using current Rapier
collider frames, including parented/kinematic support movement. They ray-check
continued support and reject removed, steep, unreachable or lost supports. A new
stance always captures a fresh sole contact. Swing decay clears below.02 rather
than waiting for unreachable exact zero. This corrects stale prior-stride plants;
no physical capsule, pelvis or body is translated to satisfy animation.

## Verification and limits

`buffered_strikes_wait_for_contact_and_resolve_once_at_all_render_rates` uses real
ButtonInput -> pre-fixed buffer -> drive -> scheduler -> contact/LOS -> NPC hit.
It covers both punches and both kick legs at30/60/120/144Hz, including zero-tick
render frames. It requires zero damage before threshold, one result afterward,
and compares the actual Animator rotation sample with the extended runtime key.
It does not inject an already-resolved attempt. Render-observed first-contact
age can exceed.2s on a30Hz frame containing two fixed ticks; that is observation
quantization, not a delayed authoritative event.

The windup test also verifies frozen kick side, pause age equality, ignored
replacement intent, stagger cancellation and recovery cancellation. Existing
input/reset, three-hit/expiry, retaliation, LOS, foot-finish, reverse, live pickup,
100crash/reset-cycle and src/models/tests.rs physical/recovery differential
regressions remain required. No existing expected damage is faked at input;
older integrated melee tests now wait twelve fixed ticks for real contact.

Terrain tests run the grounding adapter against a real Rapier kinematic support
at60Hz, stride3.2m, declared walk velocity1m/s and explicitly selected stance
phases. They verify finite swing release, new plant after.3m runner movement,
.2m support translation/.1m lift, support removal and steep-support rejection.
A one-tick swing retains weight above.5 before a new stance; reentry still captures
the new point. This is adversarial phase skipping, not a claimed supported sprint
speed. Getting-up double support transitions to walking with the swing sole
released. A separate120-transform translation/rotation fixture bounds local-lock
round-trip drift below1e-6m. Existing anatomical IK tests preserve segment lengths.
These fixtures do not measure full native foot-slide/pop distributions on
stairs/curbs/turns, body clearance or pelvis reach; V2-03-003/006 remain open.

The ordinary native animation lab cycles eleven motions/three views. It directly
increments attack serials and exercises the visual fallback clock, not the new
live scheduler. The optional native smoke extension uses queued KeyboardInput
J press/release, observes actual fixed windup/contact and captures the next
rendered interpolated frame. It checks first-contact age within one60Hz tick of
the threshold, not subjective impact feel or a full human fight. Its first
iteration at frame100 collided with the older REPAIRS fixture's manually held
attack timer and correctly failed the lifecycle assertion; the keyboard probe now
runs at frame60 before those visual fixtures. The failed experiment is retained
as native-fixture-initial.txt; final receipts are separate.

## Part03 dispositions

| IDs | Disposition after this pass |
| --- | --- |
| COMBAT-FEEL-01 | Fixed contact/pause/interrupt/side contract integrated; impact/audio/user playtest approval open. |
| V2-03-001, P05-006 | Contact clocks/event dedup extended; complete priority/state/weapon/police/locomotion graph and approval open. |
| V2-03-002 | Existing clip interpolation retained; complete approved keyframe/transitions/vault coverage open. |
| V2-03-003 | Stance release/replant and support-local tracking repaired; full terrain/pelvis/stair/turn/pop/slide acceptance open. |
| V2-03-004 | Existing fixed-length bike contacts preserved; runtime deformation/clearance approval open. |
| V2-03-005, RAG-B-01, RAG-B-02, RAG-B-03 | Accepted velocity muscles and passive baseline retained; no native-motor candidate integration or new work/energy tuning. Remaining prototype/integration/tuning gates open. |
| V2-03-006, P06-009 | Existing physical-pose get-up/live pickup preserved; direction/obstruction/clearance acceptance open. |
| V2-03-007 | Confirmed contact feedback follows the delayed result; broader breathing/look/contact-audio/performance polish open. |
| ANIM-09, RAG-05, ART-02, ART-04 | Existing source models/rig preserved; production deformation/distal NPC skin/final multi-angle/user/hardware approval open. |
| RAG-06 | No broad race performance approval; existing physical differential fixtures and lab remain regression checks. |
| P11-008 | Runtime human improvements partial; production rig/animal animation/safe cliff recovery open. |
| FOOT-003, FOOT-008, RIGHTS-012, P06-008, P06-010, P06-012, P06-013 | Existing manual/assist/foot-finish/reset/recovery regressions retained; obstacle/cliff/trapping/interaction/full polish and deferred police outcomes open. |
| V2-03-008 | Full motion/user/performance exit gate open. |

## Research, provenance and reproduction

Only local repository code, installed locked Bevy/Rapier sources and existing
context/source-phase catalogs were inspected. No external repository, media,
asset, copied implementation or new dependency was adopted. Existing source
reference/license notes remain in docs/research/ANIMATION_AND_RAGDOLL.md and
docs/third_party/REPOSITORIES.md. No motion.json, source GLB, bind, source pose,
hand profile, audio PCM or map changed. Registered hashes are refreshed only for
the actually changed NPC test source and newly captured native hand provenance.

The old hand gallery source hashes initially became stale after src/animation.rs
changed. A fresh current-release --hand-review native run recaptured all33 views;
package_hand_review.py and --check now pass with the actual new capture hashes.
The registered provenance hash is refreshed; no old image was relabeled without
recapture. The original versions remain in git history. These held-pose views do
not validate scheduler timing. Static authoring hand provenance and source/runtime
coverage also remain independently current. New runtime contact captures and
source/binary hashes belong to this receipt; none certifies final finger/normal/
tessellation art parity.

Commands and final results are recorded in [results](RESULTS.md). Native graphics
use the supplied DISPLAY=:0 environment. Scripted Linux/Vulkan graphics/audio,
a discovered gamepad with a default-mapping warning, and test counts do not imply
Windows/controller hardware, human race acceptance, legal clearance or production
performance certification. No performance improvement is claimed.
