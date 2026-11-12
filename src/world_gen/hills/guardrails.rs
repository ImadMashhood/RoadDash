//! Original solid turn-side barriers, grounded independently of the rail profile.
pub mod survey;
use super::{Art, Census, HillsSolid, Track, bridge, geometry};
use crate::world::Obstacle;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rr_core::guardrails::{self as policy, Run};

#[derive(Component)]
pub struct Guardrail;
#[derive(Component)]
pub struct RailPost;
#[derive(Component)]
pub struct RailTerminal;
#[derive(Resource, Default)]
pub struct RailRuns(pub Vec<Run>);

/// Actual resident shoulder triangles, captured before surface upload. Bounding
/// boxes avoid testing the entire route for each support at generation time.
#[derive(Default)]
pub(crate) struct GroundSamples(Vec<SurfaceTriangles>);
struct SurfaceTriangles {
    min: Vec2,
    max: Vec2,
    triangles: Vec<[Vec3; 3]>,
}
impl GroundSamples {
    pub(crate) fn add(&mut self, vertices: &[Vec3], indices: &[[u32; 3]]) {
        let min = vertices
            .iter()
            .fold(Vec2::splat(f32::INFINITY), |a, p| a.min(p.xz()));
        let max = vertices
            .iter()
            .fold(Vec2::splat(f32::NEG_INFINITY), |a, p| a.max(p.xz()));
        self.0.push(SurfaceTriangles {
            min,
            max,
            triangles: indices
                .iter()
                .map(|tri| tri.map(|i| vertices[i as usize]))
                .collect(),
        });
    }
    fn height(&self, track: &Track, p: Vec3) -> f32 {
        let mut height = if track.is_hills() {
            geometry::ground_height(track, p.x, p.z)
        } else {
            f32::NEG_INFINITY
        };
        for SurfaceTriangles {
            min,
            max,
            triangles,
        } in &self.0
        {
            if p.xz().cmplt(*min).any() || p.xz().cmpgt(*max).any() {
                continue;
            }
            for &[a, b, c] in triangles {
                let ab = b.xz() - a.xz();
                let ac = c.xz() - a.xz();
                let ap = p.xz() - a.xz();
                let determinant = ab.perp_dot(ac);
                if determinant.abs() < 0.00001 {
                    continue;
                }
                let u = ap.perp_dot(ac) / determinant;
                let v = ab.perp_dot(ap) / determinant;
                if u >= -0.0001 && v >= -0.0001 && u + v <= 1.0001 {
                    height = height.max(a.y + u * (b.y - a.y) + v * (c.y - a.y));
                }
            }
        }
        height
    }
}
pub fn side(track: &Track, station: f32) -> Option<i8> {
    if !(1580. ..5380.).contains(&station) || bridge::on_deck(station) {
        return None;
    }
    signed_side(track, station)
}
fn signed_side(track: &Track, station: f32) -> Option<i8> {
    let a = track.sample(station - 20., 0.).1 * Vec3::Z;
    let b = track.sample(station + 20., 0.).1 * Vec3::Z;
    policy::turn_side([a.x, a.z], [b.x, b.z])
}
pub fn layout(track: &Track) -> Vec<Run> {
    policy::runs(
        &(1580..5380)
            .step_by(8)
            .map(|d| {
                let start = d as f32;
                let side = if start < bridge::END && start + 8. > bridge::START {
                    None
                } else {
                    side(track, start + 4.)
                };
                (start, side)
            })
            .collect::<Vec<_>>(),
    )
}
fn point(track: &Track, ground: &GroundSamples, station: f32, side: i8, flare: f32) -> Vec3 {
    let (road, q) = track.sample(station, 0.);
    let lane = side as f32 * (track.half_width(station) + 1.8 + flare);
    // Avoid sample's off-road terrain fallback: the rail follows the road grade.
    let p = road - q * Vec3::X * lane;
    p.with_y((p.y + policy::RAIL_CENTER_M).max(ground.height(track, p) + 0.85))
}
fn solid(commands: &mut Commands, art: &Art, a: Vec3, b: Vec3, width: f32, height: f32) -> Entity {
    let forward = (b - a).normalize();
    let right = Vec3::Y.cross(forward).normalize();
    let rotation = Quat::from_mat3(&Mat3::from_cols(right, forward.cross(right), forward));
    let e = crate::world::part(
        commands,
        art.cube.clone(),
        art.metal.clone(),
        (a + b) * 0.5,
        Vec3::new(width, height, (b - a).length() + 0.02),
        rotation,
    );
    commands.entity(e).insert((
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
        Obstacle,
        Guardrail,
    ));
    e
}
fn post(commands: &mut Commands, art: &Art, track: &Track, ground: &GroundSamples, top: Vec3) {
    // The bottom is measured from the highest resident terrain/shoulder triangle,
    // never a fixed-length extension from the road centre. Small embed seals feet.
    let bottom = ground.height(track, top) - 0.12;
    let height = top.y + 0.12 - bottom;
    let e = crate::world::part(
        commands,
        art.cube.clone(),
        art.metal.clone(),
        top.with_y(bottom + height * 0.5),
        Vec3::new(0.16, height, 0.16),
        Quat::IDENTITY,
    );
    commands.entity(e).insert((
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
        Obstacle,
        RailPost,
    ));
    if track.is_hills() {
        commands.entity(e).insert(HillsSolid);
    }
}
pub(super) fn spawn(
    commands: &mut Commands,
    art: &Art,
    track: &Track,
    census: &mut Census,
    ground: &GroundSamples,
) {
    let runs = layout(track);
    spawn_runs(commands, art, track, census, ground, runs);
}
pub(crate) fn spawn_legacy(
    commands: &mut Commands,
    art: &Art,
    track: &Track,
    ground: &GroundSamples,
    layout: Option<&crate::world_gen::layout::WorldLayout>,
) {
    let cells: Vec<_> = (0..track.length as usize)
        .step_by(8)
        .map(|d| {
            let start = d as f32;
            let side = signed_side(track, start + 4.).filter(|side| {
                [start, start + 4., start + 8.].into_iter().all(|station| {
                    track.sector(station) == rr_content::track::Sector::Hills
                        && layout.is_none_or(|l| {
                            let (road, q) = track.sample(station, 0.);
                            let p = road
                                - q * Vec3::X * (*side as f32 * (track.half_width(station) + 2.45));
                            l.scenery_allowed(p, 8.)
                        })
                })
            });
            (start, side)
        })
        .collect();
    spawn_runs(
        commands,
        art,
        track,
        &mut Census::default(),
        ground,
        policy::runs(&cells),
    );
}
fn spawn_runs(
    commands: &mut Commands,
    art: &Art,
    track: &Track,
    census: &mut Census,
    ground: &GroundSamples,
    runs: Vec<Run>,
) {
    for run in &runs {
        let mut d = run.start;
        // Two-metre flared terminals turn the blunt cap away from the riding line.
        // They remain inside the run interval, so the whole 16m gap stays clear.
        while d < run.end - 0.01 {
            let end = if d == run.start {
                d + 2.
            } else if d < run.end - 2. {
                (d + 4.).min(run.end - 2.)
            } else {
                run.end
            };
            let flare =
                |s: f32| (1. - ((s - run.start).min(run.end - s) / 2.).clamp(0., 1.)) * 0.65;
            let a = point(track, ground, d, run.side, flare(d));
            let b = point(track, ground, end, run.side, flare(end));
            let e = solid(commands, art, a, b, 0.16, policy::RAIL_DEPTH_M);
            if track.is_hills() {
                commands.entity(e).insert(HillsSolid);
            }
            if d == run.start || end == run.end {
                commands.entity(e).insert(RailTerminal);
            }
            post(commands, art, track, ground, a);
            d = end;
        }
        for station in [run.start, run.end] {
            let p = point(track, ground, station, run.side, 0.65);
            if station == run.end {
                post(commands, art, track, ground, p);
            }
            // Broad solid folded end shoe: no naked thin beam edge or collider
            // protruding along the opening. Reuses the original shared cube.
            let (_, q) = track.sample(station, 0.);
            let cap = crate::world::part(
                commands,
                art.cube.clone(),
                art.metal.clone(),
                p,
                Vec3::new(0.32, 0.46, 0.12),
                q,
            );
            commands.entity(cap).insert((
                RigidBody::Fixed,
                Collider::cuboid(0.5, 0.5, 0.5),
                Obstacle,
                RailTerminal,
            ));
            if track.is_hills() {
                commands.entity(cap).insert(HillsSolid);
            }
        }
        census.guardrail_segments += ((run.end - run.start) / policy::SEGMENT_M) as usize;
    }
    commands.insert_resource(RailRuns(runs));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::Ground;
    use bevy::ecs::system::RunSystemOnce;
    type RailFilter = Or<(With<Guardrail>, With<RailPost>, With<RailTerminal>)>;
    #[test]
    fn placed_rails_have_grounded_feet_solid_terminals_and_clear_full_size_gaps() {
        let mut app = bridge::tests::scene();
        let runs = app.world().resource::<RailRuns>().0.clone();
        assert!(runs.iter().any(|r| r.side == -1));
        assert!(runs.iter().any(|r| r.side == 1));
        assert!(
            runs.iter()
                .all(|r| r.end <= bridge::START || r.start >= bridge::END)
        );
        let posts: Vec<_> = app
            .world_mut()
            .query_filtered::<&Transform, With<RailPost>>()
            .iter(app.world())
            .copied()
            .collect();
        let terminals: Vec<_> = app
            .world_mut()
            .query_filtered::<&Transform, With<RailTerminal>>()
            .iter(app.world())
            .copied()
            .collect();
        assert_eq!(
            terminals.len(),
            runs.len() * 4,
            "two flared rails and two solid end shoes per run"
        );
        let gaps: Vec<_> = runs
            .windows(2)
            .filter(|p| {
                p[0].side == p[1].side && (p[1].start - p[0].end - policy::GAP_M).abs() < 0.01
            })
            .map(|p| ((p[0].end + p[1].start) * 0.5, p[0].side))
            .collect();
        assert!(
            gaps.len() >= 2,
            "actual long curves need deliberate openings: {runs:?}"
        );
        app.world_mut()
            .run_system_once(
                move |context: ReadRapierContext,
                      ground: Query<(), With<Ground>>,
                      rails: Query<(), RailFilter>,
                      track: Res<Track>| {
                    let ctx = context.single().unwrap();
                    let is_ground =
                        |e, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(e);
                    let is_rail =
                        |e, _: &bevy_rapier3d::rapier::geometry::Collider| rails.contains(e);
                    for post in &posts {
                        let p = post.translation;
                        let (_, toi) = ctx
                            .cast_ray(
                                p + Vec3::Y * 50.,
                                -Vec3::Y,
                                100.,
                                true,
                                QueryFilter::default().predicate(&is_ground),
                            )
                            .expect("every post stands on real resident collision");
                        let y = p.y + 50. - toi;
                        let bottom = p.y - post.scale.y * 0.5;
                        assert!(
                            (bottom - y + 0.12).abs() < 0.025,
                            "post ground mismatch: {bottom} vs {y}"
                        );
                    }
                    for t in &terminals {
                        let direction = t.rotation * Vec3::Z;
                        assert!(
                            ctx.cast_ray(
                                t.translation - direction * 2.,
                                direction,
                                4.,
                                true,
                                QueryFilter::default().predicate(&is_rail)
                            )
                            .is_some(),
                            "terminal must be solid"
                        );
                    }
                    for &(s, side) in &gaps {
                        // Test the useful middle 12m of every16m opening with a rider and
                        // a full chassis footprint, including both directions of travel.
                        for along in [-5., 0., 5.] {
                            let (road, q) = track.sample(s + along, 0.);
                            let from = road - q * Vec3::X * (side as f32 * 13.) + Vec3::Y * 1.;
                            let to = road - q * Vec3::X * (side as f32 * 4.) + Vec3::Y * 1.;
                            for shape in [
                                Collider::capsule_y(0.65, 0.38),
                                Collider::cuboid(0.45, 0.7, 1.2),
                            ] {
                                for (a, b) in [(from, to), (to, from)] {
                                    assert!(
                                        ctx.cast_shape(
                                            a,
                                            q,
                                            (b - a).normalize(),
                                            shape.raw.as_ref(),
                                            ShapeCastOptions {
                                                max_time_of_impact: (b - a).length(),
                                                ..default()
                                            },
                                            QueryFilter::default().predicate(&is_rail)
                                        )
                                        .is_none(),
                                        "gap blocked at {s}, offset{along}"
                                    );
                                }
                            }
                        }
                    }
                    for run in &runs {
                        let s = (run.start + run.end) * 0.5;
                        let (p, q) = track.sample(s, 0.);
                        let direction = -(q * Vec3::X) * run.side as f32;
                        let shoulder = p + direction * (track.half_width(s) + 1.8);
                        let (_, support_toi) = ctx
                            .cast_ray(
                                shoulder + Vec3::Y * 50.,
                                -Vec3::Y,
                                100.,
                                true,
                                QueryFilter::default().predicate(&is_ground),
                            )
                            .expect("rail-side resident support");
                        let support_y = shoulder.y + 50. - support_toi;
                        // Cross-sloping shoulders can lie above the road: aim at
                        // the raised rail using independent collision support.
                        let origin = p.with_y((p.y + policy::RAIL_CENTER_M).max(support_y + 0.85));
                        assert!(
                            ctx.cast_ray(
                                origin,
                                direction,
                                12.,
                                true,
                                QueryFilter::default().predicate(&is_rail)
                            )
                            .is_some(),
                            "turn-side rail missing at {s}: road={p:?}, side={}, ground={:?}",
                            run.side,
                            ctx.cast_ray(
                                p + direction * 8.3 + Vec3::Y * 50.,
                                -Vec3::Y,
                                100.,
                                true,
                                QueryFilter::default().predicate(&is_ground)
                            )
                        );
                        assert!(
                            ctx.cast_ray(
                                origin,
                                -direction,
                                12.,
                                true,
                                QueryFilter::default().predicate(&is_rail)
                            )
                            .is_none(),
                            "opposite side should stay open at{s}"
                        );
                    }
                },
            )
            .unwrap();
    }
}

