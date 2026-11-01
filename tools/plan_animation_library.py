#!/usr/bin/env python3
"""Reproduce the semantic keyframe plan; durations are deliberately not recovered guesses."""
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'art/animation'
records=[]
def pose(id,name,family,mode='ground',refs=(),**params):
    records.append(dict(id=id,name=name,family=family,actor='cow' if mode=='cow' else 'human',mode=mode,source_slots=list(refs),params=params,review='draft',timing_seconds=None,timing_basis='unassigned; original sprite slots do not encode duration'))
# Preserve the old review numbering, including view/distance variants. Their
# old classification is not upgraded into verified animation sequencing.
legacy=json.loads((OUT/'legacy-index.json').read_text())
for row in legacy:
    pose(row['frame_id'],row['name'],'mounted','legacy',[row['frame_id']])
for start,family,amplitude in [(1000,'walk',.22),(1010,'run',.34),(1020,'sprint',.42)]:
    for phase in range(8):pose(start+phase,f'{family}-{phase:02}', 'locomotion', gait=family,phase=phase/8,stride=amplitude)
for slot in range(291,309):
    phase=(slot-291)%6
    pose(slot,f'reference-run-{slot:04}', 'locomotion',refs=[slot],gait='run',phase=phase/6,stride=.34)
for i,(name,kwargs) in enumerate([
 ('idle',{}),('start-left',dict(phase=.15,stride=.18)),('start-right',dict(phase=.65,stride=.18)),
 ('stop-left',dict(phase=.1,stride=.22,torso=-10)),('stop-right',dict(phase=.6,stride=.22,torso=-10)),
 ('turn-left',dict(turn=35)),('turn-right',dict(turn=-35)),
 ('strafe-left',dict(phase=.2,stride=.16,strafe=.12)),('strafe-right',dict(phase=.7,stride=.16,strafe=-.12)),
 ('reverse-left',dict(phase=.2,stride=-.16)),('reverse-right',dict(phase=.7,stride=-.16)),
 ('uphill-left',dict(phase=.1,stride=.2,slope=15,torso=10)),('uphill-right',dict(phase=.6,stride=.2,slope=15,torso=10)),
 ('downhill-left',dict(phase=.1,stride=.2,slope=-15,torso=-8)),('downhill-right',dict(phase=.6,stride=.2,slope=-15,torso=-8)),
 ('cross-slope-left',dict(cross_slope=12)),('cross-slope-right',dict(cross_slope=-12)),
 ('step-up-left',dict(step='L')),('step-up-right',dict(step='R')),('ledge-brace',dict(torso=18,crouch=.18,arms='brace')),
 ('pedestrian-idle',{}),('pedestrian-startle',dict(crouch=.1,arms='protect')),('pedestrian-flee',dict(phase=.2,stride=.38)),
]):pose(1040+i,name,'terrain' if 'hill' in name or 'slope' in name or 'step' in name or 'ledge' in name else 'locomotion',**kwargs)
# Physical reaction targets are authoring-only. No force, impulse or authority is exported.
for i,(name,kwargs,refs) in enumerate([
 ('brace-front',dict(crouch=.15,arms='brace',torso=15),[]),('protect-head',dict(crouch=.2,arms='protect'),[]),
 ('tuck-airborne',dict(crouch=.35,arms='protect',air=True),[262]),('reach-left',dict(arms='reach-left',crouch=.1),[]),('reach-right',dict(arms='reach-right',crouch=.1),[]),
 ('prone-settle',dict(lying=90,arms='brace'),[258]),('supine-settle',dict(lying=-90,arms='protect'),[264]),
 ('roll-left',dict(lying=80,roll=-60,arms='protect',crouch=.25),[265]),('roll-right',dict(lying=80,roll=60,arms='protect',crouch=.25),[267]),
 ('failed-righting',dict(lying=-80,roll=35,crouch=.25,arms='protect'),[256]),
 ('side-left',dict(lying=90,roll=-85,crouch=.2),[260]),('side-right',dict(lying=90,roll=85,crouch=.2),[257]),
 ('repeat-impact',dict(lying=70,arms='protect',crouch=.3),[268]),('slide-front',dict(lying=85,arms='brace'),[241]),
]):pose(1080+i,name,'ragdoll',refs=refs,**kwargs)
for i,(name,kwargs,refs) in enumerate([
 ('get-up-front-support',dict(lying=70,crouch=.25,arms='brace'),[283]),
 ('get-up-front-kneel',dict(crouch=.34,arms='brace',torso=30),[284]),
 ('get-up-front-rise',dict(crouch=.16,torso=20),[285]),
 ('get-up-back-roll',dict(lying=-75,crouch=.3,arms='protect'),[282]),
 ('get-up-back-sit',dict(crouch=.42,arms='brace',torso=-15),[246]),
 ('get-up-back-rise',dict(crouch=.18,torso=15),[286]),
 ('get-up-side-left',dict(lying=50,roll=-45,crouch=.3,arms='brace'),[263]),
 ('get-up-side-right',dict(lying=50,roll=45,crouch=.3,arms='brace'),[263]),
 ('landing-contact',dict(crouch=.12,arms='brace'),[]),('landing-compression',dict(crouch=.3,arms='brace',torso=18),[]),
 ('stumble-left',dict(phase=.25,stride=.32,arms='brace',torso=22),[]),('stumble-right',dict(phase=.75,stride=.32,arms='brace',torso=22),[]),
 ('bike-approach',dict(phase=.1,stride=.2,turn=20),[]),('bike-lift-grasp',dict(crouch=.35,arms='brace',torso=35),[]),
 ('bike-lift-weight',dict(crouch=.2,arms='brace',torso=20),[]),('bike-lift-upright',dict(arms='brace',torso=5),[]),
]):pose(1100+i,name,'recovery',refs=refs,**kwargs)
for i,(name,params) in enumerate([
 ('contact-brace',dict(torso=-10)),('nose-down-contact',dict(pitch=18)),('vault-release',dict(pitch=40,release='both')),
 ('combat-swerve-left',dict(bank=25)),('combat-counter-left',dict(bank=-10)),('combat-swerve-right',dict(bank=-25)),('combat-counter-right',dict(bank=10)),
 ('tuck',dict(torso=15)),('brake',dict(torso=-12)),('shift-left-foot',dict(release='peg.L')),
]):pose(1140+i,name,'crash' if i<7 else 'riding','seated',**params)
for i,name in enumerate(['idle','graze','startle','walk-contact-left','walk-pass-left','walk-contact-right','walk-pass-right','run-flight','impact-brace','fallen-side','righting-attempt','stand-recover']):
    pose(1200+i,'cow-'+name,'animal','cow',phase=(i-3)/4 if 3<=i<=7 else 0,action=name)
