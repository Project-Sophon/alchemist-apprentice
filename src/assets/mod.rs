use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::world::global_state::GlobalState;

use self::{
    assets_game_data::{Ingredient, SideEffect},
    resources_game_data::{GameDataAssetDynamicCollection, IngredientAssets, SideEffectAssets},
    resources_standard::{
        AudioAssets, CharacterAssets, GlobalAssets, ToxAssets, UiAssets, WorkshopAssets,
    },
};

pub mod assets_game_data;
pub mod resources_game_data;
pub mod resources_standard;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<GameDataAssetDynamicCollection>::new(&[
            "game-data.ron",
        ]))
        .add_asset::<SideEffect>()
        .add_asset::<Ingredient>()
        .add_loading_state(
            LoadingState::new(GlobalState::AssetLoading).continue_to_state(GlobalState::Splash),
        )
        .add_collection_to_loading_state::<_, GlobalAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, WorkshopAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, AudioAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, CharacterAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, UiAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, ToxAssets>(GlobalState::AssetLoading)
        .add_dynamic_collection_to_loading_state::<_, GameDataAssetDynamicCollection>(
            GlobalState::AssetLoading,
            "data/ingredients.game-data.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, GameDataAssetDynamicCollection>(
            GlobalState::AssetLoading,
            "data/side_effects.game-data.ron",
        )
        .add_collection_to_loading_state::<_, IngredientAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, SideEffectAssets>(GlobalState::AssetLoading);
    }
}
