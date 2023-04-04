use bevy::prelude::*;

use crate::assets::standard_assets::{CharacterAssets, GlobalAssets};

use super::{despawn::despawn_entity, dialogue::create_dialogue_box, state::GamePhase};

pub struct CustomerPlugin;
impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Customer>()
            .add_system(customer_enter.in_schedule(OnEnter(GamePhase::CustomerEnter)))
            .add_system(customer_intro_countdown.in_set(OnUpdate(GamePhase::CustomerEnter)))
            .add_system(despawn_entity::<Customer>.in_schedule(OnExit(GamePhase::CustomerExit)))
            .add_system(customer_display_ailment.in_schedule(OnEnter(GamePhase::AilmentStatement)))
            .add_system(customer_ailment_countdown.in_set(OnUpdate(GamePhase::AilmentStatement)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Customer;

#[derive(Resource, Deref, DerefMut)]
struct CustomerIntroTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct CustomerAilmentTimer(Timer);

fn customer_enter(mut commands: Commands, character_assets: Res<CharacterAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: character_assets.bjorn.clone(),
            transform: Transform {
                translation: Vec3::new(150., 150., 1.),
                scale: Vec3::new(5., 5., 0.),
                ..default()
            },
            ..default()
        },
        Customer,
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

fn customer_display_ailment(mut commands: Commands, global_assets: Res<GlobalAssets>) {
    let font = global_assets.font.clone();
    let dialogue_text =
        "Hello, my name is Bjorn Bjornson ...\n(OnEnter(GamePhase::AilmentStatement))";

    create_dialogue_box(&mut commands, font, dialogue_text);

    commands.insert_resource(CustomerAilmentTimer(Timer::from_seconds(
        2.0,
        TimerMode::Once,
    )));
}

fn customer_ailment_countdown(
    mut game_state: ResMut<NextState<GamePhase>>,
    time: Res<Time>,
    mut timer: ResMut<CustomerAilmentTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GamePhase::IngredientAssembly);
    }
}
