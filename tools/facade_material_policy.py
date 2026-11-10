"""Original offline finish policy for the 24 currently placed Metropolis families.
Apply to a fresh copy of authored source materials, never to runtime instances.
The named textures, shade variants, geometry and alpha modes remain authored.
"""
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FAMILIES = set(json.loads((ROOT/'content/world/metropolis_facades.json').read_text())['families'])


def coordinate(identity, material):
    result = copy.deepcopy(material)
    if identity not in FAMILIES:
        return result
    name = result['name']
    base = name
    for suffix in ('_masonry_tile', '_dark', '_light'):
        base = base.removesuffix(suffix)
    pbr = result['pbrMetallicRoughness']
    # Painted metal is a dielectric coating, like masonry/plaster/concrete.
    # Keep original hues and shade variants. Slightly quieter masonry makes
    # the existing pale entry surrounds legible without new geometry/overlays.
    pbr['metallicFactor'] = .08 if 'metal' in base else 0.0
    factor = list(pbr.get('baseColorFactor', [1, 1, 1, 1]))
    if 'baseColorTexture' in pbr:
        if 'brick' in base or base in ('weathered_cream', 'blue_plaster', 'green_plaster'):
            factor[:3] = [v*.94 for v in factor[:3]]
        elif base == 'concrete':
            factor[:3] = [v*t for v,t in zip(factor[:3], [.98,.99,1.0])]
    if base == 'glass':
        factor = [.028, .060, .078, factor[3]]
        pbr['roughnessFactor'] = .28
    elif base == 'interior':
        factor = [.009, .013, .016, factor[3]]
        pbr['roughnessFactor'] = .90
    elif base == 'gold':
        pbr['metallicFactor'] = .72
        pbr['roughnessFactor'] = .38
    pbr['baseColorFactor'] = factor
    return result
