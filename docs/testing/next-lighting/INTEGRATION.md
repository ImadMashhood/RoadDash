# Proposed shared record updates (root integrates)

## Decision

Use Bevy 0.17.3's existing EnvironmentMapLight with once-prepared, original
cosine-convolved diffuse and GGX-convolved roughness mips from the same procedural
sky as the visible cube. Correct world/cube Z; retain shadowed warm direct sun,
fixed exposure, tonemapping, MSAA, fog, cascade settings and caster policy.
Balance illumination through a reduced uniform ambient floor (35% of authored
ambient) and stronger directional environment (3x authored environment). This
preserves authoring input compatibility and keeps the exact older presentation
available via ROADDASH_LIGHTING=legacy. Default and any other value select balanced.
No dependency migration. No per-frame filtering, extra light, geometry or render
pass. This is static sky IBL, not scene-aware GI or local contact occlusion.

## Checklist

Extend existing SKY-01 evidence; P15-004 stays open for scalable native settings,
SSAO/SSR, temporal/hardware/performance acceptance. V2-06-007 retains user landscape
art and full route gates. No new requirement IDs/closures/count/placement changes.
Link docs/testing/next-lighting/README.md beside existing SKY-01/P15-004 evidence.
README and generated PROGRESS must be synchronized by root.

## Known issues

Static global sky IBL cannot darken an arch/interior from unseen neighboring
geometry, capture nearby reflections, or supply dynamic bounced sunlight. The
64-sample approximation and 128px specular cube trade reflection detail for bounded
startup/memory. Display sky remains 256px; no new cloud motion. Ground bounce is
an authored average, not current local ground material. Existing black HDR trial
is not reopened; no automatic exposure/HDR path. SSAO/AA changes deferred because
MSAA compatibility, temporal artifacts and additional GPU cost require separate
measurement. Human visual approval and Windows/target GPU gates remain open.

## Provenance and history

Research links/revisions/license observations: RESEARCH.md. The new filter is
original Rust implementation of standard math, using existing original procedural
radiance. No new media or copied implementation. Record src/sky/lighting.rs as
original in ASSETS/research registry as appropriate. Add a chronological history
entry with native measurements and checks from README after root integration.
