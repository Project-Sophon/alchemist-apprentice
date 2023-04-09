use bevy::prelude::*;

use crate::{
    assets::resources_standard::{CharacterAssets, GlobalAssets, UiAssets},
    game::game_phase::GamePhase,
    style::color::{PALETTE_CREAM, PALETTE_DARK_BLUE},
    ui::buttons::{
        get_menu_button_style, get_menu_button_text_style, get_normal_menu_button_color, MenuButton,
    },
    world::{
        common::{WINDOW_HEIGHT, WINDOW_WIDTH},
        despawn::despawn_entity,
        global_state::GlobalState,
    },
};

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_win.in_schedule(OnEnter(GlobalState::Win)))
            .add_system(on_lose.in_schedule(OnEnter(GlobalState::Lose)))
            .add_system(reset_state.in_set(OnUpdate(GlobalState::Win)))
            .add_system(reset_state.in_set(OnUpdate(GlobalState::Lose)))
            .add_system(despawn_entity::<EndScreen>.in_schedule(OnExit(GlobalState::Win)))
            .add_system(despawn_entity::<EndScreen>.in_schedule(OnExit(GlobalState::Lose)));
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

fn reset_state(
    interaction_query: Query<
        (&Interaction, &ResetButton),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut game_state: ResMut<NextState<GlobalState>>,
    mut game_phase: ResMut<NextState<GamePhase>>,
) {
    for (interaction, _) in &interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GlobalState::Game);
                game_phase.set(GamePhase::AilmentStatement);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn on_win(
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    character_assets: Res<CharacterAssets>,
) {
    build_end_screen(&mut commands, &global_assets, &character_assets, true)
}

fn on_lose(
    mut commands: Commands,
    global_assets: Res<GlobalAssets>,
    character_assets: Res<CharacterAssets>,
) {
    build_end_screen(&mut commands, &global_assets, &character_assets, false)
}

fn build_end_screen(
    commands: &mut Commands,
    global_assets: &Res<GlobalAssets>,
    character_assets: &Res<CharacterAssets>,
    win: bool,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Undefined,
                        bottom: Val::Undefined,
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    gap: Size::height(Val::Px(20.)),
                    size: Size::new(Val::Px(WINDOW_WIDTH.into()), Val::Px(WINDOW_HEIGHT.into())),
                    ..default()
                },
                background_color: Color::hex(PALETTE_CREAM).unwrap().into(),
                ..default()
            },
            EndScreen,
        ))
        .with_children(|parent| {
            let img = if win {
                &character_assets.alchemist_win
            } else {
                &character_assets.bjorn_dead
            };

            parent.spawn(TextBundle::from_section(
                if win { "You Win!" } else { "Game Over!" },
                TextStyle {
                    font: global_assets.font.clone(),
                    font_size: 48.,
                    color: Color::hex(PALETTE_DARK_BLUE).unwrap(),
                },
            ));

            parent.spawn(ImageBundle {
                style: Style { ..default() },
                image: UiImage::new(img.clone()),
                ..default()
            });

            parent
                .spawn((
                    ButtonBundle {
                        style: get_menu_button_style(),
                        background_color: get_normal_menu_button_color().into(),
                        ..default()
                    },
                    MenuButton,
                    ResetButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play Again",
                        get_menu_button_text_style(&global_assets.font),
                    ));
                });
        });
}
