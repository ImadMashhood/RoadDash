# Issue #42 map atmosphere — staged implementation

Branch: `codex/issue-42-map-fx`; source baseline `e2a31918`, plus scope commit `36246757` (cherry-picked as `aef7c9b6`). Engine remains pinned to Bevy 0.17.3. No external repository, image, shader, sound, or proprietary media was copied. The alpha textures and motion are generated in `src/map_fx.rs`.

RFU-FX-04 is the new stable Part 15 requirement for Valley coverage under issue #42. Its reason, dependency, acceptance and verification are in README and the synced progress index. The top-level checklist is 224 verified / 293 open; no RFU-FX gate is closed.

## Scope and bounds

| Map | Cue | Default capacity | Visibility |
| --- | --- | ---: | --- |
| Hills western town/desert | Rising translucent heat bands | 16 | 115 m, off road |
| Hills town/desert | Horizontal wind-blown sand gusts | 48 | 125 m, off road |
| Metropolis and legacy city | Slow vent steam | 16 | 100 m, off road |
| Valley | Quiet air motes | 16 | 90 m, off road |

`ROADDASH_MAP_FX=0` allocates none, `low` uses half the counts, and the default uses the table above. Every preset distributes stations over the intended map span. One 64×64 soft texture and, on Hills, one 64×64 heat-band texture are generated. A single quad mesh and one immutable material per cue kind are shared. The animation scales soft sprite envelopes and never mutates material assets during PostUpdate. Placement is deterministic from route station and index. All entities are created during world setup, and presentation animation runs in PostUpdate after the camera, before transform propagation. The phase derives from fixed-loop race time, so zero-tick render frames and frame-rate changes do not advance it; pause freezes motion and retry resets it. No collider, drivetrain, Sand surface, save schema, camera postprocess, or fixed simulation path is edited.

The heat bands are a translucent rising air cue, not screen-space refraction or a custom shader. They need native visual review for whether they convincingly read as heat shimmer. Their low alpha avoids obvious foreground distortion, but that also may make them too subtle. Steam is the first city effect because a wet road would require matching model-derived distant proxy materials under MAT-03. Existing depth fog is the option for later city depth grading after baseline visual review. The transparent StandardMaterial pipeline is reused; first-use shader compilation stalls have not yet been measured.

The opt-in `--map-fx-survey` takes paired small/large phase views at the same Hills heat and sand anchors, normal chase-height views in the western town and desert, plus a paired view on each other map. Run it with `ROADDASH_MAP_FX=0` and the default for native before/after; per-run JSON records pool counts and bounded paused-view and active-simulation wall frames. These views are not a full-race or GPU timing certificate.

## Evidence and remaining gates

Implementation review: finite capacities, original procedural texture generation, map and station routing, quality selector, camera culling, no late mesh creation. `cargo fmt --all --check`, strict `cargo clippy --workspace --locked --all-targets -- -D warnings`, `cargo test --workspace --locked` (618 passed, two ignored), `cargo build --locked`, `python3 tools/audit.py`, `python3 tools/test_roadmap.py`, and `python3 tools/sync_progress.py --check` passed on the previous candidate; the audit reports 580 dependency declarations, 199 runtime asset records and 28 reference observations. A raised-ground visual correction has passed Clippy/build but awaits final tests and native review.

The first native Hills capture was occluded by storefronts. The next paired Hills run used open fixed and chase-height views, binary SHA `bc2a9daf770ab2c9a0bd788c6789dd1c59743d0be23d02543b8be14fbfe5681c`, and exited successfully in both settings. Its fixed camera faced away from the intended anchors, and image difference at threshold >5 RGB showed only 23 changed pixels in the heat-large frame and zero in the sand-large frame. This failed visual review. The 75-frame active-simulation wall p95 was 19.16 ms off and 20.06 ms default.

The following same-binary 0/default native pair ran on all four maps and exited successfully with binary SHA `0030a98612f2b8bcb32b9b9dc8394f5ed47f33aaf56951d130479eaca63f97de`. Camera signs pointed at the Hills anchors, but ground-buried quads made the heat/sand cues thin; Metropolis and legacy city cameras clipped facades. Valley had a subtle visible difference. The 75-frame active-simulation wall p95 in ms was Hills 18.79/19.00, Metropolis 29.07/28.75, City Run 50.97/51.06 and Valley 16.29/16.53 (off/default). These short, sequential samples lack full gameplay traversal, repeated runs and supported GPU pass attribution; no performance acceptance follows. The new raised-ground and city road-center visual correction awaits native review.

No GPU frame cost or 60 FPS claim is made. Need visibly distinct paired Hills town/dunes and city captures, plus temporal sand/steam video, supported GPU samples, and user visual review. Recheck cloud/bird and grass shader compatibility after issues #30/#31 integrate, and decide migration timing with issue #37. RFU-FX-01–04 remain open.
