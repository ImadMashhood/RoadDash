# Pass11 source and asset provenance

Internal investigation only: src/models/articulation.rs and tests.rs,
src/animation.rs, locomotion.rs, grounding.rs and preview.rs; rider-bindings.json,
motion.json, original riding.rdmesh, hand contacts, ADR051/080 and prior motion,
animation/recovery/civilian deformation evidence. Installed Cargo.toml/lock retain
Bevy0.17.3/Rapier adapter0.32.0/Rapier0.31.0/wgpu26.0.1.

No external repository, implementation, model, image, audio or dependency intake.
Original rider/bike meshes, textures, joint weights and authored animation files
are unchanged. Runtime-generated skin bind matrices/inverse binds change together.
No asset ledger hashes change. Native images are actual application captures.

The 0.5580462m ankle distance is from the existing original motion.json foot.L/
foot.R parent offsets, not a new proportion or physical-body specification. Tests
compare both sides to those source offsets independently (sub-micrometre rounding).
The11body and16body physical pivots and all their physics properties remain intact.
