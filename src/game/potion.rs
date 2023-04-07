use bevy::prelude::*;

use crate::{
    assets::{assets_game_data::Ingredient, resources_standard::UiAssets},
    style::color::PALETTE_PURPLE,
};

pub struct PotionPlugin;
impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PotionPanel>();
    }
}

// ------ RESOURCES ------

#[derive(Resource)]
pub struct PotionMix {
    pub ingredient_1: Option<Handle<Ingredient>>,
    pub ingredient_2: Option<Handle<Ingredient>>,
    pub ingredient_3: Option<Handle<Ingredient>>,
    pub ready: bool,
}

impl Default for PotionMix {
    fn default() -> PotionMix {
        PotionMix {
            ingredient_1: None,
            ingredient_2: None,
            ingredient_3: None,
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
pub struct PotionMixSlot;

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
                    spawn_potion_mix_slot(parent, &ui_assets.plus_dark_gold_64);
                    spawn_potion_mix_slot(parent, &ui_assets.plus_dark_gold_64);
                    spawn_potion_mix_slot(parent, &ui_assets.plus_dark_gold_64);
                });
        })
        .id()
}

pub fn spawn_potion_mix_slot(commands: &mut ChildBuilder, icon: &Handle<Image>) {
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
            PotionMixSlot,
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
