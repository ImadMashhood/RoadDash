# Execution roadmap v2 — 2026-10-31

The user-directed Parts 01–08 below define acceptance criteria for the revised execution order. README is the single live checklist: all verified requirements now live in Part 00; unfinished tasks, including carried-forward original requirements, live in Parts 01–16. Original P01–P08/BW requirements are also preserved in the frozen BLUEPRINT_PARTS_01_08.md snapshot. Stable IDs retain their original numbers; current README placement determines execution order. TASK_PLACEMENT.json maps every task. Parts 09/10 retain their existing gates. Parts 11–16 remain cross-cutting/future requirements.

Restart, AI steering, pre-contact momentum and visual-limb fixes were verified previously and now appear in README Part 00. Their acceptance rows below remain for evidence; the unfinished Part 01 starts at V2-01-005. Measured vehicle-contact observations and the default combat swerve/countersteer are integrated partially; over-car trajectory/vault and gameplay acceptance remain open. Active recovery, animation and world acceptance retain their separate gates. See testing/part01-runtime/README.md for the current29-task audit.

For each new checkbox, the per-part reason/dependencies below apply along with its row-specific acceptance and verification. Research entries are candidates, not adopted code/dependencies.

<a id="part-01"></a>

## Part 01 — Stabilize restart, rival steering and crash entry

**Reason:** Preserve the user-approved player handling and passive ragdoll while correcting reported regressions.

**Dependencies:** Accepted ART-03 build; existing fixed simulation and tests.

| ID | Requirement | Acceptance | Verification |
| --- | --- | --- | --- |
| V2-01-001 | Restart at the authored grid, including on a zero-physics-tick frame | Reset all racer transforms, chassis state, velocity, interpolation history, gates, visibility/recovery and camera focus; no mid-course resume. | Regression from racing/paused/results at 3.2 km; native retry and cleanup checks. |
| V2-01-002 | Give AI proportionate, gradual steering and lean | Small lane corrections produce shallow banks, reverse through neutral, and avoid repeated full-lock weaving; player steering feel stays unchanged. | 30/60/120 Hz lane-response tests, crowd driving captures and steering telemetry. |
| V2-01-003 | Use pre-contact world momentum for physical knockoffs | The velocity sample used to detect an impact also seeds detached rider/wreck motion; a post-solver backward bounce cannot become the spawn velocity. | Synthetic pre-forward/post-backward contact regression; native collisions and ordinary momentum tests. |
| V2-01-004 | Repair visual skeleton attachments and anatomical bending | Source-model joint pivots and connected chains preserve limb lengths; elbows/knees do not twist or hyperextend; existing physical body/joint settings remain unchanged. | Source-bind and extreme-pose endpoint/hinge tests, physics differential trace and native riding/running/crash review. |
| V2-01-005 | Design and implement measured vehicle-impact and over-car crash responses | Record both world velocities and closing speed; distinguish rear, oncoming, side and grazing hits. Prototype bounded nose-down pitch/vault over different car heights through solid contacts, without teleporting or confusing closing speed with ground speed. | 100/60 same-direction case (40 closing), 100/60 oncoming case (160 closing), stopped/cross traffic, angular momentum/CCD/clearance and recovery comparisons. |
| V2-01-006 | Add a combat knockoff loss-of-balance sequence | A decisive blow causes a short outward swerve and smaller countersteer before separation; retain useful forward momentum, honor obstacles and preserve the three-direct-hit rule. | Left/right symmetry, forward/lateral velocity bounds, interruption/contact scenarios and recorded gameplay review. |
| V2-01-007 | Compare the reference crash/control code and evaluate bounded physics changes | Pinned observations separate original rules, current bugs and new arcade choices; no blanket engine migration or unmeasured ragdoll rewrite. Active recovery work is deferred to Part 03. | Reference notes, before/after traces and ADR for every behavior change; existing passive ragdoll fixture still passes. |
| V2-01-008 | Exit gate: accept corrections before authoring motion | Native restart, AI, contact and anatomical tests pass; user review confirms retained feel; unresolved vault/swerve criteria remain open. | Full regression suite, actual gameplay clips and explicit acceptance of changed crash responses. |

<a id="part-02"></a>

## Part 02 — Animation A — reference study, labeled keyframes and pose approval

