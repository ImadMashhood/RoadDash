#!/usr/bin/env python3
"""Produce reviewable street/property/height plans BEFORE rearranging the world.
Existing lots/footprints are measured; ownership, lane hierarchy and fences are proposals.
"""
import argparse,json,math,html,hashlib
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'docs/planning/world-redesign';OUT.mkdir(parents=True,exist_ok=True)
parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('report',type=Path,help='JSON emitted by --inspect-world');args=parser.parse_args()
r=json.loads(args.report.read_text());layout=r['layout'];buildings=layout['buildings']+layout['neighborhood'];points=r['centerline']
allxz=[(p[0],p[2]) for p in points]+[tuple(b['position'][::2]) for b in buildings]
minx=min(x for x,z in allxz)-160;maxx=max(x for x,z in allxz)+160;minz=min(z for x,z in allxz)-160;maxz=max(z for x,z in allxz)+160
zones=[dict(id='country',start_m=0,end_m=2700,lanes=2,shoulder_m=.6,sidewalk_m=0,land_use='fields, farmyards, hedgerows; no urban pavement'),dict(id='gateway',start_m=2700,end_m=3150,lanes=3,shoulder_m=.4,sidewalk_m=2.2,land_use='village shops, yards, turn pockets'),dict(id='city',start_m=3150,end_m=4550,lanes=4,shoulder_m=.3,sidewalk_m=2.8,land_use='continuous frontage, courtyards, services, parks'),dict(id='edge',start_m=4550,end_m=4950,lanes=3,shoulder_m=.4,sidewalk_m=2.0,land_use='workshops and lower density housing'),dict(id='hills',start_m=4950,end_m=r['baked_length_m'],lanes=2,shoulder_m=.5,sidewalk_m=0,land_use='pasture boundaries, stone walls, clustered woodland')]
def corners(rect):
 x,z=rect['center'];a,b=rect['right'];hx,hz=rect['half'];return [[x+a*hx*s-b*hz*t,z+b*hx*s+a*hz*t] for s,t in [(-1,-1),(1,-1),(1,1),(-1,1)]]
parcels=[]
for b in buildings:
 zone=next(z for z in zones if z['start_m']<=b['station_m']<=z['end_m'])
 block=f"{zone['id']}-{int(b['station_m']//100):02d}-{int(b['lane_m']>0)}"
 parcels.append(dict(id=b['lot_id'],building_id=b['id'],owner_id='OWNER-'+block,block_id=block,zone=zone['id'],boundary=corners(b['lot']),building_footprint=corners(b['footprint']),entrance=b['access'],height_m=b['size'][1],floors=b['floors'],height_status='existing measured',ownership_status='proposed planning identity; not legal ownership',fence_policy='street frontage open in city; rear/side fence with access gaps' if zone['id'] in ['city','gateway','edge'] else 'field/yard boundary, with driveway gate and road sightline clearance'))
for p in parcels:
 boundary=p['boundary'];access=p['entrance'][::2]
 edges=list(zip(boundary,boundary[1:]+boundary[:1]))
 gate=min(range(4),key=lambda i:sum(((edges[i][0][k]+edges[i][1][k])/2-access[k])**2 for k in range(2)))
 p['proposed_fence_segments']=[dict(start=a,end=b,style='stone wall' if p['zone'] in ['country','hills'] else 'rear courtyard fence') for i,(a,b) in enumerate(edges) if i!=gate]
 p['access_edge']=dict(start=edges[gate][0],end=edges[gate][1],status='keep open; gate dimensions require access-reservation validation')
