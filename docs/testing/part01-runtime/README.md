# Part01 runtime audit — 2026-11-08

Development pass on top of local glove/socket commit9fb55ca. No dependency,
asset, save schema or physics-rig migration. ADR079 records the narrow new policy.
All29 existing Part01 requirements remain visible and unchecked: this is actual
runtime work with partial acceptance, not a certificate that the part has exited.

## Implemented correction and observations

The default velocity-muscle path already performs three-direct-hit stagger,
away swerve/countersteer, continuous seated release and protective-arm control.
Older proposed-only README wording was stale. Low-speed solid walls could still
leave the mounted stagger running because the ordinary wall gate was50km/h.
An existing stagger now releases at a tall-solid or vehicle normal closing speed
of at least1m/s, in either phase. Zero-speed rests, glances below that threshold,
low suspension support and ordinary riding/baseline thresholds are unchanged.
No launch, spin, position correction or new energy source is applied.

A128-record adapter ring identifies each evaluated vehicle contact with monotonic
simulation tick, both entities, both pre-solver world velocities, normal, normal
closing speed, approach class, solid height and crash decision. Set
`ROADDASH_TRACE_CONTACTS=1` for native text observations. It is bounded diagnostic
state, not a persistent race replay or authoritative server event stream.
100/60 same-direction measures40km/h closing; oncoming measures160km/h. These are
not substituted for world entry velocity or the existing retained-speed policy.

## All29 open requirements

No row below is a newly invented task. Stable IDs and placement remain unchanged.

| ID | Current functionality / evidence | Remaining acceptance or external gate |
| --- | --- | --- |
| AI-11 | Deterministic racer/killer roles, attack/retaliation regressions | Human races on both branches, aggression/difficulty approval |
| CRASHREF-004 | Pinned original reconstruction/retail arithmetic comparisons and current contact fixtures | Full caller tables, reciprocal traffic, original/modern bounce/drag trajectories and review |
| V2-01-005 | New measured approach/height records; solid-car trajectory comparison fixture; existing CCD and energy envelope | Contact-driven nose-down/vault mechanism across car profiles, angular/clearance/recovery traces and user approval |
| V2-01-006 | Existing default three-hit swerve/countersteer/release; new both-side/both-phase solid interruption regressions | Broad obstacle/terrain/multi-rider continuity traces and native user gameplay review |
| V2-01-007 | Pinned research and preserved baseline; ADR079 narrow interruption policy | Full before/after original calibration/trajectory comparison and bounded-change review |
| V2-01-008 | Automated/native corrections have scoped receipts | Explicit retained-feel/crash-response acceptance and unresolved vault criteria |
| ARCH-001 | rr_core/rr_content and subsystem plugins isolate real pure rules and adapters | Full client/sim/physics/content/tools separation; authoritative server later |
| ARCH-002 | ContentId validation, schema versions, save corruption protection | All bike/rider/weapon/cosmetic IDs and migration-safe future protocol/catalog |
| ARCH-003 | AttackAttempt and subsystem outcomes exist; finish path deduplicates racer results | Shared typed tick/ID intent/resolution/crash/recovery/finish/police contracts and server semantics |
| ARCH-007 | Fixed60Hz, buffered input and presentation interpolation tested at30/60/120/144Hz | AI perception cadence5–15Hz and complete adapter-observed authority contract |
| RIGHTS-006 | Workspace/CI/debug metrics and existing native Fedora receipts | Full session acceptance retained with architecture/platform gates |
| RIGHTS-008 | Acceleration/brakes/two wheel rays and camera regressions | Full handling/session acceptance and human rides |
| RIGHTS-009 | Lean/grip/three presets implemented and numeric tested | Subjective distinct-preset/handling acceptance |
| P01-003 | Real core/content workspace and plugins; Conventional Commits | Remaining game.rs subsystem split and eventual server/network responsibilities |
| P01-004 | Pure core rules separate from Bevy/Rapier; adapter/presentation modules | Complete subsystem isolation and future authority boundary |
| P01-006 | Action contexts, buffering, stable ContentId/schema infrastructure | Complete typed authoritative event contracts/catalog |
| P01-007 | offline/devtools/debug_physics and parallel feature exist | Real netplay boundary and reproducible quality modes |
| P01-009 | Ubuntu/Windows CI matrix plus workspace/content/license audit steps exist | Current remote run and separately validated coordinated dependency migration; no migration done here |
| P01-010 | Existing100 pause/crash/reset automated cycles and Fedora native smoke | Actual Windows native launch/input/audio; human native cycle acceptance |
| P01-011 | Measured independent fixed ticks and CI configuration | Both-platform runtime and current required remote CI evidence |
| P01-018 | Real Fedora/RTX3080/Vulkan smoke receipts | Windows hardware, physical input/audio and clean-install receipts |
| P03-010 | Existing ignored simulated ten-minute aggressive soak, numeric fixtures | Ten human rides, ten-minute real aggressive soak/controller feedback |
| P03-011 | Presets and automated stability implemented | Beginner finish, distinct/fun/stable handling acceptance |
| P03-020 | Verified local disc provides original tables; research records availability | Complete drive/brake/gear/nitro/grip/fixed-point numeric calibration and unresolved source reuse permission |
| P04-004 | Render-only interpolated camera,120/144Hz numeric continuity tests | Measured native high-refresh tight-turn/impact/jump readability/jitter acceptance |
| P04-008 | Collision-aware camera, controls and persisted comfort implemented | Both real input devices, external tester and comfort/clipping feedback |
| P04-009 | Keyboard/native scripted finish/recovery fixtures | Human keyboard/gamepad race comfort/responsiveness acceptance |
| P04-014 | Gilrs device discovery/default mapping documented | Real buttons/axes/reconnect verification, licensed mapping if needed |
| P06-006 | Three-hit and existing contact/recovery thresholds | Independent stamina/durability/balance/police/bad-landing thresholds and single typed CrashStarted contract |

