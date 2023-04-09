use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use std::{collections::HashSet, fmt, ops::Sub};

use crate::{
    assets::assets_game_data::{cant_you_just_do_it, SideEffect, SideEffectClass},
    world::global_state::GlobalState,
};

use super::phases::concoct::Concoction;

pub struct BjornPlugin;
impl Plugin for BjornPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BjornStatus>()
            .add_system(setup_initial_bjorn_status.in_schedule(OnEnter(GlobalState::Game)));
    }
}

// ------ RESOURCES ------

#[derive(Resource, Clone)]
pub struct BjornStatus {
    pub side_effects: HashSet<SideEffect>,
    pub toxicity: i32,
}

impl Default for BjornStatus {
    fn default() -> BjornStatus {
        BjornStatus {
            side_effects: HashSet::new(),
            toxicity: 0,
        }
    }
}

impl fmt::Display for BjornStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bjorn Status:: Toxicity:{:?}, Ailments: {:?}",
            self.toxicity, self.side_effects
        )
    }
}

// ------ SYSTEMS ------

fn setup_initial_bjorn_status(
    mut bjorn_status: ResMut<BjornStatus>,
    side_effect_assets: Res<Assets<SideEffect>>,
) {
    let mut initial_side_effect_pool: HashSet<SideEffect> = HashSet::new();
    info!("Setup initial status on entering game state...");
    for (_, side_effect) in side_effect_assets.iter() {
        if (side_effect.class.len() == 1) {
            initial_side_effect_pool.insert(side_effect.clone());
        }
    }

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let rand_index: usize = rng.gen_range(0..initial_side_effect_pool.len());

    let initial_side_effect = Vec::from_iter(&initial_side_effect_pool)[rand_index];
    bjorn_status.side_effects = HashSet::from_iter(vec![initial_side_effect.clone()]);
    info!(
        "Initial Side Effects of Bjorn: {:?}",
        bjorn_status.side_effects
    );
    bjorn_status.toxicity = 0;
}

pub fn give_bjorn_concoction(
    concoction: Concoction,
    bjorn_status: &mut ResMut<BjornStatus>,
    side_effects: &Res<Assets<SideEffect>>,
) -> (i32, usize) {
    let cures = concoction.cures;
    for cure in cures {
        let current_bjorn_side_effects = bjorn_status.side_effects.clone();
        let cured_side_effects: Vec<&SideEffect> = current_bjorn_side_effects
            .iter()
            .filter(|s| s.class.contains(&cure))
            .collect();
        for cured in cured_side_effects {
            bjorn_status.side_effects.remove(cured);
        }
    }

    let causes = concoction.causes.clone();
    let all_side_effect_classes = cant_you_just_do_it();
    let exclude_classes = all_side_effect_classes.sub(&causes);

    info!("Number of side effects {}", side_effects.len().to_string());
    let side_effect_iter = side_effects.clone().iter();
    let mut possible_side_effects: Vec<SideEffect> = side_effect_iter
        .filter(|s| {
            let side_effect = s.1;
            for excluded in &exclude_classes {
                if side_effect.class.contains(excluded) {
                    return false;
                }
            }
            return true;
        })
        .map(|s| s.1.clone())
        .collect();


    let mut final_side_effects: Vec<SideEffect> = Vec::new();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    //let rand_index: usize = rng.gen_range(0..initial_side_effect_pool.len());
    info!("CAUSES LENGTH: {}", causes.len());
    for _ in 0..causes.len() {
        let rand_index: usize = rng.gen_range(0..possible_side_effects.len());
        final_side_effects.push(possible_side_effects.remove(rand_index));
    }

    info!("final_side_effects{:?}", final_side_effects);
    for se in final_side_effects {
        bjorn_status.side_effects.insert(se.clone());
    }

    bjorn_status.toxicity = bjorn_status.toxicity + concoction.toxicity;

    info!("{}", bjorn_status.to_string());

    // Return (toxicity, num_side_effects)
    return (bjorn_status.toxicity, bjorn_status.side_effects.len());
}
