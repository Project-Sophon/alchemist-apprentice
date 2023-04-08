use bevy::prelude::*;

use crate::{
    assets::{
        assets_game_data::Ingredient,
        resources_standard::{GlobalAssets, UiAssets},
    },
    style::color::PALETTE_CREAM,
    world::{
        common::{WINDOW_HEIGHT, WINDOW_WIDTH},
        despawn::despawn_entity,
        global_state::GlobalState,
    },
};

use super::{
    information::build_information_panel,
    ingredients::{build_ingredients_panel, SelectedIngredient},
    potion::build_potion_panel,
    status::build_status_panel,
};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(build_level.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(despawn_entity::<LevelContainer>.in_schedule(OnExit(GlobalState::Game)));
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct LevelContainer;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GameUiContainer;

// ------ SYSTEMS ------

pub fn build_level(
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    ui_assets: Res<UiAssets>,
    ingredients: Res<Assets<Ingredient>>,
    selected_ingredient: Res<SelectedIngredient>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(WINDOW_WIDTH.into()), Val::Px(WINDOW_HEIGHT.into())),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Undefined,
                        bottom: Val::Undefined,
                    },
                    ..default()
                },
                background_color: Color::hex(PALETTE_CREAM).unwrap().into(),
                ..default()
            },
            LevelContainer,
            Name::new("Level Container"),
        ))
        .with_children(|parent| {
            // Build Status Panel
            build_status_panel(parent, &ui_assets);

            // Build Main UI
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            flex_direction: FlexDirection::Column,
                            flex_basis: Val::Percent(100.),
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Auto,
                                top: Val::Undefined,
                                bottom: Val::Undefined,
                            },
                            size: Size::new(
                                Val::Px(WINDOW_WIDTH.into()),
                                Val::Px(WINDOW_HEIGHT.into()),
                            ),
                            ..default()
                        },
                        ..default()
                    },
                    GameUiContainer,
                    Name::new("Game UI Container"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(WINDOW_WIDTH.into()), Val::Px(456.0)),
                                    align_self: AlignSelf::FlexEnd,
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    padding: UiRect::all(Val::Px(20.)),
                                    ..default()
                                },
                                image: ui_assets.game_ui_bkg.clone().into(),
                                ..default()
                            },
                            Name::new("Game UI Control Area"),
                        ))
                        .with_children(|parent| {
                            build_ingredients_panel(parent, &ui_assets, &ingredients);
                            build_information_panel(
                                parent,
                                &ingredients,
                                &selected_ingredient,
                                &global_assets.font,
                                &ui_assets,
                            );
                            build_potion_panel(parent, &global_assets, &ui_assets);
                        });
                });
        });
}
