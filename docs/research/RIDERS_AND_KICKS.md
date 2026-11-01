# Rider, kick and pursuit reference — 2026-10-31

Inspected arkits/road-rash commit 4e052aca9552a5690c2b145b0f53856d5c31e980
already present at /tmp/roadrash-handling-source. REFERENCE ONLY; no source copied.
The same verified user-supplied 1997 disc was inspected locally, without shipping
its executable, sprite sheets, remap tables, audio or language resources.

Sources:
- [race.cpp](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/race.cpp):
  original_opponents accepts 14 opponents; combat shove adds 3000 steering units
  to a landed victim, plus 8000 for a kick, with steering-limit clamps. Pursuit
  uses target position/lateral/speed, offsets alongside, bounded speed changes.
- [race_setup.cpp](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/race_setup.cpp):
  names use personality indices and localized string slots; course-specific
  spare names and rare alternate sets exist.
- bob.cpp, rider_sprites.cpp and pe_strings.cpp describe remap and localization
  metadata. Color table VA 0x46e158 has 15 remaps. PALETTE.RAW is 256 four-byte
  entries, not three-byte RGB.

The standard named opponents resolve to Mike, Cydney, Jim, Pearl, Jon, Rhonda,
Bose and Axle. CITY spare names are Eric, Buzz, Druid, Hog, Serge and Slash.
RoadDash selects that stable CITY roster, with YOU as slot zero. This is not a
claim that every original course/seed/character uses this exact lineup.
Level-zero opponent bike table indices in slots 1–14:
1,2,4,4,1,1,2,1,4,1,4,1,4,1.
Bike performance replication is outside this color-scheme request.

Local visual inspection of all fifteen remapped upright sprites informed
independently selected RGB swatches for shirts, helmets and bike paint.
Player: pale shirt/blue helmet/red bike; opponents include gold/purple,
gold/black, blue/green, magenta/purple, green/green, white/white, pale-blue/blue,
brown/brown, black/black, lavender/purple, gold/green, cyan/cyan, red/red,
gold/purple combinations. Trousers follow bike paint and boots remain dark.
Existing original 3D geometry is shared across seated, walking and ragdoll states.
The local reference preview remains outside the repository.

Original steering words are not metres or a fixed kick travel distance. RoadDash
uses an independent 8 m/s lateral shove, up from 3, with existing 2.5/s decay.
Its unconstrained integral is approximately 3.2 m; actual travel depends on tire
forces, steering, contacts and terrain. This is an arcade adaptation, not measured
binary displacement parity. The user subsequently requested +50% turn response,
less auto-steering, fallen bikes and deliberate AI pursuit. These use existing
physics and pure deterministic rr_core decisions, not imported reference code.
