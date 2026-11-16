//! Opt-in visual and audio comparisons. These systems only read the race.
use crate::{
    controls::Controls,
    game::{ChaseCamera, Phase, Race, Rider},
    recovery::{Recovery, RecoveryStage},
    runtime::AppScreen,
    vehicle::BikeState,
};
use bevy::{
    core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    light::NotShadowCaster,
    prelude::*,
    render::{RenderApp, extract_component::ExtractComponent, render_resource::ShaderType},
    shader::ShaderRef,
};

#[derive(Resource, Default)]
pub(crate) struct Options {
    pub(crate) rev: bool,
    pub(crate) exhaust: bool,
    pub(crate) shake: bool,
    grade: bool,
    wind: bool,
    heat: bool,
    pub(crate) hanabi: bool,
}

impl Options {
    fn from_flags(flags: &str) -> Self {
        let has = |name| flags.split(',').any(|flag| flag.trim() == name);
        let all = has("all");
        Self {
            rev: all || has("rev"),
            exhaust: all || has("exhaust"),
            shake: all || has("shake"),
            grade: all || has("grade"),
            wind: all || has("wind"),
            heat: all || has("heat"),
            hanabi: has("hanabi"),
        }
    }
}

#[derive(Resource)]
pub(crate) struct GridEngine(pub(crate) f32);

const IDLE_RPM: f32 = 1450.;

impl Default for GridEngine {
    fn default() -> Self {
        Self(IDLE_RPM)
    }
}

fn free_rev(rpm: f32, throttle: f32, dt: f32) -> f32 {
    let target = IDLE_RPM + throttle.clamp(0., 1.).powf(0.8) * 6000.;
    let response = if target > rpm { 6. } else { 3.5 };
    rpm.lerp(target, 1. - (-dt * response).exp())
}

fn grid_engine(
    time: Res<Time<Fixed>>,
    race: Res<Race>,
    controls: Res<Controls>,
    mut rev: ResMut<GridEngine>,
) {
    if race.phase == Phase::Countdown {
        rev.0 = free_rev(
            rev.0,
            controls.buffer.held.throttle_strength(),
            time.delta_secs(),
        );
    } else if race.phase != Phase::Paused {
        rev.0 = IDLE_RPM;
    }
}

#[derive(Component)]
struct GridTachometer;

fn tachometer(
    mut commands: Commands,
    race: Res<Race>,
    screen: Res<State<AppScreen>>,
    engine: Res<GridEngine>,
    mut labels: Query<(Entity, &mut Text), With<GridTachometer>>,
) {
    let visible = *screen.get() == AppScreen::Race && race.phase == Phase::Countdown;
    let value = format!("{:.0} RPM", engine.0);
    if let Ok((entity, mut text)) = labels.single_mut() {
        if visible {
            if text.0 != value {
                text.0 = value;
            }
        } else {
            commands.entity(entity).despawn();
        }
    } else if visible {
        commands.spawn((
            GridTachometer,
            Text::new(value),
            TextFont {
                font_size: FontSize::Px(21.),
                ..default()
            },
            TextColor(Color::srgb(0.87, 0.94, 0.92)),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(40.),
                bottom: Val::Px(204.),
                ..default()
            },
        ));
    }
}

fn local_id(online: Option<&crate::online::NetState>) -> usize {
    online.map_or(0, |state| state.local_slot)
}

fn tail(pose: &Transform) -> Vec3 {
    pose.transform_point(Vec3::new(0.31, 0.64, -1.18))
}

#[derive(Component, ExtractComponent, Clone, Copy, Default, ShaderType)]
#[extract_app(RenderApp)]
struct RoadEffects {
    weights: Vec4,
    heat_center: Vec4,
}