#[cfg(test)]
mod legacy_tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    #[test]
    fn legacy_valley_guardrails_preserve_reservations_and_real_support() {
        let mut app = crate::world_gen::construction_tests::scene();
        let runs = &app.world().resource::<RailRuns>().0;
        assert!(
            !runs.is_empty(),
            "inspect placed legacy barriers, not only enum names"
        );
        let expected_terminals = runs.len() * 4;
        let terminals = app
            .world_mut()
            .query_filtered::<Entity, With<RailTerminal>>()
            .iter(app.world())
            .count();
        assert_eq!(terminals, expected_terminals);
        let posts: Vec<_> = app
            .world_mut()
            .query_filtered::<&Transform, With<RailPost>>()
            .iter(app.world())
            .copied()
            .collect();
        assert!(!posts.is_empty());
        app.world_mut()
            .run_system_once(
                move |context: ReadRapierContext,
                      ground: Query<(), With<crate::world::Ground>>,
                      layout: Res<crate::world_gen::layout::WorldLayout>| {
                    let ctx = context.single().unwrap();
                    let is_ground =
                        |e, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(e);
                    for post in &posts {
                        assert!(
                            layout.scenery_allowed(post.translation, 0.2),
                            "rail cannot invade lot/access reservations"
                        );
                        let p = post.translation;
                        let (_, toi) = ctx
                            .cast_ray(
                                p + Vec3::Y * 50.,
                                -Vec3::Y,
                                100.,
                                true,
                                QueryFilter::default().predicate(&is_ground),
                            )
                            .expect("resident legacy support");
                        assert!(
                            (p.y - post.scale.y * 0.5 - (p.y + 50. - toi) + 0.12).abs() < 0.025,
                            "legacy foot must touch actual terrain"
                        );
                    }
                },
            )
            .unwrap();
    }
}

/// Validate authored openings against current Ground and obstacle collision.
/// No teleport, rig/controller change or global path search is involved.
pub(crate) fn reentry_path(
    track: &Track,
    runs: &RailRuns,
    start: Vec3,
    goal: Vec3,
    mut supported: impl FnMut(Vec3) -> Option<Vec3>,
    clear: impl Fn(Vec3, Vec3) -> bool,
) -> Option<Vec<Vec3>> {
    let (station, lane, _) = track.project(start, 0.);
    let (goal_station, goal_lane, _) = track.project(goal, station);
    let candidates = policy::reentry_candidates(
        &runs.0,
        [station, lane],
        [goal_station, goal_lane],
        track.half_width(station),
    );
    for candidate in candidates {
        let mut path = Vec::new();
        let mut previous = start;
        let mut valid = true;
        for [station, lane] in candidate {
            let Some(p) = supported(track.sample(station, lane).0) else {
                valid = false;
                break;
            };
            if !clear(previous, p) {
                valid = false;
                break;
            }
            path.push(p);
            previous = p;
        }
        if valid && !path.is_empty() {
            return Some(path);
        }
    }
    None
}
