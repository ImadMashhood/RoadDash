#!/usr/bin/env python3
"""Explicit semantic authoring recipes for the complete prop concept queue."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'art/props/mesh_assets'
# Exact-name exceptions override the provisional source categories.
EXACT={
 'COWS2':'warning_sign','CARSLIP2':'warning_sign','LIGAHDR2':'warning_sign','LIGAHDY2':'warning_sign',
 'SLATS12':'pallets','SLATS22':'pallets','TRAILSN2':'information_board','INTRSGN2':'information_board',
 'TIRE2':'tire','METERMD2':'service_trike','LOGS2':'log_stack','LOGV2':'stump',
 'FLOWRBX2':'flower_planter','VINEA2':'vine','VINEB2':'vine','BRNGRN2':'barn',
 'CHINLN2':'guardian_lion','COMBOA2':'lamp_sign','COMBOB2':'bus_stop_bin',
 'CRITPIL2':'roadside_animal','CRITPSM2':'roadside_animal','CRNRGRP2':'utility_assembly',
 'GRNDMKB2':'abutment','GRNDMKC2':'retaining_wall','ICEPDSM2':'ice_patch','RECYSTF2':'recycling_pair',
 'CALTRAN2':'arrow_trailer','RTRYCLB2':'civic_board','SIGNMTL2':'motel_sign','SIGNPUM2':'produce_sign',
 'FNSHCON2':'checked_bollard','BIGFNCN2':'checked_bollard','CONE2':'cone',
 'CNTAINY2':'crash_barrels','CONTNBY2':'crash_barrels','VEGSTND2':'produce_stand',
 'WGNBRRL2':'barrel_wagon','WINDVN2':'windmill','PARCEL2':'parcel_box','ORNGBOX2':'utility_box',
 'SDWLKBX2':'flower_planter','NEWSPIL2':'paper_pile','PHNBTH22':'pagoda_phone_booth',
 'PHONBTH2':'phone_booth','PHONE2':'payphone_pair','PHONEB2':'payphone','PHONEC2':'payphone',
 'CLBXGRY2':'callbox','CLBXRED2':'callbox','CLLBXBL2':'callbox','MUNI2':'bus_sign',
 'MAIL2':'mailbox','MAILBXS2':'mailboxes','MLBXROW2':'mailboxes',
 'TRSHBNB2':'barrel','TRSHDBL2':'bin_pair','TRASHA2':'trash_can','TRASHB2':'fallen_can',
 'TRSHBNA2':'street_bin','BINBLU@2':'street_bin','RECYCLE2':'recycling_pair',
 'BKRGRP10':'biker_group','COWEEK0':'cow','DEER2':'warning_sign',
 'TRKPGEF2':'fire_engine','TRCKWHT2':'flatbed_truck','TRKORGR2':'flatbed_truck','TRUCK2':'work_truck',
 'VWBUS2':'camper','VAN2':'van','CARSTRF2':'car_hulk','CARSTRR2':'car_hulk',
 'COPCARF2':'police_car','COPCARR2':'police_car'}

def recipe(name,group):
 if name in EXACT:return EXACT[name]
 if group=='animals':return 'sheep' if name.startswith('SHEEP') else 'cow'
 if group=='pedestrians_animals':return 'human'
 if group=='trees':
  if name.startswith(('TREEBR','TREED')):return 'bare_tree'
  if name.startswith(('TREEE','TREPLM')):return 'palm'
  if name.startswith(('TREE4','TREE5','TR5','TRE5','TREEF','TREEG','TREEU1','TREEU2','TREEW','TREEX')):return 'conifer'
  return 'broadleaf'
 if group=='shrubs_plants':return 'shrub'
 if group=='rocks_landforms':return 'rock_cluster'
 if group=='loose_materials_hazards':
  if name.startswith('HAY'):return 'hay_bales'
  if name.startswith('OIL'):return 'oil_patch'
  if name.startswith(('WAT','WTR')):return 'water_patch'
  return 'rubble_pile'
 if group=='signs_billboards':
  if name.startswith('BLLBRD'):return 'billboard'
  if name.startswith('MARKER'):return 'reflector_post'
  return 'traffic_sign'
 if group=='fences_barriers':return 'barrier' if name.startswith('BRRIC') else 'bollard'
 if group=='street_lighting_utilities':
  if name.startswith('HYDRNT'):return 'hydrant'
  if name.startswith(('TEL','TELE')):return 'utility_pole'
  if name=='METER2':return 'parking_meter'
  return 'street_lamp'
 if group=='street_furniture':
  if name.startswith(('BENCH','BNCH')):return 'bench'
  if name.startswith(('NEWSRCK','NWSRCK')):return 'news_boxes'
  if name=='PICNTBL2':return 'picnic_table'
  if name=='BOXES2':return 'box_pile'
  if name.startswith(('BAGS','TRSHBG')):return 'trash_bags'
 if group=='traffic_vehicles':
  if name.startswith('POLICE'):return 'police_car'
  if name.startswith('TAXI'):return 'taxi'
  if name.startswith('PICK'):return 'pickup'
  if name.startswith('VAN'):return 'van'
  if name.startswith(('MIATA','MUST','VETTE','PORCH','MZ')):return 'sports_car'
  return 'hatchback' if name.startswith(('GOLF','TERCL')) else 'sedan'
 raise ValueError(f'Unreviewed recipe: {name} ({group})')

def plan():
 q=json.loads((ROOT/'docs/research/props/rebuild-queue.json').read_text());designs=[]
 for j in q['jobs']:
  name=j['source_names'][0];concept=j['concept']
  if not concept:raise ValueError('Concept missing '+j['id'])
  seed=int(hashlib.sha256(name.encode()).hexdigest()[:8],16)
  designs.append(dict(id=j['id'],name=name,group=j['group'],builder=recipe(name,j['group']),seed=seed,
    concept=concept,concept_sha256=hashlib.sha256((ROOT/concept['file']).read_bytes()).hexdigest(),
    dimensions_policy='authored metre-scale; not inferred as exact original dimensions',
    acceptance='draft mesh; visual likeness, art polish and live integration pending',
    physics_policy='independent conservative authoring proxy only; never replace accepted runtime bodies'))
 OUT.mkdir(parents=True,exist_ok=True)
 result=dict(schema_version=1,coordinates='Blender Z up, front -Y; GLB Y up, front +Z; grounded origin',
   source_queue_sha256=hashlib.sha256((ROOT/'docs/research/props/rebuild-queue.json').read_bytes()).hexdigest(),designs=designs)
 (OUT/'designs.json').write_text(json.dumps(result,indent=2)+'\n')
 print('Planned',len(designs),'source-mapped meshes;',len({d['builder']for d in designs}),'semantic builders')
if __name__=='__main__':plan()
