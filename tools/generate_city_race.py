#!/usr/bin/env python3
"""Bake the playable 5.3km city and isolated authoring preview from one config."""
import argparse
import json
from pathlib import Path
from city_buildings import ROOT,generate


def outputs(config):
    scene=generate(config);track=json.loads((ROOT/'content/tracks/valley.json').read_text());length=config['length_m']
    track.update(id='city-run-5300',name='City Run — 5.3 km / city districts',scenery_seed=config['seed'],verge_width=config['sidewalk_width_m'],closed=False)
    track['control_points']=[dict(position=[0,.025,z*length/5300],half_width=config['road_width_m']/2,bank_degrees=0,sector='city',surface='asphalt')for z in [0,500,1300,2000,3000,4000,4800,5300]]
    track['props']=[dict(id='city-crossing-'+str(i),kind='crossing',fraction=i/length,lane=0)for i in range(500,int(length),500)]
    return {ROOT/'content/world/city5300.json':scene,ROOT/'content/world/city_settings.json':config,ROOT/'content/tracks/city5300.json':track,ROOT/'art/buildings/city_preview/generated/scene.json':scene}


if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('--config',type=Path,default=ROOT/'art/buildings/city_preview/config.json');ap.add_argument('--check',action='store_true');ap.add_argument('--seed',type=int);a=ap.parse_args();config=json.loads(a.config.read_text())
    if a.seed is not None:
        config['seed']=a.seed
        if not a.check:a.config.write_text(json.dumps(config,indent=2)+'\n')
    for path,data in outputs(config).items():
        if a.check:
            if json.loads(path.read_text())!=json.loads(json.dumps(data)):raise SystemExit('Stale city artifact '+str(path))
        else:path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(data,separators=(',',':'))+'\n')
    print('Playable city and preview match one validated deterministic plan'if a.check else'Baked city track, settings, whole buildings and preview')
