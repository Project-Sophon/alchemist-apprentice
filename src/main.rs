mod assets;
mod game;
mod game_data;
mod menu;
mod splash;
mod ui;
mod world;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::state::GlobalState;

use assets::AssetPlugin;
use game::DefaultGamePlugins;
use game_data::GameDataPlugin;
use menu::MenuPlugin;
use splash::SplashPlugin;
use ui::DefaultUIPlugins;
use world::DefaultWorldPlugins;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Alchemist's Apprentice".into(),
                resolution: (1024., 768.).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GlobalState>()
        // Game Data
        .add_plugin(GameDataPlugin)
        // Asset Loader
        .add_plugin(AssetPlugin)
        // Inspector Plugin
        .add_plugin(WorldInspectorPlugin::new())
        // Our Plugins
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugins(DefaultWorldPlugins)
        .add_plugins(DefaultGamePlugins)
        .add_plugins(DefaultUIPlugins)
        .run();
}
