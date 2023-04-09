use std::default;

use bevy::prelude::*;

use crate::{
    assets::resources_standard::{GlobalAssets, UiAssets},
    game::{
        bjorn::BjornStatus,
        dialogue::{create_dialogue_box, DialogueBox},
        game_phase::GamePhase,
        level::LevelContainer,
    },
    world::{despawn::despawn_entity, global_state::GlobalState},
};

pub struct AilmentStatementPlugin;
impl Plugin for AilmentStatementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AilmentStatementState>()
            .add_system(reset_state.in_schedule(OnEnter(GlobalState::Game)))
            .add_system(spawn_ailment_statement.in_schedule(OnEnter(GamePhase::AilmentStatement)))
            .add_system(on_ailment_timer.in_set(OnUpdate(GamePhase::AilmentStatement)))
            .add_systems(
                (on_ailment_statement_exit, despawn_entity::<DialogueBox>)
                    .in_schedule(OnExit(GamePhase::AilmentStatement)),
            );
    }
}

// ------ RESOURCES ------

#[derive(Resource)]
pub struct AilmentStatementState {
    pub previous_side_effect_count: usize,
    pub intro_statement_read: bool,
}

impl Default for AilmentStatementState {
    fn default() -> AilmentStatementState {
        AilmentStatementState {
            previous_side_effect_count: 0,
            intro_statement_read: false,
        }
    }
}

// ------ CONSTSANTS ------

const AILMENT_STATEMENT_DURATION: f32 = 5.;

// ------ COMPONENTS ------

#[derive(Resource, Deref, DerefMut)]
struct AilmentStatementTimer(Timer);

// ------ SYSTEMS ------

fn spawn_ailment_statement(
    mut commands: Commands,
    level_container: Query<Entity, With<LevelContainer>>,
    global_assets: Res<GlobalAssets>,
    ui_assets: Res<UiAssets>,
    ailment_statement_state: Res<AilmentStatementState>,
    bjorn_status: Res<BjornStatus>,
) {
    if let Ok(parent) = level_container.get_single() {
        let text = get_text_for_state(&ailment_statement_state, &bjorn_status);

        commands.entity(parent).with_children(|parent| {
            create_dialogue_box(parent, &global_assets.font, &ui_assets, text.as_str());
        });
    }

    commands.insert_resource(AilmentStatementTimer(Timer::from_seconds(
        AILMENT_STATEMENT_DURATION,
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

fn get_text_for_state(state: &AilmentStatementState, bjorn_status: &BjornStatus) -> String {
    if !state.intro_statement_read {
        return "Hello Assistant! I am Bjorn Bjornson and am in desperate need of alchemical assistance!\n\nCan you help me?".into();
    }

    let first_string = if bjorn_status.side_effects.len() < state.previous_side_effect_count {
        "I feel better than before. I'll soon be good as new!"
    } else if bjorn_status.side_effects.len() == state.previous_side_effect_count {
        "My old ailments are gone but now I have new ones in their place!"
    } else {
        "My ailments have increased, hopefully you get the hang of this soon!"
    };

    let second_string = match bjorn_status.toxicity {
        0 => "Feeling pretty good about this, lets keep going!",
        1 => "Feel a little strange though, probably nothing right?",
        2 => "Is there any alcohol in these potions? I feel like I just drank an entire cask of mead!",
        3 => "I think I may need to lie down for a bit, I'm starting to regret this course of action ...",
        4 => "If I make it through this, I'm going to sleep for a week, I feel terrible!",
        _ => "I think another potion might kill me, hopefully you've got it right this time ..."
    };

    return format!("{} {}", first_string, second_string);
}

fn on_ailment_statement_exit(
    mut ailment_statement_state: ResMut<AilmentStatementState>,
    bjorn_status: Res<BjornStatus>,
) {
    // set intro statement to read
    ailment_statement_state.intro_statement_read = true;

    // update state with latest from bjorn_status
    ailment_statement_state.previous_side_effect_count = bjorn_status.side_effects.len();
}

fn reset_state(mut ailment_statement_state: ResMut<AilmentStatementState>) {
    ailment_statement_state.intro_statement_read = false;
    ailment_statement_state.previous_side_effect_count = 0;
}
