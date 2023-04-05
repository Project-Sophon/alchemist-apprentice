use bevy::prelude::*;

use crate::assets::standard_assets::UiAssets;

use super::global_state::GlobalState;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_bkg_setup.in_schedule(OnEnter(GlobalState::Game)));
    }
}

fn level_bkg_setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn(SpriteBundle {
        texture: ui_assets.level_background.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}
