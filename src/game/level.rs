use bevy::prelude::*;

use crate::{assets::{UiAssets}, game_data::{GameDataHandle, GameData}};

use super::state::{GamePhase, GlobalState};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_bkg_setup.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(customer_intro.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(debug_game_assets.in_schedule(OnEnter(GlobalState::Game)));
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

fn debug_game_assets(game_data_handle: Res<GameDataHandle>, game_data: Res<Assets<GameData>>) {
    if let Some(data) = game_data.get(&game_data_handle.0) {
        for class in data.symptom_classes.iter() {
            info!(class);
        }

        for symptom in data.symptoms.iter() {
            info!(symptom.name);
            info!(symptom.class);
            info!(symptom.description);
        }

        for ingredient in data.ingredients.iter() {
            info!(ingredient.name);
            info!(ingredient.cures);
            info!(ingredient.causes);
        }
    }
}
