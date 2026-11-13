# Demo settings and proximity voice

## Game comparison

| Reference | Shipped options | RoadDash implementation and gaps |
| --- | --- | --- |
| [Forza Horizon 5 accessibility](https://support.forza.net/hc/en-us/articles/4409280058259-Forza-Horizon-5-Accessibility-Support) and [PC options](https://support.forza.net/hc/en-us/articles/4409262786579-Forza-Horizon-5-PC-Specs) | Separate mix levels, adjustable camera, input remapping, HUD controls, display options, reduced menu motion, subtitles and visual accessibility | Added engine, wind and effects levels, window presets, a shadow toggle, speed units, minimap and standings toggles, and stationary menu scenery. Existing FOV, camera comfort, VSync, fullscreen and input controls remain available. HDR, frame caps, quality presets, captions, color-vision filters and expanded controller and wheel remapping remain open. |
| [Apex Legends voice settings](https://help.ea.com/en/articles/apex-legends/voice-chat/) | Open mic or push-to-talk, mic threshold, incoming voice level, device choice and transcription | Added activation and keyboard push-to-talk, configurable threshold/key/volume and live microphone/speaker status. Input and output follow system defaults. In-game device selection, controller push-to-talk, transcription and a mic test remain open. |
| [Sea of Thieves in-game safety options](https://support.seaofthieves.com/articles/360035182774) | Voice with players encountered in the world and controls to mute other players | Added distance-based volume, an overall voice switch and individual rider mute. Reporting, persistent blocking and platform moderation remain open. |

No reference code or media was used. Voice uses CPAL 0.17.3 (Apache-2.0),
already pinned and vendored by the game audio stack.

## Using voice

Open **Voice Settings** in the multiplayer lobby or race menu. You can also
open **Settings → Voice Chat** from the main menu. Voice activation is the
default mode. Incoming chat is on by default. Turn on **Microphone** to transmit
during the online countdown and race. The game closes the capture stream while
the window is unfocused or settings are open. Push-to-talk defaults to **V** and
can use another letter key that is free of gameplay bindings. A local rider mute
lasts for the session.

The host derives proximity from its rider positions, including the walking
or ragdoll position when available. Volume fades to zero at 55 m. Frames
contain 40 ms of 8 kHz mono audio. The host validates frame size and encoding,
binds speech to the admitted rider and rate-limits per peer. Playback queues
are bounded and speech is dropped when the existing socket has queued output.
Protocol 3 clients and hosts must run matching builds. Voice travels in plaintext
over the existing player-hosted TCP session. Public demo release still needs
transport confidentiality, two-network audio testing, and voice quality and
moderation review.

## Verification

The checked-in tests exercise legacy preference defaults, menu adjustments
through the real panel input, voice frame encoding and malformed frame
rejection, and nearby-only relay over loopback TCP.

- Formatting, strict workspace Clippy, locked debug build, dependency/asset audit,
  roadmap index and five roadmap tests passed.
- The full workspace run reported 714 passes, two ignored and two pre-existing
  physics failures. The failures are high-speed ragdoll anchor error and
  Metropolis static-collider trajectory parity. Both are documented in the
  preceding menu and combat work.
- Native settings walkthrough passed its six-category save/reload checkpoint.
  It recorded persisted master volume, restored VSync, mph, keyboard
  push-to-talk and a disabled microphone. Programmatic viewport changes now
  refresh all window-targeted camera projections before Bevy prepares render
  attachments. [Settings receipt](settings-native.json)
  [Voice settings at 720p](voice-settings-720p.png)
  [Accessibility settings at 720p](accessibility-settings-720p.png).
- The protocol 3 admission fixture verified older-protocol rejection, private
  lobby admission, late joins and disconnect, and a full roster. An independent
  loopback fixture passed one valid voice frame through the host pump and
  discarded malformed speech without echoing to the sender.
  [Admission receipt](online-admission.json) [Voice wire receipt](voice-wire.json).
- A native Linux host using an isolated synthetic PipeWire source transmitted
  zero frames during silence and delivered ten nonzero frames to a real
  loopback connection after the source exceeded the activation threshold.
  [Native capture receipt](voice-capture-native.json).
- The full native frontend walkthrough passed 22 stages, all 16 rider portraits,
  and five measured viewports from 1280×720 through 3840×2160. The race-entry
  fixture now checks that the approved riders have their starting weapons and
  that the visuals stay holstered until an armed strike.
  [Frontend receipt](frontend-native.json).
- A native Linux host decoded synthetic packets from an admitted loopback
  peer and played them through CPAL into an isolated PipeWire output. The
  monitor captured nonzero output with all gameplay mix levels at zero.
  This confirms playback and does not measure continuity or headset quality.
  [Native playback receipt](voice-playback-native.json).
- A two-process native loopback race matched 371 authoritative snapshots.
  Both peers observed remote movement, attack, crash, on-foot recovery and
  remount without session errors. This was a same-machine protocol 3 race.
  [Native pair receipt](online-native-pair.json).
- The native Metropolis smoke test initialized four game-audio sinks, completed
  both crash/recovery sequences and reached the results fixture.
  [Metropolis smoke receipt](metropolis-smoke.json).
- The native Hills smoke test passed the city, town and hill route fixture
  with both remounts, zero exceptional rescues and working game audio.
  [Hills smoke receipt](hills-smoke.json).
