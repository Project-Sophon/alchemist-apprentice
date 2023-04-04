use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::game::state::GlobalState;

use self::{
    game_data::{GameDataAssetDynamicCollection, GameData},
    standard_assets::{CharacterAssets, GlobalAssets, UiAssets},
};

pub mod game_data;
pub mod standard_assets;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<GameDataAssetDynamicCollection>::new(&[
            "game-data.ron",
        ]))
        .add_loading_state(
            LoadingState::new(GlobalState::AssetLoading).continue_to_state(GlobalState::Splash),
        )
        .add_collection_to_loading_state::<_, GlobalAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, CharacterAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, UiAssets>(GlobalState::AssetLoading)
        .add_dynamic_collection_to_loading_state::<_, GameDataAssetDynamicCollection>(
            GlobalState::AssetLoading,
            "data/test.game-data.ron",
        )
        .add_collection_to_loading_state::<_, GameData>(GlobalState::AssetLoading);
    }
}
