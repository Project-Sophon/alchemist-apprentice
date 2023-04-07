use bevy::prelude::*;
use core::fmt;

use crate::{
    assets::{assets_game_data::Ingredient, resources_standard::UiAssets},
    style::color::PALETTE_PURPLE,
    world::global_state::GlobalState,
};

use super::ingredients::SelectedIngredient;

pub struct PotionPlugin;
impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PotionPanel>()
            .init_resource::<PotionMix>()
            .add_system(slot_interactions.in_set(OnUpdate(GlobalState::Game)));
    }
}

// ------ RESOURCES ------

#[derive(Resource)]
pub struct PotionMix {
    pub ingredients: [Option<Handle<Ingredient>>; 3],
    pub ready: bool,
}

impl PotionMix {
    pub fn update_ingredients(&mut self, index: usize, ingredient: Option<Handle<Ingredient>>) {
        self.ingredients[index] = ingredient;
    }
}

impl fmt::Display for PotionMix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PotionMix: {:?}", self.ingredients)
    }
}

impl Default for PotionMix {
    fn default() -> PotionMix {
        PotionMix {
            ingredients: [None, None, None],
            ready: false,
        }
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PotionPanel;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PotionMixSlot {
    pub index: usize,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct FilledMixSlot;

// ------ SYSTEMS ------

pub fn build_potion_panel(commands: &mut ChildBuilder, ui_assets: &Res<UiAssets>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(300.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            PotionPanel,
            Name::new("Potion Panel"),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(208.), Val::Px(200.)),
                        ..default()
                    },
                    background_color: Color::hex(PALETTE_PURPLE).unwrap().into(),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_potion_mix_slot(parent, &ui_assets.plus_dark_gold_64, 0);
                    spawn_potion_mix_slot(parent, &ui_assets.plus_dark_gold_64, 1);
                    spawn_potion_mix_slot(parent, &ui_assets.plus_dark_gold_64, 2);
                });
        })
        .id()
}

pub fn spawn_potion_mix_slot(commands: &mut ChildBuilder, icon: &Handle<Image>, index: usize) {
    commands
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
            PotionMixSlot { index: index },
            Name::new("Potion Mix Slot"),
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(icon.clone()),
                    ..default()
                },
                Name::new("Slot Icon"),
            ));
        });
}

fn slot_interactions(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut PotionMixSlot),
        (Changed<Interaction>, Without<FilledMixSlot>),
    >,
    selected_ingredient: Res<SelectedIngredient>,
    mut potion_mix: ResMut<PotionMix>,
    ingredients: Res<Assets<Ingredient>>,
) {
    if selected_ingredient.ingredient.is_none() {
        return;
    }

    for (entity, interaction, mut potion_mix_slot) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                // Get Handles and update PotionMix resource
                let handle = selected_ingredient.ingredient.clone().unwrap();
                let ingredient = ingredients.get(&handle).unwrap();
                potion_mix.update_ingredients(potion_mix_slot.index, Option::Some(handle.clone()));

                // Despawn the default slot icon
                commands.entity(entity).despawn_descendants();

                // Add texture of the ingredient
                commands.entity(entity).with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage::new(ingredient.texture.clone()),
                            ..default()
                        },
                        Name::new(format!("Selected: {}", ingredient.name)),
                    ));
                });

                // Log inner array
                info!("{}", potion_mix.to_string());
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
