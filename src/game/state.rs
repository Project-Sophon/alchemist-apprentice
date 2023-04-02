use bevy::prelude::*;

// ------ ENUMS, CONSTANTS ------

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GlobalState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GamePhase {
    #[default]
    CustomerEnter,
    AilmentStatement,
    IngredientAssembly,
    Treatment,
    TreatmentEffect,
    CustomerExit,
    Rest
}

// ------ PLUGINS ------

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GamePhase>();
    }
}
