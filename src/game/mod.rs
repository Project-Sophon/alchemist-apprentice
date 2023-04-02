use bevy::{app::PluginGroupBuilder, prelude::*};

use self::level::LevelPlugin;

pub mod level;

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(LevelPlugin)
    }
}
