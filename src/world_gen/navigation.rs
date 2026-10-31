//! Small, bounded visibility-graph routes around structural footprints.
use super::layout::WorldLayout;
use crate::track::Track;
use bevy::prelude::*;
fn segment_distance(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let edge = b - a;
    let t = ((p - a).dot(edge) / edge.length_squared().max(0.001)).clamp(0.0, 1.0);
    p.distance(a + edge * t)
}
pub fn clear(layout: &WorldLayout, from: Vec2, to: Vec2) -> bool {
    !layout
        .buildings
        .iter()
        .any(|b| b.footprint.blocks_segment(from, to, 0.4))
}
/// Returned waypoints are actual terrain-relative positions. Physics still owns
/// movement/contact; a route never teleports the recovering actor.
pub fn route(layout: &WorldLayout, track: &Track, from: Vec3, to: Vec3) -> Option<Vec<Vec3>> {
    if from.xz().distance(to.xz()) > 350.0 {
        return None;
    }
    let point = |p: Vec2| Vec3::new(p.x, track.terrain_height(p.x, p.y) + 0.95, p.y);
    if clear(layout, from.xz(), to.xz()) {
        return Some(vec![point(to.xz())]);
    }
    let mut nearby: Vec<_> = layout
        .buildings
        .iter()
        .filter(|b| {
            segment_distance(Vec2::from_array(b.footprint.center), from.xz(), to.xz()) < 35.0
        })
        .collect();
    nearby.sort_by(|a, b| {
        segment_distance(Vec2::from_array(a.footprint.center), from.xz(), to.xz()).total_cmp(
            &segment_distance(Vec2::from_array(b.footprint.center), from.xz(), to.xz()),
        )
    });
    nearby.truncate(48);
    let mut nodes = vec![from.xz(), to.xz()];
    for b in nearby {
        nodes.extend(b.footprint.expanded(1.0).corners());
    }
    let n = nodes.len();
    let mut costs = vec![f32::INFINITY; n];
    let mut parent = vec![usize::MAX; n];
    let mut closed = vec![false; n];
    costs[0] = 0.0;
    for _ in 0..n {
        let current = (0..n).filter(|i| !closed[*i]).min_by(|a, b| {
            (costs[*a] + nodes[*a].distance(nodes[1]))
                .total_cmp(&(costs[*b] + nodes[*b].distance(nodes[1])))
        })?;
        if !costs[current].is_finite() {
            return None;
        }
        if current == 1 {
            break;
        }
        closed[current] = true;
        for next in 1..n {
            if closed[next] || next == current {
                continue;
            }
            let length = nodes[current].distance(nodes[next]);
            if costs[current] + length >= costs[next] || !clear(layout, nodes[current], nodes[next])
            {
                continue;
            }
            costs[next] = costs[current] + length;
            parent[next] = current;
        }
    }
    if parent[1] == usize::MAX {
        return None;
    }
    let mut result = vec![point(nodes[1])];
    let mut i = 1;
    for _ in 0..n {
        i = parent[i];
        if i == 0 {
            result.reverse();
            return Some(result);
        }
        if i == usize::MAX {
            return None;
        }
        result.push(point(nodes[i]));
    }
    None
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct WalkAudit {
    pub building_id: String,
    pub waypoints: usize,
    pub length_3d_m: f32,
}
/// Every building has an outside-corner structural route to the road.
/// Kinematic/physics integration tests additionally verify real capsule movement.
pub fn audit(layout: &WorldLayout, track: &Track) -> Result<Vec<WalkAudit>, String> {
    let mut report = Vec::new();
    for b in &layout.buildings {
        let (_, q) = super::layout::horizontal_frame(track, b.station_m);
        let road = track.sample(b.station_m, b.lane_m.signum() * 5.0).0 + Vec3::Y * 0.95;
        let start = Vec3::from_array(b.position)
            + q * Vec3::X * (-b.lane_m.signum()) * (b.size[0] * 0.5 + 3.5)
            + q * Vec3::Z * (b.size[2] * 0.5 + 3.5);
        let start = Vec3::new(
            start.x,
            track.terrain_height(start.x, start.z) + 0.95,
            start.z,
        );
        let path = route(layout, track, start, road)
            .ok_or_else(|| format!("{}: no structural foot escape route", b.id))?;
        let mut before = start;
        let mut length = 0.0;
        for p in &path {
            if !clear(layout, before.xz(), p.xz()) {
                return Err(format!("{}: routed through building", b.id));
            }
            length += before.distance(*p);
            before = *p;
        }
        report.push(WalkAudit {
            building_id: b.id.clone(),
            waypoints: path.len(),
            length_3d_m: length,
        });
    }
    Ok(report)
}