## Scope and research

Read current code/lockfile/CI and pinned local research. No new external repository,
code or media was imported. The handling research already superseded its old
missing-calibration-files statement after the verified341-file retail disc became
available; README now agrees. Availability is not numeric parity or distribution
permission. New task IDs: none. TASK_PLACEMENT and checklist counts are unchanged.

## Verification

Exact final commands, results and measured trajectory conclusions are appended
below after execution. Native smoke injects recovery/results fixtures and does
not establish uninterrupted human races, controller feel or broad GPU performance.

### Measured over-car result and missing mechanism

Run `cargo test --locked measured_solid_car_height -- --nocapture` to reproduce
[the12-case solid-contact trace](solid-car-trajectories.txt). It uses real Rapier
contacts/CCD,100km/h rider, stopped/60same/60oncoming traffic,1.2m and2.5m solid
box proxies, and both accepted/default and eleven-body passive control. Each
case samples180 ticks of detached torso translation/orientation/velocity.
All cases remained finite; only the default1.2m/oncoming case crossed the forward
face above the roof inside the lateral footprint. No2.5m case did. Default1.2m
stopped contact peaked3.0785m relative torso height yet did not clear that face.
Peak height alone is therefore not a vault certificate. These measurements are
solver-specific observations, not bitwise deterministic cross-platform bounds.
The fixture observes3seconds of crash motion; it does not itself certify complete
get-up/remount, arbitrary mesh clearance, angular momentum conservation or parity.

Existing solid contacts already produce some fore/aft tumble and occasional low
car clearance. General nose-down/vault remains missing: the production box proxy
has a vertical front rather than a profiled hood/windshield, and no car-profile-
aware bounded pitch/clearance policy has been specified/validated against the
accepted passive control. A synthetic upward/angular impulse or repositioning
would manufacture the requested outcome and violate the no-added-energy intent.
Implementing a new energy-converting contact/pitch policy requires paired work,
angular momentum, whole-body solid-clearance and recovery evidence rather than
retuning the accepted rig here. V2-01-005 stays visibly partial with a reproducible
failure/variation fixture. No guaranteed over-tall-car response is claimed.

### Executed checks

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: passed.
- `cargo test --workspace --locked`:323 passed, one existing simulated ten-minute
  aggressive-turning soak ignored; includes body/recovery differential tests,
  unchanged contact energy envelope, rear-bike/downed-body routing, parking/graze,
  reverse/pickup regression and100 pause/crash/reset cycles.
- `cargo build --locked` and `cargo build --release --locked`: passed.
- `python3 tools/audit.py`: passed declaration/hash audit; no registered media changed.
- `python3 tools/test_roadmap.py`:5 tests passed.
- `python3 tools/sync_progress.py --check` and `git diff --check`: passed.
- `target/release/road-dash --ragdoll-lab --output /tmp/part01-ragdoll-lab.json`:
  all18 paired baseline/default scenarios finite. [JSON](ragdoll-lab.json),
  [text](ragdoll-lab.txt). These existing fixtures compare9 scenarios per path;
  this change adds no muscle, rig or recovery parameter tuning.
- Native graphics/audio route0 smoke passed on the supplied Fedora/RTX3080 setup:
  seven racers/seven killers, fifteen-racer grid retry,5 playback sinks,
  reverse−1.827m/s, two remounts with0.0000m pickup error, both recovery phases
  and results fixture. [Log](native-route0.txt). Command:
  `DISPLAY=:0 ROADDASH_TRACK=metro ROADDASH_VERIFY_REPAIRS=1 ROADDASH_VERIFY_REVERSE=1 ROADDASH_TRACE_CONTACTS=1 target/release/road-dash --smoke-test`.

The native smoke invokes crash fixtures directly; it does not exercise a full
human vehicle-contact race or certify the new obstacle interruption visually.
Physical-controller mapping remains unverified (Gilrs default-mapping warning
persists); Windows hardware, subjective feel, original-game parity and broad
performance gates remain open. No push or merge was performed.

- Final release native route1 used the same environment plus
  `ROADDASH_SMOKE_ROUTE=1`; all scripted grid/reverse/recovery/results checks
  passed, two pickup errors0.0000m. [Log](native-route1.txt).
- Explicit eleven-body control used the same route0 environment plus
  `--ragdoll-baseline --smoke-test`; all scripted checks passed, including both
  recoveries, reverse−1.827m/s and two pickup errors0.0000m.
  [Log](native-baseline-route0.txt). This is paired native recovery/roster
  regression, not a new fifteen-NPC simultaneous crash/performance certificate.

[Workspace tests](workspace-tests.txt), [Clippy](clippy.txt),
[debug build](build.txt), [release build](release-build.txt),
[audit](audit.json) and [roadmap tests](roadmap-tests.txt) retain command output.
