# Hills road and shoulder thickness

The screenshot’s race time matches the Hills playtest near station
10,878 m in the dunes. The asphalt and dirt shoulders were rendered
as thin ribbons. A shoulder lip could sit above the coarser terrain
mesh and expose a gap when viewed on foot or from below. The old lip rose
0.28 m above the base terrain in the sampled dune section. Nearly
coplanar terrain and shoulder surfaces also competed in the depth buffer.

Both generated Hills routes now have render-only asphalt walls and
undersides. Dirt shoulders get textured outer walls. Each wall extends
straight down in world Y to at least 0.30 m below the lower of its
top edge and sampled resident ground. The original top vertices and
triangles still define the colliders and the rendered top. Sidewall
vertices are separate so they do not affect the original top normals.
The new faces share their current chunk meshes and draw calls.

Asphalt, shoulders, and the resident terrain use separate depth offsets
at nearly coplanar joins. The imported opening city and the bridge’s
existing supported deck retain their established geometry. No physics
or track surface definitions changed.

The screenshot location is covered by a regression that checks the
original shoulder geometry and the new intersection below the terrain.
A second test checks both routes, top-face identity, and wall and
underside winding. The bridge continuity regression stays part of
the focused test set.

The native Hills surface survey completed its asphalt and sand runs near
11.0 km. Both wheels stayed grounded in each sampled pass. The sand-side
capture shows the asphalt edge meeting the shoulder. The camera angle
differs from the on-foot screenshot. The exact screenshot station was
checked separately in the geometry fixture.

Hills native smoke completed both remounts and the results fixture with
zero exceptional rescues. Formatting, strict Clippy, locked builds,
roadmap synchronization, and the asset audit passed. Workspace
verification retained the two existing Rapier failures for wall-impact
bounds and Metropolis static-chunk contact/CCD parity.
