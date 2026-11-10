# Persistent farm crops — 2026-11-09

Task **CROP-TRAIL-01**, Part06. Full art/course/hardware acceptance remains open.

Implementation on `codex/next-crops`, based on `ef3eecc0`. Original wheat geometry
in authored Metropolis grain parcels, separate from existing grass and civic
meadows. Existing grain-parcel undergrowth is shortened to~0.2m so upright old
grass cannot hide flattened wheat; its placements/batching/wind remain unchanged.
Stems, leaves and seed heads are opaque triangles. Road/paving/building,
parcel-edge and farm-prop exclusions preserve access. No crop collider or physics
parameter is added. Legacy maps remain unchanged.

## Method and limits

Static meshes are batched by 16m world tile. Each tile has 256 one-metre damage
cells, each 16 bytes (direction X/Z, strength, first-contact time). Only populated
tiles allocate storage; hard cap 2048 tiles / 8MiB payload per CPU/GPU copy.
The CPU additionally retains planting masks and map/asset bookkeeping; this is
not an assertion of total process VRAM. New marks upload only dirty 4KiB tiles;
ordinary frames update one 16-byte animation clock. No per-stalk entities, dynamic
mesh uploads, per-frame simulation of stalks or damage decay. Already touched
cells do not restart their animation on repeated contact. Reset clears damaged
tiles; pause freezes stamping and the shader clock. Memory does not grow with
trail length or race duration.

The authoritative fixed loop sweeps actual bike/rider-owned body transforms.
It rejects disabled and vertically separated bodies, then checks Rapier shape
distance against the crop column at candidate cell centres. Thus both baseline
and biomechanical ragdoll limbs, physical wrecks, mounted bikes and walking
capsules use the same path. Body motion over12m per fixed tick is a teleport,
not a swept trail. The one-metre damage grid and0.55m column tolerance intentionally
approximate a crop clump, not each individual stem. An opaque wheat canopy rotates
smoothly over0.32s and stays flattened after the actor leaves. No save schema change:
persistence lasts for the current race only, including culling and pause.

Only authored grain parcels within72m of the farm course receive new wheat.
Pasture/fallow keep their existing grass; unlimited whole-map agriculture and
fine per-stem contact are not claimed. Existing distance dither limits crop detail
to70–105m; underlying field surfaces stay resident. Roots sit0.02m above the authored grain surface (`terrain_height -0.12m`). Broad art/hardware
and full-race performance acceptance remain open.

## Research / decision

Reviewed primary sources:

