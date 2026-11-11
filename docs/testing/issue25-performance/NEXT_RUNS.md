# Reproduction and remaining gates

The issue25 cold/warm matrix and four dedicated recovery cases are complete;
see REPORT.md and controlled/. Final combined-content/engine measurements remain
required after the separately coordinated branches and Bevy migration integrate.
No local branch result certifies the full combined workload or16.7ms p95.

Build and freeze the desired release binary outside an actively rebuilt target.
Record source commit/tree, clean/dirty state, build log and binary SHA256. A
`road-dash.provenance.json` beside the binary may record these fields and must
include its matching `binary_sha256`; the runner rejects a mismatch. The older
frozen-binary-provenance.json explicitly records the initial snapshot limitation.
Measurement checkout metadata is separately named and is not build provenance.

Reserve exclusive CPU/GPU time before running the Linux matrix. All output paths
must be new; prohibit other builds/games and source writes that trigger indexing.

```sh
flock /tmp/roaddash-native.lock python3 tools/run_performance_matrix.py --binary /absolute/path/to/frozen/road-dash --output output/testing/new-performance-matrix --gpu
```

Only if explicitly authorized and necessary, add `--indexer-pid PID` for the
currently verified same-user long-lived baloo_file parent. Never copy an old PID
without revalidation. The runner verifies UID, executable and process starttime,
uses pidfds to avoid PID reuse, pauses matching existing extractor children, and
restores the actual process handles in finally/cancellation. It records all actual
signals. This is not permission to disable an indexing service/configuration or
pause unrelated processes. Active indexers detected without protection abort the
case and preserve its receipt. The completed controlled batch's actual proof is
controlled/indexer-control.json; PID2060 was restored afterward.

The matrix runs cold-asset control→candidate then warm candidate→control on both
maps. Cold mode evicts only checkout asset pages and records fincore residency;
it never claims a cold machine, metadata/executable/driver-cache reset. The
original query/multi-dispatch/two-job controls remain selectable even though the
production defaults are the measured candidate. The raw-frame warmup is180racing
frames; use identical frame cohorts across configurations for secondary analyses.
All-frame samples retain terminal verification while gameplay samples exclude it;
partial final windows flush and the writer drains on normal shutdown.

Functional recovery is separate from performance measurement; serialize games,
but bounded builds may overlap without any timing claim. Four cases cover both
rigs/maps, including alternate Hills route, and require an explicit completion
marker plus no render errors. Each case is bounded at900seconds.

```sh
flock /tmp/roaddash-native.lock python3 tools/run_performance_recovery.py --binary /absolute/path/to/frozen/road-dash --output output/testing/new-performance-recovery --production-defaults
ROADDASH_FIXED_EXECUTOR=multi /absolute/path/to/frozen/road-dash --ragdoll-lab --output lab-multi.json
ROADDASH_FIXED_EXECUTOR=single /absolute/path/to/frozen/road-dash --ragdoll-lab --output lab-single.json
```

Compare the18lab cases excluding only wall time/executor label. Both must retain
finite/bounded physical behavior, and the original eleven-body baseline remains
an independent control. No physics tuning is implied by profiling results.
