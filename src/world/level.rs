use bevy::prelude::*;

use crate::assets::resources_standard::UiAssets;

use super::{despawn::despawn_entity, global_state::GlobalState};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_bkg_setup.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(despawn_entity::<LevelBackground>.in_schedule(OnExit(GlobalState::Game)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct LevelBackground;

fn level_bkg_setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: ui_assets.level_background.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        LevelBackground,
        Name::new("Level Background Sprite"),
    ));
}
