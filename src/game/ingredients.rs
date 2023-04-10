use bevy::{asset::HandleId, prelude::*};
use bevy_kira_audio::{prelude::*, Audio};

use crate::{
    assets::{
        assets_game_data::Ingredient,
        resources_standard::{AudioAssets, UiAssets},
    },
    ui::disable_ui::DisabledUiElement,
    world::global_state::GlobalState,
};

use super::game_phase::GamePhase;
pub struct IngredientsPlugin;
impl Plugin for IngredientsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedIngredient>()
            .register_type::<IngredientsPanel>()
            .register_type::<IngredientButton>()
            .add_system(reset_ingredients.in_schedule(OnEnter(GlobalState::Game)))
            .add_systems(
                (ingredient_button_interactions, select_ingredient)
                    .in_set(OnUpdate(GamePhase::AilmentStatement)),
            )
            .add_systems(
                (ingredient_button_interactions, select_ingredient)
                    .in_set(OnUpdate(GamePhase::PotionAssembly)),
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
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for (interaction, ingredient_button, mut bkg_image) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                audio.play(audio_assets.click.clone());
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
                    justify_content: JustifyContent::SpaceEvenly,
                    align_content: AlignContent::SpaceEvenly,
                    gap: Size::width(Val::Px(48.)),
                    ..default()
                },
                ..default()
            },
            IngredientsPanel,
            Name::new("Ingredients Panel"),
        ))
        .with_children(|parent| {
            // Sort Ingredients before output
            let mut sorted_ingredient: Vec<(HandleId, &Ingredient)> =
                ingredients.clone().iter().collect();

            sorted_ingredient.sort_by(|a, b| a.1.order.cmp(&b.1.order));

            for (id, ingredient) in sorted_ingredient {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(64.), Val::Px(64.)),
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
            }
        })
        .id()
}

pub fn update_ingredients_used(
    ingredients: &mut ResMut<Assets<Ingredient>>,
    used_ingredients: &[Option<Handle<Ingredient>>; 3],
) {
    used_ingredients.iter().for_each(|maybe_ingredient| {
        maybe_ingredient.as_ref().map(|ingredient| {
            if let Some(ingredient) = ingredients.get_mut(&ingredient) {
                ingredient.used = true;
            }
        });
    });
}

pub fn reset_ingredients(mut ingredients: ResMut<Assets<Ingredient>>) {
    ingredients.iter_mut().for_each(|i| i.1.used = false);
}
