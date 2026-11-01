# City proportions — 2026-10-31

Read the user's local 1997 rerelease disc; all 341 importer hashes previously
matched. This session measures `DATA/COURSES/CITY.CRS` only; no disc data/media or
reference implementation is shipped. Reproducible metadata-only reader:
`python3 tools/measure_reference_city.py /local/path/to/CITY.CRS`.

Reference-only source inspected at commit
`4e052aca9552a5690c2b145b0f53856d5c31e980`:
[course.cpp](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/course.cpp),
[road.cpp](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/road.cpp),
[course_scene.cpp](https://github.com/arkits/road-rash/blob/4e052aca9552a5690c2b145b0f53856d5c31e980/src/course_scene.cpp),
and resource/terrain readers. Course lane units convert to 250 original world
units per lane. Building facades are projected at the outer shoulder edge,
without the extra gravel-plus-setback bands our generator previously introduced.

The supplied City's 15 section definitions contain 10 building-terrain and 5
hill-terrain profiles. Across interval endpoints, the most common side width is
2 lanes (199 entries); shoulder widths 224 and 250 occur 100 and 56 times.
These are endpoint counts, **not a distance-weighted route census**. Full metadata
and SHA-256 are in ../testing/city-combat/reference-dimensions.json.

RoadDash keeps its 13 m main roadway, equivalent to four 3.25 m lanes. At that
scale 224/250 original shoulder widths correspond to 2.912/3.25 m. Its 3 m city
sidewalk is therefore appropriate; the extra 4.5 m gravel strip before it was
not. City sidewalks now start at the asphalt edge. Inner-city/core front lots
bring their building face to 2.5 m beyond the sidewalk, retaining walking/pad
clearance for physical 3D play. This is an explicit safety adaptation to the
reference's facade-at-shoulder arrangement, not a claim of pixel or SI parity.
Additional streets form a road-relative grid around the curved main corridor;
the race gate sequence remains the only race route.

The user's seven-pose image is a visual reference, not a numeric angle table.
Symmetric bank poses interpolate through -37.24/-24.83/-12.41/0/12.41/24.83/37.24
degrees from steering force only. These are authored 3D angles approximating the
image, not angles recovered from proprietary sprite assets. Original handling
calibration P03-020 remains separate and open.

2026-10-31 frontage correction (CITY-02): the earlier change retained detached
18 m frontages on a 24 m pitch. That six-metre gap did not satisfy the user's
original-game reference. The already inspected course_scene building panels form
continuous facade strips; RoadDash now joins neighboring building end faces with
solid full-depth infill. Street and junction openings remain deliberate. No new
reference repository or source snapshot was imported for this correction.
