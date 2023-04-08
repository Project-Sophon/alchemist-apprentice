use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use std::{collections::HashSet, fmt};

use crate::{assets::assets_game_data::SideEffect, world::global_state::GlobalState};

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
        initial_side_effect_pool.insert(side_effect.clone());
    }

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let rand_index: usize = rng.gen_range(0..2);

    let initial_side_effect = Vec::from_iter(&initial_side_effect_pool)[rand_index];
    bjorn_status.side_effects = HashSet::from_iter(vec![initial_side_effect.clone()]);
    info!("Initial Side Effects of Bjorn: {:?}", bjorn_status.side_effects);
}

pub fn give_bjorn_concoction(
    concoction: Concoction,
    bjorn_status: &mut ResMut<BjornStatus>,
    side_effects: &Res<Assets<SideEffect>>,
) {
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

    let causes = concoction.causes;
    info!("Number of side effects {}", side_effects.len().to_string());
    for cause in causes {
        let side_effect_iter = side_effects.clone().iter();
        let possible_side_effects: Vec<SideEffect> = side_effect_iter
            .filter(|s| {
                let side_effect = s.1;
                side_effect.class.contains(&cause)
            })
            .map(|s| s.1.clone())
            .collect();

        if possible_side_effects.len() > 0 {
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
            let rand_index: usize = rng.gen_range(0..possible_side_effects.len());
            bjorn_status
                .side_effects
                .insert(possible_side_effects[rand_index].clone());
        }
    }

    bjorn_status.toxicity = bjorn_status.toxicity + concoction.toxicity;

    info!("{}", bjorn_status.to_string());
}
