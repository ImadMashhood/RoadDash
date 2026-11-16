# Hills graphics and motorcycle feedback experiments

The options default to off. Hills road variants use the existing road installer
and collision. Countdown revs use the existing engine recordings and keep the
bike stationary. The accessibility setting for anti-nausea suppresses
experimental wind and heat as well as the camera shake.

Run the combined native trial from the repository directory.

```sh
ROADDASH_POLISH=all ROADDASH_POLISH_ROAD=ambient ./play-bevy20.sh --hills
```

Set `ROADDASH_POLISH` to a comma-separated subset of
`grade,wind,heat,rev,exhaust,shake`. Set `ROADDASH_POLISH_ROAD` to
`ambient`, `poly`, or `procedural`. The procedural comparison uses the
existing normal and roughness with a flat albedo. The imported comparisons
replace normal and roughness while keeping the original road albedo.
Start a race and hold the normal throttle during the countdown to hear revs.
Launch the separate GPU exhaust test with
`ROADDASH_POLISH=rev,hanabi cargo run --release --locked --features hanabi-experiment -- --hills`.

## Sources

- ambientCG Asphalt004 NormalGL and Roughness, and Poly Haven Asphalt Track
  OpenGL normals and roughness, are listed in the asset manifest with packed
  texture hashes. The experiment softens normals, remaps roughness and adds
  linear mips. [ambientCG asset](https://ambientcg.com/view?id=Asphalt004) ·
  [ambientCG CC0 terms](https://docs.ambientcg.com/license/) ·
  [Poly Haven asset](https://polyhaven.com/a/asphalt_track) ·
  [Poly Haven CC0 terms](https://polyhaven.com/license).
- The fullscreen pass follows Bevy's native render integration. The WESL shader
  is original RoadDash code. [Bevy post-process example](https://bevy.org/examples/shaders/custom-post-processing/).
- Hanabi 0.20.0 is an optional MIT/Apache-2.0 dependency matched to Bevy 0.20.
  The comparison emitter holds up to 32 particles. [Hanabi repository and compatibility table](https://github.com/djeedai/bevy_hanabi#compatible-bevy-versions).
- vkBasalt was unpacked into a temporary directory for the Vulkan trial. The
  external configuration tests SMAA and CAS at a sharpening intensity of 0.18.
  [vkBasalt project](https://github.com/DadSchoorse/vkBasalt).
- Material Maker 1.8 and 1.5p1 both segfaulted during isolated export. An
  upstream stock project also failed. The authored graph is available in
  `art/experiments/hills-asphalt.ptex`. An in-game Material Maker comparison
  requires a successful export. [Material Maker project](https://github.com/RodZill4/material-maker).

## Capture method

`tools/capture_polish.sh` runs the same Hills fixtures with a fixed racing
seed, isolated settings, and one effect per comparison. The 16 captures were verified at 1440 × 900.
The bridge shot holds
the camera still. The grid shot holds throttle. The speed shot starts in the
dunes and schedules the same upshift. Each PNG has a JSON companion for the
runtime effect values. Wind, heat, shake and exhaust change over time. The
stills sample those effects at one instant. The native offscreen X11 session
and short fixtures are suitable for visual inspection. The CPU/wall logs do
not establish a production frame budget.

## Matched comparisons

- [Asphalt](road-materials.jpg) shows the baseline, flat color with procedural
  normals, ambientCG, and Poly Haven. All use the same bridge camera.
- [Grid effects](grid-effects.jpg) compares stationary revving, CPU and GPU
  exhaust, and the localized heat shimmer. The rev fixture held approximately
  7,444 RPM at 0 km/h and the CPU exhaust showed five active puffs.
- [Camera and native shaders](camera-effects.jpg) compares the grade, wind,
  upshift motion, and combined trial. The isolated wind fixture was moving
  at about 173 km/h. The upshift capture recorded a 0.13 camera pulse.
- [Vulkan layer](vulkan-effects.jpg) uses window captures after presentation.
  The layer log confirmed that SMAA and CAS were applied. The native render
  capture is upstream of the external Vulkan layer.

The NVIDIA RTX 3080 ran the native Vulkan captures. The combined Hills
smoke test verified the roster, audio initialization, two remounts and the
results screen. Formatting, the default and Hanabi Clippy checks, the
locked build, the roadmap tests and the asset audit passed. Focused testing
verified rev response and release. The full Rust test run reproduced the known
biomechanical high-speed ejection and Metropolis static-chunk support failures.
These are outside the presentation changes. [Runtime capture values](captures.json) accompany the gallery. Art selection,
audio judgment and performance across a full human race remain open.
