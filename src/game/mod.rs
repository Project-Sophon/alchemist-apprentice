use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod despawn;
pub mod level;
pub mod state;
pub mod customer;

use self::{despawn::DespawnPlugin, level::LevelPlugin, state::StatePlugin, customer::CustomerPlugin};

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LevelPlugin)
            .add(DespawnPlugin)
            .add(StatePlugin)
            .add(CustomerPlugin)
    }
}
