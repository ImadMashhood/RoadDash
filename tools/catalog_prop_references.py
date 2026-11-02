#!/usr/bin/env python3
"""Inventory user-local retail world art; pixels stay outside the repository.
Independent format reader; existing external decoder is used only for decompression.
No retail bytes or third-party implementation are embedded here.
"""
import argparse
from collections import Counter
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import struct
from measure_reference_districts import resources, course_loads

ROOT = Path(__file__).resolve().parents[1]
COMMIT = 'aa9cb6d3181e20736af71022cd4d6aac0e2788d1'
MAGIC = struct.pack('<I', 0xbc614e)


def word(data, offset):
    if offset < 0 or offset + 4 > len(data):
        raise ValueError('word outside payload')
    return struct.unpack_from('<I', data, offset)[0]


def texture(data, offset):
    if word(data, offset) != 0xbc614e or offset + 60 > len(data):
        raise ValueError('not an indexed CEL')
    width, height, levels = struct.unpack_from('<III', data, offset + 32)
    if not (0 < width <= 256 and width & (width - 1) == 0 and 0 < height <= 512 and 0 < levels <= 9):
        raise ValueError('invalid CEL dimensions')
    name = data[offset + 16:offset + 32].split(b'\0')[0].decode('ascii')
    if not name or not all(32 <= ord(c) < 127 for c in name):
        raise ValueError('invalid CEL name')
    begin = offset + 28 + word(data, offset + 56)
    if begin < offset + 60 or begin + width * height > len(data):
        raise ValueError('CEL pixels outside payload')
    pixels = data[begin:begin + width * height]
    digest = hashlib.sha256(struct.pack('<II', width, height) + pixels).hexdigest()
    return dict(name=name, offset=offset, width=width, height=height,
                pixel_sha256=digest, flags=word(data, offset + 12)), pixels


def cells(data):
    result = {}; rejected = []; start = 0
    while (offset := data.find(MAGIC, start)) >= 0:
        start = offset + 4
        try:
            result[offset] = texture(data, offset)[0]
        except (ValueError, UnicodeError, struct.error) as error:
            rejected.append(dict(offset=offset, reason=str(error)))
    return result, rejected


def leaves(data):
    """Walk category/selector branches, keeping aliases and refusing cycles."""
    result = []
    def visit(offset, path, ancestors):
        if offset in ancestors or len(path) > 32:
            raise ValueError('family hierarchy cycle/depth')
        tag = word(data, offset)
        if tag < 4096:
            if offset + 4 + tag * 4 > len(data):
                raise ValueError('truncated family branch')
            for index in range(tag):
                visit(offset + word(data, offset + 4 + index * 4), path + [index], ancestors | {offset})
        elif tag == 0x4f465354:
            visit(offset + word(data, offset + 4), path, ancestors | {offset})
        else:
            result.append((path, offset, tag))
    visit(0, [], set())
    return result


def references(data, offset, tag):
    """All stored graphics frames/size cells, including repeated temporal slots."""
    if tag == 0x5252414e:
        info = offset + word(data, offset + 8)
        count = word(data, info + 16)
        if count > (len(data) - offset - 28) // 28:
            raise ValueError('truncated RRAN frames')
        return [dict(frame=i, cel_offset=offset + word(data, offset + 28 + i * 28 + 4) + 8)
                for i in range(count) if word(data, offset + 28 + i * 28 + 4)]
    if tag == 0x434c4750:
        mode = data[offset + 20] & 3
        table = offset + data[offset + 21] * 4
        cols, rows = data[offset + 24] + 1, data[offset + 25] + 1
        if mode == 0:
            return [dict(row=y, column=x, cel_offset=offset + 8 + word(data, offset + word(data, table + 4 * y) + 4 * x))
                    for y in range(rows) for x in range(cols)]
        if mode == 1:
            return [dict(column=x, cel_offset=offset + 8 + word(data, table + 4 * x)) for x in range(cols)]
        raise ValueError('unsupported CLGP mode')
    return []


