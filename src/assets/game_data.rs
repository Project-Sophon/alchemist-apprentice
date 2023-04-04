use bevy::{
    prelude::{AssetServer, Assets, Handle, HandleUntyped, Resource, StandardMaterial, World},
    reflect::TypeUuid,
    utils::HashMap,
};
use bevy_asset_loader::prelude::{
    DynamicAsset, DynamicAssetCollection, DynamicAssetType, DynamicAssets, AssetCollection,
};

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "2df00c92-cf7b-42c1-a989-dccbad659c13"]
pub struct GameDataAssetDynamicCollection(HashMap<String, Vec<GameDataAsset>>);

impl DynamicAssetCollection for GameDataAssetDynamicCollection {
    fn register(&self, dynamic_assets: &mut DynamicAssets) {
        for (key, assets) in self.0.iter() {
            for asset in assets {
                dynamic_assets.register_asset(key, Box::new(asset.clone()));
            }
        }
    }
}


#[derive(AssetCollection, Resource)]
pub struct GameData {
    #[asset(key = "symptoms", collection(typed))]
    symptoms: Vec<Handle<Symptom>>,
    #[asset(key = "ingredients", collection(typed))]
    ingredients: Vec<Handle<Ingredient>>,
}

#[derive(TypeUuid)]
#[uuid = "766152e8-d85f-4e58-b4f8-4e375a99ac53"]
pub struct Symptom {
    name: String,
    class: Vec<SymptomClass>,
    description: String,
}

#[derive(TypeUuid)]
#[uuid = "9f249ef7-0fbe-441e-bf87-6cacdc9340e4"]
pub struct Ingredient {
    name: String,
    texture: Handle<StandardMaterial>,
    cures: Vec<SymptomClass>,
    causes: Vec<SymptomClass>,
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
    EndGame,
}

#[derive(serde::Deserialize, Debug, Clone)]
enum GameDataAsset {
    Symptom {
        name: String,
        class: Vec<SymptomClass>,
        description: String,
    },
    Ingredient {
        name: String,
        texture: String,
        cures: Vec<SymptomClass>,
        causes: Vec<SymptomClass>,
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
                class,
                description,
            } => {
                let mut symptoms = cell
                    .get_resource_mut::<Assets<Symptom>>()
                    .expect("Failed to get symptom assets");

                let handle = symptoms
                    .add(Symptom {
                        name: name.clone(),
                        class: class.clone(),
                        description: description.clone(),
                    })
                    .clone_untyped();

                Ok(DynamicAssetType::Single(handle))
            }
            GameDataAsset::Ingredient {
                name,
                texture,
                cures,
                causes,
            } => {
                let mut ingredients = cell
                    .get_resource_mut::<Assets<Ingredient>>()
                    .expect("Failed to get ingredient assets");

                let handle = ingredients
                    .add(Ingredient {
                        name: name.clone(),
                        texture: asset_server.load(texture.clone()),
                        cures: cures.clone(),
                        causes: causes.clone(),
                    })
                    .clone_untyped();

                Ok(DynamicAssetType::Single(handle))
            }
        }
    }
}
