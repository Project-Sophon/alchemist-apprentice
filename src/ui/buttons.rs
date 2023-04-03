use bevy::prelude::*;

pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PanelButton>()
            .add_system(panel_button_interactions);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PanelButton;

const PANEL_NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PANEL_HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PANEL_PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn panel_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
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

pub fn create_panel_button(parent: &mut ChildBuilder, font: &Handle<Font>, text: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: PANEL_NORMAL_BUTTON.into(),
            ..default()
        })
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
