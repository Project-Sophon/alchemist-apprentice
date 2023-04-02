use bevy::prelude::*;

use crate::game::state::GlobalState;
pub struct RootUiPlugin;
impl Plugin for RootUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(root_ui_setup.in_schedule(OnEnter(GlobalState::Game)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct UiRoot {}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct UiLeft {} // todo: rename

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct UiRight {} // todo: rename

fn root_ui_setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(384.0)),
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                },
                ..default()
            },
            UiRoot {},
            Name::new("UI Root"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(800.0), Val::Percent(100.0)),
                        ..default()
                    },
                    // temp placeholder style
                    background_color: Color::rgb(0.45, 0.45, 0.45).into(),
                    ..default()
                },
                UiLeft {},
                Name::new("UI Left"),
            ));

            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(324.0), Val::Percent(100.0)),
                        ..default()
                    },
                    // temp placeholder style
                    background_color: Color::rgb(0.65, 0.65, 0.5).into(),
                    ..default()
                },
                UiRight {},
                Name::new("UI Right"),
            ));
        });
}