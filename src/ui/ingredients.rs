use bevy::prelude::*;

use crate::assets::IngredientAssets;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IngredientsArea;

pub fn create_ingredients_area(parent: &mut ChildBuilder) {
    parent.spawn((
        NodeBundle {
            style: Style {
                align_content: AlignContent::Start,
                align_items: AlignItems::Start,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                size: Size {
                    width: Val::Px(300.),
                    height: Val::Px(350.),
                },
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        },
        IngredientsArea,
        Name::new("Ingredients Area")
    ));
}

#[derive(Component)]
pub struct Ingredient;

pub fn spawn_ingredients(
    mut commands: Commands,
    mut ingredients_selector: Query<Entity, With<IngredientsArea>>,
    assets: Res<IngredientAssets>,
) {
    let ingredients_area = ingredients_selector.get_single_mut().unwrap();

    let first = commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size { width: Val::Px(64.), height: Val::Px(64.) },
                    ..default()
                },
                image: assets.ingredients.get("textures/ingredients/auria_leaf.png").unwrap().clone().into(),
                ..default()
            },
            Ingredient,
        ))
        .id();

    let second = commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size { width: Val::Px(64.), height: Val::Px(64.) },
                    ..default()
                },
                image: assets.ingredients.get("textures/ingredients/crow_foot.png").unwrap().clone().into(),
                ..default()
            },
            Ingredient,
        ))
        .id();

    let third = commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size { width: Val::Px(64.), height: Val::Px(64.) },
                    ..default()
                },
                image: assets.ingredients.get("textures/ingredients/dluger_heart.png").unwrap().clone().into(),
                ..default()
            },
            Ingredient,
        ))
        .id();

        let fourth = commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size { width: Val::Px(64.), height: Val::Px(64.) },
                    ..default()
                },
                image: assets.ingredients.get("textures/ingredients/shadow_beetle.png").unwrap().clone().into(),
                ..default()
            },
            Ingredient,
        ))
        .id();
    
        let fifth = commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size { width: Val::Px(64.), height: Val::Px(64.) },
                    ..default()
                },
                image: assets.ingredients.get("textures/ingredients/zizima_root.png").unwrap().clone().into(),
                ..default()
            },
            Ingredient,
        ))
        .id();

    commands.entity(ingredients_area).add_child(first);
    commands.entity(ingredients_area).add_child(second);
    commands.entity(ingredients_area).add_child(third);
    commands.entity(ingredients_area).add_child(fourth);
    commands.entity(ingredients_area).add_child(fifth);
}
