use bevy::prelude::*;

use crate::buttons::ButtonAction;

#[derive(Component)]
pub struct Play;

pub fn setup_play(mut commands: Commands, asset_server: Res<AssetServer>) {
    // top banner
    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                display: Display::Flex,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(Play)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle { ..default() })
                .with_children(|parent| {
                    // text
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(ButtonAction::MENU)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load("fonts/ComicSansMS3.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                        },
                    ));
                });
        });
    // main window
}

pub fn teardown_play(mut commands: Commands, mut node_query: Query<Entity, With<Play>>) {
    for play_entity in &mut node_query {
        commands.entity(play_entity).despawn_recursive();
    }
}
