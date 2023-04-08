use bevy::{
    prelude::{Handle, Image},
    reflect::TypeUuid,
};

#[derive(TypeUuid, Clone, Eq, Hash)]
#[uuid = "766152e8-d85f-4e58-b4f8-4e375a99ac53"]
pub struct Symptom {
    pub name: String,
    pub class: Vec<SymptomClass>,
}

impl PartialEq for Symptom {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}

#[derive(TypeUuid)]
#[uuid = "9f249ef7-0fbe-441e-bf87-6cacdc9340e4"]
pub struct Ingredient {
    pub name: String,
    pub description: String,
    pub texture: Handle<Image>,
    pub cures: Vec<SymptomClass>,
    pub causes: Vec<SymptomClass>,
    pub toxicity: i32,
    pub starter: bool,
    pub used: bool,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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
