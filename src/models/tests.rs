use super::*;
#[test]
fn bundle_has_shared_standing_seated_and_bike_geometry_and_rejects_bad_data() {
    let records = rr_content::riding_model::decode(BUNDLE).unwrap();
    for group in [
        "standing",
        "seated",
        "motorcycle",
        "front-wheel",
        "rear-wheel",
    ] {
        assert!(records.iter().any(|r| r.group == group));
    }
    let count = |group: &str| {
        records
            .iter()
            .filter(|r| r.group == group)
            .map(|r| r.indices.len() / 3)
            .sum::<usize>()
    };
    assert_eq!(count("standing"), 13_896);
    assert_eq!(count("standing"), count("seated"));
    assert_eq!(
        count("motorcycle") + count("front-wheel") + count("rear-wheel"),
        44_871
    );
    assert!(rr_content::riding_model::decode(&BUNDLE[..BUNDLE.len() - 1]).is_err());
    let mut corrupt = BUNDLE.to_vec();
    corrupt[0] = 0;
    assert!(rr_content::riding_model::decode(&corrupt).is_err());
}

fn authored(app: &mut App) {
    let mut meshes = Assets::<Mesh>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let mut binds = Assets::<SkinnedMeshInverseBindposes>::default();
    let mut images = Assets::<Image>::default();
    let models = AuthoredModels::load(&mut meshes, &mut materials, &mut binds, &mut images, None);
    app.world_mut().resource_mut::<world::Art>().models = Some(models);
    app.add_plugins(RoadModelPlugin);
    app.insert_resource(meshes)
        .insert_resource(materials)
        .insert_resource(binds)
        .insert_resource(images);
}
#[test]
fn all_four_human_states_use_the_same_skin_and_render_entities_have_no_physics() {
    use bevy::ecs::system::RunSystemOnce;
    use bevy_rapier3d::prelude::*;
    let mut app = crate::game::tests::simulation();
    authored(&mut app);
    app.world_mut()
        .run_system_once(|mut commands: Commands, art: Res<world::Art>| {
            world::bike(&mut commands, &art, 4, true);
            world::runner(&mut commands, &art, 4);
            let owner = commands.spawn_empty().id();
            crate::recovery::humanoid(
                &mut commands,
                &art,
                owner,
                4,
                Vec3::new(5.0, 3.0, 0.0),
                Quat::IDENTITY,
                Vec3::ZERO,
            );
            crate::environment::spawn_pedestrian(
                &mut commands,
                &art,
                Vec3::new(-5.0, 0.0, 0.0),
                Quat::IDENTITY,
                1.0,
            );
        })
        .unwrap();
    app.update();
    let w = app.world_mut();
    let skins:Vec<_>=w.query_filtered::<(Entity,&SkinnedMesh,&MeshMaterial3d<StandardMaterial>),With<AuthoredHuman>>().iter(w).map(|(e,s,m)|(e,s.joints.clone(),m.0.clone())).collect();
    assert_eq!(
        skins.len(),
        20,
        "four material regions plus isolated gloves in each of four human states"
    );
    assert_eq!(w.query::<&human::HumanSegment>().iter(w).count(), 44);
    for (e, joints, _) in &skins {
        assert_eq!(joints.len(), 16);
        for joint in joints {
            assert!(w.get::<SkinJoint>(*joint).is_some());
            assert!((w.get::<Transform>(*joint).unwrap().scale - Vec3::ONE).length() < 0.00001);
        }
        assert!(w.get::<RigidBody>(*e).is_none());
        assert!(w.get::<Collider>(*e).is_none());
        assert!(w.get::<ImpulseJoint>(*e).is_none());
    }
    for (e, _) in w
        .query_filtered::<(Entity, &Mesh3d), With<AuthoredMotorcycle>>()
        .iter(w)
    {
        assert!(w.get::<Collider>(e).is_none());
        assert!(w.get::<RigidBody>(e).is_none());
    }
    // Three standing states share mesh/material handles; seated uses the same atlas materials.
    let mut skin_material_counts = std::collections::HashMap::new();
    for (_, _, m) in &skins {
        *skin_material_counts.entry(m.id()).or_insert(0) += 1;
    }
    // Region handles may differ per bind mesh, but standing appearances must be shared.
    assert!(skin_material_counts.values().filter(|c| **c == 3).count() == 5);
}
#[test]
fn authored_skin_does_not_change_any_ragdoll_body_joint_or_180_tick_trajectory() {
    use bevy::ecs::system::RunSystemOnce;
    use bevy_rapier3d::prelude::*;
    for (enabled, palette) in [
        (false, 4),
        (true, 4),
        (false, OFFICER_PALETTE),
        (true, OFFICER_PALETTE),
    ] {
        let mut old = crate::game::tests::simulation();
        let mut new = crate::game::tests::simulation();
        authored(&mut new);
        old.world_mut()
            .resource_mut::<crate::biomechanics::Settings>()
            .enabled = enabled;
        new.world_mut()
            .resource_mut::<crate::biomechanics::Settings>()
            .enabled = enabled;
        fn spawn(app: &mut App, palette: usize) -> Vec<Entity> {
            app.world_mut()
                .run_system_once(move |mut commands: Commands, art: Res<world::Art>| {
                    let owner = commands.spawn_empty().id();
                    crate::recovery::humanoid(
                        &mut commands,
                        &art,
                        owner,
                        palette,
                        Vec3::new(0.0, 3.0, 8.0),
                        Quat::from_rotation_y(0.3),
                        Vec3::new(3.0, -2.0, 18.0),
                    )
                })
                .unwrap()
        }
        let a = spawn(&mut old, palette);
        let b = spawn(&mut new, palette);
        for (a, b) in a.iter().zip(&b) {
            let a = old.world().entity(*a);
            let b = new.world().entity(*b);
            for component in [
                "body",
                "collider",
                "mass",
                "damping",
                "friction",
                "restitution",
                "ccd",
                "joint",
            ] {
                let signature = |e: bevy::ecs::world::EntityRef<'_>| match component {
                    "body" => format!("{:?}", e.get::<RigidBody>()),
                    "collider" => format!(
                        "{:?}",
                        e.get::<Collider>()
                            .unwrap()
                            .as_cuboid()
                            .unwrap()
                            .half_extents()
                    ),
                    "mass" => format!("{:?}", e.get::<ColliderMassProperties>()),
                    "damping" => format!("{:?}", e.get::<Damping>()),
                    "friction" => format!("{:?}", e.get::<Friction>()),
                    "restitution" => format!("{:?}", e.get::<Restitution>()),
                    "ccd" => format!("{:?}", e.get::<Ccd>()),
                    _ => format!("{:?}", e.get::<ImpulseJoint>().map(|j| &j.data)),
                };
                assert_eq!(
                    signature(a),
                    signature(b),
                    "physics component changed: {component}"
                );
            }
        }
        let mut max_position_error = 0.0_f32;
        for tick in 0..180 {
            old.update();
            new.update();
            if enabled {
                fn segments(
                    app: &mut App,
                ) -> std::collections::BTreeMap<usize, (Transform, Velocity)> {
                    app.world_mut()
                        .query::<(&crate::biomechanics::Segment, &Transform, &Velocity)>()
                        .iter(app.world())
                        .map(|(s, t, v)| (s.part, (*t, *v)))
                        .collect()
                }
                let physical_a = segments(&mut old);
                let physical_b = segments(&mut new);
                assert_eq!(physical_a.len(), 16);
                assert_eq!(physical_b.len(), 16);
                for (part, (ta, va)) in physical_a {
                    let (tb, vb) = physical_b[&part];
                    assert!(
                        ta.translation.distance(tb.translation) < 0.0001,
                        "distal position {part} at {tick}"
                    );
                    assert!(
                        ta.rotation.angle_between(tb.rotation) < 0.001,
                        "distal rotation {part} at {tick}"
                    );
                    assert!(
                        va.linvel.distance(vb.linvel) < 0.0001
                            && va.angvel.distance(vb.angvel) < 0.0001,
                        "distal velocity {part} at {tick}"
                    );
                }
            }
            for (a, b) in a.iter().zip(&b) {
                let ta = old.world().get::<Transform>(*a).unwrap();
                let tb = new.world().get::<Transform>(*b).unwrap();
                let va = old.world().get::<Velocity>(*a).unwrap();
                let vb = new.world().get::<Velocity>(*b).unwrap();
                max_position_error =
                    max_position_error.max(ta.translation.distance(tb.translation));
                assert!(
                    ta.translation.distance(tb.translation) < 0.0001,
                    "body trajectory changed at tick {tick}: {ta:?} vs {tb:?}"
                );
                assert!(
                    ta.rotation.angle_between(tb.rotation) < 0.001,
                    "body rotation changed at tick {tick}"
                );
                assert!(
                    va.linvel.distance(vb.linvel) < 0.0001
                        && va.angvel.distance(vb.angvel) < 0.0001,
                    "momentum changed at {tick}"
                );
                assert_eq!(ta.scale, tb.scale);
            }
        }
        println!(
            "MODEL_PHYSICS_PARITY palette={palette} enabled={enabled} ticks=180 source_bodies=11 max_position_error_m={max_position_error}"
        );
    }
}
#[test]
fn visual_replacement_preserves_full_crash_run_remount_and_cleans_up_skin() {
    for palette in [0, OFFICER_PALETTE] {
        recovery_parity(palette);
    }
}
fn recovery_parity(palette: usize) {
    use crate::{
        game::{Phase, Race, Rider},
        recovery::{Recovery, RecoveryStage},
    };
    let mut old = crate::game::tests::simulation();
    let mut new = crate::game::tests::simulation();
    authored(&mut new);
    for app in [&mut old, &mut new] {
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let w = app.world_mut();
        let mut q = w.query::<&mut Rider>();
        let mut r = q.single_mut(w).unwrap();
        r.palette_index = palette;
        r.speed = 8.0;
        r.motion_velocity = Vec3::new(3.0, 0.0, 8.0);
        r.health = 0.0;
    }
    let mut visited = [false; 4];
    let mut remounted = false;
    for tick in 0..1800 {
        old.update();
        new.update();
        let state = |app: &mut App| {
            let w = app.world_mut();
            let mut q = w.query::<(&Rider, &Recovery, &Transform)>();
            let (r, c, t) = q.single(w).unwrap();
            (c.stage, r.health, r.speed, *t)
        };
        let a = state(&mut old);
        let b = state(&mut new);
        assert_eq!(a.0, b.0, "recovery timing changed at {tick}");
        assert!(
            (a.1 - b.1).abs() < 0.0001
                && (a.2 - b.2).abs() < 0.0001
                // Rebuilding the visual hierarchy changes entity ordering. Allow
                // 2 cm for contact-order/remount projection after gravity-led
                // wreck toppling; stages/velocities remain strict. Isolated
                // eleven-body render parity is tested separately above.
                && a.3.translation.distance(b.3.translation) < 0.02,
            "racer state changed at {tick}: old={a:?} new={b:?}"
        );
        match a.0 {
            RecoveryStage::Falling => visited[0] = true,
            RecoveryStage::GettingUp => visited[1] = true,
            RecoveryStage::Running => visited[2] = true,
            RecoveryStage::Mounting => visited[3] = true,
            RecoveryStage::Riding if visited[0] => {
                remounted = true;
                break;
            }
            _ => {}
        }
    }
    assert!(
        remounted && visited.iter().all(|s| *s),
        "full recovery not observed: {visited:?}"
    );
    let w = new.world_mut();
    assert_eq!(
        w.query::<&AuthoredHuman>().iter(w).count(),
        0,
        "detached skin leaked after remount"
    );
}

