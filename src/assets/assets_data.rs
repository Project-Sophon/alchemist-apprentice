use bevy::{
    prelude::{Handle, StandardMaterial},
    reflect::TypeUuid,
};

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
