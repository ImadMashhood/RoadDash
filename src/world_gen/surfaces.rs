//! Continuous structural surfaces; no new NPC/prop population.
use super::{
    WorldPlan,
    layout::{WorldLayout, horizontal_frame},
};
use crate::{track::Track, vehicle::RideableStep, world::Ground};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
#[derive(Component)]
pub struct StructuralSurface(pub &'static str);
fn surface(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    vertices: Vec<Vec3>,
    indices: Vec<[u32; 3]>,
    kind: &'static str,
) {
    if indices.is_empty() {
        return;
    }
    commands.spawn((
        Name::new(kind),
        Mesh3d(meshes.add(crate::environment::mesh(&vertices, &indices))),
        MeshMaterial3d(material),
        Transform::default(),
        RigidBody::Fixed,
        Collider::trimesh(vertices, indices).expect("structural ground"),
        Ground,
        RideableStep,
        StructuralSurface(kind),
        Friction::coefficient(0.85),
    ));
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
    layout: &WorldLayout,
) {
    let asphalt = materials.add(Color::srgb(0.14, 0.15, 0.16));
    let walk = materials.add(Color::srgb(0.55, 0.54, 0.49));
    let dirt = materials.add(Color::srgb(0.46, 0.38, 0.26));
    for j in &layout.settings.junctions {
        let (p, q) = horizontal_frame(track, j.station_m);
        let (_, bank) = track.sample(j.station_m, 0.0);
        for side in [-1.0, 1.0] {
            let mut v = Vec::new();
            let mut ix = Vec::new();
            let count = ((j.extent_m - track.half_width(j.station_m)) / 2.0).ceil() as usize;
            for i in 0..=count {
                let lane = side
                    * (track.half_width(j.station_m)
                        + (j.extent_m - track.half_width(j.station_m)) * i as f32 / count as f32);
                for offset in [-j.half_width_m, j.half_width_m] {
                    let mut point = p - q * Vec3::X * lane + q * Vec3::Z * offset;
                    point.y = p.y - (bank * Vec3::X).y * lane + (bank * Vec3::Z).y * offset + 0.02;
                    v.push(point);
                }
                if i < count {
                    let n = i as u32 * 2;
                    ix.extend([[n, n + 1, n + 2], [n + 1, n + 3, n + 2]]);
                }
            }
            // Across-lane ordering changes with side; ensure top-facing triangles.
            upward(&v, &mut ix);
            surface(commands, meshes, asphalt.clone(), v, ix, "junction");
        }
    }
    let start = layout.settings.sidewalk_start_m;
    let end = layout.settings.sidewalk_end_m;
    let mut breaks = vec![start, end];
    for j in &layout.settings.junctions {
        for d in [j.station_m - j.half_width_m, j.station_m + j.half_width_m] {
            if d > start && d < end {
                breaks.push(d);
            }
        }
    }
    breaks.sort_by(f32::total_cmp);
    for side in [-1.0, 1.0] {
        for span in breaks.windows(2) {
            if layout.settings.junction_at((span[0] + span[1]) * 0.5, 0.0) {
                continue;
            }
            let mut d = span[0];
            while d < span[1] {
                let next = (d + 8.0).min(span[1]);
                let mut v = Vec::new();
                for at in [d, next] {
                    let (p, q) = track.sample(at, 0.0);
                    let nearest = layout
                        .settings
                        .junctions
                        .iter()
                        .map(|j| (at - j.station_m).abs() - j.half_width_m)
                        .fold(f32::INFINITY, f32::min);
                    let curb = layout.settings.curb_height_m * (nearest / 5.0).clamp(0.0, 1.0);
                    for lane in [
                        track.half_width(at) + track.definition.verge_width,
                        track.half_width(at)
                            + track.definition.verge_width
                            + layout.settings.sidewalk_width_m,
                    ] {
                        v.push(p - q * Vec3::X * side * lane + Vec3::Y * (curb + 0.025));
                    }
                }
                let mut ix = vec![[0, 1, 2], [1, 3, 2]];
                for i in 0..4 {
                    v.push(v[i] - Vec3::Y * layout.settings.curb_height_m);
                }
                for (a, b) in [(0, 1), (1, 3), (3, 2), (2, 0)] {
                    ix.extend([[a, b, a + 4], [b, b + 4, a + 4]]);
                }
                // Only top triangles need upward winding; side walls retain their normal.
                upward(&v, &mut ix[..2]);
                surface(commands, meshes, walk.clone(), v, ix, "sidewalk");
                d = next;
            }
        }
    }
    for alley in &layout.alleys {
        let mut d = alley.start_m;
        let mut v = Vec::new();
        let mut ix = Vec::new();
        let mut i = 0u32;
        loop {
            let (p, q) = horizontal_frame(track, d);
            for lane in [
                alley.lane_m - alley.width_m * 0.5,
                alley.lane_m + alley.width_m * 0.5,
            ] {
                let mut point = p - q * Vec3::X * lane;
                point.y = track.terrain_height(point.x, point.z) + 0.025;
                v.push(point);
            }
            if d >= alley.end_m {
                break;
            }
            ix.extend([[i, i + 1, i + 2], [i + 1, i + 3, i + 2]]);
            i += 2;
            d = (d + 2.0).min(alley.end_m);
        }
        upward(&v, &mut ix);
        surface(commands, meshes, dirt.clone(), v, ix, "alley");
    }
    // Exact flat foundation planes fill holes cut into the terrain mesh.
    for b in &layout.buildings {
        let corners = b
            .footprint
            .expanded(layout.settings.pad_flat_margin_m)
            .corners();
        let v = corners
            .map(|p| Vec3::new(p.x, b.foundation_m, p.y))
            .to_vec();
        let mut ix = vec![[0, 1, 2], [0, 2, 3]];
        upward(&v, &mut ix);
        surface(commands, meshes, dirt.clone(), v, ix, "foundation_pad");
    }
}
fn upward(v: &[Vec3], indices: &mut [[u32; 3]]) {
    for t in indices {
        if (v[t[1] as usize] - v[t[0] as usize])
            .cross(v[t[2] as usize] - v[t[0] as usize])
            .y
            < 0.0
        {
            t.swap(1, 2);
        }
    }
}
pub fn terrain_color(plan: &WorldPlan, layout: &WorldLayout, track: &Track, p: Vec3) -> [f32; 4] {
    use rr_content::world::District;
    let (d, lane) = track.nearby_station(p.x, p.z);
    let i = plan
        .section_at(d.clamp(0.0, track.length), track)
        .unwrap_or(0);
    let section = &plan.definition.sections[i];
    let mut color = match section.district {
        District::Farms => [0.27, 0.42, 0.16],
        District::RuralEdge => [0.34, 0.43, 0.22],
        District::Suburban => [0.38, 0.45, 0.30],
        District::InnerCity => [0.40, 0.41, 0.38],
        District::CityCore => [0.38, 0.39, 0.39],
        District::CityToHills => [0.33, 0.43, 0.27],
        District::Hills => [0.30, 0.43, 0.20],
    };
    if d < layout.settings.field_end_m && (18.0..120.0).contains(&lane.abs()) {
        let field = (d / (layout.settings.field_end_m / (layout.settings.field_count / 2) as f32))
            as usize
            * 2
            + usize::from(lane > 0.0);
        let shades = [
            [0.36, 0.44, 0.19],
            [0.43, 0.42, 0.22],
            [0.24, 0.38, 0.14],
            [0.32, 0.46, 0.19],
        ];
        color = shades[field % 4];
    }
    let variation = 0.92 + i as f32 * 0.008;
    [
        color[0] * variation,
        color[1] * variation,
        color[2] * variation,
        1.0,
    ]
}
