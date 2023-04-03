use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod assembly;
pub mod customer;
pub mod despawn;
pub mod dialogue;
pub mod level;
pub mod state;

use self::{
    assembly::AssemblyPlugin, customer::CustomerPlugin, despawn::DespawnPlugin, level::LevelPlugin,
    state::StatePlugin,
};

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LevelPlugin)
            .add(DespawnPlugin)
            .add(StatePlugin)
            .add(CustomerPlugin)
            .add(AssemblyPlugin)
    }
}

// ------ ENUMS, CONSTANTS ------

pub const GAME_BACKGROUND_COLOR: &str = "#F5EDE9";
