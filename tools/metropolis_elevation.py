"""Shared authored C2 route/land height profile; preserves all plan footprints."""
import json, math
from pathlib import Path
PROFILE = json.loads((Path(__file__).resolve().parents[1]/'content/world/metropolis_elevation.json').read_text())
def height(x, z, profile=PROFILE):
    knots=profile['knots']
    y=knots[0][1] if z<=knots[0][0] else knots[-1][1]
    for (a,lo),(b,hi) in zip(knots,knots[1:]):
        if a<=z<=b:
            t=(z-a)/(b-a)
            blend=t*t*t*(10+t*(-15+6*t))
            y=lo+(hi-lo)*blend
            break
    return y+profile['crossfall_amplitude']*math.sin(x/profile['crossfall_wavelength'])
