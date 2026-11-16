# Western town bends and AI lean

The previously straight western main street has a broad left-right bend
between stations 7,450 and 8,500 m. Both Hills routes use the same authored
points after the merge. The road remains at the original town elevation.
Road, shoulder, traffic and nearby frontage follow the updated course spline.

Both vehicle adapters now use the same rival bank rule. Full steering at
3 m/s can request at most 0.1625 rad of visible lean. The cap reaches its
normal maximum at 12 m/s. Visible roll changes by at most 1.8 rad/s, so a
reversal goes through upright. The player bank logic and physical steering
inputs retain their existing rules.

The final Hills revision survey and recovery smoke exited successfully.
The recovery fixture completed both remounts with zero exceptional rescues.
The [native western main street view](main-street.jpg) shows the bend and
nearby frontage. Focused tests passed for the shared route, station-speed
bounds and low-speed bank reversal. The smoke logged audio buffer underruns.

Locked builds, strict Clippy, formatting, roadmap checks and the asset audit
passed. Workspace testing retained the two known Rapier failures.
