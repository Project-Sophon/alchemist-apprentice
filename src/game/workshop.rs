use bevy::prelude::*;

use crate::assets::resources_standard::{CharacterAssets, WorkshopAssets};

pub struct WorkshopPlugin;
impl Plugin for WorkshopPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Workshop;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct BjornCharacter;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AlchemistCharacter;

// ------ SYSTEMS ------

pub fn build_workshop(
    commands: &mut Commands,
    workshop_assets: &Res<WorkshopAssets>,
    character_assets: &Res<CharacterAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: workshop_assets.workshop_bkg.clone(),
            transform: Transform::from_xyz(-154., 224., 1.),
            ..default()
        },
        Workshop,
        Name::new("Workshop"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: character_assets.bjorn.clone(),
            transform: Transform::from_xyz(26., 192., 2.),
            ..default()
        },
        BjornCharacter,
        Name::new("Bjorn Character"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: character_assets.alchemist.clone(),
            transform: Transform::from_xyz(156., 192., 2.),
            ..default()
        },
        AlchemistCharacter,
        Name::new("Alchemist Character"),
    ));
}