PATTERNS = [
    ('trees', r'^(TRE|TR5G|TRJ3|TREE|PALM|PINE|OAK|CEDAR|REDWD|REDW|BIRCH|WILLOW|SPRUCE|EUC|FIR)'),
    ('shrubs_plants', r'^(VINE|BUSH|BSH|SHRUB|GRASS|WEED|FLOW|CACT|FERN|HEDGE)'),
    ('street_lighting_utilities', r'^(LMPPST|LIGAHD|METER|PRKGLMP|STRTLTE|TELPLE|LAMP|LITE|LIGHT|POLE|PWR|POWER|TELE|HYDRA|HYDR|STL|ELEC|WIRE|TLITE)'),
    ('signs_billboards', r'^(CALTRAN|CURVES|DIAGBCK|DONTENT|FLGSAHD|FLSHFLD|FWENTR|HNDCPBK|HWYJN|HWYJUNC|INTRSGN|L-TURN|LANE|MARKER|NOPRKNG|NOUTURN|RTRYCLB|SLIPPRY|SLOWARW|SLWARBK|XWALK|YIELD|SIGN|SGN|BLLBRD|BILL|STOP|SPEED|2HR|2HOUR|ROADSG|PARK|CHEVR|ONEWAY)'),
    ('fences_barriers', r'^(FNSHCON|POST|PST|SLATS|FNC|FENCE|BIGFNC|GUARD|RAIL|BRRIC|BARR|GATE|WALL|CONE)'),
    ('rocks_landforms', r'^(ROCK|BOULD|CLIFF|STONE|STUMP|LOG)'),
    ('street_furniture', r'^(CLBX|CLLBX|CNTAIN|CONTN|MLBX|MUNI|NEWS|NWSRCK|ORNGBOX|PARCEL|PHNBTH|PHONBTH|PICNTBL|RECYC|SDWLKBX|TRSH|VEGSTND|WGNBRRL|WINDVN|BENCH|BNCH|BIN|TRASH|GARB|BOX|BAGS|PHONE|MAIL|DUMP|BUSSTOP|PORTLET)'),
    ('loose_materials_hazards', r'^(GRAVPL|GRVP|HAY|PILE|OILP|SNDP|WATP|WTRP)'),
    ('animals', r'^(COW|BULL|DOG|DEER|HORSE|SHEEP|BIRD|PIG|CAT|ANML)'),
    ('parked_vehicles_machinery', r'^(COPCAR|TRCK|TRK|VWBUS|CAR|TRUCK|VAN|BUS|TRACT|TRAIL|WHEEL|WRECK|TIRE|TYRE|BOAT)'),
]


def classify(category, names, architectural=False, traffic=False):
    if traffic:
        return 'traffic_vehicles'
    if 'DEER2' in names:
        return 'signs_billboards'  # Visually verified deer-crossing sign, not a deer.
    if category == 2:
        return 'pedestrians_animals'  # Actual category, species requires visual review.
    if architectural:
        return 'architecture_already_cataloged'
    if category in (1, 3, 5, 6):
        return 'architecture_terrain_materials'
    for group, pattern in PATTERNS:
        if any(re.search(pattern, name, re.I) for name in names):
            return group
    return 'unclassified_review'


