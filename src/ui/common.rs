use bevy::prelude::*;

pub struct CommonUiPlugin;
impl Plugin for CommonUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DisableUiElement>()
            .register_type::<DisabledUiElement>()
            .register_type::<EnableUiElement>()
            .add_system(enable_ui_elements)
            .add_system(disable_ui_elements);
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct DisableUiElement;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct DisabledUiElement;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct EnableUiElement;

// ------ SYSTEMS ------

fn enable_ui_elements(
    mut commands: Commands,
    enable_ui_query: Query<Entity, (With<Button>, With<EnableUiElement>, With<DisabledUiElement>)>,
) {
    for entity in &enable_ui_query {
        commands.entity(entity).remove::<EnableUiElement>();
        commands.entity(entity).remove::<DisabledUiElement>();
    }
}

fn disable_ui_elements(
    mut commands: Commands,
    disable_ui_query: Query<
        Entity,
        (
            With<Button>,
            With<DisableUiElement>,
            Without<DisabledUiElement>,
        ),
    >,
) {
    for entity in &disable_ui_query {
        commands.entity(entity).remove::<DisableUiElement>();
        commands.entity(entity).insert(DisabledUiElement);
    }
}
