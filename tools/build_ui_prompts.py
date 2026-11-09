#!/usr/bin/env python3
"""Build original-art prompts for every proposed UI screen and cataloged UI image."""
import json
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UI = ROOT / 'docs/ui'
STYLE = ('Create original artwork for RoadDash, a modern motorcycle combat racer. '
         'Use grounded human proportions, crisp moderately faceted 3D forms, painterly restrained surface detail, '
         'soft daylight and readable silhouettes: the approachable stylized countryside direction in the supplied '
         'Come Bye: Sheepdog Simulator screenshots, carried into a gritty American farm-to-city motorcycle world. '
         'Environment palette: rich burgundy accents, worn charcoal, warm ivory, asphalt gray, muted brick and natural greens; preserve each chosen rider/bike color scheme rather than recoloring every racer burgundy; '
         'subtle leather wrinkles, metal wear and believable materials. Keep one coherent visual language across the series. '
         'Use the supplied current RoadDash rider/bike model views for outfit, helmet, proportions and bike continuity. '
         'Use original-game references only to understand composition and function; create a new scene, designs and identity.')
EXCLUDE = ('No baked-in lettering, numbers, logos, watermarks, fake menu text, button captions, stat bars, '
           'copyrighted game title, proprietary character artwork, neon cyberpunk, glossy mobile-game styling, '
           'photorealism or excessive grain. No extra limbs, floating armor, bent wrists or inward-buckled knees. '
           'UI text, icons requiring exact semantics, meters, focus states and hit boxes will be added separately.')
