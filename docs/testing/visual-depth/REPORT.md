# Visual depth pilot

Two startup switches make same-binary comparisons possible.

- `ROADDASH_DEPTH_LIGHTING=1` selects the existing analytic display transform
  in place of Reinhard and raises camera EV100 by 0.35 stops to offset its
  brighter midtones. The display sky uses the same curve. Hills also gets a
  lower, more lateral sun and stronger distance haze. The sun, sky, clouds,
  reflections and fog all use the same course profile.
- `ROADDASH_CONTACT_SHADOWS=1` enables Bevy contact shadows on the chase and
  menu camera, with eight samples over 45 cm. The existing shadow preference
  gates both kinds of shadows. The depth prepass adds rendering work, so this
  remains opt-in while the frame cost and foliage edges are evaluated.

The BUSS4 facade pilot uses the existing recessed glazing and modeled trim.
Darker, smoother glass and a darker cavity increase separation from the frame.
The material change is authored across all three source LODs. The storey kit,
model-derived atlas and Metropolis crops are rebuilt together. This pass leaves
a deeper physical storefront and block-level composition review open.

The implementation reuses existing RoadDash art, the pinned Bevy renderer and
the established facade toolchain. No external art or runtime dependency was
added. [Bevy contact shadows](https://docs.rs/bevy/latest/bevy/pbr/struct.ContactShadows.html)
and the repository's [earlier lighting research](../next-lighting/RESEARCH.md)
describe the rendering paths.

## Verification

- [Hills bend](hills-bend.jpg) and [distance comparison](hills-distance.jpg)
  show baseline on the left and the depth profile on the right at matched
  camera positions. The shallower sun gives terrain and trees longer shadows.
  Haze at 500 m rises from about 7% to 18%, with less than 1% at 100 m.
- [BUSS4 atlas crop](buss4-albedo.png) shows the ground band before and after.
  The [six-asset receipt](buss4-geometry.json) records unchanged geometry.
  Atlas and runtime crop verification passed for 383 bands and 15 mip images.
- The [Metropolis recovery fixture](metro-recovery.json) passed with both
  switches enabled on the rebuilt binary. It reached results after both
  remounts. An earlier contact-shadow survey also rendered all views.
- Both final Metropolis survey runs exited successfully. The [street pair](metro-contact.jpg)
  shows contact shadows off on the left and on on the right. The difference
  is subtle at street scale and is strongest on lit details.
- One sequential same-binary debug survey pair measured **28.76 → 29.55 ms**
  mean and **33.24 → 36.25 ms** P95 wall-frame time. Both exceeded 16.7 ms.
  This is a local wall-clock observation. Repeated release runs and GPU timing
  remain open.
- Formatting, strict Clippy, locked build, provenance, roadmap and facade
  checks passed. The game library recorded 490 passing tests, one ignored,
  and the two known Rapier failures. Core and content passed 228 tests with
  one ignored.

## Softer grade and Hills profiling

The opt-in grade uses an additional 0.10 EV of camera exposure compensation.
At neutral scene values around 1.8–1.9 after baseline exposure, this moves
the highlight response about a quarter of the way toward baseline Reinhard.
The amount varies with luminance and color. The scene and display sky use
the same exposure. Hills sun angle and fog density retain the depth profile
values.

An initial Hills playtest recorded about 50–57 ms per frame while racing.
The frame logs showed substantial main-world, fixed gameplay and physics
time. AI forecasts issued roughly 40,000–46,000 static sphere casts per
60 fixed ticks. Identical clear segments now share their result within a
rider’s fixed-tick planning pass. Blocked casts retain their hit diagnostics.
The existing `ROADDASH_AI_STATIC_CACHE=0` switch bypasses both caches.

The local before/after release pair ran sequentially with the compiler idle,
the same Hills launch fixture, default lighting, and GPU diagnostics enabled.
Both ran for 1,050 rendered frames. During race seconds 6–13, static casts
fell from 708 to 627 per fixed tick (11%). Median wall-frame time was
26.45 → 26.82 ms and P95 was 30.07 → 30.20 ms. Median sampled main-world
CPU time was 25.56 → 25.30 ms. The median sampled opaque 3D GPU pass was
2.70 → 2.22 ms. The GPU number describes one pass. Overall frame pacing
did not improve in this pair. The two native racer receipts were byte-identical,
including the sampled rider decisions and states. Whole-course performance and 60 FPS remain
open under PERF-08.

The native Hills recovery smoke with the softer grade passed both remounts
and reached results with zero exceptional rescues. Locked development and
release builds, formatting, strict Clippy, provenance and roadmap sync
passed. The workspace test run reported the two known Rapier parity
failures in high-speed wall ejection and Metropolis static-chunk trajectories.
