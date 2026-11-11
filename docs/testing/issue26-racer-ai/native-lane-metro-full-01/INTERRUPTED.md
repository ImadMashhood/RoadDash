# Incomplete debug full-race attempt

Command: `flock /tmp/roaddash-native.lock python3 tools/run_racer_ai_checks.py --binary /home/imad/Documents/ChatGPT/RoadDash-issue-26-target/debug/road-dash --case race --track metro --frames 30000 --require-complete --label native-lane-metro-full-01`.

Started 2026-11-11 about 05:06 UTC and interrupted with SIGINT after the
survey reached about 53.9 simulated seconds. The full race was not completed;
the runner exited 130 and wrote no completion receipt. Concurrent city,
guardrail and effects compilers slowed rendered-frame progress. The retained
native and performance logs are diagnostics only and must not be used for
frame-time comparison or AI completion claims. A fresh release-binary full-race
case is required.
