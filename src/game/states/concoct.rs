use crate::{
    assets::assets_game_data::{Ingredient, SymptomClass},
    game::potion::PotionMix,
};
use bevy::prelude::*;
use std::collections::HashSet;

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

#[test]
fn test_conconction() {
    let mut app = App::new();

    app.add_plugin(crate::world::global_state::GlobalStatePlugin);
    app.add_plugin(bevy::prelude::AssetPlugin { ..default() });
    app.add_plugin(crate::assets::AssetPlugin);

    let mut next_state = app
        .world
        .resource_mut::<NextState<crate::world::global_state::GlobalState>>();
    next_state.set(crate::world::global_state::GlobalState::Game);

    print!("Iterating through asserts to see if loaded... \n");
    let ingredients = app.world.resource::<Assets<Ingredient>>();
    for (_, ingredient) in ingredients.iter() {
        print!("{:?}", ingredient.name);
    }
}
