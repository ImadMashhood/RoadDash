# District and original-course asset sets

All 146 modeled IDs are organized into 13 primary district/support sets.
Each model belongs to exactly one primary set. Shared services, infrastructure
and trim are separate so they are not mistaken for urban facade buildings.

These district labels are **RoadDash art direction**, informed by the original
asset names, architecture and measured family membership. They are not claims
that the original game contained districts with these names or this layout.

| Set | Assets | Intended use |
|---|---:|---|
| Commercial high street | 9 | BUSS storefront rows |
| Office centre | 16 | OFFC business blocks |
| Chinatown | 7 | Six CHINA fronts and CHINGAT2 gateway |
| Victorian residential | 31 | VICT residential rows |
| Small-town main street | 11 | CLSTGA town frontages |
| Run-down quarter | 7 | RUNDWN worn/boarded urban blocks |
| Industrial / warehouses | 20 | WHSE and WRHS service/warehouse blocks |
| Farms / agriculture | 6 | Barns, grain silos and hay shelter |
| Rural village / winery | 6 | Homes, church, school and winery |
| Roadside services | 9 | Shops, stations, bus stop and phone shelter |
| Utilities / service yards | 5 | Tanks, sanitation and ancillary facilities |
| Retaining walls / tunnels | 15 | Engineered wall/tunnel modules and tower |
| Shared roof / edge modules | 4 | TOP trim modules |

## Original-game associations

Read the user's local 1997 disc, with the pinned arkits resource/course readers
as format research. Matched every ID against named CEL records in decompressed
FAMILIES.RSC and compared membership to MSRR family-load requests (types 1/3)
in all section definitions of the five course files. No game pixels were written.

| Course data | Familiar location label | Assets in loaded families |
|---|---|---:|
| CITY | The City | 90 |
| HIWAY | Sierra Nevada | 24 |
| MEDLY | The Peninsula | 73 |
| NAPA | Napa Valley | 27 |
| CANYN | Pacific Coast Highway | 40 |

Membership overlaps: these counts must not be added as unique assets. The CHINA
fronts/gateway match CITY only; the TUN/TUNN modules match CANYN only. Other
families/services can appear in multiple course-family pools. Exact matches
and family IDs are stored per asset, not guessed from district labels.

**A family being loaded does not prove that every texture in it was selected
or visibly placed.** This investigation does not reconstruct scenery selectors,
level-dependent route order, spawn frequency or exact building coordinates.
Course eligibility indexes are research filters, not faithful recreated tracks.

## Use and files

- `districts.json`: complete district membership, original family/course evidence,
  canonical GLB paths and unchanged output hashes. All paths resolve from here.
- `asset_assignments.csv`: sortable one-row-per-ID inventory.
- Each district folder: manifest with paths relative to that manifest, labeled
  contact sheet of all members and `models/` links to all three original GLB LODs.
  Meshes are not copied or modified. Unix links remain within this asset kit.
- `original_courses/`: five eligibility manifests/contact sheets; these are
  many-to-many views of the same canonical models.
- `district-overview.png`: representative examples, not every model.

Generation: `python3 tools/group_building_districts.py`; consistency check:
`python3 tools/group_building_districts.py --check`. Uses the checked-in numeric
research evidence; normal generation does not need the disc or external decoder.
To remeasure, run tools/measure_reference_districts.py with the local disc and
an explicit local decompression module; see research evidence for hashes.

The index/preview bundle overlays the existing complete-model ZIP directory tree.
It does not re-download or duplicate the meshes. On systems without Unix symlink
support, use manifest/CSV canonical paths; running the grouping script to create
links requires symlink-capable permissions.

No current city-preview selection or racing/world_gen placement is changed.
Next: use the appropriate district pool per reserved block/site, with authored
transitions and existing road/pad/collision/navigation constraints. That remains
within V2-04-003 and the broader world-generation work.

Verification: ../../../../docs/testing/building-districts/README.md.
