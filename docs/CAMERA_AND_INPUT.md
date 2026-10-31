# Part 04 camera, comfort and controls

Native Bevy 0.17.3 / Rapier 0.32.0 retained. Bevy's compatible `bevy_gilrs`
feature enables OS controller discovery, connect/disconnect and normalized state.
No input-library migration or unrelated rendering/audio asset pack.

## Fixed input contract

PreUpdate (after Bevy input/UI focus) runs settings interactions, menu actions,
input capture and race actions. Held intentions feed the existing fixed buffer;
attack edges survive zero-tick frames and are consumed once. VehicleIntent adds
optional/defaulted analog amount fields while preserving existing digital fields.
Keyboard/full AI input uses full strength; normalized trigger strength scales real
engine/service-brake force. Steering is continuous; digital keys/D-pad take
precedence. Input uses the lowest connected Gamepad entity ID deterministically.
Removal immediately drops its held axes/buttons; keys continue to work. Another
connected controller becomes the owner; there is no saved hardware identity yet.

Left stick steers; RT/LT throttle/brake; X/B punch; Y nitro; A start; Start pause;
Select retry; right-thumb click camera; left-thumb click mute; right stick freelook.
LB/Guide open settings and are reserved. The configured deadzone (default 0.15)
remaps remaining magnitude continuously to 0–1, with finite checks/clamping.
Bevy's device-level calibration/filtering happens before the application deadzone.
Trigger rebinding reads analog values for the selected button, so compatible
analog buttons preserve pressure while ordinary buttons give digital strength.
Axis identity is currently fixed (left steering/right look), not arbitrarily
remappable. Physical device mapping/OS permissions still require real hardware QA.

InputContext explicitly separates Menu, Riding and OnFoot. Dismounted movement
intent has a separate vector and mounted attacks/vehicle input are suppressed;
existing automatic recovery remains unchanged. Manual movement/remount choice and
foot finish are Part 06/07, not claimed by allocating this context.

## Comfort and rebinding panel

F2 or LB opens a modal panel; a running race/countdown pauses and restores its
previous phase on close. Inputs cannot start/retry/attack beneath the panel.
Up/down or D-pad selects; left/right adjusts; Enter/A activates; Tab/Y switches
keyboard/controller binding mode. Mouse rows activate too. Press a key/button to
replace an action; duplicate/unknown/reserved values are rejected. Esc/Start
cancels an active rebind, otherwise saves/closes. Save row persists without close.

Versioned preferences (`schema_version:1`) store keyboard/controller maps and:
FOV 60–95° (default 75), follow response 2–20, shake 0–1 (default 0.5), speed-FOV,
anti-nausea, optional speed lines and deadzone 0.05–0.4. Pure Comfort validation,
FOV rules and axis normalization live in rr_core. Engine key/button decoding and
persistence stay in the client. ROADDASH_BINDINGS remains a validated keyboard
override; closing the panel can persist that map into preferences.

Paths: ROADDASH_SETTINGS, otherwise XDG_CONFIG_HOME/roaddash/settings.json or
~/.config/roaddash/settings.json; Windows APPDATA/roaddash/settings.json. Saves
use a temporary file and retain the previous valid file as .json.bak. Malformed,
unsupported and unreadable originals are preserved read-only, with a visible
status/warning; move/restore the original before enabling saves. No career-save
format changed. Tests/smoke use in-memory preferences or isolated temporary paths,
not the player's real preferences. No settings write occurs each render frame.

## Camera and feedback

Chase focus and riding heading derive from interpolated body pose, while the
horizon stays upright. Recovery follows interpolated torso/runner without copying
ragdoll rotation into the view. Existing near/default/far views remain. Exponential
response is frame-time based; steering adds bounded turn anticipation. Right stick
orbits relative to chase and release returns smoothly. A large (>75 m) target warp
snaps instead of flying across the old world; pose history already resets for
large simulation discontinuities. None of this writes a camera pose to physics.

A radius-0.3 m sphere sweep protects the near-plane volume against solid static
and moving geometry. Exclude sensors, the player's hidden chassis and owned
recovery bodies to avoid starting inside the tracked ragdoll. Complete arbitrary
confinement/overlap is not proven; standard obstacle/route checks are covered.
FOV adds at most 3° with speed and is clamped to 60–95°. Anti-nausea removes shake,
speed-FOV, anticipation and peripheral speed lines, with a gentler follow rate.

A regular HUD label reports AIRBORNE or TRACTION LIMIT / SLIP percentage using
actual BikeState support/slip (independent of F3). Optional speed lines are twelve
reused UI entities, disabled by default/anti-nausea and during recovery/menus;
there is no unbounded particle spawning. Existing RPM engine/wind audio remains.

## Verification boundary

Automated camera scenarios run at 60/120/144 render rates on fixed 60 Hz physics;
wall-volume clearance, warp snapping, analog force scaling, disconnect fallback,
modal input isolation, rebinding and corrupt-file persistence are checked. Native
smoke captures the actual comfort panel and race/recovery/city/hill views.
See [receipt](testing/PART_04.md). A simulated Gamepad component is not a physical
controller test or an external tester's responsiveness/readability acceptance.
P04-004/008/009 remain open to that extent. No Part 05 advancement.

## Current handling / lean correction

Latest feedback supersedes the earlier upright-rider pivot. Both bike and human
bank together from accumulated steering input only. Presentation removes physical
road/chassis roll before adding this bank; road curves do not add lean. Grade and
actual physical support remain. Shared human models, civilian clearance jumps and
cross-car wipeouts are preserved. See [source investigation](research/ROAD_RASH_HANDLING.md).

Near/default/far offsets relative to the focus are (0,1.1,-3.4), (0,1.5,-4.6),
(0,2,-6) m. Velocity feed-forward offsets follow lag; a hard bound of offset length
+0.35 m prevents speed/smoothing from pulling the view farther away. Recovery and
garage keep their separate framing. Optional speed FOV adds at most 3 degrees.
Camera anticipation follows resolved steering force rather than raw input edges.
The camera horizon itself never rolls with the bike.

`rr_core::steering` holds normalized player steering force with speed-scaled
angle target, bounded buildup and separate release/reversal response. The fixed
driver consumes validated response tuning and clears memory on crash/reset.
Raw input and resolved force are separately traced. Current timing is provisional
RoadDash data; the entire original handling is not reproduced without missing
reference specs/tables. [Correction receipt](testing/HANDLING_CORRECTION.md).
