use bevy::{utils::HashMap, prelude::{AssetServer, HandleUntyped, World, Assets}};
use bevy_asset_loader::prelude::{DynamicAssetCollection, DynamicAssets, DynamicAsset, DynamicAssetType};

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct GameDataAssetCollection(HashMap<String, GameDataAsset>);

impl DynamicAssetCollection for GameDataAssetCollection {
    fn register(&self, dynamic_assets: &mut DynamicAssets) {
        for (key, asset) in self.0.iter() {
            dynamic_assets.register_asset(key, Box::new(asset.clone()));
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
enum GameDataAsset {
    StringList(Vec<String>),
    SymptomList(Vec<Symptom>),
    IngredientList(Vec<Ingredient>)
}

impl DynamicAsset for GameDataAsset {
    fn load(&self, asset_server: &AssetServer) -> Vec<HandleUntyped> {
        match self {
            GameDataAsset::StringList { .. } => vec![],
            GameDataAsset::SymptomList { .. } => vec![],
            GameDataAsset::IngredientList { .. } => vec![],
        }
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        let cell = world.cell();
        let asset_server = cell
            .get_resource::<AssetServer>()
            .expect("Failed to get asset server");

        match self {
            GameDataAsset::StringList { .. } => {
                let mut stringLists = cell.get_resource_mut::<Assets<Vec<String>>>().expect("Failed to get string list asset");
                let handle = stringLists.add(..).clone_untyped();

                Ok(DynamicAssetType::Single((handle)))
            }
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum SymptomClass {
    Pain,
    STI,
    Congestion,
    Gastro,
    Skin,
    Parasite,
    Occult,
    Mental,
    EndGame
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Symptom {
    pub name: String,
    pub class: Vec<String>,
    pub description: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Ingredient {
    pub name: String,
    pub cures: Vec<String>,
    pub causes: Vec<String>,
}
