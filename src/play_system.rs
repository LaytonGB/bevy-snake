use bevy::prelude::*;

use crate::{buttons::ButtonAction, window::*};

const HEADER_HEIGHT: f32 = 80.0;
const HEADER_OFFSET: f32 = -HEADER_HEIGHT / 2.0;
const GRID_SQUARES: u32 = 9;
const GRID_SQUARE_SIZE: f32 = 500. / GRID_SQUARES as f32;

#[derive(Component)]
pub struct Playing;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn setup_play(mut commands: Commands, asset_server: Res<AssetServer>) {
    // top banner
    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(HEADER_HEIGHT),
                display: Display::Flex,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(Playing)
        .with_children(|parent| {
            // empty left button
            parent
                .spawn(ButtonBundle { ..default() })
                .with_children(|_parent| {});

            // right button
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

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, HEADER_OFFSET, 0.0),
                    scale: Vec3::splat(GRID_SQUARE_SIZE),
                    ..default()
                },
                ..default()
            },
            SnakeHead {
                direction: Direction::Up,
            },
            Position::new(0, 0),
        ))
        .insert(Playing);
}

pub fn draw_grid(mut gizmos: Gizmos) {
    // main window
    for i in 1..GRID_SQUARES {
        let i = i as f32 - GRID_SQUARES as f32 / 2.0;
        gizmos.line_2d(
            Vec2::new(GRID_SQUARE_SIZE * i, -WINDOW_HEIGHT + HEADER_OFFSET),
            Vec2::new(GRID_SQUARE_SIZE * i, WINDOW_HEIGHT + HEADER_OFFSET),
            Color::DARK_GRAY,
        );
    }
    for i in 1..GRID_SQUARES {
        let i = i as f32 - GRID_SQUARES as f32 / 2.0;
        gizmos.line_2d(
            Vec2::new(-WINDOW_WIDTH, GRID_SQUARE_SIZE * i + HEADER_OFFSET),
            Vec2::new(WINDOW_WIDTH, GRID_SQUARE_SIZE * i + HEADER_OFFSET),
            Color::DARK_GRAY,
        );
    }
}

pub fn teardown_play(mut commands: Commands, mut node_query: Query<Entity, With<Playing>>) {
    for play_entity in &mut node_query {
        commands.entity(play_entity).despawn_recursive();
    }
}
