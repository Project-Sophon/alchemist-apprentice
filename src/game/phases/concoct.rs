use crate::{
    assets::{
        assets_game_data::{Ingredient, SideEffectClass, Symptom},
        resources_standard::UiAssets,
    },
    game::{
        bjorn::{give_bjorn_concoction, BjornStatus},
        potion::PotionMix,
    },
    style::color::PALETTE_DARK_BLUE,
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

pub fn concoct_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiImage),
        (With<ConcoctAction>, With<EnableUiElement>),
    >,
    potion_mix: Res<PotionMix>,
    ingredients: Res<Assets<Ingredient>>,
    ui_assets: Res<UiAssets>,
    mut bjorn_status: ResMut<BjornStatus>,
    symptoms: Res<Assets<Symptom>>,
    buttons: Res<Input<MouseButton>>,
) {
    for (entity, interaction, mut ui_image) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if buttons.just_pressed(MouseButton::Left) {
                    let concoction = concoct(potion_mix.clone(), &ingredients);
                    info!("{}", concoction.to_string());
                    ui_image.texture = ui_assets.concoct_button_click.clone();
                    give_bjorn_concoction(concoction, &mut bjorn_status, &symptoms)
                }
            }
            Interaction::Hovered => {
                ui_image.texture = ui_assets.concoct_button_hover.clone();
            }
            Interaction::None => {
                ui_image.texture = ui_assets.concoct_button_normal.clone();
            }
        }
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
                        color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
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

pub fn concoct(potion_mix: PotionMix, ingredients: &Res<Assets<Ingredient>>) -> Concoction {
    let mut cures: HashSet<SideEffectClass> = HashSet::new();
    let mut causes: HashSet<SideEffectClass> = HashSet::new();
    let mut toxicity: i32 = 0;

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

    info!("Current potion: {:?}", cures);

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
