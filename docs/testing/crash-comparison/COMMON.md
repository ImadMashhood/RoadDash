# Shared crash comparison lifecycle

Both candidate branches share physical combat stagger intent, continuous mounted
release and a finite0.65s exception only for the released rider and its own wreck.
Anatomical/self contacts and all unrelated solids remain enabled. Foot re-knockdown
retains the prior safe placement path. The accepted eleven-body baseline bypasses
new automatic stagger/continuous release behavior.

`cargo run --release -- --crash-comparison`: F8 starts the combat sequence, F9
starts a frontal release, F10 cycles and sets20/60/120/200km/h while mounted/racing.
Input edges are captured before fixed simulation and consumed once. Race menu/
pause/on-foot states discard these commands. Debug speed setting is explicit;
it is never applied automatically to normal riding. Experiment mode disables
profile writes. The branch/method is identified on the overlay. No candidate is
merged by this comparison; tune/visual acceptance remain user-review gates.