**Reason:** The static library must become a complete reviewed pose vocabulary before interpolation.

**Dependencies:** Part 01 accepted; original authored avatar/bike and existing numbered pose/contact data.

| ID | Requirement | Acceptance | Verification |
| --- | --- | --- | --- |
| V2-02-001 | Build a complete animation/use-case catalog | Every action has a stable name/ID, state, side, source frame IDs or original-pose ID, confidence, contacts, transitions and expected use; do not invent recovered timings. | Catalog coverage audit against all gameplay states and sprite slots. |
| V2-02-002 | Inspect all original rider animation categories | Study riding banks, acceleration/braking/tucks, punches/kicks, weapons, hit reactions, crashes, mounting/dismounting, get-up and all running frames; empty/duplicate/view-distance slots are labeled. | Pinned local-disc/reference-code notes, sequence/contact sheets and coverage counts; proprietary images stay in local research. |
| V2-02-003 | Research Synty/POLYGON-style games and authentic locomotion | Identify verified games using that asset family; compare starts/stops, slopes, stride, planted feet, turns, reach and weight. The RoadDash rider is independently authored, not a licensed Synty asset. | Publisher/developer sources plus recorded observations; exact pack or internal technique stays unverified unless supported. |
| V2-02-004 | Investigate animation, IK and active-ragdoll repositories | Compare the pinned shortlist for algorithm, license, assets, engine compatibility, maintenance, performance and preservation of accepted physics. | docs/research/ANIMATION_AND_RAGDOLL.md and REPOSITORIES.md; prototype results required before adoption. |
| V2-02-005 | Author and label the full original 3D keyframe set | Create semantic windup/contact/follow-through/recovery and locomotion/contact poses with bike-specific seat/grip/peg anchors, including new vault and combat swerve poses. | Pose files, original sprite-number links where known, use-case labels and contact-error checks. |
| V2-02-006 | Cover running, on-foot recovery and non-rider characters | Include idle/walk/run/sprint/turn/start/stop, stumble/brace/roll/get-up/mount and pedestrian/animal-specific poses; do not force human anatomy onto cows. | State coverage matrix, terrain stance poses and separate quadruped review. |
| V2-02-007 | Publish a numbered pose gallery with anatomical review | Check silhouette, joint ranges, grip/fist placement, weapon sockets and source-model continuity before any new clips are approved. | Front/back/side/action screenshots and joint/contact validation; no completion based solely on file counts. |
| V2-02-008 | Exit gate: keyframes and catalog approved | All required use cases have reviewed original poses and resolved contact states; missing motion references are explicitly documented. | User/art review and a complete pose-to-gameplay mapping before Part 03. |

<a id="part-03"></a>

## Part 03 — Animation B — interpolation, terrain locomotion and active recovery

**Reason:** Turn approved poses into responsive motion that respects physics and terrain.

**Dependencies:** Part 02 approved catalog/poses; Part 01 contact and rig corrections.

| ID | Requirement | Acceptance | Verification |
| --- | --- | --- | --- |
| V2-03-001 | Implement a data-driven animation state/blend graph | Use stable clip IDs, event timing, interruptibility and transition rules; no animation root motion overwrites authoritative bodies. | State/transition/event-dedup tests at 30/60/120 Hz and native motion review. |
| V2-03-002 | Interpolate approved keyframes and combat/recovery transitions | Quaternion interpolation, easing and additive reactions preserve input response, contact phases and source-model consistency. | Recorded bank, attack, hit, vault, dismount and mount transitions without snaps or planted-contact drift. |
| V2-03-003 | Make walking/running match velocity and terrain | Blend acceleration/deceleration, stride length/cadence, turn-in-place and directional movement with foot IK, pelvis adjustment, slope/stair/curb probes and foot locking. | Flat/uneven/hill/stair/sidewalk tests; measure foot slide, penetration and popping at multiple frame rates. |
| V2-03-004 | Keep hands and feet on bike contacts when required | IK/pole targets constrain seat, grips and pegs while freeing attacking limbs; future bikes provide contact frames rather than bespoke rider meshes. | Different bike-contact layouts, reach rejection and held/released contact tests. |
| V2-03-005 | Prototype bounded active ragdoll and player attempts to recover | Conscious riders may brace, roll or regain control through capped forces/torques. Preserve joint limits, collision and gravity; unconscious mode cannot steer or inject unlimited energy. Keep a passive baseline and accessibility auto-assist. | Compare passive/active scenarios for energy, contact stability, input authority, recovery time and feel; ADR and user approval before replacing defaults. |
| V2-03-006 | Blend physical landings into terrain-aware get-up and remount | Handle face-up/down, slope and obstructions; align pelvis/support contacts and select a clear get-up direction without teleporting through traffic or scenery. | Repeated hill/flat/confined recovery, secondary impacts, camera continuity and remount tests. |
| V2-03-007 | Add motion/audio polish without changing gameplay outcomes | Use restrained head/torso look, breathing, gait weight, landing anticipation and footsteps tied to contacts; bound 15-racer/NPC CPU/GPU cost. | Optional features tested independently for readability, determinism, duplicate events and performance. |
| V2-03-008 | Exit gate: motion is responsive, coherent and physically readable | User accepts riding/running/combat/recovery; terrain/contact and performance checks pass; no passive ragdoll regression remains. | Native clips, quantitative foot/contact metrics, frame-rate regressions and human review. |

