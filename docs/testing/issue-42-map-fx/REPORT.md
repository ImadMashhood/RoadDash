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

The opt-in `--map-fx-survey` takes paired small/large phase views at the same Hills heat and sand anchors, plus a paired view on each other map. Run it with `ROADDASH_MAP_FX=0` and the default for native before/after; per-run JSON records pool counts and bounded paused-view wall frames. These views are not a gameplay or GPU timing certificate.

## Evidence and remaining gates

Implementation review: finite capacities, original procedural texture generation, map and station routing, quality selector, camera culling, no late mesh creation. `cargo fmt --all --check`, strict `cargo clippy --workspace --locked --all-targets -- -D warnings` (before the final render-session guard), `python3 tools/audit.py`, `python3 tools/test_roadmap.py`, and `python3 tools/sync_progress.py --check` pass; the audit reports 580 dependency declarations, 199 runtime asset records and 28 reference observations. Workspace test/build and final Clippy rerun/native checks are pending. No GPU frame cost or 60 FPS claim is made. Need side-by-side `0`/default captures at Hills town and dunes, Metropolis city, legacy city, Valley, plus temporal sand/steam video, supported GPU samples, and user visual review. Recheck cloud/bird and grass shader compatibility after issues #30/#31 integrate, and decide migration timing with issue #37. RFU-FX-01–04 remain open.
