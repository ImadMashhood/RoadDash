#!/usr/bin/env python3
"""Write metadata-only rebuild queues, separating reference art from deliverables."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
EXCLUDED = {'architecture_already_cataloged', 'architecture_terrain_materials', 'global_materials_review'}
ROLES = {
    'animals': ('mobile collision hazard; preserve existing NPC ragdoll behavior', 'species mesh/rig; independent capsule/compound proxy', 'fields/verges with readable approach and escape room'),
    'pedestrians_animals': ('walking/reactive actor; species and exact identity need review', 'reuse human or quadruped rig after species review; do not change accepted physical rig', 'sidewalk/plaza paths; crossing sight lines and recovery routes stay open'),
    'traffic_vehicles': ('moving same-direction/oncoming/cross-traffic obstacle', 'separate body/wheels and authored collision proxy; preserve traffic/crash calculations', 'American traffic lanes and authored crossing windows'),
    'parked_vehicles_machinery': ('solid parked obstacle or field machinery', 'body/wheel meshes plus independent collider', 'parking/service yards/fields outside race clearances'),
    'trees': ('solid trunk obstacle and visibility landmark', 'trunk/canopy LOD meshes; independent trunk collider', 'biome/sidewalk planting sockets with minimum visibility and spacing'),
    'shrubs_plants': ('vegetation dressing; collision depends on reviewed plant scale', 'cluster geometry/alpha-tested foliage LODs; reviewed proxy if gameplay relevant', 'planters/verges/fields outside navigation and road reservations'),
    'street_lighting_utilities': ('street landmark and solid roadside obstacle', 'slender pole/base/fixture mesh and independent proxy; light emissive separate', 'sidewalk edge/service sockets, clear crossing sight lines'),
    'signs_billboards': ('route/traffic readability and roadside obstacle', 'post/panel geometry, original authored lettering texture, independent proxy', 'approach-facing junction/roadside sockets outside racing line'),
    'fences_barriers': ('lane/field boundary and physical obstacle', 'modular posts/rails/panels, matching independent collision', 'whole-lot boundaries with access/recovery gaps'),
    'rocks_landforms': ('solid terrain obstacle and landmark', 'closed rock mesh/LODs and conservative collision', 'biome clusters outside roadway and recovery corridors'),
    'street_furniture': ('solid local obstacle/dressing', 'distinct authored mesh/material plus independent proxy', 'sidewalk/service sockets preserving pedestrian clear width'),
    'loose_materials_hazards': ('review solid pile versus surface grip hazard', 'closed pile mesh or authored surface material; collision policy requires review', 'authored readable hazard sockets; no unavoidable random blockage'),
    'unclassified_review': ('unresolved; inspect reference before naming or modeling', 'blocked pending visual identification', 'no procedural placement until identified'),
}
CURRENT = [
    dict(category='rider_and_recovery_human', sources=['src/models/mod.rs', 'src/human.rs', 'src/recovery.rs'], status='existing authored visual rig; accepted eleven-body physics must be preserved'),
    dict(category='pedestrians_cows', sources=['src/environment.rs:spawn_pedestrian', 'src/environment.rs:spawn_cow', 'src/npc.rs'], status='existing human/quadruped knockdowns and respawn; visual replacement needs differential tests'),
    dict(category='traffic_cross_traffic', sources=['src/world.rs', 'src/game.rs:traffic', 'src/environment.rs:spawn_crossing'], status='existing procedural vehicle bodies and behavior; art replacement must preserve motion/collision authority'),
    dict(category='trees_fences_guardrails_lamps', sources=['src/environment.rs:build', 'src/world_gen/city/mod.rs'], status='existing procedural static geometry; preserve reservations and independent collision'),
    dict(category='road_markings_start_finish_sidewalks', sources=['src/world.rs', 'src/environment.rs', 'src/world_gen/city/mod.rs'], status='existing road/support geometry; decorative replacement must not remove support'),
]


def plan(folder):
    data = json.loads((folder / 'catalog.json').read_text())
    concept_file = ROOT / 'art/props/concept_3d/MANIFEST.json'
    concepts = {}
    if concept_file.exists():
        concept = json.loads(concept_file.read_text())
        concepts = {c['id']: dict(file='art/props/concept_3d/' + concept['file'], slot=c['slot']) for c in concept['cells']}
    batch_file = ROOT / 'art/props/concept_3d/BATCH_02.json'
    if batch_file.exists():
        for sheet in json.loads(batch_file.read_text()):
            if sheet['status'] == 'draft_not_approved':
                for c in sheet['cells']:
                    concepts[c['id']] = dict(file='art/props/concept_3d/' + sheet['file'], slot=c['slot'])
    remaining_file = ROOT / 'art/props/concept_3d/ALL_REMAINING.json'
    if remaining_file.exists():
        for sheet in json.loads(remaining_file.read_text())['sheets']:
            if sheet['status'] == 'draft_not_approved':
                for c in sheet['cells']:
                    concepts[c['id']] = dict(file=sheet['file'], slot=c['slot'])
    for record in data['records']:
        if record['id'] in concepts:
            record['concept_status'] = 'draft_not_approved'
            record['concept'] = concepts[record['id']]
    metadata = ROOT / 'docs/research/props'; metadata.mkdir(parents=True, exist_ok=True)
    (metadata / 'source-inventory.json').write_text(json.dumps(data, indent=2) + '\n')
    mesh_manifest = ROOT / 'art/props/mesh_assets/MANIFEST.json'
    meshes = {r['id']: r for r in json.loads(mesh_manifest.read_text())['records']} if mesh_manifest.exists() else {}
    runtime_file = ROOT / 'content/props.json'
    runtime = {r['id']: r for r in json.loads(runtime_file.read_text())['records']} if runtime_file.exists() else {}
    native = set()
    for receipt in (ROOT / 'docs/testing/props-in-game').glob('native-*.json'):
        native.update(json.loads(receipt.read_text())['selected_ids'])
    jobs = []
    for r in data['records']:
        if r['group'] in EXCLUDED:
            continue
        role, collider, placement = ROLES[r['group']]
        jobs.append(dict(id=r['id'], source_names=r['names'], group=r['group'], review_number=r['review_number'],
                         reference_sheet=r['sheet'], sheet_slot=r['sheet_slot'], source_aliases=[a['source_identity'] for a in r['aliases']],
                         course_family_eligibility=r['course_family_load_matches'], role=role, planned_geometry_and_collision=collider,
                         placement_rule=placement, rule_status='RoadDash proposed design; not measured original placement', dimensions_m=None,
                         dimensions_status='must author and review; sprites do not establish metres or hidden geometry',
                         reference_status='extracted_local_only', identity_status='needs_visual_review', concept_status=r['concept_status'], concept=concepts.get(r['id']),
                         model_status='not_started', lod_status='not_started', collision_status='not_verified',
                         animation_status='needs_actor_review' if r['group'] in {'pedestrians_animals', 'animals'} else 'not_applicable_or_unreviewed',
                         native_status='not_integrated', approved=False,
                         required_outputs=['labeled concept', 'editable source', 'GLB geometry LODs', 'authored materials', 'independent collider', 'native multi-angle proof'],
                         preserve_existing_physics=True))
    for job in jobs:
        if job['id'] in meshes:
            mesh = meshes[job['id']]
            job['model_status'] = 'draft_volumetric_mesh_not_approved'
            job['lod_status'] = 'three_exported_geometry_lods'
            job['mesh_catalog'] = 'art/props/mesh_assets/MANIFEST.json'
            job['mesh_files'] = ['art/props/mesh_assets/' + l['file'] for l in mesh['lods']]
            job['animation_status'] = mesh['rig_status'] if mesh['rig_status']=='not_rigged' else job['animation_status']
        if job['id'] in native:
            job['native_status'] = 'development_selected_in_native_scene'
            job['native_evidence'] = 'docs/testing/props-in-game/'
        if runtime.get(job['id'], {}).get('skin_files'):
            job['animation_status'] = 'draft_bound_to_existing_body_frames_not_final_deformation'
    result = dict(schema_version=1, source_catalog_sha256=hashlib.sha256((metadata / 'source-inventory.json').read_bytes()).hexdigest(),
                  queue_entries=len(jobs), categories=dict(Counter(j['group'] for j in jobs)),
                  warning='Queue entries are source view/color/size banks, not a count of unique finished 3D models. Archetype consolidation requires review.',
                  original_art_style_reference='art/buildings/concept_3d/facades_01.png',
                  runtime_inventory=CURRENT, stages=['reference coverage', 'visual identity/archetype consolidation', 'new concept/material authoring',
                                                   'real mesh/rig/LOD export', 'semantic placement', 'collision/audio/navigation QA'], jobs=jobs)
    (metadata / 'rebuild-queue.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


if __name__ == '__main__':
    ap = argparse.ArgumentParser(); ap.add_argument('folder', type=Path); a = ap.parse_args(); d = plan(a.folder)
    print(json.dumps({k: d[k] for k in ['queue_entries', 'categories', 'warning']}))
