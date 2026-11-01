# Supporting production-blueprint context

Frozen context retained during the completed-first README reorganization.
Task checkboxes live only in the current [README](../../README.md); section
numbers below may describe historical grouping. The v3 PDF is a planning reference.

### Baseline and dependency decisions

The current build remains **Rust 1.94.0, Bevy 0.17.3, Rapier 0.32.0, built-in Bevy audio**, as pinned in [Cargo.toml](../../Cargo.toml), [Cargo.lock](../../Cargo.lock), and [rust-toolchain.toml](../../rust-toolchain.toml). Gameplay and Rapier now run at fixed 60 Hz with three physics substeps; render-only visual children and camera focus interpolate independently. The source-verified newer plugin manifests target Bevy 0.20, so migration remains a coordinated decision rather than mixing incompatible versions.

The following are **the PDF's proposed targets**, not newly verified compatibility claims or dependencies already installed. Recheck releases, compatibility, licenses, and platform builds before adopting them.

| Component | Blueprint target | Tracking decision |
| --- | --- | --- |
| Bevy | 0.20.1 patch line | Pending compatibility review and migration decision |
| Avian3D | 0.8.0 | Pending decision on replacing the current Rapier adapter |
| Bevy Enhanced Input | 0.27.x | Pending action maps, controllers, and rebinding |
| Lightyear | 0.31.0 | Defer until the offline slice gate and networking spike |
| Hanabi | Explicit compatible release/commit | Optional; adopt only after VFX/performance evaluation |
| Bevy Kira Audio | 0.27.x | Optional; evaluate only if built-in audio fails the mixing/crossfade requirements |
| Blender / glTF 2.0 | Tested, pinned authoring/export workflow | Pending production asset pipeline |


### Repository and source map

Repository roles below follow the PDF. A research listing is not permission to copy code or assets. Recheck file-level licensing when considering reuse; media licenses are separate from code licenses.

