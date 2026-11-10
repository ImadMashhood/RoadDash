# Sky-driven lighting — 2026-11-09

Branch `codex/next-lighting`, isolated from `ef3eecc0`. Research and implementation
respond to the user's request to find a better lighting system online.
[Primary-source research](RESEARCH.md), [proposed shared records](INTEGRATION.md).

The default now prepares diffuse irradiance and roughness-filtered GGX reflections
from the original visible sky's **linear radiance**, with correct cube handedness
and muted lower-hemisphere ground bounce. The old environment used a different
sky approximation, raw directional diffuse samples and ordinary box mipmaps.
The ambient floor is reduced to35% and environment fill increased3x so surfaces
respond more to sky/ground orientation. Direct sunlight, exposure, tone operator,
visible sky, haze, shadow cascades, MSAA and all gameplay remain unchanged.

Use `ROADDASH_LIGHTING=legacy cargo run --release --locked` for the exact previous
lighting path. Default (also `balanced`) uses the new path. This is a startup
comparison/fallback switch, not a saved settings-menu feature. No new dependency,
shader pass, light, per-frame filter, material, facade atlas or geometry change.
Global IBL cannot occlude unseen geometry or replace scene-aware GI/contact AO.
SSAO was investigated and deferred because it requires MSAA off and introduces
prepass/compute/temporal-AA tradeoffs; full P15-004 remains open.

## Native evidence

- [Legacy street](before-legacy/screenshots/metro-03.png) /
  [balanced street](after/screenshots/metro-03.png)
- [Legacy farm](before-legacy/screenshots/metro-00.png) /
  [balanced farm](after/screenshots/metro-00.png)
- [Balanced recovery](after-recovery/screenshots/06-remounting.png)

All three final native runs exit0: same-binary legacy survey, balanced survey,
balanced farm/city recovery smoke. Frozen executable copied outside the shared
target to `../.lighting-binary/road-dash`; receipts hash binary and content before/
after. Saves/settings are isolated. Runs were serial with no cargo/rustc/game/bake
process overlap. The earlier `before` run used the pre-change executable and is
retained; final metrics compare the same binary's explicit profiles.

Visually inspected farm00, shadowed street03, distant horizon/aerial12 and close
remount06. Streets retain shadow-face/window/curb detail and lane contrast; sunny
surfaces retain the warm key light. The fill is less uniformly blue, and the bike
and rider remain readable. The change is subtle in diffuse-dominated street views;
it is a corrected environment-light response, not a photorealism or human art
approval claim. No dedicated under-arch fixture was captured. Global sky IBL
cannot supply local architectural occlusion; that limitation remains explicit.

Survey census is identical:19,108buildings,3,075chunks,4,071,964render triangles,
2,671,666collision triangles,2,535static batches,15racers. Recovery verifies reverse
−1.898m/s, two0.0000m pickup errors, all15riders on retry and5audio playback sinks.
No camera, physical transform, rig, collision, control, save or recovery change.

## Cost and limits

The balanced survey measured17.934ms mean /21.725ms p95 versus same-binary legacy
18.309ms /23.243ms. These are bounded whole-app wall samples, not isolated GPU
measurements, full-course performance or a causal speedup. Both miss16.7ms.
Raw values and equal-census assertion: [comparison.json](comparison.json).

Display cube generation129.204ms plus381.831ms IBL preparation =511.035ms startup;
recovery repeated at503.611ms total. Legacy display116.886ms plus28.610ms old maps
=145.496ms. New work is once at startup, with64samples,16px diffuse and128px
specular faces,8specular mips. Environment pixels532,464bytes versus2,105,328;
unchanged display cube2,097,144bytes gives total2,629,608bytes per CPU/GPU pixel
payload side, excluding engine overhead. No claim that payload equals actual
process memory, or that smaller maps produce a measured performance gain.

Limits: low-resolution approximate static global reflections,64-sample quadrature,
authored average ground bounce, no dynamic bounced light/local GI, no independent
cloud motion, no HDR migration. Human visual acceptance, temporal/full-course,
under-arch, Windows/controller and target hardware coverage remain open. Existing
SKY-01 evidence extends; no new IDs/checkbox states/counts/placements change.

## Reproduction

Build with `CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo build
--release --locked`; copy the executable to a task-owned location, then set
`ROADDASH_BINARY` if different from the script's default. Run separately:

```
python3 docs/testing/next-lighting/run_native.py before-legacy
python3 docs/testing/next-lighting/run_native.py after
python3 docs/testing/next-lighting/run_native.py after-recovery
```

Source changes: `src/sky.rs`, `src/sky/lighting.rs`. Root owns shared README,
PROGRESS, decisions/history/research/ASSETS synchronization and the integrated full
workspace regression gate. No push or merge by this branch.

## Source checks and integration gates

`cargo fmt --all --check` passes. `cargo test --locked sky::` passes all7targeted
sky/filter tests (full transcript retained). `cargo build --release --locked`
passes; final native executable contains the shipping runtime changes (later
source edits add tests only). `git diff --check` passes.

Strict locked workspace/all-target Clippy was queued behind another worktree's
build lock, then explicitly canceled at the parent's request to free its serial
validation window; **not claimed passed**. Full workspace tests (including model
physics/recovery), mandatory debug build and Clippy are delegated to the parent's
integrated final checks. `python3 tools/audit.py` ran and rejects the expected
`original-cloud-daylight` source-hash mismatch. Root owns that shared ASSETS row
and will refresh it, register the new original filter, and rerun audit; no passing
audit is claimed for this isolated commit. Exact proposed shared changes are in
INTEGRATION.md. No pushes or merges performed.
