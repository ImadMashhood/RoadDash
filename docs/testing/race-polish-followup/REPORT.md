# Race polish follow-up

Metropolis, Hills, Valley and City Run finish 1,000 m earlier. Physical roads,
spawn positions, route branches and authored checkpoint IDs remain in place.
Checks beyond the new line are skipped. The handling circuit keeps its lap
distance. The Hills late cattle crossing stays within the playable interval.

The race HUD has a bottom-right digital speedometer with a fill gauge. Race
position and the closest rival’s signed course gap appear at bottom left.
A geometric red chevron points up for a rival ahead, and a green chevron
points down for a rival behind. The old map and full roster UI are removed.
The previous visibility fields still deserialize so existing settings load.

The same recorded wind loop was high-pass filtered at 110 Hz and low-pass
filtered at 2.6 kHz, with 10 dB of attenuation. PCM peak fell from −0.64 dBFS
to −9.41 dBFS, and RMS from −11.12 to −21.75 dBFS. The speed-dependent gain
is capped at 0.18. An in-game headphone check is still open.

Hills grass is closer to the roadside with combined mesh chunks and the
existing distance fade. At the densest full chunk, the triangle cap is
6,264 instead of 7,056. Valley adds grass on eligible triangles close to the
verge and parses its terrain plan once per course load, which removes
repeated parsing per terrain tile. Metropolis adds grass in the sparser
farm bands within its existing placement budget.

An existing Hills play session recorded 301 racing windows with a frame
time p50 of 19.36 ms, p95 of 32.22 ms and p99 of 36.61 ms. Median fixed
gameplay cost was 7.36 ms per frame and physics cost was 5.40 ms per frame.
This is a baseline from the previous build. A matched GPU and gameplay
comparison is still open.

The previous Hills session's city-opening windows had a raw frame P95 of
45.21 ms. The bridge windows had a P95 of 25.86 ms. City-opening median
main-world CPU time was 31.60 ms, so the bridge mesh consolidation alone
does not address the largest observed bottleneck.

The bridge renders with one continuous deck skin and 10 m buried abutment
ends. The existing sixty deck colliders still carry the physical load.
Cattle crossing speed drops from 2.8 to 1.35 m/s and drives the gait
cadence. Mounted riders animate thighs, shins and boots as separate
visual pieces. Waiting horses settle the rider's legs.

A mounted bike uses a swept soft-target check for pedestrians, cattle,
recovering riders and wrecks. It keeps its horizontal velocity while the
target is knocked down or shoved and the bike receives a bounded vertical
hop. Walls, cars and upright bike impacts still use their existing contact
rules. The passing planner checks a projected rider ahead early enough to
consider a clear side lane while the lead rider is braking.

Four original Engine Simulator presets produce separate car, pickup,
tractor and semi voices. Each has six 1,000–2,000 RPM load/overrun captures.
Vehicle speed and acceleration drive the mix with distance falloff and
up to six simultaneous traffic voices. The separate 689 cc motorcycle
voice remains the default. [Capture method and licenses](../../third_party/TRAFFIC_ENGINE_CAPTURES.md).

## Verification

Formatting, strict workspace Clippy, locked debug and release builds, the
asset audit and roadmap checks passed. The final workspace suite had 740
passes, two ignored tests and the two existing Rapier failures
(`wall_impact_and_high_speed_ejection_remain_bounded` and
`standalone_static_chunks_keep_contact_ccd_and_support_trajectories`).

Targeted regressions passed for the four finish windows, missed gates,
finishes on foot, braking-player and oncoming-traffic passes, soft contacts
in both vehicle adapters, nearby-rival HUD direction, wind headroom, all four
traffic banks, the spatial voice cap, bridge section joins, grass placement
and mounted limb motion. Native Hills release smoke on isolated X11
initialized audio and completed both recoveries with zero exceptional
rescues. The race capture showed both new bottom-corner HUD panels.
The Metropolis release smoke passed on the same isolated display with
both remounts and the results fixture. These native runs are functional
checks and were executed alongside a live session, so their frame timings
are unsuitable for comparison.
