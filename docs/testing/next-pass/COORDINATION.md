# Fresh-agent pass coordination

User explicitly requested one fresh contextless GPT-6 Astra medium agent per
change. Each task is spawned with fork_turns=none, model=gpt-6-astra and
reasoning_effort=medium. Three worker slots run in waves; parent reviews and
integrates isolated Conventional Commits. No prior chat transcript is passed.

Tasks: town hall; biker outfit restoration; lighting; race bounds/gates; city
entry lanes; queue overtaking; texture fidelity; persistent crops; elevation;
police uniform/bike. Each has a distinct initial agent and worktree. No task is
considered complete merely because a worker reported success.

All source starts from verified ef3eecc0 or a reviewed descendant. Parent updates
shared roadmap/provenance/history. Native captures/performance and release builds
are serialized; assets/binary hashes and process guards are retained. Original
physical rigs, controls, saves and legacy maps remain regression fixtures.

Accepted interpretations: marks persist for current race and reset on retry;
approximately 500m off each race end preserves full physical map; no visible
checkpoint gates. The width-reduction map from the previous pass remains only
a proposal and is not part of this implementation request.

## Worker handoffs and final integration

Worker-specific pending notes below describe their handoff state; the final parent
suite listed at the end resolved those code/native gates. Broader acceptance stays
open as recorded in README.md.

- Lighting: agent lighting, commit c03c01e3 → parent f83cfbf7. Seven sky tests,
  release, matched profile surveys/recovery pass; parent strict suite pending.
- Original outfit: agent original_outfit,58c19a84 → parent ebc70d73. Exact source
  restore of models.rs/tests.rs from6039be4e; style.rs removed. Runtime checks
  intentionally deferred to parent; no grass or physical-rig reversal.
- Town hall: agent town_hall,3fc90993 → parent579ff70e. Five meshes/19,764 added
  triangles; geometry and11model tests, release and iterated five-view captures.
- Race bounds: race_bounds,8e4c18a3 → parent9ceae5c2. 500..11500m playable;
  full 12km map preserved; source checks only, integrated runtime gates pending.
- City lanes: city_lanes,8bc7ac2c → parentd6547ec2. Seven Python tests,
  30,120 whole-car footprints and17,828 paint quads pass. Rust/native deferred.
- Textures: texture_fidelity,2cdb5afe → parentb3f14b48. Original civic maps/UVs;
  Rust/native checks deferred to parent.
- Crops: persistent_crops,2c13f49e → parentd7b219f5. Seven focused tests, strict
  Clippy/release/native off-on fixtures pass; parent inspected visible trails.
- Elevation: moderate_elevation,a285dd72 → parentf2469d89. Five authoring tests,
  height-only preservation and actual triangle support checks pass; native pending.
- Queue passing: queue_passing,eb6c3c35/bbfaf7f5 → parent0a507de9/728f7760.
  Fourteen planner tests; physical fixture passed before last refinement, final
  parent suite must rerun it. No shared player physics change.
- Police: police_art,9df8a8d6 → parente96d80c4. Twelve model and10police
  tests, native gallery and both-rig police smoke pass; newest screenshot hook
  awaits parentnative. All ten tasks had fresh agents and are now integrated.
- Parent owns the build/native slot for final clean-workspace combined checks.
- Parent authoring:23Metropolis tests,5roadmap tests and full lane audit passed.
- No cargo during another agent's native captures; coordinate expensive Python
  authoring too. /tmp is near capacity; frozen binaries belong on /home outside git.
  Shared target had stale rr_core metadata across worktrees; force clean/rebuild
  local workspace packages before final integrated checks, not all dependencies.
- Parent completed provenance/shared records,481-test strict suite,28Python tests,
 12release native cases and18paired lab cases; inspected Hall, original/cop models,
 entry lanes and persistent/reset crops. README.md is the final integrated report.
