use bevy::prelude::*;

use crate::{
    assets::{assets_data::Ingredient, resources_standard::UiAssets},
    style::color::PALETTE_PURPLE,
    ui::buttons::IngredientButton,
    world::{
        common::{WINDOW_HEIGHT, WINDOW_WIDTH},
        despawn::despawn_entity,
        global_state::GlobalState,
    },
};

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_ui_setup.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(despawn_entity::<GameUiContainer>.in_schedule(OnExit(GlobalState::Game)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GameUiContainer;

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
    ui_assets: Res<UiAssets>,
    ingredients: Res<Assets<Ingredient>>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Undefined,
                        bottom: Val::Undefined,
                    },
                    size: Size::new(Val::Px(WINDOW_WIDTH.into()), Val::Px(WINDOW_HEIGHT.into())),
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
                            padding: UiRect::all(Val::Px(20.)),
                            gap: Size::all(Val::Px(20.)),
                            ..default()
                        },
                        image: ui_assets.game_ui_bkg.clone().into(),
                        ..default()
                    },
                    Name::new("Game UI Control Area"),
                ))
                .with_children(|parent| {
                    build_ingredients_panel(parent, ingredients);
                    build_information_panel(parent);
                    build_potion_panel(parent);
                });
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
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                ..default()
            },
            IngredientsPanel,
            Name::new("Ingredients Panel"),
        ))
        .with_children(|parent| {
            for (_id, ingredient) in ingredients.iter() {
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
                                IngredientButton,
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
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(432.), Val::Px(408.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::hex(PALETTE_PURPLE).unwrap().into(),
                ..default()
            });
        })
        .id()
}

fn build_potion_panel(commands: &mut ChildBuilder) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(33.33), Val::Percent(100.)),
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
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(208.), Val::Px(200.)),
                    ..default()
                },
                background_color: Color::hex(PALETTE_PURPLE).unwrap().into(),
                ..default()
            });
        })
        .id()
}
