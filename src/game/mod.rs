use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod despawn;
pub mod level;

use self::{despawn::DespawnPlugin, level::LevelPlugin};

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LevelPlugin)
            .add(DespawnPlugin)
    }
}
