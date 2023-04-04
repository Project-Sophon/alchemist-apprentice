use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};

use crate::game::state::GlobalState;

#[derive(AssetCollection, Resource)]
pub struct GlobalAssets {
    #[asset(path = "branding/splash.png")]
    pub splash: Handle<Image>,
    #[asset(path = "branding/main_menu.png")]
    pub main_menu_banner: Handle<Image>,
    #[asset(path = "fonts/FiraCode-Bold.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "textures/ui/right.png")]
    pub right: Handle<Image>,
    #[asset(path = "textures/ui/wrench.png")]
    pub wrench: Handle<Image>,
    #[asset(path = "textures/ui/exitRight.png")]
    pub exit: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "textures/ui/alchemy_background.png")]
    pub level_background: Handle<Image>,
    #[asset(path = "textures/ui/workbench_placeholder.png")]
    pub workbench: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(path = "textures/characters/bjorn.png")]
    pub bjorn: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct IngredientAssets {
    #[asset(path = "textures/ingredients/auria_leaf.png")]
    pub auria_leaf: Handle<Image>,
    #[asset(path = "textures/ingredients/crow_foot.png")]
    pub crow_foot: Handle<Image>,
    #[asset(path = "textures/ingredients/dluger_heart.png")]
    pub dluger_heart: Handle<Image>,
    #[asset(path = "textures/ingredients/shadow_beetle.png")]
    pub shadow_beetle: Handle<Image>,
    #[asset(path = "textures/ingredients/zizima_root.png")]
    pub zizima_root: Handle<Image>,
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GlobalState::AssetLoading).continue_to_state(GlobalState::Splash),
        )
        .add_collection_to_loading_state::<_, GlobalAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, CharacterAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, UiAssets>(GlobalState::AssetLoading)
        .add_collection_to_loading_state::<_, IngredientAssets>(GlobalState::AssetLoading);
    }
}
