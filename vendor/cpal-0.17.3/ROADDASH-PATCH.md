CPAL 0.17.3 from crates.io / https://github.com/RustAudio/cpal, Apache-2.0.

RoadDash changes only `src/host/alsa/mod.rs`: route poll-level XRUN and
suspended-device states through CPAL's existing `prepare()` recovery branch,
and back off unrecoverable poll errors to avoid a tight error loop. This ports
the matching 0.15.3 patch to the Bevy 0.20 audio backend; native playback
verification is still required. The upstream license is preserved.
