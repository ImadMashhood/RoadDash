# Traffic engine captures

RoadDash authored four exhaust presets with the same pipeline as the 689 cc twin in
ADR179. The presets run in Engine Simulator v0.1.11a and use its bundled exhaust
impulse responses. The shipped PCM contains the simulated audio output. [Engine
Simulator release](https://github.com/ange-yaghi/engine-sim/releases/tag/v0.1.11a) ·
[MIT notice](licenses/engine-sim-MIT.txt).

| Voice | Engine Simulator layout | Role |
| --- | --- | --- |
| Car | 1.8 L inline-four | Hatchbacks, sedans, sports cars, taxis, and vans |
| Truck | 4.6 L inline-six | Pickups |
| Tractor | 3.2 L inline-three | Hills farm tractors |
| Semi | 8.2 L inline-six | Hills tractor-trailers |

The heavy-duty presets approximate the exhaust sound using Engine Simulator's
spark-ignition model. Each bank contains fixed dyno captures at 1,000, 1,500,
and 2,000 RPM. Pull uses full throttle. Coast uses the simulator's 20% speed
control for a quieter recording. The runtime infers RPM and load from traffic
speed and acceleration. It selects up to six nearby traffic actors, keeps parked
art silent, and routes the voices through the engine mix. The default 689 cc
bike bank uses its original playback path.

Generate a preset with `python3 tools/audio/traffic_engine_sim.py <model>`.
Save its output in the simulator's `assets` directory and import it from
`assets/main.mr` alongside the stock theme call. In neutral, enable ignition
with A, start the engine with S, and enable dyno hold with H and D. Hold G and
scroll to step the target by 100 RPM from 1,000 RPM. Hold R for pull and E for
coast. Record the simulator output as 48 kHz mono signed 16-bit little-endian
raw audio. The shipped bank was captured under Proton 11.0 using an isolated
virtual X11 display and PipeWire monitor.

For each anchor, select a stable start in the capture and call the same script
with `--capture file.raw --windows 1000:pull:6.1 1000:coast:9.5 ...`.
The script phase-aligns to a dominant exhaust pulse, resamples to 256 frames
per crank cycle, and crossfades over two cycles at the wrap. Each clip stores
4,096 mono frames. RoadDash needs only the baked clips during play.
