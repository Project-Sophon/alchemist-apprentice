use bevy::{
    prelude::{AssetServer, Assets, Handle, HandleUntyped, Resource, World},
    utils::HashMap,
};
use bevy_asset_loader::prelude::{
    AssetCollection, DynamicAsset, DynamicAssetCollection, DynamicAssetType, DynamicAssets,
};

use super::assets_game_data::{Ingredient, Symptom, SymptomClass};

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
    #[asset(key = "genital_sores")]
    pub genital_sores: Handle<Symptom>,
    #[asset(key = "flatulence")]
    pub flatulence: Handle<Symptom>,
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
        class: Vec<SymptomClass>,
    },
    Ingredient {
        name: String,
        description: String,
        texture: String,
        cures: Vec<SymptomClass>,
        causes: Vec<SymptomClass>,
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
            GameDataAsset::Symptom { name, class } => {
                let mut symptoms = cell
                    .get_resource_mut::<Assets<Symptom>>()
                    .expect("Failed to get symptom asset");

                let handle = symptoms
                    .add(Symptom {
                        name: name.clone(),
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
