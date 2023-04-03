use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCamera>()
            .add_startup_system(setup_camera);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GameCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        GameCamera,
        Name::new("Main Camera"),
    ));
}