#[test]
fn visual_intake_rejects_nonfinite_vertices_bad_joint_ids_and_physics_payloads() {
    let size = u32::from_le_bytes(BUNDLE[8..12].try_into().unwrap()) as usize;
    let start = 12 + size;
    let mut corrupt = BUNDLE.to_vec();
    corrupt[start..start + 4].copy_from_slice(&f32::NAN.to_le_bytes());
    assert!(rr_content::riding_model::decode(&corrupt).is_err());
    let mut corrupt = BUNDLE.to_vec();
    corrupt[start + 48..start + 50].copy_from_slice(&11_u16.to_le_bytes());
    assert!(rr_content::riding_model::decode(&corrupt).is_err());
    let mut header: serde_json::Value = serde_json::from_slice(&BUNDLE[12..start]).unwrap();
    header["physics_data"] = serde_json::json!(true);
    let json = serde_json::to_vec(&header).unwrap();
    let mut corrupt = b"RDMESH01".to_vec();
    corrupt.extend_from_slice(&(json.len() as u32).to_le_bytes());
    corrupt.extend_from_slice(&json);
    corrupt.extend_from_slice(&BUNDLE[start..]);
    assert!(rr_content::riding_model::decode(&corrupt).is_err());
}

#[test]
fn visual_chains_keep_anatomical_endpoints_and_hinges_connected_in_every_pose() {
    let data = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    for (seated, frames) in [(false, data.standing), (true, data.seated)] {
        let source = frames.map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let legacy = std::array::from_fn(|i| {
            if seated {
                human::seated(i)
            } else {
                Transform::from_translation(human::SPECS[i].0)
            }
        });
        let rest = connected_pose(legacy, source, legacy);
        for i in 0..11 {
            assert!(rest[i].translation.distance(source[i].translation) < 0.0001);
            assert!(rest[i].rotation.angle_between(source[i].rotation) < 0.001);
        }
        for tick in 0..120 {
            let input = std::array::from_fn(|i| {
                Transform::from_translation(legacy[i].translation + Vec3::new(10.0, 2.0, -5.0))
                    .with_rotation(
                        Quat::from_euler(
                            EulerRot::XYZ,
                            (tick as f32 * 0.07 + i as f32).sin() * 3.0,
                            (i as f32 * 0.71).sin(),
                            (tick as f32 * 0.05).cos(),
                        ) * legacy[i].rotation,
                    )
            });
            let pose = connected_pose(input, source, legacy);
            for i in RIG_ORDER {
                let parent = rr_content::riding_model::RENDER_PARENTS[i];
                if parent < 0 {
                    continue;
                }
                let p = parent as usize;
                let expected = pose[p].translation
                    + pose[p].rotation
                        * (source[p].rotation.inverse()
                            * (source[i].translation - source[p].translation));
                assert!(
                    expected.distance(pose[i].translation) < 0.0001,
                    "detached joint {i}"
                );
                assert!(
                    (pose[i].translation.distance(pose[p].translation)
                        - source[i].translation.distance(source[p].translation))
                    .abs()
                        < 0.0001,
                    "stretched limb {i}"
                );
                if matches!(i, 4 | 6 | 8 | 10) {
                    let bend =
                        (pose[p].rotation * Vec3::Y).angle_between(pose[i].rotation * Vec3::Y);
                    assert!(
                        bend <= if i < 7 { 2.351 } else { 2.451 },
                        "impossible hinge {i}: {bend}"
                    );
                }
                assert!(pose[i].translation.is_finite() && pose[i].rotation.is_finite());
            }
        }
    }
}

