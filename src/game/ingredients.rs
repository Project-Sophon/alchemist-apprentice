use bevy::prelude::*;

use crate::{
    assets::{assets_game_data::Ingredient, resources_standard::UiAssets},
    style::color::{PALETTE_CREAM, PALETTE_DARK_GOLD, PALETTE_GOLD},
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
                (ingredient_button_interactions, select_ingredient)
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

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SelectedIngredientButton;

// ------ SYSTEMS ------

fn ingredient_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut IngredientButton, &mut UiImage),
        (
            Changed<Interaction>,
            Without<DisabledUiElement>,
            Without<SelectedIngredientButton>,
        ),
    >,
    mut selected_ingredient: ResMut<SelectedIngredient>,
    ui_assets: Res<UiAssets>,
) {
    for (interaction, ingredient_button, mut bkg_image) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                // Update the selected ingredient (handle)
                selected_ingredient.ingredient = Some(ingredient_button.ingredient.clone());

                *bkg_image = UiImage::new(ui_assets.ingredient_button_selected.clone());
            }
            Interaction::Hovered => {
                *bkg_image = UiImage::new(ui_assets.ingredient_button_hover.clone());
            }
            Interaction::None => {
                *bkg_image = UiImage::new(ui_assets.ingredient_button_normal.clone());
            }
        }
    }
}

fn select_ingredient(
    mut commands: Commands,
    mut ingredient_buttons: Query<(Entity, &IngredientButton, &mut UiImage)>,
    selected_ingredient: Res<SelectedIngredient>,
    ui_assets: Res<UiAssets>,
) {
    // only run when the selected ingredient has changed!
    if !selected_ingredient.is_changed() {
        return;
    }

    match &selected_ingredient.ingredient {
        Some(selected) => {
            for (entity, button, mut bkg_image) in &mut ingredient_buttons {
                if selected.id() == button.ingredient.id() {
                    commands.entity(entity).insert(SelectedIngredientButton);
                    *bkg_image = UiImage::new(ui_assets.ingredient_button_selected.clone());
                } else {
                    commands.entity(entity).remove::<SelectedIngredientButton>();
                    *bkg_image = UiImage::new(ui_assets.ingredient_button_normal.clone());
                }
            }
        }
        None => {
            for (entity, _, mut bkg_image) in &mut ingredient_buttons {
                commands.entity(entity).remove::<SelectedIngredientButton>();
                *bkg_image = UiImage::new(ui_assets.ingredient_button_normal.clone());
            }
        }
    }
}

pub fn build_ingredients_panel(
    commands: &mut ChildBuilder,
    ui_assets: &Res<UiAssets>,
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
                                    image: UiImage::new(ui_assets.ingredient_button_normal.clone()),
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
