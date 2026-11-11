use super::*;
use crate::world::Obstacle;
use bevy_rapier3d::prelude::*;

#[derive(Default)]
pub(super) struct Model {
    vertices: Vec<Vec3>,
    colors: Vec<[f32; 4]>,
    indices: Vec<[u32; 3]>,
}
impl Model {
    fn front_quad(&mut self, p: Vec3, size: Vec2, color: [f32; 3]) {
        let k = self.vertices.len() as u32;
        for (x, y) in [(-1., -1.), (-1., 1.), (1., 1.), (1., -1.)] {
            self.vertices
                .push(p + Vec3::new(x * size.x, y * size.y, 0.) * 0.5);
            self.colors.push([color[0], color[1], color[2], 1.]);
        }
        self.indices.extend([[k, k + 1, k + 2], [k, k + 2, k + 3]]);
    }
    pub(super) fn block(&mut self, p: Vec3, size: Vec3, color: [f32; 3]) {
        let f = |x, y, z| p + Vec3::new(x, y, z) * size * 0.5;
        for face in [
            [
                f(-1., -1., -1.),
                f(-1., 1., -1.),
                f(1., 1., -1.),
                f(1., -1., -1.),
            ],
            [
                f(1., -1., 1.),
                f(1., 1., 1.),
                f(-1., 1., 1.),
                f(-1., -1., 1.),
            ],
            [
                f(-1., -1., 1.),
                f(-1., 1., 1.),
                f(-1., 1., -1.),
                f(-1., -1., -1.),
            ],
            [
                f(1., -1., -1.),
                f(1., 1., -1.),
                f(1., 1., 1.),
                f(1., -1., 1.),
            ],
            [
                f(-1., 1., -1.),
                f(-1., 1., 1.),
                f(1., 1., 1.),
                f(1., 1., -1.),
            ],
            [
                f(-1., -1., 1.),
                f(-1., -1., -1.),
                f(1., -1., -1.),
                f(1., -1., 1.),
            ],
        ] {
            let k = self.vertices.len() as u32;
            self.vertices.extend(face);
            self.colors.extend([[color[0], color[1], color[2], 1.]; 4]);
            self.indices.extend([[k, k + 1, k + 2], [k, k + 2, k + 3]]);
        }
    }
    /// Original radial geometry shared by barrels, cactus limbs and deadwood.
    pub(super) fn rod(&mut self, a: Vec3, b: Vec3, radius: f32, color: [f32; 3]) {
        let q = Quat::from_rotation_arc(Vec3::Y, (b - a).normalize());
        let k = self.vertices.len() as u32;
        for center in [a, b] {
            for i in 0..12_u32 {
                let angle = i as f32 * std::f32::consts::TAU / 12.;
                self.vertices
                    .push(center + q * Vec3::new(angle.cos() * radius, 0., angle.sin() * radius));
                let shade = if i.is_multiple_of(2) { 1. } else { 0.88 };
                self.colors
                    .push([color[0] * shade, color[1] * shade, color[2] * shade, 1.]);
            }
        }
        self.vertices.extend([a, b]);
        self.colors.extend([[color[0], color[1], color[2], 1.]; 2]);
        for i in 0..12_u32 {
            let next = (i + 1) % 12;
            self.indices.extend([
                [k + i, k + 12 + i, k + next],
                [k + next, k + 12 + i, k + 12 + next],
                [k + 24, k + i, k + next],
                [k + 25, k + 12 + next, k + 12 + i],
            ]);
        }
    }
    pub(super) fn ellipsoid(&mut self, p: Vec3, radii: Vec3, color: [f32; 3], seed: f32) {
        let k = self.vertices.len() as u32;
        for latitude in 0..=6 {
            for longitude in 0..12 {
                let a = latitude as f32 * std::f32::consts::PI / 6.;
                let b = longitude as f32 * std::f32::consts::TAU / 12.;
                let noise =
                    1. + seed * 0.08 * a.sin() * ((longitude * 19 + latitude * 7) as f32).sin();
                self.vertices.push(
                    p + Vec3::new(a.sin() * b.cos(), a.cos(), a.sin() * b.sin()) * radii * noise,
                );
                let shade = 0.86 + 0.14 * (longitude as f32 * 1.7 + latitude as f32).sin().abs();
                self.colors
                    .push([color[0] * shade, color[1] * shade, color[2] * shade, 1.]);
            }
        }
        for y in 0..6_u32 {
            for x in 0..12_u32 {
                let a = k + y * 12 + x;
                let b = k + y * 12 + (x + 1) % 12;
                if y > 0 {
                    self.indices.push([a, b, a + 12]);
                }
                if y < 5 {
                    self.indices.push([b, b + 12, a + 12]);
                }
            }
        }
    }
    pub(super) fn cone(&mut self, base: Vec3, radius: f32, height: f32, color: [f32; 3]) {
        let k = self.vertices.len() as u32;
        self.vertices.extend([base, base + Vec3::Y * height]);
        self.colors.extend([[color[0], color[1], color[2], 1.]; 2]);
        for i in 0..8 {
            let angle = i as f32 * std::f32::consts::TAU / 8.;
            self.vertices
                .push(base + Vec3::new(angle.cos(), 0., angle.sin()) * radius);
            self.colors.push([color[0], color[1], color[2], 1.]);
        }
        for i in 0..8 {
            let a = k + 2 + i;
            let b = k + 2 + (i + 1) % 8;
            self.indices.extend([[k + 1, b, a], [k, a, b]]);
        }
    }
    pub(super) fn hull(&self) -> Collider {
        Collider::convex_hull(&self.vertices).expect("original scenery convex hull")
    }
    pub(super) fn mesh(self) -> Mesh {
        let mut m = crate::environment::mesh(&self.vertices, &self.indices);
        m.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors);
        m
    }
}
fn root(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    model: Model,
    p: Vec3,
    q: Quat,
    collider: Collider,
) -> Entity {
    commands
        .spawn((
            Mesh3d(meshes.add(model.mesh())),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(p).with_rotation(q),
            RigidBody::Fixed,
            collider,
            Obstacle,
            HillsSolid,
        ))
        .id()
}
fn clear(occupied: &[(Vec3, f32)], p: Vec3, radius: f32) -> bool {
    occupied
        .iter()
        .all(|(other, r)| (p - *other).with_y(0.).length() > radius + r + 2.)
}
pub(super) fn site(
    track: &Track,
    route: usize,
    s: f32,
    lane: f32,
    radius: f32,
) -> Option<(Vec3, Quat)> {
    let (mut p, q) = track.sample_route(route, s, lane);
    for route in 0..2 {
        let (d, _, distance) = track.project_route(route, p, s);
        if distance < track.half_width(d) + radius + 3. {
            return None;
        }
    }
    p.y = super::geometry::ground_height(track, p.x, p.z);
    let half = track.half_width(s);
    if (half..=30.).contains(&lane.abs()) {
        // Match the actual shoulder strip, including the town boardwalk actors
        // and parked truck tires, rather than placing them on the lower field.
        let (inner, iq) = track.sample_route(route, s, lane.signum() * half);
        let (outer, oq) = track.sample_route(route, s, lane.signum() * 30.);
        let weight = (lane.abs() - half) / (30. - half);
        p.y = (inner - iq * Vec3::Y * 0.08)
            .y
            .lerp((outer - oq * Vec3::Y * 0.08).y, weight);
    }
    let f = (q * Vec3::Z).with_y(0.).normalize();
    Some((p, Quat::from_rotation_y(f.x.atan2(f.z))))
}
// Reserve actual frontage corners rather than a large circular empty lot.
fn frontage_site(
    track: &Track,
    route: usize,
    s: f32,
    lane: f32,
    western: bool,
) -> Option<(Vec3, Quat)> {
    let (p, road) = site(track, route, s, lane, 0.)?;
    let q = road
        * Quat::from_rotation_y(if lane < 0. {
            std::f32::consts::FRAC_PI_2
        } else {
            -std::f32::consts::FRAC_PI_2
        });
    let half_width = if western { 8.5 } else { 10. };
    let (front, back) = if western { (-10., 6.5) } else { (-11.5, 11.5) };
    for x in [-half_width, 0., half_width] {
        for z in [front, 0., back] {
            let point = p + q * Vec3::new(x, 0., z);
            if super::mounted::reserved(track, point, 1.) {
                return None;
            }
            for branch in 0..2 {
                let (d, _, distance) = track.project_route(branch, point, s);
                if distance < track.half_width(d) + 1.6 {
                    return None;
                }
            }
        }
    }
    Some((p, q))
}
fn sign_letters(model: &mut Model, label: &str, center: Vec3) {
    let scale = 0.10;
    let left = -(label.len() as f32 * 6. - 1.) * scale / 2.;
    for (letter, c) in label.chars().enumerate() {
        let rows = match c {
            'A' => [14, 17, 17, 31, 17, 17, 17],
            'B' => [30, 17, 17, 30, 17, 17, 30],
            'E' => [31, 16, 16, 30, 16, 16, 31],
            'H' => [17, 17, 17, 31, 17, 17, 17],
            'I' => [31, 4, 4, 4, 4, 4, 31],
            'K' => [17, 18, 20, 24, 20, 18, 17],
            'L' => [16, 16, 16, 16, 16, 16, 31],
            'N' => [17, 25, 25, 21, 19, 19, 17],
            'O' => [14, 17, 17, 17, 17, 17, 14],
            'R' => [30, 17, 17, 30, 20, 18, 17],
            'S' => [15, 16, 16, 14, 1, 1, 30],
            'T' => [31, 4, 4, 4, 4, 4, 4],
            'V' => [17, 17, 17, 17, 17, 10, 4],
            'Y' => [17, 17, 10, 4, 4, 4, 4],
            _ => [0; 7],
        };
        for (row, bits) in rows.into_iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    model.front_quad(
                        center
                            + Vec3::new(
                                // The storefront faces local -Z: a viewer's
                                // right is local -X. Mirror layout positions,
                                // leaving each front-facing quad's winding intact.
                                -(left + (letter as f32 * 6. + col as f32) * scale),
                                (3. - row as f32) * scale,
                                0.,
                            ),
                        Vec2::splat(scale * 0.82),
                        [0.18, 0.10, 0.045],
                    );
                }
            }
        }
    }
}
#[allow(clippy::too_many_arguments)]
fn building(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    track: &Track,
    p: Vec3,
    q: Quat,
    index: usize,
    western: bool,
) -> bool {
    let mut model = Model::default();
    let width = if western {
        12. + (index % 3) as f32 * 2.
    } else {
        19.
    };
    let depth = if western { 12. } else { 22. };
    let height = if western {
        5. + (index % 2) as f32 * 3.
    } else {
        10. + (index % 5) as f32 * 4.
    };
    let corner_heights = [-1., 1.]
        .into_iter()
        .flat_map(|x| {
            [-1., 1.].map(move |z| {
                let v = p + q * Vec3::new(x * width / 2., 0., z * depth / 2.);
                track.terrain_height(v.x, v.z)
            })
        })
        .collect::<Vec<_>>();
    let floor = corner_heights.iter().copied().fold(p.y, f32::max) + 0.12;
    let bottom = corner_heights.iter().copied().fold(p.y, f32::min) - 0.6;
    let p = p.with_y(floor);
    let wood = [
        [0.48, 0.27, 0.13],
        [0.64, 0.44, 0.26],
        [0.42, 0.48, 0.43],
        [0.65, 0.36, 0.24],
    ][index % 4];
    model.block(
        Vec3::new(0., -(floor - bottom) * 0.5, 0.),
        Vec3::new(width + 0.5, floor - bottom, depth + 0.5),
        [0.38, 0.32, 0.25],
    );
    model.block(Vec3::Y * height / 2., Vec3::new(width, height, depth), wood);
    model.block(
        Vec3::Y * (height + 0.15),
        Vec3::new(width + 0.5, 0.3, depth + 0.5),
        [0.22, 0.20, 0.17],
    );
    if western {
        // False front, projecting boardwalk/porch and exposed timber posts.
        model.block(
            Vec3::new(0., height + 0.9, -depth / 2. - 0.15),
            Vec3::new(width + 0.3, 1.8, 0.4),
            wood,
        );
        model.block(
            Vec3::new(0., height + 1.8, -depth / 2. - 0.2),
            Vec3::new(width + 0.8, 0.25, 0.65),
            [0.24, 0.16, 0.09],
        );
        model.block(
            Vec3::new(0., 0.16, -depth / 2. - 1.5),
            Vec3::new(width + 1., 0.32, 3.),
            [0.48, 0.35, 0.22],
        );
        model.block(
            Vec3::new(0., 3.1, -depth / 2. - 1.5),
            Vec3::new(width + 1., 0.25, 3.4),
            [0.37, 0.23, 0.12],
        );
        for x in [-width / 2., 0., width / 2.] {
            model.block(
                Vec3::new(x, 1.55, -depth / 2. - 2.7),
                Vec3::new(0.2, 3.1, 0.2),
                [0.30, 0.19, 0.1],
            );
        }
        // Board seams and contrasting sign panels are geometry, not borrowed sprites.
        for y in 0..(height / 0.42) as usize {
            model.block(
                Vec3::new(0., y as f32 * 0.42, -depth / 2. - 0.03),
                Vec3::new(width, 0.025, 0.035),
                [0.26, 0.18, 0.1],
            );
        }
        model.block(
            Vec3::new(0., height + 0.7, -depth / 2. - 0.38),
            Vec3::new(width * 0.7, 0.8, 0.08),
            [0.83, 0.70, 0.46],
        );
        sign_letters(
            &mut model,
            ["SALOON", "HOTEL", "STORE", "BANK", "LIVERY"][index % 5],
            Vec3::new(0., height + 0.7, -depth / 2. - 0.43),
        );
        // Original shop-specific goods are batched into the existing storefront.
        // All lie inside the existing covered boardwalk collision envelope.
        for goods in 0..3 {
            let x = -width * 0.30 + goods as f32 * 0.72;
            let p = Vec3::new(x, 0.62, -depth / 2. - 1.2);
            if index.is_multiple_of(2) {
                model.block(p, Vec3::new(0.62, 0.65, 0.65), [0.48, 0.33, 0.17]);
                for y in [-0.22, 0.22] {
                    model.block(
                        p + Vec3::new(0., y, -0.335),
                        Vec3::new(0.65, 0.08, 0.04),
                        [0.27, 0.19, 0.11],
                    );
                }
            } else {
                model.ellipsoid(p, Vec3::new(0.3, 0.42, 0.3), [0.61, 0.52, 0.33], 0.5);
            }
        }
        for x in [-width * 0.43, width * 0.43] {
            model.block(
                Vec3::new(x, 2.6, -depth / 2. - 0.7),
                Vec3::new(0.23, 0.42, 0.23),
                [0.72, 0.58, 0.28],
            );
            model.block(
                Vec3::new(x, 2.87, -depth / 2. - 0.7),
                Vec3::new(0.34, 0.1, 0.34),
                [0.16, 0.17, 0.16],
            );
        }
        // A coopered barrel, open water trough and hitching rail make the street
        // read at riding height; all stay inside the existing14m reserved lot.
        let barrel = Vec3::new(width * 0.32, 0.32, -depth / 2. - 1.4);
        model.rod(barrel, barrel + Vec3::Y * 1.05, 0.48, [0.44, 0.27, 0.12]);
        for y in [0.16, 0.84] {
            model.rod(
                barrel + Vec3::Y * y,
                barrel + Vec3::Y * (y + 0.09),
                0.49,
                [0.17, 0.18, 0.16],
            );
        }
        let trough = Vec3::new(-width * 0.30, 0., -depth / 2. - 3.4);
        for z in [-0.43, 0.43] {
            model.block(
                trough + Vec3::new(0., 0.55, z),
                Vec3::new(2.3, 0.85, 0.14),
                [0.40, 0.28, 0.16],
            );
        }
        for x in [-1.08, 1.08] {
            model.block(
                trough + Vec3::new(x, 0.55, 0.),
                Vec3::new(0.14, 0.85, 0.86),
                [0.40, 0.28, 0.16],
            );
        }
        for x in [-0.75, 0.75] {
            model.block(
                trough + Vec3::new(x, 0.025, 0.),
                Vec3::new(0.24, 0.45, 0.75),
                [0.28, 0.19, 0.10],
            );
        }
        model.block(
            trough + Vec3::new(0., 0.74, 0.),
            Vec3::new(2.05, 0.025, 0.7),
            [0.24, 0.38, 0.37],
        );
        for x in [width * 0.30 - 1., width * 0.30 + 1.] {
            model.rod(
                Vec3::new(x, -0.2, -depth / 2. - 3.4),
                Vec3::new(x, 1.15, -depth / 2. - 3.4),
                0.11,
                [0.31, 0.20, 0.1],
            );
        }
        model.rod(
            Vec3::new(width * 0.30 - 1.2, 1.05, -depth / 2. - 3.4),
            Vec3::new(width * 0.30 + 1.2, 1.05, -depth / 2. - 3.4),
            0.12,
            [0.36, 0.23, 0.12],
        );
        // A substantial display fills the empty apron between porch and road.
        // Its nearest edge remains inside the frontage_site road-clear samples.
        let front = -depth / 2. - 2.8;
        match index % 4 {
            0 => {
                // General store: a tall produce counter and stacked freight.
                model.block(
                    Vec3::new(0., 0.68, front),
                    Vec3::new(2.8, 1.35, 1.15),
                    [0.48, 0.31, 0.15],
                );
                for x in [-0.9, 0., 0.9] {
                    model.ellipsoid(
                        Vec3::new(x, 1.53, front),
                        Vec3::new(0.38, 0.34, 0.38),
                        [0.69, 0.44, 0.16],
                        0.4,
                    );
                }
            }
            1 => {
                // Saloon: two seats and a broad table, clear of the porch posts.
                for x in [-1.5, 1.5] {
                    model.block(
                        Vec3::new(x, 0.6, front),
                        Vec3::new(0.9, 0.15, 1.2),
                        [0.55, 0.36, 0.17],
                    );
                    for z in [-0.4, 0.4] {
                        model.block(
                            Vec3::new(x, 0.26, front + z),
                            Vec3::new(0.14, 0.52, 0.14),
                            [0.39, 0.24, 0.12],
                        );
                    }
                    model.block(
                        Vec3::new(x, 0.43, front + 0.4),
                        Vec3::new(0.9, 0.85, 0.16),
                        [0.39, 0.24, 0.12],
                    );
                }
                model.block(
                    Vec3::new(0., 0.85, front),
                    Vec3::new(1.8, 0.18, 1.05),
                    [0.50, 0.32, 0.17],
                );
            }
            2 => {
                // Freight exchange: distinct two-tier crates at riding height.
                for (x, y) in [(-0.95, 0.57), (0.95, 0.57), (0., 1.7)] {
                    model.block(
                        Vec3::new(x, y, front),
                        Vec3::new(1.75, 1.1, 1.2),
                        [0.58, 0.40, 0.21],
                    );
                    model.block(
                        Vec3::new(x, y, front - 0.61),
                        Vec3::new(1.78, 0.1, 0.04),
                        [0.26, 0.18, 0.1],
                    );
                }
            }
            _ => {
                // Livery: hay bales and a raised feed rack.
                for x in [-1.05, 1.05] {
                    model.block(
                        Vec3::new(x, 0.53, front),
                        Vec3::new(1.95, 1.05, 1.35),
                        [0.67, 0.53, 0.25],
                    );
                    for y in [0.2, 0.85] {
                        model.block(
                            Vec3::new(x, y, front - 0.69),
                            Vec3::new(1.97, 0.09, 0.05),
                            [0.37, 0.27, 0.14],
                        );
                    }
                }
            }
        }
    }
    for level in 0..(height / 3.) as usize {
        for side in [-1., 1.] {
            let x = side * width * 0.28;
            model.block(
                Vec3::new(x, 1.8 + level as f32 * 3., -depth / 2. - 0.09),
                Vec3::new(2.2, 1.6, 0.12),
                [0.15, 0.22, 0.23],
            );
            model.block(
                Vec3::new(x, 1.8 + level as f32 * 3., -depth / 2. - 0.17),
                Vec3::new(0.10, 1.7, 0.12),
                [0.74, 0.65, 0.48],
            );
            model.block(
                Vec3::new(x, 1.8 + level as f32 * 3., -depth / 2. - 0.17),
                Vec3::new(2.3, 0.10, 0.12),
                [0.74, 0.65, 0.48],
            );
        }
    }
    model.block(
        Vec3::new(0., 1.25, -depth / 2. - 0.08),
        Vec3::new(1.3, 2.5, 0.15),
        [0.21, 0.15, 0.10],
    );
    let mut solids = vec![(
        Vec3::Y * (height - (floor - bottom)) / 2.,
        Quat::IDENTITY,
        Collider::cuboid(width / 2., (height + floor - bottom) / 2., depth / 2.),
    )];
    if western {
        solids.push((
            Vec3::new(0., 0.16, -depth / 2. - 1.5),
            Quat::IDENTITY,
            Collider::cuboid((width + 1.) / 2., 0.16, 1.5),
        ));
    }
    if western {
        let front = -depth / 2. - 2.8;
        let display = match index % 4 {
            0 => (Vec3::new(0., 0.8, front), Vec3::new(1.45, 0.8, 0.62)),
            1 => (Vec3::new(0., 0.7, front), Vec3::new(2.05, 0.7, 0.65)),
            2 => (Vec3::new(0., 1.1, front), Vec3::new(1.85, 1.1, 0.65)),
            _ => (Vec3::new(0., 0.6, front), Vec3::new(2.1, 0.6, 0.72)),
        };
        solids.push((
            display.0,
            Quat::IDENTITY,
            Collider::cuboid(display.1.x, display.1.y, display.1.z),
        ));
        for goods in 0..3 {
            solids.push((
                Vec3::new(-width * 0.30 + goods as f32 * 0.72, 0.62, -depth / 2. - 1.2),
                Quat::IDENTITY,
                Collider::cuboid(0.32, 0.42, 0.34),
            ));
        }
        solids.extend([
            (
                Vec3::new(width * 0.32, 0.845, -depth / 2. - 1.4),
                Quat::IDENTITY,
                Collider::cylinder(0.525, 0.49),
            ),
            (
                Vec3::new(-width * 0.30, 0.4, -depth / 2. - 3.4),
                Quat::IDENTITY,
                Collider::cuboid(1.15, 0.6, 0.5),
            ),
            (
                Vec3::new(width * 0.30, 1.05, -depth / 2. - 3.4),
                Quat::IDENTITY,
                Collider::cuboid(1.2, 0.12, 0.12),
            ),
            (
                Vec3::new(width * 0.30 - 1., 0.475, -depth / 2. - 3.4),
                Quat::IDENTITY,
                Collider::cylinder(0.675, 0.11),
            ),
            (
                Vec3::new(width * 0.30 + 1., 0.475, -depth / 2. - 3.4),
                Quat::IDENTITY,
                Collider::cylinder(0.675, 0.11),
            ),
        ]);
    }
    // This second street layer uses only the existing storefront reservation.
    // Validate its actual outer edge against both branches and mounted paths;
    // tight bends omit the layer rather than pushing collision into the road.
    let street_group = western
        && [-3.7, 0., 3.7].into_iter().all(|x| {
            let point = p + q * Vec3::new(x, 0., -10.9);
            !super::mounted::reserved(track, point, 1.)
                && (0..2).all(|route| {
                    let (s, _, distance) = track.project_route(route, point, 6000.);
                    distance > track.half_width(s) + 2.2
                })
        });
    if street_group {
        let z = -10.;
        match index % 3 {
            0 => {
                // Striped canvas stall: a large readable silhouette above the
                // goods, with a solid roof and grounded front posts.
                for strip in 0..6 {
                    model.block(
                        Vec3::new(-2.5 + strip as f32, 3.35, -9.7),
                        Vec3::new(1., 0.12, 2.2),
                        if strip % 2 == 0 {
                            [0.72, 0.30, 0.16]
                        } else {
                            [0.80, 0.67, 0.42]
                        },
                    );
                }
                solids.push((
                    Vec3::new(0., 3.35, -9.7),
                    Quat::IDENTITY,
                    Collider::cuboid(3., 0.06, 1.1),
                ));
                for x in [-2.5, 2.5] {
                    lot_block(
                        &mut model,
                        &mut solids,
                        Vec3::new(x, 1.65, -10.65),
                        Vec3::new(0.16, 3.3, 0.16),
                        [0.40, 0.27, 0.14],
                    );
                }
            }
            1 => {
                // Low freight handcart, with wheels that stay resident solids.
                lot_block(
                    &mut model,
                    &mut solids,
                    Vec3::new(0., 1.05, z),
                    Vec3::new(2.7, 0.5, 1.5),
                    [0.52, 0.33, 0.16],
                );
                for x in [-1.35, 1.35] {
                    for dz in [-0.5, 0.5] {
                        let wheel = Vec3::new(x, 0.55, z + dz);
                        model.ellipsoid(wheel, Vec3::new(0.16, 0.55, 0.55), [0.18, 0.15, 0.11], 0.);
                        solids.push((
                            wheel,
                            Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                            Collider::cylinder(0.16, 0.55),
                        ));
                    }
                }
                for x in [-0.95, 0.95] {
                    model.rod(
                        Vec3::new(x, 1., z - 0.7),
                        Vec3::new(x, 1., z - 1.05),
                        0.09,
                        [0.38, 0.24, 0.12],
                    );
                }
            }
            _ => {
                // Hitch rail and visible barrel pair leave an open approach.
                for x in [-2.6, 2.6] {
                    lot_block(
                        &mut model,
                        &mut solids,
                        Vec3::new(x, 0.85, z),
                        Vec3::new(0.18, 1.7, 0.18),
                        [0.38, 0.23, 0.11],
                    );
                }
                lot_block(
                    &mut model,
                    &mut solids,
                    Vec3::new(0., 1.45, z),
                    Vec3::new(5.35, 0.18, 0.18),
                    [0.48, 0.30, 0.16],
                );
                for x in [-3.35, 3.35] {
                    let a = Vec3::new(x, 0.05, z + 0.3);
                    model.rod(a, a + Vec3::Y * 1.05, 0.38, [0.43, 0.27, 0.13]);
                    solids.push((
                        a + Vec3::Y * 0.525,
                        Quat::IDENTITY,
                        Collider::cylinder(0.525, 0.38),
                    ));
                }
            }
        }
    }
    let e = root(
        commands,
        meshes,
        material,
        model,
        p,
        q,
        Collider::compound(solids),
    );
    commands.entity(e).insert((
        HillsBuilding,
        Name::new(format!(
            "Hills {} {index}",
            if western {
                "western storefront"
            } else {
                "city building"
            }
        )),
    ));
    street_group
}
fn horse(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    p: Vec3,
    q: Quat,
    index: usize,
) {
    let mut m = Model::default();
    let hide = if index.is_multiple_of(2) {
        [0.36, 0.18, 0.08]
    } else {
        [0.65, 0.49, 0.30]
    };
    m.block(Vec3::new(0., 1.35, 0.), Vec3::new(0.70, 0.85, 1.9), hide);
    m.block(Vec3::new(0., 1.8, 0.65), Vec3::new(0.48, 1., 0.55), hide);
    m.block(Vec3::new(0., 2.18, 1.05), Vec3::new(0.42, 0.45, 0.90), hide);
    for x in [-0.26, 0.26] {
        for z in [-0.65, 0.6] {
            m.block(Vec3::new(x, 0.57, z), Vec3::new(0.17, 1.14, 0.18), hide);
            m.block(
                Vec3::new(x, 0.10, z + 0.04),
                Vec3::new(0.22, 0.20, 0.30),
                [0.11, 0.08, 0.05],
            );
        }
    }
    for x in [-0.16, 0.16] {
        m.block(Vec3::new(x, 2.51, 0.78), Vec3::new(0.11, 0.28, 0.15), hide);
    }
    m.block(
        Vec3::new(0., 1.85, 0.39),
        Vec3::new(0.17, 1., 0.20),
        [0.10, 0.07, 0.04],
    );
    m.block(
        Vec3::new(0., 0.93, -1.05),
        Vec3::new(0.20, 1.2, 0.25),
        [0.10, 0.07, 0.04],
    );
    m.block(
        Vec3::new(0., 1.82, -0.1),
        Vec3::new(0.83, 0.20, 0.72),
        [0.23, 0.12, 0.06],
    );
    let e = root(
        commands,
        meshes,
        material,
        m,
        p,
        q,
        Collider::compound(vec![(
            Vec3::Y * 1.2,
            Quat::IDENTITY,
            Collider::cuboid(0.38, 1.2, 1.25),
        )]),
    );
    commands
        .entity(e)
        .insert((HillsHorse, Name::new("Hills original saddled horse")));
}
/// Coherent backlots occupy the spaces between and behind the two main streets.
/// Each is one batched mesh/compound collider; no geometry appears in update.
#[derive(Component)]
struct TownLot;
#[derive(Resource, Default, serde::Serialize)]
pub struct TownDetailCensus {
    pub backlots: usize,
    pub central_ranches: usize,
    pub median_courtyards: usize,
    pub median_review_band: usize,
    pub outer_fields: usize,
    pub field_cells: usize,
    pub roadside_groups: usize,
    pub outer_shrubs: usize,
    pub reused_props: usize,
}
fn lot_block(
    m: &mut Model,
    solids: &mut Vec<(Vec3, Quat, Collider)>,
    p: Vec3,
    size: Vec3,
    color: [f32; 3],
) {
    m.block(p, size, color);
    solids.push((
        p,
        Quat::IDENTITY,
        Collider::cuboid(size.x / 2., size.y / 2., size.z / 2.),
    ));
}
#[derive(Component)]
struct TownGround(bool);
#[allow(clippy::too_many_arguments)]
fn town_ground(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    track: &Track,
    p: Vec3,
    q: Quat,
    size: Vec2,
    crop: bool,
) {
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    let mut uv = Vec::new();
    let nx = (size.x / 2.).ceil() as u32;
    let nz = (size.y / 2.).ceil() as u32;
    for z in 0..=nz {
        for x in 0..=nx {
            let local = Vec3::new(
                (x as f32 / nx as f32 - 0.5) * size.x,
                0.,
                (z as f32 / nz as f32 - 0.5) * size.y,
            );
            let world = p + q * local;
            vertices.push(
                Vec3::new(
                    world.x,
                    super::geometry::ground_height(track, world.x, world.z)
                        + if crop { 0.075 } else { 0.045 },
                    world.z,
                ) - p,
            );
            uv.push([local.x / 5., local.z / 5.]);
        }
    }
    for z in 0..nz {
        for x in 0..nx {
            let a = z * (nx + 1) + x;
            triangles.extend([[a, a + nx + 1, a + 1], [a + 1, a + nx + 1, a + nx + 2]]);
        }
    }
    let mut mesh = crate::environment::mesh(&vertices, &triangles);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    commands.spawn((
        Name::new("Hills textured town parcel"),
        TownGround(crop),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(p),
    ));
}
fn town_materials(commands: &mut Commands) {
    commands.queue(|world: &mut World| {
        let Some(mut images) = world.get_resource_mut::<Assets<Image>>() else {
            return;
        };
        let texture =
            crate::world_gen::metropolis::geometry::landscape::ground_texture(&mut images);
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let soil = materials.add(
            crate::world_gen::metropolis::geometry::landscape::ground_material(
                Color::srgb(0.39, 0.31, 0.20),
                &texture,
            ),
        );
        let crop = materials.add(
            crate::world_gen::metropolis::geometry::landscape::ground_material(
                Color::srgb(0.37, 0.43, 0.18),
                &texture,
            ),
        );
        for (tag, mut material) in world
            .query::<(&TownGround, &mut MeshMaterial3d<StandardMaterial>)>()
            .iter_mut(world)
        {
            material.0 = if tag.0 { crop.clone() } else { soil.clone() };
        }
    });
}
#[allow(clippy::too_many_arguments)]
fn town_asset(
    commands: &mut Commands,
    art: &Art,
    p: Vec3,
    q: Quat,
    name: &str,
    size: Vec3,
    census: &mut TownDetailCensus,
) {
    let Some(props) = &art.props else { return };
    let root = commands
        .spawn((
            Name::new(format!("Hills ranch {name}")),
            Transform::from_translation(p).with_rotation(q),
            Visibility::default(),
            Collider::compound(vec![(
                Vec3::Y * size.y / 2.,
                Quat::IDENTITY,
                Collider::cuboid(size.x / 2., size.y / 2., size.z / 2.),
            )]),
            Obstacle,
            HillsSolid,
        ))
        .id();
    let visual = props.fitted(
        commands,
        props.named(name),
        Vec3::ZERO,
        Quat::IDENTITY,
        size,
        "hills-town-ranch",
    );
    commands.entity(root).add_child(visual);
    census.reused_props += 1;
}
#[allow(clippy::too_many_arguments)]
fn central_ranches(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    art: &Art,
    track: &Track,
    occupied: &mut Vec<(Vec3, f32)>,
    census: &mut TownDetailCensus,
) {
    // These are derived from the two actual road branches, not guessed offsets.
    for (index, station) in (5700..7310).step_by(75).enumerate() {
        let (a, qa) = track.sample_route(0, station as f32, 0.);
        let (b, qb) = track.sample_route(1, station as f32, 0.);
        let mut p = (a + b) * 0.5;
        p.y = super::geometry::ground_height(track, p.x, p.z);
        let f = (qa * Vec3::Z + qb * Vec3::Z).with_y(0.).normalize();
        let q = Quat::from_rotation_y(f.x.atan2(f.z));
        // A compact 20-by-30 m central parcel can fit between retained storefronts.
        let radius = 19.;
        if !clear(occupied, p, radius) || super::mounted::reserved(track, p, radius + 3.) {
            continue;
        }
        if (0..2).any(|route| {
            let (s, _, d) = track.project_route(route, p, station as f32);
            d < track.half_width(s) + radius + 3.
        }) {
            continue;
        }
        let at = |x: f32, z: f32| {
            let w = p + q * Vec3::new(x, 0., z);
            w.with_y(super::geometry::ground_height(track, w.x, w.z))
        };
        let heights = [
            at(-10., -15.).y,
            at(-10., 15.).y,
            at(10., -15.).y,
            at(10., 15.).y,
        ];
        if heights.iter().copied().fold(f32::NEG_INFINITY, f32::max)
            - heights.iter().copied().fold(f32::INFINITY, f32::min)
            > 1.6
        {
            continue;
        }
        occupied.push((p, radius));
        census.central_ranches += 1;
        town_ground(
            commands,
            meshes,
            material,
            track,
            p,
            q,
            Vec2::new(20., 30.),
            false,
        );
        let mut m = Model::default();
        let mut solids = Vec::new();
        if index.is_multiple_of(2) {
            town_asset(
                commands,
                art,
                at(-4., -3.),
                q,
                "BRNGRN2",
                Vec3::new(9., 6.7, 13.1),
                census,
            );
            town_asset(
                commands,
                art,
                at(6.5, -7.),
                q,
                "WINDVN2",
                Vec3::new(3.1, 7.2, 1.75),
                census,
            );
            for z in [4., 7., 10.] {
                town_asset(
                    commands,
                    art,
                    at(5.5, z),
                    q,
                    "HAYTRIP2",
                    Vec3::new(3.5, 0.93, 1.84),
                    census,
                );
            }
            for x in [-7., -4., -1.] {
                let c = q.inverse() * (at(x, 11.) - p);
                lot_block(
                    &mut m,
                    &mut solids,
                    c + Vec3::Y * 0.8,
                    Vec3::new(2., 1.6, 2.),
                    [0.59, 0.43, 0.22],
                );
            }
        } else {
            // Productive orchard/vegetable plot and a conspicuous shared water tower.
            town_ground(
                commands,
                meshes,
                material,
                track,
                at(-3., 2.),
                q,
                Vec2::new(12., 24.),
                true,
            );
            for x in [-7., -3., 1.] {
                for z in [-6., 0., 6., 12.] {
                    let c = q.inverse() * (at(x, z) - p);
                    m.rod(c, c + Vec3::Y * 2.7, 0.16, [0.32, 0.22, 0.12]);
                    solids.push((
                        c + Vec3::Y * 1.3,
                        Quat::IDENTITY,
                        Collider::cylinder(1.3, 0.17),
                    ));
                    m.ellipsoid(
                        c + Vec3::Y * 3.,
                        Vec3::new(1.7, 1.8, 1.7),
                        [0.24, 0.35, 0.14],
                        0.5,
                    );
                }
            }
            let c = q.inverse() * (at(6., -7.) - p);
            for x in [-1.5, 1.5] {
                for z in [-1.5, 1.5] {
                    lot_block(
                        &mut m,
                        &mut solids,
                        c + Vec3::new(x, 3., z),
                        Vec3::new(0.3, 6., 0.3),
                        [0.33, 0.23, 0.14],
                    );
                }
            }
            lot_block(
                &mut m,
                &mut solids,
                c + Vec3::Y * 6.8,
                Vec3::new(4.4, 2.4, 4.4),
                [0.43, 0.32, 0.18],
            );
            for y in [5.7, 6.8, 7.9] {
                m.block(
                    c + Vec3::Y * y,
                    Vec3::new(4.5, 0.12, 4.5),
                    [0.23, 0.24, 0.22],
                );
            }
            town_asset(
                commands,
                art,
                at(6., 5.),
                q,
                "BENCH12",
                Vec3::new(3., 1.4, 1.15),
                census,
            );
        }
        // Fences only along the sides leave both ends walkable.
        for x in [-9.7, 9.7] {
            for z in [-12., -6., 0., 6., 12.] {
                let c = q.inverse() * (at(x, z) - p);
                lot_block(
                    &mut m,
                    &mut solids,
                    c + Vec3::Y * 0.7,
                    Vec3::new(0.18, 1.4, 0.18),
                    [0.43, 0.29, 0.15],
                );
                if z < 12. {
                    for y in [0.55, 1.1] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            c + Vec3::new(0., y, 3.),
                            Vec3::new(0.14, 0.12, 6.),
                            [0.57, 0.41, 0.22],
                        );
                    }
                }
            }
        }
        let e = root(
            commands,
            meshes,
            material,
            m,
            p,
            q,
            Collider::compound(solids),
        );
        commands
            .entity(e)
            .insert((TownLot, Name::new("Hills central ranch/orchard")));
    }
}

