use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::state::GamePhase;

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(customer_enter.in_schedule(OnEnter(GamePhase::CustomerEnter)));
    }
}

fn customer_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(150., 150., 0.)),
        ..default()
    });
}
