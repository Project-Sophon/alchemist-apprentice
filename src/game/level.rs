use bevy::prelude::*;

use crate::assets::UiAssets;

use super::state::{GamePhase, GlobalState};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_bkg_setup.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(customer_intro.in_schedule(OnEnter(GlobalState::Game)));
    }
}

fn level_bkg_setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn(SpriteBundle {
        texture: ui_assets.level_background.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}

fn customer_intro(mut game_phase: ResMut<NextState<GamePhase>>) {
    game_phase.set(GamePhase::CustomerEnter);
}
