use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{buttons::ButtonPlugin, common::CommonUiPlugin, game_ui::RootUiPlugin};

pub mod buttons;
pub mod common;
pub mod game_ui;

pub struct DefaultUIPlugins;
impl PluginGroup for DefaultUIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RootUiPlugin)
            .add(ButtonPlugin)
            .add(CommonUiPlugin)
    }
}