def catalog(disc, decoder, output):
    output = output.resolve()
    if output == ROOT or ROOT in output.parents:
        raise ValueError('retail reference output must be outside repository')
    output.mkdir(parents=True, exist_ok=True)
    spec = importlib.util.spec_from_file_location('local_prop_decompressor', decoder)
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    palette = (disc / 'DATA/PALETTE.RAW').read_bytes()
    if len(palette) != 1024:
        raise ValueError('unsupported palette')
    from PIL import Image
    architecture = {r['id'] for r in json.loads((ROOT / 'art/buildings/mesh_assets/catalog_designs.json').read_text())['designs']}
    course_data = {p.stem: course_loads(p.read_bytes()) for p in sorted((disc / 'DATA/COURSES').glob('*.CRS'))}
    packed = resources((disc / 'DATA/FAMILIES.RSC').read_bytes())
    assets = []; source_cells = []; rejected = []; nonpixel_leaves = Counter(); images = {}
    inputs = [dict(path='DATA/PALETTE.RAW', sha256=hashlib.sha256(palette).hexdigest())]
    inputs.extend(dict(path=str(p.relative_to(disc)), sha256=hashlib.sha256(p.read_bytes()).hexdigest())
                  for p in sorted((disc / 'DATA/COURSES').glob('*.CRS')))
    def add_payload(data, source, hierarchy=None, course=None):
        found, invalid = cells(data)
        rejected.extend(dict(source=source, **r) for r in invalid)
        linked = set()
        graphics = leaves(data) if hierarchy is not None else [([], 0, word(data, 0))]
        grouped = {}
        for path, offset, tag in graphics:
            refs = references(data, offset, tag)
            if not refs:
                nonpixel_leaves[f'{tag:08x}'] += 1
                continue
            key = tuple(path[:2]) if hierarchy is not None else ()
            record = grouped.setdefault(key, dict(source=source, family_id=hierarchy, category=path[0] if path else None,
                                                 selector=path[1] if len(path) > 1 else None, leaves=[], cell_offsets=set()))
            resolved = [r for r in refs if r['cel_offset'] in found]
            record['leaves'].append(dict(path=path, offset=offset, tag=f'{tag:08x}', references=resolved,
                                         non_cel_pixel_references=[r for r in refs if r['cel_offset'] not in found]))
            record['cell_offsets'].update(r['cel_offset'] for r in resolved)
            linked.update(record['cell_offsets'])
        for offset, cell in found.items():
            source_cells.append(dict(source=source, family_id=hierarchy, linked=offset in linked, **cell))
            digest = cell['pixel_sha256']
            if digest not in images:
                _, pixels = texture(data, offset)
                im = Image.new('RGBA', (cell['width'], cell['height']))
                im.putdata([(*palette[v * 4:v * 4 + 3], 0 if v == 0 else 255) for v in pixels])
                images[digest] = im
        for key, record in grouped.items():
            selected = [found[p] for p in sorted(record.pop('cell_offsets'))]
            if not selected:
                continue
            record['cells'] = selected
            record['names'] = sorted({c['name'] for c in selected})
            record['group'] = classify(record['category'], record['names'], bool(set(record['names']) & architecture), course is not None)
            record['course_family_load_matches'] = [course] if course else [c for c, v in course_data.items() if hierarchy in v['loaded_family_ids']]
            record['source_identity'] = source + ':' + '.'.join(map(str, key))
            assets.append(record)
        # Keep every unlinked CEL visible; no name-only exclusion can hide unknown props.
        for offset in sorted(set(found) - linked):
            cell = found[offset]
            assets.append(dict(source=source, family_id=hierarchy, category=None, selector=None,
                               source_identity=f'{source}:unlinked:{offset}', leaves=[], cells=[cell], names=[cell['name']],
                               group=classify(None, [cell['name']], cell['name'] in architecture),
                               course_family_load_matches=[course] if course else [c for c, v in course_data.items() if hierarchy in v['loaded_family_ids']],
                               unresolved='unlinked indexed cell; original use not established'))
    for (tag, identity), data in sorted(packed.items()):
        if tag == 'FAM ':
            add_payload(module.unpack(data), f'DATA/FAMILIES.RSC:FAM:{identity}', identity)
    inputs.append(dict(path='DATA/FAMILIES.RSC', sha256=hashlib.sha256((disc / 'DATA/FAMILIES.RSC').read_bytes()).hexdigest()))
    car_paths = sorted((disc / 'DATA/CARS').glob('*.CAR'))
    if {p.stem for p in car_paths} != {'CITY', 'CANYN', 'HIWAY', 'MEDLY', 'NAPA'}:
        raise ValueError('all five course vehicle libraries are required')
    for path in car_paths:
        raw = path.read_bytes(); inputs.append(dict(path=str(path.relative_to(disc)), sha256=hashlib.sha256(raw).hexdigest()))
        for (tag, identity), data in sorted(resources(raw).items()):
            if tag == 'ANIM':
                add_payload(data, f'{path.relative_to(disc)}:ANIM:{identity}', course=path.stem)
    path = disc / 'DATA/GLOBAL.RSC'
    raw = path.read_bytes(); inputs.append(dict(path=str(path.relative_to(disc)), sha256=hashlib.sha256(raw).hexdigest()))
    for (tag, identity), data in sorted(resources(raw).items()):
        if tag == 'CEL ':
            first = len(assets)
            add_payload(data, f'DATA/GLOBAL.RSC:CEL:{identity}')
            for asset in assets[first:]:
                asset['group'] = 'global_materials_review'
    # Collapse only pixel-identical libraries, keeping every logical alias and frame table.
    unique = {}
    for asset in assets:
        signature = (asset['group'], tuple(sorted({c['pixel_sha256'] for c in asset['cells']})))
        entry = unique.setdefault(signature, dict(group=asset['group'], aliases=[], cells={}, names=set(), courses=set()))
        entry['aliases'].append({k: v for k, v in asset.items() if k not in ['cells', 'names', 'group', 'course_family_load_matches']})
        entry['cells'].update({c['pixel_sha256']: c for c in asset['cells']})
        entry['names'].update(asset['names']); entry['courses'].update(asset['course_family_load_matches'])
    records = []
    for index, (signature, entry) in enumerate(sorted(unique.items(), key=lambda v: (v[0][0], sorted(v[1]['names']), v[0][1]))):
        identity = 'PROP-' + hashlib.sha256(json.dumps([signature[1], sorted(a['source_identity'] for a in entry['aliases'])]).encode()).hexdigest()[:12].upper()
        cells_ = sorted(entry['cells'].values(), key=lambda c: (-c['width'] * c['height'], c['name'], c['pixel_sha256']))
        records.append(dict(id=identity, group=entry['group'], names=sorted(entry['names']), cells=cells_, aliases=entry['aliases'],
                            course_family_load_matches=sorted(entry['courses']), representative=cells_[0]['pixel_sha256'],
                            production_status='reference_only', concept_status='not_started', mesh_status='not_started',
                            classification_evidence='category + name heuristic; art identity/species and intended placement require review'))
    raw_dir = output / 'individual'; raw_dir.mkdir(exist_ok=True)
    for digest, im in sorted(images.items()):
        im.save(raw_dir / (digest + '.png'))
    result = dict(schema_version=1, reference_commit=COMMIT, inputs=inputs, palette_sha256=hashlib.sha256(palette).hexdigest(),
                  decoder_sha256=hashlib.sha256(decoder.read_bytes()).hexdigest(),
                  family_count=sum(tag == 'FAM ' for tag, _ in packed), car_files=len(car_paths),
                  scope='All indexed CELs in every FAM, every ANIM in five course CAR files, and GLOBAL CEL resources. Not all disc media: rider BIKERS banks, UI, skies/audio excluded.',
                  limitation='Family loads are eligibility, not selected placements; pixel-identical aliases deduplicated, different views/frames retained. Group names are provisional, no retail art is a new mesh.',
                  asset_count=len(records), groups=dict(Counter(r['group'] for r in records)),
                  scanned_cells=len(source_cells), unique_images=len(images), rejected_magic_candidates=rejected,
                  nonpixel_leaf_tags=dict(nonpixel_leaves), source_cells=source_cells, records=records)
    (output / 'catalog.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


if __name__ == '__main__':
    ap = argparse.ArgumentParser(); ap.add_argument('disc', type=Path); ap.add_argument('--decoder', type=Path, required=True)
    ap.add_argument('--output', type=Path, required=True); args = ap.parse_args()
    result = catalog(args.disc, args.decoder, args.output)
    print(json.dumps({k: result[k] for k in ['family_count', 'car_files', 'asset_count', 'groups', 'scanned_cells', 'unique_images']}))
