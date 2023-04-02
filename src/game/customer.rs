use bevy::prelude::*;

use super::{despawn::despawn_entity, state::GamePhase};

pub struct CustomerPlugin;
impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(customer_enter.in_schedule(OnEnter(GamePhase::CustomerEnter)))
            .add_system(customer_intro_countdown.in_set(OnUpdate(GamePhase::CustomerEnter)))
            .add_system(despawn_entity::<Customer>.in_schedule(OnExit(GamePhase::CustomerExit)))
            .add_system(customer_display_ailment.in_schedule(OnEnter(GamePhase::AilmentStatement)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Customer {}

#[derive(Resource, Deref, DerefMut)]
struct CustomerIntroTimer(Timer);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AilmentDialogueBox;

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

fn customer_display_ailment(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Hello, my name is Bjorn Bjornson ...\n(OnEnter(GamePhase::AilmentStatement))",
            TextStyle {
                font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(120.0),
                left: Val::Px(80.0),
                ..default()
            },
            ..default()
        }),
        AilmentDialogueBox,
        Name::new("Ailment Dialogue Box"),
    ));
}