#[test]
fn distal_skin_weights_are_normalized_and_all_five_joints_are_used() {
    let data = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    let mut used = [0usize; 16];
    for mut mesh in rr_content::riding_model::decode(BUNDLE).unwrap() {
        let frames = match mesh.group.as_str() {
            "standing" => data.standing,
            "seated" => data.seated,
            _ => continue,
        };
        let base = frames.map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let bind = articulation::bind_frames(base);
        articulation::weights(&mut mesh, &bind);
        for (joints, weights) in mesh.joints.iter().zip(&mesh.weights) {
            assert!((weights.iter().sum::<f32>() - 1.0).abs() < 1e-5);
            for (&j, &w) in joints.iter().zip(weights) {
                assert!(j < 16 && w.is_finite() && w >= 0.0);
                if w > 0.01 {
                    used[j as usize] += 1;
                }
            }
        }
        let mut physical = [Transform::IDENTITY; 16];
        for frame in physical.iter_mut().skip(11) {
            frame.rotation = Quat::from_rotation_x(0.4);
        }
        let pose = articulation::extend(base, bind, Some(&physical));
        for i in 11..16 {
            assert!(pose[i].rotation.angle_between(bind[i].rotation) > 0.39);
            assert!(pose[i].translation.distance(bind[i].translation) < 1e-5);
        }
    }
    for (i, n) in used.into_iter().enumerate().skip(11) {
        assert!(n > 10, "unused distal joint {i}: {n}");
    }
}