- [NVIDIA GPU Gems chapter7, Kurt Pelzer](https://developer.nvidia.com/gpugems/gpugems/part-i-natural-effects/chapter-7-rendering-countless-blades-waving-grass): batching and vertex animation trade geometry, draw calls and CPU work. Its historical hardware figures are not measurements of RoadDash.
- [Bevy v0.17.3 storage buffer example](https://github.com/bevyengine/bevy/blob/v0.17.3/examples/shader/storage_buffer.rs): installed-version `ShaderStorageBuffer`, read-only material bindings and update API. No dependency migration.

Compared approaches: CPU per-stalk mesh edits would touch all animated vertices;
per-stalk entities add transform/extraction overhead; a single world damage texture
would need a large sparse atlas or custom subregion transfer/render integration.
The chosen small storage-buffer tiles use existing pinned Bevy APIs, preserve
marks outside view and confine uploads to touched regions. They cost one material/
draw per visible tile; a custom instanced atlas could reduce that and remains a
future measured optimization. This is an evidence-based bounded implementation,
not a claim of globally fastest foliage rendering. All mesh/shader/domain code is
original; no external code, reference images or media were imported.

## Reproduction

`cargo test --locked -p rr_core crops -- --nocapture` exercises persistent sweeps,
reset, no-contact/unplanted/teleport exclusions and a deterministic 16-body load.
`cargo test --locked geometry::crops -- --nocapture` exercises production contact
adapters for runner/wreck/ragdoll shapes,8m sweeps, disabled/airborne/pause and retry.

`ROADDASH_CROP_SURVEY=1 cargo run --release --locked` records actual grain geometry
before, during bending, after scripted contact, while paused and after reset.
The fixture uses a wreck collider followed by a walking capsule; it is deliberately
labelled a scripted contact fixture rather than a free-running physics crash.
Screenshots and a JSON assertion receipt are written under `screenshots/`.

Focused test receipts and native results are recorded below.

## Verified source and bounded costs

All7focused tests pass:3domain and4native-adapter/geometry/undergrowth tests
([transcript](tests.txt)). Strict `cargo clippy --workspace --locked --all-targets
-- -D warnings` passes ([receipt](clippy.txt)). The geometry census proves147,552
wheat stems,885,312static triangles and635populated tiles/2,600,960damage bytes.
The first geometry design duplicated each stem/head plane and produced1,770,624
triangles; the three existing stem orientations made those extra faces redundant.
The shipping mesh halves that count and keeps perpendicular leaves/seed heads.
The runtime geometry guard is1,000,000triangles; this is an allocation limit,
not a displayed-triangle or FPS claim.

Mesh payload is approximately123,943,680bytes (12vertices/stem×64bytes of current
position/normal/UV/colour/tangent attributes, plus18indices/stem×4bytes). Meshes
use `RENDER_WORLD` residency so the main-world CPU mesh copy is released after
GPU extraction. Actual GPU allocation includes engine/backend overhead and is not
measured by this arithmetic. The2.60MB damage payload exists in both domain and
asset CPU memory plus GPU storage; masks/map/handle overhead are additional.

The deterministic optimized-dev rule test exercises36,000no-contact sweeps and
576,000sweeps from16simulated bodies, with bounded resets. Its transcript reports
31200newly damaged cells and32768tile bytes in that small fixture. These timings
exclude Rapier shape-distance checks, terrain lookup, Bevy scheduling and all GPU
work; they must not be interpreted as gameplay frame time or a globally fastest
rendering method.

For a same-binary drawing comparison, run `python3
 docs/testing/next-crops/run_native.py off` and then `... on` after freezing the
release executable at the script's default path (or set `ROADDASH_BINARY`).
`ROADDASH_CROP_RENDER=off` hides only wheat meshes: state/contact work, trajectories,
fixed-time fixture and camera remain the same. Receipts report actual visible
batches/triangles, dirty-tile upload bytes and full-app wall-frame samples. The
stationary-contact assertion requires zero further damage-buffer uploads while
the crop clock/animation continue; reset uploads are included separately in the
total. This comparison does not isolate GPU timestamps, and wider hardware and
full-race performance remain open.

`python3 tools/audit.py` currently rejects the expected original geometry source
hash changed by this branch ([receipt](audit.txt)). Root owns shared ASSETS rows
and the new shader provenance entry and will rerun audit after integration. Full
workspace tests, mandatory debug build and free-running native recovery checks
remain parent integration gates, not claimed by these focused fixtures.

### Native inspection caught a stale-buffer defect

The first native run passed CPU assertions but was rejected on visual inspection
([retained rejected captures](rejected-buffer-update/)). In Bevy0.17.3,
`ShaderStorageBuffer::set_data` creates a new GPU allocation; untouched material
bind groups still referenced the initial cells/clock. As a result the crop state
changed correctly on CPU while visible wheat stayed upright. The early intact
capture also preceded pipeline readiness. Those receipts are **not** evidence of
working visual deformation or performance acceptance.

The corrected renderer creates `STORAGE | COPY_DST` buffers once and extracts
only sparse queued byte writes. After storage-asset preparation, `RenderQueue`
writes into the existing bound GPU allocations. Dirty4KiBtiles and the shared
16-byte render clock update without material rebinding or mesh uploads. Pending
writes retain their latest value until allocation preparation succeeds; the queue
is bounded by planted tiles. A240-frame fixed-camera warm-up precedes native
before/after sampling. Source API verified in the installed Bevy0.17.3
`bevy_render/src/storage.rs`; this integration is original code.

## Corrected native result

Both same-binary render modes exit successfully, with input/binary hashes retained:
[off receipt](native-off/receipt.json), [on receipt](native-on/receipt.json).
The warm-up reference contains fully prepared wheat. Original native images:
[intact](native-on/screenshots/crops-before.png),
[bending under the scripted wreck](native-on/screenshots/crops-bending.png),
[trail after the probe leaves](native-on/screenshots/crops-trail.png),
[persistent while paused](native-on/screenshots/crops-persistent.png),
[reset](native-on/screenshots/crops-reset.png).
The trail and later paused capture are pixel-identical, proving the visible mark
and visual clock remain frozen ([image analysis](visual-analysis.json)). Reset
was visually reviewed as upright plants; the general scene is not expected to
be pixel-identical to its earlier frame.

The actual near-field view displays13crop batches /24,624crop triangles (off:0),
out of635allocated tiles. The crash writes10cells; the walking capsule extends
that to15. Both runs perform11dirty-tile writes /45,056bytes **including reset**;
stationary contact generates zero further tile writes. The shared16-byte animated
clock is accounted separately. These are queued write payloads, not measured
PCIe traffic or allocations.

Local adapter: NVIDIA GeForce RTX3080, Vulkan, driver615.78.08;1440×900window.
One warmed bounded pair, full-application wall timing (not GPU timestamps):

| Window | Crops hidden mean / P95 ms | Crops shown mean / P95 ms | Samples each |
| --- | ---: | ---: | ---: |
| Quiet camera |6.914 /8.743|8.339 /14.447|40|
| Contact + racing actors |17.217 /19.792|15.827 /20.499|105|

Variation in the active means does **not** establish a speedup from drawing crops.
The active P95 exceeds16.7ms in both runs; the full-course60FPS target is not met
or certified by this fixture. See [off data](native-off/screenshots/crops-survey.json)
and [on data](native-on/screenshots/crops-survey.json). The isolated serial domain
benchmark was7.371ms for36,000no-contact sweeps and106.106ms for576,000body sweeps
([receipt](domain-benchmark-serial.txt)); it excludes native contact adapters.

Current crop meshes/buffers are Startup-resident and their handles remain owned by
`Crops` across retries. There is no in-app map/scene teardown: resetting a race
clears cell values while reusing exactly those allocations. The pending GPU map
therefore has at most one current entry per known tile plus clock, and application
teardown drops it with the render world. Any future runtime map replacement must
also clear pending writes for removed buffers; that lifecycle is not implemented
or claimed here.
