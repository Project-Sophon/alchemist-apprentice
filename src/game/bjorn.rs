use std::collections::HashSet;

use bevy::prelude::*;

use crate::{assets::assets_game_data::Symptom, world::global_state::GlobalState};

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
    bjorn_status: ResMut<BjornStatus>,
) {
    info!("Setup initial status on entering game state...");
}
