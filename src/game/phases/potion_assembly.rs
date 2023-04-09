use bevy::prelude::*;

use crate::{
    assets::resources_standard::{GlobalAssets, UiAssets},
    game::{
        game_phase::GamePhase,
        information::{build_default_information_text, InformationPanel},
        ingredients::{IngredientButton, SelectedIngredient},
        potion::{PotionMix, PotionMixSlot},
    },
    ui::disable_ui::{DisabledUiElement, EnableUiElement},
    world::global_state::GlobalState,
};

pub struct PotionAssemblyPlugin;
impl Plugin for PotionAssemblyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                enable_potion_assembly_ui,
                reset_ingredient_selection,
                reset_information_panel,
                reset_potion_slots,
            )
                .in_schedule(OnEnter(GamePhase::PotionAssembly)),
        )
        .add_system(disable_potion_assembly_ui.in_schedule(OnExit(GamePhase::PotionAssembly)))
        .add_system(reset_potion_slots.in_schedule(OnExit(GlobalState::Game)));
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
    global_assets: Res<GlobalAssets>,
) {
    if let Ok(target) = panel_content.get_single() {
        // remove existing child elements
        commands.entity(target).despawn_descendants();

        commands
            .entity(target)
            .with_children(|parent| build_default_information_text(parent, &global_assets.font));
    }
}

fn reset_potion_slots(
    mut commands: Commands,
    mut potion_slot_query: Query<(Entity, &mut UiImage, &mut PotionMixSlot), With<PotionMixSlot>>,
    mut potion_mix: ResMut<PotionMix>,
    ui_assets: Res<UiAssets>,
) {
    for (entity, mut ui_image, potion_mix_slot) in &mut potion_slot_query {
        commands.entity(entity).despawn_descendants();
        potion_mix.ingredients[potion_mix_slot.index] = Option::None;
        ui_image.texture = ui_assets.potion_circle_slot_empty.clone();
    }
}
