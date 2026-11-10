# Portrait cast AI and living city — 2026-11-09

Branch `codex/living-city-ai`, base `b4dd5e7`. Extends all earlier clothing, grass,
verge, farm-prop and gameplay polish. User requests implementation across AI,
traffic, city population, civic landmarks, approach density and street layout.
ADR120 records decisions. No main merge/push or next major roadmap part.

## Implementation

- Race HUD/catalog names now use the approved portrait cast; stable rider/bike
  IDs and saves remain. Fourteen active opponents retain the15rider race format;
  George and Elif remain in the16portrait bar roster. Descriptor-derived pace,
  lane patience, aggression and kick preferences drive behavior. Eight named
  weapon loadouts replace random starting equipment: Esteban/Club, Osama/Whip,
  Luigi/Rolling Pin, Hog/Chain, Cleetus/Club, Abeyomi/Baseball Bat,
  Sha’queesha/Chain and Dolph/Club. Unarmed Tyrone/Ellen prefer kicks.
- Seven race-focused/seven aggressive riders remain; retaliation is identity
  specific and race-focused grudges expire after24seconds of active simulation.
  Stable lane intentions replace idle sinusoidal weaving. Curve lookahead brakes
  before bends. The real vehicle speed limiter respects the new character pace.
- Bounded fixed-step obstruction memory requests1.25seconds of parking reverse
  after2.5seconds stuck while requesting progress, with6second retry cooldown.
  A full rear chassis sweep must remain clear. Ordinary traffic queues do not
  request backing. Uses the existing force/reverse adapter, not a teleport.
- Cross-car recycling resets current/next Rapier poses and velocity, avoiding
  the140m one-frame jump becoming a collision impulse. Main-car recycling uses
  the same helper. Regression checks actual Rapier velocity across a wrap.
- Original64main cars keep their farm distribution;32additional cars are city
  only and recycle within urban bounds. Urban pedestrians96->192. More city
  junctions receive crossing cars. Farm cows/props and legacy population remain.
- Reproducible `tools/bake_metropolis.py` + `content/world/living_city.json`
  overlay: approximately20%more sidewalk area (a width approximation on irregular
  streets), landward0.689m expansion with bounded building setbacks. All19,068old
  building IDs/closed envelopes retained, each retaining at least65%of its old
  footprint; road carriageway and race routes/gates/fields are unchanged.
- Nine standalone homes/workshops increase approach density toward the city.
  The user-selected park3 (LC-S0152) becomes City Hall:174m frontage,116m
  copper clock tower, pediment and seven monumental arches facing the actual adjacent driving street. Park5 (LC-S0246) becomes the detailed fountain park, with grass across the full irregular block interior, promenade
  paths,26trees,11flower beds,15benches and a shaded pergola. Other parks remain.12covered
  building-side cafes leave a clear walking strip. Civic solid geometry retains
  compound collision independently of rendering. New zebra crossings and more
  visible lane paint project onto actual pavement triangles.

This is original procedural architecture, not downloaded/AI-generated bitmap art
or copied proprietary models. Existing cast descriptions, weapon models, facade
families and licenses are reused. No external repository research or dependency
migration in this pass. Pinned Bevy0.17.3/Rapier0.32 remain. Existing model-derived
far atlas is rebaked through the normal generator; family materials/hashes remain
consistent. Ragdoll anatomy/masses/joints/recovery controls are not tuned.

## Verification and remaining gates

`authoring.json`, `geometry-audit.json`, targeted reverse/recycle tests and native
folders contain implementation evidence. Independent audit samples whole1.9×3.9m
car footprints against baked asphalt, checks retained source paths/gates/fields,
old building identities/areas and full pedestrian corridors/cafe clearance.
Geometry boundary audits allow10micrometres for the existing1micrometre authoring
quantization; this does not allow meaningful road/building overlap.

New IDs CAST-AI-01, CITY-CIVIC-01, CITY-WALK-02 and CITY-LIFE-02 remain open for
integrated art/balance/performance acceptance. README/PROGRESS now220verified,
249open /469total. User art review, full-course long streaming, broad AI fairness,
GPU timing and Windows/hardware performance remain separate gates. The previous
16.7ms target was already unmet; denser city life is not a performance improvement
claim. Fountain jets are stylized static geometry, not a fluid simulation.

Integration findings: the first full suite caught the old1500street-furniture
ceiling after wider walks admitted1519valid placements (1521after the selected park relocation); ADR120 explicitly caps
this new city budget at1600. The deliberately broken old grounded-down-drive
positive control stopped reproducing its old rim trap on the remodeled pavement.
Its exact31surface/6KB local baseline fixture is now frozen from b4dd5e7; current
controller/road-crossing tests still use the new map. No physics behavior was
changed to satisfy that historical test. Cafe placement excludes their occupied
patios from both new furniture and full pedestrian sweep authoring; recovery
navigation includes civic foundations/basins/patios and park furniture.

