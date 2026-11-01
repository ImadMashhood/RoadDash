#!/usr/bin/env python3
"""Assign source-call-site action names; label ordered phases, never raw slot order."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];BASE=ROOT/'docs/animation/context'
SOURCE='https://github.com/arkits/road-rash/blob/cd5504caea48aee59806dbfd11c920f33df21200/src/'
# These names describe observed selectors; an unresolved caller stays unresolved.
USES={0:('RIDE_FAR','actor_animation.cpp','NPC camera distance below second cutoff'),1:('RIDE_VERY_FAR','actor_animation.cpp','NPC distant camera selector'),2:('RIDE_NORMAL','actor_animation.cpp','Normal mounted ride'),3:('RIDE_FAST_TUCK','actor_animation.cpp','High speed / lateral-speed condition'),4:('RIDE_ACCELERATE','actor_animation.cpp','Low speed plus throttle >30'),5:('RIDE_WRECK_WARNING','actor_animation.cpp','Player wrecked flag and wreck_count <4'),6:('AI_POST_HIT_GESTURE','race.cpp','NPC probabilistic queued animation after hit'),7:('UNRESOLVED_RIDE_CODE_07','actor_descriptors.cpp','Present descriptor; no confirmed call site'),8:('RIDE_LOW_SPEED','actor_animation.cpp','Speed <300'),9:('MOUNTED_HIT_REACTION','race.cpp','Nonfatal reason-4 hit selects code9'),14:('CHAIN_STEAL_TABLE_INDEX_2','race.cpp','Steal table index2, acquired chain'),15:('CHAIN_STEAL_TABLE_INDEX_6','race.cpp','Steal table index6, acquired chain'),20:('CLUB_STEAL_UNAVAILABLE_LEFT','race.cpp','Steal table selects absent sequence; fallback16'),21:('CLUB_STEAL_UNAVAILABLE_RIGHT','race.cpp','Steal table selects absent sequence; fallback17'),28:('CRASH_GROUND_CYCLE','actor_animation.cpp','Moving crashed rider ground cycle'),29:('CRASH_FAST_GROUND_CYCLE','actor_animation.cpp','Faster ground cycle; repeats first image as phase8'),30:('CRASH_TUMBLE_A','actor_animation.cpp','Random low-speed crash cycle'),31:('CRASH_TUMBLE_B','actor_animation.cpp','Random low-speed crash cycle'),32:('CRASH_TUMBLE_C','actor_animation.cpp','Random low-speed crash cycle'),33:('CRASH_AIRBORNE_ENTRY','crash.cpp','Default begin_crash entry'),34:('CRASH_GROUND_TRANSITION','actor_animation.cpp','Airborne entry reaches low height'),35:('CRASH_COMBAT_ENTRY','crash.cpp','Begin crash reason4 or5'),36:('GET_UP','actor_animation.cpp','Stopped crash selects36 then queues37'),37:('RECOVERY_WAIT','crash.cpp','Wait/loops before running; also damaged bike context'),38:('RUN_TO_BIKE','crash.cpp','Recovery state38 moves rider toward bike / on-foot input'),39:('MOUNT_ALIAS_39','actor_descriptors.cpp','Mount-like descriptor alias; specific caller unresolved'),40:('MOUNT_ALIAS_MIRRORED_40','actor_descriptors.cpp','Mirrored descriptor alias; specific caller unresolved'),41:('REMOUNT','crash.cpp','Recovery arrival selects41, queues ride2'),42:('MOUNT_ALIAS_MIRRORED_42','actor_descriptors.cpp','Mirrored descriptor alias; specific caller unresolved'),43:('BIKE_SEQUENCE_43_EMPTY_ART','actor_descriptors.cpp','Sequence resolves to empty HIBOB slots314-318; caller unresolved'),44:('BIKE_SLIDE_FORWARD','actor_animation.cpp','Moving crash bike queues44'),45:('BIKE_SLIDE_REVERSE_ALIAS','actor_descriptors.cpp','Reverse-order wreck images; specific caller unresolved'),46:('BIKE_FALL','actor_animation.cpp','Crash bike randomly selects46/47'),47:('BIKE_FALL_MIRRORED','actor_animation.cpp','Crash bike randomly selects46/47'),48:('BIKE_REST','actor_animation.cpp','Zero-speed crash bike selects48'),49:('BIKE_REST_MIRRORED_ALIAS','actor_descriptors.cpp','Rest image alias; specific caller unresolved'),50:('UNAVAILABLE_BSS_CODE_50','actor_descriptors.cpp','Unfilled relocation; fallback22'),51:('UNAVAILABLE_BSS_CODE_51','actor_descriptors.cpp','Unfilled relocation; fallback16')}
VISUAL={41:'upright acceleration stance',42:'both arms raised near shoulder height',43:'both arms raised higher',44:'one-arm bent-elbow gesture',45:'one-arm high gesture',47:'low-speed foot-down stance',63:'mounted flinch / loss of balance',81:'raised-chain preparation',82:'circular chain windup',83:'extended chain sweep',84:'chain recovery backswing',85:'over-head chain recovery',86:'backhand chain draw',87:'backhand overhead sweep',88:'cross-body chain follow-through',89:'raised-chain recovery',140:'overhead club windup',141:'forward club swing',142:'extended club swing',143:'backhand club chamber',176:'draw-in / forehand preparation',177:'cross-body punch chamber',178:'extended punch',179:'backhand punch preparation',225:'side-kick leg lift / chamber',226:'extended side kick',282:'supine with one leg raised',283:'seated hand-to-head recovery',284:'supported half-kneel',285:'rising crouch',286:'standing after get-up',289:'standing wait with hand near helmet',297:'run-cycle phase 1 of 6',298:'run-cycle phase 2 of 6',299:'run-cycle phase 3 of 6',300:'run-cycle phase 4 of 6',301:'run-cycle phase 5 of 6',302:'run-cycle phase 6 of 6',309:'beside fallen bike / begin lift',310:'lift bike toward upright',311:'prepare leg-over while holding bike',312:'leg-over / mount transfer',313:'settle onto bike'}
def catalog(data):
    attacks={}
    for a in data['attack_table']:
        code=a['animation_code'];name=a['weapon'].upper()+'_'+a['side'].upper()
        if a['weapon']!='kick':name+='_'+a['variant'].upper()
        attacks.setdefault(code,{'name':name,'selectors':[]})['selectors'].append(a)
    actions=[]
    for bank,rows in enumerate(data['banks']):
        for s in rows:
            code=s['code'];attack=attacks.get(code)
            if attack:name=attack['name'];file='race_setup.cpp';context='Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick'
            else:name,file,context=USES[code]
            if bank:name='POLICE_'+name
            phases=[]
            for p in s['phases']:
                label=f'{name}_{p["phase"]:02d}'
                # Descriptor combat bits are evidence; frame-index gate is evaluated
                # after advance, so it is not mislabeled as an exact contact image.
                phases.append(dict(p,label=label,role='combat-flagged phase' if p['combat_mask'] else 'ordered phase',review='needs pose reconstruction',source_evidence=SOURCE+file,visual_description=VISUAL.get(p['rear']['slot'],'ordered source pose; inspect silhouette') if bank==0 else 'police-specific source pose; separate graphics bank',visual_description_basis='qualitative local sprite review, separate from decoded selector/phase evidence'))
            actions.append({'id':name,'bank':'police' if bank else 'rider','code':code,'available':s['available'],'fallback_code':s['fallback'],'selectors':attack['selectors'] if attack else [],'context':context,'semantic_confidence':'unresolved caller' if 'unresolved' in context.lower() else 'source selector + executable data','source_evidence':SOURCE+file,'phases':phases})
    usage=[{'slot':i,'uses':[]} for i in range(326)]
    for a in actions:
        if a['bank']!='rider':continue
        for p in a['phases']:
            for slot in sorted({v['slot'] for v in p['variants']}):usage[slot]['uses'].append({'action':a['id'],'phase':p['phase'],'label':p['label'],'rear_slot':p['rear']['slot'],'camera_variant':slot!=p['rear']['slot']})
    original=json.loads((ROOT/'docs/animation/reference-slots.json').read_text())['slots']
    for row,meta in zip(usage,original):
        row.update({k:meta[k] for k in ['width','height','empty','duplicate_of']})
        row['status']='referenced' if row['uses'] else 'unused-populated' if not row['empty'] else 'empty-unused'
    police_meta=json.loads((ROOT/'docs/animation/police-slots.json').read_text())['slots']
    police_usage=[{'slot':i,'uses':[],'empty':police_meta[i]['empty']} for i in range(45)]
    for action in actions:
        if action['bank']=='police':
            for phase in action['phases']:
                for slot in sorted({v['slot'] for v in phase['variants']}):police_usage[slot]['uses'].append({'label':phase['label'],'phase':phase['phase'],'action':action['id']})
    return {'schema_version':2,'supersedes':'v1 slot-based speculative pose labels; do not use v1 labels as semantic evidence','source_commit':data['source_commit'],'retail_hashes':data['retail_hashes'],'contains_media':False,'timing_unit':'reference logical ticks, CANS delay_raw divided by20; not a Bevy playback rate','hit_gate_caution':'fight_hit_frame checks retained descriptor flags OR post-advance frame index2; no guessed exact contact instant','actions':actions,'rider_slot_usage':usage,'police_slot_usage':police_usage}
def main():
    out=catalog(json.loads((BASE/'playback.json').read_text()));(BASE/'catalog.json').write_text(json.dumps(out,indent=2)+'\n')
    lines=['# Context-first original animation catalog','','**This supersedes the speculative v1 action labels.** Sequence phases are numbered inside actions. Camera cells and mirrored sprite reuse are recorded separately. Native playback control and timing are retained; pose art remains unapproved.','','| Action | Code | Ordered rear-view slots (M = mirrored) | Logical durations | End control | Source use |','| --- | --- | --- | --- | --- | --- |']
    for a in out['actions']:
        if a['bank']=='police':continue
        p=a['phases'];slots=' → '.join(str(x['rear']['slot'])+('M' if x['rear']['mirror_x'] else '') for x in p) or 'unavailable; fallback '+str(a['fallback_code'])
        lines.append(f'| {a["id"]} | {a["code"]} | {slots} | '+', '.join(str(x['duration_ticks']) for x in p)+' | '+str(p[-1]['control_action'] if p else '—')+' | '+a['context']+' |')
    lines+=['','Phase labels append `_01`, `_02`, etc. A repeated slot remains a separate phase when its timing, flags or control differ. End action0 advances,1 loops to its high-nibble phase when no queued animation exists,2 hands off to queued animation. Fallbacks are separate from sequence identity.','','Police uses the same selector namespace with a separate sparse graphics bank; unavailable police actions are not falsely presented as authored police poses. Source URLs and every cell/phase/mirror relationship are in catalog.json. Unreferenced and empty sprite slots remain visible in the slot usage ledger.']
    (BASE/'CATALOG.md').write_text('\n'.join(lines)+'\n');print(len(out['actions']),'action slots;',sum(len(a['phases']) for a in out['actions']),'phases')
if __name__=='__main__':main()
