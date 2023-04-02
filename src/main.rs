mod game;
mod menu;
mod splash;

pub use game::DefaultGamePlugins;
pub use menu::MenuPlugin;
pub use splash::SplashPlugin;

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GlobalState {
    #[default]
    Splash,
    Menu,
    Game,
}

const GAME_BACKGROUND_COLOR: &str = "#F5EDE9";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Alchemist's Apprentice".into(),
                resolution: (1024., 768.).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GlobalState>()
        .add_startup_system(setup_camera)
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugins(DefaultGamePlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
