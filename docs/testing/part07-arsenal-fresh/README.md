# Part07 arsenal development pass

2026-11-09; branch `codex/part07-arsenal-fresh`. This advances the ten open Part07
requirements without closing human/art/production exit gates or starting another part.

The original training club, chain, whip, rolling pin and baseball bat now have
stable domain identities, authored validated family abilities, seeded rival
loadouts, typed route pickups, frozen contact snapshots and identity-preserving
manual/crash drops. Existing E/Q/side attacks and kicks retain their input path.
Route pickups cycle through all five families; default armed rival count remains
seven of fourteen. Retry rerolls families only with the existing new-grid event.
Police and the player start unarmed as before.

| Family | Cost | Cooldown (s) | Lateral reach (m) | Longitudinal reach (m) |
| --- | ---: | ---: | ---: | ---: |
| Club | 300 | .85 | 2.55 | .90 |
| Chain | 340 | 1.00 | 2.85 | 1.00 |
| Whip | 260 | 1.10 | 3.00 | 1.10 |
| Rolling pin | 220 | .70 | 2.10 | .75 |
| Baseball bat | 380 | 1.15 | 2.75 | 1.00 |

Every attack still contributes exactly one direct hit to the accepted three-hit
rule; this is original bounded tuning, not recovered retail behavior or human
balance approval. All families currently share the original club side-swing
silhouette and source contact .32s, .055s entry and 1.8 playback. Flexible chain/
whip motion, specialist animations, holstering/stealing, clearance/art approval,
dynamic weapon LOD, broad performance and subjective fairness remain open.

Original existing GLBs now automatically match the inventory family on ground
and both prepared hand sockets. Scene changes affect presentation children only,
retain resident geometry until readiness, and discard old scene children when
switching family. A stale scene-ready callback cannot hide the current fallback.
The optional ROADDASH_WEAPON_MODEL remains an explicit presentation override.
The HUD names the actual family. No physics body, joint, mass, motor, crash policy,
save format, input bindings, audio, world geometry or third-party asset changed.

Content schema2 adds the family fields; schema1 remains accepted with original
club defaults for missing families. Unknown versions, nonfinite/out-of-bounds
abilities and motion/contact mismatches fail validation. Club enum identity is
retained; no existing profile field stores runtime weapons.

Native fixture: use existing `ROADDASH_VERIFY_CLUB=1` and
`ROADDASH_VERIFY_STRIKES=1`, optionally set
`ROADDASH_VERIFY_WEAPON_FAMILY=chain|whip|rolling_pin|baseball_bat` (default club).
The fixture uses actual keyboard pickup/attack/drop and checks family identity
and frozen family reach during the native strike.

Research: inspected existing ADR085/092/097, Part07 evidence, licensed original
model catalog and code. This pass imports no external code/media or new package.
The existing pinned R01 observations remain research-only; the family numbers
above are independently chosen game tuning, not claimed historical facts.
Test outcomes are recorded in RESULTS.md after execution.
