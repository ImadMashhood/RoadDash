# Online frontend handoff — 2026-11-09

Implementation branch: `codex/online-ui`, starting at `75eadcf1`.

The frontend retains the offline course/street/settings/results/profile flow and
adds Multiplayer → Quick Play, Host Custom Race, and Join With Code.
Host creation selects public/private visibility and Metropolis/Valley/City.
The authoritative `Online` resource supplies lobby code, roster, map, countdown,
connection error and host permission. Buttons enqueue `OnlineAction`; no online
button releases the local start menu or creates local-only opponents.

The join editor accepts a ten-character public address code or36-character private capability code, with keyboard typing,
Backspace/Delete, D-pad position/character editing, and mouse character controls. The code-type button chooses the controller editor
length; typing can fill either length. Online results offer Leave, preventing an
independent local restart.
A/Enter finishes editing; B/Escape first cancels editing, then returns.
The discovery status must describe the actual configured provider; the UI does
not infer global discoverability from a LAN response. The networking
worker owns actual listen-server transport, address-code encoding, map reload,
15-second matchmaking countdown, random grid and authoritative race replication.

Main menu action labels are clearer and left-aligned. Online screens use a dark
readable panel, explicit disabled guest controls and cancellation during loading.
Existing input ownership still clears game action edges at the menu boundary.

## External original pictures

The user's new request explicitly selects original game pictures, superseding
older original-art-only presentation preferences for this local runtime override.
The optional kit defaults to `$HOME/Documents/ChatGPT/RoadDash-UI-References`.
`ROADDASH_UI_REFERENCES=/path/to/kit` overrides it; an empty/nonexistent directory
selects portable project artwork. Files are decoded directly into image assets;
no retail image is copied into the repository or packaged game assets. Missing or
undecodable individual pictures fall back to the existing project image.

| Screen | External slot |
| --- | --- |
| Splash | UI-IMG-0142 / SPLASH.RRI |
| Main / Quit | UI-IMG-0068 / MAIN.RRI |
| Rider | RR-0E composed character selection |
| Street | UI-IMG-0146 / STREET.RRI |
| Course / loading / results | RR-0A composed course selection |
| Shop | UI-IMG-0071 / OLLEYS.RRI |
| Bar | UI-IMG-0072 / PANZER.RRI |
| Online pages | UI-IMG-0045 / CHARS/MANOMANO.RRI |

The composed RR-0A picture preserves its original postcard offsets; the narrow
243×480 POSTCARD.RRI is never stretched into a full-screen background. External
pictures use aspect-preserving contain; project artwork retains its cover crop.
The sixteen approved original portrait identities remain in their existing gallery.

Metadata/hash evidence: [external-art.json](external-art.json). Existing source
catalog `docs/ui/IMAGE_LIST.csv` and `SCREEN_MAP.md` were consulted. No new download,
external repository or dependency. No redistribution/license clearance claimed.
Screenshots containing these pictures must remain outside Git.

## Verification

Initial targeted `cargo test --locked --lib frontend -- --nocapture`:12passed.
Initial `cargo build --locked`:passed. Initial10-state native visual fixture:exit0;
main, join and host lobby reviewed, correct original slots/aspect and readable text.
Missing arrow glyphs were found and replaced with ASCII controls in source.
Final rebuild/captures (including36-character codes) and integrated transport
validation remain pending. [Initial capture hashes](initial-native.json) link
to external files rather than bundling retail pixels. Root owns combined
README/checklist/ADR/known-issue updates.

Targeted tests cover queued host/join actions without local launch, guest host-only
permissions, leave, code editor edge handling and bounded controller cursor.
The existing menu tests retain offline/settings/results input ownership coverage.

Run `ROADDASH_VERIFY_ONLINE_UI=1 ROADDASH_UI_CAPTURE_DIR=/external/capture/path`
with `--frontend-smoke` for eleven native visual fixtures: main, online, host, join,
connecting, host lobby, guest lobby, Quick Play countdown, disconnected,720p
private-code entry and the live-race Resume/Leave menu.
These injected presentation fixtures are explicitly **not network evidence**.
Actual two-process lobby/race tests belong to the networking integration evidence.
Human controller, Internet/NAT reachability, Windows/GPU and broad performance
remain separate gates until measured; no next major roadmap part is started.

