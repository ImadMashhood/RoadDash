# Original Road Rash rider AI: source inspection (2026-11-11)

This is read-only research. No RoadDash rider behavior was changed from these sources, and no reference code or media was copied.

## Evidence and scope

- The [1991 Genesis manual](https://segaretro.org/images/9/99/Road_Rash_MD_US_Manual.pdf) describes rival riders, personalities and combat, but does not document a catch-up formula.
- [arkits/road-rash](https://github.com/arkits/road-rash) is a reverse-engineered port of the **1996 Windows release**. Its behavior should not be assumed to describe the 1991 Genesis game.
- The port reads a 20-entry speed-modifier table from the original executable in [bike.cpp](https://github.com/arkits/road-rash/blob/main/src/bike.cpp). Its [race planning logic](https://github.com/arkits/road-rash/blob/main/src/race.cpp) applies a gap-dependent modifier to a rider's *desired cruising speed* when the player is the selected neighbor. The trailing modifier can be as high as about 15%. This is rubber banding of the target pace, not a direct teleport or hard velocity assignment.
- The same planning code adjusts target pace for curves, considers lateral passing, and uses throttle/coast/brake control to approach that pace. The project documents incomplete understanding of parts of its lookahead behavior.

## Conclusion

The inspected 1996 Windows implementation supports a qualified **yes**: AI riders can receive gap-dependent desired-speed assistance relative to the player. We have **not confirmed** the algorithm for the 1991 Genesis game, nor independently executed and instrumented the original executable. The reported modifier is one factor among route planning, passing, contact and control, so it cannot by itself explain RoadDash's current launch behavior. Do not import the table or implementation; any RoadDash balancing should be independently designed and verified against its own simulation.
