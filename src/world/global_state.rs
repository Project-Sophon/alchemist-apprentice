use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GlobalState {
    #[default]
    AssetLoading,
    Splash,
    Menu,
    Game,
    Win,
    Lose,
}

pub struct GlobalStatePlugin;

impl Plugin for GlobalStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GlobalState>();
    }
}
