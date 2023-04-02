use bevy::{app::PluginGroupBuilder, prelude::*};

use self::root_ui::RootUiPlugin;

pub mod root_ui;

pub struct DefaultUIPlugins;
impl PluginGroup for DefaultUIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RootUiPlugin)
    }
}