The initial small public-site proposal and its native captures are retained under
`superseded-small-sites/`; they are not final art evidence. The user explicitly
selected numbered park3 and park5 using `park-location-options.png`.

## Final request checklist

All requested implementation items are integrated on `codex/living-city-ai`.
This confirms implementation and the listed checks, not subjective art/balance
approval or completion of the existing performance/hardware roadmap.

| Request | Implemented result | Verification |
| --- | --- | --- |
| New character names, weapons and traits | 14 dossier-linked opponents, 8 authored weapon loadouts, pace/patience/aggression/kick traits | Core cast/loadout/retaliation tests; native race/recovery |
| More capable AI and backing up | Stable lane intentions, bend lookahead, expiring grudges, blocked/rear-clear bounded reverse | Pure decision tests and actual dynamic reverse fixture |
| Increase only city traffic/pedestrians | 96 main cars (32 urban extras), 58 crossing cars, 192 city pedestrians; original farm distribution retained | Native census; all 32 extras above urban boundary; 187 pedestrians moved over 0.5m |
| Stop extreme crossing-car speeds | Reset current/next Rapier kinematic pose on wrap/recycle | Actual backend wrap velocity regression and native smoke |
| Gradual farm-to-city approach | 9 additional standalone homes/workshops at progressively closer stations | Baked placement audit and route captures |
| Grand City Hall | Park3 LC-S0152; 174m frontage, 116m clock tower, pediment, 7 monumental arches facing the adjacent driving street | Actual block containment/building-overlap audit and native civic capture00 |
| Detailed fountain park | Park5 LC-S0246; fountain, lawn, cross paths, 26 trees, 11 flower beds, 15 benches, 8 lamps, pergola | Block containment, persistent collider/navigation integration and native civic capture01 |
| Wider sidewalks and correct lanes | 1.2×sidewalk area, ~0.689m landward expansion; stronger lane paint and shared authored traffic paths retained | 8,572 whole-car footprints all on pavement, old IDs retained, native four-lane entry capture03 |
| Covered restaurants and crosswalks | 12 facade-side cafe canopies with tables/seats; zebra strips on actual carriageway triangles | Patio/whole pedestrian corridor audit; resident collision and native city survey |

## Final validation

- `cargo fmt --all --check`, strict locked all-target workspace Clippy,
  `cargo test --workspace --locked` (450 passed, 1 pre-existing ignored),
  `cargo build --locked` and release build all passed.
- 20 Metropolis Python authoring tests passed. Independent audit found zero
  whole-car pavement failures and zero civic/building overlaps; both selected
  park boundaries contain the authored areas and the hall entrance aligns with the adjacent driving street.
- `python3 tools/audit.py` and README/progress synchronization passed. This is
  provenance declaration/hash verification, not legal clearance.
- Five serial final native runs (`civic`, `after`, `recovery`,
  `baseline-recovery`, `actors`) exited0, with unchanged frozen input hashes
  and nonblank captures. Both native reverse checks measured -1.898m/s; both
  remounts on each recovery route had0.0000m pickup error. NPC human/quadruped
  contact and body bounds passed. The paired18-case ragdoll lab remained finite.
- Bounded native wall timing:19.91ms average,22.65ms p95;16.7ms target **not met**.
  This samples the existing survey workload, not full-course GPU/hardware timing.
  Fourteen racers remain simulated with the player (15 total); the camera moves
  through survey scenes. No claim that higher population improved performance.
- Census:19,077 buildings,1,521 street props,122 parked cars,16 cows and16 farm
  props;53,639 grass tufts in333batches. Civic meshes/prop-scene meshes are not
  included in the static chunk triangle census, so it is not a whole-frame budget.

Key files: `crates/rr_core/src/cast.rs`, `src/game.rs`, `src/environment.rs`,
`src/vehicle.rs`, `src/world_gen/metropolis/civic.rs`, `navigation.rs`,
`geometry.rs`, `tools/living_city.py`, `tools/bake_metropolis.py`,
`content/world/living_city.json`, generated Metropolis manifest and cast catalog.
ADR120 and the four new requirement IDs record scope/decisions. Their qualitative
art/balance and broad performance acceptance remain open; no next major part was
started. Main checkout was not merged or pushed. Final change is committed on
`codex/living-city-ai`; use `git log -1` for the commit containing this report.

Final user correction supersedes the west-facing iteration: the arched entrance
must face the driving street, not the city-centre direction. Heading now derives
from the closest actual race traffic-path point, with alignment dot1.0. Native
west-facing/rectangular-lawn evidence is retained in `superseded-west-facing/`,
not accepted as final. Park5 uses its entire irregular source boundary minus
pavement:60,964.44m² of continuous turf, with perimeter tree planting and a second
promenade/gardens in the formerly empty side. City Hall plaza likewise fills its
source block interior. `civic-placement.json` records areas, heading and input
hash; `audit_living_city.py` independently rejects reversed frontage or incomplete
block coverage. Superseded folders keep hall/park images and logs/receipts;
redundant route/smoke PNGs were omitted.
