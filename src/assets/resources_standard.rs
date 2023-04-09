use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct GlobalAssets {
    #[asset(path = "branding/splash.png")]
    pub splash: Handle<Image>,
    #[asset(path = "fonts/alagard.ttf")]
    pub font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "textures/ui/game_ui_bkg.png")]
    pub game_ui_bkg: Handle<Image>,
    #[asset(path = "textures/ui/status_panel_bkg.png")]
    pub status_panel_bkg: Handle<Image>,
    #[asset(path = "textures/ui/info_panel_bkg.png")]
    pub info_panel_bkg: Handle<Image>,
    #[asset(path = "textures/ui/ingredient_button_normal.png")]
    pub ingredient_button_normal: Handle<Image>,
    #[asset(path = "textures/ui/ingredient_button_hover.png")]
    pub ingredient_button_hover: Handle<Image>,
    #[asset(path = "textures/ui/ingredient_button_selected.png")]
    pub ingredient_button_selected: Handle<Image>,
    #[asset(path = "textures/ui/potion_circle_bkg.png")]
    pub potion_circle_bkg: Handle<Image>,
    #[asset(path = "textures/ui/potion_circle_slot_empty.png")]
    pub potion_circle_slot_empty: Handle<Image>,
    #[asset(path = "textures/ui/potion_circle_slot_hover.png")]
    pub potion_circle_slot_hover: Handle<Image>,
    #[asset(path = "textures/ui/potion_circle_slot_occupied.png")]
    pub potion_circle_slot_occupied: Handle<Image>,
    #[asset(path = "textures/ui/potion_circle_slot_occupied_hover.png")]
    pub potion_circle_slot_occupied_hover: Handle<Image>,
    #[asset(path = "textures/ui/concoct_button_click.png")]
    pub concoct_button_click: Handle<Image>,
    #[asset(path = "textures/ui/concoct_button_hover.png")]
    pub concoct_button_hover: Handle<Image>,
    #[asset(path = "textures/ui/concoct_button_normal.png")]
    pub concoct_button_normal: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(path = "textures/characters/bjorn.png")]
    pub bjorn: Handle<Image>,
    #[asset(path = "textures/characters/bjorn_dead.png")]
    pub bjorn_dead: Handle<Image>,
    #[asset(path = "textures/characters/alchemist.png")]
    pub alchemist: Handle<Image>,
    #[asset(path = "textures/characters/alchemist_win.png")]
    pub alchemist_win: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct WorkshopAssets {
    #[asset(path = "textures/workshop/workshop_bkg.png")]
    pub workshop_bkg: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ToxAssets {
    #[asset(path = "textures/tox/tox_0.png")]
    pub tox_0: Handle<Image>,
    #[asset(path = "textures/tox/tox_1.png")]
    pub tox_1: Handle<Image>,
    #[asset(path = "textures/tox/tox_2.png")]
    pub tox_2: Handle<Image>,
    #[asset(path = "textures/tox/tox_3.png")]
    pub tox_3: Handle<Image>,
    #[asset(path = "textures/tox/tox_4.png")]
    pub tox_4: Handle<Image>,
    #[asset(path = "textures/tox/tox_5.png")]
    pub tox_5: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sounds/menu.wav")]
    pub menu: Handle<AudioSource>,
    #[asset(path = "sounds/click.wav")]
    pub click: Handle<AudioSource>,
    #[asset(path = "sounds/concoct.wav")]
    pub concoct: Handle<AudioSource>,
}
