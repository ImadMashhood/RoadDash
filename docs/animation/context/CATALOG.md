# Context-first original animation catalog

**This supersedes the speculative v1 action labels.** Sequence phases are numbered inside actions. Camera cells and mirrored sprite reuse are recorded separately. Native playback control and timing are retained; pose art remains unapproved.

| Action | Code | Ordered rear-view slots (M = mirrored) | Logical durations | End control | Source use |
| --- | --- | --- | --- | --- | --- |
| RIDE_FAR | 0 | 22 | 3 | 1 | NPC camera distance below second cutoff |
| RIDE_VERY_FAR | 1 | 23 | 3 | 1 | NPC distant camera selector |
| RIDE_NORMAL | 2 | 3 | 21 | 1 | Normal mounted ride |
| RIDE_FAST_TUCK | 3 | 252 | 21 | 1 | High speed / lateral-speed condition |
| RIDE_ACCELERATE | 4 | 41 | 15 | 1 | Low speed plus throttle >30 |
| RIDE_WRECK_WARNING | 5 | 42 → 43 | 15, 15 | 1 | Player wrecked flag and wreck_count <4 |
| AI_POST_HIT_GESTURE | 6 | 44 → 45 | 15, 15 | 1 | NPC probabilistic queued animation after hit |
| UNRESOLVED_RIDE_CODE_07 | 7 | 46 | 9 | 1 | Present descriptor; no confirmed call site |
| RIDE_LOW_SPEED | 8 | 47 | 15 | 1 | Speed <300 |
| MOUNTED_HIT_REACTION | 9 | 63 → 63 | 9, 9 | 2 | Nonfatal reason-4 hit selects code9 |
| CHAIN_LEFT_FOREHAND | 10 | 81M → 82 → 83 → 84 → 85 | 7, 7, 7, 7, 7 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CHAIN_RIGHT_FOREHAND | 11 | 81M → 82M → 83M → 84M → 85M | 7, 7, 7, 7, 7 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CHAIN_LEFT_BACKHAND | 12 | 85 → 86 → 87 → 88 → 89 | 7, 7, 7, 7, 7 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CHAIN_RIGHT_BACKHAND | 13 | 85M → 86M → 87M → 88M → 89M | 7, 7, 7, 7, 7 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CHAIN_STEAL_TABLE_INDEX_2 | 14 | 83M → 81M | 7, 7 | 2 | Steal table index2, acquired chain |
| CHAIN_STEAL_TABLE_INDEX_6 | 15 | 83 → 85 | 7, 7 | 2 | Steal table index6, acquired chain |
| CLUB_LEFT_FOREHAND | 16 | 140 → 141 → 142 | 11, 7, 11 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CLUB_RIGHT_FOREHAND | 17 | 140M → 141M → 142M | 11, 7, 11 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CLUB_LEFT_BACKHAND | 18 | 143 → 142 → 142 | 11, 11, 11 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CLUB_RIGHT_BACKHAND | 19 | 143M → 142M → 142M | 11, 11, 11 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CLUB_STEAL_UNAVAILABLE_LEFT | 20 | unavailable; fallback 16 |  | — | Steal table selects absent sequence; fallback16 |
| CLUB_STEAL_UNAVAILABLE_RIGHT | 21 | unavailable; fallback 17 |  | — | Steal table selects absent sequence; fallback17 |
| PUNCH_LEFT_FOREHAND | 22 | 176 → 177 → 178 | 7, 6, 7 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| PUNCH_RIGHT_FOREHAND | 23 | 176M → 177M → 178M | 7, 6, 7 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| PUNCH_LEFT_BACKHAND | 24 | 179 → 178 → 178 | 7, 7, 7 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| PUNCH_RIGHT_BACKHAND | 25 | 179M → 178M → 178M | 7, 7, 7 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| KICK_LEFT | 26 | 225 → 226 | 12, 12 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| KICK_RIGHT | 27 | 225M → 226M | 12, 12 | 2 | Attack table [side][forehand/backhand][weapon]; weapon0 fist,1 club,2 chain,3 kick |
| CRASH_GROUND_CYCLE | 28 | 241M → 242M → 243M → 244M → 245M → 246M → 247M | 6, 6, 6, 6, 6, 6, 6 | 1 | Moving crashed rider ground cycle |
| CRASH_FAST_GROUND_CYCLE | 29 | 241M → 242M → 243M → 244M → 245M → 246M → 247M → 241M | 2, 4, 4, 4, 4, 4, 4, 2 | 1 | Faster ground cycle; repeats first image as phase8 |
| CRASH_TUMBLE_A | 30 | 256M → 257M → 258M → 259M | 7, 7, 7, 7 | 1 | Random low-speed crash cycle |
| CRASH_TUMBLE_B | 31 | 260M → 261M → 262M → 263M | 7, 7, 7, 7 | 1 | Random low-speed crash cycle |
| CRASH_TUMBLE_C | 32 | 264M → 265M → 266M → 267M | 7, 7, 7, 7 | 1 | Random low-speed crash cycle |
| CRASH_AIRBORNE_ENTRY | 33 | 268M → 269M → 270M → 271M → 272M | 6, 6, 6, 6, 6 | 1 | Default begin_crash entry |
| CRASH_GROUND_TRANSITION | 34 | 273M → 274M → 275M → 276M | 6, 6, 6, 6 | 2 | Airborne entry reaches low height |
| CRASH_COMBAT_ENTRY | 35 | 276M → 274M | 8, 8 | 2 | Begin crash reason4 or5 |
| GET_UP | 36 | 282 → 283 → 284 → 285 → 286 | 30, 15, 15, 15, 15 | 2 | Stopped crash selects36 then queues37 |
| RECOVERY_WAIT | 37 | 289 → 289 → 289 → 289 | 4, 4, 4, 4 | 1 | Wait/loops before running; also damaged bike context |
| RUN_TO_BIKE | 38 | 297 → 298 → 299 → 300 → 301 → 302 | 6, 6, 6, 6, 6, 6 | 1 | Recovery state38 moves rider toward bike / on-foot input |
| MOUNT_ALIAS_39 | 39 | 309 → 310 → 311 → 312 → 313 | 16, 16, 16, 16, 16 | 2 | Mount-like descriptor alias; specific caller unresolved |
| MOUNT_ALIAS_MIRRORED_40 | 40 | 309M → 310M → 311M → 312M → 313M | 16, 16, 16, 16, 16 | 2 | Mirrored descriptor alias; specific caller unresolved |
| REMOUNT | 41 | 309 → 310 → 311 → 312 → 313 | 16, 16, 16, 16, 16 | 2 | Recovery arrival selects41, queues ride2 |
| MOUNT_ALIAS_MIRRORED_42 | 42 | 309M → 310M → 311M → 312M → 313M | 16, 16, 16, 16, 16 | 2 | Mirrored descriptor alias; specific caller unresolved |
| BIKE_SEQUENCE_43_EMPTY_ART | 43 | 314M → 315M → 316M → 317M → 318M | 10, 10, 10, 10, 10 | 1 | Sequence resolves to empty HIBOB slots314-318; caller unresolved |
| BIKE_SLIDE_FORWARD | 44 | 319 → 320 → 321 → 322 | 10, 10, 10, 10 | 1 | Moving crash bike queues44 |
| BIKE_SLIDE_REVERSE_ALIAS | 45 | 322 → 321 → 320 → 319 | 10, 10, 10, 10 | 1 | Reverse-order wreck images; specific caller unresolved |
| BIKE_FALL | 46 | 319 → 319 → 319 | 15, 15, 15 | 2 | Crash bike randomly selects46/47 |
| BIKE_FALL_MIRRORED | 47 | 319M → 319M → 319M | 15, 15, 15 | 2 | Crash bike randomly selects46/47 |
| BIKE_REST | 48 | 321 | 5 | 1 | Zero-speed crash bike selects48 |
| BIKE_REST_MIRRORED_ALIAS | 49 | 321M | 5 | 1 | Rest image alias; specific caller unresolved |
| UNAVAILABLE_BSS_CODE_50 | 50 | unavailable; fallback 22 |  | — | Unfilled relocation; fallback22 |
| UNAVAILABLE_BSS_CODE_51 | 51 | unavailable; fallback 16 |  | — | Unfilled relocation; fallback16 |

Phase labels append `_01`, `_02`, etc. A repeated slot remains a separate phase when its timing, flags or control differ. End action0 advances,1 loops to its high-nibble phase when no queued animation exists,2 hands off to queued animation. Fallbacks are separate from sequence identity.

Police uses the same selector namespace with a separate sparse graphics bank; unavailable police actions are not falsely presented as authored police poses. Source URLs and every cell/phase/mirror relationship are in catalog.json. Unreferenced and empty sprite slots remain visible in the slot usage ledger.
