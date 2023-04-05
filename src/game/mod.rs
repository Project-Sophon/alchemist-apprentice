use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod ailment_statement;
pub mod concoct;
pub mod potion_assembly;
pub mod rest;
pub mod game_phase;
pub mod treatment;
pub mod treatment_effect;

pub mod dialogue;

use self::{potion_assembly::PotionAssemblyPlugin, game_phase::GamePhasePlugin};

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GamePhasePlugin)
            .add(PotionAssemblyPlugin)
    }
}
