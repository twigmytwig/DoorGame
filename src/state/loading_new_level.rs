use bevy::prelude::*;
use crate::level_entity::LevelEntity;
use crate::level::{CurrentLevel, LoadedLevelData, spawn_level_from_data_internal};
use crate::level_schema::LevelData;
use super::GameState;
use super::loading::{LoadingScreen, LoadingText};

pub fn spawn_loading_new_level_screen(mut commands: Commands){
    commands.spawn((
        LoadingScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
    )).with_children(|parent| {
        parent.spawn((
            LoadingText,
            Text::new("Loading Room..."),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

pub fn animate_loading_room(
    time: Res<Time>,
    mut query: Query<&mut Text, With<LoadingText>>,
) {
    for mut text in query.iter_mut() {
        let dots = (time.elapsed_secs() * 2.0) as usize % 4;
        **text = format!("Loading Room{}", ".".repeat(dots));
    }
}

pub fn despawn_level_entities(
    mut commands: Commands,
    query: Query<Entity, With<LevelEntity>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// Start loading the next level asset
pub fn start_loading_next_level(
    mut current_level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
) {
    let path = format!("levels/{}.ron", current_level.level_id);
    current_level.handle = asset_server.load(&path);
    current_level.loaded = false;
    info!("Loading next level: {}", path);
}

// Check if level asset is loaded, spawn it, then transition
pub fn check_new_level_ready(
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
        info!("Next level loaded: {}", level_data.name);
        loaded_data.0 = Some(level_data.clone());
        current_level.loaded = true;

        // Spawn the level entities
        spawn_level_from_data_internal(&mut commands, level_data, &windows);

        // Transition based on dialogue and level type
        if !level_data.dialogue.is_empty() {
            next_state.set(GameState::Dialogue);
        } else if level_data.room_type == "boss" {
            next_state.set(GameState::BossFight);
        } else {
            next_state.set(GameState::Playing);
        }
    }
}
