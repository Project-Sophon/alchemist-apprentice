#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod assets;
mod end;
mod game;
mod menu;
mod splash;
mod style;
mod ui;
mod world;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use end::EndPlugin;
use style::color::PALETTE_CREAM;
use world::{
    common::{WINDOW_HEIGHT, WINDOW_WIDTH},
    global_state::GlobalState,
};

use assets::AssetPlugin;
use game::DefaultGamePlugins;
use menu::MenuPlugin;
use splash::SplashPlugin;
use ui::DefaultUIPlugins;
use world::DefaultWorldPlugins;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn main() {
    App::new()
        // Set game background color
        .insert_resource(ClearColor(Color::hex(PALETTE_CREAM).unwrap()))
        // Add Plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Alchemist's Apprentice".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                // required for pixel perfect rendering
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(AudioPlugin)
        // Inspector Plugin can be enabled for debugging
        .add_plugin(WorldInspectorPlugin::new())
        // Our Plugins
        .add_plugins(DefaultWorldPlugins)
        .add_plugins(DefaultGamePlugins)
        .add_plugins(DefaultUIPlugins)
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(AssetPlugin)
        .add_plugin(EndPlugin)
        .run();
}
