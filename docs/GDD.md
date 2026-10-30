# RoadDash GDD v0.1

Date: 2026-10-30. Owner: RoadDash maintainers. Scope: Part 00 preproduction.
Status: accepted implementation direction; production acceptance remains measured
by the blueprint gates. This short design is independent of original Road Rash
code, characters, courses, names, media and statistics.

## Fantasy and pillars

Race a fast motorcycle through a living road corridor, pick a passing line,
trade readable melee attacks, survive slapstick physical crashes, then recover
as an independent person. Scenery changes decisions: town intersections, farm
traffic/livestock, coastal/cliff sightlines and hazards. Responsive arcade control
wins over unnecessarily exact mechanical simulation.

Historical references describe intent, not implementation specifications. We do
not target binary or tick parity with a proprietary game. Research observations
are explicitly documentary unless a play session is actually recorded.

## Priority and feature contract

| Priority | Requirement | Acceptance owner/part |
| --- | --- | --- |
| P0 | One approximately three-minute authored point-to-point farmland/city/hills corridor | P02/P09 |
| P0 | One assisted bike with two cosmetic variants; distinguish physics from appearance | P03/P11 |
| P0 | Three AI opponents, fair shared controls and live civilian traffic | P08 |
| P0 | Left/right punch and kick, damage, readable hits and knockoff | P05/P06 |
| P0 | Independent rider/bike; camera follows crash and foot movement | P04/P06 |
| P0 | Manual camera-relative walking overrides optional neutral auto-return | P06 |
| P0 | Forward/checkpoint-validated finish on bike OR on foot, one payout | P07 |
| P0 | Results/retry; save-safe earned progression | P07/P12 |
| P0 | Fixed 60 Hz simulation, Fedora/Windows baseline, usable input and diagnostics | P01 |
| P0 | Original/cleared media, captions/comfort/input access and audible cues | P00/P15 |
| P1 | Police pressure and arrest, garage/upgrades/repair economy | P08/P12 |
| P1 | Three handling presets, controller/rebinding, scenic/audio zone production | P03/P04/P11/P15 |
| P1 | Four-player authoritative racing after offline fun gate and network spike | P10/P13 |
| P2 | More tracks/bikes/modes, eight-player scaling, replays | P14/P15 |
| P2 | Weather, customization, modding, crossplay, online career | Future decisions only |

## Session and feedback

Target roughly three minutes of racing with a readable countdown and immediate
results/retry. Proposed fun gate: 80% of new players finish within three tries,
more than 50% voluntarily retry, less than 12% of the session is outside player
control. Record at least ten external playtests, failures and frustration causes.
These are targets, not results already achieved.

Throttle/brake/steer and melee map to explicit intentions. Camera, animation,
particles and sound show simulation events; they do not independently award hits
or wins. All critical states need text/shape cues, not color alone.

After knockoff, retain the actual fallen bike and physical rider. Following
settlement, manual foot direction takes precedence immediately. Neutral controls
may resume assisted bike seeking after a tunable 0.25-0.5 s. Safe proximity/low
bike speed permits remount, with a manual option later. No compelled teleport
while walking away. Valid prior checkpoints permit foot finish. Traffic remains
a threat, but repeated spawn kills and trapping need explicit protection/tests.

## World and content

Author a corridor, not an unbounded open world. Towns need curbs/intersections,
farms need fields/fences/tractors/cattle, cliffs need telegraphed drops and safe
recovery rules. Aim for a recognizable landmark or encounter every 10-25 s at
100-220 km/h. Never surprise-spawn an unavoidable hazard or unload a visible
collision hazard. Model budgets and streaming come from measurements.

Mechanical content has stable IDs and validated units; cosmetic variants refer
to the same tuning ID unless intentionally a different bike. Initial definition
and field sheets: content/catalog.json and docs/content/SHEETS.md. Prototype
tuning values are new design, not extracted original stats.

## Performance and reliability

60 Hz gameplay/physics; render interpolation independent of frame rate. Mandatory
production target: 1080p medium stable 60 FPS with representative traffic/four
players, P95 <=16.7 ms; RTX 3080 high 120 FPS is aspirational. Server target P95
<8 ms at 60 Hz is future work. Do not claim either from a scripted smoke run.

Pause must freeze simulation; reset must remove recovery actors, contacts and
transient combat state. Save updates must not discard readable legacy progress
or overwrite unsupported/corrupt files. Hardware/controller/Windows and clean
install tests remain required even when headless CI succeeds.

## Boundaries and risks

Public distribution is an original spiritual successor. No license to EA branding
or proprietary assets is assumed. No original game files are required or shipped.
No reference implementation or media enters the runtime without verified rights.

The current prototype is not the slice definition: flat procedural road, automatic
recovery only, no kicks, foot finish, authored town/farm/cliff route, police,
production rig or online authority. Preserve its working loop while filling these
requirements in the assigned later parts. Do not silently reduce the P0 gate.

## User map refinement — 2026-10-30

The current playable route starts in open farmland, develops into dense city
streets with crossing traffic and pedestrians, then climbs through rolling hills.
The user explicitly rejected a return loop. Farm cows can lift a bike on contact.
Real elevation affects where the rider and wreck settle and the walk back.
These refinements supersede the initial route order, not the remaining P0 gates.
