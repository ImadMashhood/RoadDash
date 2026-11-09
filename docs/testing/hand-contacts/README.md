# Animation A — glove/contact refinement

ANIM-15 verified technical subset; ANIM-02/16 remain open. ADR078, 2026-11-08.

The runtime now distinguishes relaxed hands, wrapped grips and closed-finger
punch profiles. Left/right shapes follow their existing animation keys and
interpolate with the body; an attacking hand does not release the other grip.
Ragdolls without animation targets relax rather than retaining a punch.

The source mesh originally had rigid digits with too few lengthwise edges.
At model load, only glove surfaces receive at most three rounds of long-edge
refinement and four shared GPU morph targets. Hinge curls rotate position,
normal and tangent together. Body/UV/material identity and the original GLB /
.rdmesh sources remain unchanged; one extra glove surface is rendered per human.
Raw shared morph image data totals 894,240 bytes for both standing/seated binds,
not a per-rider allocation or a measurement of actual GPU resource overhead.

Typed Grip/Weapon socket entities attach to existing visual wrists with explicit
side/bind metadata. Seated socket offsets are calibrated against universal grip
points. Weapon gameplay/held-weapon art remains Part 07; sockets alone do not
implement weapons. No physical skeleton/body/joint/mass/motor/impulse/solver,
input, save, collision or recovery behavior changes.

## Native review

Open the [numbered gallery](gallery/index.html): 33 original native screenshots,
11 modes × 3 camera views. Hxx-y IDs, view names, related authored pose IDs,
known source sprite slots and exact-key versus dynamic-snapshot status are
recorded in gallery/PROVENANCE.json. Tuck 2030, extended punches 2222/2232 and idle 2364
are held exactly for close-up inspection. Other views are runtime snapshots,
not proof of a particular source phase. They are not extracted retail images.

```bash
cargo run --release --locked -- --animation-lab-view --hand-review
# Automated captures and shape assertions:
cargo run --release --locked -- --animation-lab-view --hand-review --animation-lab-smoke
python3 tools/package_hand_review.py
python3 tools/package_hand_review.py --check
```

Keys 1–8 select mounted modes, 9 idle, 0 walk, R run, C camera. Omit --hand-review for
the original whole-body animation lab with full dynamic playback. The hand lab
uses a camera fill light to inspect black glove surfaces; gameplay lighting is
unchanged. These are draft gloved shapes; final thumb/finger contours,
self-overlap and handlebar clearance still need art review. No claim of final
anatomical fingers, finger-bone articulation or simulated tendons.

## Verification and limits

- 317 workspace tests passed, one existing ten-minute soak ignored. Includes
  original model/physical-body/180-tick trajectory and full recovery tests.
- Content rejects invalid hinge ranges/order, quaternion/socket authoring,
  duplicate/missing/busy-hand fist keys and unknown payload fields.
- Tests cover independent side weights, smooth bounded grip/fist/open blending,
  fixed wrists, finger-section length, universal neutral grip socket error < 6 mm,
  finite GPU deltas, non-hand isolation and two shared morph surfaces < 16 MiB.
- Native hand review observes relaxed, both gripping, left fist/right grip and
  right fist/left grip; all 33 views packaged with source/image hashes. Native
  Metropolis restart/audio/reverse/farm-and-city crash/run/remount/results passed;
  reverse remained −1.827 m/s and both remount errors 0.0000 m. These are scripted
  fixtures, not a full human race or hardware performance certificate.
- fmt, workspace clippy with warnings denied, workspace tests, locked debug /
  release builds, asset audit, roadmap checks/sync, coverage/source-context tests
  and git diff --check passed; raw receipts are alongside.

The later [Part02 authoring pass](../part02-authoring/README.md) aligns the static
Blender profile and calibrated socket fitting, and verifies actual GLB reload.
ANIM-16 remains open for exact source/runtime tessellation/hard-normal parity,
silhouette/clearance and user approval. ANIM-02, full V2-02 catalog/keyframe/
approval and ANIM-09 production motion gates stay unchecked.
The absence of a Windows/controller receipt and broad performance acceptance is
retained. A working GPU shape is not approval of its silhouette.

## Change record

Runtime: src/models/hands.rs (new), models.rs, animation.rs, animation/preview.rs,
world.rs and biomechanics/view.rs (image asset wiring only). Content:
content/hand-contacts.json and rr_content::hand_contacts (new typed validation).
Tests: model/shape/pose/content regressions. Authoring: new native packaging tool,
numbered gallery/provenance and review evidence. Documentation: README/progress /
placement, ADR078, issues/history/test matrix and original asset records. No files
removed or new dependency adopted; existing Bevy v0.17.3 API study is recorded
under R08 with MIT/Apache licensing. No external source or media copied.

Roadmap: ANIM-15 technical integration checked in Part 00; ANIM-16 authoring
round-trip added to Part 02; ANIM-02 remains partial. 205 verified tasks / 243 open,
448 total. The next work is static authoring alignment, thumb/grip clearance and
full pose-family review, before declaring Animation A complete.

## Part03 recapture — 2026-11-08

After animation.rs gained a shared fixed strike clock, its old source-hash check
correctly failed. The current release was run again with --animation-lab-view
--hand-review --animation-lab-smoke, then package_hand_review.py and --check.
All33 current captures, source hashes and actual held glove/socket checks pass.
These held poses do not validate live melee scheduling; see
[Part03 runtime contact evidence](../part03-runtime/README.md). Final art,
source/runtime normals/tessellation and user acceptance remain open.
