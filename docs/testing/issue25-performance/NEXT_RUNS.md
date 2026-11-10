# Serialized continuation commands

Run from `/home/imad/Documents/ChatGPT/RoadDash-issue-25`, after the parent grants
an exclusive measurement window. The frozen binary is independent of Cargo target
changes and resolves assets from this compiled checkout. Its SHA256 must remain
`6fe419d28e8833e2c4777b94b1cd642b04e8a76ae0303cced84b74636371ad44`.
All output directories below must be new. Do not use these commands while another
game, compiler or Rust test process is active. The original Baloo PID has exited;
no indexer PID is passed. If one is explicitly identified later, only that exact
validated process may be reversibly suspended by the harness.

Repeat with targeted cold asset pages, approximately5–6minutes total:

```sh
python3 tools/run_performance_profiles.py --binary output/testing/issue25-optimized/road-dash --output output/testing/issue25-repeat-cold/control --runs 1 --gpu --terrain-control --cold-assets
python3 tools/run_performance_profiles.py --binary output/testing/issue25-optimized/road-dash --output output/testing/issue25-repeat-cold/candidate --runs 1 --gpu --fixed-executor single --load-jobs 8 --cold-assets
```

Warm candidate repeat (approximately130seconds), then reverse-order warm control
(approximately175seconds) supplies a second warm result and reduces fixed-order
bias. These require a separate explicit measurement window:

```sh
python3 tools/run_performance_profiles.py --binary output/testing/issue25-optimized/road-dash --output output/testing/issue25-repeat-warm/candidate --runs 1 --gpu --fixed-executor single --load-jobs 8
python3 tools/run_performance_profiles.py --binary output/testing/issue25-optimized/road-dash --output output/testing/issue25-repeat-warm/control --runs 1 --gpu --terrain-control
```

Functional dedicated recovery checks are separate from performance measurements;
serialize games but these may share a bounded build window. Four cases cover
both physical rigs and both tracks, including alternate Hills recovery route.
Each case requires the existing smoke completion marker and no render errors;
exit0alone is not acceptance. No timing claim is made. Budget several minutes
per case; the runner bounds each case at900seconds.

```sh
python3 tools/run_performance_recovery.py --binary output/testing/issue25-optimized/road-dash --output output/testing/issue25-recovery --fixed-executor single
```

Afterward send receipt paths to the issue25 agent. Do not merge or set new dispatch/
loading defaults solely from this checkpoint. Recheck deterministic census parity,
repeat distributions, memory and readiness; preserve any failed/noisy samples
with explicit exclusions. Final integration must incorporate latestmain, rerun
appropriate checks and use the parent-approved integration slot.
