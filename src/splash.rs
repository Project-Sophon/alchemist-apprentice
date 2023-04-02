use bevy::prelude::*;
use crate::{GameState, despawn_screen};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_splash)
            .add_system(countdown.in_set(OnUpdate(GameState::Splash)))
            .add_system(despawn_screen::<OnSplashScreen>.in_schedule(OnExit(GameState::Splash)));
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let splash_image = asset_server.load("branding/splash.png");
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::hex("F5EDE9").unwrap().into(),
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    ..default()
                },
                image: UiImage::new(splash_image),
                ..default()
            });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
