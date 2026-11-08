# World repair session report

1. **Implementation:** static facade core-depth correction; wall closing-speed
   gate; animation knee planes; faster attacks/kick-facing/impact cue; ranked rival
   race pace; 24 route cars with downed-rider yielding/horns; narrow rural profiles;
   measured property/street/height plans and the confirmed **3D** art direction.
2. **Checklist:** eight scoped technical tasks verified. The larger world art,
   material rebuild, civilian skin seams, contact-timed combat and visual/performance
   exit gates are still open. No major development part is declared complete.
3. **Files:** game/animation/combat/recovery/sound and facade assembly/render/tests;
   rr_core pursuit/traffic; rr_content track validation; Valley track data;
   planning generator and plans; GPU range probe; README, progress, placement,
   decisions, history, issues, asset ledger and test evidence. No files removed.
4. **Research:** existing pinned Bevy 0.17.3 / Rapier 0.32 APIs inspected locally;
   no newly adopted GitHub project, copied library code or dependency migration.
   [Come Bye official gallery](https://store.steampowered.com/app/1282380/) and the
   supplied 3D screenshot are reference only; no game media imported. Existing MIT/
   Apache engine/dependency notices remain in the repository inventory.
5. **New task IDs:** WORLDPLAN-01, LOD-05, ANIM-KNEE-01, COLLISION-08, AI-09,
   COMBAT-09, ROAD-02, TRAFFIC-04 verified; WORLDART-01, NPC-SEAMS-01 and
   COMBAT-FEEL-01 open. Full reason/dependencies/acceptance/verification in README.
6. **Tests:** [command logs/native receipts and limits](README.md).271 tests pass,
   one existing ignored; formatting, strict Clippy, build, audit, roadmap and
   progress checks pass. Native Valley/actor/animation and both LOD fixtures pass.
   World geometry survey passes, but the 16.7 ms performance gate fails at 22.38 ms p95.
7. **Architecture:** ADR062 keeps facade cores static behind scaled authored
   surfaces, keeps animation independent of physics, preserves car-crash policy,
   adapts roster pace without rubber-banding and separates traffic cruise/actual speed.
8. **Known issues:** complete art/property layout implementation, civilian skin
   seams, prolonged streaming, contact-timed/approved combat feel, general traffic
   queues/intersections, Windows/hardware coverage and performance remain open.
9. **Next:** use the measured plans to validate property-wide fences/entrances,
   land use and height groups, then deliver coherent material/terrain/sky families
   with synchronized near/far assets. Correct NPC binds/weights. Profile before
   increasing scenery budgets. Keep long native high-speed/reverse review open.
10. **Git:** focused `codex/world-repair-plan` branch from894ed7a. Local commit
    reference is reported in the session response. No push, PR or merge claimed.