#[test]
fn neutral_distal_skin_joints_remain_attached_and_wrist_axes_follow_forearms() {
    let data = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    let lib = crate::animation::Library::default();
    for (seated, frames, id) in [(false, data.standing, 2364), (true, data.seated, 2020)] {
        let base = frames.map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let bind = articulation::bind_frames(base);
        let out = lib.skin_frames(lib.key(id), bind, seated);
        for (i, parent) in [(12, 4), (13, 6)] {
            let rest =
                Transform::from_matrix(bind[parent].to_matrix().inverse() * bind[i].to_matrix());
            let expected = out[parent].mul_transform(rest);
            assert!(
                out[i].rotation.angle_between(expected.rotation) < 0.001,
                "neutral wrist kink {i}"
            );
            assert!(
                out[i].translation.distance(expected.translation) < 1e-5,
                "detached wrist {i}"
            );
        }
    }
}

#[test]
fn glove_morphs_are_shared_bounded_and_never_deform_non_hand_vertices() {
    let mut meshes = Assets::<Mesh>::default();
    let mut mats = Assets::<StandardMaterial>::default();
    let mut binds = Assets::<SkinnedMeshInverseBindposes>::default();
    let mut images = Assets::<Image>::default();
    let models = AuthoredModels::load(&mut meshes, &mut mats, &mut binds, &mut images, None);
    let mut count = 0;
    let mut bytes = 0;
    for part in &models.parts {
        let mesh = meshes.get(&part.mesh).unwrap();
        let Some(handle) = mesh.morph_targets() else {
            continue;
        };
        count += 1;
        let image = images.get(handle).unwrap();
        let data = image.data.as_ref().unwrap();
        bytes += data.len();
        assert_eq!(image.texture_descriptor.size.depth_or_array_layers, 4);
        let VertexAttributeValues::Uint16x4(joints) =
            mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX).unwrap()
        else {
            panic!()
        };
        let VertexAttributeValues::Float32x4(weights) =
            mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT).unwrap()
        else {
            panic!()
        };
        let layer = data.len() / 4;
        let mut moved = [0; 4];
        for (target, moved) in moved.iter_mut().enumerate() {
            let hand = if target < 2 { 13 } else { 12 };
            for (i, joints) in joints.iter().enumerate() {
                let start = layer * target + i * 36;
                let values: Vec<f32> = data[start..start + 36]
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
                    .collect();
                assert!(values.iter().all(|v| v.is_finite()));
                let displacement = Vec3::new(values[0], values[1], values[2]);
                assert!(displacement.length() < 0.25);
                if displacement.length() > 0.00001 {
                    *moved += 1;
                    assert!(
                        joints
                            .iter()
                            .zip(weights[i])
                            .any(|(j, w)| *j == hand && w > 0.001)
                    );
                }
            }
            assert!(*moved > 100, "missing hand geometry in {target}");
        }
    }
    assert_eq!(count, 2, "one shared glove-morph surface per authored bind");
    assert!(
        bytes < 16 * 1024 * 1024,
        "unexpected morph texture growth: {bytes}"
    );
    println!("shared glove targets: {count} surfaces, {bytes} bytes");
}

