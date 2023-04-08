use std::fmt;

use bevy::{
    prelude::{Handle, Image},
    reflect::TypeUuid,
};

#[derive(TypeUuid, Clone, Eq, Hash, Debug)]
#[uuid = "766152e8-d85f-4e58-b4f8-4e375a99ac53"]
pub struct SideEffect {
    pub name: String,
    pub description: String,
    pub class: Vec<SideEffectClass>,
}

impl PartialEq for SideEffect {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}

impl fmt::Display for SideEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n{:?}", self.name, self.description)
    }
}

#[derive(TypeUuid)]
#[uuid = "9f249ef7-0fbe-441e-bf87-6cacdc9340e4"]
pub struct Ingredient {
    pub name: String,
    pub description: String,
    pub texture: Handle<Image>,
    pub cures: Vec<SideEffectClass>,
    pub causes: Vec<SideEffectClass>,
    pub toxicity: i32,
    pub starter: bool,
    pub used: bool,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SideEffectClass {
    Pain,
    Parasite,
    Occult,
    Gastrointestinal,
    Skin,
    Mental,
}

impl fmt::Display for SideEffectClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
