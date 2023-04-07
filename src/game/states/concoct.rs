use bevy::prelude::*;
use std::collections::HashSet;

use crate::{
    assets::assets_game_data::{Ingredient, SymptomClass},
    game::potion::PotionMix,
};

pub struct Concoction {
    pub toxicity: i32,
    pub cures: HashSet<SymptomClass>,
    pub causes: HashSet<SymptomClass>,
}

pub fn concoct(potion_mix: PotionMix, ingredients: Res<Assets<Ingredient>>) -> Concoction {
    let mut cures: HashSet<SymptomClass> = HashSet::new();
    let mut causes: HashSet<SymptomClass> = HashSet::new();
    let mut toxicity: i32 = 0;

    for ingredient in potion_mix.ingredients {
        match ingredient {
            Some(handle) => {
                let i = ingredients.get(&handle).unwrap().clone();
                toxicity += i.toxicity;
                for c in i.cures.clone() {
                    cures.insert(c);
                }
                for c in i.causes.clone() {
                    causes.insert(c);
                }
            }
            None => {
                // do nothing
            }
        }
    }

    Concoction {
        toxicity: toxicity,
        cures: cures,
        causes: causes,
    }
}
