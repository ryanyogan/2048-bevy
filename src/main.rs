mod colors;

use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

const TILE_SIZE: f32 = 40.0; // Size is finite, 40px; Note: Window is 1024x748
const TILE_SPACER: f32 = 10.0;

///
/// Components (Data Entities)
///

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;

        Self {
            size,
            physical_size,
        }
    }

    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }

    fn size(&self) -> Vec2 {
        Vec2::new(self.physical_size, self.physical_size)
    }
}

#[derive(Component)]
struct Points {
    value: u32,
}

impl Points {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Component)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn new(x: &u8, y: &u8) -> Self {
        Self { x: *x, y: *y }
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
        .add_systems(
            Startup,
            (spawn_camera, spawn_board, apply_deferred, spawn_tiles).chain(),
        )
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

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(board.size()),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(tile.0),
                        board.cell_position_to_physical(tile.1),
                        1.0,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);
}

///
/// Spawn Functions
///

fn spawn_tiles(mut commands: Commands, query_board: Query<&Board>) {
    dbg!(&query_board);
    let board = query_board.single();

    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for (x, y) in starting_tiles.iter() {
        let pos = Position::new(x, y);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: colors::TILE,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(pos.x),
                    board.cell_position_to_physical(pos.y),
                    1.0,
                ),
                ..default()
            })
            .insert(Points::new(2))
            .insert(pos);
    }
}
