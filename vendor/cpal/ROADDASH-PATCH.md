CPAL 0.15.3 from crates.io / https://github.com/RustAudio/cpal, Apache-2.0.

RoadDash changes only `src/host/alsa/mod.rs`: route poll-level XRUN and
suspended-device states through CPAL's existing `prepare()` recovery branch,
and back off unrecoverable poll errors to avoid a tight error loop. The
unpatched backend could stop producing audio and flood stderr on PipeWire.

The upstream LICENSE is preserved. All other source is unchanged.
