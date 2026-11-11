//! Prepare only normal-mapped render meshes, in bounded background batches.
use bevy::{
    mesh::VertexAttributeValues,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future},
};
use std::collections::HashSet;
type Tangents = Vec<(AssetId<Mesh>, Option<Vec<[f32; 4]>>)>;
struct Job {
    task: Task<Tangents>,
}
#[derive(Resource, Default)]
pub struct Queue {
    pending: HashSet<AssetId<Mesh>>,
    done: HashSet<AssetId<Mesh>>,
    jobs: Vec<Job>,
    pub generated: usize,
    pub unsupported: usize,
}
impl Queue {
    pub(crate) fn loading_evidence(&self) -> serde_json::Value {
        serde_json::json!({"pending":self.pending.len(),"jobs":self.jobs.len(),"done":self.done.len()})
    }
    pub fn settled(&self) -> bool {
        self.pending.is_empty() && self.jobs.is_empty()
    }
}
#[allow(clippy::type_complexity)]
pub fn update(
    mut events: MessageReader<AssetEvent<Mesh>>,
    mut material_events: MessageReader<AssetEvent<StandardMaterial>>,
    materials: Res<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut queue: ResMut<Queue>,
    instances: Query<(&Mesh3d, &MeshMaterial3d<StandardMaterial>)>,
    new_instances: Query<
        (&Mesh3d, &MeshMaterial3d<StandardMaterial>),
        Or<(Added<Mesh3d>, Changed<MeshMaterial3d<StandardMaterial>>)>,
    >,
) {
    for event in events.read() {
        if let AssetEvent::Removed { id } = event {
            queue.pending.remove(id);
            queue.done.remove(id);
        }
    }
    let changed: HashSet<_> = material_events
        .read()
        .filter_map(|e| match e {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => materials
                .get(*id)
                // Alpha-only dust animation cannot require tangent generation.
                // A newly assigned normal map still admits existing mesh instances.
                .is_some_and(|material| material.normal_map_texture.is_some())
                .then_some(*id),
            _ => None,
        })
        .collect();
    let mut candidates = Vec::new();
    if !changed.is_empty() {
        candidates.extend(
            instances
                .iter()
                .filter(|(_, m)| changed.contains(&m.0.id()))
                .map(|(mesh, mat)| (mesh.0.id(), mat.0.id())),
        );
    }
    candidates.extend(
        new_instances
            .iter()
            .map(|(mesh, mat)| (mesh.0.id(), mat.0.id())),
    );
    for (id, mat) in candidates {
        if !queue.done.contains(&id)
            && materials
                .get(mat)
                .is_some_and(|m| m.normal_map_texture.is_some())
        {
            queue.pending.insert(id);
        }
    }
    let mut i = 0;
    while i < queue.jobs.len() {
        if let Some(results) = block_on(future::poll_once(&mut queue.jobs[i].task)) {
            queue.jobs.swap_remove(i);
            for (id, result) in results {
                if let Some(mesh) = meshes.get_mut(id) {
                    if let Some(tangents) = result {
                        mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangents);
                        queue.generated += 1;
                    } else {
                        queue.unsupported += 1;
                    }
                }
            }
        } else {
            i += 1;
        }
    }
    while queue.jobs.len() < 2 && !queue.pending.is_empty() {
        let pending: Vec<_> = queue.pending.iter().copied().collect();
        let mut batch = Vec::new();
        for id in pending {
            queue.pending.remove(&id);
            if queue.done.contains(&id) {
                continue;
            }
            queue.done.insert(id);
            let Some(mesh) = meshes.get(id) else { continue };
            if mesh.contains_attribute(Mesh::ATTRIBUTE_TANGENT)
                || !mesh.contains_attribute(Mesh::ATTRIBUTE_UV_0)
                || !mesh.contains_attribute(Mesh::ATTRIBUTE_NORMAL)
            {
                continue;
            }
            let large = mesh.count_vertices() > 20000;
            batch.push((id, mesh.clone()));
            if large || batch.len() >= 12 {
                break;
            }
        }
        if batch.is_empty() {
            continue;
        }
        let task = AsyncComputeTaskPool::get().spawn(async move {
            batch
                .into_iter()
                .map(|(id, mut mesh)| {
                    let result = if mesh.generate_tangents().is_ok() {
                        match mesh.attribute(Mesh::ATTRIBUTE_TANGENT) {
                            Some(VertexAttributeValues::Float32x4(t)) => Some(t.clone()),
                            _ => None,
                        }
                    } else {
                        None
                    };
                    (id, result)
                })
                .collect()
        });
        queue.jobs.push(Job { task });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tangent_preparation_keeps_mesh_positions_unchanged() {
        let mut m = Mesh::from(Cuboid::new(2.0, 3.0, 4.0));
        let before = m.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().clone();
        m.generate_tangents().unwrap();
        let Some(VertexAttributeValues::Float32x3(a)) = m.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!()
        };
        let VertexAttributeValues::Float32x3(b) = before else {
            panic!()
        };
        assert_eq!(*a, b);
        assert!(m.contains_attribute(Mesh::ATTRIBUTE_TANGENT));
    }
    #[test]
    fn adding_a_normal_map_to_an_existing_material_prepares_its_mesh() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Queue>()
            .add_message::<AssetEvent<Mesh>>()
            .add_message::<AssetEvent<StandardMaterial>>()
            .add_systems(Update, update);
        let mut geometry = Mesh::from(Rectangle::new(2., 2.));
        geometry.remove_attribute(Mesh::ATTRIBUTE_TANGENT);
        let positions = geometry
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .clone();
        let mesh = app.world_mut().resource_mut::<Assets<Mesh>>().add(geometry);
        let material = app
            .world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial::default());
        app.world_mut()
            .spawn((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone())));
        app.update();
        app.update();
        // Existing dust-like materials may emit alpha changes continuously, but
        // none of their existing geometry should enter the tangent job queue.
        app.world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .get_mut(&material)
            .unwrap()
            .base_color
            .set_alpha(0.2);
        app.world_mut()
            .write_message(AssetEvent::<StandardMaterial>::Modified { id: material.id() });
        app.update();
        assert_eq!(app.world().resource::<Queue>().generated, 0);
        assert!(app.world().resource::<Queue>().pending.is_empty());
        assert!(app.world().resource::<Queue>().done.is_empty());
        // Neither Mesh3d nor its material handle changes here. Only the material
        // event can discover this already-existing mesh after normal-map intake.
        app.world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .get_mut(&material)
            .unwrap()
            .normal_map_texture = Some(Handle::default());
        app.world_mut()
            .write_message(AssetEvent::<StandardMaterial>::Modified { id: material.id() });
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while app.world().resource::<Queue>().generated == 0 && std::time::Instant::now() < deadline
        {
            app.update();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        assert_eq!(app.world().resource::<Queue>().generated, 1);
        let meshes = app.world().resource::<Assets<Mesh>>();
        let prepared = meshes.get(&mesh).unwrap();
        let Some(VertexAttributeValues::Float32x3(actual)) =
            prepared.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!()
        };
        let VertexAttributeValues::Float32x3(expected) = positions else {
            panic!()
        };
        assert_eq!(*actual, expected);
        assert!(prepared.contains_attribute(Mesh::ATTRIBUTE_TANGENT));
    }
}
