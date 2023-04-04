use bevy::prelude::*;

use crate::{assets::standard_assets::{GlobalAssets, UiAssets}, game::state::GlobalState};

use super::buttons::create_panel_button;
pub struct RootUiPlugin;
impl Plugin for RootUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(root_ui_setup.in_schedule(OnEnter(GlobalState::Game)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Workbench;

fn root_ui_setup(mut commands: Commands, global_assets: Res<GlobalAssets>, ui_assets: Res<UiAssets>) {
    let font = global_assets.font.clone();

    commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(384.0)),
                    align_content: AlignContent::End,
                    align_self: AlignSelf::FlexEnd,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                image: ui_assets.workbench.clone().into(),
                ..default()
            },
            Workbench,
            Name::new("UI Root"),
        )).with_children(|parent| {
            create_panel_button(parent, &font, "Base Ingredients");
            create_panel_button(parent, &font, "Processes");
            create_panel_button(parent, &font, "Concoct");
        });
}
