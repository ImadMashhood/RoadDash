# Default multicore physics

PERF-09 · 2026-11-07 · codex/parallel-physics · parentb8243bc · ADR076.

`parallel_physics` forwards `bevy_rapier3d/parallel` and is enabled by default.
Bevy0.17.3, bevy_rapier3d0.32.0 and rapier3d0.31.0 are unchanged; fixed60Hz,
substeps, anatomy/muscle parameters, masses/constraints, momentum/contact policy,
world/content, controls, saves and recovery are unchanged.

The installed Rapier pipeline parallelizes **independent simulation islands** via
Rayon. One connected ragdoll/contact pile is not split across all cores in this
version. Synchronization, collision detection, writeback, gameplay and rendering
are not all made parallel by this flag. Multicore support is not a60FPS guarantee.

## Run and compare

Default multicore game:

```sh
cargo run --release --locked
```

Explicit serial fallback with the same game/developer features:

```sh
cargo run --release --locked --no-default-features --features offline,devtools
```

A controlled thread-count experiment can use `RAYON_NUM_THREADS=4` before the
normal run command. Unset it to restore the library's automatic worker selection.
This is an experiment, not an unmeasured recommended setting. The local JSONL now
records compiled `parallel_physics`, available CPU threads and requested Rayon
thread environment. Availability/request are not an active CPU-utilization meter.

## Comparison method

A copy of the original serial release executable was preserved before changing
Cargo features. Baseline paired ragdoll lab ran18 cases (nine scenarios in passive
and active modes), each900 fixed ticks. Same test parameters apply to the parallel
release. `headless_wall_ms` measures900 app updates, not pure solver time; compare
finite state, anchor/limit errors and momentum as well as elapsed time.

The same serial/parallel native Metropolis route0 scripted smoke records actual
wall/main/physics/render spans; the parallel route1 fixture is also required.
For summaries, use native windows at simulation elapsed >=8s (manual60Hz). These are scripted recovery
comparisons, not a replay of the user's specific slow4.73km incident and not
bitwise deterministic physics or GPU benchmarks. PERF-08 remains open.

The first feature activation requires a lockfile update: rayon1.12.0,
rayon-core1.13.0, crossbeam-deque0.8.8 and crossbeam-epoch0.9.21. Existing engine
versions are unchanged. Actual locked package sources and dual MIT/Apache license
files were inspected; see third_party/REPOSITORIES.md. No media or upstream source
is copied into project code.

## Verification

Execution receipts and numerical comparisons are stored alongside this document.
Required checks: default-feature formatting/Clippy/workspace tests/debug/release
build, declared-license/source audit, serial-feature graph/fallback, paired crash
lab, actual native roster/crash/getup/run/remount/restart on both branch choices
and real JSONL reporting the parallel build. Broad human feel, Windows hardware,
future networking determinism and60FPS remain separate gates. No automatic merge
or next major development part is included.

## Recorded results

- `cargo fmt --all --check`, Clippy workspace/all targets with warnings denied,
  workspace tests and debug/release locked builds: passed.307 tests passed;
  one pre-existing ignored test remains ignored.
- Initial `cargo check --workspace` intentionally resolved/locked only four newly
  activated packages. Subsequent checks/builds are locked. Dependency/source audit:
 523 declared dependencies,86 runtime asset/source records,27 reference observations.
  Existing engine versions unchanged; exact Rayon/Crossbeam notice copies archived
  under docs/third_party/licenses. Declaration audit is not release legal signoff.
- Serial fallback `cargo check --locked --no-default-features --features offline,devtools`
  passed; feature graphs prove that fallback omits Rapier parallel and default enables it.
- Paired headless18-case900-tick crash labs: all finite. **Every non-timing field
  matched**, including body count, mass, initial momentum error, joint/limit errors,
  energy/speed maxima and resting/sleeping observations. See physical-differences.json.
  This machine/case result is not a promise of future bitwise network determinism.
- Both parallel native branch smoke fixtures and repeated route0 serial/parallel
  runs passed: seven racers/seven killers, authored15-racer grid restart, farm/city
  crash/getup/run/remount and result fixture; actual audio sinks initialized.
  New native JSONL reports parallel_physics=true,12 available CPU threads,
  no requested Rayon override and zero dropped samples.

### Timing evidence and limits

| Headless fixture,900 ticks | Serial ms | Parallel ms | Throughput ratio |
| --- | ---: | ---: | ---: |
|15 passive crashes |3100 |2261 |1.37× |
|15 active crashes |8971 |5830 |1.54× |
|Single active drop |692 |973 |0.71× |

The active15-rider case used35% less wall time. Small isolated cases were slower
because dispatch overhead outweighs useful island parallelism. These are complete
headless app-update durations, not GPU or pure solver timings. The complete18-case
lab total is similar because small-case overhead offsets the large-case gain;
no general35% game-FPS improvement is claimed. Exact values: lab-comparison.json.

Native route0 comparisons below use logged windows after **simulation elapsed8s**
(native manual60Hz, at least480 fixed ticks). This avoids startup frames lingering in the rolling240-frame
statistics and aligns later scenario timing. Values are means of logged windows,
not whole-run/global quantiles, GPU timings or a controlled full-race replay.

| Native route0 run | Wall avg ms | Mean logged P95 ms | Main world ms/frame | Physics span ms/frame |
| --- | ---: | ---: | ---: | ---: |
|Serial first |18.513 |23.466 |17.095 |9.325 |
|Parallel first |17.993 |21.939 |16.271 |9.347 |
|Serial repeat |17.856 |22.213 |17.289 |9.646 |
|Parallel repeat |17.417 |20.663 |16.551 |9.511 |

Whole-process elapsed first pair25.881s/26.867s, repeat24.856s/21.420s includes
startup/driver/window teardown and is **not** the isolated frame benchmark.
Native gains are modest and physics cost is nearly unchanged in this ordinary
recovery fixture. That is consistent with work outside island solving remaining
serial. It does not diagnose every part of the user's4.73km slowdown or close the
60FPS gate. PERF-08 should next profile synchronization, collision/solver and
writeback separately and repeat a normal race at the troublesome location.

No new task beyond PERF-09; the existing PERF-08 owns remaining profiling and
optimization. Modified files: Cargo.toml/Cargo.lock, compiled-backend diagnostics,
summary CLI, README/progress/placement, ADR076, history/issues/testing/repository
records and dependency notices/evidence. No content, assets, physical tuning or
save files changed; no files removed. Local branch codex/parallel-physics, no
push/PR/merge claimed. Windows hardware and human feel review remain open.
