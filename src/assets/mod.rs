use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::yaml::YamlAssetPlugin;

use crate::game::state::GlobalState;

use self::{
    game_data::GameDataAssetCollection,
    standard_assets::{CharacterAssets, GlobalAssets, UiAssets},
};

pub mod game_data;
pub mod standard_assets;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<GameDataAssetCollection>::new(&[
            "game-data.yaml",
        ]))
        .add_loading_state(
            LoadingState::new(GlobalState::AssetLoading).continue_to_state(GlobalState::Splash),
        )
        .add_collection_to_loading_state::<_, GlobalAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, CharacterAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, UiAssets>(GlobalState::AssetLoading)
        .add_dynamic_collection_to_loading_state::<_, GameDataAssetCollection>(
            GlobalState::AssetLoading,
            "data/test.game-data.yaml",
        );
    }
}
