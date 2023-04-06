use bevy::prelude::*;

use crate::style::color::PALETTE_PURPLE;

pub struct PotionPlugin;
impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PotionPanel>();
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PotionPanel;

// ------ SYSTEMS ------

pub fn build_potion_panel(commands: &mut ChildBuilder) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(33.33), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            PotionPanel,
            Name::new("Potion Panel"),
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(208.), Val::Px(200.)),
                    ..default()
                },
                background_color: Color::hex(PALETTE_PURPLE).unwrap().into(),
                ..default()
            });
        })
        .id()
}
