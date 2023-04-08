use bevy::prelude::*;

use crate::{
    assets::resources_standard::GlobalAssets,
    game::{
        bjorn::BjornStatus,
        game_phase::GamePhase,
        information::{build_default_information_text, InformationPanel},
        ingredients::{IngredientButton, SelectedIngredient},
    },
    ui::disable_ui::{DisabledUiElement, EnableUiElement},
};

pub struct PotionAssemblyPlugin;
impl Plugin for PotionAssemblyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                enable_potion_assembly_ui,
                reset_ingredient_selection,
                reset_information_panel,
            )
                .in_schedule(OnEnter(GamePhase::PotionAssembly)),
        )
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

fn reset_ingredient_selection(mut selected_ingredient: ResMut<SelectedIngredient>) {
    selected_ingredient.ingredient = None;
}

fn reset_information_panel(
    mut commands: Commands,
    panel_content: Query<Entity, With<InformationPanel>>,
    bjorn_status: ResMut<BjornStatus>,
    global_assets: Res<GlobalAssets>,
) {
    if !bjorn_status.is_changed() {
        return;
    }

    if let Ok(target) = panel_content.get_single() {
        // remove existing child elements
        commands.entity(target).despawn_descendants();

        commands
            .entity(target)
            .with_children(|parent| build_default_information_text(parent, &global_assets.font));
    }
}
