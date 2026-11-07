#!/usr/bin/env python3
"""Package actual native comparison screenshots; no generated poses or retail media."""
from pathlib import Path
from PIL import Image, ImageDraw
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'docs/testing/ragdoll'
OUT.mkdir(parents=True, exist_ok=True)
for mode in ('biomechanical', 'baseline'):
    paths = sorted((ROOT / 'screenshots' / f'ragdoll-{mode}-frames').glob('*.png'))
    assert len(paths) == 41, (mode, len(paths))
    frames=[]
    for p in paths:
        crop=Image.open(p).convert('RGB').crop((360,120,940,800)).resize((435,510))
        frame=Image.new('RGB',(435,540),'#151e28');frame.paste(crop,(0,30))
        ImageDraw.Draw(frame).text((8,8),f'NATIVE | {mode} | {int(p.stem)/60:.2f} s',fill='white')
        frames.append(frame)
    frames[0].save(OUT / f'{mode}.gif', save_all=True, append_images=frames[1:],
                   duration=100, loop=0, optimize=True)
    for stage in ('airborne', 'impact', 'tumble', 'late_settling'):
        Image.open(ROOT / 'screenshots' / f'ragdoll-{mode}-{stage}.png').convert('RGB').save(
            OUT / f'{mode}-{stage}.jpg', quality=88)
sheet = Image.new('RGB', (1280, 860), '#151e28')
draw = ImageDraw.Draw(sheet)
for column, mode in enumerate(('baseline', 'biomechanical')):
    draw.text((column*640+16, 10), mode.upper() + ' | native captures', fill='white')
    for row, stage in enumerate(('impact', 'late_settling')):
        shot = Image.open(OUT / f'{mode}-{stage}.jpg').resize((640, 400))
        sheet.paste(shot, (column*640, row*420+30))
sheet.save(OUT / 'comparison.jpg', quality=90)
for name in ('03-crash', '04-getting-up', '06-remounting'):
    p = Path('/tmp/roaddash-biomechanical-native') / (name+'.png')
    Image.open(p).convert('RGB').save(OUT / (name+'.jpg'), quality=88)
print('Packaged two real native GIFs, comparison and recovery captures')
