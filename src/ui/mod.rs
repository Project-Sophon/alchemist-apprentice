use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{buttons::ButtonsPlugin, disable_ui::DisableUiPlugin};

pub mod buttons;
pub mod disable_ui;
pub struct DefaultUIPlugins;
impl PluginGroup for DefaultUIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ButtonsPlugin)
            .add(DisableUiPlugin)
    }
}
