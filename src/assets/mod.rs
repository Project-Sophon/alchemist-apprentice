use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::game::state::GlobalState;

use self::{
    game_data::{GameDataAssetDynamicCollection, Ingredient, IngredientAssets, Symptom, SymptomAssets},
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
        .add_asset::<Symptom>()
        .add_asset::<Ingredient>()
        .add_loading_state(
            LoadingState::new(GlobalState::AssetLoading).continue_to_state(GlobalState::Splash),
        )
        .add_collection_to_loading_state::<_, GlobalAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, CharacterAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, UiAssets>(GlobalState::AssetLoading)
        .add_dynamic_collection_to_loading_state::<_, GameDataAssetDynamicCollection>(
            GlobalState::AssetLoading,
            "data/ingredients.game-data.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, GameDataAssetDynamicCollection>(
            GlobalState::AssetLoading,
            "data/symptoms.game-data.ron",
        )
        .add_collection_to_loading_state::<_, IngredientAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, SymptomAssets>(GlobalState::AssetLoading);
    }
}
