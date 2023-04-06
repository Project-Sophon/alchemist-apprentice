use bevy::prelude::*;

use crate::{
    assets::{assets_game_data::Ingredient, resources_game_data::IngredientAssets},
    style::color::{PALETTE_CREAM, PALETTE_DARK_GOLD, PALETTE_GOLD},
    ui::disable_ui::DisabledUiElement,
};
pub struct IngredientsPlugin;
impl Plugin for IngredientsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedIngredient>()
            .register_type::<IngredientsPanel>()
            .register_type::<IngredientButton>()
            .add_system(ingredient_button_interactions);
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
