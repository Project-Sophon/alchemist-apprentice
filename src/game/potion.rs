use bevy::prelude::*;
use core::fmt;
use std::borrow::Borrow;

use crate::{
    assets::{
        self,
        assets_game_data::Ingredient,
        resources_standard::{GlobalAssets, UiAssets},
    },
    style::color::PALETTE_PURPLE,
    world::global_state::GlobalState,
};

use super::{ingredients::SelectedIngredient, phases::concoct::spawn_concoct_action};

pub struct PotionPlugin;
impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PotionPanel>()
            .init_resource::<PotionMix>()
            .add_system(slot_interactions.in_set(OnUpdate(GlobalState::Game)));
    }
}

// ------ RESOURCES ------

#[derive(Resource, Clone)]
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
pub struct PotionCircle;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PotionMixSlot {
    pub index: usize,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct FilledMixSlot;

// ------ SYSTEMS ------

pub fn build_potion_panel(
    commands: &mut ChildBuilder,
    global_assets: &Res<GlobalAssets>,
    ui_assets: &Res<UiAssets>,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(228.), Val::Px(360.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            },
            PotionPanel,
            Name::new("Potion Panel"),
        ))
        .with_children(|parent| {
            // Spawn Potion Circle
            parent
                .spawn((
                    ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(228.), Val::Px(238.)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        image: UiImage::new(ui_assets.potion_circle_bkg.clone()),
                        ..default()
                    },
                    PotionCircle,
                    Name::new("Potion Circle"),
                ))
                .with_children(|parent| {
                    spawn_potion_mix_slot(parent, &ui_assets.potion_circle_slot_empty, 0);
                    spawn_potion_mix_slot(parent, &ui_assets.potion_circle_slot_empty, 1);
                    spawn_potion_mix_slot(parent, &ui_assets.potion_circle_slot_empty, 2);
                });

            // Spawn Concoct Button
            spawn_concoct_action(
                parent,
                &ui_assets.concoct_button_normal,
                &global_assets.font,
            );
        })
        .id()
}

pub fn spawn_potion_mix_slot(commands: &mut ChildBuilder, icon: &Handle<Image>, index: usize) {
    // In the potion slot, the top slot is the reference point for the stable start position
    let stable_start_pos = (82., 38.);

    // Compute button position based on index
    let (pox_x, pos_y): (f32, f32) = match index {
        0 => (stable_start_pos.0, stable_start_pos.1),
        1 => (stable_start_pos.0 - 41., stable_start_pos.1 + 66.),
        2 => (stable_start_pos.0 + 41., stable_start_pos.1 + 66.),
        // draw unknown values off screen
        _ => (1000., 1000.),
    };

    commands.spawn((
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(64.), Val::Px(64.)),
                position: UiRect::new(
                    Val::Px(pox_x),
                    Val::Undefined,
                    Val::Px(pos_y),
                    Val::Undefined,
                ),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            image: UiImage::new(icon.clone()),
            ..default()
        },
        PotionMixSlot { index: index },
        Name::new("Potion Mix Slot"),
    ));
}

fn slot_interactions(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiImage, &mut PotionMixSlot),
        (Changed<Interaction>, With<PotionMixSlot>),
    >,
    selected_ingredient: Res<SelectedIngredient>,
    mut potion_mix: ResMut<PotionMix>,
    ingredients: Res<Assets<Ingredient>>,
    ui_assets: Res<UiAssets>,
) {
    for (entity, interaction, mut ui_image, potion_mix_slot) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selected_ingredient.ingredient.is_none() {
                    return;
                }

                // Get Handles and update PotionMix resource
                let handle = selected_ingredient.ingredient.clone().unwrap();
                let ingredient = ingredients.get(&handle).unwrap();
                potion_mix.update_ingredients(potion_mix_slot.index, Option::Some(handle.clone()));

                ui_image.texture = ui_assets.potion_circle_slot_occupied.clone();

                // Add texture of the ingredient with correct background
                commands.entity(entity).with_children(|parent| {
                    render_occupied_slot(
                        parent,
                        ingredient,
                    );
                });

                // Log inner array
                info!("{}", potion_mix.to_string());
            }
            Interaction::Hovered => {
                // Should be safe to unwrap as ingredients is a fixed array of Optional<Ingredient>
                let ingredient_handle = potion_mix.ingredients.get(potion_mix_slot.index).unwrap();
                match ingredient_handle {
                    Some(_) => {
                        ui_image.texture = ui_assets.potion_circle_slot_occupied_hover.clone();
                    }
                    None => {
                        ui_image.texture = ui_assets.potion_circle_slot_hover.clone();
                    }
                }
            }
            Interaction::None => {
                // Should be safe to unwrap as ingredients is a fixed array of Optional<Ingredient>
                let ingredient_handle = potion_mix.ingredients.get(potion_mix_slot.index).unwrap();
                match ingredient_handle {
                    Some(_) => {
                        ui_image.texture = ui_assets.potion_circle_slot_occupied.clone();
                    }
                    None => {
                        ui_image.texture = ui_assets.potion_circle_slot_empty.clone();
                    }
                }
            }
        }
    }
}

fn render_occupied_slot(
    parent: &mut ChildBuilder,
    ingredient: &Ingredient,
) {
    parent.spawn((
        ImageBundle {
            style: Style {
                size: Size {
                    height: Val::Px(32.),
                    width: Val::Px(32.),
                },
                ..default()
            },
            image: UiImage::new(ingredient.texture.clone()),
            ..default()
        },
        Name::new(format!("Selected Slot Icon: {}", ingredient.name)),
    ));
}
