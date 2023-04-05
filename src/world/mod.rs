use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod camera;
pub mod despawn;
pub mod level;
pub mod global_state;

use self::{camera::CameraPlugin, despawn::DespawnPlugin, level::LevelPlugin, global_state::GlobalStatePlugin};

pub struct DefaultWorldPlugins;
impl PluginGroup for DefaultWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GlobalStatePlugin)
            .add(LevelPlugin)
            .add(CameraPlugin)
            .add(DespawnPlugin)
    }
}
