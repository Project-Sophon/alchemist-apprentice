use bevy::prelude::*;

use crate::{
    assets::resources_standard::{GlobalAssets, ToxAssets, UiAssets},
    style::color::PALETTE_DARK_BLUE,
    world::global_state::GlobalState,
};

use super::bjorn::BjornStatus;

pub struct StatusPlugin;
impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (initial_status_panel_values, update_status_panel).in_set(OnUpdate(GlobalState::Game)),
        );
    }
}

// ------ COMPONENTS ------

#[derive(Component)]
pub struct StatusPanel;

#[derive(Component)]
pub struct NotInitialized;

// ------ SYSTEMS ------

pub fn build_status_panel(commands: &mut ChildBuilder, ui_assets: &Res<UiAssets>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(298.), Val::Px(300.)),
                flex_basis: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::FlexEnd,
                padding: UiRect::new(Val::Px(33.), Val::Undefined, Val::Px(16.), Val::Undefined),
                margin: UiRect::new(Val::Undefined, Val::Px(10.), Val::Px(10.), Val::Undefined),
                ..default()
            },
            image: ui_assets.status_panel_bkg.clone().into(),
            ..default()
        },
        Name::new("Status Panel"),
        StatusPanel,
        NotInitialized,
    ));
}

fn initial_status_panel_values(
    mut commands: Commands,
    query: Query<Entity, (With<StatusPanel>, With<NotInitialized>)>,
    global_assets: Res<GlobalAssets>,
    bjorn_status: Res<BjornStatus>,
    tox_assets: Res<ToxAssets>,
) {
    query.for_each(|e| {
        commands.entity(e).remove::<NotInitialized>();
        commands.entity(e).with_children(|parent| {
            render_side_effects_in_panel(
                parent,
                bjorn_status.clone(),
                &global_assets.font,
                &tox_assets,
            );
        });
    })
}

fn update_status_panel(
    mut commands: Commands,
    query: Query<Entity, With<StatusPanel>>,
    global_assets: Res<GlobalAssets>,
    bjorn_status: Res<BjornStatus>,
    tox_assets: Res<ToxAssets>,
) {
    if !bjorn_status.is_changed() {
        return;
    }
    query.for_each(|e| {
        commands.entity(e).despawn_descendants();
        commands.entity(e).with_children(|parent| {
            render_side_effects_in_panel(
                parent,
                bjorn_status.clone(),
                &global_assets.font,
                &tox_assets,
            );
        });
    });
}

fn render_side_effects_in_panel(
    parent: &mut ChildBuilder,
    bjorn_status: BjornStatus,
    font: &Handle<Font>,
    tox_assets: &Res<ToxAssets>,
) {
    let text_sections: Vec<TextSection> = bjorn_status
        .side_effects
        .iter()
        .map(|s| {
            vec![
                TextSection::new(
                    format!("- {}\n", s.name.to_string()),
                    TextStyle {
                        font: font.clone(),
                        font_size: 14.,
                        color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                    },
                ),
                TextSection::new(
                    format!("{}\n", s.description.to_string()),
                    TextStyle {
                        font: font.clone(),
                        font_size: 12.,
                        color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                    },
                ),
            ]
        })
        .flatten()
        .collect();

    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                margin: UiRect {
                    bottom: Val::Px(8.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Toxicity:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.,
                            color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                        },
                    )],
                    ..default()
                },
                style: Style {
                    margin: UiRect::right(Val::Px(6.)),
                    ..default()
                },
                ..default()
            });

            parent.spawn(ImageBundle {
                image: get_tox_asset(bjorn_status.toxicity, tox_assets).into(),
                ..default()
            });
        });

    parent.spawn(TextBundle {
        style: Style {
            margin: UiRect {
                bottom: Val::Px(6.),
                ..default()
            },
            ..default()
        },
        text: Text {
            sections: vec![TextSection::new(
                format!("Ailments:\n"),
                TextStyle {
                    font: font.clone(),
                    font_size: 16.,
                    color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                },
            )],
            ..default()
        },
        ..default()
    });

    parent.spawn(TextBundle {
        style: Style {
            size: Size {
                width: Val::Px(246.),
                ..default()
            },
            ..default()
        },
        text: Text {
            sections: text_sections,
            ..default()
        },
        ..default()
    });
}

fn get_tox_asset(tox: i32, tox_assets: &Res<ToxAssets>) -> Handle<Image> {
    match tox {
        0 => tox_assets.tox_0.clone(),
        1 => tox_assets.tox_1.clone(),
        2 => tox_assets.tox_2.clone(),
        3 => tox_assets.tox_3.clone(),
        4 => tox_assets.tox_4.clone(),
        _ => tox_assets.tox_5.clone(),
    }
}
