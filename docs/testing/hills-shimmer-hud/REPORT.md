# Hills heat shimmer and race instruments

Hills uses the existing road post-process for ground shimmer. The old
transparent heat sprites and their material are gone. The shader displaces
the scene inside a soft low strip projected onto the road ahead. It fades into
the western town and strengthens in the dunes. The HUD composites separately.
The pass runs only for active effects and shares the existing grade, wind,
and optional exhaust pass. `ROADDASH_MAP_FX=0` disables ground shimmer,
`ROADDASH_MAP_FX=low` halves the strength, and anti-nausea disables it.

The bottom-right race display has a segmented speed arc, a centered digital
speed with the selected metric or imperial unit, and an engine RPM bar.
Countdown revs enabled with `ROADDASH_POLISH=rev` drive the same bar at
zero wheel speed. The regular scale follows the bike's engine speed and
clears when the followed rider is on foot.

Source is original RoadDash code. The existing Bevy 0.20.1 fullscreen
material and MSAA writeback pipeline were reused.
[Bevy post-processing example](https://bevy.org/examples/shaders/custom-post-processing/).

## Native evidence

The [instrument comparison](race-instruments.jpg) captures a stationary
countdown at 7,444 RPM and a riding view at 173 km/h. The
[desert comparison](desert-shimmer.jpg) uses the same chase position with
default effects and with `ROADDASH_MAP_FX=0`. Shimmer activation is zero at
the bridge and full in the dunes. The Hills map effects survey wrote six
frames, including the town and desert. It counted zero heat sprites and the
existing 48 sand sprites. [Capture values](captures.json).

The RTX 3080 Vulkan diagnostics recorded 0.128 ms for one fullscreen
effect pass at 1440 × 900 during the survey. This sample excludes the
HUD writeback and does not establish a full-route frame cost.

## Verification

Formatting, locked debug and release builds, strict workspace Clippy, the
roadmap consistency tests, and the asset audit passed. The focused HUD,
map effects, and rev tests passed. The native Hills smoke test completed
two remounts and the results fixture. The workspace tests reproduced the
known biomechanical wall/ejection and Metropolis static-chunk support
failures.
