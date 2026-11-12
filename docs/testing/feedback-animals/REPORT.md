# Hills animal and mounted follow-up — 2026-11-11

Scope: RFU-MOUNT-01..03, HILLS-05 and V2-06-002. The user reported sinking
mounted horses, upright animals/cowboys after impacts, stiff cow legs and too
few mounted cowboys in the desert.

The previous mounted root was a single kinematic solid and was absent from the
NPC knockdown query. The new target reuses the existing quadruped ragdoll and
NPC occupancy/respawn path. On a confirmed mounted horse fall, the cowboy gets
a separate eleven-part NPC ragdoll; its bodies are removed at horse respawn.
Static saddled horses also enter the existing quadruped knockdown path. Crossing
cattle retain their preexisting collision and ragdoll, and their skinned leg
drivers now move in a four-beat walk during patrol. Horse grounding samples
all four hoof positions against resident eight-metre terrain triangles and
the crossing road surface. Three extra patrols populate later Hills desert;
existing town crossings keep their original route/clock assignment.

No external repositories, assets or reference code were used. Existing art is
procedural and the changed source is not a registered media asset, so no asset
hash entry changed. Player rider physics, eleven-body baseline, controls,
saves and non-contact scenic herds were not modified.

Verification receipts are pending final compile and native runs. Close human
review of the horse/cowboy fall silhouette and full-course desert density is
still required, as are matched performance and target-hardware measurements.

## Integrated native receipt (2026-11-11)

The combined release binary passed Metropolis and Hills `--smoke-test` with two remounts each; Hills had zero exceptional rescues. The Hills polish survey reported 11 mounted pairs, 13.04 m mounted travel and a confirmed mounted collision, and exited after seven screenshot captures. The added mount lifecycle and path tests passed in the 467-passing library run; only the two known Bevy/Rapier engine parity cases failed. Close fall silhouette and every desert patrol remain for human review.
