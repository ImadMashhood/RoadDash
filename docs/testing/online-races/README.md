# Online race implementation checkpoint — 2026-11-09

This is an integration checkpoint, not final Internet/hardware acceptance.

Implemented player-hosted TCP listen authority using the existing 60 Hz gameplay,
Rapier, combat and recovery systems. Guest messages contain bounded sequence-numbered
intentions, never transforms or results. Host snapshots replicate racer chassis,
separate wreck/runner/articulated bodies, recovery, attacks/results, traffic,
NPCs and persistent prop damage. Render interpolation cannot write authoritative
body transforms. Guest physics/gameplay advancement is disabled. Offline input,
saves and eleven-body/default anatomy remain available.

The native lobby API supports public/private hosting, direct capability codes,
map selection, host start, 15-second countdown, shuffled grid, and replacing
unfinished bot slots with humans. Quick Play combines LAN, configured peer seeds
and bounded public DHT discovery; a live nonce/protocol handshake measures RTT
and rejects unavailable or >250 ms hosts. A failed search becomes a listen host.
Private capability suffixes use OS randomness. Automatic finite UPnP leases and
DHT observed addresses are best effort, not proof of WAN reachability. No public
service was deployed and no cross-ISP or live router success is claimed.

## Evidence collected

- `cargo check --all-targets --locked`: passed after integration.
- `cargo build --locked`: passed; first frozen debug binary at
  `/home/imad/roaddash-online-checkpoint/road-dash`.
- Twelve focused network tests passed: isolated loopback DHT announce/search,
  cancellation, address filtering, lease lifetime/handoff, real protocol nonce/
  version/availability/RTT rejection, framed transport bounds and fragmentation,
  snapshot validation, visual interpolation and offline cleanup. Log included.
- Three independent OS processes used the production network and RoadSimulation
  plugins against a GPU-free physical test world. With `ROADDASH_DHT=0` and
  `ROADDASH_UPNP=0`, one host and two guests formed a three-human/12-bot grid.
  Host captured659 snapshots; each guest captured601. Every one of each guest's
  601 shared tick/hash pairs matched its authoritative host snapshot exactly.
  All three observed the remote human attack, crash, on-foot state and remount;
  no connection errors. Full receipts in `first-three-process/`.
- The GPU-free fixture does not validate native city payload size, rendered
  remote art, bandwidth targets, latency/loss, Windows or WAN/NAT behavior.
- An earlier host-only harness attempt launched background guests from a shell
  that did not preserve their lifetimes. It was excluded; the accepted harness
  waited on all three live subprocesses.

## Pending verification and limitations

Final workspace fmt/clippy/tests/build/audit, current-map native host/guest,
map-relaunch/reconnect, late takeover/disconnect, private rejection, finish order,
and broader physics regressions remain required. Later source refinements to
vehicle presentation/weapon snapshots and immediate replicated anatomy setup
postdate the first frozen binary and need a fresh build. Complete snapshot JSON
at20 Hz is intentionally conservative; prediction, lag compensation, interest
culling/delta compression, production bandwidth/8-player stress, relay/CGNAT,
authenticated accounts and durable online rewards remain open roadmap gates.

The test entry point is `road-dash --online-test-node host BIND REPORT` or
`... client CODE REPORT`. Start the host, read its temporary `HOST_CODE=...`,
start guests, and keep the parent harness alive until every process exits.

## Subsequent verification and corrections

See `compression-followup.md` and `native-compressed/` for the actual native
Metropolis host/guest pass (598 exact matching authoritative snapshots), measured
encoded traffic (~6.32Mbps per guest, still above the production target), bounded
compression and the retained initial payload/strike-clock failure.

`admission/` records actual host capability/protocol rejection, private probe
behavior, late takeover continuity, disconnect-to-bot fallback, all14 guest slots
and full-host rejection. `host-loss/` verifies the actual client application
records host termination as connection loss. `valley-investigation/` retains both
unsuccessful native reload fixture runs and explains the bounded-duration/render
sampling investigation. Those runs already established stable private capability
and connected children; final corrected recovery-state validation is pending.

Later source corrections align remote-human obstacle behavior and footsteps with
the local player, exclude the actual local slot from camera occlusion, keep
session time monotonic after local result-display time freezes, and restore
downed NPC collision/pose when leaving. Regression tests cover actual recovery
against a blocking wall, compressed TCP result-clock publication and NPC cleanup.
Root runs the integrated full suite; no unrun latest tests are claimed here.

The corrected private native Valley reload passed (`final-valley-reload/`,1497
matching samples), and the final denser-map-loaded Metropolis farm-area pair
passed (`final-metropolis-density/`,1495 matching samples). The latter's encoded
traffic is~6.41Mbps per guest; host progress reached514.773m. Urban neighborhoods,
15 scattered humans, WAN/NAT deployment and production bandwidth remain open.
Final-binary admission was rerun successfully (`final-admission/`). The final
Quick Play same-map fallback correction has separate Valley/City regressions;
it does not alter these direct Host/Join native paths.
