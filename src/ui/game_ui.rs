use bevy::{asset::Asset, prelude::*};

use crate::{
    assets::{
        assets_data::Ingredient,
        resources_data::IngredientAssets,
        resources_standard::{GlobalAssets, UiAssets},
    },
    style::color::PALETTE_GOLD,
    world::{despawn::despawn_entity, global_state::GlobalState},
};

use super::buttons::create_basic_button;
pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_ui_setup.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(despawn_entity::<GameUI>.in_schedule(OnExit(GlobalState::Game)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GameUI;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IngredientsPanel;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct InformationPanel;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PotionPanel;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IngredientButton;

fn game_ui_setup(
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    ui_assets: Res<UiAssets>,
    ingredients: Res<Assets<Ingredient>>,
) {
    let font = global_assets.font.clone();

    commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(456.0)),
                    align_content: AlignContent::End,
                    align_self: AlignSelf::FlexEnd,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                image: ui_assets.game_ui_bkg.clone().into(),
                ..default()
            },
            GameUI,
            Name::new("Game UI"),
        ))
        .with_children(|parent| {
            build_ingredients_panel(parent, ingredients);
            build_information_panel(parent);
            build_potion_panel(parent);
        });
}

fn build_ingredients_panel(
    commands: &mut ChildBuilder,
    ingredients: Res<Assets<Ingredient>>,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(33.33), Val::Percent(100.)),
                    gap: Size::all(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
            IngredientsPanel,
            Name::new("Ingredients Panel"),
        ))
        .with_children(|parent| {
            for (id, ingredient) in ingredients.iter() {
                info!(ingredient.name);
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(20.), Val::Px(20.)),
                            ..default()
                        },
                        background_color: Color::hex(PALETTE_GOLD).unwrap().into(),
                        ..default()
                    },
                    IngredientButton,
                    Name::new("Ingredient Button"),
                ));
            }
        })
        .id()
}

fn build_information_panel(commands: &mut ChildBuilder) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(33.33), Val::Percent(100.)),
                    ..default()
                },
                ..default()
            },
            InformationPanel,
            Name::new("Information Panel"),
        ))
        .id()
}

fn build_potion_panel(commands: &mut ChildBuilder) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(33.33), Val::Percent(100.)),
                    ..default()
                },
                ..default()
            },
            PotionPanel,
            Name::new("Potion Panel"),
        ))
        .id()
}
