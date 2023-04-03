use bevy::prelude::*;

use crate::ui::{
    buttons::PanelButton,
    common::{DisabledUiElement, EnableUiElement},
};

use super::state::GamePhase;

pub struct AssemblyPlugin;
impl Plugin for AssemblyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enable_assembly_ui.in_schedule(OnEnter(GamePhase::IngredientAssembly)));
    }
}

fn enable_assembly_ui(
    mut commands: Commands,
    disabled_panel_buttons_query: Query<Entity, (With<PanelButton>, With<DisabledUiElement>)>,
) {
    for entity in &disabled_panel_buttons_query {
        commands.entity(entity).insert(EnableUiElement);
    }
}
