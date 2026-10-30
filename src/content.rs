use bevy::prelude::*;
use rr_content::Catalog;
#[derive(Resource)]
pub struct GameContent(pub Catalog);
impl Default for GameContent {
    fn default() -> Self {
        Self(Catalog::builtin())
    }
}
#[derive(Component)]
pub struct ContentIdentity {
    pub rider: rr_core::ContentId,
    pub bike: rr_core::ContentId,
}
pub struct RoadContentPlugin;
impl Plugin for RoadContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameContent>();
    }
}
