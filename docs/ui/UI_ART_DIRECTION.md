# UI art direction — original Road Rash screen study

2026-11-08. User rejected the first polished 3D screen batch. Those 28 images are archived drafts, disabled by default. This revision concerns frontend artwork only; the world art and physical rider assets are unchanged.

## Inspected private originals

| Local source ID | Screen | Observed visual language |
| --- | --- | --- |
| UI-IMG-0068 | MAIN.RRI | Tilted wet-city photographic composite, deep violet lighting and dark brick; distinct from caricature pages. |
| UI-IMG-0142 | SPLASH.RRI | Grotesque biker crowd, giant noses/chins, warped overhead table, oily sculptural painting, violet shadows and distressed red title. |
| UI-IMG-0146 | STREET.RRI | Bowed storefronts, fisheye street perspective, cobalt night and dirty neon. |
| UI-IMG-0071 | OLLEYS.RRI | Sneering huge-nosed salesman, enormous hands, tiny body, filthy checkerboard shop and sickly yellow-green light. |
| UI-IMG-0072 | PANZER.RRI | Seedy biker bar, exaggerated heads, smoky purple/green shadows, cluttered pool-table foreground. |
| UI-IMG-0089 | REST1.RRI | Gross toilet gag, bug-eyed caricature, extreme close-up and warped room. |

These are local reference observations, not claims that every original menu used one medium. Private reference pixels remain outside Git and runtime. Original screens do not grant reuse rights.

## Replacement direction

Abrasive 1990s biker satire: ugly funny adult characters, asymmetrical faces, crooked teeth, greasy skin, exaggerated hands, cramped compositions and warped perspectives. Use oily airbrushed/digitally painted shading, rough printed texture, indigo/purple shadows, sickly yellow-green lights and rusty reds. New characters and compositions; no original logos or copied artwork. Keep live labels, menu controls and exact metrics separate.

The initial Come Bye / project-model-led UI prompts and their 28 outputs are superseded as art direction. Their historical prompts stay unchanged for provenance. Do not use them to generate further screens. Gameplay world inspiration does not determine this frontend style.

## Corrected sample

[Main menu v1](generated/candidates/main-gritty-v1.png), [exact prompt](generated/candidates/main-gritty-v1.txt), [dimensions/hash](generated/candidates/manifest.json). Generated with built-in imagegen using privately inspected SPLASH, OLLEYS and STREET as style references. Original scene and characters; no source images bundled. Unmodified tool output, no upscale. Production rights review and user art approval remain open. This is a direction sample, not a recreation of MAIN.RRI composition or finished native UI.

Visual inspection: grotesque asymmetric faces, oily purple/yellow rendering, grimy room, distorted foreground hand and quiet right menu area are present. The sample is more detailed than the low-resolution originals; matching their exact coarseness remains a review question. No new whole-batch approval is implied.

## User-approved style and four scene variations

The user approved the main-menu sample's visual direction and requested further variations inspired by the originals. [Four-scene gallery](generated/candidates/variations.html): biker bar (PANZER), shady dealer (OLLEYS), rival gang (SPLASH), crooked night street (STREET). These preserve the approved oily grotesque rendering with new compositions and characters. Exact prompts, reference IDs, dimensions and hashes: generated/candidates/variations.json and variation-jobs.json. Built-in imagegen; original PNGs copied unchanged. No runtime installation. Individual scene approval, live-widget fit, layered masters and UI-02/UI-05 remain open. No original source pixels bundled.

Inspection: the bar has pool-table foreground and expressive characters; dealer has giant foreshortened key hand and sportbikes; gang has five caricature racers and title space; street uses warped facades and supporting characters. Outputs retain the approved style. Minor incidental marks resemble lettering on small packages/keys despite the no-text brief; these are draft details, not UI labels or confirmed original branding. Bar and dealer are dense compositions and still require UI-placement review.