impl FullscreenMaterial for RoadEffects {
    fn fragment_shader() -> ShaderRef {
        "shaders/road_polish.wesl".into()
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn camera_effects(
    mut commands: Commands,
    time: Res<Time>,
    options: Res<Options>,
    race: Res<Race>,
    screen: Res<State<AppScreen>>,
    preferences: Res<crate::settings::Preferences>,
    controls: Res<Controls>,
    online: Option<Res<crate::online::NetState>>,
    riders: Query<(&Rider, &Recovery, &Transform), Without<ChaseCamera>>,
    mut cameras: Query<(Entity, &Camera, &Transform, Option<&mut RoadEffects>), With<ChaseCamera>>,
) {
    let found = riders
        .iter()
        .find(|(r, _, _)| r.id == local_id(online.as_deref()));
    let comfort = &preferences.file.comfort;
    for (entity, camera, transform, current) in &mut cameras {
        let mut weights = Vec4::ZERO;
        let mut heat_center = Vec4::ZERO;
        if *screen.get() == AppScreen::Race
            && !matches!(race.phase, Phase::Paused | Phase::Garage | Phase::Finished)
        {
            weights.x = f32::from(options.grade);
            if let Some((rider, recovery, pose)) = found
                && !comfort.anti_nausea
                && recovery.stage == RecoveryStage::Riding
            {
                if options.wind && race.phase == Phase::Racing {
                    weights.y = ((rider.speed - 14.) / 35.).clamp(0., 1.);
                }
                if options.heat
                    && let Some(ndc) =
                        camera.world_to_ndc(&GlobalTransform::from(*transform), tail(pose))
                {
                    let uv = Vec2::new(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
                    if ndc.z >= 0. && uv.cmpge(Vec2::ZERO).all() && uv.cmple(Vec2::ONE).all() {
                        heat_center = uv.extend(0.).extend(0.);
                        let throttle = if race.phase == Phase::Countdown {
                            controls.buffer.held.throttle_strength()
                        } else {
                            rider.drive_intent.throttle_strength()
                        };
                        weights.z = throttle;
                    }
                }
            }
        }
        let active = weights.truncate() != Vec3::ZERO;
        weights.w = time.elapsed_secs();
        if active {
            let effect = RoadEffects {
                weights,
                heat_center,
            };
            if let Some(mut current) = current {
                *current = effect;
            } else {
                commands.entity(entity).insert(effect);
            }
        } else if current.is_some() {
            commands.entity(entity).remove::<RoadEffects>();
        }
    }
}

const PUFF_LIFE: f32 = 0.58;
const PUFF_SLOTS: usize = 10;

#[derive(Component)]
pub(crate) struct Puff {
    age: f32,
    velocity: Vec3,
}

#[derive(Resource)]
pub(crate) struct Exhaust {
    next: f32,
    materials: [Handle<StandardMaterial>; PUFF_SLOTS],
}

pub(crate) fn spawn_exhaust(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) {
    let shared = meshes.add(Rectangle::new(1., 1.));
    let texture = images.add(crate::map_fx::soft_sprite());
    let fades = std::array::from_fn(|level| {
        materials.add(StandardMaterial {
            base_color: Color::srgba(0.54, 0.56, 0.6, level as f32 * 0.019),
            base_color_texture: Some(texture.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    });
    for _ in 0..PUFF_SLOTS {
        commands.spawn((
            Puff {
                age: PUFF_LIFE,
                velocity: Vec3::ZERO,
            },
            Mesh3d(shared.clone()),
            MeshMaterial3d(fades[0].clone()),
            Transform::IDENTITY,
            Visibility::Hidden,
            NotShadowCaster,
        ));
    }
    commands.insert_resource(Exhaust {
        next: 0.,
        materials: fades,
    });
}

#[allow(clippy::too_many_arguments)]
fn animate_exhaust(
    time: Res<Time>,
    race: Res<Race>,
    screen: Res<State<AppScreen>>,
    controls: Res<Controls>,
    options: Res<Options>,
    online: Option<Res<crate::online::NetState>>,
    riders: Query<(&Rider, &Recovery, &Transform), Without<Puff>>,
    camera: Query<&Transform, (With<ChaseCamera>, Without<Puff>)>,
    mut exhaust: ResMut<Exhaust>,
    mut puffs: Query<
        (
            &mut Puff,
            &mut Transform,
            &mut Visibility,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<Rider>,
    >,
) {
    let Ok(camera) = camera.single() else { return };
    if *screen.get() != AppScreen::Race || matches!(race.phase, Phase::Garage | Phase::Finished) {
        exhaust.next = 0.;
        for (mut puff, _, mut visibility, _) in &mut puffs {
            puff.age = PUFF_LIFE;
            visibility.set_if_neq(Visibility::Hidden);
        }
        return;
    }
    if race.phase == Phase::Paused {
        return;
    }
    let dt = time.delta_secs().min(0.05);
    exhaust.next = (exhaust.next - dt).max(0.);
    let emitter = riders.iter().find(|(r, stage, _)| {
        r.id == local_id(online.as_deref()) && stage.stage == RecoveryStage::Riding
    });
    let emit = emitter.filter(|(r, _, _)| {
        (race.phase == Phase::Countdown
            && options.rev
            && controls.buffer.held.throttle_strength() > 0.35)
            || (race.phase == Phase::Racing
                && r.drive_intent.throttle_strength() > 0.68
                && r.speed < 38.)
    });
    for (mut puff, mut pose, mut visibility, mut material) in &mut puffs {
        if puff.age >= PUFF_LIFE {
            if let Some((rider, _, bike)) = emit
                && exhaust.next == 0.
            {
                exhaust.next = 0.095;
                puff.age = 0.;
                puff.velocity =
                    rider.motion_velocity * 0.44 - bike.rotation * Vec3::Z * 1.6 + Vec3::Y * 0.24;
                pose.translation = tail(bike);
            } else {
                visibility.set_if_neq(Visibility::Hidden);
                continue;
            }
        }
        puff.age += dt;
        let fade = (1. - puff.age / PUFF_LIFE).clamp(0., 1.);
        pose.translation += puff.velocity * dt;
        pose.rotation = camera.rotation;
        pose.scale = Vec3::splat(0.15 + (1. - fade) * 0.58);
        let level = (fade * (PUFF_SLOTS - 1) as f32).round() as usize;
        material.set_if_neq(MeshMaterial3d(exhaust.materials[level].clone()));
        visibility.set_if_neq(if level == 0 {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        });
    }
}

#[derive(Default)]
struct CameraPulse {
    last: Option<(usize, bool)>,
}

fn shake(
    mut pulse: Local<CameraPulse>,
    mut race: ResMut<Race>,
    options: Res<Options>,
    controls: Res<Controls>,
    online: Option<Res<crate::online::NetState>>,
    riders: Query<(&Rider, &Recovery, &BikeState)>,
) {
    if race.phase != Phase::Racing {
        pulse.last = None;
        if race.phase == Phase::Countdown && options.rev {
            race.shake = race
                .shake
                .max(0.08 * controls.buffer.held.throttle_strength());
        }
        return;
    }
    let Some((rider, _, bike)) = riders.iter().find(|(r, stage, _)| {
        r.id == local_id(online.as_deref()) && stage.stage == RecoveryStage::Riding
    }) else {
        pulse.last = None;
        return;
    };
    let grounded = bike.grounded.iter().any(|&wheel| wheel);
    if let Some((gear, was_grounded)) = pulse.last {
        if gear != bike.gear && rider.speed > 12. {
            race.shake = race.shake.max(0.13);
        }
        if grounded && !was_grounded && bike.impact_velocity.y < -2. {
            race.shake = race.shake.max((-bike.impact_velocity.y * 0.075).min(0.45));
        }
    }
    pulse.last = Some((bike.gear, grounded));
}

pub(crate) fn install(app: &mut App) {
    let options = Options::from_flags(&std::env::var("ROADDASH_POLISH").unwrap_or_default());
    if options.rev {
        app.init_resource::<GridEngine>()
            .add_systems(FixedUpdate, grid_engine.after(crate::game::drive))
            .add_systems(Update, tachometer);
    }
    if options.exhaust {
        app.add_systems(
            PostUpdate,
            animate_exhaust
                .run_if(resource_exists::<Exhaust>)
                .after(crate::game::camera)
                .before(bevy::transform::TransformSystems::Propagate),
        );
    }
    if options.grade || options.wind || options.heat {
        // Preserve the processed scene when the separate MSAA HUD camera renders.
        app.add_plugins((
            bevy_post_process::msaa_writeback::MsaaWritebackPlugin,
            FullscreenMaterialPlugin::<RoadEffects>::default(),
        ))
        .add_systems(PostUpdate, camera_effects.after(crate::game::camera));
    }
    if options.shake {
        app.add_systems(
            FixedUpdate,
            shake.after(crate::runtime::SimSet::AfterPhysics),
        );
    }
    if options.hanabi {
        #[cfg(feature = "hanabi-experiment")]
        hanabi::install(app);
        #[cfg(not(feature = "hanabi-experiment"))]
        eprintln!("Hanabi comparison requires --features hanabi-experiment");
    }
    app.insert_resource(options);
}

#[cfg(feature = "hanabi-experiment")]
mod hanabi;
pub(crate) mod road;
pub(crate) mod survey;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_rev_rises_and_releases_independently_of_wheel_speed() {
        let peak = (0..45).fold(IDLE_RPM, |rpm, _| free_rev(rpm, 1., 1. / 60.));
        assert!((7200. ..7450.).contains(&peak));
        let coast = (0..65).fold(peak, |rpm, _| free_rev(rpm, 0., 1. / 60.));
        assert!((IDLE_RPM..1700.).contains(&coast));
    }
}
