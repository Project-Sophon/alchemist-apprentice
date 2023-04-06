use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod camera;
pub mod common;
pub mod despawn;
pub mod global_state;

use self::{camera::CameraPlugin, despawn::DespawnPlugin, global_state::GlobalStatePlugin};

pub struct DefaultWorldPlugins;
impl PluginGroup for DefaultWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GlobalStatePlugin)
            .add(CameraPlugin)
            .add(DespawnPlugin)
    }
}
