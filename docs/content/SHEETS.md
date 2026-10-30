# Content sheets and authoring contract

Part 00 definitions; initial runtime subset in ../../content/catalog.json.
All example values are original prototype tuning. No original game stats/art are
reproduced. Proposed future definitions are not falsely represented as loaded.

| Sheet | Stable fields / units | Present vs planned |
| --- | --- | --- |
| Bike | id, class, mass_kg, torque/RPM/gear curve, top/nitro speed_mps, acceleration/brake/coast_mps2, grip, durability, visual ID | Runtime speed/acceleration/braking plus two cosmetic variants; dynamic tire fields P03 |
| Cosmetic | id, display name, color_hex, physics bike ID, source/rights ID | Runtime mint/ember colors; no implicit physics modifiers |
| Rider | id, display name, bike_id, palette_index, style weights, outfit IDs, rights ID | Eight validated runtime references/palettes; styles/outfits P08/P11 |
| Weapon | id, side, startup/active/recovery_ms, range_m, stamina_cost, damage, stagger, impulse, pickup rules | Schema design P05; current punches remain prototype logic |
| Track | id, centerline control points_m, widths_m, bank/elevation, surfaces, ordered gates, sectors, zones, spawns | Stable graybox ID only; authored format P02 |
| Shop | item_id, bike/cosmetic/upgrade reference, price_credits, repair rule, unlock condition | P12; no purchasable items today |
| Progression | schema_version, profile ID, owned IDs, credits, unlocks, class/track records | Versioned prototype aggregate profile; complete economy P12 |
| UI | screen/action IDs, labels, accessibility alternatives, bindings, comfort options | AppScreen lifecycle and validated action bindings; persistence/settings P04/P15 |
| Asset | path or generated source, author, source URL, exact license/copy/hash, edits, attribution, redistribution, shipped | docs/third_party/ASSETS.csv is the intake register |

Contracts: lowercase ASCII string IDs (1-80 characters; digits, hyphens and
underscores permitted), finite positive tuning, valid references, explicit schema
version and separated cosmetic/physics identities. rr_content rejects unknown
fields, duplicate IDs, invalid colors, missing bikes, unsupported schemas and
non-finite tuning. It currently requires eight prototype riders; roster expansion
is a tracked P08 authoring task, not an unrestricted loader claim.

Research sheets keep source_id, source_url, build/version, observed_behavior,
confidence, date, asset_owner, allowed_usage and follow_up separately from new
our_id/stats/visuals/unlocks. QA records validator version, last tested, failures.
