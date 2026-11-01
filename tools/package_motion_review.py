#!/usr/bin/env python3
"""Package original Blender review renders; no retail media input."""
from pathlib import Path
import json
from PIL import Image, ImageDraw
ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'art/animation/motion-review'
RENDERS = Path.home() / 'Downloads/RoadDash_Animation_Interpolation_v1/renders'
clips = json.loads((BASE / 'clips.json').read_text())
sheet = Image.new('RGB', (1000, 225 * len(clips)), '#1b222a')
draw = ImageDraw.Draw(sheet)
for row, clip in enumerate(clips):
    frames = [Image.open(p).convert('RGB') for p in sorted((RENDERS / clip['name']).glob('*.png'))]
    assert len(frames) == clip['frames'], clip['name']
    frames[0].save(BASE / (clip['name'] + '.gif'), save_all=True,
                   append_images=frames[1:], duration=83, loop=0, optimize=True)
    draw.text((8, row * 225 + 3), clip['name'] + ' | authoring preview', fill='white')
    for j, index in enumerate([0, len(frames)//3, 2*len(frames)//3, len(frames)-1]):
        sheet.paste(frames[index].resize((220, 200)), (j*245, row*225+24))
sheet.save(BASE / 'overview.jpg', quality=90)
print('Packaged', len(clips), 'original animation previews')