# Specific art direction, readability and export type for each draft page. These are proposals, not retail states.
SCREENS = {
'splash': ('A confident helmeted rider and motorcycle at a rural overlook, road leading from farms toward a dense distant American skyline; three-quarter rear view, early morning light, purposeful rather than comic. Frame the rider in the lower-left third and leave broad calm sky in the upper-middle for the future RoadDash title.', 'Keep the right third low contrast and the bottom 13% calm for the continue action.'),
'main': ('A motorcycle outside a lively roadside repair garage at the farm-to-city boundary. Helmeted rider rests one gloved hand on the tank; burgundy suit inserts and thin ivory lower piping. Camera at chest height, bike and rider occupy the left 55%; open garage depth, tools and a few distant racers imply a living world.', 'Reserve the right 35% as a quiet shaded wall for four to six menu actions; upper 10% and lower 13% remain uncluttered.'),
'courses': ('An original tabletop route-planning scene: five separate blank postcard wells across the left and center, with evocative landscape fragments for dense city, vineyard farmland, coast, peninsula and forested mountain roads. Physical paper, understated map lines and garage lighting; no copied course-map topology.', 'Keep postcard centers clean for separately generated location pictures; reserve the right 32% for route description and difficulty, and bottom 13% for navigation.'),
'riders': ('A roadside meet-up portrait backdrop outside a brick workshop: neutral slate wall, subtle hanging jackets, warm side light. One full-body helmeted rider stands naturally in the left half, shoulders relaxed, hands straight at sides; a second empty portrait space suggests selectable rivals without painting a roster.', 'Keep the right 32% plain for name, stats and description; export the character on a separate transparent layer as well as the background.'),
'hub': ('An eye-level American neighborhood street that can serve as a career hub. Put a rider clubhouse at left, a motorcycle repair shop near the center and a modest home entrance farther right. The street connects naturally, sidewalks meet doorsteps and storefronts share a consistent perspective; warm afternoon, parked bikes and quiet neighborhood detail.', 'Keep doorways unobstructed for later interactive markers. Reserve upper 10% for status and lower 13% for actions. No painted sign lettering or invented active quest markers.'),
'hangout': ('Interior of a friendly but rough motorcycle clubhouse, pool table, leather seats, helmet shelf and warm window light. Two distinct riders converse in the left half with natural hands and believable posture, leaving a clear foreground seat. Detailed setting, restrained clutter and a readable focal group.', 'Keep right 32% darker and calm for available conversations; do not paint dialogue balloons or text.'),
'dialogue': ('A waist-up biker conversation portrait in a roadside diner booth. The speaker sits on the left, torso turned slightly toward the viewer, attentive expression or closed visor, one naturally resting hand visible; workshop and street softly simplified behind. Reuse the chosen rider identity across later relationship variants.', 'Right 32% and bottom quarter must be flat and quiet for live dialogue and choices. Export a separate transparent portrait plus the text-free diner background.'),
'garage': ('A practical motorcycle showroom/repair garage, three empty presentation bays across the left 60%, concrete floor, tool cabinets, subtle overhead lights and restrained daylight. Make the bays suitable for rat/custom, sport and superbike silhouettes, using consistent perspective and lighting.', 'No motorcycles fused into the floor or stat labels; render bike cutouts separately. Right 32% and bottom 13% are reserved for live class selection, credits and actions.'),
'bike': ('A single original motorcycle on a low garage inspection stand in the left half, three-quarter front view at tank height, entire tires, mirrors and exhaust visible. Mechanically coherent frame and suspension, matte painted panels with restrained wear. Use the selected bike reference for silhouette and exact color continuity, without copying branding.', 'Leave right 32% plain for price and performance; bottom 13% clear. Export the same bike as an isolated transparent cutout for reuse in the garage.'),
'settings': ('An original workshop notice-board background: dark painted timber, brushed steel edge, discreet pinned blank paper and a softly lit tool silhouette. A small helmet and glove sit low at left; the surface has enough character to belong to a biker game but remains quiet.', 'Keep left/center and right 35% uniformly readable for settings categories and live controls. No baked-in sliders, labels, checkmarks or icons.'),
'controls': ('A close, text-free garage workbench background, warm dark slate rubber mat, gloves and a compact motorcycle handlebar assembly confined to the far left edge. A neutral clear central panel suggests a technical manual without actually drawing a diagram.', 'Leave left-center 55% for live binding rows and right 32% for a separately drawn controller/keyboard diagram. Do not invent key labels, wiring arrows or controller glyphs.'),
'display': ('The same quiet workshop materials as the settings hub, with a small off-center framed landscape swatch showing farm green, brick red, sky blue and asphalt gray. Balanced soft light, no glowing screens or distracting reflections.', 'Leave 55% left-center and right 32% calm for resolution, quality and comfort controls. Color and contrast calibration graphics will be vector UI, not painted into this background.'),
'audio': ('The same workshop settings environment, a compact original radio and a pair of worn headphones occupy only the lower-left corner. Soft materials and modest warm light, a restrained brushed-metal and charcoal palette.', 'Keep most of the canvas empty for live volume sliders and toggles; do not paint slider tracks, equalizers, numbers or music titles.'),
'jukebox': ('A small roadside diner jukebox in the left third, original design with brushed steel, burgundy casing and warm ivory glass, paired with a motorcycle helmet on a stool. Evening diner light remains soft, grounded and slightly nostalgic.', 'Keep right half and bottom 13% quiet for live track list and playback controls; no album covers, song names or illegible fake lettering.'),
'saves': ('An original garage locker and route journal, three blank file-card pockets on a subdued wall at left, one small keyring and helmet shelf for personality. Practical storage aesthetic, soft side light, restrained depth.', 'Empty pocket interiors; reserve right 32% for selected-save details. Dates, rider names, thumbnails, file counts and warnings are live UI and must not be generated.'),
'loading': ('A helmeted racer waits beside a motorcycle at the start of a winding two-lane rural road that widens toward the city. Fields, fence boundaries and dense distant skyline make a coherent farm-to-city route. Low three-quarter camera, rider and bike on left, a road vanishing point just right of center.', 'Keep right 32% calm for course information and bottom 13% for a separate progress bar/tip. No fake loading text, percentages or painted road map.'),
'hud': ('Create an original minimal racing HUD frame as a transparent overlay only. Small charcoal translucent corner plates with warm ivory edge accents: upper-left compact standings plate, upper-right small route-map frame, lower-right speed/gear instrument housing, and a thin lower-left condition area. Slight tactile garage-metal treatment; clean, restrained and readable.', 'The central 60% of the image is fully transparent. No scenery, bike, rider, opaque full-width dashboard, digits, needles, route lines or filled meters. Export RGBA PNG; exact live metrics and racer-shirt dots are drawn by the engine.'),
'pause': ('A reusable translucent pause-panel art layer with understated charcoal workshop-metal edges, a thin burgundy accent and gentle warm ivory rim light. Rectangular panel on the right third, one small original helmet motif at lower left.', 'Transparent outside the panel; central gameplay remains visible. No screenshot of gameplay, blur baked into artwork, menu words or button states. Export RGBA PNG.'),
'results': ('An original finish-area background: motorcycle parked to the left, barriers and dense city finish street receding softly behind, late afternoon light. A subdued blank standings board occupies the right half and has clear horizontal space without painted lines or lettering.', 'Leave board interior and upper title zone clean for live standings, credits and time; bottom 13% for actions. Do not prepopulate winners, scores or payouts.'),
'won': ('A restrained victory moment after an urban motorcycle race: helmeted rider on the left lifts one gloved fist, motorcycle safely parked beside them, a few cheering riders beyond a finish barrier. Athletic grounded stance, natural anatomy, warm late-afternoon rim light, rich burgundy and charcoal outfit.', 'Keep right 32% quiet for qualification and payout. No trophy text, huge confetti wall, finish banner lettering or fake medal numbers.'),
'lost': ('A disappointed but composed rider stands beside their intact motorcycle at the edge of the city finish area. Head slightly lowered, one hand on the tank, exhausted rather than slapstick. Overcast soft light, coherent street geography and a subdued cool palette with burgundy focal accents.', 'Subject left 55%, right 32% quiet for outcome and retry actions. No injury, exaggerated broken equipment or painted race placement.'),
'wrecked': ('A helmeted rider sits safely on the roadside curb after a crash, their original motorcycle visibly scuffed and tipped nearby; small bent mirror, displaced indicator and tire marks imply the event. Stylized grounded anatomy, a coherent city sidewalk, no gore and no explosive spectacle.', 'Frame people and bike at left, leave right 32% low contrast for repair/outcome text. Keep the bottom action strip clear.'),
'busted': ('An original roadside police stop in the American city: helmeted racer standing beside their motorcycle on the left; a clearly distinct motorcycle officer waits nearby, calm controlled posture. Police bike with restrained red/blue accents, daylight, no violence, no handcuff close-up.', 'Keep right 32% quiet for penalty and retry UI. No real department badges, copied police branding, text, numerical fines or painted flashing UI.'),
'level': ('A confident rider and motorcycle pass from a quiet industrial neighborhood toward a taller, denser downtown skyline. Low but natural camera, new district visible ahead, warm morning light catching varied roof heights and brick details. The rider remains on the left, conveying progression through environment rather than an RPG level graphic.', 'Keep right 32% and top 10% calm for advancement/unlock text; no numbers, arrows, giant badges or fictional confirmed unlocks.'),
'penalty': ('A quiet garage after a difficult race, helmet and gloves on a workbench, parked motorcycle in shadow at left, warm desk lamp, an empty clipboard and a closed cash tin as subtle setback storytelling. Serious, grounded, no comic humiliation.', 'Reserve right 32% for live explanation and recovery/retry options. No bill amounts, game-over lettering, bankruptcy claim or irreversible penalty implied by the image.'),
'credits': ('A panoramic evening roadside gathering outside an independent motorcycle garage, several bikes and helmeted riders confined to the outer thirds. Warm garage light, soft blue sky and natural material detail; celebratory but calm, consistent with the rest of the RoadDash artwork.', 'The central 45% is a dark, low-detail column for live scrolling credits, and the bottom edge is quiet. Do not draw names, logos, credit text or license notices.'),
'network': ('A future-use motorcycle meet-up staging area in a garage courtyard. Four empty motorcycle parking bays on the left, subtle original floor markings and soft daylight, restrained charcoal, brick and burgundy details. No people or bikes in the slots, so future portraits can be layered independently.', 'Keep right 32% empty for a future lobby. No connection-state artwork, real service logos, player handles or implication that multiplayer is already implemented.'),
'confirm': ('Create a reusable original confirmation-dialog frame only: a centered restrained charcoal rectangular plate, small warm ivory rim, understated burgundy warning accent, subtle matte metal grain. Clear silhouette at small size, generous quiet interior and soft separated shadow.', 'Transparent surroundings. Panel occupies center 50% width and 40% height with a clean lower strip for two live buttons. No warning text, symbols requiring exact semantics, yes/no captions or baked dimmed background. Export RGBA PNG.'),
}
COURSES = {'city':'dense American brick city with a diagonal racing avenue and varied skyline',
           'napa':'vineyard farmland, rolling planted rows, timber fences and a narrow two-lane road',
           'pch':'Pacific coastal road above cliffs, restrained ocean blue and dry coastal vegetation',
           'penin':'a peninsula road between hillside neighborhoods and water',
           'sierra':'forested mountain road, granite slopes and distant peaks',
           'sierrar':'a second forested mountain-road composition in the same visual family',
           'klub':'a small motorcycle clubhouse exterior', 'resta':'a roadside diner exterior',
           'restb':'an alternate roadside diner exterior'}


