use bevy::prelude::*;

use crate::{
    ui::{
        buttons::BasicButton,
        common::{DisabledUiElement, EnableUiElement},
    },
};

use super::game_phase::GamePhase;

pub struct PotionAssemblyPlugin;
impl Plugin for PotionAssemblyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enable_potion_assembly_ui.in_schedule(OnEnter(GamePhase::PotionAssembly)))
            .add_system(disable_potion_assembly_ui.in_schedule(OnExit(GamePhase::PotionAssembly)));
    }
}

fn enable_potion_assembly_ui(
    mut commands: Commands,
    disabled_panel_buttons_query: Query<Entity, (With<BasicButton>, With<DisabledUiElement>)>,
) {
    for entity in &disabled_panel_buttons_query {
        commands.entity(entity).insert(EnableUiElement);
    }
}

fn disable_potion_assembly_ui(
    mut commands: Commands,
    disabled_panel_buttons_query: Query<Entity, (With<BasicButton>, With<DisabledUiElement>)>,
) {
    for entity in &disabled_panel_buttons_query {
        commands.entity(entity).insert(EnableUiElement);
    }
}
