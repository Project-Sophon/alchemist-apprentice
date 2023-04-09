use bevy::prelude::*;

use crate::{assets::resources_standard::UiAssets, style::color::PALETTE_DARK_BLUE};

// ------ ENUMS, CONSTANTS ------

const DIALOGUE_BOX_POS_X: f32 = 110.0;
const DIALOGUE_BOX_POS_Y: f32 = 75.0;
const DIALOGUE_BOX_WIDTH: f32 = 371.0;
const DIALOGUE_BOX_HEIGHT: f32 = 195.0;
const DIALOGUE_BOX_PADDING: f32 = 20.0;

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct DialogueBox;

// ------ PUB FUNCTIONS ------

pub fn create_dialogue_box(
    commands: &mut ChildBuilder,
    font: &Handle<Font>,
    ui_assets: &Res<UiAssets>,
    text: &str,
) {
    commands
        .spawn((
            ImageBundle {
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
                image: UiImage::new(ui_assets.dialogue_bkg.clone()),
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
                        font: font.clone(),
                        font_size: 18.,
                        color: Color::hex(PALETTE_DARK_BLUE).unwrap(),
                    },
                )
                .with_text_alignment(TextAlignment::Left)
                .with_style(Style {
                    size: Size::new(
                        Val::Px(DIALOGUE_BOX_WIDTH - (DIALOGUE_BOX_PADDING * 2.)),
                        Val::Px(DIALOGUE_BOX_HEIGHT - (DIALOGUE_BOX_PADDING * 2.)),
                    ),
                    ..default()
                }),
            );
        });
}
