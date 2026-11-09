# Sequential development passes — 2026-11-08

User instruction: one new fresh-context sub-agent for each Part01–07, strictly
one at a time, with primary review and a collapsed checklist after each pass.
User clarification: **Continue development passes; keep unmet boxes open.**
A reviewed development pass is not a completed milestone/exit gate. Human art /
feel, Windows/controller hardware, full reference parity and unfinished technical
requirements remain visible and unchecked. Never delete requirements to advance.

Each worker starts without inherited chat history, reads the current repository
and AGENTS, implements/tests/documents only its assigned part, commits locally,
and hands off exact evidence. Primary reviews before starting the next worker.
All branches build on the previous reviewed HEAD; the local glove integration
9fb55ca and merged pickup/reverse baseline d8d8876 are preserved. Nothing is
published or merged by this sequence unless separately coordinated.

| Part | Fresh-context agent | Reviewed implementation commit | Status / retained limits |
| --- | --- | --- | --- |
| 01 handling/crashes/runtime | part01_handling_crashes_runtime | 05e0d5633072895be6e7c1578e79840e6dbba064 | Reviewed development pass;323 tests/1ignored, native routes/baseline and18 lab cases. All29 umbrella requirements stay open. General vault mechanism, calibration and human/hardware/authority gates remain partial. |
| 02 animation poses/reference | part02_animation_poses_authoring | 931ac60 +71231c3 | Reviewed development pass;326 tests/1ignored, source/GLB/contact checks,138poses/41usecases and18authoring/33native hand views. All15 umbrella gates remain open for geometry/normal parity, specialist poses and art/clearance/caller approval. |
| 03 animation/interpolation/recovery | part03_animation_runtime_transitions | e1a5cde | Reviewed development pass;331 tests/1ignored,16 real-input contact/rate cases, moving-support tests, native routes/baseline/18labs and current hand provenance. All27 umbrella requirements remain open. |
| 04 buildings/procedural world | part04_buildings_world | a3022f3 | Reviewed development pass;332 tests/1ignored,24district facade families,13Python checks,12native views and both route recovery smokes. All29 gates open;21.08ms p95 misses16.7ms target. |
| 05 props/materials/dressing | part05_props_materials | 5a85c32 | Reviewed development pass;336 tests/1ignored,12broken meshes/six roles,13native impacts/reset and race smoke. All10 gates open; seven topple roles, close art and performance remain. |
| 06 NPCs/landscape | part06_npcs_landscape | 7b363f4 | Reviewed candidate pass;336 tests/1ignored,110human skins,source/pose/native checks. All11 gates open: individual seam regressions, art/clearance, animals/landscape and performance remain. |
| 07 weapons/combat | part07_weapons_combat | ff765f4 | Reviewed development pass;349 tests/1ignored, first club, queued pickup/drop inputs, native lifecycle/recovery and current hand/body captures. P05-003 verified intoPart00; ten broader gates remain open. |

Part01 review: inspected the contact-policy diff, velocity/normal classification,
solid-contact stagger interruption and both-side/phase tests. Primary identified
and worker fixed the parallel side-door case that velocity alignment alone had
misclassified as a rear impact. Reviewed receipts and ran roadmap5/sync/diff
checks. Broad tests/native runs were executed by the worker and archived in
[Part01 evidence](../testing/part01-runtime/README.md). The twelve real solid-car
trajectory cases deliberately retain failed vaults instead of inventing clearance
or adding arbitrary energy. No physical-rig or dependency migration.


Part02 review: primary inspected typed hand-state validation, socket bone-parent
and tail-frame math, source/runtime metre conversion and two actual static hand
renders. Primary requested a real socket-to-bike measurement rather than relying
on tiny export/reload error; worker found/fixed10.467mm source contact mismatch
with bounded existing IK and refreshed83 affected source views. Primary reran
source/native provenance checks, context7, roadmap5/sync/diff and headless static
catalog validation138poses/41usecases. Source frame/GLB errors are not silhouette,
normal/tessellation or user approval; angular contours remain visibly draft.
[Part02 evidence](../testing/part02-authoring/README.md).

Part03 review: inspected fixed intent/contact clocks, explicit extended key IDs,
confirmed-hit audio, duplicate/cancel/finished guards and physical-support-local
foot plants. Primary identified the punch chamber/contact distinction and asked
for real buffered-input timing and fresh native hand provenance. Worker corrected
these and retained native fixture failure evidence. Primary independently reran
authoring/hand provenance, roadmap5, progress sync and diff checks. Human impact
feel, full terrain motion/clearance and broader animation graph remain open.
[Part03 evidence](../testing/part03-runtime/README.md).

Part04 review: inspected district source/type rules, source-derived three-band
atlas and shared full/coarse dimensions. Primary reran three placement/mip tests
with the existing Shapely authoring interpreter plus roadmap5/sync/audit/diff, and
viewed street/aerial captures. Before/after world geometry equality is recorded.
Resident input roughly doubles and coarse RGBA triples;21.08ms survey p95 misses
16.7ms target with no controlled before/after comparison. Gray courts, simple
tower sides, atmospheric wash and extended streaming/art approval remain open.
[Part04 evidence](../testing/part04-world/README.md).

Part05 review: primary identified and worker repaired the Part03 live-strike
prop consumer regression, an intact/damaged visibility ordering race, and the
native fixture observing between those writers. Ordered final-state assertions,
normal/plain material readiness and bounded preparation waits now pass. Primary
independently decoded12variants/3576vertices/1632triangles with source identity
and collider checks, reran audit/roadmap5/sync/diff and viewed actual mailbox
frames. Medium-distance images do not establish detailed art approval. Native
13impacts/reset and race smoke pass; p95 phases18.41/19.38/20.88ms miss16.7ms.
411hidden roots/2626entities/902mesh instances include fixture cost; no rigid
bodies added. Seven topple types and broad art/performance remain open.
[Part05 evidence](../testing/part05-props/README.md).