#[allow(clippy::too_many_arguments)]
fn median_courtyards(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    track: &Track,
    occupied: &mut Vec<(Vec3, f32)>,
    census: &mut TownDetailCensus,
) {
    for (index, station) in (5710..7330).step_by(23).enumerate() {
        let (a, qa) = track.sample_route(0, station as f32, 0.);
        let (b, qb) = track.sample_route(1, station as f32, 0.);
        let f = (qa * Vec3::Z + qb * Vec3::Z).with_y(0.).normalize();
        let q = Quat::from_rotation_y(f.x.atan2(f.z));
        for across in [-7., 0., 7.] {
            let mut p = (a + b) * 0.5 + q * Vec3::X * across;
            p.y = super::geometry::ground_height(track, p.x, p.z);
            let kind = (index + usize::from(across > 0.)) % 3;
            let radius = if kind == 0 { 3.2 } else { 5.2 };
            if !clear(occupied, p, radius)
                || super::mounted::reserved(track, p, radius + 3.)
                || (0..2).any(|route| {
                    let (s, _, d) = track.project_route(route, p, station as f32);
                    d < track.half_width(s) + radius + 3.
                })
            {
                continue;
            }
            occupied.push((p, radius));
            census.median_courtyards += 1;
            if (6650..=6950).contains(&station) {
                census.median_review_band += 1;
            }
            let at = |x: f32, z: f32| {
                let w = p + q * Vec3::new(x, 0., z);
                Vec3::new(x, super::geometry::ground_height(track, w.x, w.z) - p.y, z)
            };
            town_ground(
                commands,
                meshes,
                material,
                track,
                p,
                q,
                Vec2::splat(radius * 1.3),
                kind == 2,
            );
            let mut m = Model::default();
            let mut solids = vec![];
            let wood = [0.46, 0.31, 0.17];
            match kind {
                0 => {
                    // Stone well, heavy square timber gantry and two feed crates.
                    for x in [-0.9, 0.9] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            at(x, 0.) + Vec3::Y * 0.55,
                            Vec3::new(0.35, 1.1, 2.15),
                            [0.52, 0.46, 0.36],
                        );
                    }
                    for z in [-0.9, 0.9] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            at(0., z) + Vec3::Y * 0.55,
                            Vec3::new(1.45, 1.1, 0.35),
                            [0.52, 0.46, 0.36],
                        );
                    }
                    for x in [-1.25, 1.25] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            at(x, 0.) + Vec3::Y * 1.45,
                            Vec3::new(0.2, 2.9, 0.2),
                            wood,
                        );
                    }
                    lot_block(
                        &mut m,
                        &mut solids,
                        Vec3::Y * 2.9,
                        Vec3::new(2.9, 0.25, 2.7),
                        wood,
                    );
                    for z in [-2., 2.] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            at(0., z) + Vec3::Y * 0.45,
                            Vec3::new(1.6, 0.9, 0.75),
                            [0.60, 0.43, 0.23],
                        );
                    }
                }
                1 => {
                    // Open-sided market wagon shelter, no solid room or trapped interior.
                    for x in [-2., 2.] {
                        for z in [-2., 2.] {
                            lot_block(
                                &mut m,
                                &mut solids,
                                at(x, z) + Vec3::Y * 1.7,
                                Vec3::new(0.18, 3.4, 0.18),
                                wood,
                            );
                        }
                    }
                    for strip in 0..5 {
                        lot_block(
                            &mut m,
                            &mut solids,
                            Vec3::new(-2. + strip as f32, 3.4, 0.),
                            Vec3::new(1., 0.15, 5.),
                            if strip % 2 == 0 {
                                [0.64, 0.30, 0.15]
                            } else {
                                [0.83, 0.70, 0.47]
                            },
                        );
                    }
                    lot_block(
                        &mut m,
                        &mut solids,
                        at(0., 0.) + Vec3::Y * 0.8,
                        Vec3::new(3.7, 1.6, 1.4),
                        wood,
                    );
                    for x in [-1.2, 0., 1.2] {
                        m.ellipsoid(
                            Vec3::new(x, 1.8, 0.),
                            Vec3::new(0.45, 0.3, 0.5),
                            [0.64, 0.38, 0.14],
                            0.,
                        );
                    }
                    for x in [-3., 3.] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            at(x, 1.) + Vec3::Y * 0.6,
                            Vec3::new(1., 1.2, 1.),
                            [0.50, 0.35, 0.20],
                        );
                    }
                }
                _ => {
                    // Shade courtyard with broad trees, timber seats and herb beds.
                    for (x, z) in [(-2., -1.5), (2., 1.5)] {
                        let c = at(x, z);
                        m.rod(c, c + Vec3::Y * 3.4, 0.22, [0.35, 0.24, 0.13]);
                        solids.push((
                            c + Vec3::Y * 1.7,
                            Quat::IDENTITY,
                            Collider::cylinder(1.7, 0.23),
                        ));
                        m.ellipsoid(
                            c + Vec3::Y * 4.,
                            Vec3::new(2., 2.4, 2.),
                            [0.30, 0.40, 0.17],
                            0.6,
                        );
                    }
                    for z in [-3., 3.] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            at(0., z) + Vec3::Y * 0.45,
                            Vec3::new(2.6, 0.9, 0.65),
                            wood,
                        );
                    }
                }
            }
            let e = root(
                commands,
                meshes,
                material,
                m,
                p,
                q,
                Collider::compound(solids),
            );
            commands
                .entity(e)
                .insert(Name::new("Hills compact median courtyard"));
        }
    }
}
// Distance to a polyline is 1-Lipschitz. Every field corner lies within
// 60 m (48 by36 half-extents); use64 m to absorb rotation/corner roundoff.
fn field_road_bound(track: &Track, p: Vec3, station: f32, maximum_half_width: f32) -> bool {
    (0..2).all(|route| track.project_route(route, p, station).2 - 64. > maximum_half_width + 12.)
}
#[allow(clippy::too_many_arguments)]
fn outer_fields(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    track: &Track,
    occupied: &[(Vec3, f32)],
    census: &mut TownDetailCensus,
) {
    let maximum_half_width = track
        .distances
        .iter()
        .map(|s| track.half_width(*s))
        .fold(0., f32::max);
    for (index, station) in (5660..8380).step_by(103).enumerate() {
        for side in [-1., 1.] {
            // On the split, use the outside of each branch; after the merge, both sides.
            let route = usize::from(side < 0. && station < 7350);
            let (p, q) = track.sample_route(
                route,
                station as f32,
                side * (143. + (index % 3) as f32 * 8.),
            );
            let p = p.with_y(super::geometry::ground_height(track, p.x, p.z));
            let q = q * Quat::from_rotation_y(((index % 5) as f32 - 2.) * 0.055);
            let whole_field_road_clear =
                field_road_bound(track, p, station as f32, maximum_half_width);
            let trees_allowed = !super::mounted::reserved(track, p, 70.);
            let mut clearance_cache = std::collections::HashMap::<(u32, u32), bool>::new();
            let mut height_cache = std::collections::HashMap::<(u32, u32), f32>::new();
            let mut height = |w: Vec3| {
                *height_cache
                    .entry((w.x.to_bits(), w.z.to_bits()))
                    .or_insert_with(|| super::geometry::ground_height(track, w.x, w.z))
            };
            let mut vertices = vec![];
            let mut triangles = vec![];
            let mut uv = vec![];
            let mut m = Model::default();
            let mut solids = vec![];
            for iz in -9_i32..9 {
                for ix in -12_i32..12 {
                    // Rounded irregular field perimeter, with larger acreage than the yards.
                    let limit = 11. - (iz.abs() as f32 * 0.26)
                        + ((index as i32 + iz).rem_euclid(3) as f32 - 1.);
                    if ix.abs() as f32 > limit {
                        continue;
                    }
                    let x = ix as f32 * 4.;
                    let z = iz as f32 * 4.;
                    let corners = [
                        Vec3::new(x, 0., z),
                        Vec3::new(x, 0., z + 4.),
                        Vec3::new(x + 4., 0., z + 4.),
                        Vec3::new(x + 4., 0., z),
                    ];
                    let world = corners.map(|c| p + q * c);
                    if world.iter().any(|w| {
                        *clearance_cache
                            .entry((w.x.to_bits(), w.z.to_bits()))
                            .or_insert_with(|| {
                                !clear(occupied, *w, 4.)
                                    || super::mounted::reserved(track, *w, 5.)
                                    || (!whole_field_road_clear
                                        && (0..2).any(|r| {
                                            let (s, _, d) =
                                                track.project_route(r, *w, station as f32);
                                            d < track.half_width(s) + 12.
                                        }))
                            })
                    }) {
                        continue;
                    }
                    let k = vertices.len() as u32;
                    vertices.extend(world.map(|w| Vec3::new(w.x, height(w) + 0.06, w.z) - p));
                    triangles.extend([[k, k + 1, k + 2], [k, k + 2, k + 3]]);
                    uv.extend(corners.map(|c| [c.x / 5., c.z / 5.]));
                    census.field_cells += 1;
                    let seed = (ix * 37 + iz * 23 + index as i32 * 17).rem_euclid(31);
                    let tree = index.is_multiple_of(3) && seed == 0 && trees_allowed;
                    if !tree && seed % 3 != 0 {
                        continue;
                    }
                    let w = p + q * Vec3::new(x + 2., 0., z + 2.);
                    let c = Vec3::new(w.x, height(w), w.z) - p;
                    if tree {
                        m.rod(c, c + Vec3::Y * 2.8, 0.18, [0.34, 0.25, 0.13]);
                        solids.push((
                            c + Vec3::Y * 1.4,
                            Quat::IDENTITY,
                            Collider::cylinder(1.4, 0.19),
                        ));
                        m.ellipsoid(
                            c + Vec3::Y * 3.3,
                            Vec3::new(2., 2.2, 2.),
                            [0.29, 0.39, 0.14],
                            0.5,
                        );
                    } else if seed % 3 == 0 {
                        // Broad low-poly crop clusters stay in one mesh per field.
                        m.block(
                            c + Vec3::Y * 0.35,
                            Vec3::new(1.5, 0.7, 2.2),
                            if index.is_multiple_of(2) {
                                [0.46, 0.49, 0.19]
                            } else {
                                [0.25, 0.37, 0.14]
                            },
                        );
                    }
                }
            }
            if triangles.is_empty() {
                continue;
            }
            census.outer_fields += 1;
            let mut mesh = crate::environment::mesh(&vertices, &triangles);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
            commands.spawn((
                TownGround(true),
                Name::new("Hills broad irregular crop field"),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(p),
            ));
            if !m.vertices.is_empty() {
                let e = commands
                    .spawn((
                        Name::new("Hills batched field orchard/crops"),
                        Mesh3d(meshes.add(m.mesh())),
                        MeshMaterial3d(material.clone()),
                        Transform::from_translation(p),
                    ))
                    .id();
                if !solids.is_empty() {
                    commands
                        .entity(e)
                        .insert((Collider::compound(solids), Obstacle, HillsSolid));
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn town_infill(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    art: &Art,
    track: &Track,
    occupied: &mut Vec<(Vec3, f32)>,
) {
    let mut census = TownDetailCensus::default();
    central_ranches(
        commands,
        meshes,
        material,
        art,
        track,
        occupied,
        &mut census,
    );
    median_courtyards(commands, meshes, material, track, occupied, &mut census);
    for route in 0..2 {
        for (index, station) in (5640..8370).step_by(113).enumerate() {
            if route == 1 && station > 7320 {
                continue;
            }
            for lane in [-122., -65., 65., 122.] {
                let radius = 19.;
                let Some((p, q)) = site(track, route, station as f32, lane, radius) else {
                    continue;
                };
                if !clear(occupied, p, radius) || super::mounted::reserved(track, p, radius + 3.) {
                    continue;
                }
                let ground = |x: f32, z: f32| {
                    let w = p + q * Vec3::new(x, 0., z);
                    Vec3::new(x, super::geometry::ground_height(track, w.x, w.z) - p.y, z)
                };
                // Avoid tall fill walls on a shelf; small objects still follow the actual mesh.
                let corners = [(-13., -13.), (-13., 13.), (13., -13.), (13., 13.)]
                    .map(|(x, z)| ground(x, z).y);
                if corners.iter().copied().fold(f32::NEG_INFINITY, f32::max)
                    - corners.iter().copied().fold(f32::INFINITY, f32::min)
                    > 2.5
                {
                    continue;
                }
                occupied.push((p, radius));
                town_ground(
                    commands,
                    meshes,
                    material,
                    track,
                    p,
                    q,
                    Vec2::new(28., 28.),
                    false,
                );
                let kind = (index + route + usize::from(lane > 0.)) % 4;
                if kind == 3 {
                    let w = p + q * ground(0., -6.);
                    town_asset(
                        commands,
                        art,
                        w,
                        q,
                        "BRNGRN2",
                        Vec3::new(8., 5.97, 7.),
                        &mut census,
                    );
                }
                let mut m = Model::default();
                let mut solids = Vec::new();
                let wood = [0.39, 0.26, 0.13];
                // Gates face an open access aisle: no enclosed recovery traps.
                for x in [-13., 13.] {
                    for z in [-12., -6., 0., 6., 12.] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            ground(x, z) + Vec3::Y * 0.75,
                            Vec3::new(0.22, 1.5, 0.22),
                            wood,
                        );
                    }
                    for z in [-9., -3., 3., 9.] {
                        for y in [0.55, 1.15] {
                            lot_block(
                                &mut m,
                                &mut solids,
                                ground(x, z) + Vec3::Y * y,
                                Vec3::new(0.15, 0.14, 6.2),
                                [0.55, 0.40, 0.22],
                            );
                        }
                    }
                }
                if kind == 0 || kind == 1 {
                    // Market/feed stalls or a blacksmith's work shelter.
                    for x in [-7., 7.] {
                        let floor = ground(x, 2.);
                        for dx in [-2.3, 2.3] {
                            for z in [-1., 5.] {
                                lot_block(
                                    &mut m,
                                    &mut solids,
                                    ground(x + dx, z) + Vec3::Y * 1.6,
                                    Vec3::new(0.2, 3.2, 0.2),
                                    wood,
                                );
                            }
                        }
                        for strip in 0..6 {
                            lot_block(
                                &mut m,
                                &mut solids,
                                floor + Vec3::new(-2.5 + strip as f32, 3.2, 0.),
                                Vec3::new(1., 0.18, 7.),
                                if strip % 2 == 0 {
                                    [0.70, 0.48, 0.28]
                                } else {
                                    [0.81, 0.72, 0.51]
                                },
                            );
                        }
                        lot_block(
                            &mut m,
                            &mut solids,
                            floor + Vec3::new(0., 0.9, -1.),
                            Vec3::new(4.8, 1.8, 1.2),
                            [0.48, 0.33, 0.18],
                        );
                        for k in 0..5 {
                            let c = floor + Vec3::new(-1.8 + k as f32 * 0.9, 1.95, -1.);
                            m.ellipsoid(
                                c,
                                Vec3::new(0.3, 0.22, 0.3),
                                if kind == 0 {
                                    [0.61, 0.24, 0.12]
                                } else {
                                    [0.22, 0.23, 0.21]
                                },
                                0.,
                            );
                        }
                    }
                } else if kind == 2 {
                    // Freight wagon, separate wheels and an open well courtyard.
                    let c = ground(-6., 1.);
                    lot_block(
                        &mut m,
                        &mut solids,
                        c + Vec3::Y * 1.15,
                        Vec3::new(3.8, 0.4, 5.),
                        wood,
                    );
                    for x in [-1.85, 1.85] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            c + Vec3::new(x, 1.8, 0.),
                            Vec3::new(0.18, 1.2, 5.),
                            [0.51, 0.33, 0.15],
                        );
                    }
                    for x in [-2., 2.] {
                        for z in [-1.6, 1.6] {
                            m.ellipsoid(
                                c + Vec3::new(x, 0.75, z),
                                Vec3::new(0.18, 0.75, 0.75),
                                [0.19, 0.15, 0.11],
                                0.,
                            );
                            solids.push((
                                c + Vec3::new(x, 0.75, z),
                                Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                                Collider::cylinder(0.18, 0.75),
                            ));
                        }
                    }
                    let c = ground(6., 2.);
                    for x in [-1.4, 1.4] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            c + Vec3::new(x, 0.65, 0.),
                            Vec3::new(0.3, 1.3, 3.),
                            [0.48, 0.44, 0.35],
                        );
                    }
                    for z in [-1.4, 1.4] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            c + Vec3::new(0., 0.65, z),
                            Vec3::new(2.8, 1.3, 0.3),
                            [0.48, 0.44, 0.35],
                        );
                    }
                    for x in [-1.8, 1.8] {
                        lot_block(
                            &mut m,
                            &mut solids,
                            c + Vec3::new(x, 1.6, 0.),
                            Vec3::new(0.2, 3.2, 0.2),
                            wood,
                        );
                    }
                    lot_block(
                        &mut m,
                        &mut solids,
                        c + Vec3::Y * 3.2,
                        Vec3::new(4.2, 0.25, 4.),
                        wood,
                    );
                } else {
                    // Stockyard water trough and stacked lumber/hay/feed storage.
                    let c = ground(-6., 1.);
                    lot_block(
                        &mut m,
                        &mut solids,
                        c + Vec3::Y * 0.55,
                        Vec3::new(2., 1.1, 6.),
                        [0.32, 0.25, 0.16],
                    );
                    m.block(
                        c + Vec3::Y * 1.11,
                        Vec3::new(1.7, 0.02, 5.7),
                        [0.22, 0.32, 0.32],
                    );
                    for row in 0..4 {
                        for level in 0..3 {
                            lot_block(
                                &mut m,
                                &mut solids,
                                ground(5. + row as f32 * 0.7, 3.)
                                    + Vec3::Y * (0.2 + level as f32 * 0.4),
                                Vec3::new(0.55, 0.35, 7.),
                                [0.58, 0.43, 0.24],
                            );
                        }
                    }
                }
                // Barrels and freight crates bring activity into the open backlots.
                // A pair of original low shrubs/cacti occupies the outer lot
                // corners, inside the already checked fence and clear of access.
                for x in [-10.3, 10.3] {
                    let c = ground(x, -9.);
                    if kind == 2 {
                        m.rod(c, c + Vec3::Y * 1.4, 0.2, [0.27, 0.40, 0.20]);
                        solids.push((
                            c + Vec3::Y * 0.7,
                            Quat::IDENTITY,
                            Collider::cylinder(0.7, 0.2),
                        ));
                        for side in [-1., 1.] {
                            m.rod(
                                c + Vec3::Y * 0.7,
                                c + Vec3::new(side * 0.55, 1.25, 0.),
                                0.12,
                                [0.30, 0.43, 0.20],
                            );
                        }
                    } else {
                        m.ellipsoid(
                            c + Vec3::Y * 0.55,
                            Vec3::new(0.85, 0.58, 0.8),
                            [0.26, 0.37, 0.18],
                            0.6,
                        );
                        solids.push((
                            c + Vec3::Y * 0.55,
                            Quat::IDENTITY,
                            Collider::cuboid(0.65, 0.55, 0.65),
                        ));
                    }
                    census.outer_shrubs += 1;
                }
                for k in 0..5 {
                    let c = ground(-9. + k as f32 * 4.5, 10.);
                    lot_block(
                        &mut m,
                        &mut solids,
                        c + Vec3::Y * 0.65,
                        Vec3::new(1.4, 1.3, 1.3),
                        [0.53, 0.38, 0.21],
                    );
                    for y in [0.25, 1.05] {
                        m.block(
                            c + Vec3::Y * y,
                            Vec3::new(1.43, 0.08, 1.33),
                            [0.23, 0.22, 0.18],
                        );
                    }
                }
                let e = root(
                    commands,
                    meshes,
                    material,
                    m,
                    p,
                    q,
                    Collider::compound(solids),
                );
                commands
                    .entity(e)
                    .insert((TownLot, Name::new(format!("Hills western backlot {kind}"))));
                census.backlots += 1;
                if let Some(props) = &art.props {
                    let (name, size) = if kind == 3 {
                        ("HAYDOUB2", Vec3::new(3.5, 0.95, 1.85))
                    } else {
                        ("BENCH12", Vec3::new(2.6, 1.2, 1.))
                    };
                    let position = ground(0., if kind == 3 { 8. } else { -8. });
                    let child = commands
                        .spawn((
                            Transform::from_translation(position),
                            Visibility::default(),
                            Collider::cuboid(size.x / 2., size.y / 2., size.z / 2.),
                            Obstacle,
                            HillsSolid,
                        ))
                        .id();
                    // Collider is centred above the feet, independent of the authored visual origin.
                    commands.entity(child).insert(Collider::compound(vec![(
                        Vec3::Y * size.y / 2.,
                        Quat::IDENTITY,
                        Collider::cuboid(size.x / 2., size.y / 2., size.z / 2.),
                    )]));
                    let visual = props.fitted(
                        commands,
                        props.named(name),
                        Vec3::ZERO,
                        Quat::IDENTITY,
                        size,
                        "hills-western-backlot",
                    );
                    commands.entity(child).add_child(visual);
                    commands.entity(e).add_child(child);
                    census.reused_props += 1;
                }
            }
        }
    }
    // Small hitching posts and direction boards occupy frontage breaks, not a fifth row.
    for route in 0..2 {
        for station in (5575..8420).step_by(67) {
            if route == 1 && station > 7320 {
                continue;
            }
            for lane in [-11., 11.] {
                let Some((p, q)) = site(track, route, station as f32, lane, 1.2) else {
                    continue;
                };
                if !clear(occupied, p, 1.2) || super::mounted::reserved(track, p, 2.) {
                    continue;
                }
                let mut m = Model::default();
                let mut solids = vec![];
                lot_block(
                    &mut m,
                    &mut solids,
                    Vec3::Y * 1.25,
                    Vec3::new(0.17, 2.5, 0.17),
                    [0.38, 0.24, 0.12],
                );
                lot_block(
                    &mut m,
                    &mut solids,
                    Vec3::Y * 2.2,
                    Vec3::new(2.1, 0.5, 0.16),
                    [0.58, 0.40, 0.22],
                );
                sign_letters(
                    &mut m,
                    if station % 2 == 0 { "FEED" } else { "WELL" },
                    Vec3::new(0., 2.2, -0.09),
                );
                root(
                    commands,
                    meshes,
                    material,
                    m,
                    p,
                    q,
                    Collider::compound(solids),
                );
                occupied.push((p, 1.2));
                census.roadside_groups += 1;
            }
        }
    }
    outer_fields(commands, meshes, material, track, occupied, &mut census);
    town_materials(commands);
    commands.insert_resource(census);
}

pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    art: &Art,
    track: &Track,
    census: &mut Census,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.90,
        ..default()
    });
    let mut occupied: Vec<(Vec3, f32)> = Vec::new();
    for (western, start, end, step) in [(true, 5530, 8460, 30)] {
        for route in 0..2 {
            for d in (start..end).step_by(step) {
                if route == 1 && (!western || d >= 7350) {
                    continue;
                }
                for side in [-1., 1.] {
                    // Occasional deliberate pull-outs keep parked trucks visible without burying them in storefronts.
                    if western
                        && (5760..8200).step_by(180).enumerate().any(|(i, station)| {
                            let parking_side = if i % 2 == 0 { -1. } else { 1. };
                            side == parking_side && (d as f32 - (station + 65) as f32).abs() < 28.
                        })
                    {
                        continue;
                    }
                    let lane = side * if western { 19.8 } else { 24. };
                    if let Some((p, facing)) = frontage_site(track, route, d as f32, lane, western)
                        .filter(|(p, _)| clear(&occupied, *p, if western { 12.5 } else { 12. }))
                    {
                        occupied.push((p, if western { 12.5 } else { 12. }));
                        census.frontage_groups += usize::from(building(
                            commands,
                            meshes,
                            &material,
                            track,
                            p,
                            facing,
                            census.buildings,
                            western,
                        ));
                        census.buildings += 1;
                    }
                }
            }
        }
    }
    for route in 0..2 {
        for (i, d) in (5760..8200).step_by(180).enumerate() {
            if route == 1 && d > 7300 {
                continue;
            }
            let side = if i % 2 == 0 { -1. } else { 1. };
            if let Some((p, q)) = site(track, route, d as f32, side * 70., 17.)
                .filter(|(p, _)| clear(&occupied, *p, 17.))
            {
                occupied.push((p, 17.));
                crate::environment::spawn_cow(commands, art, p, q);
                census.cows += 1;
                horse(commands, meshes, &material, p + q * Vec3::Z * 8., q, i);
                census.horses += 1;
                // Timber paddocks have openings and keep stock far from the boardwalk.
                let mut m = Model::default();
                for z in [-5., 14.] {
                    for x in [-5., 0., 5.] {
                        m.block(
                            Vec3::new(x, 0.7, z),
                            Vec3::new(0.18, 1.4, 0.18),
                            [0.40, 0.26, 0.13],
                        );
                    }
                    for y in [0.55, 1.05] {
                        m.block(
                            Vec3::new(0., y, z),
                            Vec3::new(10., 0.13, 0.13),
                            [0.48, 0.33, 0.18],
                        );
                    }
                }
                root(
                    commands,
                    meshes,
                    &material,
                    m,
                    p,
                    q,
                    Collider::compound(vec![
                        (
                            Vec3::new(0., 0.7, -5.),
                            Quat::IDENTITY,
                            Collider::cuboid(5., 0.7, 0.1),
                        ),
                        (
                            Vec3::new(0., 0.7, 14.),
                            Quat::IDENTITY,
                            Collider::cuboid(5., 0.7, 0.1),
                        ),
                    ]),
                );
            }
            let station = d as f32 + 15.;
            let (p, q) = track.sample_route(route, station, side * 8.8);
            let pedestrian_clear = [-4., 0., 4.].into_iter().all(|along| {
                (0..2).all(|branch| {
                    let point = p + q * Vec3::Z * along;
                    let (s, _, distance) = track.project_route(branch, point, station);
                    distance > track.half_width(s) + 1.5
                })
            });
            if pedestrian_clear {
                let p = p - q * Vec3::Y * 0.08;
                let cowboy =
                    crate::environment::spawn_pedestrian_patrol(commands, art, p, q, i as f32, 4.);
                commands.entity(cowboy).insert(HillsCowboy);
                let mut hat = Model::default();
                hat.block(
                    Vec3::Y * 1.88,
                    Vec3::new(0.85, 0.06, 0.70),
                    [0.36, 0.24, 0.12],
                );
                hat.block(
                    Vec3::Y * 2.02,
                    Vec3::new(0.40, 0.24, 0.35),
                    [0.43, 0.30, 0.15],
                );
                let hat = commands
                    .spawn((
                        Mesh3d(meshes.add(hat.mesh())),
                        MeshMaterial3d(material.clone()),
                        Transform::default(),
                    ))
                    .id();
                crate::runtime::attach_visuals(commands, cowboy, &[hat]);
                census.cowboys += 1;
            }
            if let Some((p, q)) = site(track, route, d as f32 + 65., side * 15., 4.)
                .filter(|(p, _)| clear(&occupied, *p, 4.))
            {
                occupied.push((p, 4.));
                let truck = commands
                    .spawn((
                        Transform::from_translation(p).with_rotation(q),
                        Visibility::default(),
                        RigidBody::Fixed,
                        Collider::compound(vec![(
                            Vec3::Y * 1.3,
                            Quat::IDENTITY,
                            Collider::cuboid(1.28, 1.3, 3.31),
                        )]),
                        Obstacle,
                        HillsSolid,
                    ))
                    .id();
                if let Some(props) = &art.props {
                    let visual = props.fitted(
                        commands,
                        props.named("TRUCK2"),
                        Vec3::ZERO,
                        Quat::IDENTITY,
                        Vec3::new(2.56, 2.62, 6.6),
                        "hills-truck",
                    );
                    commands.entity(truck).add_child(visual);
                } else {
                    let mut m = Model::default();
                    m.block(Vec3::Y, Vec3::new(2.5, 1.4, 6.5), [0.44, 0.29, 0.16]);
                    m.block(
                        Vec3::new(0., 2., 1.7),
                        Vec3::new(2.4, 1.3, 2.),
                        [0.59, 0.47, 0.29],
                    );
                    let e = commands
                        .spawn((
                            Mesh3d(meshes.add(m.mesh())),
                            MeshMaterial3d(material.clone()),
                            Transform::default(),
                        ))
                        .id();
                    commands.entity(truck).add_child(e);
                }
                census.trucks += 1;
            }
        }
    }
    town_infill(commands, meshes, &material, art, track, &mut occupied);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::mesh::VertexAttributeValues;
    #[test]
    fn outer_field_bound_preserves_original_road_clearance() {
        let track = Track::hills().unwrap();
        let maximum = track
            .distances
            .iter()
            .map(|s| track.half_width(*s))
            .fold(0., f32::max);
        for station in [5660., 6300., 6800., 7308., 8340.] {
            for side in [-1., 1.] {
                let route = usize::from(side < 0. && station < 7350.);
                let (p, q) = track.sample_route(route, station, side * 143.);
                assert!(field_road_bound(&track, p, station, maximum));
                for yaw in [-0.11, 0., 0.11] {
                    for x in [-48., 0., 48.] {
                        for z in [-36., 0., 36.] {
                            let w = p + (q * Quat::from_rotation_y(yaw)) * Vec3::new(x, 0., z);
                            for r in 0..2 {
                                let (s, _, d) = track.project_route(r, w, station);
                                assert!(d >= track.half_width(s) + 12.);
                            }
                        }
                    }
                }
                let near = track.sample_route(route, station, 10.).0;
                assert!(!field_road_bound(&track, near, station, maximum));
            }
        }
    }
    #[test]
    fn storefront_letters_read_left_to_right_from_the_front() {
        let mut model = Model::default();
        sign_letters(&mut model, "LH", Vec3::ZERO);
        let pixels: Vec<_> = model
            .vertices
            .chunks_exact(4)
            .map(|q| q.iter().copied().sum::<Vec3>() / 4.)
            .collect();
        // L has six stem pixels and five bottom pixels. Viewed from -Z,
        // its foot extends right, then the H begins farther to the right.
        let screen_x = |p: Vec3| -p.x;
        assert!((screen_x(pixels[10]) - screen_x(pixels[0]) - 0.4).abs() < 0.001);
        assert!(screen_x(pixels[11]) > screen_x(pixels[10]));
        for tri in &model.indices {
            let [a, b, c] = tri.map(|i| model.vertices[i as usize]);
            assert!((b - a).cross(c - a).dot(Vec3::NEG_Z) > 0.);
        }
    }
    #[test]
    fn original_rounded_forms_have_outward_facing_triangles() {
        let mut rod = Model::default();
        rod.rod(Vec3::ZERO, Vec3::Y * 2., 0.5, [0.3, 0.4, 0.2]);
        for tri in &rod.indices {
            let [a, b, c] = tri.map(|i| rod.vertices[i as usize]);
            let normal = (b - a).cross(c - a);
            let radial = (a + b + c) / 3. - Vec3::Y;
            assert!(normal.dot(radial) > 0.);
        }
        let mut sphere = Model::default();
        sphere.ellipsoid(Vec3::ZERO, Vec3::ONE, [0.3, 0.4, 0.2], 0.);
        for tri in &sphere.indices {
            let [a, b, c] = tri.map(|i| sphere.vertices[i as usize]);
            assert!((b - a).cross(c - a).dot((a + b + c) / 3.) > 0.);
        }
    }
    #[test]
    fn placed_hills_geometry_has_every_family_and_buildings_clear_both_streets() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Track::hills().unwrap())
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Startup, crate::world::setup_world);
        app.update();
        let details = app.world().resource::<TownDetailCensus>();
        assert!(
            details.backlots >= 30,
            "actual western backlot placement: {}",
            details.backlots
        );
        assert!(
            details.central_ranches >= 4,
            "centre-derived town ranches: {}",
            details.central_ranches
        );
        assert!(
            details.median_review_band >= 2,
            "compact median coverage at6650–6950: {}",
            details.median_review_band
        );
        assert!(details.outer_fields >= 20 && details.field_cells > 3000);
        assert_eq!(details.outer_shrubs, details.backlots * 2);
        let mut lots = app
            .world_mut()
            .query_filtered::<&Transform, With<TownLot>>();
        let track = app.world().resource::<Track>();
        for t in lots.iter(app.world()) {
            assert!(!super::super::mounted::reserved(track, t.translation, 22.));
            for route in 0..2 {
                let (s, _, d) = track.project_route(route, t.translation, 6000.);
                assert!(d >= track.half_width(s) + 22.);
            }
        }
        let mut storefronts = app
            .world_mut()
            .query_filtered::<&Transform, With<HillsBuilding>>();
        let track = app.world().resource::<Track>();
        for t in storefronts.iter(app.world()) {
            for x in [-2.1, 0., 2.1] {
                let point = t.translation + t.rotation * Vec3::new(x, 0., -9.5);
                assert!(!super::super::mounted::reserved(track, point, 0.5));
                for route in 0..2 {
                    let (s, _, distance) = track.project_route(route, point, 6000.);
                    assert!(
                        distance > track.half_width(s) + 1.6,
                        "storefront apron obstructs route {route} near {s}: {distance}"
                    );
                }
            }
        }
        let census = app.world().resource::<Census>();
        assert!(census.buildings > 150);
        assert!(census.frontage_groups > 100);
        assert!(
            census.cows - census.farm_cows == census.horses && census.horses > 3,
            "{}",
            serde_json::to_string(census).unwrap()
        );
        assert!(
            census.trees > 200
                && census.grass_tufts > 10000
                && census.farms == 6
                && census.farm_cows == 18
        );
        assert!(census.cowboys > 3 && census.trucks > 3);
        assert!(
            census.guardrail_segments > 100 && census.guardrail_segments < 950,
            "rails now protect curves and bridge panels, rather than every straight"
        );
        let mut solids = app
            .world_mut()
            .query_filtered::<Option<&Collider>, With<HillsSolid>>();
        assert!(solids.iter(app.world()).all(|c| c.is_some()));
        // The entire moving horse footprint must clear actual resident static
        // colliders for a complete patrol cycle, not only its spawn centre.
        let mut fixed = app
            .world_mut()
            .query_filtered::<(Entity, &Transform, &Collider, Option<&RigidBody>), (
                Without<crate::world::Ground>,
                Without<super::super::mounted::Mounted>,
            )>();
        let bounds: Vec<_> = fixed
            .iter(app.world())
            .filter(|(_, _, _, body)| body.is_none_or(|body| *body == RigidBody::Fixed))
            .map(|(entity, t, c, _)| {
                let mut shape = c.clone();
                shape.set_scale(t.scale, 12);
                let iso = bevy_rapier3d::na::Isometry3::from_parts(
                    t.translation.into(),
                    t.rotation.into(),
                );
                (entity, shape.raw.compute_aabb(&iso))
            })
            .collect();
        assert!(
            bounds.len() > 500,
            "mounted clearance must inspect real resident colliders"
        );
        let track = app.world().resource::<Track>();
        for index in 0..super::super::mounted::COUNT {
            for tick in 0..160 {
                let p = super::super::mounted::pose(track, index, tick as f32 * 0.5).translation;
                for (entity, aabb) in &bounds {
                    let overlap = p.x + 1.65 > aabb.mins.x
                        && p.x - 1.65 < aabb.maxs.x
                        && p.z + 1.65 > aabb.mins.z
                        && p.z - 1.65 < aabb.maxs.z
                        && p.y + 3.2 > aabb.mins.y
                        && p.y + 0.05 < aabb.maxs.y;
                    assert!(
                        !overlap,
                        "mounted patrol {index} at {tick} intersects {entity:?}"
                    );
                }
            }
        }
        let mut buildings = app
            .world_mut()
            .query_filtered::<(&Transform, &Mesh3d), With<HillsBuilding>>();
        let mut branch_counts = [0; 2];
        let mut merged = 0;
        for (transform, handle) in buildings.iter(app.world()) {
            let track = app.world().resource::<Track>();
            let mesh = app
                .world()
                .resource::<Assets<Mesh>>()
                .get(&handle.0)
                .unwrap();
            let Some(VertexAttributeValues::Float32x3(vertices)) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                panic!("building vertices")
            };
            let min = vertices
                .iter()
                .map(|p| Vec3::from_array(*p))
                .fold(Vec3::splat(f32::INFINITY), Vec3::min);
            let max = vertices
                .iter()
                .map(|p| Vec3::from_array(*p))
                .fold(Vec3::splat(f32::NEG_INFINITY), Vec3::max);
            for x in [min.x, max.x] {
                for z in [min.z, max.z] {
                    let corner = transform.transform_point(Vec3::new(x, 0., z));
                    for route in 0..2 {
                        let (station, _, distance) = track.project_route(route, corner, 0.);
                        assert!(
                            distance > track.half_width(station) + 1.,
                            "building overlaps road at {station}"
                        );
                    }
                }
            }
            let a = track.project_route(0, transform.translation, 0.);
            let b = track.project_route(1, transform.translation, 0.);
            if (5600.0..7300.0).contains(&a.0) {
                branch_counts[usize::from(b.2 < a.2)] += 1;
            }
            if (7500.0..8400.0).contains(&a.0) {
                merged += 1;
            }
        }
        assert!(
            branch_counts[0] > 20 && branch_counts[1] > 20 && merged > 20,
            "{branch_counts:?}, {merged}"
        );
    }
}
