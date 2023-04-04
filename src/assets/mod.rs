use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use crate::game::state::GlobalState;

use self::standard_assets::{GlobalAssets, CharacterAssets, UiAssets};

pub mod standard_assets;
pub mod game_data;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GlobalState::AssetLoading).continue_to_state(GlobalState::Splash),
        )
        .add_collection_to_loading_state::<_, GlobalAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, CharacterAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, UiAssets>(GlobalState::AssetLoading);
    }
}
