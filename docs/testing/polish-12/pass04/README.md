# Pass04 — unified lighting and exposure

Follows pass03 `b26d7f59c34bf91f9362c4a1d9e5ccb247424b85` in the isolated
RoadDash-polish worktree. ADR108; existing SKY-01 evidence extended without new
IDs, moved tasks or milestone closures. README/PROGRESS remain synchronized.

The existing sky installation overrode world light defaults, while camera
Reinhard selection and fixed colors/direction lived elsewhere. The bounded
rr_content art configuration now owns sun/ambient colors, shared sun direction,
exposure and an explicit analytic tonemapping choice. The renderer installs
these together, leaving simulation and shadow policy untouched. Config rejects
invalid colors, below-horizon/degenerate directions and unsupported LUT operators.
An ECS regression checks actual installed components, direction and shadow bias.

| Setting | Before | After |
|---|---|---|
| Sun illuminance | 48000 | 42000 lux |
| Sun sRGB | 1,.94,.83 | 1,.88,.72 |
| Sun elevation | 53.3° | 34.7° |
| Ambient brightness | 2200 | 3400 |
| Ambient sRGB | .78,.86,1 | .72,.83,1 |
| Environment intensity | 1100 | 1500 |
| Exposure EV100 | 13 | 12.7 |
| Tonemapping | implicit camera Reinhard | explicit profile Reinhard |

Reinhard is deliberately retained: gradual highlight rolloff without a new
contrast curve or LUT media. Stronger cool fill and0.3stop exposure increase
help shadowed road/entrance detail; reduced warmer sun balances lit surfaces.
The sky disc uses the same initial direction as the directional light. Existing
sky color/cloud/fog/resolution/wind settings remain. The old drifting cubemap
also rotates its disc; reconciling that long-term direction belongs to pass10.
No additional lights/render passes/textures/resolution increases. No facade
material/geometry/source band/atlas changes, physics, controls, saves or audio edits.

Research: inspected local environment.rs, world.rs, sky.rs, game camera and
pinned Bevy0.17.3 tonemapping enum/shader. No external repositories downloaded,
code copied, assets generated or new dependencies. Existing source provenance
row hash updated. Declaration audit is not legal/hardware certification.

## Native evidence

Fresh before survey uses pass03 release; after uses rebuilt pass04 release.
Same scripted camera stations at1440×900; moving traffic/cloud phase can differ.
Native process snapshots retained. Compilation/authoring and games were serial;
isolated save directories protect real saves. Survey shots are actual engine
captures, not mockups. Human art approval and racing-speed readability remain open.

[Farm before](before/screenshots/metro-01.png) /
[farm after](after/screenshots/metro-01.png),
[entry before](before/screenshots/metro-03.png) /
[entry after](after/screenshots/metro-03.png),
[city before](before/screenshots/metro-09.png) /
[city after](after/screenshots/metro-09.png),
[finish before](before/screenshots/metro-13.png) /
[finish after](after/screenshots/metro-13.png).

Verification and cost measurements follow.

The inspected farm and city-entry pairs retain readable lane markings and lit
surface detail; shaded road and facade detail is lifted. Fixed entry screenshot
regions have weighted display values44.06→58.01 on the right shaded facade,
29.82→40.72 on the shaded road,97.82→112.87 on the upper lit facade (0–255).
These are screenshot sample statistics, not photometric truth or human approval;
changed sun elevation also changes shadow footprints. `comparison.json` retains
coordinates, RGB means, census and exact frame metrics.

One isolated bounded native pair measures19.313ms average /39.287ms p95 before,
19.664ms /39.191ms after. No established speedup/regression or GPU-cost inference.
Both miss16.7ms. Census is exactly unchanged:2,909,113 render triangles,
2,613,119 collision triangles,3067 chunks,2531 static collision batches,
19,068 buildings,1494 props,122 parked cars,96 walkers. No new rendering effects.
This survey simulates fifteen racers in the farm while measuring at the urban
finish; it is not a full-route performance benchmark. Performance, complete
manual races, human art/racing readability, Windows/controller/hardware gates
remain open. Later passes own shadows/grounding, materials, atmosphere and sky.

## Completed verification and files

`checks.json`: fmt, strict workspace/all-target locked Clippy,421 workspace tests
(1 pre-existing ignored soak),locked debug build,audit,5roadmap tests and progress
check all exit0. Fresh locked release build exits0 (`release.txt`). This includes
src/models/tests.rs differential rig/recovery regressions. New tests exercise
invalid lighting intake and the actual installed light/camera profile, preserving
shadow enable/bias. No additional native failures required changes.

`native-results.json`: metro survey,both anatomy/baseline recovery and both NPC
smokes exit0. Both rigs reverse at−1.898m/s,all four pickup errors0.0000m,15-racer
retry and five initialized audio sinks. Paired18-case lab exits0,all finite,
including15 simultaneous crashes per rig (`lab.json`,`lab-exit.txt`). Actual
[anatomy remount](anatomy-recovery/screenshots/06-remounting.png) inspected with
both wheel contacts true. Scripted recovery is not a manually driven full race.
Final city/finish captures also inspected; no user art approval claimed.

Changed implementation: `content/world/art.json`, `crates/rr_content/src/world_art.rs`,
`src/sky.rs`. Tracking: README,PROGRESS,ADR108,KNOWN_ISSUES,CHANGELOG_DEV and existing
ASSETS source hash. Local source inspection only,no new research repos/assets,
no new task IDs or checklist closures. All uncommitted changes belong to this
pass in the dedicated branch; parent integrates. No push or merge performed.
`git-status.txt` captures the final working tree; proposed commit:
`feat(render): unify late-afternoon lighting and exposure`.
