use bevy::prelude::*;
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin};

use super::common::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCamera>()
            .add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::rgb(0.1, 0.1, 0.1),
            })
            .add_startup_system(setup_camera);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GameCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        PixelCameraBundle::from_resolution(WINDOW_WIDTH.into(), WINDOW_HEIGHT.into()),
        GameCamera,
        Name::new("Main Camera"),
    ));
}
