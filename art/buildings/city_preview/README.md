# City Run authoring viewer — playable world source

The current map is **5.3km** and is also the default playable race. It has53 blocks,
977 complete exterior building lots and85 original-City-family-eligible geometric
facades across eight district zones. This viewer shares rendering with the game;
the previous1km studies are historical evidence, not current scope.

```sh
cargo run --release                         # actual racing game
python3 tools/generate_city_race.py --check  # baked data consistency
cargo run --locked --bin city_preview       # isolated authoring camera
ROADDASH_TRACK=valley cargo run --release    # original Valley world
```

config.json is the canonical city authoring config. generate_city_race.py writes
the track, settings, whole-building plan and generated/scene.json together.
Recompile after re-baking. config.legacy-1km.json preserves rejected-study tests.

Geometry uses our actual3D front modules, full-depth rear/party walls, real rear
openings, shared floor/roof/plant and cross-street returns. No facade-image cards
are loaded. District/course rules come from ../mesh_assets/districts; source
eligibility is not a claim of exact original game coordinates.

Viewer:1–4 choose views;5 opens the original catalog gallery;PageUp/PageDown pages
through all146 source assets;WASD/right mouse/R/F/Shift inspect. The game retains
its existing riding/combat/recovery controls. The gallery requires original GLBs
in models/; the playable city uses front_modules/ and materials/.

Actual game survey: `cargo run -- --world-survey`. Evidence, native captures,
checks and remaining gates: ../../../docs/testing/city-5300/README.md.
