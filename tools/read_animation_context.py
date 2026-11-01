#!/usr/bin/env python3
"""Independent bounded metadata reader; never exports retail pixel/audio payloads.

PE addresses are explicit observed build-layout facts in reference-layout.json.
This tool records temporal descriptor order separately from CANS camera cells.
No C++ source from the research repository is included or required at runtime.
"""
import argparse,hashlib,json,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
class Bytes:
    def __init__(self,data):self.data=data
    def take(self,p,n):
        if p<0 or n<0 or p+n>len(self.data):raise ValueError('Truncated metadata range')
        return self.data[p:p+n]
    def u32(self,p):return struct.unpack('<I',self.take(p,4))[0]
    def i32(self,p):return struct.unpack('<i',self.take(p,4))[0]
    def u16(self,p):return struct.unpack('<H',self.take(p,2))[0]
def pe_data(exe,va,n):
    b=Bytes(exe)
    if b.take(0,2)!=b'MZ':raise ValueError('Not PE')
    pe=b.u32(60)
    if b.take(pe,4)!=b'PE\0\0' or b.u16(pe+4)!=0x14c or b.u16(pe+24)!=0x10b:raise ValueError('Not PE32 i386')
    rva=va-b.u32(pe+52);start=pe+24+b.u16(pe+20)
    for p in range(start,start+b.u16(pe+6)*40,40):
        origin,size,offset=b.u32(p+12),b.u32(p+16),b.u32(p+20)
        if origin<=rva<origin+size:
            if rva+n>origin+size:raise ValueError('PE cross-section range')
            return b.take(offset+rva-origin,n)
    raise ValueError('Unmapped PE address')
def resources(data):
    b=Bytes(data)
    if b.take(0,4)!=b'CRSR' or b.take(12,4)!=b'LBTR':raise ValueError('Not CRSR')
    table=b.u32(16)
    if b.take(table,4)!=b'LBTR':raise ValueError('Invalid resource table')
    n=b.u32(table+8)
    if n>len(data)//32:raise ValueError('Invalid resource count')
    rows={}
    for p in range(table+16,table+16+n*32,32):
        key=(b.take(p,4)[::-1].decode('ascii'),b.u32(p+4));payload=b.take(b.u32(p+8),b.u32(p+12))
        if key in rows:raise ValueError('Duplicate resource')
        rows[key]=payload
    return rows
def cans_groups(data):
    b=Bytes(data);header=b.u32(4);kind=b.u32(8)
    if b.u32(0)!=0x43414e53 or kind not in (1,2,3,4):raise ValueError('Not SNAC')
    count=b.u32(44 if kind==1 else 12)
    if count>65536:raise ValueError('Invalid group count')
    rows=[None]*count;p=48 if kind==1 else 16
    while p<header:
        id,n,delay=b.u32(p),b.u32(p+4),b.u32(p+8)
        if not 1<=id<=count or not n or n>65536 or rows[id-1] is not None:raise ValueError('Invalid CANS group')
        cells=[]
        for q in range(p+16,p+16+n*28,28):
            slot=(b.u32(q)>>16)-1
            if slot<0:raise ValueError('Invalid one-based graphic reference')
            cells.append({'slot':slot})
        rows[id-1]={'group':id-1,'delay_raw':delay,'cells':cells};p+=16+n*28
    if p!=header or any(r is None for r in rows):raise ValueError('Incomplete CANS groups')
    flags=(0 if kind==1 else header);flags+=b.u32(flags+4)+12
    for row in rows:
        words=b.u32(flags)
        if words<len(row['cells']) or words>65536:raise ValueError('Incomplete cell flags')
        for j,cell in enumerate(row['cells']):cell['flags']=b.u32(flags+4+j*4)
        flags+=4+words*4
    table=header
    for _ in range(3):table+=b.u32(table+4)
    table+=12
    for j,row in enumerate(rows):row['semantic_code']=b.u32(table+j*16)
    return rows
