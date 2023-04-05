use bevy::{
    prelude::{AssetServer, Assets, Handle, HandleUntyped, Resource, StandardMaterial, World},
    reflect::TypeUuid,
    utils::HashMap,
};
use bevy_asset_loader::prelude::{
    AssetCollection, DynamicAsset, DynamicAssetCollection, DynamicAssetType, DynamicAssets,
};

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

#[derive(TypeUuid)]
#[uuid = "766152e8-d85f-4e58-b4f8-4e375a99ac53"]
pub struct Symptom {
    pub name: String,
    pub class: Vec<SymptomClass>,
    pub description: String,
}

#[derive(TypeUuid)]
#[uuid = "9f249ef7-0fbe-441e-bf87-6cacdc9340e4"]
pub struct Ingredient {
    pub name: String,
    pub texture: Handle<StandardMaterial>,
    pub cures: Vec<SymptomClass>,
    pub causes: Vec<SymptomClass>,
}

#[derive(AssetCollection, Resource)]
pub struct IngredientAssets {
    #[asset(key = "newt_eyes")]
    pub newt_eyes: Handle<Ingredient>,
    #[asset(key = "lead")]
    pub lead: Handle<Ingredient>,
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
                    .expect("Failed to get symptom asset");

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
                    .expect("Failed to get ingredient asset");

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
