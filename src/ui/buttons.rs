use bevy::prelude::*;

use super::common::{DisableUiElement, DisabledUiElement, EnableUiElement};

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PanelButton>()
            .add_system(panel_button_interactions)
            .add_system(enable_panel_buttons)
            .add_system(disable_panel_button);
    }
}

// ------ ENUMS, CONSTANTS ------

const DISABLE_NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const PANEL_NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PANEL_HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PANEL_PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PanelButton;

// ------ SYSTEMS ------

fn panel_button_interactions(
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
                *color = PANEL_PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = PANEL_HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = PANEL_NORMAL_BUTTON.into();
            }
        }
    }
}

fn enable_panel_buttons(
    mut panel_buttons_query: Query<
        &mut BackgroundColor,
        (With<Button>, With<EnableUiElement>, With<DisabledUiElement>),
    >,
) {
    for mut color in &mut panel_buttons_query {
        *color = PANEL_NORMAL_BUTTON.into();
    }
}

fn disable_panel_button(
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

pub fn create_panel_button(parent: &mut ChildBuilder, font: &Handle<Font>, text: &str) {
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
            PanelButton,
            DisabledUiElement,
            Name::new("Panel Button")
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
