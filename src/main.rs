mod game;
mod menu;
mod splash;
mod world;

use game::state::GlobalState;

pub use game::DefaultGamePlugins;
pub use menu::MenuPlugin;
pub use splash::SplashPlugin;
pub use world::DefaultWorldPlugins;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

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
        // Inspector Plugin
        .add_plugin(WorldInspectorPlugin::new())
        // Our Plugins
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugins(DefaultWorldPlugins)
        .add_plugins(DefaultGamePlugins)
        .run();
}
