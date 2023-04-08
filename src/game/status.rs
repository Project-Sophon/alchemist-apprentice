use bevy::prelude::*;

use crate::assets::resources_standard::UiAssets;

pub struct StatusPlugin;
impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct StatusPanel;

pub fn build_status_panel(commands: &mut ChildBuilder, ui_assets: &Res<UiAssets>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(274.), Val::Px(300.)),
                flex_basis: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::FlexEnd,
                margin: UiRect::new(Val::Undefined, Val::Px(10.), Val::Px(10.), Val::Undefined),
                ..default()
            },
            image: ui_assets.status_panel_bkg.clone().into(),
            ..default()
        },
        Name::new("Status Panel"),
        StatusPanel,
    ));
}
