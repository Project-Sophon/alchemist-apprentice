use crate::{
    assets::resources_standard::{AudioAssets, GlobalAssets},
    game::game_phase::GamePhase,
    style::color::PALETTE_CREAM,
    ui::buttons::{
        get_menu_button_style, get_menu_button_text_style, get_normal_menu_button_color, MenuButton,
    },
    world::{
        common::{WINDOW_HEIGHT, WINDOW_WIDTH},
        despawn::despawn_entity,
    },
    GlobalState,
};
use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::{prelude::*, Audio};

// ------ ENUMS, CONSTANTS ------

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}

// ------ PLUGINS ------

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_system(menu_setup.in_schedule(OnEnter(GlobalState::Menu)))
            .add_system(despawn_entity::<MainMenu>.in_schedule(OnExit(GlobalState::Menu)))
            .add_systems(
                (main_menu_setup, start_background_audio).in_schedule(OnEnter(MenuState::Main)),
            )
            .add_system(menu_action.in_set(OnUpdate(GlobalState::Menu)))
            .add_system(stop_background_audio.in_schedule(OnExit(MenuState::Main)));
    }
}

// ------ COMPONENTS ------

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}

// ------ SYSTEMS ------

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn start_background_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_assets.menu.clone()).looped();
}

fn stop_background_audio(audio: Res<Audio>) {
    audio.stop();
}

fn main_menu_setup(mut commands: Commands, global_assets: Res<GlobalAssets>) {
    let font = global_assets.font.clone();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(WINDOW_WIDTH.into()), Val::Px(WINDOW_HEIGHT.into())),
                    align_self: AlignSelf::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Undefined,
                        bottom: Val::Undefined,
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::hex(PALETTE_CREAM).unwrap().into(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::hex(PALETTE_CREAM).unwrap().into(),
                    ..default()
                })
                .with_children(|parent| {
                    let main_menu_image = global_assets.main_menu_banner.clone();
                    parent.spawn(ImageBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(15.0)),
                            margin: UiRect {
                                bottom: Val::Px(50.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: main_menu_image.into(),
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
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                get_menu_button_text_style(&font),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_menu_button_style(),
                                background_color: get_normal_menu_button_color().into(),
                                ..default()
                            },
                            MenuButton,
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                get_menu_button_text_style(&font),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_menu_button_style(),
                                background_color: get_normal_menu_button_color().into(),
                                ..default()
                            },
                            MenuButton,
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit",
                                get_menu_button_text_style(&font),
                            ));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GlobalState>>,
    mut game_phase: ResMut<NextState<GamePhase>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            audio.play(audio_assets.click.clone());
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(GlobalState::Game);
                    menu_state.set(MenuState::Disabled);
                    game_phase.set(GamePhase::PotionAssembly);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
            }
        }
    }
}
