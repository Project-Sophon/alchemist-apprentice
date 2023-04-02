use bevy::{app::PluginGroupBuilder, prelude::*};

use self::root::RootUiPlugin;

pub mod root;

pub struct DefaultUIPlugins;
impl PluginGroup for DefaultUIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RootUiPlugin)
    }
}
