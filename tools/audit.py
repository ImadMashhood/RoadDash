#!/usr/bin/env python3
"""Headless provenance/dependency declaration audit; no downloads of game media.
This verifies declarations, hashes and references, not legal ownership or safety.
"""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
ALLOWED = {"MIT", "MIT-0", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Zlib", "CC0-1.0", "Unlicense", "0BSD", "Unicode-3.0", "OFL-1.1"}

def permitted(expression):
    tokens = re.findall(r"\(|\)|[A-Za-z0-9.+-]+", expression.replace("/", " OR "))
    position = 0
    def term():
        nonlocal position
        if position >= len(tokens): raise ValueError("incomplete expression")
        token=tokens[position]; position+=1
        if token=="(":
            value=disjunction()
            if position>=len(tokens) or tokens[position]!=")": raise ValueError("missing parenthesis")
            position+=1
            return value
        value=token in ALLOWED
        if position<len(tokens) and tokens[position]=="WITH":
            position+=1
            if position>=len(tokens): raise ValueError("missing exception")
            value=value and tokens[position]=="LLVM-exception"; position+=1
        return value
    def conjunction():
        nonlocal position
        value=term()
        while position<len(tokens) and tokens[position]=="AND":
            position+=1; other=term(); value=value and other
        return value
    def disjunction():
        nonlocal position
        value=conjunction()
        while position<len(tokens) and tokens[position]=="OR":
            position+=1; other=conjunction(); value=value or other
        return value
    try:
        value=disjunction()
        return value and position==len(tokens)
    except ValueError: return False

def metadata():
    return json.loads(subprocess.check_output(["cargo","metadata","--locked","--format-version","1"],cwd=ROOT,text=True))

def audit_assets(meta):
    packages={(p["name"],p["version"]):Path(p["manifest_path"]).parent for p in meta["packages"]}
    required=["asset_id","path","creator","source_url","license","license_copy","sha256","modifications","attribution","redistribution","shipped"]
    seen=set(); count=0
    with (ROOT/"docs/third_party/ASSETS.csv").open(newline="") as f:
        rows=list(csv.DictReader(f))
    for row in rows:
        if not all(row.get(k) for k in required): raise ValueError(f"incomplete provenance: {row.get('asset_id')}")
        if row['asset_id'] in seen: raise ValueError("duplicate asset ID")
        seen.add(row['asset_id'])
        if row['shipped'] not in ('true','false'): raise ValueError("invalid shipped flag")
        if row['shipped']=='false': continue
        if not permitted(row['license']) or row['redistribution']!='permitted': raise ValueError(f"uncleared runtime asset {row['asset_id']}")
        if row['path'].startswith('cargo:'):
            package,relative=row['path'][6:].split('/',1); name,version=package.split('@')
            path=packages[(name,version)]/relative
        else: path=ROOT/row['path']
        if hashlib.sha256(path.read_bytes()).hexdigest()!=row['sha256']: raise ValueError(f"asset/source hash mismatch: {row['asset_id']}")
        if not (ROOT/row['license_copy']).is_file(): raise ValueError(f"missing license copy: {row['asset_id']}")
        count+=1
    # New file-based runtime media must enter intake, not bypass it.
    for path in (ROOT/'assets').rglob('*') if (ROOT/'assets').exists() else []:
        if path.is_file() and path.relative_to(ROOT).as_posix() not in {r['path'] for r in rows if r['shipped']=='true'}: raise ValueError(f"unregistered runtime file: {path}")
    return count

def audit_research():
    with (ROOT/'docs/research/OBSERVATIONS.csv').open(newline='') as f: rows=list(csv.DictReader(f))
    required=['source_id','source_url','build_version','observed_behavior','confidence','date','asset_owner','allowed_usage','follow_up']
    if len(rows)<20 or len({r['source_id'] for r in rows})!=len(rows): raise ValueError('need 20 unique source observations')
    if any(not all(r.get(k) for k in required) for r in rows): raise ValueError('incomplete research record')
    return len(rows)

def main():
    parser=argparse.ArgumentParser(); parser.add_argument('--output',type=Path); args=parser.parse_args()
    meta=metadata()
    failures=[f"{p['name']} {p['version']}: {p.get('license')}" for p in meta['packages'] if not permitted(p.get('license') or '')]
    if failures: raise ValueError('Dependency declaration requires review: '+repr(failures))
    package_names={p['id']:p['name'] for p in meta['packages']}
    for node in meta['resolve']['nodes']:
        if package_names[node['id']] in ('bevy_core_pipeline','bevy_pbr') and set(node['features']) & {'tonemapping_luts','bluenoise_texture'}:
            raise ValueError('Embedded third-party rendering media requires explicit provenance before enabling this feature')
    report={'dependency_declarations':len(meta['packages']),'runtime_asset_records':audit_assets(meta),'reference_observations':audit_research(),'limitations':'Declaration/hash audit only; not legal clearance, vulnerability scanning, or platform certification.'}
    print(json.dumps(report,indent=2))
    if args.output: args.output.parent.mkdir(parents=True,exist_ok=True); args.output.write_text(json.dumps(report,indent=2)+'\n')
if __name__=='__main__': main()