plan=dict(schema_version=1,status='planning first; existing geometry measured, proposed changes not yet applied',source_geometry_fingerprint=r['geometry_fingerprint'],source_sha256=hashlib.sha256(args.report.read_bytes()).hexdigest(),art_reference=dict(name='Come Bye: A Sheepdog Simulator, 3D pastoral version',url='https://store.steampowered.com/app/1282380/',source='user-provided 3D screenshot and official Steam page; reference only, no art imported'),lane_proposal=zones,parcels=parcels,public_access_reservations=layout['access_reservations'],field_boundaries=layout['fields'],street_network=layout['grid_streets'],scope='Preserve lot IDs/quotas. Reconcile property-wide coherent use, driveways, fences and heights before city changes; actual ownership names are fictional planning IDs.')
(OUT/'PLAN.json').write_text(json.dumps(plan,indent=2)+'\n')
W,H=1600,2200;PAD=100;S=min((W-2*PAD)/(maxx-minx),(H-280)/(maxz-minz))
def xy(p):return ((W-(maxx-minx)*S)/2+(p[0]-minx)*S,H-110-(p[1]-minz)*S)
try:font=ImageFont.truetype('DejaVuSans.ttf',18)
except OSError:font=ImageFont.truetype('NotoSans-Regular.ttf',18)
colors={'country':'#b6c697','gateway':'#dac493','city':'#cbb6a6','edge':'#adc1ba','hills':'#8eae88'}
svgs={}
for mode,title in [('streets','STREET AND LAND USE PLAN'),('property','PROPERTY AND FENCE PLAN'),('heights','BUILDING HEIGHT MAP')]:
 im=Image.new('RGB',(W,H),'#17252c');d=ImageDraw.Draw(im);d.text((55,28),'ROAD / DASH — '+title,font=font,fill='white');d.text((55,62),'Measured existing lots + PROPOSED hierarchy / ownership / fence policies',font=font,fill='#e8ca84')
 svg=[f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}"><rect width="100%" height="100%" fill="#17252c"/>']
 for a,b in zip(points,points[1:]):
  aa=xy((a[0],a[2]));bb=xy((b[0],b[2]));d.line([aa,bb],fill='#d9d2ba',width=5);svg.append(f'<path d="M{aa[0]},{aa[1]} L{bb[0]},{bb[1]}" stroke="#d9d2ba" stroke-width="5"/>')
 for street in layout['grid_streets']:
  aa=xy(street['a'][::2]);bb=xy(street['b'][::2]);d.line([aa,bb],fill='#72858e',width=2);svg.append(f'<path d="M{aa[0]},{aa[1]} L{bb[0]},{bb[1]}" stroke="#72858e" stroke-width="2"/>')
 for i,p in enumerate(parcels):
  h=min(p['height_m']/75,1);color=('#%02x%02x%02x'%(int(80+h*160),int(190-h*110),int(210-h*150))) if mode=='heights' else colors[p['zone']]
  shape=p['boundary'] if mode=='property' else p['building_footprint'];poly=[xy(v) for v in shape];d.polygon(poly,fill=color,outline='#22313a')
  title=html.escape(f"{p['id']} | {p['owner_id']} | {p['height_m']:.1f} m | {p['floors']} floors | {p['fence_policy']}")
  svg.append(f'<polygon points="'+ ' '.join(f'{x},{y}' for x,y in poly)+f'" fill="{color}" stroke="#22313a" stroke-width=".5"><title>{title}</title></polygon>')
  if mode=='property':
   start=xy(p['entrance'][::2]);center=xy([sum(v[0] for v in p['building_footprint'])/4,sum(v[1] for v in p['building_footprint'])/4]);d.line([start,center],fill='#faf0b6',width=1)
 for i,z in enumerate(zones):
  label=f"{z['id'].upper()}  {z['lanes']} lanes proposed / {z['sidewalk_m']} m pavement";d.text((65,110+i*28),label,font=font,fill=colors[z['id']])
 d.text((55,H-65),f"{len(parcels)} stable parcels / colour on height map: blue low → orange tall / NOT construction approval",font=font,fill='white')
 im.save(OUT/(mode+'.png'));svg.append('</svg>');svgs[mode]=''.join(svg);(OUT/(mode+'.svg')).write_text(svgs[mode])