<a id="part-04"></a>

## Part 04 — World A — facade study, original textures and procedural buildings

**Reason:** Rebuild the original facade vocabulary in the approved new art direction, then generate a convincing city/world.

**Dependencies:** Part 03 motion gate; existing Valley structural layout, supported pads, streets and collision.

| ID | Requirement | Acceptance | Verification |
| --- | --- | --- | --- |
| V2-04-001 | Catalog every reference building/facade family locally | Extract and index the available original facade/roof/window/shop reference textures for research, recording dimensions, palettes, use and provenance; raw retail files remain outside public/runtime assets. | Local extraction manifest and coverage/contact sheets; no claim that extraction grants distribution rights. |
| V2-04-002 | Create original facade textures in the RoadDash style | Redesign the facade families as cohesive original atlases/PBR materials: dark/full colors, readable trim, doors/windows, storefronts, signs, roofs and wear, with source-to-new-family mapping. | Art comparisons, material/tiling/texel-density checks and provenance for actual exported media. |
| V2-04-003 | Plan a data-driven city/world construction grammar | Define connected street grids, blocks, touching frontage, sidewalks/alleys, building roles, farm-to-city-to-hill transitions, density and sightlines before generating new meshes. | Approved top-down plan plus road-height previews and explicit variation rules. |
| V2-04-004 | Generate detailed building archetypes and whole blocks | Use the approved facade kit for geometry, depth, roofline/height variety, corner/side/rear faces and coherent connected frontages, not detached random boxes. | Placed building/texture coverage, facade screenshots and repeatability checks. |
| V2-04-005 | Retain supported ground, roads, collision and navigation | Preserve canonical quotas/stable IDs/rights of way, independent render LOD/collision, pad support and walking/recovery access while adding art. | World inspection, geometry/ground/route tests and native survey; allocation counts alone do not prove placement. |
| V2-04-006 | Validate city life, visual density and performance | Review street/building/sidewalk proportions, cross traffic, street furniture, tree spacing and distant depth at riding speed and on foot. | Full route survey, actual street captures, readable hazards and recorded frame-time budget. |
| V2-04-007 | Exit gate: coherent procedural buildings are accepted | All facade/building families have original production art and useful variation; streets remain playable and visually connected. | Coverage matrix, user art review, on-foot recovery and performance receipts. |

<a id="part-05"></a>

## Part 05 — World B — props, materials and procedural dressing

**Reason:** Repeat the reference-to-original-art-to-procedural workflow for props.

**Dependencies:** Part 04 approved buildings and layout grammar.

