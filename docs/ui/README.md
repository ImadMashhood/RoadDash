# RoadDash UI reference and artist handoff

This is the starting kit for creating new RoadDash interface artwork. It catalogs
the user-supplied original game and provides an editable, independent skeleton.
**A first native entry flow now uses the approved gritty art.** See [NATIVE_FRONTEND.md](NATIVE_FRONTEND.md). The artist desk is separate; gameplay physics are unchanged. Broader UI/art acceptance remains open under UI-02/P15-006/P15-007.

## Open it

- Complete local kit with original reference pictures:
  `/home/imad/Documents/ChatGPT/RoadDash-UI-References/index.html`.
  It opens directly in a browser; no server/build/install is required.
- Repository-safe skeleton: [index.html](index.html). It contains metadata and
  original wireframes, with placeholders when retail references are unavailable.
- While the local preview server is running:
  [UI art desk](http://127.0.0.1:8875/index.html).

Start with **UI-IMG-0068**, the main-menu reference. Create your new RoadDash art
and export it as `replacements/UI-IMG-0068.png` in the local kit. Use the image
catalog to preview your file in the skeleton. Your files remain on disk; the
browser preview is session-only. Export/import progress saves **notes/statuses,
not the uploaded image binaries**. No original reference is automatically copied
into a replacement slot or into game assets. `blank-progress.json` resets draft
metadata if explicitly imported.

To change page names, actions or art-slot matching, edit `plan.json` beside the
kit and run `python3 refresh_plan.py`, then reload `index.html`. This needs only
Python’s standard library, not the original installation. Edit `style.css` for
zone placement/visual treatment. The default geometry is a proposal, not a
locked production layout.

## Generated screen artwork

All 28 proposed pages now have selected ChatGPT built-in imagegen artwork.
Open **Generated screens** to browse/download the PNGs and record review notes.
Each draft loads automatically behind the matching page's live HTML labels and
buttons. **Show layout guides** reveals the old zones; **Use generated screen**
switches back to the wireframe. Source-resource uploads remain separate.

Images are saved in `generated/screens/`, exact prompts in `generated/prompts/`,
and dimensions/alpha/hashes in `generated/manifest.json`. Actual output resolution
is 1672×941 (main:1671×941), not the requested 3840×2160 master target. Pixels are
preserved directly from the tool without stretching or synthetic upscaling.
HUD/pause/confirmation have real alpha; their preview uses a project native city
screenshot underneath, without modifying the game. Only the active page loads a
full-size background; the gallery uses lazy image elements.

The [28-item screen review](SCREEN_ART_REVIEW.md) remains unchecked. Generated
screen compositions do not complete all142 source-resource variants or15 component
families, and presence is not user approval. Native Bevy integration stays UI-02.
Provenance/limitations: [generated/PROVENANCE.md](generated/PROVENANCE.md).

## Generation prompts

[All 28 screen prompts](SCREEN_GENERATION_PROMPTS.md) provide specific composition,
lighting, subject direction, quiet widget areas and output instructions.
[All 142 individual image prompts](IMAGE_GENERATION_PROMPTS.md) cover every UI
replacement ID in 66 master families, with source aspect ratio and target filename.
Attach the relevant local reference and current model views; create one master
identity before its variants. These are briefs, not generated/approved artwork.
Rebuild after catalog/plan changes with `python3 tools/build_ui_prompts.py` from
the repository root. Machine-readable coverage: `art_prompts.json`.

## What is included

| Inventory | Count / meaning |
| --- | --- |
| UI image resources | 142:134 UI files plus8 embedded application-icon resources |
| Distinct source hashes | 137 UI hashes; some resources repeat or differ only by size/units |
| Proposed art masters | 66 families before further art-direction reuse |
| Other IMAGES files | 12 horizon/cloud files, clearly marked world context |
| Original screen table | All36 slots:29 populated screen entries;0–6 are transitions/reserved |
| Original popup tables | 13;46 stored popup rows |
| Native Windows dialogs | 7 unique templates repeated in2EXEs:14 resources,208 stored controls |
| Editable geometry SVGs | 63:36screen slots +13popups +14Windows resource layouts |
| RoadDash draft pages | 28 with working preview navigation |
| New reusable component briefs | 15, with hover/focus/disabled/error/locked and HUD states |
| Optional videos | 59 clips inventoried with dimensions, duration, hashes and local thumbnails |
| Contact sheets | 26 numbered3×2 sheets for154 total image resources, including context |
| Fonts / localization | Four font-file roles and five language DLL inventories; no font binaries copied |

UI image groups:8 menu backgrounds,6 options illustrations,25 character/Mano
pictures,15 course postcards/insets,18 showroom images,15 bike-detail pictures,
29 result/level/finale illustrations,3 street-hub insets,15 dashboard/meter/matte
images and8 embedded icons. These counts describe the supplied PC1997 build,
not every platform/release or every possible dynamic UI state.

## Work files

- [IMAGE_LIST.csv](IMAGE_LIST.csv): every source ID/file, dimensions, replacement
  path, master family/variant, screen use, suggested master size and art notes.
- [ASSET_CHECKLIST.md](ASSET_CHECKLIST.md):142 image-resource boxes and15 component
  boxes. **All remain unchecked.** Placeholder/code completion is not art approval.
- [MASTER_ART_BRIEFS.md](MASTER_ART_BRIEFS.md):66 grouped jobs. Reuse scalable
  dashboard/icon masters instead of redrawing every low/high or mph/kmh bitmap.
- [SCREEN_MAP.md](SCREEN_MAP.md): original slots, popup rows and Windows dialogs,
  with clear separation from the reconstruction’s newer native/online pages.
- [plan.json](plan.json): editable RoadDash navigation, notes,28screen layouts,
  per-screen reference matching, preferred images and component states.
- [VIDEO_LIST.csv](VIDEO_LIST.csv):optional cinematic references. Static cards or
  new scenes may replace them; they are not59 additional required UI stills.
- [catalog.json](catalog.json), [source IDs](../../tools/ui/source_ids.json):
  sanitized source metadata, frozen identifiers and supported-input hashes.
- `wireframes/`:editable source-geometry SVGs, without original media/fonts.

The complete local kit additionally contains `reference-images/`,
`menu-previews/`, `contact-sheets/`, `video-thumbnails/`, full source captions and
an empty `replacements/` folder. Original postcard and street insets are composed
at their source offsets in illustrative menu previews; they are not stretched
into fake fullscreen backgrounds.

## Art contract

For full backgrounds,3840×2160 is a **proposed RoadDash master**. Original screens
were640×480; keep important subjects in the central4:3 region until the new layout
is approved. Other pieces retain their source aspect as a suggested4× master.
Keep a5% safe area, transparent cutouts where appropriate, editable source layers
and separate type/number/price/meter/focus layers. Backgrounds must not bake in
live values. Prefer vector/nine-slice widgets and runtime-generated bars/needles.

The skeleton intentionally uses gray art boxes. It is not final art direction,
a complete WYSIWYG compositor, or functioning career/shop/network UI. An uploaded
image previews in its art box; multilayer native composition is future work.
Input, audio, accessibility and results states are component briefs for that
later pass, not claims that these features are newly integrated into Bevy.

## Research boundaries

R01 `arkits/road-rash` commit7aecf6044f2dd773b925b325823f30d648d61c89 supplied
format/role research:frontend tables, RRI composition, HUD strides, PE strings,
font roles and original/reconstruction screen separation. Root code-reuse license
remains unverified; no C++ source was copied. The independent Python readers and
HTML/CSS/JS skeleton are project-original. Retail pictures, video frames and
captions remain outside this repository and runtime. Fonts/AVIs/EXEs/DLLs are
inventoried, not bundled into the kit. [Research ledger](../third_party/REPOSITORIES.md).

Native dialog dimensions are dialog units rather than pixels; font/DPI conversion
is not recreated. The parser follows Microsoft’s documented
[DLGTEMPLATE](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-dlgtemplate)
and [DLGITEMTEMPLATE](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-dlgitemtemplate)
formats. Extended templates are supported/tested too. No Microsoft implementation
source is incorporated. Dialog300 is the original Windows Sound Mix; other
resource purposes remain generic IDs where runtime labels are populated later.

JPEG decode is not original IJG/GDI quantization parity. BOB index0 is shown
transparent for inspection; actual opaque/keyed treatment depends on the screen.
Unused cloud strides are explicitly inferred and context-only. Stored controls
may be mutated by handlers; original font drawing, selection/disabled logic,
dialog labels, videos and a retail playthrough were not recreated or executed.
File coverage does not establish complete behavioral parity or permission to
ship original media.

## Reproduce / verify

From the repository, with Python/Pillow and existing FFmpeg/FFprobe:

```bash
python3 tools/catalog_ui_references.py --install /path/to/ROADRASH \
  --output /path/outside/repository/RoadDash-UI-References
python3 tools/build_ui_kit.py --install /path/to/ROADRASH \
  --kit /path/outside/repository/RoadDash-UI-References
python3 -m unittest discover -s tools -p test_ui_kit.py
```

The frozen IDs/fingerprints support this inspected build. Unknown inputs/files
must be reviewed and appended to the registry without renumbering existing art.
Retail output paths inside the repository are rejected. The builder changes only
sanitized metadata/wireframes in `docs/ui` and the specified external kit.

[Verification report](../testing/ui-kit/README.md) records parser/coverage tests,
fresh extraction comparison, browser navigation/upload/progress checks and game
regression commands. Architecture:ADR088. No new runtime dependency, gameplay
change or automatic push/merge.

## Art direction correction

The first polished screen batch was rejected. Main menu now previews a separate original grotesque painted direction sample; other first-batch images are opt-in archived comparisons. Read [UI_ART_DIRECTION.md](UI_ART_DIRECTION.md) before generating further art. Current prompt catalogs describe the rejected pass and are not approved production briefs. No game assets are replaced.

User-approved gritty style: [four scene variations](generated/candidates/variations.html), with individual PNGs, exact prompts and reference metadata. The earlier polished pass remains rejected.

Actual native menu captures: [native frontend gallery](../testing/native-frontend/index.html).