#[test]
fn animated_boot_pivots_match_authored_ankles_across_contacts_and_transitions() {
    let source = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    let motion =
        rr_content::motion::Library::from_json(include_str!("../../assets/models/motion.json"))
            .unwrap();
    let lib = crate::animation::Library::default();
    let mut maximum = 0.0_f32;
    let mut flex_error = 0.0_f32;
    let mut contact_max = [0.0_f32; 2];
    let mut held_max = [0.0_f32; 2];
    for (seated, raw) in [(false, source.standing), (true, source.seated)] {
        let base = raw.map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let bind = articulation::bind_frames(base);
        let neutral = lib.key(if seated { 2020 } else { 2364 });
        let ref_frames = lib.frames(neutral);
        let ref_fk = lib.world_pose(neutral);
        for name in [
            "normal",
            "tuck",
            "run",
            "getup",
            "pickup",
            "pickup_mirrored",
            "steer_tuck_blend",
            "walk",
            "running",
            "stop",
        ] {
            let clip = motion.clips.iter().find(|c| c.name == name);
            let duration: f32 = clip.map_or(1.0, |c| c.durations.iter().sum());
            let mut errors = [0.0_f32; 3];
            for tick in 0..=120 {
                let fraction = tick as f32 / 120.0;
                let pose = if clip.is_some() {
                    lib.sample(name, duration * fraction)
                } else if name == "walk" || name == "running" || name == "stop" {
                    lib.locomotion(
                        fraction,
                        match name {
                            "walk" => 2.0,
                            "running" => 6.0,
                            _ => 6.0 * (1.0 - fraction),
                        },
                    )
                } else {
                    lib.key(2900)
                        .blend(lib.key(2030).blend(lib.key(2901), fraction), fraction)
                };
                let frames = lib.skin_frames(pose, bind, seated);
                let authored = lib.world_pose(pose);
                for (joint, parent, node, bit) in [
                    (14, 8, 19, 3),
                    (15, 10, 16, 2),
                    (12, 4, 13, 1),
                    (13, 6, 9, 0),
                ] {
                    let endpoint =
                        frames[parent].transform_point(Vec3::from_array(motion.offsets[node]));
                    maximum = maximum.max(endpoint.distance(frames[joint].translation));
                    let limb = motion.limbs.iter().find(|l| l.bit == bit).unwrap();
                    let reference_local =
                        ref_frames[parent].rotation.inverse() * ref_fk[node].rotation;
                    // A sole/grip landmark in source space, independently based
                    // on the actual authored endpoint rather than the skin pivot.
                    let source_contact = base[parent]
                        .transform_point(Vec3::from_array(motion.offsets[node]))
                        + bind[joint].rotation * reference_local * Vec3::from_array(limb.offset);
                    let deformed = (frames[joint].to_matrix() * bind[joint].to_matrix().inverse())
                        .transform_point3(source_contact);
                    let expected = authored[node].transform_point(Vec3::from_array(limb.offset));
                    let kind = usize::from(bit < 2);
                    let error = deformed.distance(expected);
                    contact_max[kind] = contact_max[kind].max(error);
                    errors[kind] = errors[kind].max(error);
                    if pose.held & (1 << bit) != 0 {
                        let error = deformed.distance(Vec3::from_array(limb.target));
                        held_max[kind] = held_max[kind].max(error);
                        errors[2] = errors[2].max(error);
                    }
                }
            }
            println!(
                "contact bind={seated} clip={name} sole={:.9} grip={:.9} held={:.9}",
                errors[0], errors[1], errors[2]
            );
        }
        for (joint, parent, node) in [(14, 8, 19), (15, 10, 16)] {
            let ankle = base[parent].transform_point(Vec3::from_array(motion.offsets[node]));
            let mut physical = [Transform::IDENTITY; 16];
            physical[joint].rotation = Quat::from_rotation_x(0.6);
            let posed = articulation::extend(base, bind, Some(&physical));
            let skin = posed[joint].to_matrix() * bind[joint].to_matrix().inverse();
            flex_error = flex_error.max(skin.transform_point3(ankle).distance(ankle));
        }
    }
    println!(
        "ankle_metrics {}",
        serde_json::json!({"pivot_m":maximum,"flex_m":flex_error,"sole_m":contact_max[0],"grip_m":contact_max[1],"held_sole_m":held_max[0],"held_grip_m":held_max[1]})
    );
    assert!(
        flex_error < 1e-5,
        "ankle surface drifts under distal flex: {flex_error}m"
    );
    assert!(
        maximum < 1e-5,
        "skin ankle detached from authored endpoint: {maximum}m"
    );
    assert!(
        contact_max.into_iter().all(|e| e < 1e-5),
        "sole/grip retarget drift"
    );
    assert!(
        held_max.into_iter().all(|e| e < 0.006),
        "held sole/grip drift"
    );
}

