# Run the verified world update

From the repository root:

```sh
cargo run --release
```

The launch default is Valley Run. To select the retained City Run explicitly:

```sh
ROADDASH_TRACK=city cargo run --release
```

The asset IO stack budget is configured by the app; no RUST_MIN_STACK override
is needed. Reproducible checks:

```sh
cargo test --workspace --locked
cargo run -- --smoke-test
cargo run -- --world-survey
```

The recorded local survey reached15.83ms wall-frame p95 at1080p; this is a single
host/sample, not a multi-platform performance certificate. See README.md alongside
for known art/streaming limits and the five-second offscreen NPC crash policy.
