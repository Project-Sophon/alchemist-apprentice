use bevy::{asset::HandleId, prelude::*};

use crate::{
    assets::assets_game_data::Ingredient,
    style::color::{PALETTE_CREAM, PALETTE_DARK_GOLD, PALETTE_GOLD, PALETTE_DARK_BLUE},
    ui::disable_ui::DisabledUiElement,
    world::global_state::GlobalState,
};
pub struct IngredientsPlugin;
impl Plugin for IngredientsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedIngredient>()
            .register_type::<IngredientsPanel>()
            .register_type::<IngredientButton>()
            .add_systems(
                (
                    ingredient_button_interactions,
                    highlight_selected_ingredient,
                )
                    .in_set(OnUpdate(GlobalState::Game)),
            );
    }
}

// ------ RESOURCES ------

#[derive(Resource)]
pub struct SelectedIngredient {
    pub ingredient: Option<Handle<Ingredient>>,
}

impl Default for SelectedIngredient {
    fn default() -> SelectedIngredient {
        SelectedIngredient { ingredient: None }
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IngredientsPanel;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IngredientButton {
    ingredient: Handle<Ingredient>,
}

// ------ SYSTEMS ------

fn ingredient_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut IngredientButton, &mut BackgroundColor),
        (Changed<Interaction>, Without<DisabledUiElement>),
    >,
    mut selected_ingredient: ResMut<SelectedIngredient>,
) {
    for (interaction, ingredient_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                // Update the selected ingredient (handle)
                selected_ingredient.ingredient = Some(ingredient_button.ingredient.clone());
                // *color = Color::hex(PALETTE_CREAM).unwrap().into();
            }
            Interaction::Hovered => {
                // *color = Color::hex(PALETTE_GOLD).unwrap().into();
            }
            Interaction::None => {
                // *color = Color::hex(PALETTE_DARK_GOLD).unwrap().into();
            }
        }
    }
}

fn highlight_selected_ingredient(
    mut ingredient_buttons: Query<(&IngredientButton, &mut BackgroundColor)>,
    selected_ingredient: Res<SelectedIngredient>,
) {
    // only run when the selected ingredient has changed!
    if !selected_ingredient.is_changed() {
        return;
    }

    // update background color for all buttons
    for (button, mut color) in &mut ingredient_buttons {
        let new_color = selected_ingredient
            .ingredient
            .clone()
            .map(|selected| {
                if let HandleId::Id(id1, _) = selected.id() {
                    info!("Selected: {}", id1);
                }

                if let HandleId::Id(id2, _) = button.ingredient.id() {
                    info!("Button: {}", id2);
                }

                if selected.id() == button.ingredient.id() {
                    // todo: this is working but not sticking around for some reasons
                    return Color::hex(PALETTE_DARK_BLUE).unwrap();
                    // return Color::hex(PALETTE_CREAM).unwrap();
                } else {
                    return Color::hex(PALETTE_DARK_GOLD).unwrap();
                }
            })
            .unwrap_or(Color::hex(PALETTE_DARK_GOLD).unwrap());

        *color = new_color.into();
    }
}

pub fn build_ingredients_panel(
    commands: &mut ChildBuilder,
    ingredients: &Res<Assets<Ingredient>>,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(300.), Val::Percent(100.)),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                ..default()
            },
            IngredientsPanel,
            Name::new("Ingredients Panel"),
        ))
        .with_children(|parent| {
            for (id, ingredient) in ingredients.iter() {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_basis: Val::Percent(33.33),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Ingredient Container"),
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(64.), Val::Percent(64.)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                IngredientButton {
                                    ingredient: Handle::weak(id),
                                },
                                Name::new("Ingredient Button"),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageBundle {
                                        image: UiImage::new(ingredient.texture.clone()),
                                        ..default()
                                    },
                                    Name::new("Ingredient Button Img"),
                                ));
                            });
                    });
            }
        })
        .id()
}
