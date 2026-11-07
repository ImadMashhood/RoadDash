//! Deterministic biome/sidewalk furnishing with whole-footprint and recovery clearance.
use super::{layout::WorldLayout, spatial::Rect, streetscape::sidewalk_point};
use crate::track::Track;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rr_content::dressing::{Config, Placement};
use std::collections::BTreeMap;
pub const JSON: &str = include_str!("../../content/world/dressing.json");
#[derive(Clone)]
pub struct Socket {
    pub id: String,
    pub role: String,
    pub builder: String,
    pub group: String,
    pub zone: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub size: Vec3,
    pub bounds: Rect,
    pub seed: usize,
}
#[derive(Component)]
pub struct DressedProp {
    pub role: String,
    pub group: String,
    pub zone: String,
}
#[derive(Resource, Default)]
pub struct Evidence {
    pub planned: usize,
    pub placed: usize,
    pub rejected_existing: usize,
    pub roles: BTreeMap<String, usize>,
    pub groups: BTreeMap<String, usize>,
    pub zones: BTreeMap<String, usize>,
}
fn safe(track: &Track, layout: &WorldLayout, socket: &Socket, others: &[Socket], gap: f32) -> bool {
    let radius = socket.size.x.max(socket.size.z) * 0.5;
    track.scenery_fits(socket.position, radius, 1.0)
        && !layout
            .blockers()
            .chain(layout.access_reservations.iter())
            .chain(layout.junction_rects.iter())
            .any(|r| r.overlaps(&socket.bounds))
        && !layout
            .grid_streets
            .iter()
            .any(|s| s.reservation().overlaps(&socket.bounds))
        && !others.iter().any(|s| {
            s.bounds.overlaps(&socket.bounds)
                || s.position.distance(socket.position)
                    < gap + radius + s.size.x.max(s.size.z) * 0.5
        })
}
fn hash(mut value: u32) -> u32 {
    value = (value ^ (value >> 16)).wrapping_mul(0x7feb352d);
    value = (value ^ (value >> 15)).wrapping_mul(0x846ca68b);
    value ^ (value >> 16)
}
fn zone_seed(track: &Track, id: &str) -> u32 {
    id.bytes().fold(track.definition.scenery_seed, |h, b| {
        (h ^ u32::from(b)).wrapping_mul(16777619)
    })
}
fn signed(seed: u32) -> f32 {
    (hash(seed) & 65535) as f32 / 65535.0 * 2.0 - 1.0
}
pub fn plan(track: &Track, layout: &WorldLayout, c: &Config) -> Vec<Socket> {
    let mut out = Vec::new();
    // Reserve each zone a fair share so farms/city cannot consume every hill socket.
    let total_weight: usize = c.zones.iter().map(|z| z.budget_weight as usize).sum();
    let discretionary = c.max_props - c.zones.len();
    let mut shares: Vec<_> = c
        .zones
        .iter()
        .map(|z| 1 + discretionary * z.budget_weight as usize / total_weight)
        .collect();
    let remainder = c.max_props - shares.iter().sum::<usize>();
    for share in shares.iter_mut().take(remainder) {
        *share += 1;
    }
    for (zone_index, zone) in c.zones.iter().enumerate() {
        let start_count = out.len();
        let allowance = shares[zone_index];
        let salt = zone_seed(track, &zone.id);
        let mut sites: Vec<_> = if zone.placement == Placement::FieldEdge {
            layout
                .fields
                .iter()
                .enumerate()
                .filter_map(|(i, f)| {
                    let d = (f.start_m + f.end_m) * 0.5;
                    (d >= zone.from_m && d < zone.to_m).then_some((
                        i,
                        d,
                        f.side * (f.outer_m - zone.offset_m),
                    ))
                })
                .collect()
        } else {
            let count = ((zone.to_m - zone.from_m) / zone.spacing_m).ceil() as usize;
            (0..count)
                .flat_map(|i| {
                    [-1.0, 1.0].map(move |side| {
                        let index = i * 2 + usize::from(side > 0.0);
                        let d = zone.from_m
                            + (i as f32 + 0.5 + signed(salt ^ index as u32) * 0.22)
                                * zone.spacing_m;
                        (index, d, side)
                    })
                })
                .filter(|(_, d, _)| *d < zone.to_m)
                .map(|(i, d, side)| (i, d, side * (track.half_width(d) + zone.offset_m)))
                .collect()
        };
        // Priority is seeded, not distance ordered: a finite budget still dresses
        // the entire zone instead of filling only its first few hundred metres.
        sites.sort_by_key(|(i, _, _)| hash(salt ^ *i as u32));
        for (anchor, station, lane) in sites {
            let seed = hash(salt ^ anchor as u32);
            let fallback;
            let (group_name, members) = if zone.groups.is_empty() {
                let role = &zone.roles[seed as usize % zone.roles.len()];
                fallback = vec![rr_content::dressing::Member {
                    role: role.clone(),
                    along_m: 0.0,
                    outward_m: 0.0,
                    yaw_deg: 0.0,
                }];
                ("single", fallback.as_slice())
            } else {
                let name = &zone.groups[seed as usize % zone.groups.len()];
                (name.as_str(), c.groups[name].as_slice())
            };
            for (member, authored) in members.iter().enumerate() {
                if out.len() - start_count >= allowance {
                    break;
                }
                let item_seed = hash(seed ^ (member as u32).wrapping_mul(0x9e3779b9));
                let d = station + authored.along_m;
                if d < zone.from_m || d >= zone.to_m || layout.settings.junction_at(d, 18.0) {
                    continue;
                }
                let role = &c.roles[&authored.role];
                let scale = 1.0 + signed(item_seed ^ 17) * 0.08;
                let size = Vec3::from_array(role.size) * scale;
                let lateral_jitter = if zone.placement == Placement::Sidewalk {
                    0.04
                } else {
                    1.4
                };
                let lane = lane
                    + lane.signum() * (authored.outward_m + signed(seed ^ 91) * lateral_jitter);
                let (centre, _) = track.sample(d, 0.0);
                let (mut p, _) = track.sample(d, lane);
                if zone.placement == Placement::Sidewalk {
                    p = sidewalk_point(track, layout, d, lane);
                } else {
                    p.y = track.terrain_height(p.x, p.z) + 0.015;
                }
                let toward = (centre - p).with_y(0.0).normalize_or_zero();
                let yaw_jitter = if zone.placement == Placement::Sidewalk {
                    2.0
                } else {
                    12.0
                };
                let rotation = Quat::from_rotation_y(
                    toward.x.atan2(toward.z)
                        + (authored.yaw_deg + signed(item_seed ^ 43) * yaw_jitter).to_radians(),
                );
                let bounds = Rect::new(p, rotation, Vec2::new(size.x, size.z) * 0.5);
                let support: Vec<_> = bounds
                    .corners()
                    .into_iter()
                    .map(|v| {
                        if zone.placement == Placement::Sidewalk {
                            p.y - 0.015
                        } else {
                            track.terrain_height(v.x, v.y)
                        }
                    })
                    .collect();
                let highest = support.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                if highest - support.iter().copied().fold(f32::INFINITY, f32::min)
                    > c.max_support_relief_m
                {
                    continue;
                }
                let socket = Socket {
                    id: format!("DRESS-{}-{anchor:03}-{member}", zone.id),
                    role: authored.role.clone(),
                    builder: role.builder.clone(),
                    group: group_name.into(),
                    zone: zone.id.clone(),
                    position: p,
                    rotation,
                    size,
                    bounds,
                    seed: item_seed as usize,
                };
                if safe(track, layout, &socket, &out, c.minimum_gap_m) {
                    out.push(socket);
                }
            }
        }
    }
    out
}
#[allow(clippy::type_complexity)]
pub fn spawn(
    mut commands: Commands,
    art: Option<Res<crate::world::Art>>,
    track: Res<Track>,
    layout: Option<Res<WorldLayout>>,
    existing: Query<(&Transform, &crate::world::SceneryFootprint), Without<ChildOf>>,
) {
    let (Some(art), Some(layout)) = (art, layout) else {
        return;
    };
    let Some(ref props) = art.props else { return };
    let config = Config::from_json(JSON, &props.catalog).expect("validated dressing roles");
    let sockets = plan(&track, &layout, &config);
    let mut evidence = Evidence {
        planned: sockets.len(),
        ..default()
    };
    for s in sockets {
        let radius = s.size.x.max(s.size.z) * 0.5;
        if existing.iter().any(|(t, f)| {
            t.translation.xz().distance(s.position.xz()) < f.radius + radius + config.minimum_gap_m
        }) {
            evidence.rejected_existing += 1;
            continue;
        }
        let model = props.choose(&s.builder, s.seed);
        let child = props.fitted(
            &mut commands,
            model,
            Vec3::ZERO,
            Quat::IDENTITY,
            s.size,
            "semantic_dressing",
        );
        let owner = commands
            .spawn((
                Name::new(s.id.clone()),
                DressedProp {
                    role: s.role.clone(),
                    group: s.group.clone(),
                    zone: s.zone.clone(),
                },
                Transform::from_translation(s.position).with_rotation(s.rotation),
                Visibility::default(),
                RigidBody::Fixed,
                Collider::compound(vec![(
                    Vec3::Y * s.size.y * 0.5,
                    Quat::IDENTITY,
                    Collider::cuboid(s.size.x * 0.5, s.size.y * 0.5, s.size.z * 0.5),
                )]),
                crate::world::Obstacle,
                crate::world::SceneryFootprint { radius },
                Friction::coefficient(0.7),
            ))
            .id();
        commands.entity(owner).add_child(child);
        evidence.placed += 1;
        *evidence.roles.entry(s.role).or_default() += 1;
        *evidence.groups.entry(s.group).or_default() += 1;
        *evidence.zones.entry(s.zone).or_default() += 1;
    }
    commands.insert_resource(evidence);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_dressing_is_seeded_bounded_and_preserves_roads_and_access() {
        let track = Track::from_json(include_str!("../../content/tracks/valley.json")).unwrap();
        let world = super::super::WorldPlan::from_json(
            super::super::WORLD_JSON,
            super::super::ARCHETYPES_JSON,
            &track,
        )
        .unwrap();
        let layout = WorldLayout::generate(
            &world,
            &track,
            super::super::layout::Construction::from_json(
                super::super::layout::CONSTRUCTION_JSON,
                &track,
            )
            .unwrap(),
        )
        .unwrap();
        let catalog = rr_content::props::Catalog::from_json(crate::props::JSON).unwrap();
        let c = Config::from_json(JSON, &catalog).unwrap();
        let first = plan(&track, &layout, &c);
        let second = plan(&track, &layout, &c);
        assert!(
            first.len() > 30 && first.len() <= c.max_props,
            "{}",
            first.len()
        );
        assert_eq!(
            first.iter().map(|s| &s.id).collect::<Vec<_>>(),
            second.iter().map(|s| &s.id).collect::<Vec<_>>()
        );
        for (i, s) in first.iter().enumerate() {
            assert!(safe(&track, &layout, s, &first[..i], c.minimum_gap_m));
        }
        let roles: std::collections::BTreeSet<_> = first.iter().map(|s| &s.role).collect();
        let groups: std::collections::BTreeSet<_> = first.iter().map(|s| &s.group).collect();
        assert!(
            roles.len() >= 28,
            "insufficient role variety: {}",
            roles.len()
        );
        assert!(
            groups.len() >= 16,
            "insufficient group variety: {}",
            groups.len()
        );
        for zone in &c.zones {
            let sites: Vec<_> = first.iter().filter(|s| s.zone == zone.id).collect();
            assert!(
                sites.len() >= 12,
                "{} was starved: {}",
                zone.id,
                sites.len()
            );
            let mut stations: Vec<_> = sites
                .iter()
                .map(|s| {
                    track
                        .points
                        .iter()
                        .enumerate()
                        .min_by(|(_, a), (_, b)| {
                            a.distance_squared(s.position)
                                .total_cmp(&b.distance_squared(s.position))
                        })
                        .map(|(i, _)| track.distances[i])
                        .unwrap()
                })
                .collect();
            stations.sort_by(f32::total_cmp);
            assert!(
                stations[0] < zone.from_m + (zone.to_m - zone.from_m) * 0.35,
                "{} missing zone start",
                zone.id
            );
            assert!(
                stations[stations.len() - 1] > zone.from_m + (zone.to_m - zone.from_m) * 0.65,
                "{} missing zone end",
                zone.id
            );
        }
        assert!(
            first
                .iter()
                .zip(&second)
                .all(|(a, b)| a.position == b.position
                    && a.rotation == b.rotation
                    && a.size == b.size
                    && a.seed == b.seed)
        );
        let mut changed_track = track;
        changed_track.definition.scenery_seed ^= 0x12345678;
        let changed = plan(&changed_track, &layout, &c);
        assert!(
            first
                .iter()
                .zip(&changed)
                .any(|(a, b)| a.position != b.position
                    || a.builder != b.builder
                    || a.seed != b.seed)
        );
        let mut bad: serde_json::Value = serde_json::from_str(JSON).unwrap();
        bad["roles"]["bench"]["builder"] = "unlicensed_or_missing".into();
        assert!(Config::from_json(&bad.to_string(), &catalog).is_err());
        for (field, value) in [
            ("role", serde_json::json!("missing")),
            ("along_m", serde_json::json!(100.0)),
            ("outward_m", serde_json::json!(-1.0)),
        ] {
            let mut bad: serde_json::Value = serde_json::from_str(JSON).unwrap();
            bad["groups"]["city_transit"][0][field] = value;
            assert!(Config::from_json(&bad.to_string(), &catalog).is_err());
        }
        let mut bad: serde_json::Value = serde_json::from_str(JSON).unwrap();
        bad["groups"]["city_transit"][0]["outward_m"] = 2.0.into();
        assert!(
            Config::from_json(&bad.to_string(), &catalog).is_err(),
            "sidewalk cluster may not block pedestrian strip"
        );
    }
}
