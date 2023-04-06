use bevy::prelude::*;

use crate::{
    game::game_phase::GamePhase,
    ui::{
        buttons::IngredientButton,
        common::{DisabledUiElement, EnableUiElement},
    },
};

pub struct PotionAssemblyPlugin;
impl Plugin for PotionAssemblyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enable_potion_assembly_ui.in_schedule(OnEnter(GamePhase::PotionAssembly)))
            .add_system(disable_potion_assembly_ui.in_schedule(OnExit(GamePhase::PotionAssembly)));
    }
}

fn enable_potion_assembly_ui(
    mut commands: Commands,
    disabled_panel_buttons_query: Query<Entity, (With<IngredientButton>, With<DisabledUiElement>)>,
) {
    for entity in &disabled_panel_buttons_query {
        commands.entity(entity).insert(EnableUiElement);
    }
}

fn disable_potion_assembly_ui(
    mut commands: Commands,
    disabled_panel_buttons_query: Query<Entity, (With<IngredientButton>, With<DisabledUiElement>)>,
) {
    for entity in &disabled_panel_buttons_query {
        commands.entity(entity).insert(EnableUiElement);
    }
}
