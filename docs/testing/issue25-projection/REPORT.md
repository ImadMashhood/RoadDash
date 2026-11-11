# Hills route projection loading experiment — 2026-11-11

Status: a frozen A–B–A comparison and a final Hills-only run support a large
Hills loading gain. The first measured candidate indexed all course centerlines;
the selected source limits indexing to Hills and Hills east. This report does
not certify a runtime frame gain or the 16.7 ms p95 target.

## Bottleneck and implementation

The frozen `47b7b8ed` control spent 15.515 seconds in Hills countryside
construction, within 22.787 seconds of world setup. Grass placement evaluates
approximately 4.3 km of roadside rows at 1.6 m spacing. Accepted candidates
call `Track::project_route` twice; Hills projection previously scanned every
roughly 1.25 m centerline segment. This was identified from the local source
and measured loading spans, not inferred from entity counts alone.

The candidate groups 64 consecutive centerline segments under conservative
two-dimensional bounds. A supplied station gives an upper distance bound only;
every block that could beat it is still visited in original segment order. The
original per-segment float calculations, progress/lane outputs, negative apron
and tie policy remain unchanged. The final source builds the index only for
`hills-01` and `hills-east-01`; Metropolis and legacy courses use their original
full scan. No collision, surface, physical body, input, save, asset or render
policy changes are intended. No dependency or media was added.

The profile runner also now ignores defunct processes correctly. On this host,
`ps` renders a zombie as `cargo <defunct> ZN`; the old parser treated
`<defunct>` as its state and falsely vetoed a clean run. A Python regression
checks zombie, paused indexer and live compiler handling.

## First A–B–A comparison, before Hills-only scope

Control and candidate were clean release rebuilds from the same checkout path,
with package artifacts removed before each build. The control binary SHA256 was
`4c9df22aa44074345b066d1524c72d36ce4a1684a494c757faf0808a09bdeac1`
on both control legs. The first candidate SHA256 was
`5cc849e440b7121dd7f53d4648080cd598615be842990688ea40ab3bdee0e33f`.
Both executables were frozen before the paired runs. These numbers describe
that first all-course candidate, not the final Hills-only source.

All six native surveys used seed 25, default single fixed executor, eight
loading material jobs, the same Hills/Metropolis fixtures and 1440×900 Linux
Ryzen 5 3600 / RTX 3080 host. The runner isolated settings/saves, suppressed
screenshots during timing, captured raw racing frames after its 180-frame
warm-up and reported no competing process. These are warm asset-page runs,
not cold boots or ordinary free-driving races. `required_assets_ready` is not
a first-presented-pixel timestamp. GPU diagnostics are asynchronous latest-pass
samples, not whole-frame GPU percentiles.

| Case | Ready s | World setup s | Hills countryside s | Raw racing p50 / p95 / p99 ms | Frames |
| --- | ---: | ---: | ---: | --- | ---: |
| Hills control first | 25.862 | 22.787 | 15.515 | 17.227 / 20.481 / 22.914 | 4,350 |
| Hills indexed | 8.242 | 4.910 | 2.235 | 17.196 / 20.268 / 21.977 | 4,350 |
| Hills control last | 25.867 | 22.674 | 15.412 | 17.349 / 20.292 / 22.484 | 4,350 |
| Metro control first | 8.124 | 2.302 | — | 22.599 / 28.271 / 29.816 | 805 |
| Metro indexed | 8.140 | 2.286 | — | 23.323 / 28.666 / 31.319 | 805 |
| Metro control last | 8.079 | 2.308 | — | 22.412 / 29.393 / 31.907 | 805 |

The control return to roughly 25.9 seconds supports an approximately 17.6
second (68%) Hills readiness reduction under this warm fixture. Hills frame
p95 remains over 16.7 ms; Metropolis p95 is within the two controls, while its
single candidate p50 is 3–4% higher. This is why the final source limits the
index to Hills. Its separate comparison follows below.

The final Hills-only release binary SHA256 is
`c48c5af6992cd1a81ea451030f8156380715f7432d23e681bfca71b51a2c6f48`.
Its exact-source native run was uncontaminated and completed both courses:

| Final scoped source on frozen 47b content | Ready s | World setup s | Countryside s | Raw p50 / p95 / p99 ms | Frames |
| --- | ---: | ---: | ---: | --- | ---: |
| Hills | 8.246 | 4.977 | 2.255 | 17.258 / 20.497 / 22.368 | 4,350 |
| Metropolis | 8.276 | 2.330 | — | 22.209 / 28.111 / 31.953 | 805 |

The original full-scan algorithm is retained as an independent test oracle.
The scoped source passed bitwise comparison for all three projection outputs
on the four base courses plus Hills east: 25 stations × seven lateral offsets,
128 deterministic off-route points per path across bounds ±400 m, wrong/extreme
station hints, exact spline knots, negative starting aprons and beyond-finish
positions. Hills and Metro native survey JSONs matched the first control on all
non-timing structural fields. This is sampled evidence, not a formal proof for
every floating-point input. No frame-time improvement is claimed; Metro CPU
physics/render work still needs a separate measured investigation.

## Combined main integration and remaining limits

The branch was rebased onto main `0f54ff7c` after the map FX and AI refuge
merges. The exact combined release binary SHA256 was
`22822e0a12f6bb7f21b165b9c86ed04185cbc7d9089fe55b7b49c856d8ee3149`.
Native Hills and Metropolis surveys exited zero, with no render errors or
panics and no detected competing process. Hills required-assets-ready was
8.210 s, setup 4.952 s and countryside 2.265 s; raw racing p50/p95/p99 was
17.201/20.372/22.551 ms over 4,350 frames. Metro readiness was 8.171 s,
setup 2.301 s and raw p50/p95/p99 was 22.964/28.061/31.009 ms over 805
frames. These combined-source numbers have no paired `0f54ff7c` unindexed
control and therefore are smoke/attribution evidence, not another speedup
estimate. The structural surveys passed; full-race combat and recovery were
not repeated on this performance branch.

`cargo fmt --all --check`, strict workspace/all-target Clippy, locked workspace
tests (457 library, one binary, 59 content and 154 core passed; two ignored),
locked debug and clean release builds passed on the combined source. Four Python
profile tests, five roadmap tests and the README/progress sync check passed.
The declaration audit passed (580 dependency declarations, 205 runtime asset
records, 28 reference observations); it is not legal clearance.
[Machine-readable summaries, compressed raw JSONL and
survey receipts](evidence/) preserve all ten course cases in the A–B–A,
scoped and combined runs. The warm survey is not a cold boot or human full-race
test. Broad hardware, memory stress, recovery under the final combined source
and sustained 60 FPS remain open.

Source review used this repository's `Track::project`, Hills countryside
generation, the pinned `Cargo.lock` (Bevy 0.17.3 / Rapier adapter 0.32.0) and
earlier issue #25/#40 performance evidence. The installed performance skill
supplied measurement discipline only. No external repository code or media was
copied; the existing asset/license audit remains separate from legal clearance.
