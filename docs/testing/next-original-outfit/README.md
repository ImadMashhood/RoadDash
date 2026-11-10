# Original biker outfit restoration

User rejected the recent stylized clothing and requested the original biker
outfit. This narrowly reverses the rider presentation hunks of `b4dd5e72`, whose
parent `6039be4e` contains the requested prior outfit, on integration `f83cfbf7`.

## Exact boundary

- Restore original authored base, normal and roughness textures, original shirt
  palette (including player), PBR coefficients and authored smooth normals.
- Remove the runtime cloth partitions, replacement UVs, plaid vertex tint,
  rust-red override and forced flat-normal treatment.
- Retain helmets/visors, glove splitting/refinement, hand contacts, sixteen-joint
  skin articulation, accepted ankle bind correction, original asset bytes and
  current motorcycle. No physical body, collider, mass, joint, muscle, recovery,
  save, control or gameplay change.
- Keep curved wind grass, current prop/civilian treatment, every later world,
  lighting, AI and civic change. Police-specific appearance is separate work.

`src/models.rs` and `src/models/tests.rs` match `6039be4e` byte for byte;
`src/models/style.rs` is removed. No subsequent changes to those three files
existed between the rejected style commit and the integration base. Four-state
render assertions return to exactly20 sections (five per state), including exactly
five shared standing material handles, rather than28/seven from the removed
cloth partition. The physical assertions and differential trajectories/recovery
checks are unchanged. The removed test covered only the deleted partition code.

## Verification and outstanding gates

`python3 docs/testing/next-original-outfit/verify_boundary.py` passed and produced
`source-boundary.json`: exact prior source parity plus unchanged hashes for the
mesh, textures, bindings, skin articulation, hands, physical/recovery source,
props/world, wind shader and lockfile. `git diff --check` passed.

Cargo fmt/clippy/workspace tests/build, models differential tests and new native
captures were **not run in this worker**. The parent explicitly reserved shared
build/native slots for concurrent tasks and will run integrated checks and actual
before/after outfit views before acceptance. Existing historical native evidence
is in `../stylized-humans-grass/poses` (rejected style) and
`../polish-12/pass11` (prior outfit); neither is a new capture of this change.
No performance, Windows, broad hardware or final art approval claim is made.

## Root integration suggestions

Supersede only ADR119's biker clothing decision, retaining its grass and separate
prop presentation work. Update ART-04's partial progress in README and regenerate
PROGRESS together; keep ART-04 open. Record the user's rejection/restoration in
DECISIONS, CHANGELOG_DEV and KNOWN_ISSUES without erasing historical evidence.
No new task ID or checklist closure is needed: this corrects ART-04 presentation.
No new asset or third-party source was acquired, so no new asset license is added.
Existing rider texture/mesh provenance continues unchanged. Installed Bevy0.17.3
and Rapier0.32.0 remain pinned.
