# Verification results — Part04 district palette

2026-11-08 Linux / DISPLAY=:0 / NVIDIA RTX3080 / Vulkan /1440×900 native release.
Inherited baseline7465f98. Final source/binary/atlas/capture hashes are recorded in
capture-provenance.json; the screenshots were archived immediately after survey,
before smoke reused the screenshots directory. No push/merge claimed.

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS332tests,1existing ignored;0failures |
| `cargo build --locked` | PASS |
| `cargo build --release --locked` | PASS |
| `python3 tools/audit.py` | PASS523dependency declarations/88runtime records/27reference observations; hash/declaration audit only |
| `python3 tools/test_roadmap.py` | PASS5tests |
| `python3 tools/sync_progress.py --check` | PASS after regenerating canonical README text in PROGRESS |
| `git diff --check` | PASS |
| Existing authoring venv `tools/test_metropolis_facades.py` | PASS3tests; all19068assignments/all72sourcebands×5mips |
| Existing authoring venv `tools/verify_facade_lod.py` | PASS unchanged128design/383band global source atlas and all15images |
| Existing authoring venv `tools/test_metropolis_street_life.py` | PASS4tests |
| Existing authoring venv `tools/test_metropolis_streetscape.py` | PASS2tests |
| `python3 tools/test_world_inputs.py` | PASS4tests |
| Existing authoring venv `tools/bake_metropolis.py` | PASS byte-identical final manifest, facade bytes and provenance on repeat |
| Release `--inspect-world` | PASS19068buildings/36568surfaces/2routes/12000logical metres |
| Release `--metro-survey` | PASS12views; ready GLB primitive/actual triangle/collider/census checks |
| Release west `--smoke-test` with `ROADDASH_TRACK=metro ROADDASH_VERIFY_REPAIRS=1` | PASS grid restart/audio/farm+city crashes/both pickups/results fixture |
| Release east `--smoke-test`, additionally `ROADDASH_SMOKE_ROUTE=1` | PASS grid restart/audio/farm+city crashes/both pickups/results fixture |

Native source audit:809distinct resident primitive meshes,197303source triangles,
18067392actual vertex-attribute bytes (excludes indices/textures/allocator/VRAM).
Final detail view5471instances/1044360triangles; peak5998/1499452within6000/1500000
caps, no cosmetic colliders. Resident base3061chunks,2896899render triangles and
2606377collision triangles remain present throughout the survey. Those base
render triangles are additional to the bounded authored detail. Census counts
are actual generated geometry, not a production performance guarantee.

Material readiness took6.724680268seconds from kit creation in this one run.
Filesystem/cache state was not controlled; this is not a cold-start benchmark.
All24families are actually assigned to emitted closed coarse building geometry;
22are observed as near-detail family faces in the12sampled views. VICT12/WHSE12
were not near-detail faces in these camera samples; no claim of native close-up
review of every family. Every sampled detail mesh is checked against the loaded
original GLB handle set; actual index counts establish triangle cost. Aerial
views intentionally have no near-detail faces and exercise the coarse atlas.

Bounded native release wall-frame average18.29195ms/p9521.08162ms: **FAIL16.7ms**.
Fifteen racers simulate on the farm; the render camera surveys the city. Not GPU
timing/full-course racing/multi-crash profiling. The earlier AUTHORED-01 archived
19.81ms p95 used a different dev build/workload/view sequence, so no controlled
before/after frame regression or improvement is established. The source inputs
roughly double, coarse RGBA triples and the crown band adds wall quads; unchanged
hot detail caps do not imply unchanged total cost. Performance acceptance stays open.

Native frames reviewed: street metro-05 and aerial metro-10 (also primary review
of metro-08). Real modeled fronts are visible. Plain/gray courts and mid-block
grounds, heavy atmospheric wash, simple tower sides and final crown/texture/
corner/signage art are unfinished. This pass does not extend into landscape or
prop art. Paired street/aerial sets:05/06at6800m,07/08at8500m,09/10at10400m;
11is the urban finish view.03/04are west/east branches. Images are stationary
review views, not proof of prolonged high-speed/reverse streaming acceptance.

Smoke results are scripted fixture checks: they validate actual crash/get-up/
run/remount and grid/audio integration, then inject Results. They are not a
complete manually driven physical race/finish. Vulkan DRM extension and unmapped
controller/default-mapping warnings are retained in native logs. No hardware
controller/Windows certification, final art approval, broad world performance or
independent legal clearance. All29 Part04 requirements remain open.
