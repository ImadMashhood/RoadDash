# Hard-braking skid feedback

A fast, hard brake application on asphalt leaves a narrow rear-tire mark
and brief smoke. The effect blends brake input, forward speed, and tire
slip. The dynamic bike uses the rear suspension's existing ground contact
point and normal, so the mark follows grades and banked pavement. The
assisted bike uses the route surface and hop state. Loose ground uses the
existing surface effects.

The pool holds 192 segments and 16 smoke puffs. Low effects quality halves
both budgets, and `ROADDASH_MAP_FX=0` disables the effect. Smoke rises and
fades after release, and rubber marks last until the pool reuses their slots
or the race restarts. The effect uses RoadDash's original soft particle
texture and reads physical contacts after the fixed simulation.

## Native comparison

The [matched Hills capture](brake-comparison.jpg) held the normal brake
input from the same starting speed. Both runs reached 39.96 m/s at the same
station. The default-quality frame counted 35 rubber segments and six
visible puffs. The effects-off frame counted zero of each.
[Capture values](captures.json).

The focused tests covered hard and normal braking, reverse, contact
continuity, smoke expiration, and clearing marks on restart. Formatting,
strict workspace Clippy, the locked builds, roadmap tests, and the asset
audit passed. The first workspace run reproduced the two known physical
impact and Metropolis support failures.
The native Hills smoke test completed two remounts and reached results.
