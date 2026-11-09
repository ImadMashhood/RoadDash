"""Append six original club poses without regenerating calibrated released keys.
Run with Blender --background --python tools/append_club_motion.py.
"""
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
exporter = ROOT / 'tools/export_motion_library.py'
namespace = {'__file__': str(exporter)}
# Only the existing coordinate conversion; never execute the wholesale exporter.
exec(exporter.read_text().split('rest=frames(base);keys=[]')[0], namespace)
frames, names = namespace['frames'], namespace['names']
path = ROOT / 'assets/models/motion.json'
before = json.loads(path.read_text())
after = json.loads(path.read_text())
assert names == before['names']
for name, ids in [('club_left', [2160,2161,2162]), ('club_right', [2170,2171,2172])]:
    for key_id in ids:
        pose = json.loads((ROOT/f'art/animation/context/poses/{key_id}.json').read_text())
        local = frames(pose)
        rotations = [m.to_quaternion().normalized() for m in local]
        key = dict(id=key_id, held=sum(1 << i for i,n in enumerate(['grip.L','grip.R','peg.L','peg.R']) if n in pose['held_contacts']), root=list(local[0].translation), rotations=[[q.x,q.y,q.z,q.w] for q in rotations])
        if not any(k['id'] == key_id for k in after['keys']): after['keys'].append(key)
    if not any(c['name'] == name for c in after['clips']):
        after['clips'].append(dict(name=name, keys=ids, durations=[.17,.15,.16]))
# ALL released keys, clip fields, hierarchy, offsets and IK calibration deep-equal.
for key, value in before.items():
    assert after[key][:len(value)] == value if key in ['keys','clips'] else after[key] == value
path.write_text(json.dumps(after,separators=(',',':'))+'\n')
print('APPEND_ONLY_CLUB',len(before['keys']),len(after['keys']),len(before['clips']),len(after['clips']))
