//! A comparable exhaust emitter for the optional GPU-particle experiment.
use super::*;
use bevy_hanabi::prelude::*;

#[derive(Component)]
struct GpuExhaust;

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let writer = ExprWriter::new();
    let pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.065).expr(),
        dimension: ShapeDimension::Volume,
    };
    let velocity = SetAttributeModifier::new(
        Attribute::VELOCITY,
        writer.lit(Vec3::new(0., 0.32, -1.8)).expr(),
    );
    let age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).expr());
    let life = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(PUFF_LIFE).expr());
    let mut module = writer.finish();
    let round = RoundModifier::constant(&mut module, 1.);
    let mut fade = bevy_hanabi::Gradient::new();
    fade.add_key(0., Vec4::new(0.58, 0.62, 0.67, 0.14));
    fade.add_key(1., Vec4::new(0.58, 0.62, 0.67, 0.));
    let mut size = bevy_hanabi::Gradient::new();
    size.add_key(0., Vec3::splat(0.13));
    size.add_key(1., Vec3::splat(0.47));
    let asset = effects.add(
        EffectAsset::new(
            32,
            SpawnerSettings::rate(15.0.into()).with_starts_active(false),
            module,
        )
        .with_name("RoadDash GPU exhaust comparison")
        .init(pos)
        .init(velocity)
        .init(age)
        .init(life)
        .render(ColorOverLifetimeModifier::new(fade))
        .render(SizeOverLifetimeModifier {
            gradient: size,
            screen_space_size: false,
        })
        .render(round),
    );
    commands.spawn((GpuExhaust, ParticleEffect::new(asset), Transform::IDENTITY));
}

fn follow(
    race: Res<Race>,
    controls: Res<Controls>,
    screen: Res<State<AppScreen>>,
    online: Option<Res<crate::online::NetState>>,
    riders: Query<(&Rider, &Recovery, &Transform), Without<GpuExhaust>>,
    mut emitters: Query<(&mut Transform, &mut EffectSpawner), With<GpuExhaust>>,
) {
    let rider = riders.iter().find(|(r, stage, _)| {
        r.id == local_id(online.as_deref()) && stage.stage == RecoveryStage::Riding
    });
    for (mut transform, mut spawner) in &mut emitters {
        spawner.active = *screen.get() == AppScreen::Race
            && rider.is_some_and(|(r, _, _)| {
                (race.phase == Phase::Countdown && controls.buffer.held.throttle_strength() > 0.35)
                    || (race.phase == Phase::Racing
                        && r.drive_intent.throttle_strength() > 0.68
                        && r.speed < 38.)
            });
        if let Some((_, _, bike)) = rider {
            transform.translation = tail(bike);
            transform.rotation = bike.rotation;
        }
    }
}

pub(super) fn install(app: &mut App) {
    app.add_plugins(HanabiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            PostUpdate,
            follow.before(bevy::transform::TransformSystems::Propagate),
        );
}
