use crate::{
    assets::assets_game_data::{Ingredient, SideEffect, SideEffectClass},
    game::{
        bjorn::{give_bjorn_concoction, BjornStatus},
        game_phase::GamePhase,
        ingredients::update_ingredients_used,
        potion::PotionMix,
    },
    style::color::PALETTE_DARK_CREAM,
    ui::disable_ui::EnableUiElement,
    world::global_state::GlobalState,
};
use bevy::prelude::*;
use core::fmt;
use std::collections::HashSet;

pub struct ConcoctPlugin;
impl Plugin for ConcoctPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_concoct.in_schedule(OnEnter(GamePhase::Concoct)));
    }
}

// ------ ENUMS, CONSTANTS ------

const MAX_TOXICITY: i32 = 5;

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ConcoctAction;

#[derive(Component, Default)]
pub struct Concoction {
    pub toxicity: i32,
    pub cures: HashSet<SideEffectClass>,
    pub causes: HashSet<SideEffectClass>,
}

impl fmt::Display for Concoction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Concoction with toxicity: {:?}, cures: {:?}, and causes: {:?}",
            self.toxicity, self.cures, self.causes
        )
    }
}

// ------ SYSTEMS ------

fn on_concoct(
    mut global_state: ResMut<NextState<GlobalState>>,
    mut game_phase: ResMut<NextState<GamePhase>>,
    potion_mix: Res<PotionMix>,
    mut ingredients: ResMut<Assets<Ingredient>>,
    mut bjorn_status: ResMut<BjornStatus>,
    side_effects: Res<Assets<SideEffect>>,
) {
    let concoction = concoct(potion_mix.clone(), &ingredients);
    info!("{}", concoction.to_string());

    // Update ingredients used in concoction
    update_ingredients_used(&mut ingredients, &potion_mix.ingredients);

    // Get Bjorn's toxicity and number of side effects after giving him the concoction
    let (toxicity, num_side_effects) =
        give_bjorn_concoction(concoction, &mut bjorn_status, &side_effects);

    // Advance game appropriately based on Bjorn's toxicity and side effect count
    if toxicity > MAX_TOXICITY {
        global_state.set(GlobalState::Lose);
    } else if num_side_effects == 0 {
        global_state.set(GlobalState::Win);
    } else {
        game_phase.set(GamePhase::AilmentStatement);
    }
}

pub fn spawn_concoct_action(
    commands: &mut ChildBuilder,
    background_img: &Handle<Image>,
    font: &Handle<Font>,
) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(132.), Val::Px(36.)),
                    ..default()
                },
                image: UiImage::new(background_img.clone()),
                ..default()
            },
            ConcoctAction,
            EnableUiElement,
            Name::new("Concoct Action"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "CONCOCT",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.,
                        color: Color::hex(PALETTE_DARK_CREAM).unwrap().into(),
                    },
                ),
                style: Style {
                    margin: UiRect::new(Val::Px(34.), Val::Undefined, Val::Px(11.), Val::Undefined),
                    ..default()
                },
                ..default()
            });
        });
}

fn concoct(potion_mix: PotionMix, ingredients: &ResMut<Assets<Ingredient>>) -> Concoction {
    let mut cures: HashSet<SideEffectClass> = HashSet::new();
    let mut causes: HashSet<SideEffectClass> = HashSet::new();
    let mut toxicity: i32 = 0;

    // First build cures of the concoction
    for ingredient in potion_mix.ingredients.clone() {
        match ingredient {
            Some(handle) => {
                let i = ingredients.get(&handle).unwrap().clone();
                toxicity += i.toxicity;
                for c in i.cures.clone() {
                    cures.insert(c);
                }
            }
            None => {}
        }
    }

    // Now that we have gathered all cures, figure out which side effects are still relevant
    for ingredient in potion_mix.ingredients.clone() {
        match ingredient {
            Some(handle) => {
                let i = ingredients.get(&handle).unwrap().clone();
                for c in i.causes.clone() {
                    if !cures.contains(&c) {
                        causes.insert(c);
                    }
                }
            }
            None => {}
        }
    }

    // return Concoction
    Concoction {
        toxicity: toxicity,
        cures: cures,
        causes: causes,
    }
}

// ------ TESTS ------

#[test]
fn test_concoction() {
    let mut app = App::new();

    app.add_plugin(crate::world::global_state::GlobalStatePlugin);
    app.add_plugin(bevy::prelude::AssetPlugin { ..default() });
    app.add_plugin(crate::assets::AssetPlugin);

    let mut next_state = app
        .world
        .resource_mut::<NextState<crate::world::global_state::GlobalState>>();
    next_state.set(crate::world::global_state::GlobalState::Game);

    print!("Iterating through asserts to see if loaded... \n");
    let ingredients = app.world.resource::<Assets<Ingredient>>();
    for (_, ingredient) in ingredients.iter() {
        print!("{:?}", ingredient.name);
    }
}
