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

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartGame), spawn_level);
    }
}