## Lifecycle review follow-up

Map-relaunch connection/lobby/error states now replace the initial splash.
Escape/Start during an online race opens Resume/Leave without pausing authoritative
simulation; local driving/attack/on-foot edges are suppressed until Resume.
The transport pump must follow `FrontendSet::Input` to send masked client input.
The overlay remains over the live scene. Leaving as host explicitly ends the room.
Host/client tests and the final native overlay fixture await integrated verification.
The read-only network review also identified cleanup/slot-reset/result-metric and
local camera/mute issues; the network worker is fixing those in owned files.

## Frozen-checkpoint native review

Binary `aa2c8b5489e1f996fe8822ead76eb7757cd04741cb76cbea3f384bee805476dd`
ran with DHT/UPnP disabled and fresh external saves/settings. Original-art and
missing-kit fallback fixtures both exited0 with10screens each. The checkpoint
predates the eleventh live-overlay fixture. Visual inspection confirms readable
36-character join entry at1280×720, all15lobby riders, private code, independent
status strip,15-second countdown and disabled host controls. Missing-kit fallback
uses project artwork correctly. Stale ping text underneath a connection error was
removed in follow-up source; it needs the next binary for recapture.

Offline smoke captured29screens and exited101 at its obsolete seven-club assertion.
The approved cast has eight equipped riders; baseline `GridWeapons::roll` and its
existing unit test already enforce that content. The smoke assertion now checks
the exact authored IDs3/4/7/9/10/11/12/14 and all eight rendered weapons. This is
a test repair, not gameplay tuning. Full offline rerun and live-overlay capture
remain required. [Checkpoint receipt and review](checkpoint-native.json).

## Repaired integrated rerun

Frozen binary `a52dee9f2330a662e80d3ee602bd083717c6c091941fbef6b0e53c6c788373c9`
passed both eleven-state visual fixtures (original pictures and missing-kit
project-art fallback) and the complete22-stage offline native flow. Offline
verification includes16portraits, all eight exact dossier weapon IDs/rendered
weapons, pause/resume/results/quit and five actual viewport sizes, including720p,
ultrawide and4K. It produced33offline screenshots. The earlier failed checkpoint
receipt remains above; it has not been overwritten.

The36-character private code fits its control at1280×720, all15lobby riders and
connection status remain separate, and failed connections no longer show stale
ping text. The new live online menu rendered Resume/Leave over the running world,
but the underlying HUD text competed with its body text. Follow-up4b6578c masks
only the HUD while the menu is open; normal `update_ui` restores it on Resume.
This last visual fix requires one online-fixture recapture.

[Repaired receipt, hashes, offline report and review](repaired-native.json). All
original pixels remain external. These are functional/art observations on this
Linux system, not Internet transport, human controller, broad hardware or frame
budget certification.

## Final overlay recapture — verified

Binary `417388c3f866d162d088871a9990e985e64db5a3a3843ee611e9ffae3f15da64`
(`/home/imad/roaddash-online-eof-fixed/road-dash`) exited0 and captured all eleven
fixture states with DHT/UPnP disabled. The1280×720 live-race menu was inspected:
HUD leaderboard/map/speed text is now hidden, while the3Dworld remains visible.
Resume/Leave actions, host warning, connection status and input hints are readable
and separated. This verifies the last visual correction4b6578c; no source changed
during the recapture. The passing repaired offline/original/fallback evidence
above remains valid, and both earlier checkpoint failures/observations are retained.

[Final overlay receipt and screenshot hashes](final-overlay-native.json). Original
pixels remain outside Git. All requested UI native visual checks are now recorded;
human art/controller acceptance, Internet/NAT reachability, broad hardware and
performance remain distinct gates. These scripted fixtures do not certify real
multiplayer transport; that evidence belongs to the networking integration report.