Part06 review: primary identified wrist-field and pelvic-midline discontinuity
risks, requested side-specific compound3D and actual-native matrix measurements,
and independently passed six weight tests,110source/rest-surface audits and native
reanalysis. Native means walk8.08→6.98mm/impact9.40→6.94/ragdoll10.16→6.91;
individual mean regressions remain (8/6/10 native cases and830synthetic cases).
Primary inspected close walking: waist/belt, shoulders and foot clearance are
still draft. Catalog68.1→101.3MB and305828→777932triangles; eight cow files exact.
Same-binary old/new asset surveys miss16.7ms and do not establish improvement.
No physical-driver changes; candidate art remains unapproved.
[Part06 evidence](../testing/part06-npcs/README.md).

Part07 review: primary independently checked17weapon capture hashes,33hand
capture hashes,18authoring views/154states,context7,roadmap5/sync/audit/diff and
all original36keys/13clips/calibration against d825305 (now42/15). Review led to
safe malformed-clip validation, shared/frozen ability data, actual queued drop,
arm-only ready layering retaining lean/tuck, inherited visibility, below-point
support and pickup LOS. Raw club hand images establish attachment, not convincing
finger wrap or approved art. P05-003 alone is fully verified; no broad exit gate
was closed. [Part07 evidence](../testing/part07-weapons/README.md).

## Final development-pass report

1. **Implementation.** Seven fresh-context workers ran strictly one at a time,
   with primary review and completed-pass collapse between them. Integrated
   contact/stagger telemetry, shared authoring sockets, fixed strike timing and
   support-local foot plants,24district facade families/coarse crowns, six-role
   damaged furniture, a measured civilian skin candidate and first-club combat.
   Accepted physical rigs, maps, input-owned handling and recovery remain.
2. **Checklist.**206verified complete /242open /448stable IDs. P05-003 moved once
   intoPart00. Reviewed pass does not mean completed milestone: all other selected
   umbrella gates retain their unchecked status and exact remaining criteria.
3. **Files.** Exact added/modified/removed source/asset/evidence paths are in
   [the change list](PARTS_01_07_FILES.txt), baseline9fb55ca→implementationff765f4.
   Principal areas: rr_core contact/motion/weapons, rr_content motion/hand/world/
   weapon validation, runtime combat/animation/grounding/destruction/weapon code,
   original authoring/derived models, world palette/atlas and review/verification
   tools. Each part receipt links its source, tests and native evidence. README,
   PROGRESS, decisions, issues, history and provenance accompany implementations.
4. **Research.** Local code/locked engine APIs and existing original assets were
   used throughout. Pinned arkits/road-rash7aecf6044f2dd773b925b325823f30d648d61c89
   was studied for weapon side/frame/reach/latch behavior; project reuse license
   remains unverified. Classification REFERENCE ONLY; no reference code, retail
   media or new dependency imported. [Repository ledger](../third_party/REPOSITORIES.md).
5. **New tasks.** No new requirement IDs introduced. Existing task scopes cover
   the discovered repairs and unfinished acceptance. P05-003 is a status/placement
   update, not a new task. No unfinished task was deleted.
6. **Testing.** Final source:349tests passed, one pre-existing soak ignored;
   formatting, strict locked workspace Clippy, locked debug/release builds, asset,
   roadmap, progress and provenance checks pass. Native tests include both routes
   in earlier passes, actual club keyboard pickup/contact/drop, reverse, zero-error
   remounts, recovery/results, prop impacts/reset, NPC/animation views and18paired
   ragdoll cases. Raw failures/corrections are retained. Smoke finish is a results
   fixture; no full human race/fun, Windows/controller or broad GPU claim.
7. **Architecture.** ADR079–085 record bounded observations, shared hand recipes,
   fixed contact clocks/support-local visuals, district/full-coarse identities,
   ready damaged siblings, bounded skin refinement and frozen first-club actions.
   No physics-engine migration or visual overwrite of authoritative body state.
8. **Known limits.** General car vault behavior remains partial (one of12measured
   cases cleared); original numerical parity is uncalibrated. City art/extended
   streaming review remains open. NPC source art and individual seam regressions
   remain, with human derivatives68.1→101.3MB and305828→777932catalog triangles.
   Weapon handoff/grip, full ability schema, swept contacts, arsenal and human
   balance remain unfinished. Final bounded native wall-frame p9522.633ms misses
   the16.7ms target; no controlled whole-game performance comparison is claimed.
9. **Next steps.** Review the candidate NPC/weapon visuals and human combat feel;
   profile the remaining frame-time bottleneck and finish car-profile/clearance
   work before calling those gates complete. Landscape/animal/production art and
   remaining selected tasks stay visible. Part08police was not started.
10. **Git.** All seven source passes are committed on an inherited local branch
    chain; final implementationff765f4 is on codex/part07-held-club. The primary
    coordination commit follows it. Nothing in this sequence was pushed/merged;
    published main remains unchanged. Earlier branches allow pass comparison.

Run the integrated branch from the repository root:

```sh
cd /home/imad/Documents/ChatGPT/RoadRash
cargo run --release --locked
```

Default new bindings: E picks up a nearby visible club while riding; Q drops it
(after pending contact); mouse/J/K swings when equipped; Space remains kick.
Bindings are configurable; documented defaults do not imply hardware testing.
