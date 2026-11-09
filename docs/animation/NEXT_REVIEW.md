# Animation A restart — 2026-11-08

Part 02 is resumed with a source-to-runtime coverage audit, before changing poses
or interpolation. Preserve the context-first catalog; the original V1 slot
labels were superseded and must not regain authority.

Rebuild `runtime-coverage.json` with:

```bash
python3 tools/report_animation_coverage.py
python3 tools/report_animation_coverage.py --check
python3 tools/test_animation_context.py
```

The [readable coverage table](RUNTIME_COVERAGE.md) and hash-bound report join 41 context sequences / 138 draft human poses to 13
shipped clips. It preserves source phase labels, entry/exit, interruptions,
held contacts and review status; each sequence explicitly lists matching clips.
104 context poses are absent from standalone runtime clips. This is a coverage
gap inventory, not104 broken animations: many are specialist/weapon/reaction
poses, while procedural locomotion, steering and physical crashes use other paths.
The two runtime-only keys2900/2901 are authored lean layouts; they are not newly
recovered source frames. Durations are RoadDash tuning and can be retimed by
playback, never recovered native timing or measured input-to-impact latency.

Next authoring review: ANIM-02 hands/grips, ANIM-04 support/recovery/specialists,
ANIM-06 torso/contact/silhouette fidelity and ANIM-03 unresolved caller/timing
labels. Use the context catalog and runtime report side by side; capture
front/rear/side views before approving each family. Weapon/police bindings remain
future gameplay work. Existing V2-02 catalog/keyframe/approval gates remain open;
ANIM-14 verifies this audit only. No character, animation, physics or external
asset is replaced in this restart.


## Glove/contact implementation follow-up

ANIM-15 adds runtime grip/fist shapes and typed visual sockets, with
[native close-ups](../testing/hand-contacts/gallery/index.html). Existing body
poses/clips remain unchanged; this is technical hand integration, not approval
of the full pose catalog. ANIM-02 retains final hand silhouettes/clearance;
ANIM-16 tracks aligning the static Blender recipe with the runtime profiles.

## Part02 authoring pass — 2026-11-08

ANIM-16 now uses the runtime hinge/curl/socket JSON in Blender, with four
independent keys and complete154-layout assembly hand states. Original weapon
proxies attach at the authored typed socket. See [export/reload and18 original
hand views](../testing/part02-authoring/README.md). Source1.2 scale and wrist-frame
conversion are explicit; the seated basis is not applied twice. Authoring IK now fits the calibrated socket, correcting the measured10.467mm legacy offset gap without stretching limbs. Exact runtime
tessellation/hard-normal parity, angular thumb/finger contours, clearance and
human approval remain open. This is a development pass with unmet gates, not
Animation A acceptance or permission to begin Part03.

Part07 append-only update (2026-11-08): the report now contains15 clips, including
club_left/right. All original36 keys/13 clips retain deep equality; this is first-
club runtime integration, not full catalog/art approval. See ../testing/part07-weapons/.
