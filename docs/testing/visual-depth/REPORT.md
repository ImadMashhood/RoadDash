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
