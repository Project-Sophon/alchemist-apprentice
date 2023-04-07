use crate::{
    assets::assets_game_data::{Ingredient, SymptomClass},
    game::potion::PotionMix,
    ui::disable_ui::EnableUiElement,
    world::global_state::GlobalState,
};
use bevy::prelude::*;
use core::fmt;
use std::collections::HashSet;

pub struct ConcoctPlugin;
impl Plugin for ConcoctPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(concoct_interaction.in_set(OnUpdate(GlobalState::Game)));
    }
}

// ------ COMPONENTS ------

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ConcoctAction;

#[derive(Component, Default)]
pub struct Concoction {
    pub toxicity: i32,
    pub cures: HashSet<SymptomClass>,
    pub causes: HashSet<SymptomClass>,
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

pub fn concoct_interaction(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction, &mut ConcoctAction), With<EnableUiElement>>,
    potion_mix: Res<PotionMix>,
    ingredients: Res<Assets<Ingredient>>,
) {
    for (entity, interaction, _) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let concoction = _concoct(potion_mix.clone(), &ingredients);
                info!("{}", concoction.to_string());
                commands.entity(entity).remove::<EnableUiElement>();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn spawn_concoct_action(commands: &mut ChildBuilder, icon: &Handle<Image>) {
    commands.spawn((
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(64.), Val::Percent(32.)),
                ..default()
            },
            image: UiImage::new(icon.clone()),
            ..default()
        },
        ConcoctAction,
        EnableUiElement,
        Name::new("Concoct Action"),
    ));
}

pub fn _concoct(potion_mix: PotionMix, ingredients: &Res<Assets<Ingredient>>) -> Concoction {
    let mut cures: HashSet<SymptomClass> = HashSet::new();
    let mut causes: HashSet<SymptomClass> = HashSet::new();
    let mut toxicity: i32 = 0;

    for ingredient in potion_mix.ingredients {
        match ingredient {
            Some(handle) => {
                let i = ingredients.get(&handle).unwrap().clone();
                toxicity += i.toxicity;
                for c in i.cures.clone() {
                    cures.insert(c);
                }
                for c in i.causes.clone() {
                    causes.insert(c);
                }
            }
            None => {
                // do nothing
            }
        }
    }

    Concoction {
        toxicity: toxicity,
        cures: cures,
        causes: causes,
    }
}

// ------ TESTS ------

#[test]
fn test_conconction() {
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
