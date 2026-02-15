use bevy::prelude::*;
use mapgen::MapBuilder;
use mapgen::filter::{
    NoiseGenerator,
    CellularAutomata,
};
use bevy_common_assets::ron::RonAssetPlugin;
use crate::hitbox::HitBox;
use crate::wall::Wall;
use crate::state::GameState;
use crate::level_entity::LevelEntity;
use crate::level_schema::LevelData;

const TILE_SIZE: f32 = 32.0;
const MAP_WIDTH: usize = 50;
const MAP_HEIGHT: usize = 50;

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct LevelDoor {
    pub leads_to: String,
}

// Resource to track current level and its data
#[derive(Resource)]
pub struct CurrentLevel {
    pub level_id: String,
    pub handle: Handle<LevelData>,
    pub loaded: bool,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            level_id: "intro".to_string(),
            handle: Handle::default(),
            loaded: false,
        }
    }
}

// Resource to store level data once loaded
#[derive(Resource, Default)]
pub struct LoadedLevelData(pub Option<LevelData>);

// Start loading a level
fn start_loading_level(
    mut current_level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
) {
    let path = format!("levels/level_01_{}.ron", current_level.level_id);
    current_level.handle = asset_server.load(&path);
    current_level.loaded = false;
    info!("Loading level: {}", path);
}

// Check if level asset is loaded, then spawn and transition
fn check_level_loaded(
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    level_assets: Res<Assets<LevelData>>,
    mut loaded_data: ResMut<LoadedLevelData>,
    mut next_state: ResMut<NextState<GameState>>,
    windows: Query<&Window>,
) {
    if current_level.loaded {
        return;
    }

    if let Some(level_data) = level_assets.get(&current_level.handle) {
        info!("Level loaded: {}", level_data.name);
        loaded_data.0 = Some(level_data.clone());
        current_level.loaded = true;

        // Spawn the level now that data is loaded
        spawn_level_from_data_internal(&mut commands, level_data, &windows);

        // If there's dialogue, go to Dialogue state, otherwise Playing
        if !level_data.dialogue.is_empty() {
            next_state.set(GameState::Dialogue);
        } else {
            next_state.set(GameState::Playing);
        }
    }
}

// Internal function to spawn level from data (called after asset is confirmed loaded)
fn spawn_level_from_data_internal(
    commands: &mut Commands,
    level_data: &LevelData,
    windows: &Query<&Window>,
) {
    info!("Spawning level: {} ({})", level_data.name, level_data.room_type);

    match level_data.room_type.as_str() {
        "square" => spawn_border_walls_internal(commands, windows),
        "cave" => spawn_cave_level(commands),
        _ => {
            warn!("Unknown room type: {}", level_data.room_type);
            spawn_border_walls_internal(commands, windows);
        }
    }

    // Spawn doors from level data
    for door_data in &level_data.doors {
        spawn_door_from_data(commands, door_data);
    }
}

fn spawn_border_walls_internal(commands: &mut Commands, windows: &Query<&Window>) {
    let Ok(window) = windows.single() else { return; };
    let width = window.width();
    let height = window.height();
    let wall_size = TILE_SIZE;

    let cols = (width / wall_size).ceil() as i32 + 1;
    let rows = (height / wall_size).ceil() as i32 + 1;

    let half_width = width / 2.0;
    let half_height = height / 2.0;

    for i in -1..=cols {
        let x = i as f32 * wall_size - half_width;
        spawn_wall_at(commands, Vec3::new(x, half_height, 0.0), wall_size);
        spawn_wall_at(commands, Vec3::new(x, -half_height, 0.0), wall_size);
    }

    for i in 0..rows {
        let y = i as f32 * wall_size - half_height;
        spawn_wall_at(commands, Vec3::new(-half_width, y, 0.0), wall_size);
        spawn_wall_at(commands, Vec3::new(half_width, y, 0.0), wall_size);
    }
}

fn spawn_cave_level(commands: &mut Commands) {
    let map = MapBuilder::new(MAP_WIDTH, MAP_HEIGHT)
        .with(NoiseGenerator::uniform())
        .with(CellularAutomata::new())
        .build();

    let offset_x = (MAP_WIDTH as f32 * TILE_SIZE) / 2.0;
    let offset_y = (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pos = Vec3::new(
                x as f32 * TILE_SIZE - offset_x,
                y as f32 * TILE_SIZE - offset_y,
                0.0,
            );

            if map.is_walkable(x, y) {
                commands.spawn((
                    Text2d::new("."),
                    TextFont { font_size: TILE_SIZE, ..default() },
                    TextColor(Color::srgb(0.3, 0.3, 0.3)),
                    Transform::from_translation(pos),
                    Floor,
                    LevelEntity,
                ));
            } else {
                spawn_wall_at(commands, pos, TILE_SIZE);
            }
        }
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
        LevelEntity,
    ));
}

fn spawn_door_from_data(commands: &mut Commands, door_data: &crate::level_schema::DoorData) {
    use crate::art::DOOR_ART;

    commands.spawn((
        Text2d::new(DOOR_ART),
        TextFont { font_size: 6.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(door_data.position.0, door_data.position.1, 1.0)),
        LevelDoor { leads_to: door_data.leads_to.clone() },
        HitBox { width: 80.0, height: 120.0 },
        LevelEntity,
    ));

    info!("Spawned door '{}' at ({}, {})", door_data.label, door_data.position.0, door_data.position.1);
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<LevelData>::new(&["ron"]))
           .init_resource::<CurrentLevel>()
           .init_resource::<LoadedLevelData>()
           .add_systems(OnEnter(GameState::StartGame), start_loading_level)
           .add_systems(Update, check_level_loaded.run_if(in_state(GameState::StartGame)));
    }
}
