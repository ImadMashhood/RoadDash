# Player-hosted multiplayer, UI and walking pass — 2026-11-09

Branch: `codex/multiplayer-ui-walking`; starting point `75eadcf1`.
Status: implemented and locally verified; final integrated regressions and
native checks pass. Internet acceptance is open. No push, deployment or new main-branch merge is claimed.

## Requested behavior

- Player-hosted authoritative races: Host, Join Code and Quick Play.
- Quick Play discovers/probes available public hosts and chooses measured
  request/response RTT <=250ms, otherwise hosts with bots and a15-second timer.
- Public/private custom lobbies, host start, map selection and randomized grid.
- Joining humans replace unfinished bots; disconnects restore bot control.
- Original local background pictures, cleaned menu and honest status/errors.
- Walking animation correction, preserving controls and physical rigs.

## Sources and decisions

UI implementation and local picture mapping: [UI receipt](../online-ui/README.md).
Retail pictures remain in the user's external reference kit, outside Git; missing
pictures use existing project artwork. No new retail redistribution is implied.
Walking source/tests: [animation receipt](../walking-animation/README.md).

ADR128/129/130 in [DECISIONS](../../DECISIONS.md) record player authority,
distributed discovery, temporary router mapping and presentation-only gait.
Pinned Mainline8.0.1 (MIT) supplies public DHT discovery; igd-next0.17.1 (MIT)
supplies temporary router mapping. Neither supplies a relay or guarantees host
reachability. See [repository intake](../../third_party/REPOSITORIES.md).

No user-operated directory is required by this design. Existing DHT bootstrap
peers are still external infrastructure. DHT results are not exhaustive, and
all candidates require live game-protocol/availability/ping checks. Carrier NAT,
blocked inbound TCP and unsupported routers remain distinct connectivity gates.
Tests must disable live public announcements and router changes; isolated DHT
and mocked lease tests are not substitutes for different-network playtests.

## Checklist accounting

New IDs: ONLINE-PLAY-01 (Part13), UI-RETRO-BG-01 (Part15). Walking continues under
V2-03-003. README is the live checklist; remaining original networking gates are
retained. No broad performance, hardware, prediction, loss/latency, rights or
user art/feel acceptance is closed by source compilation.

## Verification matrix

| Gate | Current evidence |
| --- | --- |
| UI targeted tests | Included in529-test combined pass |
| UI native pictures | Original/fallback11states each and22-stage offline flow pass; final HUD mask11-state recapture also passes |
| Walking targeted tests | 22animation +12model +10recovery passed in isolated worktree |
| Walking contact metric | Established flat/12% slope contact slip <=0.409mm/tick; broader contacts remain open |
| Walking after native / both rigs |152nonblank final gait captures; both-rig recovery passes on first combined checkpoint |
| Protocol / isolated discovery / lease tests | Final combined529-test pass plus actual admission fixture pass |
| Actual host plus clients / bots / crash / results | Headless host+2clients and150ms delay pass; rendered Metropolis host+guest598matching snapshots with movement/attack/crash/foot/remount; private admission/late takeover/disconnect/full14guest capacity pass; private Metropolis→Valley90s reload passes1497matching snapshots and all recovery phases |
| Different Internet connections / NAT devices | Not verified |
| Full fmt, strict clippy, workspace tests, build, audit | All pass on combined source:529tests,1existing long soak ignored;580dependency declarations/180asset records audited |

## Outstanding limitations to retain

The transport uses bounded TCP messages and20Hz losslessly compressed snapshots, with750m human-centered world-actor interest neighborhoods; this
is not the roadmap's final unreliable/delta/predicted/lag-compensated transport.
A configured/discovered address is not proof of reachable hosting. Host loss ends
the session; no host migration is promised. Existing offline profiles, accepted
rigs, collision and world content remain control fixtures.

Walking changes cover rider/officer recovery presentation; civilian NPC gait and
exact footstep sound/contact synchronization are not established by these tests.

Rendered-map transport measured789,422encoded bytes/second per guest at20Hz (about6.32Mbps, excluding TCP/IP overhead), down from45.6Mbps JSON. This remains above the production100kbps target; compression is not a bandwidth acceptance certificate. [Native receipt](../online-races/native-compressed/summary.json).

The protocol2 compressed host/two-client150ms application-delay rerun passed:604matching host snapshot ticks per guest, all remote attack/crash/foot/remount flags and no reported errors. [Receipt](compressed-delay150/receipt.json). Delay is75ms per direction on loopback; packet loss and separate-network reachability are not tested.

[Actual-host admission receipt](../online-races/admission/README.md) verifies wrong secret/protocol rejection, private discovery exclusion, existing bot pose takeover, bot restoration, invalid-input rejection and full14guest admission with overflow rejection. These are protocol peers against the production authority, distinct from rendered-player evidence.

Integration review additionally corrected remote-human recovery entering NPC obstacle holds, local-slot camera occlusion, human footstep event ownership and the results display rewinding the replicated session clock. Regression cases cover blocked-wall local/remote parity and real compressed peer snapshots after host/guest finishes; final suite evidence follows.

[Final private map-reload receipt](../online-races/final-valley-reload/README.md) passes on the combined city/walking/online binary, retaining the same36character capability and observing all fixed recovery phases. [Final admission rerun](../online-races/final-admission/receipt.json) also passes.
