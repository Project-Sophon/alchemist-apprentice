use bevy::prelude::*;

use crate::{
    assets::assets_game_data::Ingredient,
    style::color::{PALETTE_CREAM, PALETTE_DARK_BLUE},
};

use super::ingredients::SelectedIngredient;
pub struct InformationPlugin;
impl Plugin for InformationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InformationPanel>()
            .register_type::<InformationPanelContent>();
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct InformationPanel;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct InformationPanelContent;

// ------ SYSTEMS ------

pub fn build_information_panel(
    commands: &mut ChildBuilder,
    selected_ingredient: &Res<SelectedIngredient>,
    font: Handle<Font>,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(344.), Val::Percent(100.)),
                    ..default()
                },
                ..default()
            },
            InformationPanel,
            Name::new("Information Panel"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(344.), Val::Px(408.)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::hex(PALETTE_CREAM).unwrap().into(),
                        ..default()
                    },
                    InformationPanelContent,
                    Name::new("Information Panel Content"),
                ))
                .with_children(|parent| {
                    if let Some(selected_ingredient) = &selected_ingredient.ingredient {
                        build_ingredient_information(parent, font, selected_ingredient);
                    } else {
                        build_default_information_text(parent, font);
                    }
                });
        })
        .id()
}

pub fn build_ingredient_information(
    commands: &mut ChildBuilder,
    font: Handle<Font>,
    ingredient: &Ingredient,
) {
    commands.spawn((
        ImageBundle {
            image: UiImage::new(ingredient.texture.clone()),
            ..default()
        },
        Name::new("Ingredient Image"),
    ));
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                ingredient.name.clone(),
                TextStyle {
                    font,
                    font_size: 16.,
                    color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                },
            ),
            style: INFO_TEXT_STYLE,
            ..default()
        },
        Name::new("Ingredient Name"),
    ));
}

pub fn build_default_information_text(commands: &mut ChildBuilder, font: Handle<Font>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "This text shows when no ingredients are selected ...",
                TextStyle {
                    font,
                    font_size: 16.,
                    color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                },
            ),
            style: INFO_TEXT_STYLE,
            ..default()
        },
        Name::new("Default Info Text"),
    ));
}

const INFO_TEXT_STYLE: Style = Style {
    max_size: Size {
        width: Val::Px(300.),
        height: Val::Undefined,
    },
    ..Style::DEFAULT
};
