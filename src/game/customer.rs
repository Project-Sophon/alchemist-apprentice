use bevy::prelude::*;

use super::state::GamePhase;

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(customer_enter.in_schedule(OnEnter(GamePhase::CustomerEnter)));
    }
}

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
        Name::new("Customer"),
    ));
}