| ID | Requirement | Acceptance | Verification |
| --- | --- | --- | --- |
| V2-05-001 | Inventory reference prop categories and gameplay roles | Classify fences, signs, lights, benches, bins, poles, trees, farm kit, parked vehicles and hazards; record source/rights and required collisions. | Complete reference-to-original prop-family table. |
| V2-05-002 | Author original prop meshes and texture families | Match the rider/building art direction with coherent scale, PBR response, silhouettes, variants, damage readability and LOD. | Turntables/material review, source exports, provenance and budgets. |
| V2-05-003 | Place props through semantic rules | Use streets/sidewalks, lots, farm parcels, junction sightlines and biome roles; preserve clear riding/recovery space and avoid repetitive tree spam. | Seeded layout, overlap/clearance and road-height capture checks. |
| V2-05-004 | Give gameplay props reliable physical/audio behavior | Use appropriate solid proxies, hazards and contact sound/VFX; keep decoration-only surfaces explicit and avoid per-triangle physics by default. | Bike/rider/runner collision scenarios, contact events and performance tests. |
| V2-05-005 | Exit gate: dressed world remains playable | Required prop families are present, stylistically consistent and physically legible throughout the route. | Coverage, user review, recovery routes and native performance survey. |

<a id="part-06"></a>

## Part 06 — World C — NPCs first, then sky/terrain/landscape art

**Reason:** Repeat the same original-art and procedural-placement pipeline in the user-requested order.

**Dependencies:** Part 05 prop/world gate; Part 03 motion systems.

| ID | Requirement | Acceptance | Verification |
| --- | --- | --- | --- |
| V2-06-001 | Build NPC reference catalog and original character variants | Study pedestrians, residents, farmers and drivers; author coherent outfits/bodies/LOD with shared safe rig interfaces rather than copying original characters. | Family coverage, source provenance, animation/skin and collision review. |
| V2-06-002 | Create animal-specific art, animation and physical responses | Cows/wildlife use appropriate quadruped proportions, gait, idle and ragdoll bindings while retaining solid gameplay contacts. | Anatomy/gait/contact tests and native animal-impact review. |
| V2-06-003 | Populate the world with believable NPC activity | Use sidewalks, businesses, farm spaces and crossing rules with bounded density, safe spawns, readable avoidance and independent physical reactions. | Deterministic busy-world scenarios, traffic/pedestrian collisions and recovery routing. |
| V2-06-004 | NPC exit gate before the landscape pass | Humans and animals look coherent with buildings/props and use the approved motion/physical interfaces without body or entity leaks. | Native crowd/animal captures, full recovery and frame-time evidence. |
| V2-06-005 | Study skyboxes, atmosphere and landscape references | Inventory sky/horizon, lighting, terrain/road/grass/rock/water/foliage materials; keep extracted originals local as references. | Reference manifest and approved original environment art bible. |
| V2-06-006 | Author original sky and landscape textures/materials | Create matching sky/horizon/lighting/fog and terrain/foliage/road materials, with natural variation and clear road edges. | Lighting/color review, material/tiling/LOD checks and actual asset provenance. |
| V2-06-007 | Integrate landscape art with the playable world | Support rolling hills, farms, city outskirts and distant mountains without terrain crossing roads, floating foundations or blocked sightlines. | Geometry/support regressions, elevated walking/recovery and full route survey. |
| V2-06-008 | Landscape exit gate: the complete first-map art pass is accepted | Buildings, props, NPCs, sky and terrain read as one art direction and remain performant at riding speed. | User visual review, coverage matrix, collisions/navigation and native performance. |

<a id="part-07"></a>

## Part 07 — Weapons and complete riding combat

**Reason:** Add functional weapons after the motion and world-art foundations are accepted.

**Dependencies:** Parts 02–06 accepted; preserve existing unarmed direct-hit rules.

| ID | Requirement | Acceptance | Verification |
| --- | --- | --- | --- |
| V2-07-001 | Research weapon behavior and define original data/assets | Study original weapon reach/side/timing, author licensed/original weapon meshes, sockets and per-weapon tuning. | Pinned observations, provenance, validated content and no unlicensed media. |
| V2-07-002 | Integrate weapon animations, contacts and outcomes | Equip/holster/attack/recover with authoritative hit windows, stamina/cooldown, effects/audio and physical knockoffs; maintain side selection and interruption. | Timing/contact/LOS tests, duplicate-hit protection, left/right symmetry and native fights. |
| V2-07-003 | Teach rivals fair weapon use and tune combat | Individual personalities choose measured attacks and defenses with readable tells; preserve recoverability and avoid instant grid pileups. | Deterministic AI/combat scenarios plus human difficulty/feel review. |
| V2-07-004 | Exit gate: weapons improve the race loop | Core weapon families function in traffic, on slopes and during interrupts/recovery without breaking unarmed play. | Multi-racer combat, recovery/performance regressions and user acceptance. |