#[test]
fn authored_ankle_pivots_preserve_weights_and_neutral_mesh_surface() {
    let source = rr_content::riding_model::Bindings::from_json(include_str!(
        "../../assets/models/rider-bindings.json"
    ))
    .unwrap();
    let mut maximum = 0.0_f32;
    for (mesh, mut old) in rr_content::riding_model::decode(BUNDLE)
        .unwrap()
        .into_iter()
        .zip(rr_content::riding_model::decode(BUNDLE).unwrap())
    {
        let raw = match mesh.group.as_str() {
            "standing" => source.standing,
            "seated" => source.seated,
            _ => continue,
        };
        let base = raw.map(|m| Transform::from_matrix(Mat4::from_cols_array_2d(&m)));
        let bind = articulation::bind_frames(base);
        let mut legacy = bind;
        for (joint, parent) in [(14, 8), (15, 10)] {
            legacy[joint].translation = base[parent].transform_point(Vec3::Y * 0.50);
        }
        let mut new = mesh;
        articulation::weights(&mut old, &legacy);
        articulation::weights(&mut new, &bind);
        assert_eq!(old.joints, new.joints);
        assert_eq!(old.weights, new.weights);
        let neutral = articulation::extend(base, bind, None);
        let skin: [Mat4; 16] =
            std::array::from_fn(|i| neutral[i].to_matrix() * bind[i].to_matrix().inverse());
        for (i, point) in new.positions.iter().enumerate() {
            let point = Vec3::from_array(*point);
            let deformed: Vec3 = new.joints[i]
                .iter()
                .zip(new.weights[i])
                .map(|(j, w)| skin[*j as usize].transform_point3(point) * w)
                .sum();
            maximum = maximum.max(point.distance(deformed));
        }
    }
    println!("neutral source surface max error: {maximum:.9}m; weights identical");
    assert!(maximum < 1e-5);
}

