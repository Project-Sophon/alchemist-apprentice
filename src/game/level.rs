use crate::{GlobalState, GAME_BACKGROUND_COLOR};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_ui_setup.in_schedule(OnEnter(GlobalState::Game)));
    }
}

fn level_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::hex(GAME_BACKGROUND_COLOR).unwrap().into(),
            ..default()
        })
        .with_children(|parent| {
            let level_background = asset_server.load("textures/ui/alchemy_background.png");
            parent.spawn(ImageBundle {
                style: Style { ..default() },
                image: level_background.into(),
                ..default()
            });
        });
}
