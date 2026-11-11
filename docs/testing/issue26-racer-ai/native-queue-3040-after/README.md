# Station 3,040 queue diagnostic (outbound refuge variant)

This is the matched 7,500-frame unteleported Metropolis `--racer-ai-survey`
with the pure outbound-refuge change and the same opt-in trace used in the
[before run](../native-queue-3040-before/README.md). The release executable
SHA-256 was `acf89936c262591766e0831d338d21a14acf2254ab39e324645d2b40192da678`,
verified against `/proc/1407120/exe` while running from this worktree. Source
was `99b697d3` plus the uncommitted refuge and trace edits; no later main
integration is claimed by this receipt. It exited 0 at 7,500 frames / 121.99
simulated seconds before race completion. [Native log](native.txt) and
[sampled decisions](racer-ai.json) are retained.

At its first station-3,010 trace, the former queue-head rider 13 was in the
outbound lane at -1.43 m rather than -0.29 m in the before run. At the survey
samples it crossed stations 3,025.8 and 3,077.2 at 32.4 and 7.9 m/s, both
Riding; there was no sampled zero-speed stop in this 100 m window. Before, it
had 16 window samples and four consecutive stopped Riding samples near 3,054
and 3,075. Its timeline changed because the AI policy changed; these are
matched commands and source, not an exact replay of every traffic interaction.

Across all riders in the station 3,000–3,100 window, sampled `Riding`
zero-speed points with a zero avoidance plan fell from 20 to 14, non-riding
samples from 10 to 6, and zero-plan samples from 30 to 24. Riders 1 and 11
still each have three consecutive zero-speed Riding samples, roughly nine
sampled seconds. A full race, human balance, and quiet performance approval
remain open. Sample spacing is about three simulated seconds and does not
measure continuous stop duration or prove no contact between samples.
