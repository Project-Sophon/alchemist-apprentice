use bevy::prelude::*;

use crate::style::color::{PALETTE_CREAM};

pub struct StatusPlugin;
impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct StatusPanel;

pub fn build_status_panel(commands: &mut ChildBuilder) {
    commands.spawn((
        NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                size: Size {
                    width: Val::Px(290.),
                    height: Val::Percent(100.),
                },
                ..default()
            },
            background_color: Color::hex(PALETTE_CREAM).unwrap().into(),
            ..default()
        },
        Name::new("Status Panel"),
        StatusPanel,
    ));
}
