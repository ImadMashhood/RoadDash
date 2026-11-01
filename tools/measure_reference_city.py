#!/usr/bin/env python3
"""Measure local CITY.CRS metadata; never extract or redistribute game media.

Format research: arkits/road-rash 4e052aca, course/resource readers.
The counters describe stored interval endpoints, not distance-weighted samples.
"""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import struct


def measure(path):
    data = path.read_bytes()

    def word(offset):
        return struct.unpack_from('<I', data, offset)[0]

    if data[:4] != b'CRSR' or data[12:16] != b'LBTR':
        raise ValueError('Not a Road Rash resource container')
    table = word(16)
    if data[table:table + 4] != b'LBTR':
        raise ValueError('Missing resource directory')
    resources = {}
    for index in range(word(table + 8)):
        entry = table + 16 + index * 32
        name = data[entry:entry + 4][::-1].decode('ascii').strip()
        offset, size = word(entry + 8), word(entry + 12)
        if offset + size > len(data):
            raise ValueError('Resource exceeds file')
        resources[name] = offset
    base = resources['SGS']
    if data[base:base + 4] != b'SGSR':
        raise ValueError('Missing section records')
    lanes, shoulders, terrain = Counter(), Counter(), Counter()
    for index in range(word(base + 8)):
        row = base + 16 + index * 52
        for offset, tag, stride, counter in [
            (0x18, b'NALR', 20, lanes), (0x14, b'DLSR', 36, shoulders)
        ]:
            relative = word(row + offset)
            if not relative:
                continue
            chunk = base + relative
            if data[chunk:chunk + 4] != tag:
                raise ValueError(f'Unexpected interval tag at {chunk}')
            for record in range(word(chunk + 8)):
                for field in (8, 9, 10, 11):
                    counter[data[chunk + 16 + record * stride + field]] += 1
        relative = word(row + 0x10)
        if relative:
            terrain[data[base + relative:base + relative + 4].decode('ascii')] += 1
    return {
        'sha256': hashlib.sha256(data).hexdigest(),
        'section_count': word(base + 8),
        'lane_endpoint_units_250_per_lane': dict(lanes),
        'shoulder_endpoint_world_units': dict(shoulders),
        'terrain_types': dict(terrain),
    }


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('city_crs', type=Path)
    args = parser.parse_args()
    print(json.dumps(measure(args.city_crs), indent=2))
