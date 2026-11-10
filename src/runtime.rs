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
pub(crate) fn interpolate_visuals(
    time: Res<Time<Fixed>>,
    roots: Query<
        (
            &Transform,
            &PoseHistory,
            Option<&crate::online::replication::NetworkPose>,
        ),
        Without<VisualRoot>,
    >,
    mut visuals: Query<(&VisualRoot, &mut Transform)>,
) {
    for (visual, mut local) in &mut visuals {
        if let Ok((physical, history, network)) = roots.get(visual.owner) {
            let shown = network.map_or_else(
                || interpolated(physical, Some(history), time.overstep_fraction()),
                |n| n.shown(),
            );
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
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn loaded(
    content: Res<crate::content::GameContent>,
    mut next: ResMut<NextState<AppScreen>>,
    art: Option<Res<world::Art>>,
    city: Option<Res<crate::world_gen::city::visuals::ModelCatalog>>,
    (architecture, details, tangents, identity): (
        Option<Res<crate::world_gen::architecture::render::Streaming>>,
        Option<Res<crate::material_detail::Queue>>,
        Option<Res<crate::tangent_detail::Queue>>,
        Option<Res<crate::world_gen::architecture::identity::Identity>>,
    ),
    server: Option<Res<AssetServer>>,
    metro: Option<Res<crate::world_gen::metropolis::facades::Kit>>,
    props: Query<
        &SceneRoot,
        Or<(
            With<crate::props::PropVisual>,
            With<crate::props::ActorBinding>,
        )>,
    >,
) {
    if metro.as_ref().is_some_and(|k| !k.ready()) {
        return;
    }
    if identity.as_ref().is_some_and(|i| !i.ready()) {
        return;
    }
    if server.is_some()
        && (details.as_ref().is_some_and(|d| !d.settled())
            || tangents.as_ref().is_some_and(|t| !t.settled()))
    {
        return;
    }
    if let (Some(architecture), Some(server)) = (&architecture, &server)
        && !architecture.ready(server)
    {
        return;
    }
    if let (Some(city), Some(server)) = (&city, &server)
        && !city.ready(server)
    {
        return;
    }
    if let (Some(art), Some(server)) = (&art, &server)
        && let Some(models) = &art.models
        && !models.ready(server)
    {
        return;
    }
    if let Some(server) = &server {
        for scene in &props {
            if let Some(bevy::asset::LoadState::Failed(error)) = server.get_load_state(scene.id()) {
                panic!("Required prop mesh failed to load: {error}");
            }
            if !server.is_loaded_with_dependencies(scene.id()) {
                return;
            }
        }
    }
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
        let substeps = app
            .world()
            .get_resource::<crate::biomechanics::Settings>()
            .filter(|s| s.enabled)
            .map_or(3, |s| s.config.physics_substeps);
        app.add_plugins(crate::biomechanics::BiomechanicsPlugin)
            .init_resource::<crate::vehicle::VehiclePhysicsAdapter>()
            .init_resource::<crate::vehicle::Tuning>()
            .init_resource::<crate::vehicle::VehicleTelemetry>()
            .init_resource::<Race>()
            .init_resource::<crate::police::Police>()
            .init_resource::<crate::combat::StrikeTimings>()
            .init_resource::<crate::weapons::WeaponRules>()
            .init_resource::<crate::weapons::GridWeapons>()
            .init_resource::<game::Profile>()
            .init_resource::<game::Persistence>()
            .init_resource::<SimulationClock>()
            .init_resource::<crate::contact_telemetry::VehicleContacts>()
            .init_resource::<recovery::RecoveryTuning>()
            .init_resource::<crate::track::Track>()
            .add_message::<crate::sound::AudioCue>()
            .add_message::<recovery::SweptContact>()
            .add_message::<crate::combat::AttackAttempt>()
            .add_message::<crate::combat::AttackIntent>()
            .add_message::<crate::combat::StrikeContact>()
            .add_message::<crate::combat::NpcHit>()
            .insert_resource(Time::<Fixed>::from_hz(rr_core::SIMULATION_HZ))
            .insert_resource(TimestepMode::Fixed {
                dt: 1.0 / rr_core::SIMULATION_HZ as f32,
                substeps,
            })
            .add_plugins(
                RapierPhysicsPlugin::<crate::crash_behavior::ReleaseHooks>::default()
                    .in_fixed_schedule(),
            )
            .configure_sets(
                FixedUpdate,
                (
                    PhysicsSet::SyncBackend,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Writeback,
                )
                    .run_if(crate::online::simulates),
            )
            .configure_sets(
                FixedUpdate,
                SimSet::Gameplay
                    .before(PhysicsSet::SyncBackend)
                    .run_if(crate::online::simulates),
            )
            .configure_sets(
                FixedUpdate,
                SimSet::AfterPhysics
                    .after(PhysicsSet::Writeback)
                    .run_if(crate::online::simulates),
            )
            .add_systems(
                FixedPreUpdate,
                (
                    crate::vehicle::attach.run_if(crate::online::simulates),
                    remember_previous.run_if(crate::online::simulates),
                    crate::crash_behavior::prepare.run_if(crate::online::simulates),
                )
                    .chain(),
            )
            .add_systems(
                FixedUpdate,
                (
                    crate::offscreen_crash::classify,
                    recovery::impacts,
                    recovery::recover,
                    crate::weapons::inventory,
                    game::drive,
                    crate::combat::schedule,
                    crate::combat::melee,
                    crate::npc::knockdowns,
                    crate::police::think,
                    crate::police::dismount,
                    crate::environment::patrol,
                    crate::world_gen::metropolis::pedestrians::update,
                    crate::world_gen::metropolis::crossing_traffic,
                    game::debris_cleanup,
                    recovery::slope_friction,
                    crate::crash_behavior::stagger,
                    crate::vehicle::forces,
                    recovery::remember_owned_motion,
                )
                    .chain()
                    .in_set(SimSet::Gameplay),
            )
            .add_systems(
                FixedUpdate,
                (
                    crate::world_gen::hills::mounted::update,
                    crate::heavy_traffic::update,
                )
                    .after(SimSet::Gameplay)
                    .before(PhysicsSet::SyncBackend),
            )
            .add_systems(Update, crate::world_gen::hills::river::animate)
            .add_systems(
                PostUpdate,
                (
                    crate::world_gen::hills::atmosphere::reset,
                    crate::world_gen::hills::atmosphere::animate,
                )
                    .chain()
                    .run_if(resource_exists::<crate::world_gen::hills::atmosphere::Atmosphere>)
                    .after(game::camera)
                    .before(bevy::transform::TransformSystems::Propagate),
            )
            .add_systems(
                FixedUpdate,
                (
                    recovery::constrain_crash_energy,
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
            .add_systems(
                Update,
                loaded
                    .after(crate::material_detail::update)
                    .after(crate::tangent_detail::update)
                    .run_if(in_state(AppScreen::Loading)),
            )
            .add_systems(Update, screen);
    }
}
pub struct RoadWorldPlugin;
impl Plugin for RoadWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::material_detail::Queue>()
            .init_resource::<crate::tangent_detail::Queue>()
            .add_systems(
                Update,
                crate::tangent_detail::update
                    .after(crate::material_detail::update)
                    .run_if(resource_exists::<AssetServer>),
            )
            .add_systems(
                Update,
                crate::material_detail::update.run_if(resource_exists::<AssetServer>),
            )
            .add_systems(
                Startup,
                (
                    crate::world_gen::setup_plan,
                    world::setup_world,
                    world::validate_scenery,
                    crate::world_gen::report::capture_baseline,
                )
                    .chain()
                    .in_set(StartupSet::World),
            )
            .add_systems(
                Update,
                (
                    crate::world_gen::metropolis::facades::prepare,
                    crate::world_gen::metropolis::facades::update,
                )
                    .chain()
                    .after(game::camera)
                    .run_if(resource_exists::<crate::world_gen::metropolis::facades::Kit>),
            )
            .add_systems(
                PostUpdate,
                crate::world_gen::metropolis::facades::synchronize
                    .after(bevy::transform::TransformSystems::Propagate)
                    .before(bevy::camera::visibility::VisibilitySystems::CheckVisibility)
                    .run_if(resource_exists::<crate::world_gen::metropolis::facades::Kit>),
            )
            .add_systems(
                Update,
                crate::world_gen::architecture::identity::prepare
                    .run_if(resource_exists::<AssetServer>),
            )
            .add_systems(
                Update,
                (
                    crate::world_gen::city::visuals::update_lod,
                    crate::world_gen::city::visuals::fade,
                )
                    .chain()
                    .run_if(resource_exists::<crate::world_gen::city::visuals::ModelCatalog>),
            )
            .add_systems(
                Update,
                (
                    crate::world_gen::architecture::render::update,
                    crate::world_gen::architecture::render::fade,
                )
                    .chain()
                    .after(game::camera)
                    .run_if(resource_exists::<crate::world_gen::architecture::render::Streaming>),
            )
            .add_systems(
                PostUpdate,
                crate::world_gen::architecture::render::synchronize_fades
                    .after(bevy::transform::TransformSystems::Propagate)
                    .before(bevy::camera::visibility::VisibilitySystems::CheckVisibility),
            )
            .add_systems(
                PostStartup,
                (
                    crate::road_detail::install,
                    crate::sky::install,
                    crate::world_gen::dressing::spawn,
                    crate::material_detail::paving,
                ),
            )
            .add_systems(
                Startup,
                (game::setup_race, crate::police::setup).in_set(StartupSet::Race),
            )
            .init_resource::<crate::environment::TrackInspector>()
            .add_systems(
                Update,
                (crate::environment::inspector, crate::world_gen::preview).chain(),
            );
    }
}
pub struct RoadPresentationPlugin;
impl Plugin for RoadPresentationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::ui::MenuAction>()
            .add_systems(
                Startup,
                (crate::ui::setup_ui, crate::settings::setup).in_set(StartupSet::Ui),
            )
            .add_systems(Update, (crate::settings::display, crate::feedback::update))
            .add_systems(Startup, crate::feedback::setup.in_set(StartupSet::Ui))
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
