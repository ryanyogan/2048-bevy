mod colors;

use bevy::prelude::*;
use itertools::Itertools;

const TILE_SIZE: f32 = 40.0; // Size is finite, 40px; Note: Window is 1024x748
const TILE_SPACER: f32 = 10.0;

///
/// Components (Data Entities)
///

#[derive(Component)]
struct Board {
    size: u8,
}

impl Board {
    fn new(size: u8) -> Self {
        Self { size }
    }
}

///
/// App Runner
///

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_board))
        .run();
}

///
/// Setup Spawn Function
///

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);
    let physical_board_size =
        f32::from(board.size) * TILE_SIZE + f32::from(board.size + 1) * TILE_SPACER;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(Vec2::new(physical_board_size, physical_board_size)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            let offset = -physical_board_size / 2.0 + 0.5 * TILE_SIZE;

            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        offset
                            + f32::from(tile.0) * TILE_SIZE
                            + f32::from(tile.0 + 1) * TILE_SPACER,
                        offset
                            + f32::from(tile.1) * TILE_SIZE
                            + f32::from(tile.1 + 1) * TILE_SPACER,
                        1.0,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);
}
