# Rider gear

Settings → Rider Gear offers Classic, Ember and Glacier suit finishes and
Classic, Apex and Rally helmet models. The controls apply to the local rider
and save with the rest of the settings. Old settings load Classic in both
slots. The cosmetic choices use stable names in JSON. Purchases, ownership,
career progression and remote cosmetic replication remain separate work.

Ember and Glacier are original UV-matched atlas treatments built from the
existing project-owned rider texture. Regenerate them with
`python3 tools/build_rider_suits.py`. They retain the existing suit geometry,
normal map and roughness map. Apex and Rally use original modeled shell and
visor profiles with seated and standing head binds. In the race, their mesh
and material handles are swapped on the existing skin entities. The meshes
carry no physics. Rider, runner and ragdoll ownership select the same local
gear. AI rivals and police retain their authored appearances.

## Verification

The native recovery fixture rendered Ember/Apex in Metropolis and
Glacier/Rally in Hills. Both pairs completed two remounts and reached
results. The final build repeated the Hills case with zero exceptional
rescues. [Native on-foot comparison](gear-preview.jpg).

All 13 focused model tests and six settings tests passed on the final
source. The model tests cover both helmet swaps, original appearance
restoration, separate AI appearance, unchanged suit mesh, recovery cleanup,
and differential physical body trajectories. The settings tests exercise
the gear tab, selection controls and settings serialization. Formatting,
strict Clippy, the release build, the asset audit and roadmap tests passed.
The final locked development build passed. The final workspace run recorded
724 passing tests, two ignored and the known Rapier failures in wall
ejection and static-chunk trajectories.
