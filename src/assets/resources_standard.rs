use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct GlobalAssets {
    #[asset(path = "branding/splash.png")]
    pub splash: Handle<Image>,
    #[asset(path = "branding/main_menu.png")]
    pub main_menu_banner: Handle<Image>,
    #[asset(path = "fonts/DePixelKlein.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "fonts/DePixelHalbfett.ttf")]
    pub font_bold: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "textures/ui/game_level_bkg.png")]
    pub game_level_bkg: Handle<Image>,
    #[asset(path = "textures/ui/game_ui_bkg.png")]
    pub game_ui_bkg: Handle<Image>,
    #[asset(path = "textures/ui/plus_dark_gold_64.png")]
    pub plus_dark_gold_64: Handle<Image>,
    #[asset(path = "textures/ui/ingredient_button_normal.png")]
    pub ingredient_button_normal: Handle<Image>,
    #[asset(path = "textures/ui/ingredient_button_hover.png")]
    pub ingredient_button_hover: Handle<Image>,
    #[asset(path = "textures/ui/ingredient_button_selected.png")]
    pub ingredient_button_selected: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(path = "textures/characters/bjorn.png")]
    pub bjorn: Handle<Image>,
}
