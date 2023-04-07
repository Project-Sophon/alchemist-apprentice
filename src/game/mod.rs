use bevy::{app::PluginGroupBuilder, prelude::*};
pub mod dialogue;
pub mod game_phase;
pub mod information;
pub mod ingredients;
pub mod level;
pub mod potion;
pub mod states;

use self::{
    game_phase::GamePhasePlugin,
    information::InformationPlugin,
    ingredients::IngredientsPlugin,
    level::LevelPlugin,
    potion::PotionPlugin,
    states::{concoct::ConcoctPlugin, potion_assembly::PotionAssemblyPlugin},
};

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GamePhasePlugin)
            .add(LevelPlugin)
            .add(IngredientsPlugin)
            .add(InformationPlugin)
            .add(PotionPlugin)
            .add(PotionAssemblyPlugin)
            .add(ConcoctPlugin)
    }
}
