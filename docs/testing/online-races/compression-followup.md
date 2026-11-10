# Native payload investigation and protocol 2 — 2026-11-09

The first actual native Metropolis host/guest pair used frozen binary SHA256
`a52dee9f2330a662e80d3ee602bd083717c6c091941fbef6b0e53c6c788373c9`.
Host recorded648 snapshots; guest recorded506 matching authoritative tick/hash
pairs. Both observed remote attack, crash, foot movement and remount. The guest
then rejected a snapshot at approximately25.5s. This run is **not a pass**.
The validator incorrectly limited retained fired strike clocks to10s; existing
combat deliberately keeps advancing those clocks until another attack/crash.
Validation now bounds strike age by session elapsed time instead.

Full-world JSON snapshots averaged285201.56bytes (maximum287821), approximately
45.6Mbps per guest at20Hz before protocol overhead. This was materially too large.
Protocol2 now serializes once per broadcast, optionally DEFLATE-compresses once,
and shares immutable framed bytes across bounded guest queues. The frame prefix
high bit indicates compression; the lower31bits contain payload length. Both
encoded and inflated lengths remain bounded (512KiB server snapshots,4KiB guest
messages). Inflation stops after the configured limit plus one byte. Invalid
compressed data and expansion beyond the cap disconnect cleanly. Existing pinned
`flate2=1.1.10` is now a direct dependency; no new package versions were introduced.
The DHT namespace changes with protocol version to avoid stale incompatible peers.

The separate750m NPC/traffic interest filter keeps every racer and persistent
prop damage while reducing invisible world payload. Host physics/collision and
world entities remain unchanged. The actual post-change native wire measurement
is pending; compression unit tests alone do not establish Internet bandwidth.

Quick Play now repeats LAN discovery once per second during its15s search,
canonicalizes valid endpoints before its32-candidate budget, sorts live measured
matches by RTT, and retries remaining candidates when admission fails before
becoming host. Lobby messages are validated before they can affect local UI or
client authority selection. Global simultaneous empty DHT searches may still
create separate hosts; eventual discovery is not a global matchmaking lock.

19 focused network tests passed after compression/filter integration, including
bounded inflation, retained clocks, malformed candidate budget, actual handshake,
interest hide/reentry/offline restoration and existing recovery/physics cleanup.
Two additional admission-helper tests were added afterward and await the next
full run. Task-owned home temporary/log directories avoid the unrelated shared
`/tmp` quota failure encountered by the initial native performance logger.

## Completed native remeasurement

The actual native Metropolis pair passed on binary SHA256
`5666c19c53703a48ce73cec994d7376b4517b90ccb5f79382bd91ac02bff253b`.
Host captured659 snapshots; guest captured598, all598 shared tick/hash pairs exact.
Both observed remote attack/crash/foot/remount, two human players and movement
above33m/s, without connection errors. The initial15s countdown was preserved.
Receipts: `native-compressed/`.

Mean encoded frame throughput at20Hz is789,422bytes/s per guest (~6.32Mbps),
excluding lobby/TCP/IP overhead; maximum encoded frame40,445bytes. Mean JSON
snapshot239,325bytes. This is substantially smaller than the initial45.6Mbps
payload but still above the production bandwidth target. Static-actor deltas,
world cadence/quantization and per-client interest remain optimization gates.
This functional loopback run is not a WAN or frame-time performance test.
