use bevy::{
    prelude::{AssetServer, Assets, Handle, HandleUntyped, Resource, World},
    utils::HashMap,
};
use bevy_asset_loader::prelude::{
    AssetCollection, DynamicAsset, DynamicAssetCollection, DynamicAssetType, DynamicAssets,
};

use super::assets_game_data::{Ingredient, SideEffectClass, Symptom};

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
pub struct SymptomAssets {
    #[asset(key = "headache")]
    pub headache: Handle<Symptom>,
    #[asset(key = "parasitic_infestation")]
    pub parasitic_infestation: Handle<Symptom>,
    #[asset(key = "unseen_presence")]
    pub unseen_presence: Handle<Symptom>,
    #[asset(key = "explosive_burp")]
    pub explosive_burp: Handle<Symptom>,
    #[asset(key = "soothing_balm")]
    pub soothing_balm: Handle<Symptom>,
    #[asset(key = "euphoric_bliss")]
    pub euphoric_bliss: Handle<Symptom>,
    #[asset(key = "bitter_aftershock")]
    pub bitter_aftershock: Handle<Symptom>,
    #[asset(key = "burning_sensation")]
    pub burning_sensation: Handle<Symptom>,
    #[asset(key = "confusion")]
    pub confusion: Handle<Symptom>,
    #[asset(key = "muscle_cramps")]
    pub muscle_cramps: Handle<Symptom>,
    #[asset(key = "cursed")]
    pub cursed: Handle<Symptom>,
    #[asset(key = "nauseating_odor")]
    pub nauseating_odor: Handle<Symptom>,
    #[asset(key = "anxiety")]
    pub anxiety: Handle<Symptom>,
    #[asset(key = "diarrhea")]
    pub diarrhea: Handle<Symptom>,
    #[asset(key = "purging_purgatory")]
    pub purging_purgatory: Handle<Symptom>,
    #[asset(key = "vivid_dreams")]
    pub vivid_dreams: Handle<Symptom>,
    #[asset(key = "itchy_pustules")]
    pub itchy_pustules: Handle<Symptom>,
    #[asset(key = "mysterious_markings")]
    pub mysterious_markings: Handle<Symptom>,
    #[asset(key = "insomnia")]
    pub insomnia: Handle<Symptom>,
    #[asset(key = "nightmares_lullaby")]
    pub nightmares_lullaby: Handle<Symptom>,
    #[asset(key = "possession")]
    pub possession: Handle<Symptom>,
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
    Symptom {
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
            GameDataAsset::Symptom { .. } => vec![],
            GameDataAsset::Ingredient { texture, .. } => vec![asset_server.load_untyped(texture)],
        }
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        let cell = world.cell();
        let asset_server = cell
            .get_resource::<AssetServer>()
            .expect("Failed to get asset server");

        match self {
            GameDataAsset::Symptom {
                name,
                description,
                class,
            } => {
                let mut symptoms = cell
                    .get_resource_mut::<Assets<Symptom>>()
                    .expect("Failed to get symptom asset");

                let handle = symptoms
                    .add(Symptom {
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
