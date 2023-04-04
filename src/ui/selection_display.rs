use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SelectionDisplay;

pub fn create_selection_display(parent: &mut ChildBuilder) {
    parent.spawn((
        NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(300.),
                    height: Val::Px(350.),
                },
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        },
        SelectionDisplay,
    ));
}
