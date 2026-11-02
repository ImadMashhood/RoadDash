#!/usr/bin/env python3
"""Record course family loads and named CEL membership; never write retail pixels.
Resource layout researched in pinned arkits/road-rash. The optional external
local decoder supplies decompression only and is not vendored or a dependency.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import struct


def resources(data):
    def word(p):return struct.unpack_from('<I',data,p)[0]
    if len(data)<24 or data[:4]!=b'CRSR' or data[12:16]!=b'LBTR':raise ValueError('Not a resource container')
    table=word(16)
    if table+16>len(data) or data[table:table+4]!=b'LBTR':raise ValueError('Missing directory')
    if word(table+8)>(len(data)-table-16)//32:raise ValueError('Truncated resource directory')
    result={}
    for index in range(word(table+8)):
        p=table+16+index*32;tag=data[p:p+4][::-1].decode();identity,offset,size=struct.unpack_from('<III',data,p+4)
        if offset+size>len(data):raise ValueError('Resource range exceeds file')
        result[tag,identity]=data[offset:offset+size]
    return result


def course_loads(data):
    segments=resources(data)['SGS ',1]
    def word(p):return struct.unpack_from('<I',segments,p)[0]
    if len(segments)<16 or segments[:4]!=b'SGSR':raise ValueError('Missing section records')
    if word(8)>(len(segments)-16)//52:raise ValueError('Truncated section records')
    observed={};sections=[]
    for section in range(word(8)):
        p=word(16+section*52+12);loaded=set()
        if p:
            if p+24>len(segments) or segments[p:p+4]!=b'MSRR':raise ValueError('Invalid family chunk')
            size=word(p+4);count=word(p+8)
            if size<24 or p+size>len(segments) or count>(size-24)//12:raise ValueError('Truncated family events')
            for k in range(word(p+8)):
                row=p+24+k*12;kind=segments[row+4];family=word(row+8)&0xffff
                if kind in [1,3]:loaded.add(family);observed[family]=observed.get(family,0)+1
        sections.append(dict(section=section,loaded_family_ids=sorted(loaded)))
    return dict(section_count=len(sections),loaded_family_ids=sorted(observed),sections=sections)


def measure(root,decoder,catalog):
    spec=importlib.util.spec_from_file_location('local_disc_decoder',decoder);module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    ids={d['id']for d in json.loads(catalog.read_text())['designs']};members={identity:set()for identity in ids};courses={}
    labels=dict(CITY='The City',HIWAY='Sierra Nevada',MEDLY='The Peninsula',NAPA='Napa Valley',CANYN='Pacific Coast Highway')
    for code,label in labels.items():
        path=root/'DATA/COURSES'/(code+'.CRS');data=path.read_bytes();courses[code]=dict(label=label,sha256=hashlib.sha256(data).hexdigest(),**course_loads(data))
    file=root/'DATA/FAMILIES.RSC';raw=file.read_bytes();families=resources(raw);magic=struct.pack('<I',0xbc614e)
    for (tag,identity),packed in sorted(families.items()):
        if tag!='FAM ' or len(packed)<=24:continue
        data=module.unpack(packed);cursor=0
        while True:
            p=data.find(magic,cursor)
            if p<0:break
            cursor=p+4
            if p+40>len(data):continue
            name=data[p+16:p+32].split(b'\0')[0].decode('ascii',errors='ignore')
            width,height=struct.unpack_from('<II',data,p+32)
            if name in ids and 0<width<=512 and 0<height<=512:members[name].add(identity)
    associations=[]
    for identity in sorted(ids):
        family_ids=sorted(members[identity]);course_codes=[code for code,c in courses.items()if set(family_ids)&set(c['loaded_family_ids'])]
        associations.append(dict(id=identity,family_ids=family_ids,course_family_load_matches=course_codes,
            evidence='named CEL inside loaded family; eligibility only, not proof this exact texture was selected or placed' if family_ids else 'no matching named CEL found; district assignment requires interpretation'))
    return dict(schema_version=1,source='User-local 1997 rerelease; numeric/text metadata only',reference_commit='aa9cb6d3181e20736af71022cd4d6aac0e2788d1',families_sha256=hashlib.sha256(raw).hexdigest(),decoder_sha256=hashlib.sha256(decoder.read_bytes()).hexdigest(),courses=courses,assets=associations,
        limitation='Family residency is not texture placement. Section definitions are not a measured route order. District labels and kit roles are RoadDash art direction, not original named districts.')


if __name__=='__main__':
    ap=argparse.ArgumentParser();ap.add_argument('disc',type=Path);ap.add_argument('--decoder',type=Path,required=True);ap.add_argument('--catalog',type=Path,default=Path('art/buildings/mesh_assets/catalog_designs.json'));ap.add_argument('--output',type=Path,default=Path('docs/research/building-districts/source-associations.json'));a=ap.parse_args();result=measure(a.disc,a.decoder,a.catalog);a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(dict(ids=len(result['assets']),matched=sum(bool(r['family_ids'])for r in result['assets']),courses={code:len(c['loaded_family_ids'])for code,c in result['courses'].items()})))
