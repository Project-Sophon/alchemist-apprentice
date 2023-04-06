use bevy::{app::PluginGroupBuilder, prelude::*};

use self::disable_ui::DisableUiPlugin;

pub mod disable_ui;

pub struct DefaultUIPlugins;
impl PluginGroup for DefaultUIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(DisableUiPlugin)
    }
}
