#!/usr/bin/env python3
"""Read normalized rider-stat observations from a user-supplied local executable.

Reference-only format research: arkits/road-rash 4e052aca. No media extraction.
Outputs numeric observations; does not modify runtime authoring files.
"""
import argparse
from pathlib import Path
import struct,json
parser=argparse.ArgumentParser()
parser.add_argument('executable',type=Path)
d=parser.parse_args().executable.read_bytes()
if len(d)<0x77a00 or d[:2]!=b'MZ': raise ValueError('Expected reference PE executable')
w=lambda va:struct.unpack_from('<i',d,0x61600+va-0x463000)[0]
profiles=[]
for k in range(15):
 pi=k if k<9 else 0;b=0x464558+pi*0x204;r=0x463c78+k*0x40
 profiles.append(dict(personality=pi,cruise_units=w(b+0x1bc),minimum_units=w(b+0x1a8),kick_roll=d[0x61600+b+0x1d0-0x463000],reach=w(b+0x1d4),strength=w(r+0x30),stamina=w(r+0x34),weight_lb=w(r+0xc),relationships=[(w(b+i*12+4)-w(b+i*12)+w(b+i*12+8))/1024 for i in range(10)],hit_reaction=(w(b+0xf0+7*4)-w(b+0xf0+6*4)+w(b+0xf0+8*4))/1024,calm_per_second=(w(b+0x12c)+w(b+0x130)+w(b+0x134))/1024))
print(json.dumps(profiles,indent=2))
