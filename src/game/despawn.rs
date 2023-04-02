use bevy::prelude::*;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Despawn>()
            .add_system(despawn.in_base_set(CoreSet::PostUpdate));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Despawn;

fn despawn(mut commands: Commands, entities: Query<Entity, With<Despawn>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ------ PUBLIC GENERIC SYSTEMS ------

pub fn despawn_entity<T: Component>(
    to_despawn: Query<Entity, (With<T>, Without<Despawn>)>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).insert(Despawn);
    }
}
