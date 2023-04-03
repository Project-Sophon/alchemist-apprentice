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
pub struct UiAssets {}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GlobalState::AssetLoading).continue_to_state(GlobalState::Splash),
        )
        .add_collection_to_loading_state::<_, GlobalAssets>(GlobalState::AssetLoading);
    }
}
