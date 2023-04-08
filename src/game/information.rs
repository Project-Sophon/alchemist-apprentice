use bevy::prelude::*;

use crate::{
    assets::{
        assets_game_data::{Ingredient, SideEffectClass},
        resources_standard::{GlobalAssets, UiAssets},
    },
    style::color::{PALETTE_CREAM, PALETTE_DARK_BLUE},
    world::global_state::GlobalState,
};

use super::ingredients::SelectedIngredient;
pub struct InformationPlugin;
impl Plugin for InformationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InformationPanel>()
            .register_type::<InformationPanelContent>()
            .add_system(update_information_panel.in_set(OnUpdate(GlobalState::Game)));
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

fn update_information_panel(
    mut commands: Commands,
    panel_content: Query<Entity, With<InformationPanel>>,
    ingredients: Res<Assets<Ingredient>>,
    selected_ingredient: Res<SelectedIngredient>,
    global_assets: Res<GlobalAssets>,
) {
    // only run when the selected ingredient has changed!
    if !selected_ingredient.is_changed() {
        return;
    }

    if let Ok(target) = panel_content.get_single() {
        // remove existing child elements
        commands.entity(target).despawn_descendants();

        // spawn new elements
        commands.entity(target).with_children(|parent| {
            match &selected_ingredient.ingredient {
                Some(handle) => build_ingredient_information(
                    parent,
                    &global_assets.font,
                    ingredients.get(handle).unwrap(),
                ),
                None => build_default_information_text(parent, &global_assets.font),
            };
        });
    }
}

pub fn build_information_panel(
    commands: &mut ChildBuilder,
    ingredients: &Res<Assets<Ingredient>>,
    selected_ingredient: &Res<SelectedIngredient>,
    font: &Handle<Font>,
    ui_assets: &Res<UiAssets>,
) -> Entity {
    commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(344.), Val::Px(378.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                image: ui_assets.info_panel_bkg.clone().into(),
                ..default()
            },
            InformationPanel,
            Name::new("Information Panel"),
        ))
        .with_children(|parent| match &selected_ingredient.ingredient {
            Some(handle) => {
                build_ingredient_information(parent, font, ingredients.get(handle).unwrap())
            }
            None => build_default_information_text(parent, font),
        })
        .id()
}

pub fn build_ingredient_information(
    commands: &mut ChildBuilder,
    font: &Handle<Font>,
    ingredient: &Ingredient,
) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(ingredient.name.clone(), get_info_text_style(font, 24.)),
            style: Style {
                margin: UiRect::bottom(Val::Px(10.)),
                ..INFO_TEXT_BUNDLE_STYLE
            },
            ..default()
        },
        Name::new("Ingredient Name"),
    ));
    commands.spawn((
        ImageBundle {
            image: UiImage::new(ingredient.texture.clone()),
            style: Style {
                margin: UiRect::bottom(Val::Px(20.)),
                ..default()
            },
            ..default()
        },
        Name::new("Ingredient Image"),
    ));
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                ingredient.description.clone(),
                get_info_text_style(font, 16.),
            ),
            style: Style {
                margin: UiRect::bottom(Val::Px(20.)),
                ..INFO_TEXT_BUNDLE_STYLE
            },
            ..default()
        },
        Name::new("Ingredient Description"),
    ));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            },
            Name::new("Cures and Causes"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            size: Size::new(Val::Px(140.), Val::Px(200.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Ingredient Cures"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Cures",
                        get_info_text_style(font, 18.),
                    ));

                    let text_sections: Vec<TextSection> = ingredient
                        .cures
                        .iter()
                        .map(|s| {
                            TextSection::new(
                                format!(
                                    "{}\n",
                                    get_text_or_unknown(s, ingredient.starter, ingredient.used)
                                ),
                                get_info_text_style(font, 16.),
                            )
                        })
                        .collect();
                    parent.spawn((TextBundle::from_sections(text_sections),));
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            size: Size::new(Val::Px(140.), Val::Px(200.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Ingredient Causes"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Causes",
                        get_info_text_style(font, 18.),
                    ));
                    let text_sections: Vec<TextSection> = ingredient
                        .causes
                        .iter()
                        .map(|s| {
                            TextSection::new(
                                // causes are only known if they are used
                                format!("{}\n", get_text_or_unknown(s, false, ingredient.used)),
                                get_info_text_style(font, 16.),
                            )
                        })
                        .collect();
                    parent.spawn((TextBundle::from_sections(text_sections),));
                });
        });
}

pub fn build_default_information_text(commands: &mut ChildBuilder, font: &Handle<Font>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "This text shows when no ingredients are selected ...",
                get_info_text_style(font, 18.),
            ),
            style: INFO_TEXT_BUNDLE_STYLE,
            ..default()
        },
        Name::new("Default Info Text"),
    ));
}

fn get_text_or_unknown(side_effect: &SideEffectClass, starter: bool, used: bool) -> String {
    if starter || used {
        side_effect.to_string()
    } else {
        "??????".into()
    }
}

// ------ STYLES ------

const INFO_TEXT_BUNDLE_STYLE: Style = Style {
    max_size: Size {
        width: Val::Px(300.),
        height: Val::Undefined,
    },
    ..Style::DEFAULT
};

fn get_info_text_style(font: &Handle<Font>, size: f32) -> TextStyle {
    return TextStyle {
        font: font.clone(),
        font_size: size,
        color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
    };
}
