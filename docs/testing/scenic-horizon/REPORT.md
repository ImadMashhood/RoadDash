# Scenic horizon

The startup skybox generator paints two layers of distant silhouettes onto the
display cube. Metropolis and City Run use building profiles. Hills and Valley
use ridges and tree lines. The silhouettes fade at the fog edge. The same
direction-domain function feeds all six cubemap faces. Use
`ROADDASH_SCENIC_HORIZON=0` for the plain horizon.

Hills also retains more candidates in existing meadow and woodland batches.
The placement filters for road, bridge, farms and actor corridors still apply.
Sky silhouettes are visual only. Course collision and the sky radiance used for
environment lighting remain independent of the backdrop.

The art is generated from RoadDash's procedural sky and placement code. No
external textures or models are used.
