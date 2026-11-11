# Issue #32 — Hills wild horses and bison

Integrated on `codex/issue-32-wildlife` against main `99b697d3` (ADR147). Five
RFU-WILD requirements remain open for full-course, user art, physical-rig native
controls and broad hardware/performance acceptance. The animals are scenic only:
no road crossings, Rapier colliders, damage, save fields or physical-rig changes.
Existing cows, static horses and mounted cowboys are retained.

## Implementation and source

Six fixed off-road herds: three bison groups of 8/6/10 at 2050/3100/5320 m,
then three wild horse groups of 7/9/8 at 8700/9600/10800 m. Population is 48.
Groups follow deterministic race-clock ellipses with formation offsets and
individual strides; all paths are sampled against the resident terrain at
startup. Each member has 128 resident path samples, with no runtime terrain
query. Authoring validates finite, supported, road-clear, bounded-grade paths.
The complete swept envelope is reserved from new farms, props and dressing.

Original vertex-colored horse/bison meshes and gait geometry use the project's
existing `Model` helper. No third-party repository, code, animation or media was
copied. `docs/third_party/ASSETS.csv` records original MIT source provenance and
SHA256 hashes. The installed game-ai, procedural-generation, level-design and
performance skills informed group-path and measurement decisions. The Bevy skill
router targets 0.20, so no 0.20 API was applied to this locked Bevy 0.17.3 tree.

Visibility is presentation-only after camera update and before transform
propagation. A 500 m herd cull with 420–500 m scale taper avoids a hard distant
spawn. At most one nearest herd within 110 m animates four leg meshes per member;
far bodies use one consolidated mesh. Six species/variant groups provide 18 shared
mesh handles and one material. Path payload is 6,144 `Vec3` values (73,728 raw
bytes). These are allocation bounds, not GPU draw-call or frame-rate claims.

## Integrated verification

- `cargo fmt --all --check`: pass.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: pass.
- `cargo test --workspace --locked`: 667 passed, two existing ignored. Includes
  wildlife core/content/terrain/clearance tests and physical recovery regressions.
- `cargo build --locked` and `cargo build --release --locked`: pass.
- `python3 tools/audit.py`: pass, 204 runtime asset records; declaration/hash
  check only, not legal clearance.
- `python3 tools/sync_progress.py --check` and `python3 tools/test_roadmap.py`:
  pass (five roadmap tests).

[Check receipts](checks/) are from the post-PR55 source. The native visual run
used the same release binary SHA256
`e6ad1590735436c68c810951f4c6b97710c6eca2cdf31be8a5d840f3aa55379e`.
`target/release/road-dash --wildlife-survey` completed 1,100 frames and eight
captures. It confirmed 48 animals, 6.78 m tracked bison movement, maximum 40
animated limbs and 50 visible mesh candidates, and no wildlife collider. Close
[bison](native/visual/wildlife-120.png) and [horses](native/visual/wildlife-380.png)
are visible on Hills terrain; the [far landscape frame](native/visual/wildlife-790.png)
shows the distance setting, where animals are too small to establish art
readability. All eight native captures remain in the local workspace; three
representative PNGs are committed for PR review. The survey movement counter
measures bison; the two horse frames show changed positions but are not a
quantitative horse-motion trace.
The [native survey JSON](native/visual/wildlife.json) records counters.

Four alternating release cost runs used `ROADDASH_WILDLIFE_PERF=1` with the same
binary, fixed near-herd camera, 1,100 frames and 120-frame timing warmup. The
`ROADDASH_WILDLIFE=0` control skips all wildlife allocation. Active racer and
traffic world remained loaded. Summary: [performance.json](native/performance.json).

| Run | Enabled median/p95 ms | Disabled median/p95 ms | Enabled wildlife CPU median µs | Max RSS enabled/disabled MiB |
| --- | ---: | ---: | ---: | ---: |
| 1 | 17.435 / 20.577 | 17.276 / 20.257 | 8.967 | 3260.5 / 3254.1 |
| 2 | 17.252 / 19.487 | 17.301 / 19.552 | 8.616 | 3262.1 / 3257.1 |

Run variation overlaps the frame differences. This establishes bounded measured
CPU work in this fixture, not 60 FPS, GPU timing, target hardware, full-race cost
or a reliable frame improvement. Elapsed process time is about 46 s enabled and
45 s disabled; it includes startup and the survey, so loading is not isolated.
The 3–7 MiB RSS differences include whole-process variation and are not a pure
wildlife allocation measurement.

## Remaining gates

Full-course native traversal and physical-rig recovery controls on the final
wildlife binary have not yet run. Existing headless physical recovery tests pass,
and wildlife adds no contact body or fixed-simulation write. User review of the
procedural animal shapes/gait and broad GPU/Windows/hardware performance remain
open. Off-road players can intentionally pass through scenic wildlife under the
chosen contact policy. The current mid/far captures do not establish readable
animal silhouettes at those distances.
