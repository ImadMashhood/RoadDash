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
            AssetEvent::Added { id } | AssetEvent::Modified { id } => Some(*id),
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
}
