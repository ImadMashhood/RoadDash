# Pass07 — Building material consistency

2026-11-09, `codex/gameplay-graphics-polish`, previous commit
`3bdd9fb668718501cec6fd5e32cceb0dc7c77cf7`. ADR111; MAT-03 evidence extension.
No new task IDs, moves or checkbox closures. Human art/long-streaming/hardware
and broad performance gates remain open. Parent reviews and commits this pass.

The24 currently placed facade families now share an offline finish policy:
restrained masonry at94% original linear factor, subtly neutralized concrete,
dielectric painted metal, restrained metallic gold trim, and slightly lighter
opaque glazing/interiors. Existing pale surrounds stand out against masonry.
Original geometry, architectural openings, signage, source hues and texture tiles
remain. This is a measured material pass, not new ground-floor architecture or
approval of the full catalog. No proprietary or newly imported media.

`facade_material_policy.py` is consumed by the normal front compiler and the
material-only refresh tool.72front and72storey GLBs have new material headers;
`geometry-parity.json` proves identical BIN bytes and every nonmaterial JSON
field against the starting assets. Full source GLBs/blends, all other families,
collision/envelopes/rights of way, Valley quotas, props, roads, rig, controls,
saves, audio, recovery and race behavior are unchanged.

The far baker previously ignored runtime texture-profile roughness replacement.
It now uses the same provenance-linked profiles and exact quantized micro-map;
shared Rust/Python golden fixtures verify actual generated values. All383source
bands,15mip images and source/output hashes are regenerated. The runtime content
validator requires a bounded profile-input digest. Distant geometric normals
still omit subpixel micro-normal relief; flat proxies retain parallax limits.

Metropolis previously used albedo-only crops with uniform roughness0.84 and
metallic0. It now samples a linear packed G=roughness/B=metallic atlas at the same
24family/three-band/five-mip coordinates, including BUSS4's upper-band fallback.
Crop tests check every albedo and packed byte against source atlases. Runtime
tests check linear format, full mip payload and wrapped gutter pixels. Added
pixel payload:6,285,312bytes; one additional sample in the existing material,
no new mesh/draw/light/transition group. No GPU allocation/timestamp claim.

## Reproduction and evidence

Run serially, with no compiler, authoring job or other native game overlapping:

```
python3 tools/refresh_facade_materials.py --receipt docs/testing/polish-12/pass07/geometry-parity.json
/tmp/roaddash-blender-run/blender-4.5.9-linux-x64/blender -b --factory-startup --python-exit-code 1 --python tools/bake_facade_lod.py
/tmp/roaddash-layout-venv/bin/python tools/bake_metropolis.py --facades-only
python3 docs/testing/polish-12/pass07/run_authoring.py
CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo build --release --locked
python3 docs/testing/polish-12/pass07/run_native.py after
python3 docs/testing/polish-12/pass07/run_lod.py
python3 docs/testing/polish-12/pass07/run_native.py after-recovery
python3 docs/testing/polish-12/pass07/run_checks.py
```

Native runners isolate saves. Source assets/shaders must remain frozen for a
whole native run. The retained intermediate recovery failed because an old
binary loaded the newly edited binding102shader before its matching Rust build;
this was an integration sequencing error, not a physics result. The intermediate
albedo-only survey and failed recovery are retained separately; final receipts
supersede them. Final release/native logs are checked for shader/pipeline errors.

Authoring suite: all7groups pass, including3policy/geometry tests,34effective
material instances,128designs/384kit GLBs,383atlas bands/15mips, full profile
provenance, City sources and complete Metro crop/district coverage.

Legacy native sweeps passed: Valley53direct crossfades,12cached LODs,
132displayed panels/150,434triangles; City387crossfades,48cache entries and8
transition groups. These checks protect bounded ready-scene transitions; they
are not human acceptance of temporal dither, long streaming or perfect parallax.
Representative captured transition frames show the expected opposing dither.

Research was confined to this repository's original generators, material
adapter, source catalog/GLBs and installed Bevy shader definitions. No external
repository/media was imported or licensed; broader rights clearance is not
asserted. Provenance is updated in docs/third_party/ASSETS.csv. Remaining work:
human material/entrance approval, production signage/ground-floor authoring,
longer streaming, target GPU timing and hardware coverage. Stop after07.

## Final native evidence and cost

Final Metropolis survey and reverse/crash/pickup/remount/results smoke exited0;
no shader, pipeline validation or panic errors in final logs. Reverse−1.898m/s,
both pickup errors0.0000m. Final source/shader set stayed frozen throughout.
The legacy LOD sweeps used the same final legacy sources/atlas/shaders; the
subsequent change affected only the Metropolis packed material path, separately
verified with the final rebuilt executable.

Matched whole-app wall frames: before19.925ms average/39.788ms p95;
after20.062ms average/41.238ms p95. Both miss16.7ms. The retained increase is an
observation, not isolated GPU cost or a causal regression claim from one run.
Census is exactly unchanged:19,068buildings,3067chunks,2,909,113render triangles,
2,613,119collision triangles,2531static collision batches and two routes.
`comparison.json` records raw metrics, unchanged census and final log checks.

Inspected matched before/final metro09 street and final metro05 frontage views,
final remount, and representative Valley/City transition captures. Entrances,
brick/plaster hues and existing trim remain recognizable. Reflections change
with distance/angle; static captures cannot certify universally invisible LOD,
no shimmer, all district art, or human approval. Representative images:

- [Before street](before/screenshots/metro-09.png)
- [Final street](after/screenshots/metro-09.png)
- [Final frontage](after/screenshots/metro-05.png)
- [Final remount](after-recovery/screenshots/06-remounting.png)
- [Valley transition](lod-valley/screenshots/lod-valley/0720.png)
- [City transition](lod-city/screenshots/lod-city/0540.png)

## Final regression receipts

All mandatory checks pass: fmt, strict locked workspace/all-target Clippy,
426workspace tests (one pre-existing ignored), locked debug build, asset/dependency
audit, five roadmap tests and progress sync. Locked release build also passes.
Model differential body/joint/180tick trajectory and crash/run/remount fixtures
pass within the workspace suite. Broad paired rig labs were not rerun because
there is no physical/rig change. Exact commands/exits: `checks.json` and
`authoring-checks.json`; native process/log receipts remain beside each capture.
`git diff --check` passes. No performance/art/hardware/rights gates are closed.

Changed files:144derived GLBs and their manifests; far albedo/packed mip images
and manifest; Metro albedo/packed pixels/provenance; offline material policy,
refresh/front/bake tools; Metro existing material/shader/image composition;
rr_content atlas intake and regression tests; README/PROGRESS, ADR111, known
issues/history/provenance and this evidence folder. Dependencies/lockfile,
original full model sources and world geometry/planning content are unchanged.
