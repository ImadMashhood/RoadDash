# Character art shortlist — 2026-10-31

User references: five supplied screenshots of faceted clothed humans, and one
Road Rash rider sprite reference. Planning references only; no attached media
copied into runtime. Art direction: natural human proportions, faceted surfaces,
readable garment silhouettes, full-face helmets, padded jackets, fitted trousers,
gloves and boots; retain the established per-rider color schemes.

| Candidate / creator | Source and observed offer | Fit / remaining work |
|---|---|---|
| POLYGON City Characters / Synty Studios | https://syntystore.com/products/polygon-city-characters-pack — $29.99 USD at review; 19 characters including biker and punk; four color alternatives and three skin tones; FBX source; no animation clips | Closest ready-made style match by visual judgment. Adapt riding clothes/helmet and convert FBX to GLB. Bevy rig/skin/animation/ragdoll compatibility untested. |
| Universal Base Characters / Quaternius | https://quaternius.com/packs/universalbasecharacters.html and https://quaternius.itch.io/universal-base-characters — free Standard tier; paid Source tier; six base bodies, humanoid rig, average 13k triangles, FBX/glTF | Best flexible base for custom RoadDash riders. Requires original biker clothing, helmet and materials. Actual free-tier contents/archive not inspected. |
| Ultimate Modular Men / Quaternius | https://quaternius.com/packs/ultimatemodularcharacters.html — 11 characters, four interchangeable model parts, 24 animations, FBX/OBJ/glTF/Blend advertised | Alternative for clothed NPCs and prototypes. Visual fit and rig quality need an in-engine test; no motorcycle kit promised. |
| Male Biker – Rigged / Hisenberg | https://www.cgtrader.com/3d-models/character/man/male-biker-rigged — seller advertises rigging, GLB/FBX/Blend, realistic style, poly counts 17372/16786 | Existing biker outfit; potentially useful if more naturalistic art is preferred. Marketplace claims only: topology/rig quality and exact license have not been independently verified. |

## License and acquisition status

Synty One Time Purchase Licence:
https://syntystore.com/pages/one-time-purchase-licence
Permits adaptation and incorporation in games across engines. Source sharing is
restricted to the licensed team; raw models cannot be committed to a public repo.
No purchase/download performed. Restricted asset acquisition must be separate from
public source distribution, with a license receipt and exact file hashes.

Quaternius pack/itch pages advertise CC0, but the current general license
https://quaternius.com/license.html is QAL v1.0, updated 2026-08-28, and restricts
standalone asset redistribution. It says earlier acquired licenses are not changed
retroactively. That does not prove a new download is CC0. Record the exact license
bundled with any acquired version before importing or redistributing it. Candidate
status: UNVERIFIED for public raw-file redistribution.

CGTrader lists Royalty Free License (no AI). Exact acquired terms and public-source
redistribution permissions remain unverified. Not purchased/downloaded/imported.

These are ASSET SOURCE candidates, not adopted dependencies or investigated
GitHub repositories. No runtime assets, licenses or hashes are fabricated in
ASSETS.csv; add entries only for files actually acquired/adopted.

## Integration acceptance

Current humans are eleven separately posed procedural segments in src/human.rs,
shared across riding/walking/recovery/ragdoll. There is no existing skinned-avatar
pipeline to simply swap a mesh into. Bevy 0.17.3 already enables bevy_gltf; no engine
migration is justified. Start with one pilot model, not fifteen separate rigs.

Require a verified reusable humanoid skeleton, shoulder/elbow/wrist/hip/knee/ankle
deformation, riding pose, punches/kicks, walk/run, mounting and get-up. Map physics
bodies to bones and blend animation/ragdoll on the same avatar. Preserve separate
helmet/shirt/trouser materials, shirt-colored minimap, authoritative transforms,
joint limits, contacts and camera behavior. Check 15-racer CPU/GPU/frame time,
skin deformation and LODs before replacing all current models. Rough initial art
budget: 8–15k triangles for a near rider, reduced distant LODs; a proposal, not a
measured performance guarantee.

Recommendation: Synty is the closest immediate style reference; a customized
humanoid base is the more flexible long-term rider foundation. Final acquisition
and model selection remain open. No runtime code changed in this research pass.
