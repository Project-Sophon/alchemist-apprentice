use bevy::prelude::*;

use crate::{
    assets::resources_standard::{GlobalAssets, UiAssets},
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

#[derive(Component)]
pub struct StatusPanel;

#[derive(Component)]
pub struct NotInitialized;

pub fn build_status_panel(commands: &mut ChildBuilder, ui_assets: &Res<UiAssets>) {
    commands.spawn((
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(298.), Val::Px(300.)),
                flex_basis: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::FlexEnd,
                padding: UiRect::new(Val::Px(32.), Val::Px(0.), Val::Px(10.), Val::Undefined),
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
) {
    query.for_each(|e| {
        commands.entity(e).remove::<NotInitialized>();
        commands.entity(e).with_children(|parent| {
            render_side_effects_in_panel(parent, bjorn_status.clone(), &global_assets.font);
        });
    })
}

fn update_status_panel(
    mut commands: Commands,
    query: Query<Entity, With<StatusPanel>>,
    global_assets: Res<GlobalAssets>,
    bjorn_status: Res<BjornStatus>,
) {
    if !bjorn_status.is_changed() {
        return;
    }
    query.for_each(|e| {
        commands.entity(e).despawn_descendants();
        commands.entity(e).with_children(|parent| {
            render_side_effects_in_panel(parent, bjorn_status.clone(), &global_assets.font);
        });
    });
}

fn render_side_effects_in_panel(
    parent: &mut ChildBuilder,
    bjorn_status: BjornStatus,
    font: &Handle<Font>,
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
                    font_size: 18.,
                    color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                },
            ),
            TextSection::new(
                format!("{}\n", s.description.to_string()),
                TextStyle {
                    font: font.clone(),
                    font_size: 14.,
                    color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                },
            ),
            ]
        })
        .flatten()
        .collect();

    parent.spawn(TextBundle {
        style: Style {
            margin: UiRect {
                bottom: Val::Px(2.),
                ..default()
            },
            ..default()
        },
        text: Text {
            sections: vec![
                TextSection::new(
                    format!("Toxicity: {}\n", bjorn_status.toxicity),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                    },
                ),
                TextSection::new(
                    format!("Ailments:\n"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::hex(PALETTE_DARK_BLUE).unwrap().into(),
                    },
                ),
            ],
            ..default()
        },
        ..default()
    });

    parent.spawn(TextBundle {
        style: Style {
            margin: UiRect {
                bottom: Val::Px(5.),
                ..default()
            },
            size: Size {
                width: Val::Px(270.),
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
