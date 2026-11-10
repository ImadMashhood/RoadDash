"""Measure former centre-height stripe burial against actual baked pavement."""
import bisect,json,math
from pathlib import Path
from shapely import Polygon,Point,STRtree
ROOT=Path(__file__).resolve().parents[1]
def height(x,z):
    farm=12+2*math.sin(z/950)+1.3*math.sin(x/650)
    city=12+.0042*z+8*math.sin(z/1050)+4*math.sin(x/700+z/1300)+6*math.sin(z/450)
    t=max(0,min(1,(z+200)/400));t=t*t*(3-2*t)
    return farm*(1-t)+city*t

def run():
    m=json.loads((ROOT/'content/world/metropolis.json').read_text())
    tris=[];polys=[]
    for surface in m['surfaces']:
        if surface['kind']!=0:continue
        vv=[p for ring in surface['shape']['rings'] for p in ring]
        for ix in surface['shape']['triangles']:
            tri=[vv[i] for i in ix];tris.append(tri);polys.append(Polygon(tri))
    tree=STRtree(polys);buried=[];missing=[];count=0;worst=0
    for route,path in enumerate(m['traffic_paths']):
        for s in range(2800,12000,4):
            if route==1 and not 3350<=s<5362.5:continue
            profile=next(p for p in m['street_life']['lane_sections'] if p['route']==route and p['start']<=s<p['end'])
            assert profile['outbound']+profile['inbound']==(3 if 3350<=s<5362.5 else 4)
            prev=next((p for p in m['street_life']['lane_sections'] if p['route']==route and p['end']==profile['start']),profile)
            t=max(0,min(1,(s-profile['start'])/100));t=t*t*(3-2*t)
            divider=lambda p:(p['outbound']-p['inbound'])*p['lane_width']/2
            div=divider(prev)*(1-t)+divider(profile)*t
            width=prev['lane_width']*(1-t)+profile['lane_width']*t
            i=min(len(path['points'])-2,bisect.bisect_right(path['progress'],s)-1)
            a,b=path['points'][i:i+2];u=(s-path['progress'][i])/(path['progress'][i+1]-path['progress'][i]);x=a[0]+(b[0]-a[0])*u;z=a[2]+(b[2]-a[2])*u
            dx,dz=b[0]-a[0],b[2]-a[2];n=math.hypot(dx,dz);dx,dz=dx/n,dz/n
            if s%12>=6:continue
            for sign,key in [(-1,'outbound'),(1,'inbound')]:
                for slot in range(1,profile[key]):
                    lane=div+sign*slot*width;px,pz=x-dz*lane,z+dx*lane;q=Point(px,pz);heights=[]
                    for k in tree.query(q,predicate='intersects'):
                        aa,bb,cc=tris[k];ab=[bb[j]-aa[j] for j in range(2)];ac=[cc[j]-aa[j] for j in range(2)];ap=[px-aa[0],pz-aa[1]]
                        det=ab[0]*ac[1]-ab[1]*ac[0]
                        uu=(ap[0]*ac[1]-ap[1]*ac[0])/det;v=(ab[0]*ap[1]-ab[1]*ap[0])/det
                        heights.append(height(*aa)+uu*(height(*bb)-height(*aa))+v*(height(*cc)-height(*aa))+.025)
                    count+=1
                    if not heights:missing.append([route,s,lane]);continue
                    burial=max(heights)-(height(x,z)+.075);worst=max(worst,burial)
                    if burial>0:buried.append([route,s,round(lane,3),round(burial,5)])
    report=dict(samples=count,old_buried_samples=len(buried),worst_old_burial_m=round(worst,5),missing_pavement=missing,old_buried_examples=buried[:30],lane_counts='four before split / three per branch / four after rejoin',source_layout_sha256=m['source_layout_sha256'])
    dest=ROOT/'docs/testing/roads-walkability/paint-audit.json';dest.parent.mkdir(exist_ok=True);dest.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:v for k,v in report.items() if k!='old_buried_examples'},indent=2));assert not missing
if __name__=='__main__':run()
