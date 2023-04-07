use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GamePhase {
    #[default]
    AilmentStatement,
    PotionAssembly,
    Concoct,
}

pub struct GamePhasePlugin;

impl Plugin for GamePhasePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GamePhase>();
    }
}