<a id="part-08"></a>

## Part 08 — Police, pursuit and arrest/escape loop

**Reason:** Complete the police loop before returning to the original Part 09/10 gates.

**Dependencies:** Part 07 combat and complete first-map art/motion foundations.

| ID | Requirement | Acceptance | Verification |
| --- | --- | --- | --- |
| V2-08-001 | Research original police behavior and author police content | Document detection, pursuit, engagement and arrest/escape rules; build original uniforms/vehicles/animations using the approved pipeline. | Pinned behavior comparisons, content validation and asset provenance. |
| V2-08-002 | Implement bounded police pursuit and interceptions | Use authoritative state and road-aware driving with US traffic, measured aggression, target selection and explicit spawn/despawn budgets. | Deterministic pursuit/traffic/obstacle scenarios with no teleports or unbounded cops. |
| V2-08-003 | Integrate rider knockoff, on-foot pursuit and arrest/escape | Police interact with rider/bike/recovery independently; arrest/outcome rules preserve race/save integrity and keep escape readable. | Mounted/downed/on-foot cases, interrupted remount, retries and idempotent outcome tests. |
| V2-08-004 | Tune police feedback and fairness | Readable warning, chase, escalation and outcome feedback supports skill and player agency without ruining the accepted handling/ragdoll loop. | Native chase recordings, accessibility/controller and human balance review. |
| V2-08-005 | Validate the complete offline loop before Part 09 | Traffic, rivals, weapons, police, world art, physical recovery, finish/results/restart and saves work together within the frame budget. | Full end-to-end race/soak, corruption/retry tests and multi-hardware measurements. |
| V2-08-006 | Exit gate: proceed to existing Part 09, then Part 10 | Do not automatically start networking or silently close old slice requirements; retain all unmet original gates. | User acceptance and the unchanged P09/P10 checklists. |

## Historical-to-current crosswalk

| Preserved source | Current execution home |
| --- | --- |
| P01 workspace/CI/authority | Ongoing baseline maintenance, V2-01 regression gate |
| P02 track/graybox/BW | Existing foundation; V2-04 through V2-06 world-art gates |
| P03 vehicle/P04 camera-input | V2-01 corrections; V2-03 presentation and motion |
| P05 combat/weapons | V2-02 keyframes, V2-03 motion, V2-07 weapons |
| P06 ragdoll/recovery | V2-01 targeted crash fixes; V2-03 bounded active recovery |
| P07 race/HUD/results | Ongoing regression protection and original Part 09 |
| P08 AI/traffic/police | V2-01 rival steering, V2-06 world NPCs, V2-08 police |
| ART-02/04/05 and P11 art | V2-02 through V2-06; older production gates remain open |

The current RoadDash character is independent authored art. Synty/POLYGON is a style/research reference; no Synty source assets are licensed or imported by this change. Original retail textures/sprites may be extracted locally for study, but runtime/public assets must be original outputs or separately licensed. Building reference-to-new-family coverage is required; copying retail pixels into production is not an implied step.

## Physics scope

Retain accepted player handling and passive ragdoll defaults as the comparison baseline. The user authorizes targeted crash-entry/vehicle-response/combat-balance corrections and investigation of active recovery. That is not a blanket engine migration or permission to discard the eleven-body rig, limits, collisions, tissue damping or recovery. Each physical behavior change needs before/after traces, an ADR and tests. Animation may drive visual bones; active recovery must use bounded simulation forces, never pose teleports.

For 100 km/h rider and 60 km/h same-direction car, closing speed is 40 km/h. For oncoming 60 km/h car it is 160 km/h. Neither number alone determines post-impact ground speed: contact normal, vehicle motion, masses/kinematic assumptions and chosen arcade response must be recorded.

## Repository investigation gate

Start with the pinned comparison in ../research/ANIMATION_AND_RAGDOLL.md: active-ragdolls, Hairibar.Ragdoll, JoltPhysics, bevy_mod_inverse_kinematics and ozz-animation. Check compatible tags and complete licenses/assets before reuse; source or media is not imported by roadmap creation. Broaden the comparison when a concrete subsystem needs it, and record every investigated repository/version/decision in ../third_party/REPOSITORIES.md.
