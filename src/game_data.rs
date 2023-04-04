use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<GameData>::new(&["game-data.yaml"]))
            .add_system(setup_game_data.on_startup());
    }
}
#[derive(Resource)]
pub struct GameDataHandle(pub Handle<GameData>);

#[derive(serde::Deserialize, Debug, Clone, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct GameData {
    pub symptom_classes: Vec<String>,
    pub symptoms: Vec<Symptom>,
    pub ingredients: Vec<Ingredient>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Symptom {
    pub name: String,
    pub class: String,
    pub description: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Ingredient {
    pub name: String,
    pub cures: String,
    pub causes: String,
}

fn setup_game_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_data = GameDataHandle(asset_server.load("data/test.game-data.yaml"));
    commands.insert_resource(game_data);
}
