# Pass09 handoff

Work only RoadDash-polish / codex/gameplay-graphics-polish. Previous commit
75ccf0dc270d1f7166b1e50b3c77cdc0a61f378f. Parent reviews/integrates; no commit made.

Built-in exponential-squared haze replaces linear180–1050m opacity. Density
0.00075/m retains99.44% contrast100m,98.19%180m,53.79%1050m,16.16%1800m. More
resident far buildings remain visible. rr_content validates finite colours and
0.0004–0.001density, preserving>99%contrast100m throughout accepted range.
Shared horizon_color_srgb stays0.68/0.76/0.82; fog converts it to linear and
scales by sky_nits*Exposure::exposure. Sun/coolfill/EV12.7/Reinhard unchanged.
No runtime geometry/material/atlas/visibility/collision/simulation changes.
No textures, passes, volumes, lights or directional scattering added; only the
existing fog arithmetic changes. Original source provenance hash updated.

CRITICAL pass10 handoff: equal radiance does not yet mean identical display
horizon. Current NON-HDR PBR applies Reinhard in shader; skybox writes exposed
colour directly and clips. Trial of installing bevy::render::view::Hdr on chase
camera yielded ALL BLACK native screenshots throughout survey00–13 despite
exit0 and clean shader/pipeline logs. Rejected; exact renderer failure not solved.
rejected-hdr contains proof. Final Hdr component is REMOVED; fresh supported
release recaptured normally. Native runner now rejects blank/missing images and
records frozen-input hashes. Existing V2-06-007 keeps sky/horizon integration open.

For10 inspect installed Bevy0.17.3 registry:
bevy_pbr/src/render/pbr_functions.wgsl (exposure then fog then TONEMAP_IN_SHADER),
bevy_pbr/src/render/fog.wgsl (exponential-square blend),
bevy_core_pipeline/src/skybox/mod.rs (CPU brightness*exposure),
bevy_core_pipeline/src/skybox/skybox.wgsl (direct sampled output),
bevy_core_pipeline/src/tonemapping/{mod.rs,node.rs} (core plugin present,
Reinhard requires no LUT). Consider coherent NON-HDR sky output with the same
analytic operator/profile; avoid inverse-fog hacks. Sky shape/palette/cloud
motion and existing drifting sun disc remain yours. Preserve shared horizon
configuration/exposure relation and readable near contrast.

Final native metro/recovery exit0, frozen inputs, clean error logs, nonblank
captures. Reverse−1.898m/s; both pickup errors0.0000m. All14detail draw audits and
resident census identical before/after. Matched farm00/entry03/aerial10 inspected;
far layers improved, near riders/traffic retained, residual seam explicitly open.
Whole-app wall18.984avg/35.355p95→18.236avg/32.766p95ms, both miss16.7. Single
sample, no causal speedup/GPU/hardware affordability claim. Black HDR timings
excluded. Original non-HDR intermediate candidate retained separately.

ADR113; README/PROGRESS synced. No new IDs/state/placement/count changes. SKY-01
baseline extended; V2-06-007/full landscape/art/hardware/performance gates open.
No external repository/media/dependency intake. No whole-rig lab sweep because
physics unchanged; mandatory tests include model differential recovery fixtures.
Stop09. Proposed commit: feat(render): add near-readable atmospheric depth

Final mandatory fmt/strict locked alltarget Clippy/430tests(one existing ignored)/
locked build/audit/five roadmap tests/progress all exit0; locked release passes.
git diff --check clean. Git9tracked modified+pass09evidence directory untracked.
No commit/push/merge. All receipts final in checks.json; source hash audit fixed.
