#!/usr/bin/env python3
"""Attach explicitly confidence-rated visual observations to numeric slot metadata."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
def family(i):
    if i<=21:return 'riding-bank/view'
    if i in [22,23]:return 'distance-variant'
    if 41<=i<=59:return 'unarmed/raised-hands'
    if 60<=i<=70:return 'kick'
    if 81<=i<=117:return 'chain-gesture'
    if 140<=i<=179:return 'club-gesture'
    if 191<=i<=220:return 'unarmed-variant'
    if 225<=i<=232:return 'contact-reach/reaction'
    if 241<=i<=248:return 'ground-crash'
    if 252<=i<=255:return 'mounted-loss-of-balance'
    if 256<=i<=282:return 'airborne/tumble'
    if 283<=i<=289:return 'get-up'
    if 291<=i<=308:return 'run-front' if i<297 else 'run-rear' if i<303 else 'run-side'
    if 309<=i<=313:return 'mount/recovery'
    if 319<=i<=322:return 'bike-only-wreck'
    return 'unclassified-view'
anchors={3:'ride-neutral',4:'bank-right-full',11:'bank-left-full',44:'punch-right-windup',45:'punch-right-strike',51:'punch-left-windup',52:'punch-left-strike',60:'kick-right',66:'kick-left',101:'chain-right-windup',102:'chain-right-strike',105:'chain-left-windup',106:'chain-left-strike',140:'club-left-windup',141:'club-right-windup',142:'club-left-strike',143:'club-right-strike'}
for name in ['reference-slots','police-slots']:
    path=ROOT/f'docs/animation/{name}.json';d=json.loads(path.read_text())
    for slot in d['slots']:
        i=slot['slot'];slot['classification']='empty' if slot['empty'] else family(i) if name=='reference-slots' else 'police-ride/club' if i<32 else 'police-wreck' if i<36 else 'police-tumble'
        slot['confidence']='empty-payload' if slot['empty'] else 'source-helper-anchor' if name=='reference-slots' and i in anchors else 'visual-inspection-family-only'
        slot['semantic_anchor']=anchors.get(i) if name=='reference-slots' else None
        slot['original_duration_ticks']=None
        slot['sequence_order_verified']=False
    d['inspection']='Local 1997 disc sheets visually reviewed; pinned source helpers confirm only named anchor slots. No proprietary pixels or timings emitted.'
    path.write_text(json.dumps(d,indent=2)+'\n')
