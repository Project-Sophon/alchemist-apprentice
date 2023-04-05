use crate::{
    assets::resources_standard::GlobalAssets, style::color::GAME_BACKGROUND_COLOR,
    world::despawn::despawn_entity, GlobalState,
};
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_splash.in_schedule(OnEnter(GlobalState::Splash)))
            .add_system(countdown.in_set(OnUpdate(GlobalState::Splash)))
            .add_system(despawn_entity::<SplashScreen>.in_schedule(OnExit(GlobalState::Splash)));
    }
}

#[derive(Component)]
struct SplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setup_splash(mut commands: Commands, global_assets: Res<GlobalAssets>) {
    let splash_image = global_assets.splash.clone();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::hex(GAME_BACKGROUND_COLOR).unwrap().into(),
                ..default()
            },
            SplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style { ..default() },
                image: UiImage::new(splash_image),
                ..default()
            });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GlobalState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GlobalState::Menu);
    }
}
