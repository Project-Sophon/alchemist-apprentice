use bevy::prelude::*;

use crate::{
    assets::{
        resources_data::IngredientAssets,
        resources_standard::{GlobalAssets, UiAssets},
    },
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

fn game_ui_setup(
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    ui_assets: Res<UiAssets>,
    ingredients: Res<IngredientAssets>,
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
    ingredients: Res<IngredientAssets>,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(33.33), Val::Percent(100.)),
                    ..default()
                },
                ..default()
            },
            IngredientsPanel,
            Name::new("Ingredients Panel"),
        ))
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
