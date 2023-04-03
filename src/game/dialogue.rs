use bevy::prelude::*;

// ------ ENUMS, CONSTANTS ------

const DIALOGUE_BOX_POS_X: f32 = 80.0;
const DIALOGUE_BOX_POS_Y: f32 = 80.0;
const DIALOGUE_BOX_WIDTH: f32 = 500.0;
const DIALOGUE_BOX_HEIGHT: f32 = 280.0;
const DIALOGUE_BOX_PADDING: f32 = 20.0;

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct DialogueBox;

// ------ PUB FUNCTIONS ------

pub fn create_dialogue_box(commands: &mut Commands, font: Handle<Font>, text: &str) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(DIALOGUE_BOX_POS_X),
                        right: Val::Px(0.),
                        top: Val::Px(DIALOGUE_BOX_POS_Y),
                        bottom: Val::Px(0.),
                    },
                    size: Size::new(Val::Px(DIALOGUE_BOX_WIDTH), Val::Px(DIALOGUE_BOX_HEIGHT)),
                    padding: UiRect::all(Val::Px(DIALOGUE_BOX_PADDING)),
                    ..default()
                },
                background_color: Color::rgb(0.25, 0.25, 0.75).into(),
                ..default()
            },
            DialogueBox,
            Name::new("Dialogue Box"),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font,
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Left)
                .with_style(Style {
                    size: Size::new(Val::Px(DIALOGUE_BOX_WIDTH), Val::Px(DIALOGUE_BOX_HEIGHT)),
                    ..default()
                }),
            );
        });
}
