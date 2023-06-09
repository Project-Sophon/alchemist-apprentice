use bevy::{app::PluginGroupBuilder, prelude::*};
pub mod bjorn;
pub mod dialogue;
pub mod game_phase;
pub mod information;
pub mod ingredients;
pub mod level;
pub mod phases;
pub mod potion;
pub mod status;
pub mod workshop;

use self::{
    bjorn::BjornPlugin,
    game_phase::GamePhasePlugin,
    information::InformationPlugin,
    ingredients::IngredientsPlugin,
    level::LevelPlugin,
    phases::{
        ailment_statement::AilmentStatementPlugin, concoct::ConcoctPlugin,
        potion_assembly::PotionAssemblyPlugin,
    },
    potion::PotionPlugin,
    status::StatusPlugin,
    workshop::WorkshopPlugin,
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
            .add(AilmentStatementPlugin)
            .add(PotionAssemblyPlugin)
            .add(ConcoctPlugin)
            .add(BjornPlugin)
            .add(StatusPlugin)
            .add(WorkshopPlugin)
    }
}
