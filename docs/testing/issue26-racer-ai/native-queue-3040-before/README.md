# Station 3,040 queue diagnostic (before refuge change)

This is a bounded, unteleported release-native Metropolis `--racer-ai-survey` from
`99b697d3` plus trace-only instrumentation. The binary SHA-256 was
`5e11b2dbacd50039db0f8cd55ebcca8359a1c54e890bb2f6690c2ec2fbe8e571`.
Run from the worktree with `ROADDASH_AI_SURVEY_FRAMES=7500
ROADDASH_AI_TRACE_3040=1 /home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash
--racer-ai-survey`. The run
exited 0 at 7,500 frames / 121.99 simulated seconds, before race completion.
See [native.txt](native.txt) for actual planner proxies and static sweep hits and
[racer-ai.json](racer-ai.json) for the full sampled decision receipt.

The front queue rider (13) entered station 3,010 at 25.37 m/s, street lane
-0.288, while a main-road oncoming car was 46.5 m ahead in lane +1.5 at
-15 m/s. At station 3,030 that car was 10.77 m ahead and the rider's avoidance
plan remained zero. There was no static sweep rejection in these samples.
After the car passed, the rider remained near stations 3,054 and 3,075 while
fast rear cars/riders occupied the narrow outbound merge and new oncoming cars
approached. The 3 m outbound and 3 m inbound section leaves outbound planning
bounds of about `[-2.35, -0.65]` in the street frame. At sampled elapsed times
80.65 and 99.42 seconds, replaying the recorded proxy envelopes against the
pure planner found no safe outbound candidate at cruise or staged walking pace;
the rear merge was occupied. No static hits were recorded.

Rider 1 later slowed near station 3,030 behind a stopped rider roughly 12 m
ahead and an oncoming car roughly 12 m ahead; its own static sweep rejections
were empty. Rider 11 had not reached the traced station window by frame 7,500.
The survey therefore identifies this run's obstacle class and queue mechanism,
but does not replay the older 33-second rider 1/11 stop or establish that the
road was passable during each stopped tick.

Across all riders, the 3,000–3,100 m window contains 20 sampled zero-speed
`Riding` points whose avoidance plan also requests zero, 10 non-riding samples,
and 30 zero-plan samples. The longest consecutive riding stop was five samples
for rider 9, four for rider 13, and three for rider 8. Samples are approximately
three simulated seconds apart; these are sample counts, not continuous durations.