def sequence(raw,groups):
    if not raw or len(raw)%8:raise ValueError('Invalid phase descriptors')
    result=[];b=Bytes(raw)
    for p in range(0,len(raw),8):
        code=b.i32(p);offset,control=raw[p+4:p+6]
        matches=[g for g in groups if g['semantic_code']==abs(code)]
        if not matches:return []
        # The neutral angular family has neither front/back family bit set.
        middle=[g for g in matches if not g['cells'][0]['flags']&6]
        if not middle:middle=matches # Preserve unresolved cases; oracle comparison is separate.
        group=middle[len(middle)//2] if middle[0]['cells'][0]['flags']&0xf800 else middle[0]
        flags=group['cells'][0]['flags'];angular=bool(flags&0xf800);yaw=bool(flags&0x7e0)
        index=offset+(len(group['cells'])//2 if angular or yaw else 0)
        if index>=len(group['cells']):raise ValueError('Phase cell outside group')
        cell=group['cells'][index]
        variants=[]
        for g in matches:
            choices=g['cells'] if angular or yaw else g['cells'][offset:offset+1]
            variants.extend({'group':g['group'],'slot':c['slot'],'mirror_x':bool(c['flags']&1),'flags':c['flags']} for c in choices)
        result.append({'phase':p//8+1,'semantic_code':code,'source_offset':offset,'control':control,'control_action':control&15,'loop_phase':(control>>4)+1 if control&15==1 else None,'duration_ticks':group['delay_raw']//20,'delay_raw':group['delay_raw'],'rear':{'slot':cell['slot'],'mirror_x':bool(cell['flags']&1),'group':group['group'],'cell':index,'flags':cell['flags']},'descriptor_flags':middle[0]['cells'][0]['flags'],'combat_mask':middle[0]['cells'][0]['flags']&0x600000,'variants':variants})
    return result

def inspect(root):
    layout=json.loads((ROOT/'docs/animation/context/reference-layout.json').read_text());exe=(root/'ROADRASH.EXE').read_bytes();global_=(root/'DATA/GLOBAL.RSC').read_bytes();
    if {'exe':hashlib.sha256(exe).hexdigest(),'global':hashlib.sha256(global_).hexdigest()}!=layout['retail_hashes']:raise ValueError('Unsupported retail build; layout must be re-verified before decoding')
    res=resources(global_);groups=[cans_groups(res['CANS',b]) for b in (1,2)]
    relocated={row['destination_va']:sequence(pe_data(exe,row['source_va'],row['count']*8),groups[row['bank']]) for row in layout['bindings']}
    pointers=Bytes(pe_data(exe,layout['animation_bank_pointer_va'],8));fallback=Bytes(pe_data(exe,layout['fallback_va'],52*4));banks=[]
    for bank in range(2):
        table=Bytes(pe_data(exe,pointers.u32(bank*4),52*4));rows=[]
        for code in range(52):
            ptr=table.u32(code*4);rows.append({'code':code,'fallback':fallback.i32(code*4),'pointer_va':ptr,'available':bool(relocated.get(ptr)),'phases':relocated.get(ptr,[])})
        banks.append(rows)
    attacks=[]
    for side in ('left','right'):
        b=Bytes(pe_data(exe,layout[side+'_attack_va'],32))
        for kind in range(2):
            for weapon,name in enumerate(('punch','club','chain','kick')):attacks.append({'side':side,'variant':'backhand' if kind else 'forehand','weapon':name,'animation_code':b.i32((kind*4+weapon)*4)})
    return {'schema_version':1,'source_commit':layout['source_commit'],'retail_hashes':{'exe':hashlib.sha256(exe).hexdigest(),'global':hashlib.sha256(global_).hexdigest()},'contains_media':False,'banks':banks,'groups':groups,'attack_table':attacks,'steal_table':list(struct.unpack('<8i',pe_data(exe,layout['steal_va'],32)))}
if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('retail_root',type=Path);parser.add_argument('--output',type=Path,required=True);args=parser.parse_args();data=inspect(args.retail_root);args.output.write_text(json.dumps(data,indent=2)+'\n');print(sum(len(s['phases']) for b in data['banks'] for s in b),'temporal descriptors; media excluded')
