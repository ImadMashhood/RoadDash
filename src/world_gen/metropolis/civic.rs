//! Original resident civic models. All solid pieces share a persistent compound
//! collider; decorative visibility never removes architectural contact.
use super::*;
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
};
#[derive(Component)]
pub struct CivicLandmark;
pub const HALL_SCALE: f32 = 2.8;
/// Keep planting out of the fountain, beds, benches and pavilion.
pub fn park_planting_clear(p: Vec2, margin: f32) -> bool {
    if p.length() < 13. + margin || p.distance(Vec2::new(24., 130.)) < 12. + margin {
        return false;
    }
    for x in [-23., 23., 90.] {
        for z in [-100., -50., -35., 35., 50., 100., 105.] {
            if (p.x - x).abs() < 5. + margin && (p.y - z).abs() < 7. + margin {
                return false;
            }
        }
    }
    for x in [-16., 16., -9., 9., 75.] {
        for z in [-100., -50., -35., -20., 20., 35., 50., 100., 105.] {
            if (p.x - x).abs() < 3. + margin && (p.y - z).abs() < 2. + margin {
                return false;
            }
        }
    }
    true
}
pub fn park_tree_positions(site: &rr_content::metropolis::CivicSite) -> Vec<Vec2> {
    let triangles: Vec<_> = site
        .grounds
        .iter()
        .flat_map(|shape| {
            let v: Vec<_> = shape.vertices().map(Vec2::from_array).collect();
            shape
                .triangles
                .iter()
                .map(move |t| t.map(|i| v[i as usize]))
        })
        .collect();
    let inside = |p: Vec2| {
        let p = p + Vec2::from_array(site.position);
        triangles.iter().any(|[a, b, c]| {
            let signs = [
                (*b - *a).perp_dot(p - *a),
                (*c - *b).perp_dot(p - *b),
                (*a - *c).perp_dot(p - *c),
            ];
            signs.iter().all(|s| *s >= -0.001) || signs.iter().all(|s| *s <= 0.001)
        })
    };
    let clear = |p: Vec2, radius: f32| {
        [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y]
            .into_iter()
            .all(|d| inside(p + d * radius))
    };
    let mut trees: Vec<_> = [-1., 1.]
        .into_iter()
        .flat_map(|side| {
            (-3..=3)
                .filter(|i| *i != 0)
                .map(move |i| Vec2::new(side * site.size[0] * 0.38, i as f32 * site.size[1] / 8.))
        })
        .collect();
    // Follow the full irregular block perimeter, retaining clear promenades,
    // planted beds, pavilion and canopy room inside the public curb.
    for x in (-48..=144).step_by(24) {
        for z in (-144..=144).step_by(24) {
            let p = Vec2::new(x as f32, z as f32);
            if !clear(p, 6.)
                || !park_planting_clear(p, 6.)
                || p.y.abs() < 8.
                || p.x.abs() < 8.
                || ((p.x - 65.).abs() < 8.)
                || p.distance(Vec2::new(24., 130.)) < 18.
                || trees.iter().any(|other| p.distance(*other) < 14.)
            {
                continue;
            }
            trees.push(p);
        }
    }
    trees
}

pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    art: &Art,
    track: &Track,
    images: &mut Assets<Image>,
) {
    let stone = materials.add(Color::srgb(0.73, 0.68, 0.55));
    let trim = materials.add(Color::srgb(0.88, 0.84, 0.71));
    let roof = materials.add(Color::srgb(0.18, 0.29, 0.26));
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.18, 0.21),
        metallic: 0.25,
        perceptual_roughness: 0.25,
        ..default()
    });
    let water = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.46, 0.59),
        perceptual_roughness: 0.2,
        ..default()
    });
    let canopy = materials.add(Color::srgb(0.49, 0.14, 0.09));
    let soil = materials.add(Color::srgb(0.22, 0.14, 0.08));
    let flowers = [
        materials.add(Color::srgb(0.83, 0.28, 0.17)),
        materials.add(Color::srgb(0.95, 0.73, 0.2)),
        materials.add(Color::srgb(0.55, 0.35, 0.68)),
    ];
    let cylinder = meshes.add(Cylinder::new(1., 1.));
    for site in &track.metropolis.as_ref().unwrap().civic_sites {
        let center = Vec3::new(
            site.position[0],
            track.terrain_height(site.position[0], site.position[1]) + 0.16,
            site.position[1],
        );
        let rotation = Quat::from_rotation_y(site.heading);
        let root = commands
            .spawn((
                Name::new(site.id.clone()),
                CivicLandmark,
                Transform::from_translation(center)
                    .with_rotation(rotation)
                    .with_scale(Vec3::splat(if site.kind == "city_hall" {
                        HALL_SCALE
                    } else {
                        1.
                    })),
                Visibility::default(),
            ))
            .id();
        let mut contacts = Vec::new();
        let mut piece = |mut pos: Vec3,
                         size: Vec3,
                         q: Quat,
                         material: Handle<StandardMaterial>,
                         solid: bool,
                         round: bool| {
            if site.kind == "park" && pos.xz().length() > 10. {
                let world = center + rotation * pos;
                pos.y += track.terrain_height(world.x, world.z) + 0.16 - center.y;
            }
            let mesh = if round {
                cylinder.clone()
            } else {
                art.cube.clone()
            };
            let child = crate::world::part(commands, mesh, material, pos, size, q);
            commands.entity(root).add_child(child);
            if solid {
                contacts.push((
                    pos,
                    q,
                    if round {
                        Collider::cylinder(size.y * 0.5, size.x.max(size.z))
                    } else {
                        Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5)
                    },
                ));
            }
        };
        if site.kind == "city_hall" {
            // 60m civic frontage, tall central hall and copper clock tower.
            piece(
                Vec3::new(0., -0.6, 0.),
                Vec3::new(62., 3.2, 46.),
                Quat::IDENTITY,
                stone.clone(),
                true,
                false,
            );
            piece(
                Vec3::new(0., 10., -4.),
                Vec3::new(28., 20., 29.),
                Quat::IDENTITY,
                stone.clone(),
                true,
                false,
            );
            for side in [-1., 1.] {
                piece(
                    Vec3::new(side * 23., 7., -4.),
                    Vec3::new(17., 14., 29.),
                    Quat::IDENTITY,
                    stone.clone(),
                    true,
                    false,
                );
                piece(
                    Vec3::new(side * 23., 14.4, -4.),
                    Vec3::new(19., 1., 31.),
                    Quat::IDENTITY,
                    trim.clone(),
                    true,
                    false,
                );
                piece(
                    Vec3::new(side * 23., 15.2, -4.),
                    Vec3::new(17., 0.8, 29.),
                    Quat::IDENTITY,
                    roof.clone(),
                    true,
                    false,
                );
            }
            piece(
                Vec3::new(0., 20.5, -4.),
                Vec3::new(30., 1., 31.),
                Quat::IDENTITY,
                trim.clone(),
                true,
                false,
            );
            piece(
                Vec3::new(0., 28., -4.),
                Vec3::new(12., 15., 12.),
                Quat::IDENTITY,
                stone.clone(),
                true,
                false,
            );
            piece(
                Vec3::new(0., 36., -4.),
                Vec3::new(15., 1., 15.),
                Quat::IDENTITY,
                trim.clone(),
                true,
                false,
            );
            piece(
                Vec3::new(0., 39., -4.),
                Vec3::new(7., 5., 7.),
                Quat::IDENTITY,
                roof.clone(),
                true,
                true,
            );
            // Recessed facade windows and a clock, not borrowed facade textures.
            for x in (-27..=27).step_by(6) {
                for y in [4., 10.] {
                    piece(
                        Vec3::new(x as f32, y, 10.6),
                        Vec3::new(2.2, 3.6, 0.25),
                        Quat::IDENTITY,
                        glass.clone(),
                        false,
                        false,
                    );
                }
            }
            for x in [-10., -5., 0., 5., 10.] {
                piece(
                    Vec3::new(x, 16., 10.6),
                    Vec3::new(2.6, 4.2, 0.3),
                    Quat::IDENTITY,
                    glass.clone(),
                    false,
                    false,
                );
                piece(
                    Vec3::new(x, 18.4, 10.9),
                    Vec3::new(3.4, 0.35, 0.6),
                    Quat::IDENTITY,
                    trim.clone(),
                    false,
                    false,
                );
            }
            for side in [-1., 1.] {
                for z in [-15., -9., -3., 3., 8.] {
                    for y in [4., 10.] {
                        piece(
                            Vec3::new(side * 31.6, y, z),
                            Vec3::new(0.25, 3.6, 2.2),
                            Quat::IDENTITY,
                            glass.clone(),
                            false,
                            false,
                        );
                    }
                }
                piece(
                    Vec3::new(side * 28., 16.5, -14.),
                    Vec3::new(4., 3., 4.),
                    Quat::IDENTITY,
                    trim.clone(),
                    true,
                    false,
                );
                piece(
                    Vec3::new(side * 28., 18.2, -14.),
                    Vec3::new(5., 0.5, 5.),
                    Quat::IDENTITY,
                    roof.clone(),
                    true,
                    false,
                );
            }
            // Seven genuine arch openings: columns + curved voussoir segments.
            for bay in -3..=3 {
                let x = bay as f32 * 8.;
                for side in [-1., 1.] {
                    piece(
                        Vec3::new(x + side * 3.4, 3., 17.),
                        Vec3::new(1.2, 6., 1.8),
                        Quat::IDENTITY,
                        trim.clone(),
                        true,
                        false,
                    );
                    piece(
                        Vec3::new(x + side * 3.4, 0.9, 17.),
                        Vec3::new(1.7, 0.6, 2.2),
                        Quat::IDENTITY,
                        stone.clone(),
                        true,
                        false,
                    );
                }
                for segment in 0..12 {
                    let a = (segment as f32 + 0.5) / 12. * std::f32::consts::PI;
                    piece(
                        Vec3::new(x + 3.4 * a.cos(), 6. + 3.4 * a.sin(), 17.),
                        Vec3::new(1.0, 0.85, 1.8),
                        Quat::from_rotation_z(a + std::f32::consts::FRAC_PI_2),
                        trim.clone(),
                        true,
                        false,
                    );
                }
            }
            piece(
                Vec3::new(0., 10.2, 15.),
                Vec3::new(62., 1., 7.),
                Quat::IDENTITY,
                trim.clone(),
                true,
                false,
            );
            for i in 0..8 {
                piece(
                    Vec3::new(0., 0.1 + i as f32 * 0.12, 26.5 - i as f32 * 0.55),
                    Vec3::new(26., 0.2, 1.2),
                    Quat::IDENTITY,
                    trim.clone(),
                    true,
                    false,
                );
            }
        } else if site.kind == "park" {
            // Planted beds, promenade seats and a shaded pavilion furnish the
            // entire elongated park, keeping the central public path open.
            for side in [-1., 1.] {
                for (row, z) in [-100., -50., 50., 100.].into_iter().enumerate() {
                    piece(
                        Vec3::new(side * 23., 0.25, z),
                        Vec3::new(8., 0.5, 12.),
                        Quat::IDENTITY,
                        trim.clone(),
                        true,
                        false,
                    );
                    piece(
                        Vec3::new(side * 23., 0.52, z),
                        Vec3::new(7.5, 0.06, 11.5),
                        Quat::IDENTITY,
                        soil.clone(),
                        false,
                        false,
                    );
                    for k in 0..24 {
                        piece(
                            Vec3::new(
                                side * 23. - 2.8 + (k % 4) as f32 * 1.85,
                                0.72,
                                z - 4.5 + (k / 4) as f32 * 1.8,
                            ),
                            Vec3::new(0.55, 0.35, 0.55),
                            Quat::IDENTITY,
                            flowers[(k + row) % 3].clone(),
                            false,
                            true,
                        );
                    }
                    piece(
                        Vec3::new(side * 9., 0.6, z),
                        Vec3::new(4., 0.25, 1.),
                        Quat::IDENTITY,
                        roof.clone(),
                        true,
                        false,
                    );
                    piece(
                        Vec3::new(side * 9., 1.1, z + 0.5),
                        Vec3::new(4., 0.9, 0.15),
                        Quat::IDENTITY,
                        roof.clone(),
                        true,
                        false,
                    );
                    for x in [-1.5, 1.5] {
                        piece(
                            Vec3::new(side * 9. + x, 0.28, z),
                            Vec3::new(0.2, 0.55, 0.8),
                            Quat::IDENTITY,
                            art.dark.clone(),
                            true,
                            false,
                        );
                    }
                    piece(
                        Vec3::new(side * 5., 2., z + 12.),
                        Vec3::new(0.12, 4., 0.12),
                        Quat::IDENTITY,
                        art.dark.clone(),
                        true,
                        false,
                    );
                    piece(
                        Vec3::new(side * 5., 4.1, z + 12.),
                        Vec3::new(0.6, 0.5, 0.6),
                        Quat::IDENTITY,
                        trim.clone(),
                        false,
                        false,
                    );
                }
            }
            for x in [18., 30.] {
                for z in [124., 136.] {
                    piece(
                        Vec3::new(x, 2.5, z),
                        Vec3::new(0.4, 5., 0.4),
                        Quat::IDENTITY,
                        trim.clone(),
                        true,
                        false,
                    );
                }
            }
            for i in 0..9 {
                piece(
                    Vec3::new(24., 5.1, 124. + i as f32 * 1.5),
                    Vec3::new(14., 0.3, 0.65),
                    Quat::IDENTITY,
                    roof.clone(),
                    true,
                    false,
                );
            }
            // Open cross paths, circular fountain and seating under trees.
            for z in [-35., 35., 105.] {
                piece(
                    Vec3::new(90., 0.25, z),
                    Vec3::new(8., 0.5, 12.),
                    Quat::IDENTITY,
                    trim.clone(),
                    true,
                    false,
                );
                piece(
                    Vec3::new(90., 0.52, z),
                    Vec3::new(7.5, 0.06, 11.5),
                    Quat::IDENTITY,
                    soil.clone(),
                    false,
                    false,
                );
                for k in 0..24 {
                    piece(
                        Vec3::new(
                            87.2 + (k % 4) as f32 * 1.85,
                            0.72,
                            z - 4.5 + (k / 4) as f32 * 1.8,
                        ),
                        Vec3::new(0.55, 0.35, 0.55),
                        Quat::IDENTITY,
                        flowers[k % 3].clone(),
                        false,
                        true,
                    );
                }
                piece(
                    Vec3::new(75., 0.6, z),
                    Vec3::new(4., 0.25, 1.),
                    Quat::IDENTITY,
                    roof.clone(),
                    true,
                    false,
                );
                for x in [73.5, 76.5] {
                    piece(
                        Vec3::new(x, 0.28, z),
                        Vec3::new(0.2, 0.55, 0.8),
                        Quat::IDENTITY,
                        art.dark.clone(),
                        true,
                        false,
                    );
                }
                piece(
                    Vec3::new(75., 1.1, z + 0.5),
                    Vec3::new(4., 0.9, 0.15),
                    Quat::IDENTITY,
                    roof.clone(),
                    true,
                    false,
                );
            }
            piece(
                Vec3::new(0., 0.25, 0.),
                Vec3::new(8.5, 0.5, 8.5),
                Quat::IDENTITY,
                stone.clone(),
                true,
                true,
            );
            piece(
                Vec3::new(0., 0.55, 0.),
                Vec3::new(7.8, 0.12, 7.8),
                Quat::IDENTITY,
                water.clone(),
                false,
                true,
            );
            for i in 0..24 {
                let a = i as f32 / 24. * std::f32::consts::TAU;
                piece(
                    Vec3::new(8.1 * a.cos(), 0.8, 8.1 * a.sin()),
                    Vec3::new(2.2, 0.8, 0.6),
                    Quat::from_rotation_y(-a - std::f32::consts::FRAC_PI_2),
                    trim.clone(),
                    true,
                    false,
                );
            }
            piece(
                Vec3::new(0., 1.8, 0.),
                Vec3::new(0.9, 3., 0.9),
                Quat::IDENTITY,
                trim.clone(),
                true,
                true,
            );
            piece(
                Vec3::new(0., 3.2, 0.),
                Vec3::new(3., 0.35, 3.),
                Quat::IDENTITY,
                stone.clone(),
                true,
                true,
            );
            piece(
                Vec3::new(0., 4.2, 0.),
                Vec3::new(0.14, 2., 0.14),
                Quat::IDENTITY,
                water.clone(),
                false,
                true,
            );
            for side in [-1., 1.] {
                for z in [-20., 20.] {
                    piece(
                        Vec3::new(side * 16., 0.7, z),
                        Vec3::new(4., 0.25, 1.),
                        Quat::IDENTITY,
                        roof.clone(),
                        true,
                        false,
                    );
                    piece(
                        Vec3::new(side * 16., 1.2, z + 0.5),
                        Vec3::new(4., 0.9, 0.15),
                        Quat::IDENTITY,
                        roof.clone(),
                        true,
                        false,
                    );
                }
            }
        } else {
            piece(
                Vec3::new(0., 3.1, 0.),
                Vec3::new(1.3, 0.2, 7.),
                Quat::from_rotation_z(0.08),
                canopy.clone(),
                true,
                false,
            );
            for z in [-3., 3.] {
                piece(
                    Vec3::new(0.55, 1.5, z),
                    Vec3::new(0.12, 3., 0.12),
                    Quat::IDENTITY,
                    art.dark.clone(),
                    true,
                    false,
                );
            }
            for z in [-2., 1.5] {
                piece(
                    Vec3::new(0., 0.8, z),
                    Vec3::new(0.7, 0.12, 1.1),
                    Quat::IDENTITY,
                    trim.clone(),
                    true,
                    false,
                );
                piece(
                    Vec3::new(0., 0.4, z),
                    Vec3::new(0.12, 0.8, 0.12),
                    Quat::IDENTITY,
                    art.dark.clone(),
                    true,
                    false,
                );
                for side in [-1., 1.] {
                    piece(
                        Vec3::new(0., 0.45, z + side * 0.85),
                        Vec3::new(0.5, 0.15, 0.55),
                        Quat::IDENTITY,
                        roof.clone(),
                        true,
                        false,
                    );
                }
            }
        }
        if site.kind != "cafe" {
            // A terrain-conforming lawn sits below the paths, with original
            // broadleaf assets. Existing terrain remains the collision support.
            let scale = if site.kind == "city_hall" {
                HALL_SCALE
            } else {
                1.
            };
            let mut positions = Vec::new();
            let mut indices = Vec::new();
            for patch in &site.grounds {
                let base = positions.len() as u32;
                for [x, z] in patch.vertices() {
                    let world = Vec3::new(x, track.terrain_height(x, z) + 0.18, z);
                    positions.push(((rotation.inverse() * (world - center)) / scale).to_array());
                }
                for triangle in &patch.triangles {
                    let mut t = triangle.map(|i| i + base);
                    let [a, b, c] = t.map(|i| Vec3::from_array(positions[i as usize]));
                    if (b - a).cross(c - a).y < 0. {
                        t.swap(1, 2);
                    }
                    indices.extend(t);
                }
            }
            let mut lawn = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );
            lawn.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            lawn.insert_indices(Indices::U32(indices));
            lawn.compute_normals();
            let lawn_material = materials.add(Color::srgb(0.28, 0.40, 0.17));
            let lawn_entity = commands
                .spawn((
                    Mesh3d(meshes.add(lawn)),
                    MeshMaterial3d(lawn_material),
                    Transform::default(),
                ))
                .id();
            commands.entity(root).add_child(lawn_entity);
            let mut paving_positions = Vec::new();
            let mut paving_indices = Vec::new();
            for patch in &site.paving {
                let base = paving_positions.len() as u32;
                for [x, z] in patch.vertices() {
                    let p = (rotation.inverse()
                        * (Vec3::new(x, track.terrain_height(x, z) + 0.22, z) - center))
                        / scale;
                    paving_positions.push(p.to_array());
                }
                for triangle in &patch.triangles {
                    let mut t = triangle.map(|i| i + base);
                    let [a, b, c] = t.map(|i| Vec3::from_array(paving_positions[i as usize]));
                    if (b - a).cross(c - a).y < 0. {
                        t.swap(1, 2);
                    }
                    paving_indices.extend(t);
                }
            }
            let mut paving = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );
            paving.insert_attribute(Mesh::ATTRIBUTE_POSITION, paving_positions);
            paving.insert_indices(Indices::U32(paving_indices));
            paving.compute_normals();
            let paved = commands
                .spawn((
                    Mesh3d(meshes.add(paving)),
                    MeshMaterial3d(stone.clone()),
                    Transform::default(),
                ))
                .id();
            commands.entity(root).add_child(paved);
            if let Some(props) = &art.props {
                for (i, [x, z]) in site.trees.iter().copied().enumerate() {
                    let local = rotation.inverse()
                        * (Vec3::new(x, track.terrain_height(x, z) + 0.16, z) - center)
                        / scale;
                    let tree = props.fitted(
                        commands,
                        props.choose("broadleaf", i),
                        local,
                        Quat::IDENTITY,
                        Vec3::new(10., 13., 10.) / scale,
                        "tree",
                    );
                    commands.entity(root).add_child(tree);
                    contacts.push((
                        local + Vec3::Y * 2.4 / scale,
                        Quat::IDENTITY,
                        Collider::cylinder(2.4 / scale, 0.5 / scale),
                    ));
                }
            }

            if let Some(props) = &art.props
                && site.kind == "park"
            {
                for (i, p) in park_tree_positions(site).into_iter().enumerate() {
                    let mut local = Vec3::new(p.x, 0., p.y);
                    let world = center + rotation * local;
                    local.y = track.terrain_height(world.x, world.z) + 0.16 - center.y;
                    contacts.push((
                        local + Vec3::Y * 2.4,
                        Quat::IDENTITY,
                        Collider::cylinder(2.4, 0.5),
                    ));
                    let tree = props.fitted(
                        commands,
                        props.choose("broadleaf", i),
                        local,
                        Quat::from_rotation_y(i as f32),
                        Vec3::new(10., 13., 10.),
                        "tree",
                    );
                    commands.entity(root).add_child(tree);
                }
            }
        }
        if site.kind == "city_hall" {
            let vertices = vec![
                Vec3::new(-15., 21., 10.),
                Vec3::new(15., 21., 10.),
                Vec3::new(0., 27., 10.),
                Vec3::new(-15., 21., 13.),
                Vec3::new(15., 21., 13.),
                Vec3::new(0., 27., 13.),
            ];
            let mut pediment = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );
            pediment.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vertices.iter().map(|p| p.to_array()).collect::<Vec<_>>(),
            );
            pediment.insert_indices(Indices::U32(vec![
                0, 2, 1, 3, 4, 5, 0, 1, 4, 0, 4, 3, 1, 2, 5, 1, 5, 4, 2, 0, 3, 2, 3, 5,
            ]));
            pediment.duplicate_vertices();
            pediment.compute_flat_normals();
            let child = commands
                .spawn((
                    Mesh3d(meshes.add(pediment)),
                    MeshMaterial3d(trim.clone()),
                    Transform::default(),
                ))
                .id();
            commands.entity(root).add_child(child);
            contacts.push((
                Vec3::ZERO,
                Quat::IDENTITY,
                Collider::convex_hull(&vertices).expect("nondegenerate pediment"),
            ));
        }
        if site.kind == "city_hall" {
            super::hall_detail::spawn(commands, meshes, materials, root, images);
        }
        commands
            .entity(root)
            .insert((RigidBody::Fixed, Collider::compound(contacts), Obstacle));
    }
}
