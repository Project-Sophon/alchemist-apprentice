use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use std::{collections::HashSet, fmt};

use crate::{assets::assets_game_data::Symptom, world::global_state::GlobalState};

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
    pub symptoms: HashSet<Symptom>,
    pub toxicity: i32,
}

impl Default for BjornStatus {
    fn default() -> BjornStatus {
        BjornStatus {
            symptoms: HashSet::new(),
            toxicity: 0,
        }
    }
}

impl fmt::Display for BjornStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bjorn Status:: Toxicity:{:?}, Symptoms: {:?}",
            self.toxicity, self.symptoms
        )
    }
}

// ------ SYSTEMS ------

fn setup_initial_bjorn_status(
    mut bjorn_status: ResMut<BjornStatus>,
    symptom_assets: Res<Assets<Symptom>>,
) {
    let mut initial_symptom_pool: HashSet<Symptom> = HashSet::new();
    info!("Setup initial status on entering game state...");
    for (_, symptom) in symptom_assets.iter() {
        initial_symptom_pool.insert(symptom.clone());
    }

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let rand_index: usize = rng.gen_range(0..2);

    let initial_symptom = Vec::from_iter(&initial_symptom_pool)[rand_index];
    bjorn_status.symptoms = HashSet::from_iter(vec![initial_symptom.clone()]);
    info!("Initial Symptoms of Bjorn: {:?}", bjorn_status.symptoms);
}

pub fn give_bjorn_concoction(
    concoction: Concoction,
    bjorn_status: &mut ResMut<BjornStatus>,
    symptoms: &Res<Assets<Symptom>>,
) {
    let cures = concoction.cures;
    for cure in cures {
        let current_bjorn_symptoms = bjorn_status.symptoms.clone();
        let cured_symptoms: Vec<&Symptom> = current_bjorn_symptoms
            .iter()
            .filter(|s| s.class.contains(&cure))
            .collect();
        for cured in cured_symptoms {
            bjorn_status.symptoms.remove(cured);
        }
    }

    let side_effects = concoction.causes;
    info!("Number of side effects {}", side_effects.len().to_string());
    for effect in side_effects {
        let symptom_iter = symptoms.clone().iter();
        let possible_side_effects: Vec<Symptom> = symptom_iter
            .filter(|s| {
                let symptom = s.1;
                symptom.class.contains(&effect)
            })
            .map(|s| s.1.clone())
            .collect();

        if possible_side_effects.len() > 0 {
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
            let rand_index: usize = rng.gen_range(0..possible_side_effects.len());
            bjorn_status
                .symptoms
                .insert(possible_side_effects[rand_index].clone());
        }
    }

    bjorn_status.toxicity = bjorn_status.toxicity + concoction.toxicity;

    info!("{}", bjorn_status.to_string());
}
