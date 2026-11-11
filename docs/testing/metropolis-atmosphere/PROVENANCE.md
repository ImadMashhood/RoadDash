# Atmosphere source and rights notes

Original RoadDash bird geometry, procedural volume density and WGSL renderer are
reused; no external media or source code was imported for issue30.

Reviewed local installed sources: Bevy0.17.3 `bevy_camera/src/visibility/range.rs`
for per-mesh VisibilityRange, GlobalTransform and dither behavior; the existing
cloud Material/ShaderType bindings and shader preprocessor. Bevy is MIT/Apache2.
Cargo.lock remains on Bevy0.17.3/Rapier0.32.0.

Reviewed user-requested skill repositories: `chrisgliddon/bevy-skills` (MIT),
`gamedev-skills/awesome-gamedev-agent-skills` (Apache2), installed under
`/home/imad/.codex/skills`. Shader coordinate-space/sample bounds, procedural
determinism, measurement-first budgeting and driving sightlines informed the
review; examples were not copied. Bevy0.20 recipes are migration notes only.
No proprietary assets or research-reference content was copied. This records
source intake, not legal clearance for the broader game.