(OUT/'index.html').write_text('<!doctype html><meta charset="utf-8"><title>RoadDash property and street planning</title><style>body{background:#17252c;color:#eee;font:16px system-ui;margin:24px}nav{position:sticky;top:0;background:#17252c;padding:15px}button{padding:12px;margin-right:12px}svg{width:95%;max-height:85vh}section{display:none}section:first-of-type{display:block}</style><h1>RoadDash: city and countryside planning</h1><p>Existing measured footprints and heights. Proposed ownership, road hierarchy and fence policies. Hover each parcel for its ID and intended use.</p><nav>'+''.join(f'<button onclick="document.querySelectorAll(\'section\').forEach(s=>s.style.display=\'none\');document.getElementById(\'{m}\').style.display=\'block\'">{m.title()}</button>' for m in svgs)+'</nav>'+''.join(f'<section id="{m}">{s}</section>' for m,s in svgs.items()))
print('Planned',len(parcels),'parcels;street/property/height maps and machine-readable proposal written')

# Detail panels are clipped separately so off-sheet roads never cover titles.
subset=[p for p in parcels if p['building_id'] in {b['id'] for b in buildings if 3400<b['station_m']<3650}]
coords=[v for p in subset for v in p['boundary']]
minx=min(x for x,z in coords)-10;maxx=max(x for x,z in coords)+10
minz=min(z for x,z in coords)-10;maxz=max(z for x,z in coords)+10
sheet=Image.new('RGB',(1800,1100),'#17252c');sd=ImageDraw.Draw(sheet)
fontpath=font.path;heading=ImageFont.truetype(fontpath,28);small=ImageFont.truetype(fontpath,10)
sd.text((35,22),'ROAD / DASH — CITY BLOCK STUDY — 3.40–3.65 km',font=heading,fill='white')
sd.text((35,70),'Measured streets / lots / heights; proposed fences and fictional owners. Layout changes not yet applied.',font=font,fill='#dac493')
S=min(540/(maxx-minx),850/(maxz-minz))
for col,mode in enumerate(['STREETS / ACCESS','PROPERTIES / FENCES','BUILDING HEIGHTS']):
 left=25+col*595;sd.text((left,115),mode,font=font,fill='white')
 panel=Image.new('RGB',(570,860),'#17252c');pd=ImageDraw.Draw(panel)
 def px(v):return(15+(v[0]-minx)*S,850-(v[1]-minz)*S)
 for street in layout['grid_streets']:
  pd.line([px(street['a'][::2]),px(street['b'][::2])],fill='#72858e',width=5)
 pd.line([px((v[0],v[2])) for v in points],fill='#d9d2ba',width=max(4,int(13*S)))
 for p in subset:
  color=['#e0bd89','#9ab4a3','#aab8d5','#c5a0b0'][int(p['block_id'].split('-')[-2])%4]
  if col==2:
   t=min(p['height_m']/75,1);color=(int(80+160*t),int(190-110*t),int(210-150*t))
  pd.polygon([px(v) for v in (p['boundary'] if col==1 else p['building_footprint'])],fill=color,outline='#243740')
  if col==1:
   for edge in p['proposed_fence_segments']:pd.line([px(edge['start']),px(edge['end'])],fill='#efcf82',width=2)
  c=[sum(v[k] for v in p['building_footprint'])/4 for k in [0,1]]
  if col==2:pd.text(px(c),f"{p['height_m']:.0f}",font=small,fill='#14232b',anchor='mm')
  pd.line([px(p['entrance'][::2]),px(c)],fill='#eaeab8',width=1)
 sheet.paste(panel,(left,160))
 sd.line([(left+25,1040),(left+25+50*S,1040)],fill='white',width=3)
 sd.text((left+25,1055),'50 m / heights in metres / full IDs in interactive map',font=small,fill='white')
sheet.save(OUT/'city-block-study.png')
