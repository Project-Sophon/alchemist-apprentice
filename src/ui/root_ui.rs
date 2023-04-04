use bevy::prelude::*;

use crate::{
    assets::{GlobalAssets, UiAssets},
    game::state::{GlobalState, GamePhase},
};

use super::ingredients::{create_ingredients_area, spawn_ingredients};
use super::{buttons::create_panel_button, selection_display::create_selection_display};
pub struct RootUiPlugin;
impl Plugin for RootUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(root_ui_setup.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(
                spawn_ingredients
                    .in_schedule(OnEnter(GamePhase::CustomerEnter)),
            );
    }
}

struct ResetWorkbench;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Workbench;

fn root_ui_setup(
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    ui_assets: Res<UiAssets>,
) {
    let font = global_assets.font.clone();

    commands
        .spawn((
            ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(384.0)),
                    align_content: AlignContent::End,
                    align_self: AlignSelf::FlexEnd,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                image: ui_assets.workbench.clone().into(),
                ..default()
            },
            Workbench,
            Name::new("UI Root"),
        ))
        .with_children(|parent| {
            create_ingredients_area(parent);
            create_selection_display(parent);
            create_panel_button(parent, &font, "Concoct");
        });
}
