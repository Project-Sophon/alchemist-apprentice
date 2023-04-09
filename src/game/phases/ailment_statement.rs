use bevy::prelude::*;

use crate::{
    assets::resources_standard::{GlobalAssets, UiAssets},
    game::{
        dialogue::{create_dialogue_box, DialogueBox},
        game_phase::GamePhase,
        level::LevelContainer,
    },
    world::despawn::despawn_entity,
};

pub struct AilmentStatementPlugin;
impl Plugin for AilmentStatementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ailment_statement.in_schedule(OnEnter(GamePhase::AilmentStatement)))
            .add_system(on_ailment_timer.in_set(OnUpdate(GamePhase::AilmentStatement)))
            .add_system(
                despawn_entity::<DialogueBox>.in_schedule(OnExit(GamePhase::AilmentStatement)),
            );
    }
}

// ------ COMPONENTS ------

#[derive(Resource, Deref, DerefMut)]
struct AilmentStatementTimer(Timer);

// ------ SYSTEMS ------

fn spawn_ailment_statement(
    mut commands: Commands,
    level_container: Query<Entity, With<LevelContainer>>,
    global_assets: Res<GlobalAssets>,
    ui_assets: Res<UiAssets>,
) {
    if let Ok(parent) = level_container.get_single() {
        let intro_text = "Hello Assistant! I am Bjorn Bjornson and am in desperate need of alchemical assistance, surely you can help me? Your master is always going on about some rule of 3, how hard can it really be?";

        commands.entity(parent).with_children(|parent| {
            create_dialogue_box(parent, &global_assets.font, &ui_assets, intro_text);
        });
    }

    commands.insert_resource(AilmentStatementTimer(Timer::from_seconds(
        5.0,
        TimerMode::Once,
    )));
}

fn on_ailment_timer(
    mut game_state: ResMut<NextState<GamePhase>>,
    time: Res<Time>,
    mut timer: ResMut<AilmentStatementTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GamePhase::PotionAssembly);
    }
}