| ID | Repository | Role / parts |
| --- | --- | --- |
| R01 | [arkits/road-rash](https://github.com/arkits/road-rash) | Behavioral research only; P00, P02-P03, P05, P07-P09, P12, P14 |
| R02 | [Bevy-Jam-5/bike-game](https://github.com/Bevy-Jam-5/bike-game) | Camera, handling, scenery, and authoring example; code reuse only with verified notices, assets audited separately; P00, P02-P04, P09, P11, P14 |
| R03 | [blyfoten/driftmaster](https://github.com/blyfoten/driftmaster) | Tire, suspension, and telemetry research only until reuse permission is verified; P03, P06, P08 |
| R04 | [avianphysics/avian](https://github.com/avianphysics/avian) | Proposed physics adapter; P01-P08, P10, P13, P16 |
| R05 | [cBournhonesque/lightyear](https://github.com/cBournhonesque/lightyear) | Proposed later networking; P10, P13, P16 |
| R06 | [simgine/bevy_enhanced_input](https://github.com/simgine/bevy_enhanced_input) | Proposed action maps, gamepads, and rebinding; P01, P03-P06, P13 |
| R07 | [djeedai/bevy_hanabi](https://github.com/djeedai/bevy_hanabi) | Optional particles; P11, P15 |
| R08 | [bevyengine/bevy](https://github.com/bevyengine/bevy) | Current core engine; throughout |
| R09 | [NiklasEi/bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio) | Optional audio mixer alternative; P15, earlier only if justified |

### Environment contract - roads are gameplay

PDF pp. 26-27, 35. Placeholder trees/ocean/mountains do not fulfill the five production environment promises.

| Region | Required readable features | Required gameplay |
| --- | --- | --- |
| Town/city | Shops, homes, intersections, lights, parked cars, sidewalks, alleys, signs | Traffic priority, visibility blockers, hard corners/curbs, police density, ground-level walls |
| Farmland | Fields, barns, silos, tractors, cattle, fences, dirt entrances, hay bales, utility poles | Readable livestock crossings, slow tractors, mud/gravel grip, fence/ditch impacts, farm ambience |
| Cliff/mountain | Hairpins, ledges, retaining walls, valleys, rockfall, viewpoints, guardrails | Telegraph drops, downhill braking, rock hazards, explicitly permitted shortcuts, rescue/penalty instead of softlocks |
| Coast | Ocean vistas, towns, bridges, dunes, pull-offs, gulls | Scenic reveals, coastal wind/surf, grip changes, road-edge risk |
| Woodland | Dense trees, cabins, shadows, branches, deer, gravel turnoffs | Occlusion, contrast, narrow shoulders, animal crossings |


### Mandatory dismounted-control and finish test matrix

PDF pp. 28-29. These complete scenarios remain open even where the automatic-return subset already exists.


### Audio acquisition, event inventory, and mix acceptance

PDF pp. 30-31. The existing synthesized sounds are a prototype foundation, not completion of this production library.

| Source | Proposed use | Rights work to record before acquisition/shipping |
| --- | --- | --- |
| [Mixkit motorcycle SFX](https://mixkit.co/free-sound-effects/motorcycle/) | Idle/rev/shift/acceleration/pass-by | Verify the exact file and [Free SFX license](https://mixkit.co/license/modal/sfxFree/); no stock redistribution |
| [Kenney Impact Sounds](https://kenney.nl/assets/impact-sounds) | Combat, crashes, roadside impacts | Verify the selected files' CC0 terms; edit/layer variants |
| [Kenney Interface Sounds](https://kenney.nl/assets/interface-sounds) | Menus, countdown, checkpoints | Verify selected files' CC0 terms |
| [Sonniss GDC](https://gdc.sonniss.com/) | Foley, environments, vehicle details | Archive the applicable commercial EULA and restrictions, including redistribution/AI-training restrictions |
| [OpenGameArt example track](https://opengameart.org/content/super-wreck-roadway) | Prototype rock music | Verify the exact track/author/license separately for each song |
| [Poly Haven](https://polyhaven.com/license) | Textures, models, HDRIs, not sound | Verify asset-specific provenance/CC0 terms; keep in the art ledger |


### Architecture, data, and content contracts

PDF pp. 7, 32-35; concrete example values remain tunables, not original-game facts.


### Budgets and non-functional acceptance

PDF pp. 19, 22, 24, 27, 32, 34-36. These are proposed targets to measure, not achieved performance claims.

| Area | Target / guardrail | Required evidence |
| --- | --- | --- |
| Render | 1080p medium >=60 FPS; aspirational 1080p high 120 FPS on RTX 3080 | Representative busy traffic/four-player race, 1080p/1440p GPU/CPU traces, P95 <=16.7 ms for the 60 FPS preset |
| Simulation/server | Fixed 60 Hz; 16.67 ms tick budget, server gameplay/physics P95 <8 ms | Instrumented repeatable session, separate render/physics counters |
| Networking | Four players first, eight stretch; 20-30 Hz snapshots; initial <100 kbps/client | LAN, 50 ms/1%, 100 ms/2%, 150 ms/5% tests, four-player 150 ms/1% result consistency, eight-player stress |
| Track streaming | Initial 100-300 m scene chunks | Visibility culling/stream-ahead traces; hazards remain consistent when detail unloads |
| Mesh LOD0 | Bike 25-50k triangles, rider 25-45k, car 15-35k | Import validation, LOD/animation/traffic budgets; adjust based on measured GPU cost |
| Textures/shadows/VFX | 2K primary bike/rider, 1K secondary; limited shadow cascades/cutoffs; capped emitters | GPU memory, streaming spikes, overdraw and per-preset profiling |
| Surfaces | Proposed asphalt mu 1.0, gravel 0.65, grass 0.4 | Combined grip/brake/turn behavior tests, not arbitrary independent force clamps |
| Input/accessibility | Keyboard + gamepad, remapping/deadzones, FOV/blur/shake controls, captions | Real tester/device matrix and persisted preferences |
| Save/content | Interrupted-write safety, backups/migrations; provenance for every shipped asset | Fault-injection/save fixtures and export/license audit |
| Reliability/platform | Beta >99% crash-free sessions; native Fedora/Linux and Windows | Opt-in reporting, clean install/GPU/controller/reconnect tests, 30-minute soak |


### Rights, delivery evidence, and work-session order

PDF pp. 37-39. Record these as work/signoffs; the PDF itself grants no asset license or authority to publish a release.


The PDF's solo part-time planning ranges are context, not delivery commitments: physics prototype 4-8 weeks, enjoyable offline slice 3-6 months, online prototype/first content art 3-6 additional months, polished multi-track release 12-24+ months. The proposed scope cuts (four riders first, one region first, stylized PBR, simplified ragdolls, fixed garage, later crossplay/modding/online career) require an explicit decision rather than silently marking requirements complete.

