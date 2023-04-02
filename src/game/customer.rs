use bevy::prelude::*;

use super::{despawn::despawn_entity, state::GamePhase};

pub struct CustomerPlugin;
impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(customer_enter.in_schedule(OnEnter(GamePhase::CustomerEnter)))
            .add_system(customer_intro_countdown.in_set(OnUpdate(GamePhase::CustomerEnter)))
            .add_system(despawn_entity::<Customer>.in_schedule(OnExit(GamePhase::CustomerExit)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Customer {}

#[derive(Resource, Deref, DerefMut)]
struct CustomerIntroTimer(Timer);

fn customer_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/characters/bjorn.png").into(),
            transform: Transform {
                translation: Vec3::new(150., 150., 1.),
                scale: Vec3::new(5., 5., 0.),
                ..default()
            },
            ..default()
        },
        Customer {},
        Name::new("Customer"),
    ));

    commands.insert_resource(CustomerIntroTimer(Timer::from_seconds(
        2.0,
        TimerMode::Once,
    )));
}

fn customer_intro_countdown(
    mut game_state: ResMut<NextState<GamePhase>>,
    time: Res<Time>,
    mut timer: ResMut<CustomerIntroTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GamePhase::AilmentStatement);
    }
}
