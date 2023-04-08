use bevy::prelude::*;
use std::collections::HashSet;

use crate::{
    assets::{assets_game_data::Symptom, resources_game_data::SymptomAssets},
    world::global_state::GlobalState,
};

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
    pub toxicity: u32,
}

impl Default for BjornStatus {
    fn default() -> BjornStatus {
        BjornStatus {
            symptoms: HashSet::new(),
            toxicity: 0,
        }
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

    let initial_symptom = Vec::from_iter(&initial_symptom_pool)[0];
    bjorn_status.symptoms = HashSet::from_iter(vec![initial_symptom.clone()]);
    info!("Initial Symptoms of Bjorn: {:?}", bjorn_status.symptoms);
}