#[test]
fn patrol_presentation_is_exclusive_and_persists_on_wrecks_without_physics() {
    use bevy::ecs::system::RunSystemOnce;
    use bevy_rapier3d::prelude::*;
    let mut app = crate::game::tests::simulation();
    authored(&mut app);
    let art = app.world().resource::<world::Art>();
    let models = art.models.as_ref().unwrap();
    let equipment = models
        .parts
        .iter()
        .filter(|p| p.group == "patrol-equipment")
        .count();
    assert!(equipment >= 16);
    for p in models
        .parts
        .iter()
        .filter(|p| matches!(p.group.as_str(), "standing" | "seated"))
    {
        assert!(!p.materials.contains(&p.officer_material));
    }
    let base = models
        .parts
        .iter()
        .filter(|p| !matches!(p.group.as_str(), "standing" | "seated" | "patrol-equipment"))
        .count();
    for palette in (0..16).chain([OFFICER_PALETTE]) {
        let entities = app
            .world_mut()
            .run_system_once(move |mut commands: Commands, art: Res<world::Art>| {
                art.models
                    .as_ref()
                    .unwrap()
                    .motorcycle(&mut commands, palette)
            })
            .unwrap();
        assert_eq!(
            entities.len(),
            base + if palette == OFFICER_PALETTE {
                equipment - 1
            } else {
                0
            }
        );
        for entity in entities {
            let e = app.world().entity(entity);
            assert!(
                e.get::<RigidBody>().is_none()
                    && e.get::<Collider>().is_none()
                    && e.get::<ImpulseJoint>().is_none()
            );
        }
    }
    // Actual officer factory selects the reserved identity, never a roster swatch.
    app.world_mut()
        .run_system_once(crate::police::setup)
        .unwrap();
    let w = app.world_mut();
    assert_eq!(
        w.query::<&crate::game::Rider>()
            .iter(w)
            .find(|r| r.police)
            .unwrap()
            .palette_index,
        OFFICER_PALETTE
    );
}
