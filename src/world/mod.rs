use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod camera;

use self::camera::CameraPlugin;

pub struct DefaultWorldPlugins;
impl PluginGroup for DefaultWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(CameraPlugin)
    }
}