def family_subject(name):
    if name.startswith('bike-'):
        _, category, model = name.split('-', 2)
        shapes = {'rat':'an original stripped custom street motorcycle with exposed mechanical parts',
                  'sport':'an original road-going sport motorcycle with compact fairing and realistic riding geometry',
                  'super':'an original high-performance superbike with aerodynamic fairing and a purposeful low stance'}
        if model == 'bikes':
            return f'Create a coherent class-overview composition of three distinct {category} motorcycles, all fully visible and on one consistent ground plane. Use the selected source only for its overview function, not as an individual named bike.'
        return f'Create {shapes[category]}, catalog subject {model}. Match the supplied reference silhouette, proportions and color blocking in the approved RoadDash model language. Entire wheels and mirrors visible; three-quarter view, neutral studio-garage light, transparent cutout, no rider. Shop and detail images must depict the same bike, not different designs.'
    if name.startswith('dashboard-'):
        return f'Create a compact original {name.removeprefix("dashboard-")} class instrument-frame overlay, matching the selected catalog silhouette. Empty readable gauge wells, charcoal material and warm ivory thin trim, transparent surroundings. Keep needles, values, bars, unit labels and highlights separate. Kilometer/mile and high/low resolution outputs derive from one scalable master, not newly invented dashboard layouts.'
    if name.startswith('rider-'):
        return f'Create a waist-up character portrait master for catalog identity {name[6:]}, using the chosen local reference to establish consistent appearance and outfit palette, translated into the current original RoadDash character design. Natural shoulders, relaxed hands if visible, coherent faceted anatomy, neutral studio/diner lighting. Keep face, hair or helmet and suit fixed between variants. F/N/H are source relationship variants; do not guess their emotions from those letters. Interpret each supplied picture visually before making its separately labeled variant.'
    if name.startswith('courses-'):
        code = name[8:]; qualified = code.startswith('q'); code = code[1:] if qualified else code
        scene = COURSES.get(code, 'the supplied course setting, preserving its role without copying its original picture')
        return f'Create a text-free course postcard depicting {scene}, in the same RoadDash environmental art direction. Clear road and plausible connected geography, one restrained focal landmark, aerial-oblique or landscape framing matching the selected inset. {"This is a qualified-state companion: reuse the base course scene and keep the qualification badge separate, not a newly redesigned location." if qualified else "Keep matching qualified-state art based on this same composition."} No labels, maps with false precision or baked frame.'
    if name.startswith('result-'):
        key = {'win':'won','lose':'lost','wreck':'wrecked','bust':'busted','finale':'won','level':'level'}[name[7:]]
        return SCREENS[key][0] + ' Generate each numbered source variant as a separate composition within this same outcome, preserving the rider/bike identity and lighting language; the suffix is an art variant, not a proven unique game mechanic.'
    if name.startswith('hub-insets-'):
        place = {'home':'a modest home with a clear entrance','klub':'an independent motorcycle clubhouse','olley':'a neighborhood motorcycle repair shop'}[name[11:]]
        return f'Create {place} as a modular destination insert for the same career-hub street. Match the hub camera, horizon, daylight and facade scale; intact doorway and sidewalk connection. No sign lettering or hard baked panel border. Export matching background plate and a separate masked insert.'
    return {
        'menu-backgrounds-main': SCREENS['main'][0],
        'menu-backgrounds-splash': SCREENS['splash'][0],
        'menu-backgrounds-credits': SCREENS['credits'][0],
        'menu-backgrounds-street': SCREENS['hub'][0],
        'menu-backgrounds-olleys': SCREENS['garage'][0],
        'menu-backgrounds-panzer': 'Create an original motorcycle-dealer backdrop with a rugged industrial garage, coherent shop-floor perspective, empty bike-display area and quiet data space. Use the supplied image to establish function and framing; do not infer a tank or military scene from its filename.',
        'menu-backgrounds-postcard': 'Create the reusable text-free course-postcard surround: understated worn paper and a charcoal workshop-board edge. Preserve a clean inset opening and quiet description area; no location scene baked into the reusable surround.',
        'menu-backgrounds-matte': 'Create a plain low-detail charcoal workshop backdrop with subtle warm light and controlled material grain, suitable behind live menu text. No subjects, gradients that harm contrast, buttons or labels.',
        'options-background': SCREENS['settings'][0] + ' Build all six source variants from one coherent room/material design; inspect each reference for composition rather than assuming six unrelated settings pages.',
        'hud-condition-stamina': 'Create a clean original condition/stamina meter housing only, subtle charcoal and ivory edges, transparent empty fill area. Export reusable frame and mask. Live fill, warning thresholds, text and damage values remain separate.',
        'hud-matte': 'Create a transparent gameplay matte/frame with restrained corner framing only. Keep the road and central rider view completely open. Do not generate scenery or an opaque colored backdrop; exact cutout geometry must be checked against the supplied source and the new HUD.',
        'characters-manomano': 'Create an original matchup/character-comparison vignette in the established RoadDash rider style. Two separate rival portrait spaces, consistent light and natural anatomy; quiet areas for live comparison data. Inspect the supplied reference for its actual grouping; do not invent a playable fighting mode.',
        'application-brand-icon': 'Create an original compact RoadDash emblem: a strong abstract road bend combined with a motorcycle-wheel silhouette, one burgundy accent against charcoal and warm ivory. No letters or copied game logos. Center the simple silhouette within 80% of a square canvas, transparent outside, legible at 16 pixels; deliver a large master for later hand-refined icon sizes.',
    }[name]


