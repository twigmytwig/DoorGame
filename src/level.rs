use bevy::prelude::*;
use mapgen::MapBuilder;
use mapgen::filter::{
    NoiseGenerator,
    CellularAutomata,
};
use crate::hitbox::HitBox;
use crate::wall::Wall;
use crate::state::GameState;

const TILE_SIZE: f32 = 32.0;
const MAP_WIDTH: usize = 50;
const MAP_HEIGHT: usize = 50;

#[derive(Component)]
pub struct Floor;

fn spawn_level(mut commands: Commands) {
    // Generate the map using BSP rooms + corridors
    let map = MapBuilder::new(MAP_WIDTH, MAP_HEIGHT)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .build();

    // Calculate offset to center the map
    let offset_x = (MAP_WIDTH as f32 * TILE_SIZE) / 2.0;
    let offset_y = (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0;

    // Spawn tiles based on the generated map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pos = Vec3::new(
                x as f32 * TILE_SIZE - offset_x,
                y as f32 * TILE_SIZE - offset_y,
                0.0,
            );

            if map.is_walkable(x, y) {
                // Floor tile
                commands.spawn((
                    Text2d::new("."),
                    TextFont {
                        font_size: TILE_SIZE,
                        ..default()
                    },
                    TextColor(Color::srgb(0.3, 0.3, 0.3)),
                    Transform::from_translation(pos),
                    Floor,
                ));
            } else {
                // Wall tile
                commands.spawn((
                    Text2d::new("#"),
                    TextFont {
                        font_size: TILE_SIZE,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    Transform::from_translation(pos),
                    Wall,
                    HitBox { width: TILE_SIZE, height: TILE_SIZE },
                ));
            }
        }
    }
}

pub fn spawn_border_walls(
    mut commands: Commands,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else { return; };
    let width = window.width();
    let height = window.height();
    let wall_size = TILE_SIZE;

    // How many walls fit along each edge
    let cols = (width / wall_size).ceil() as i32 + 1;
    let rows = (height / wall_size).ceil() as i32 + 1;

    let half_width = width / 2.0;
    let half_height = height / 2.0;

    for i in -1..=cols {
        let x = i as f32 * wall_size - half_width;

        // Top edge
        spawn_wall_at(&mut commands, Vec3::new(x, half_height, 0.0), wall_size);
        // Bottom edge
        spawn_wall_at(&mut commands, Vec3::new(x, -half_height, 0.0), wall_size);
    }

    for i in 0..rows {
        let y = i as f32 * wall_size - half_height;

        // Left edge
        spawn_wall_at(&mut commands, Vec3::new(-half_width, y, 0.0), wall_size);
        // Right edge
        spawn_wall_at(&mut commands, Vec3::new(half_width, y, 0.0), wall_size);
    }
}

fn spawn_wall_at(commands: &mut Commands, pos: Vec3, size: f32) {
    commands.spawn((
        Text2d::new("#"),
        TextFont { font_size: size, ..default() },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        Transform::from_translation(pos),
        Wall,
        HitBox { width: size, height: size },
    ));
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartGame), spawn_level);
    }
}

pub struct BorderWallsPlugin;

impl Plugin for BorderWallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartGame), spawn_border_walls);
    }
}
