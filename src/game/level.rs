use bevy::prelude::*;

use super::state::{GamePhase, GlobalState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_ui_setup.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(
                enter_phase
                    .after(level_ui_setup)
                    .in_schedule(OnEnter(GlobalState::Game)),
            );
    }
}

fn level_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server
            .load("textures/ui/alchemy_background.png")
            .into(),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}

fn enter_phase(mut game_phase: ResMut<NextState<GamePhase>>) {
    game_phase.set(GamePhase::CustomerEnter);
}
