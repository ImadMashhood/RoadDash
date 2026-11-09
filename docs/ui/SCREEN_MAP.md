# Original screen and popup index

All36 stored screen slots are included;29 have stored items. Slots0–6 are bootstrap/race transitions, not seven extra menus. Dynamic handlers mutate several pages; this is not proof of every runtime state.

| ID | Original role | Stored controls | RoadDash draft |
| --- | --- | ---: | --- |
| RR-00 | Reserved / bootstrap | 0 | transition / reference only |
| RR-01 | Race: City | 0 | transition / reference only |
| RR-02 | Race: Sierra Nevada | 0 | transition / reference only |
| RR-03 | Race: Peninsula | 0 | transition / reference only |
| RR-04 | Race: Napa Valley | 0 | transition / reference only |
| RR-05 | Race: Pacific Coast | 0 | transition / reference only |
| RR-06 | Transition / unused | 0 | transition / reference only |
| RR-07 | Opening credits | 1 | credits |
| RR-08 | Race loading | 1 | loading |
| RR-09 | Main menu | 4 | splash, main |
| RR-0A | Thrash course selection | 6 | courses |
| RR-0B | Mano-a-mano setup | 6 | network |
| RR-0C | Mano-a-mano courses | 6 | courses, network |
| RR-0D | Network chat presets | 18 | network |
| RR-0E | Character selection | 9 | riders |
| RR-0F | Street / career hub | 3 | hub |
| RR-10 | Restroom / options hub | 13 | settings, audio, saves |
| RR-11 | Jukebox | 9 | jukebox |
| RR-12 | Configure input | 22 | controls |
| RR-13 | Graphics options | 8 | display |
| RR-14 | Olley’s bike shop | 7 | garage |
| RR-15 | Rat bike showroom | 6 | garage |
| RR-16 | Sport bike showroom | 6 | garage |
| RR-17 | Super bike showroom | 6 | garage |
| RR-18 | Bike detail / purchase | 7 | bike |
| RR-19 | Der Panzer Klub | 7 | hangout |
| RR-1A | Rider conversation | 2 | dialogue |
| RR-1B | Big Game course selection | 6 | courses |
| RR-1C | Wrecked result | 1 | wrecked |
| RR-1D | Busted result | 1 | busted |
| RR-1E | Won / qualified result | 1 | won |
| RR-1F | Lost result | 1 | lost |
| RR-20 | Level advancement | 1 | level |
| RR-21 | Penalty / game-over message | 1 | penalty |
| RR-22 | Race standings | 15 | results |
| RR-23 | Final credits | 1 | credits |

## Original stored popups

- RR-POP-01: mode (4 rows).
- RR-POP-02: level (6 rows).
- RR-POP-03: name (2 rows).
- RR-POP-04: resolution (4 rows).
- RR-POP-05: biker-list (3 rows).
- RR-POP-06: videos (4 rows).
- RR-POP-07: jukebox-video (3 rows).
- RR-POP-08: jukebox-tracks-1 (3 rows).
- RR-POP-09: jukebox-tracks-2 (2 rows).
- RR-POP-10: jukebox-tracks-3 (4 rows).
- RR-POP-11: jukebox-tracks-4 (5 rows).
- RR-POP-12: jukebox-tracks-5 (3 rows).
- RR-POP-13: jukebox-tracks-6 (3 rows).

## Dynamic UI with no dedicated image file

- Save filename list/name entry and overwrite / unsaved / failed-load messages.
- Purchase affordability, character/bike stats and contextual disabled entries.
- Fifteen-row results, penalty/game-over text, level and qualification state.
- Relationship-selected F/N/H portrait and conversation text.
- Countdown, announcements, needle and stamina/condition meters.
- Original Sound Mix opens native Windows dialog300, separately cataloged; modern fullscreen audio settings are a new proposal.

## Reconstruction additions, not retail screens

IDs0x24–0x2B add display, sound, pause, gamepad, touch, enhancements, online lobby and room browser in the inspected reconstruction. Those are not counted as original retail menu screens. RoadDash settings/pause/network proposals are explicitly marked in plan.json.
