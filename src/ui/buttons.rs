use bevy::prelude::*;

use crate::style::color::{DISABLE_NORMAL_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

use super::common::{DisableUiElement, DisabledUiElement, EnableUiElement};

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BasicButton>()
            .add_system(button_interactions)
            .add_system(enable_buttons)
            .add_system(disable_buttons);
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct BasicButton;

// ------ SYSTEMS ------

fn button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            Without<DisabledUiElement>,
        ),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn enable_buttons(
    mut panel_buttons_query: Query<
        &mut BackgroundColor,
        (With<Button>, With<EnableUiElement>, With<DisabledUiElement>),
    >,
) {
    for mut color in &mut panel_buttons_query {
        *color = NORMAL_BUTTON.into();
    }
}

fn disable_buttons(
    mut panel_buttons_query: Query<
        &mut BackgroundColor,
        (
            With<Button>,
            With<DisableUiElement>,
            Without<DisabledUiElement>,
        ),
    >,
) {
    for mut color in &mut panel_buttons_query {
        *color = DISABLE_NORMAL_BUTTON.into();
    }
}

// ------ PUB FUNCTIONS ------

pub fn create_basic_button(parent: &mut ChildBuilder, font: &Handle<Font>, text: &str) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: DISABLE_NORMAL_BUTTON.into(),
                ..default()
            },
            BasicButton,
            DisabledUiElement,
            Name::new("Panel Button"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
