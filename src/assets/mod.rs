use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::world::global_state::GlobalState;

use self::{
    assets_game_data::{Ingredient, Symptom},
    resources_game_data::{GameDataAssetDynamicCollection, IngredientAssets, SymptomAssets},
    resources_standard::{CharacterAssets, GlobalAssets, UiAssets},
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
