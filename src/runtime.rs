//! App lifecycle, explicit 60 Hz simulation schedule, and render-only interpolation.
use crate::{
    game::{self, Phase, Race},
    recovery, world,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(States, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AppScreen {
    #[default]
    Boot,
    Loading,
    MainMenu,
    Race,
    Results,
}
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StartupSet {
    World,
    Race,
    Ui,
    Audio,
}
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimSet {
    Gameplay,
    AfterPhysics,
}
#[derive(Resource, Default)]
pub struct SimulationClock {
    pub ticks: u64,
    pub active_ticks: u64,
}
#[derive(Component, Clone, Copy)]
pub struct PoseHistory {
    pub previous: Transform,
    pub current: Transform,
}
#[derive(Component)]
pub struct VisualRoot {
    pub owner: Entity,
}

pub fn attach_visuals(commands: &mut Commands, owner: Entity, children: &[Entity]) -> Entity {
    let visual = commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            VisualRoot { owner },
        ))
        .id();
    commands.entity(visual).add_children(children);
    commands.entity(owner).add_child(visual);
    visual
}
pub fn interpolated(transform: &Transform, history: Option<&PoseHistory>, alpha: f32) -> Transform {
    if let Some(history) = history {
        Transform {
            translation: history
                .previous
                .translation
                .lerp(history.current.translation, alpha),
            rotation: history
                .previous
                .rotation
                .slerp(history.current.rotation, alpha),
            scale: transform.scale,
        }
    } else {
        *transform
    }
}
fn remember_previous(
    mut commands: Commands,
    mut bodies: Query<(Entity, &RigidBody, &Transform, Option<&mut PoseHistory>)>,
) {
    for (e, body, t, history) in &mut bodies {
        if *body == RigidBody::Fixed {
            continue;
        }
        if let Some(mut h) = history {
            h.previous = h.current;
        } else {
            commands.entity(e).insert(PoseHistory {
                previous: *t,
                current: *t,
            });
        }
    }
}
fn remember_current(mut bodies: Query<(&Transform, &mut PoseHistory)>) {
    for (t, mut h) in &mut bodies {
        h.current = *t;
        if h.previous.translation.distance(t.translation) > 20.0 {
            h.previous = *t;
        }
    }
}
fn interpolate_visuals(
    time: Res<Time<Fixed>>,
    roots: Query<(&Transform, &PoseHistory), Without<VisualRoot>>,
    mut visuals: Query<(&VisualRoot, &mut Transform)>,
) {
    for (visual, mut local) in &mut visuals {
        if let Ok((physical, history)) = roots.get(visual.owner) {
            let shown = interpolated(physical, Some(history), time.overstep_fraction());
            local.translation =
                physical.rotation.inverse() * (shown.translation - physical.translation);
            local.rotation = physical.rotation.inverse() * shown.rotation;
        }
    }
}
fn tick(mut clock: ResMut<SimulationClock>, race: Res<Race>) {
    clock.ticks += 1;
    if race.phase.simulating() {
        clock.active_ticks += 1;
    }
}
fn boot(mut next: ResMut<NextState<AppScreen>>) {
    next.set(AppScreen::Loading);
}
fn loaded(content: Res<crate::content::GameContent>, mut next: ResMut<NextState<AppScreen>>) {
    content.0.validate().expect("validated game content");
    next.set(AppScreen::MainMenu);
}
fn screen(race: Res<Race>, state: Res<State<AppScreen>>, mut next: ResMut<NextState<AppScreen>>) {
    if matches!(state.get(), AppScreen::Boot | AppScreen::Loading) {
        return;
    }
    let target = match race.phase {
        Phase::Garage => AppScreen::MainMenu,
        Phase::Finished => AppScreen::Results,
        _ => AppScreen::Race,
    };
    if *state.get() != target {
        next.set(target);
    }
}
pub struct RoadSimulationPlugin;
impl Plugin for RoadSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::vehicle::VehiclePhysicsAdapter>()
            .init_resource::<crate::vehicle::Tuning>()
            .init_resource::<crate::vehicle::VehicleTelemetry>()
            .init_resource::<Race>()
            .init_resource::<game::Profile>()
            .init_resource::<game::Persistence>()
            .init_resource::<SimulationClock>()
            .init_resource::<crate::track::Track>()
            .add_message::<crate::sound::AudioCue>()
            .add_message::<recovery::SweptContact>()
            .add_message::<crate::combat::AttackAttempt>()
            .add_message::<crate::combat::NpcHit>()
            .insert_resource(Time::<Fixed>::from_hz(rr_core::SIMULATION_HZ))
            .insert_resource(TimestepMode::Fixed {
                dt: 1.0 / rr_core::SIMULATION_HZ as f32,
                substeps: 3,
            })
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule())
            .configure_sets(
                FixedUpdate,
                SimSet::Gameplay.before(PhysicsSet::SyncBackend),
            )
            .configure_sets(
                FixedUpdate,
                SimSet::AfterPhysics.after(PhysicsSet::Writeback),
            )
            .add_systems(
                FixedPreUpdate,
                (crate::vehicle::attach, remember_previous).chain(),
            )
            .add_systems(
                FixedUpdate,
                (
                    recovery::impacts,
                    recovery::recover,
                    game::drive,
                    crate::combat::melee,
                    crate::npc::knockdowns,
                    crate::environment::patrol,
                    game::debris_cleanup,
                    recovery::slope_friction,
                    crate::vehicle::forces,
                )
                    .chain()
                    .in_set(SimSet::Gameplay),
            )
            .add_systems(
                FixedUpdate,
                (
                    crate::vehicle::readback,
                    crate::vehicle::telemetry,
                    remember_current,
                    tick,
                )
                    .chain()
                    .in_set(SimSet::AfterPhysics),
            )
            .add_systems(
                Update,
                (
                    interpolate_visuals,
                    crate::vehicle::visuals,
                    crate::vehicle::live_tuning,
                )
                    .chain(),
            );
    }
}
pub struct RoadLifecyclePlugin;
impl Plugin for RoadLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppScreen>()
            .configure_sets(
                Startup,
                (
                    StartupSet::World,
                    StartupSet::Race,
                    StartupSet::Ui,
                    StartupSet::Audio,
                )
                    .chain(),
            )
            .add_systems(OnEnter(AppScreen::Boot), boot)
            .add_systems(OnEnter(AppScreen::Loading), loaded)
            .add_systems(Update, screen);
    }
}
pub struct RoadWorldPlugin;
impl Plugin for RoadWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (world::setup_world, world::validate_scenery)
                .chain()
                .in_set(StartupSet::World),
        )
        .add_systems(Startup, game::setup_race.in_set(StartupSet::Race))
        .init_resource::<crate::environment::TrackInspector>()
        .add_systems(Update, crate::environment::inspector);
    }
}
pub struct RoadPresentationPlugin;
impl Plugin for RoadPresentationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::ui::MenuAction>()
            .add_systems(Startup, crate::ui::setup_ui.in_set(StartupSet::Ui))
            .add_systems(
                Update,
                (
                    world::animate_riders,
                    world::animate_runners,
                    crate::environment::animate_civilians,
                    game::camera_occlusion,
                    crate::ui::update_ui,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                game::camera.before(bevy::transform::TransformSystems::Propagate),
            );
    }
}
pub struct RoadAudioPlugin;
impl Plugin for RoadAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::sound::Muted>()
            .add_systems(Startup, crate::sound::setup.in_set(StartupSet::Audio))
            .add_systems(Update, crate::sound::update);
    }
}
