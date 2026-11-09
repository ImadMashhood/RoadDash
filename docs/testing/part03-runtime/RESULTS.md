# Part03 executed results — 2026-11-08

| Command / evidence | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed. |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | Passed; [log](clippy.txt). |
| `cargo test --workspace --locked` |331passed, one existing ten-minute soak ignored; [log](workspace-tests.txt). Includes models physical body/joint/180-tick trajectory and crash/run/remount parity. |
| `cargo build --locked` | Passed; [log](build.txt). |
| `cargo build --release --locked` | Passed; [log](release-build.txt). |
| `python3 tools/audit.py` | Passed523dependency declarations,87runtime records,27reference observations; [JSON](audit.json). Declaration/hash audit, not legal clearance. |
| `python3 tools/test_roadmap.py` |5passed; [log](roadmap-tests.txt). |
| `python3 tools/sync_progress.py --check` | Passed; [log](progress-check.txt).205verified/243open unchanged; all27 Part03 tasks remain open. |
| `git diff --check` | Passed. |
| `cargo test --locked combat::tests::buffered_strikes -- --nocapture` |16real-input action/rate combinations passed, first render observation.216667s at30Hz and.200000s at60/120/144Hz, one hit each; [log](contact-timing.txt). Actual Animator extension pose checked. |
| `cargo test --locked animation::grounding::tests::swing_replants -- --nocapture` | Real Rapier support/replant/transition/removal/steep regression passed; [log](grounding.txt). |
| `python3 tools/report_animation_coverage.py --check` | Passed; [log](coverage-check.txt). Runtime body assets unchanged. |
| `python3 tools/test_animation_context.py` |7passed; [log](context-tests.txt). |
| `python3 tools/check_authoring_hands.py` | Current static authoring provenance passed; [log](authoring-provenance-check.txt). |
| `python3 tools/package_hand_review.py` then `--check` | Fresh current-release33-view native recapture packaged; all source/capture hashes pass; [log](hand-provenance-check.txt), [gallery](../hand-contacts/gallery/index.html). |
| `target/release/road-dash --ragdoll-lab --output /tmp/part03-ragdoll-lab.json` | All18paired baseline/default cases finite; [JSON](ragdoll-lab.json), [log](ragdoll-lab.txt). Headless wall times are not native performance. |

Native commands used `DISPLAY=:0` on the supplied Linux/Vulkan RTX3080 host:

```sh
ROADDASH_TRACK=metro ROADDASH_VERIFY_REPAIRS=1 ROADDASH_VERIFY_REVERSE=1 ROADDASH_VERIFY_STRIKES=1 target/release/road-dash --smoke-test
ROADDASH_TRACK=metro ROADDASH_SMOKE_ROUTE=1 ROADDASH_VERIFY_REPAIRS=1 ROADDASH_VERIFY_REVERSE=1 ROADDASH_VERIFY_STRIKES=1 target/release/road-dash --smoke-test
ROADDASH_TRACK=metro ROADDASH_VERIFY_REPAIRS=1 ROADDASH_VERIFY_REVERSE=1 target/release/road-dash --ragdoll-baseline --smoke-test
target/release/road-dash --animation-lab-view --animation-lab-smoke
target/release/road-dash --animation-lab-view --hand-review --animation-lab-smoke
```

Both default routes passed actual keyboard windup.016667s and contact.200000s
(threshold.199444s), fifteen-racer grid retry, audio output, reverse−1.827m/s,
farm/city recovery, two remount errors0.0000m and results fixture. The eleven-body
control also passed the scripted retry/reverse/two-recovery/result sequence.
[Route0](native-route0.txt), [route1](native-route1.txt),
[baseline](native-baseline-route0.txt), [route0metrics](native-route0-metrics.json),
[route1metrics](native-route1-metrics.json). The initial conflicting native fixture
failed as documented in the [main receipt](README.md); its log is retained
separately and is not counted as a passing run.

The final current-release animation lab passed eleven dynamic motions/three
views; [log](native-animation.txt). The independent held-pose hand lab passed
open/grip/fist and eleven motions/three views; [log](native-hands.txt). Actual
native contact/hand and running-side captures were visually inspected. The
keyboard strike capture has no guaranteed nearby hit target; headless geometry/
contact tests establish the hit path. The lab fallback/held-pose paths cannot
stand in for that scheduler test. No human fight/terrain/feel acceptance is claimed.

[Windup](strike-windup.png), [contact](strike-contact.png),
[dynamic punch rear](lab-punch-left-rear.png), [dynamic kick side](lab-kick-left-side.png),
[dynamic running side](lab-running-side.png). Source/binary/image hashes and
limitations are in [provenance](PROVENANCE.json). Existing unmapped-gamepad and
Vulkan DRM-extension warnings remain; neither caused the scripted checks to fail.
No Windows/physical-controller, production performance or legal certification.