# Use-case catalog maps every requested family to actual pose IDs. Timing/events
# are semantic labels; the game remains authoritative for hit windows in Part 03.
cases=[]
def use(id,family,ids,state,notes):
    cases.append(dict(id=id,family=family,actor='cow' if family=='animal' else 'human',state=state,
       pose_ids=ids,entry_pose=ids[0],exit_pose=ids[-1],phase_labels=['anticipation','contact','follow-through','recovery'] if family=='combat' else ['support','transition','release'],
       interrupts=['hit','crash','race-finished'],timing_basis='unassigned',notes=notes,review='draft'))
use('riding-bank','riding',[11,10,9,8,3,6,5,4],'riding','Input-only shallow-to-full banks; release/countersteer return through neutral. View variants are not extra time samples.')
use('riding-load','riding',[3,1147,1148,1149,3],'riding','Throttle tuck, braking, foot release for gear change; universal contact anchors.')
for name,ids in [('punch-right',[44,45,46,47]),('punch-left',[51,52,53,54]),('kick-right',[61,60,63,64]),('kick-left',[65,66,68,69]),('chain-right',[101,102,103]),('chain-left',[105,106,107]),('club-left',[140,142]),('club-right',[141,143])]:
 use(name,'combat',ids,'riding','Reference-supported side/anchor poses; socket only for weapon families. Current fingers and swing phases need art review.')
for start,name in [(1000,'walk'),(1010,'run'),(1020,'sprint')]:use(name,'locomotion',list(range(start,start+8)),'on-foot','Original 3D contact/down/passing/up stance candidates; world stride/timing will be fitted in Part 03.')
use('original-running-views','locomotion',list(range(291,309)),'on-foot','Six phase candidates in each original front/rear/side slot range. No claimed recovered timing.')
use('starts-stops-turns','locomotion',list(range(1040,1051)),'on-foot','Idle, left/right starts/stops/turns/strafe/reverse; actual manual input remains Part 03.')
use('terrain-stances','terrain',list(range(1051,1060)),'on-foot','Ground support and knee/pelvis placement for slopes, curbs and ledges. Runtime foot IK is not implemented here.')
use('pedestrian-motion','locomotion',list(range(1060,1063)),'npc','Shares the authored human rig; separate civilian idle/startle/flee vocabulary.')
use('physical-reactions','ragdoll',list(range(1080,1094)),'falling','Brace/protect/tuck/reach/roll/righting attempts/failed recovery. Authoring targets, not physics commands.')
use('get-up','recovery',list(range(1100,1108)),'getting-up','Front/back/side support poses; blend from actual physical pose in Part 03.')
use('land-stumble','recovery',list(range(1108,1112)),'falling','Landing and balance loss use grounded support rather than arbitrary launch.')
use('lift-and-mount','recovery',list(range(1112,1116))+list(range(309,314)),'mounting','Approach/grasp/weight/lift, then original numbered mount candidates; body/leg clearance still needs approval.')
use('vehicle-impact','crash',[1140,1141,1142,1082,1085],'falling','Nose-down/vault pose proposal only. Physics/contact geometry must permit it; these poses never inject energy.')
use('combat-knockoff-left','crash',[3,1143,1144,1082],'falling','Outward swerve, smaller countersteer, then release; preserve physical forward motion.')
use('combat-knockoff-right','crash',[3,1145,1146,1082],'falling','Mirrored target vocabulary; interrupted by actual collisions.')
use('police-hooks','combat',[3,44,45,1080,1106],'future-police','Reuse valid riding/contact/recovery targets. Dedicated arrest/restraint poses are a remaining authoring gap, not completed police behavior.')
use('animal-motion','animal',list(range(1200,1212)),'npc','Independent quadruped joint chain: graze, walk, flight/startle, impact, fall and recover. Prototype body, not final cow art.')
use('mounted-reference-variants','mounted',[r['frame_id'] for r in legacy],'riding','Legacy numbered view/distance/gesture variants for comparison, not additional timed animation samples. Broad inherited labels remain review candidates.')
plan=dict(schema_version=1,units='metres',axes='Blender +Z up, -Y forward, +X anatomical left',
    source_rider='art/source/rider.glb',source_bike='art/source/motorcycle.glb',pose_kind='static-keyframes',
    runtime_connected=False,physics_changed=False,poses=records,use_cases=cases)
(OUT/'pose-plan.json').write_text(json.dumps(plan,indent=2)+'\n')
print(len(records),'static pose specifications;',len(cases),'semantic use cases')
