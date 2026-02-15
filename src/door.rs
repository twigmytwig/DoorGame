use bevy::prelude::*;
use crate::state::GameState;
use crate::hitbox::PlayerTouchedSomething;
use crate::level::{LevelDoor, CurrentLevel};

fn handle_door_touch(
    mut messages: MessageReader<PlayerTouchedSomething>,
    doors: Query<&LevelDoor>,
    mut current_level: ResMut<CurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for message in messages.read() {
        // Try to get the LevelDoor component from the touched entity
        if let Ok(door) = doors.get(message.messaging_entity) {
            info!("Door hit! Loading level: {}", door.leads_to);

            // Update which level to load next
            current_level.level_id = door.leads_to.clone();
            current_level.loaded = false;

            next_state.set(GameState::LoadingNewLevel);
        }
    }
}

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        // Doors are spawned by level.rs from RON data
        // This plugin just handles the touch interaction
        app.add_systems(Update, handle_door_touch.run_if(in_state(GameState::Playing)));
    }
}