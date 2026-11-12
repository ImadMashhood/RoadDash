# Hills sand, dust, and city crowd follow-up (2026-11-11)

Scope: isolated Bevy 0.20.1 integration candidate. Existing terrain collision, road placement, physics, controls, saves and fixed dust pool remain intact.

- Added original deterministic 64×64 repeated grayscale ground grit and world-space UVs to existing Hills terrain material, retaining existing vertex colour strata and draw batches. No external asset or dependency.
- Dust samples the same authored sand colour at emission, uses a fixed 1.8 m quad, fades in over 0.25 s, fades near expiry, and fades from 9 to 24 m behind the player. Pool remains 96; no meshes are spawned in PostUpdate.
- The imported Hills city walker slice retains every other walker. This interprets "peds pop is too much, half it in the city" as a density request; visible spawn distance remains open for review. Metropolis is unchanged.
- Focused procedural terrain test passed in isolation before integration. Integrated `cargo fmt --all --check`, strict locked all-target Clippy, audit, roadmap tests and progress sync pass. `cargo test --workspace --locked --no-fail-fast` has 467 passing road-dash library tests, two pre-existing Bevy/Rapier parity failures, and one ignored test; other workspace targets pass. Locked debug/release builds and native smoke review are recorded below after completion.

Acceptance remains open for close moving native views, city encounter balance, matched release frame/GPU cost and user assessment of pedestrian onset.

The two unchanged engine parity failures are `biomechanics::tests::wall_impact_and_high_speed_ejection_remain_bounded` (high-speed anchor error 0.166 m over 0.05 m) and `world_gen::metropolis::geometry::tests::standalone_static_chunks_keep_contact_ccd_and_support_trajectories` (static chunk contact divergence). Neither is closed by this content pass.

## Integrated native receipt

`cargo build --locked` and `cargo build --release --locked` pass. The release executable SHA-256 is `b76519348fa7447bc5d464a74fbd4f3372ce63c04cd82571a04316e953981fe1`. That exact binary passed `./play-bevy20.sh --smoke-test` for Metropolis and `./play-bevy20.sh --hills --smoke-test` for Hills, each with two successful remounts and a results fixture. Hills reported zero exceptional rescues. Logs: `/tmp/roaddash-feedback-metro-smoke.log` and `/tmp/roaddash-feedback-hills-smoke.log`.

`--hills-polish-survey` exited successfully with seven captures, 43 emitted dust puffs, 21 maximum active of 96, 11 mounted pairs, 13.04 m measured mount travel and confirmed mounted collision. Its generated receipt is `screenshots/hills-polish.json`; visual samples are under `screenshots/polish-*.png` in the worktree. The scripted survey does not measure full-course art quality or frame/GPU performance.
