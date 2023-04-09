use bevy::prelude::*;

use crate::{
    style::color::{PALETTE_BLUE, PALETTE_DARK_BLUE, PALETTE_DARK_CREAM, PALETTE_DARK_PURPLE},
    world::global_state::GlobalState,
};

pub struct ButtonsPlugin;
impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MenuButton>()
            .register_type::<SelectedMenuButton>()
            .add_system(button_system.in_set(OnUpdate(GlobalState::Menu)))
            .add_system(button_system.in_set(OnUpdate(GlobalState::Win)))
            .add_system(button_system.in_set(OnUpdate(GlobalState::Lose)));
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct MenuButton;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SelectedMenuButton;

// ------ SYSTEMS ------

pub fn get_normal_menu_button_color() -> Color {
    return Color::hex(PALETTE_DARK_BLUE).unwrap();
}

pub fn get_hovered_menu_button_color() -> Color {
    return Color::hex(PALETTE_BLUE).unwrap();
}

pub fn get_pressed_menu_button_color() -> Color {
    return Color::hex(PALETTE_DARK_PURPLE).unwrap();
}

pub fn get_hovered_pressed_menu_button_color() -> Color {
    return Color::hex(PALETTE_DARK_PURPLE).unwrap();
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SelectedMenuButton>,
        ),
        (Changed<Interaction>, With<MenuButton>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => {
                get_pressed_menu_button_color().into()
            }
            (Interaction::Hovered, Some(_)) => get_hovered_pressed_menu_button_color().into(),
            (Interaction::Hovered, None) => get_hovered_menu_button_color().into(),
            (Interaction::None, None) => get_normal_menu_button_color().into(),
        }
    }
}

pub fn get_menu_button_style() -> Style {
    Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn get_menu_button_text_style(font: &Handle<Font>) -> TextStyle {
    TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::hex(PALETTE_DARK_CREAM).unwrap(),
    }
}
