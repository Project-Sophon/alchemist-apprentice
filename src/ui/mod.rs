use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{buttons::ButtonPlugin, common::CommonUiPlugin, root_ui::RootUiPlugin};

pub mod buttons;
pub mod common;
pub mod root_ui;

pub struct DefaultUIPlugins;
impl PluginGroup for DefaultUIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RootUiPlugin)
            .add(ButtonPlugin)
            .add(CommonUiPlugin)
    }
}
