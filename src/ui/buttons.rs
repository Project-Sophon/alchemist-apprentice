use bevy::prelude::*;

use crate::style::color::{PALETTE_CREAM, PALETTE_DARK_GOLD, PALETTE_DARK_PURPLE, PALETTE_GOLD};

use super::common::{DisableUiElement, DisabledUiElement, EnableUiElement};

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IngredientButton>()
            .add_system(ingredient_button_interactions)
            .add_system(enable_buttons)
            .add_system(disable_buttons);
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IngredientButton;

// ------ SYSTEMS ------

fn ingredient_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<IngredientButton>,
            Without<DisabledUiElement>,
        ),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::hex(PALETTE_CREAM).unwrap().into();
            }
            Interaction::Hovered => {
                *color = Color::hex(PALETTE_GOLD).unwrap().into();
            }
            Interaction::None => {
                *color = Color::hex(PALETTE_DARK_GOLD).unwrap().into();
            }
        }
    }
}

fn enable_buttons(
    mut panel_buttons_query: Query<
        &mut BackgroundColor,
        (With<Button>, With<EnableUiElement>, With<DisabledUiElement>),
    >,
) {
    for mut color in &mut panel_buttons_query {
        *color = Color::hex(PALETTE_DARK_GOLD).unwrap().into();
    }
}

fn disable_buttons(
    mut panel_buttons_query: Query<
        &mut BackgroundColor,
        (
            With<Button>,
            With<DisableUiElement>,
            Without<DisabledUiElement>,
        ),
    >,
) {
    for mut color in &mut panel_buttons_query {
        *color = Color::hex(PALETTE_DARK_PURPLE).unwrap().into();
    }
}
