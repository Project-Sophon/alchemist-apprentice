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
    #[asset(key = "solisaurum")]
    pub solisaurum: Handle<Ingredient>,
    #[asset(key = "lunadewleaf")]
    pub lunadewleaf: Handle<Ingredient>,
    #[asset(key = "draconiscale")]
    pub draconiscale: Handle<Ingredient>,
    #[asset(key = "celestine_crystal")]
    pub celestine_crystal: Handle<Ingredient>,
    #[asset(key = "venombane_flower")]
    pub venombane_flower: Handle<Ingredient>,
    #[asset(key = "ignisroot")]
    pub ignisroot: Handle<Ingredient>,
    #[asset(key = "aquamaris_pearl")]
    pub aquamaris_pearl: Handle<Ingredient>,
    #[asset(key = "ethernium_dust")]
    pub ethernium_dust: Handle<Ingredient>,
    #[asset(key = "verdantia_leaf")]
    pub verdantia_leaf: Handle<Ingredient>,
    #[asset(key = "phantasma_essence")]
    pub phantasma_essence: Handle<Ingredient>,
    #[asset(key = "amethyst_ember")]
    pub amethyst_ember: Handle<Ingredient>,
    #[asset(key = "silvermist_moss")]
    pub silvermist_moss: Handle<Ingredient>,
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
