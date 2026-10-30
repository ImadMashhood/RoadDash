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