def build():
    plan=json.loads((UI/'plan.json').read_text()); catalog=json.loads((UI/'catalog.json').read_text())
    assert set(SCREENS)=={s['id'] for s in plan['screens']}, 'every draft page needs specific direction'
    screens=[]
    for s in plan['screens']:
        scene, space=SCREENS[s['id']]
        alpha=s['id'] in {'hud','pause','confirm'}
        output=('3840×2160 RGBA PNG with true transparency; no scenery behind the overlay.' if alpha
                else '3840×2160, 16:9 text-free background plate; layered source if available. Separate portraits/bikes/inserts from the background.')
        prompt=f'{STYLE}\n\nSCREEN: {s["title"]}. {scene}\n\nCOMPOSITION: {space} Keep essential subjects inside the central 4:3 crop and preserve a 5% safe margin. This is a starting composition for the editable skeleton, not a screenshot of finished functioning UI.\n\nOUTPUT: {output}\n\nAVOID: {EXCLUDE}'
        screens.append({'id':s['id'],'title':s['title'],'prompt':prompt,'output':output})
    groups=defaultdict(list)
    for i in catalog['images']:
        if i['scope']=='ui': groups[i['master_family']].append(i)
    families=[]; images=[]
    for name, refs in sorted(groups.items()):
        subject=family_subject(name)
        families.append({'id':name,'images':[i['id'] for i in refs],'prompt':f'{STYLE}\n\nSUBJECT: {subject}\n\nAVOID: {EXCLUDE}'})
        for i in refs:
            # Keep exact source aspect here: several catalog entries are narrow insets, not fullscreen pictures.
            w,h=i['width'],i['height']
            # Per-source pieces preserve aspect; the separate full-page briefs
            # deliberately target 16:9. Never combine a 4:3 requirement with a 16:9 size.
            scale = max(4, (256 + min(w, h) - 1) // min(w, h))
            master=[w * scale, h * scale]
            prompt=f'{STYLE}\n\nSUBJECT: {subject}\n\nVARIANT: {i["variant"]}. Reference ID {i["id"]}, source {i["source"]}; inspect that specific local picture for framing and color continuity. The source size is {w}×{h}; keep its exact {w}:{h} aspect for this asset, independently of the proposed 16:9 page. Suggested master {master[0]}×{master[1]}; resize/fit manually if the generator cannot produce that ratio. No stretching or unapproved cropping. Preview/export as {i["replacement"]}; this is an artist handoff, not automatic runtime installation.\n\nAVOID: {EXCLUDE}'
            images.append({'id':i['id'],'family':name,'replacement':i['replacement'],'prompt':prompt})
    payload={'schema_version':1,'stage':'prompt briefs only; no generated or approved artwork','screens':screens,'families':families,'images':images}
    (UI/'art_prompts.json').write_text(json.dumps(payload,indent=2)+'\n')
    lines=['# RoadDash — screen generation prompts', '', 'Copy one complete prompt below. Attach the relevant local reference and approved current rider/bike views. These prompts propose original replacement art, not finished assets. Do not generate typography or live metrics as part of the picture.', '', 'The 28 pages are the proposed skeleton, not 28 proven original retail menus. Future career/network concepts remain previews. Master backgrounds target 3840×2160, then the skeleton previews at 1920×1080. Image generation may deliver another resolution; inspect and resize deliberately. Empty areas are intentional space for native UI.', '', 'For every individual source replacement, use [IMAGE_GENERATION_PROMPTS.md](IMAGE_GENERATION_PROMPTS.md): all 142 UI images are covered with stable filenames and source aspect ratios. Twelve world-context images are outside this UI scope. [Machine-readable prompts](art_prompts.json).', '']
    for n,s in enumerate(screens,1): lines += [f'## {n:02d} — {s["title"]} (`{s["id"]}`)','', '```text',s['prompt'],'```','']
    lines+=['## Approval before integration','','Check the image against the selected reference and current model: coherent anatomy, correct palette, no baked text, quiet UI zones, alpha where required, legible small-size silhouette, and no reused proprietary pixels. Save the master plus separate layers/masks; preview the replacement and record notes/status. Prompt coverage does not close any art checklist or UI-02 integration gate.','']
    lines[1:1]=['', '> **Superseded art direction:** the user rejected this polished 3D pass on 2026-11-08. Historical prompts only. Use [UI_ART_DIRECTION.md](UI_ART_DIRECTION.md) and its separate corrected candidate; no art approval is implied.', '']
    (UI/'SCREEN_GENERATION_PROMPTS.md').write_text('\n'.join(lines))
    lines=['# RoadDash — per-image generation prompts','','All 142 UI resource IDs, grouped into 66 proposed master families. Create a master first, then reuse it for compatible variants. Do not regenerate the same identity independently for each size/state. Source dimensions and family mappings come from the frozen inventory, not invented file counts. The UI art checklist remains unchecked. Full-page compositions: [screen prompts](SCREEN_GENERATION_PROMPTS.md).','']
    for family in families:
        lines += [f'## {family["id"]}', '', 'Master references: '+', '.join(family['images'])+'.','']
        for i in images:
            if i['family']==family['id']:
                lines += [f'### {i["id"]} → `{i["replacement"]}`','','```text',i['prompt'],'```','']
    lines[1:1]=['', '> **Superseded art direction:** the user rejected this polished 3D pass on 2026-11-08. Historical prompts only. Use [UI_ART_DIRECTION.md](UI_ART_DIRECTION.md) and its separate corrected candidate; no art approval is implied.', '']
    (UI/'IMAGE_GENERATION_PROMPTS.md').write_text('\n'.join(lines))
    return payload

if __name__=='__main__':
    p=build(); print(f'Built {len(p["screens"])} screen, {len(p["families"])} family and {len(p["images"])} per-image prompts; no artwork generated.')
