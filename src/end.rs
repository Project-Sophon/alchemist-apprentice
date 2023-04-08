use bevy::prelude::*;

use crate::{
    assets::resources_standard::{GlobalAssets, UiAssets},
    style::color::PALETTE_CREAM,
    world::{
        common::{WINDOW_HEIGHT, WINDOW_WIDTH},
        global_state::GlobalState,
    },
};

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_win.in_schedule(OnEnter(GlobalState::Win)))
            .add_system(on_lose.in_schedule(OnEnter(GlobalState::Lose)));
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct EndScreen;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct ResetButton;

// ------ SYSTEMS ------

fn on_win(mut commands: Commands, global_assets: Res<GlobalAssets>, ui_assets: Res<UiAssets>) {
    build_end_screen(&mut commands, &global_assets, &ui_assets, true)
}

fn on_lose(mut commands: Commands, global_assets: Res<GlobalAssets>, ui_assets: Res<UiAssets>) {
    build_end_screen(&mut commands, &global_assets, &ui_assets, false)
}

fn build_end_screen(
    commands: &mut Commands,
    global_assets: &Res<GlobalAssets>,
    ui_assets: &Res<UiAssets>,
    win: bool,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Undefined,
                        bottom: Val::Undefined,
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(WINDOW_WIDTH.into()), Val::Px(WINDOW_HEIGHT.into())),
                    ..default()
                },
                background_color: Color::hex(PALETTE_CREAM).unwrap().into(),
                ..default()
            },
            EndScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style { ..default() },
                // todo: different sprite for win vs lose
                image: UiImage::new(ui_assets.end_screen.clone()),
                ..default()
            });
        });
}
