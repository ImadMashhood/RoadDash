//! Sparse writes into stable GPU allocations. Asset mutation would replace buffers
//! and leave existing material bind groups pointing at the old allocations.
use super::*;
use bevy::{
    asset::AssetId,
    render::{
        Extract, ExtractSchedule, Render, RenderApp, RenderSystems,
        render_asset::{RenderAssets, prepare_assets},
        renderer::RenderQueue,
        storage::GpuShaderStorageBuffer,
    },
};
#[derive(Resource, Default)]
pub(super) struct Writes {
    pub epoch: u64,
    pub data: Vec<(AssetId<ShaderStorageBuffer>, Vec<u8>)>,
}
#[derive(Resource, Default)]
struct Pending {
    epoch: u64,
    data: HashMap<AssetId<ShaderStorageBuffer>, Vec<u8>>,
}
pub(super) fn install(app: &mut App) {
    app.init_resource::<Writes>();
    if let Some(render) = app.get_sub_app_mut(RenderApp) {
        render
            .init_resource::<Pending>()
            .add_systems(ExtractSchedule, extract)
            .add_systems(
                Render,
                flush
                    .in_set(RenderSystems::PrepareAssets)
                    .after(prepare_assets::<GpuShaderStorageBuffer>),
            );
    }
}
fn extract(writes: Extract<Res<Writes>>, mut pending: ResMut<Pending>) {
    if writes.epoch == pending.epoch {
        return;
    }
    pending.epoch = writes.epoch;
    for (id, bytes) in &writes.data {
        pending.data.insert(*id, bytes.clone());
    }
}
fn flush(
    mut pending: ResMut<Pending>,
    buffers: Res<RenderAssets<GpuShaderStorageBuffer>>,
    queue: Res<RenderQueue>,
) {
    pending.data.retain(|id, bytes| {
        let Some(buffer) = buffers.get(*id) else {
            return true;
        };
        queue.write_buffer(&buffer.buffer, 0, bytes);
        false
    });
}
pub(super) fn bytes(values: impl IntoIterator<Item = f32>) -> Vec<u8> {
    values.into_iter().flat_map(f32::to_le_bytes).collect()
}
pub(super) fn buffer(values: Vec<Vec4>) -> ShaderStorageBuffer {
    let mut buffer = ShaderStorageBuffer::from(values);
    buffer.buffer_description.usage |= bevy::render::render_resource::BufferUsages::COPY_DST;
    buffer
}
