use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection};

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
    #[asset(path = "textures/ui/game_level_bkg.png")]
    pub game_level_bkg: Handle<Image>,
    #[asset(path = "textures/ui/game_ui_bkg.png")]
    pub game_ui_bkg: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(path = "textures/characters/bjorn.png")]
    pub bjorn: Handle<Image>,
}
