# UI reference kit — implementation and verification

Date: 2026-11-08. Branch: `codex/ui-reference-kit`, based on `1a96667`.

## Implementation and checklist

UI-01 is verified: an independent source catalog, private reference gallery and editable artist skeleton are available. UI-02 remains open for artist-created replacements, approval and native Bevy integration. No game UI, physics, gameplay, saves, dependencies or runtime assets changed. All 142 source-image and 15 component art checkboxes remain unchecked.

The supplied installation yielded 146 IMAGES files plus eight embedded icons: 154 image records, of which 142 are UI and 12 world context. The catalog groups 66 proposed UI master families (137 unique UI source hashes). It records 36 frontend slots, 29 populated entries, 13 popup tables, 175 stored menu items, 14 native Windows dialog resources representing seven unique layouts, four font roles and 59 video clips. Video metadata/thumbnails are included; video/font/executable binaries are not copied into the kit. Twenty-six 3×2 contact sheets cover every image once.

The original source geometry is represented by 63 independent SVG wireframes. The proposed RoadDash skeleton has 28 navigable draft pages, 15 component briefs, source matching, replacement-image previews, search and progress export/import. Uploads are session-only; progress files preserve notes/status but not uploaded image bytes. Edit `plan.json` and run `python3 refresh_plan.py` to update the standalone plan without an original installation.

## Files and handoff

- [Artist guide](../../ui/README.md), [image list](../../ui/IMAGE_LIST.csv), [master briefs](../../ui/MASTER_ART_BRIEFS.md), [art checklist](../../ui/ASSET_CHECKLIST.md) and [screen map](../../ui/SCREEN_MAP.md).
- `docs/ui/`: browser app, metadata, editable plan, refresh tool, wireframes and blank progress.
- `tools/catalog_ui_references.py`: bounded PE/menu/resource readers and independent local extraction.
- `tools/build_ui_kit.py`: public metadata sanitization, private reference compositions and handoff generation.
- `tools/test_ui_kit.py` and `tools/ui/source_ids.json`: parser/coverage tests and frozen IDs/fingerprints.
- README, PROGRESS, task placement, decisions, issues, changelog and provenance ledgers updated together. No files removed.

The complete private kit is `/home/imad/Documents/ChatGPT/RoadDash-UI-References/index.html`; its sibling ZIP contains 345 files (68,561,306 bytes). The public repository contains no extracted original pictures/fonts. The external archive is a local research handoff, not a redistribution permission grant.

## Research and architecture

Studied arkits/road-rash at `7aecf6044f2dd773b925b325823f30d648d61c89`: frontend/menu layouts, career references, RRI/BOB layouts, HUD, native screens and resource strings. Its root reuse license remains unverified; classification is REFERENCE ONLY. No reference C++ implementation was incorporated. Primary Microsoft dialog-template documentation informed bounded standard/extended resource parsing; see the repository ledger and artist guide for links.

ADR088 separates private reference media from public original skeleton code and keeps final artwork/native integration gated. New IDs: UI-01 (completed, originating Part 15, tracked in completed Part 00) and UI-02 (open, Part 15). Stable artist IDs must not be renumbered by future extraction.

## Verification

| Check | Result / evidence |
| --- | --- |
| `python3 tools/test_ui_kit.py` | 17 passed; [receipt](python-tests.txt) |
| `cargo fmt --all --check` | Passed; [receipt](fmt.txt) |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | Passed; [receipt](clippy.txt) |
| `cargo test --workspace --locked` | 364 passed, one pre-existing ignored; [receipt](rust-tests.txt) |
| `cargo build --locked` | Passed; [receipt](build.txt) |
| `python3 tools/audit.py` | Passed, 106 runtime asset records unchanged; [receipt](audit.json) |
| `python3 tools/test_roadmap.py` | Five passed; [receipt](roadmap.txt) |
| `python3 tools/sync_progress.py --check` | Passed; [receipt](progress-check.txt) |
| `git diff --check` | Passed |
| Fresh source extraction | All 154 image metadata and PNG bytes reproduced; 14 dialog resources match; [receipt](reproduction-check.json), [extractor output](extraction-reproduction.txt) |

Browser verification used the actual standalone app at 1280-pixel viewport: all 28 draft pages, 49 source screen/popup views and 14 native dialog views rendered with expected headings. Main → courses → loading → HUD navigation worked. Replacement image upload/preview, actual downloaded progress export, valid import, invalid-schema rejection and blank reset were exercised. An export initially omitted an actively edited note; input-event tracking fixed it, and the downloaded [export sample](exported-progress.json) confirms the note and status. Browser console reported no errors; no horizontal overflow was observed at this viewport. See [navigation evidence](browser-navigation.json), [dialog evidence](browser-windows-dialogs.json), [main screenshot](skeleton.jpg), [HUD screenshot](hud.jpg) and [upload preview](art-preview.jpg). Repository screenshots show our own skeleton, not original game pictures.

## Limitations and next step

This is source/resource research, not an exhaustive retail playthrough. Dynamic labels, handler effects, font rasterization/DPI and exact native execution are not emulated. Cloud strides are explicitly inferred context-only; index-zero transparency is preview treatment, and JPEG decoding does not claim original quantization parity. Native Windows, controllers, mobile viewports and game runtime integration were not tested in this authoring task. No release/native-game smoke run is claimed. No game changes required one here.

Create and review original replacement art, starting with main-menu UI-IMG-0068, then approve component typography and interactive states before UI-02 Bevy integration. Career/network draft pages do not implement those game features. Art and production UI exit gates remain open. Commit is local on the branch above; no push or merge was requested for this handoff.
