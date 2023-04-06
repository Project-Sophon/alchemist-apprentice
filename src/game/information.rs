use bevy::prelude::*;

use crate::style::color::PALETTE_PURPLE;
pub struct InformationPlugin;
impl Plugin for InformationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InformationPanel>();
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct InformationPanel;

// ------ SYSTEMS ------

pub fn build_information_panel(commands: &mut ChildBuilder) -> Entity {
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
