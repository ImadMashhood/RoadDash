//! Original patrol equipment. These entities are visual children, including on wrecks.
use super::*;
/// Separate from every selectable character/roster palette; carried through recovery.
pub const OFFICER_PALETTE: usize = usize::MAX;

pub(super) fn equipment(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Part> {
    let white = materials.add(StandardMaterial {
        base_color: swatch(0xe5e9e5),
        metallic: 0.25,
        perceptual_roughness: 0.32,
        ..default()
    });
    let navy = materials.add(StandardMaterial {
        base_color: swatch(0x12334c),
        metallic: 0.3,
        perceptual_roughness: 0.35,
        ..default()
    });
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgba(0.29, 0.53, 0.62, 0.55),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.15,
        metallic: 0.1,
        ..default()
    });
    let metal = materials.add(StandardMaterial {
        base_color: swatch(0xadb8bb),
        metallic: 0.8,
        perceptual_roughness: 0.25,
        ..default()
    });
    let gold = materials.add(StandardMaterial {
        base_color: swatch(0xe5b642),
        metallic: 0.55,
        perceptual_roughness: 0.3,
        ..default()
    });
    let red = materials.add(StandardMaterial {
        base_color: swatch(0xe51d28),
        emissive: LinearRgba::rgb(2.0, 0.01, 0.02),
        ..default()
    });
    let blue = materials.add(StandardMaterial {
        base_color: swatch(0x2379ef),
        emissive: LinearRgba::rgb(0.01, 0.25, 2.0),
        ..default()
    });
    let cube = meshes.add(beveled_box());
    let lens = meshes.add(Cylinder::new(1.0, 1.0).mesh().resolution(16));
    let round = meshes.add(Sphere::new(1.0).mesh().uv(16, 10));
    let mut out = Vec::new();
    let mut add = |mesh: &Handle<Mesh>,
                   material: &Handle<StandardMaterial>,
                   pos: Vec3,
                   scale: Vec3,
                   tilt: f32| {
        out.push(Part {
            mesh: mesh.clone(),
            materials: vec![material.clone()],
            officer_material: material.clone(),
            officer_hidden: false,
            group: "patrol-equipment".into(),
            rest: Transform::from_translation(pos)
                .with_scale(scale)
                .with_rotation(Quat::from_rotation_x(tilt)),
            glove: false,
        });
    };
    // Broad touring nose, framed screen, rear radio box and twin hard panniers.
    add(
        &round,
        &white,
        Vec3::new(0., 1.13, 0.90),
        Vec3::new(0.39, 0.20, 0.19),
        0.,
    );
    add(
        &cube,
        &navy,
        Vec3::new(0., 1.16, 1.12),
        Vec3::new(0.62, 0.09, 0.035),
        0.,
    );
    add(
        &cube,
        &glass,
        Vec3::new(0., 1.53, 0.86),
        Vec3::new(0.61, 0.62, 0.035),
        -0.65,
    );
    add(
        &cube,
        &metal,
        Vec3::new(0., 1.29, 0.96),
        Vec3::new(0.68, 0.025, 0.04),
        -0.24,
    );
    add(
        &cube,
        &navy,
        Vec3::new(0., 1.17, -0.91),
        Vec3::new(0.46, 0.18, 0.38),
        0.,
    );
    add(
        &cube,
        &white,
        Vec3::new(0., 1.27, -0.91),
        Vec3::new(0.48, 0.04, 0.40),
        0.,
    );
    for side in [-1., 1.] {
        add(
            &cube,
            &white,
            Vec3::new(side * 0.43, 0.82, -0.72),
            Vec3::new(0.29, 0.43, 0.66),
            0.,
        );
        add(
            &cube,
            &navy,
            Vec3::new(side * 0.581, 0.85, -0.72),
            Vec3::new(0.015, 0.13, 0.59),
            0.,
        );
        add(
            &cube,
            &gold,
            Vec3::new(side * 0.592, 0.86, -0.72),
            Vec3::new(0.015, 0.12, 0.10),
            0.,
        );
        add(
            &cube,
            &metal,
            Vec3::new(side * 0.43, 1.045, -0.72),
            Vec3::new(0.30, 0.025, 0.67),
            0.,
        );
        add(
            &cube,
            &metal,
            Vec3::new(side * 0.25, 1.30, -0.90),
            Vec3::new(0.10, 0.10, 0.10),
            0.,
        );
        let lamp = if side < 0. { &red } else { &blue };
        add(
            &lens,
            lamp,
            Vec3::new(side * 0.25, 1.38, -0.90),
            Vec3::new(0.065, 0.09, 0.065),
            0.,
        );
        add(
            &round,
            lamp,
            Vec3::new(side * 0.33, 1.23, 1.03),
            Vec3::new(0.07, 0.065, 0.05),
            0.,
        );
    }
    out
}

/// A shared chamfered solid keeps luggage/trim readable without razor box edges.
fn beveled_box() -> Mesh {
    let mut faces = Vec::<Vec<Vec3>>::new();
    let axes = [Vec3::X, Vec3::Y, Vec3::Z];
    for i in 0..3 {
        for sign in [-1., 1.] {
            let a = axes[i] * sign * 0.5;
            let b = axes[(i + 1) % 3] * 0.42;
            let c = axes[(i + 2) % 3] * 0.42;
            faces.push(vec![a - b - c, a + b - c, a + b + c, a - b + c]);
        }
    }
    for i in 0..3 {
        for j in i + 1..3 {
            let k = 3 - i - j;
            for a in [-1., 1.] {
                for b in [-1., 1.] {
                    let p = axes[i] * a * 0.5 + axes[j] * b * 0.42;
                    let q = axes[i] * a * 0.42 + axes[j] * b * 0.5;
                    let edge = axes[k] * 0.42;
                    faces.push(vec![p - edge, p + edge, q + edge, q - edge]);
                }
            }
        }
    }
    for x in [-1., 1.] {
        for y in [-1., 1.] {
            for z in [-1., 1.] {
                faces.push(vec![
                    Vec3::new(x * 0.5, y * 0.42, z * 0.42),
                    Vec3::new(x * 0.42, y * 0.5, z * 0.42),
                    Vec3::new(x * 0.42, y * 0.42, z * 0.5),
                ]);
            }
        }
    }
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    for mut face in faces {
        let mut normal = (face[1] - face[0]).cross(face[2] - face[0]).normalize();
        if normal.dot(face[0]) < 0. {
            face.reverse();
            normal = -normal;
        }
        let start = positions.len() as u32;
        for p in &face {
            positions.push(p.to_array());
            normals.push(normal.to_array());
            uvs.push([0., 0.]);
        }
        for i in 1..face.len() - 1 {
            indices.extend([start, start + i as u32, start + i as u32 + 1]);
        }
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
