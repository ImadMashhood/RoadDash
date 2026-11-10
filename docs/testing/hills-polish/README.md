# Hills atmosphere and world polish — ADR136

User-authorized follow-up to the original Hills map: dust, birds, clouds,
lighting, cacti, richer trees/props and cowboys riding horses. The strong 12.5 km
road profile, town fork/in-town rejoin, other maps and accepted physical rigs
remain unchanged.

Run `ROADDASH_TRACK=hills cargo run --release --locked`. Use
`--hills-polish-survey` for active effect/mounted close views and `--hills-survey`
for the complete course. Native fixture setup is not ordinary race behavior.

- Dust uses 96 pooled soft sprites, emitted near moving vehicles only on supported
  unpaved verges. It has no colliders and does not change traction.
- 24 birds circle with articulated wings. Pause freezes animation; retry clears
  particles and resets the visual clock without spawning more objects.
- Hills gets its own cloud/lighting profile with one coherent stationary sun,
  fog and environment illumination. Cloud banks remain stationary; the sky is
  not rotated to fake wind. Other maps retain their prior profile.
- Five original horse/cowboy pairs walk bounded off-road loops, including visible
  openings at town entry and exit. Four-beat articulated legs and matching
  colliders update in fixed simulation. Absolute replicated race time gives
  host/guest pose agreement and pause/restart behavior without new wire data.
- Original cacti, boulder clusters and fallen branches dress desert/hill zones.
  Ten existing original tree variants retain the earlier tree count. Western
  stores get readable signs, barrels, troughs and hitching rails.

## Files and provenance

`src/world_gen/hills/{atmosphere,mounted,dressing}.rs` contain the new systems;
`props.rs`/`countryside.rs` add details and reserve mounted corridors. Runtime
wiring is in `src/runtime.rs`; Hills-only lighting is in `src/sky.rs`.
`src/hills_polish_survey.rs` supplies native verification. Recovery safeguards
and receipts live in `src/recovery.rs`, `src/verification.rs`,
`crates/rr_core/src/escape.rs` and their regression tests. New geometry,
lettering and radial dust texture are original procedural work. Trees reuse
existing catalog TREE4A2/4B2/5A2/F2/U12, TREEH2, TREOAKA2/B2, TREEK2/TREEL2.
No new external research repository, dependency, proprietary media or licensed
asset intake. Source hashes are recorded in `docs/third_party/ASSETS.csv`.
Resident Hills building geometry has no far-atlas replacement; existing city
facade atlases/materials remain unchanged.

## Verification and limits

All 548 workspace tests pass (one existing soak test remains ignored), along
with formatting, strict Clippy, locked build, asset/hash audit and five roadmap
tests. Receipts are recorded beside this report. Native
receipts cover the active polish fixture, the complete road survey, west/east
recovery, baseline-rig recovery and a Metropolis control.

The active fixture verifies 24 birds, five moving mounts, a physical collider hit
on a moving mount, 81 paused frames, and bounded dust from an actual moving bike.
Dressing census: 136 cacti, 87 rocks and 21 deadwood props. The course survey
retains 1,506 road-support probes with maximum error 0.025898 m, roughly 210 m
of elevation range and a 26.69% maximum sampled grade.

Native close-ups were inspected for mounted figures, visible dust and birds,
cacti and western details. This caught a mirrored storefront sign; glyph X
layout was corrected with a front-view orientation regression test. Initial
headless failures and the initial active report are retained as rejected
iteration evidence, not final verification.

HILLS-04/05/06 remain open for human art/full-course acceptance. README and
PROGRESS retain 222 verified / 262 open requirements; no task closure. The horses are scenic kinematic actors:
no rider ejection, dynamic avoidance, horse ragdoll or new mounted-player mode.
Other-map and recovery controls must pass before delivery. Entity counts and
scripted views do not establish full-course performance or hardware acceptance.

## Recovery investigation

The first native west/east runs stalled after the hill crash. The retained
`west-first-rejected.txt` and `east-first-rejected.txt` are failures, not passing
receipts. Hills has continuous mountain rails; a wreck across them can be
unreachable with ordinary on-foot movement. The follow-up preserves physical
rigs and rails, extends the existing delayed rescue policy to a genuinely
stationary automatic Hills recovery, and exposes the rescue count in native
evidence. `ROADDASH_VERIFY_HILLS_RESCUE=1` extends only the test deadline so the
real configured delay is exercised; it does not shorten gameplay recovery.

The actual-world frozen recovery fixture and native ray diagnostics identify
the obstruction as a solid 8 m guardrail segment, not Ground. No ground-filter
or physical-rig change was made. Manual control, walking progress and pause
are covered by policy/adapter tests; invalid below-world recovery remains a
separate emergency case. Diagnostic native processes were intentionally stopped
after collecting contact evidence and are not counted as successful smoke runs.

## Review next

Review the [mounted cowboy](active/screenshots/polish-mounted-a.png),
[western storefront](active/screenshots/polish-western-detail.png),
[dust trail](active/screenshots/polish-dust-trail.png),
[birds and lighting](active/screenshots/polish-birds-lighting.png) and
[cactus](active/screenshots/polish-cactus.png) captures, then play both town
branches and the complete course. Human art/feel, sustained performance, real
network latency and target-hardware coverage remain open; no new major part
was started and no remote publication is included.

West and east native runs each recorded rescue counts `[0, 1]`: the first crash
recovered normally, while the rail-separated hill wreck required one exceptional
rescue after 120.011 seconds in the bounded region, adding exactly 5 seconds.
Both physical remounts completed with 0.0000 m pickup error. These are explicitly
rescued hill recoveries, not evidence that a rider can walk through or vault the
continuous guardrails. The standard short Hills smoke deadline is insufficient
for this exceptional policy; use the documented polish test runner.

The retained baseline rig completed both crashes without rescue (`[0, 0]`),
also with zero pickup error. Diagnostic contact logs are retained as
`diagnostic-west/native.txt` and `diagnostic-ray/native.txt`.

## Final result

All six cases in `native-checks.json` exited successfully: active polish, full
Hills survey, west/east extended recovery, baseline rig and Metropolis control.
The Metropolis control also completed both remounts without rescue. All 548
workspace tests, five roadmap tests, strict Clippy, formatting, locked build
and asset audit pass. HILLS-04/05/06 stay open for human acceptance; ADR136
records the implementation and recovery decision.

Batch-start and delivered binary fingerprints are retained separately; each
native case records its actual binary hash. The final rebuild adds only a
diagnostic query type-complexity lint annotation, with no runtime change.
