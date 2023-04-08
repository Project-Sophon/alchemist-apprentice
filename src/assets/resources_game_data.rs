use bevy::{
    prelude::{AssetServer, Assets, Handle, HandleUntyped, Resource, World},
    utils::HashMap,
};
use bevy_asset_loader::prelude::{
    AssetCollection, DynamicAsset, DynamicAssetCollection, DynamicAssetType, DynamicAssets,
};

use super::assets_game_data::{Ingredient, SideEffectClass, SideEffect};

#[derive(AssetCollection, Resource)]
pub struct IngredientAssets {
    #[asset(key = "blackened_skull")]
    pub blackened_skull: Handle<Ingredient>,
    #[asset(key = "bowl_of_dust")]
    pub bowl_of_dust: Handle<Ingredient>,
    #[asset(key = "claw")]
    pub claw: Handle<Ingredient>,
    #[asset(key = "obsidian")]
    pub obsidian: Handle<Ingredient>,
    #[asset(key = "feather")]
    pub feather: Handle<Ingredient>,
    #[asset(key = "garlic_clove")]
    pub garlic_clove: Handle<Ingredient>,
    #[asset(key = "jar_of_eyes")]
    pub jar_of_eyes: Handle<Ingredient>,
    #[asset(key = "lead_bar")]
    pub lead_bar: Handle<Ingredient>,
    #[asset(key = "mushroom")]
    pub mushroom: Handle<Ingredient>,
    #[asset(key = "nightshade")]
    pub nightshade: Handle<Ingredient>,
    #[asset(key = "pearl")]
    pub pearl: Handle<Ingredient>,
    #[asset(key = "tear")]
    pub tear: Handle<Ingredient>,
}

#[derive(AssetCollection, Resource)]
pub struct SideEffectAssets {
    #[asset(key = "headache")]
    pub headache: Handle<SideEffect>,
    #[asset(key = "parasitic_infestation")]
    pub parasitic_infestation: Handle<SideEffect>,
    #[asset(key = "unseen_presence")]
    pub unseen_presence: Handle<SideEffect>,
    #[asset(key = "explosive_burp")]
    pub explosive_burp: Handle<SideEffect>,
    #[asset(key = "soothing_balm")]
    pub soothing_balm: Handle<SideEffect>,
    #[asset(key = "euphoric_bliss")]
    pub euphoric_bliss: Handle<SideEffect>,
    #[asset(key = "bitter_aftershock")]
    pub bitter_aftershock: Handle<SideEffect>,
    #[asset(key = "burning_sensation")]
    pub burning_sensation: Handle<SideEffect>,
    #[asset(key = "confusion")]
    pub confusion: Handle<SideEffect>,
    #[asset(key = "muscle_cramps")]
    pub muscle_cramps: Handle<SideEffect>,
    #[asset(key = "cursed")]
    pub cursed: Handle<SideEffect>,
    #[asset(key = "nauseating_odor")]
    pub nauseating_odor: Handle<SideEffect>,
    #[asset(key = "anxiety")]
    pub anxiety: Handle<SideEffect>,
    #[asset(key = "diarrhea")]
    pub diarrhea: Handle<SideEffect>,
    #[asset(key = "purging_purgatory")]
    pub purging_purgatory: Handle<SideEffect>,
    #[asset(key = "vivid_dreams")]
    pub vivid_dreams: Handle<SideEffect>,
    #[asset(key = "itchy_pustules")]
    pub itchy_pustules: Handle<SideEffect>,
    #[asset(key = "mysterious_markings")]
    pub mysterious_markings: Handle<SideEffect>,
    #[asset(key = "insomnia")]
    pub insomnia: Handle<SideEffect>,
    #[asset(key = "nightmares_lullaby")]
    pub nightmares_lullaby: Handle<SideEffect>,
    #[asset(key = "possession")]
    pub possession: Handle<SideEffect>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "2df00c92-cf7b-42c1-a989-dccbad659c13"]
pub struct GameDataAssetDynamicCollection(HashMap<String, GameDataAsset>);

impl DynamicAssetCollection for GameDataAssetDynamicCollection {
    fn register(&self, dynamic_assets: &mut DynamicAssets) {
        for (key, asset) in self.0.iter() {
            dynamic_assets.register_asset(key, Box::new(asset.clone()))
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
enum GameDataAsset {
    SideEffect {
        name: String,
        description: String,
        class: Vec<SideEffectClass>,
    },
    Ingredient {
        name: String,
        description: String,
        texture: String,
        cures: Vec<SideEffectClass>,
        causes: Vec<SideEffectClass>,
        toxicity: i32,
        starter: bool,
    },
}

impl DynamicAsset for GameDataAsset {
    fn load(&self, asset_server: &AssetServer) -> Vec<HandleUntyped> {
        match self {
            GameDataAsset::SideEffect { .. } => vec![],
            GameDataAsset::Ingredient { texture, .. } => vec![asset_server.load_untyped(texture)],
        }
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        let cell = world.cell();
        let asset_server = cell
            .get_resource::<AssetServer>()
            .expect("Failed to get asset server");

        match self {
            GameDataAsset::SideEffect {
                name,
                description,
                class,
            } => {
                let mut side_effects = cell
                    .get_resource_mut::<Assets<SideEffect>>()
                    .expect("Failed to get side effect asset");

                let handle = side_effects
                    .add(SideEffect {
                        name: name.clone(),
                        description: description.clone(),
                        class: class.clone(),
                    })
                    .clone_untyped();

                Ok(DynamicAssetType::Single(handle))
            }
            GameDataAsset::Ingredient {
                name,
                description,
                texture,
                cures,
                causes,
                toxicity,
                starter,
            } => {
                let mut ingredients = cell
                    .get_resource_mut::<Assets<Ingredient>>()
                    .expect("Failed to get ingredient asset");

                let handle = ingredients
                    .add(Ingredient {
                        name: name.clone(),
                        description: description.clone(),
                        texture: asset_server.load(texture.clone()),
                        cures: cures.clone(),
                        causes: causes.clone(),
                        toxicity: toxicity.clone(),
                        starter: starter.clone(),
                        used: false,
                    })
                    .clone_untyped();

                Ok(DynamicAssetType::Single(handle))
            }
        }
    }
}
