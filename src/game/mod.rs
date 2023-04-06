use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod dialogue;
pub mod game_phase;
pub mod states;
pub mod ingredients;

use self::{game_phase::GamePhasePlugin, states::potion_assembly::PotionAssemblyPlugin};

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GamePhasePlugin)
            .add(PotionAssemblyPlugin)
    }
}
