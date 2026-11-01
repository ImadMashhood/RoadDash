#!/usr/bin/env python3
"""Inventory local sprite slots as metadata only; never emit proprietary pixels.

Independent format reader: two dimension bytes followed by indexed pixels.
All inspection images remain outside the repository. Frame numbers are not
animation timing: empty/view/distance/duplicate slots are preserved explicitly.
"""
import argparse
import hashlib
import json
from pathlib import Path


def inventory(path, expected):
    data = Path(path).read_bytes()
    cursor = 0
    slots = []
    first = {}
    for index in range(expected):
        if cursor + 2 > len(data):
            raise ValueError('Truncated frame header')
        width, height = data[cursor:cursor + 2]
        offset = cursor
        cursor += 2
        extent = width * height
        if cursor + extent > len(data):
            raise ValueError('Truncated frame payload')
        pixels = data[cursor:cursor + extent]
        cursor += extent
        digest = hashlib.sha256(bytes((width, height)) + pixels).hexdigest()
        empty = not extent or not any(pixels)
        duplicate = first.get(digest) if not empty else None
        if not empty:
            first.setdefault(digest, index)
        slots.append(dict(slot=index, width=width, height=height, offset=offset,
                          empty=empty, duplicate_of=duplicate, sha256=digest))
    if cursor != len(data):
        raise ValueError(f'Unexpected trailing bytes: {len(data)-cursor}')
    return dict(schema_version=1, file_sha256=hashlib.sha256(data).hexdigest(),
                slot_count=expected, populated=sum(not s['empty'] for s in slots),
                empty=sum(s['empty'] for s in slots), slots=slots,
                contains_media=False, timings='not encoded in this sprite archive')


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('path', type=Path)
    p.add_argument('--slots', type=int, default=326)
    p.add_argument('--output', type=Path, required=True)
    args = p.parse_args()
    args.output.write_text(json.dumps(inventory(args.path, args.slots), indent=2)+'\n')
