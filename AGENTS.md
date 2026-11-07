# RoadDash engineering instructions

The user's active request sets implementation scope. Read README.md,
docs/PROGRESS.md, docs/DECISIONS.md, docs/KNOWN_ISSUES.md and the relevant
blueprint sections before editing. Inspect code and lockfile; proposed versions
in the blueprint are not installed dependencies.

- Implement working integrated changes and verify regressions. Preserve saves,
  controls, physics, audio, recovery and existing content.
- Pure domain rules belong in rr_core, authoring validation in rr_content.
  Keep rendering/input separate from 60 Hz fixed simulation. Visual interpolation
  must never overwrite authoritative body transforms.
- Capture input before the fixed loop; preserve button edges across zero-tick
  render frames and consume each once. Never spawn meshes late in PostUpdate.
- Pin compatible dependencies. Record decisions before migrations; do not import
  unlicensed reference code or proprietary media. Research public source is not
  permission to copy it.
- Update the README checklist and docs/PROGRESS.md together. Check only verified
  requirements; keep partial work and unmet exit gates open. Never delete tasks
  to make a milestone appear complete.
- New requirements need stable IDs, reason, part, dependencies, acceptance and
  verification. Keep future parts deferred unless they block the active scope.
- Record investigated repositories, assets/licenses, architectural decisions,
  known issues, test evidence and chronological development history.
- Use a focused branch and Conventional Commits. Never rewrite unrelated history.
- Run cargo fmt --all --check; cargo clippy --workspace --locked --all-targets --
  -D warnings; cargo test --workspace --locked; cargo build --locked. Run
  python3 tools/audit.py and native smoke tests when relevant and available.
- Do not claim unrun tests, unmeasured performance, legal clearance, Windows/GPU
  hardware verification, pushed commits or merged PRs. Explain outstanding gates.
- Final report: implementation, checklist, files, research, new task IDs, tests,
  decisions, known issues, next steps, and git status. Do not automatically start
  the next major development part.

- Valley structural generation lives in world_gen. Preserve exact quotas/IDs,
  whole-lot rights of way, shared pad support and independent LOD/collision.
  Fade pairs must share local origins; test --inspect-world and --world-survey.
  Allocation counts alone do not prove placed geometry or performance.

- The user explicitly accepts the current ragdoll feel. Keep asset/animation
  integration presentation-only: preserve the eleven body specs, collision,
  masses, joints/motors, damping, momentum and recovery unless the user explicitly
  requests physics tuning. Run src/models/tests.rs differential physics/recovery
  checks. A model's authoring skeleton is not permission to replace the physical rig.

- README is the single live checklist: verified tasks in Part 00, unfinished
  tasks in Parts 01–16; Parts 01–08 v2 precede original Parts 09/10. Stable ID
  prefixes retain historical numbering; current section determines execution order.
  docs/roadmap/BLUEPRINT_PARTS_01_08.md is a frozen snapshot, not a live backlog.
  sync_progress.py indexes README only. Keep TASK_PLACEMENT.json in sync when
  moving tasks, and update README count/summary when completing requirements.
  Never drop history or close new tasks merely because prototype tasks passed.
- The latest user permits targeted pre-impact/crash-response/knockoff corrections
  and evaluation of bounded active recovery. Preserve the accepted player feel
  and passive ragdoll as control fixtures; this is not blanket engine/joint tuning
  approval. Correct visual bind/chain defects in rendering before changing physics.

- The user approved merging biomechanical ragdolls on 2026-11-06. The native
  game and lab now default to the new anatomy/muscles; preserve the eleven-body
  control via --ragdoll-baseline. Run paired headless lab and native recovery/NPC
  regression checks when changing either path. Imported comparison-only/default
  restrictions in older evidence are historical. Keep crash contact policy,
  current world/props, controls, saves and recovery integrated. Production distal
  skinning, broad performance and hardware coverage remain open roadmap gates.

- Latest user follow-up supersedes upright rear-bike hops: share pre-contact
  normal momentum and knock the front rider off at60km/h closing speed. Keep
  downed-body/wreck lift separate. Grounded roll cues spend existing kinetic
  energy once; frontal leg-release uses existing motor limits. Preserve baseline.
  Neutral distal skin frames must inherit animated parents; do not add standing
  wrist twist to hide a bind mismatch. Valley is default; explicit city remains.

- 2026-11-07: user selected velocity muscles after A/B comparison. Do not integrate
  the native-motor candidate. Anatomical constraints/passive resistance are still
  required. Facade LOD changes must crossfade ready groups directly, use opposing
  dither branches and final propagated render-camera origins; overlap counts
  against displayed geometry budgets and cannot remove collision.
