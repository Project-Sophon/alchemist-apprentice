use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{buttons::ButtonPlugin, common::CommonUiPlugin, game_ui::GameUiPlugin};

pub mod buttons;
pub mod common;
pub mod game_ui;

pub struct DefaultUIPlugins;
impl PluginGroup for DefaultUIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GameUiPlugin)
            .add(ButtonPlugin)
            .add(CommonUiPlugin)
    }
}
