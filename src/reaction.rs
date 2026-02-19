use bevy::prelude::*;
use crate::level::LoadedLevelData;
use crate::level_event::LevelEvent;
use crate::level_schema::{Action, DialogueLine, Trigger};
use crate::state::boss_fight::{AttackTimer, PlayerArena};
use crate::story_flags::StoryFlags;
use crate::wall::Wall;
use crate::level_entity::LevelEntity;
use crate::level::{CurrentLevel, LevelDoor};
use crate::hitbox::HitBox;
use crate::art::DOOR_ART;
use crate::state::GameState;

/// Resource for queued mid-level dialogue
#[derive(Resource, Default)]
pub struct QueuedDialogue {
    pub lines: Vec<DialogueLine>,
    pub then_state: String,
}

impl QueuedDialogue {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.then_state.clear();
    }
}

fn trigger_matches(trigger: &Trigger, event: &LevelEvent, story_flags: &StoryFlags) -> bool {
    match trigger {
        Trigger::Event(event_name) => {
            event_matches_name(event, event_name)
        }
        Trigger::EventAndFlag { event: event_name, flag, equals } => {
            if !event_matches_name(event, event_name) {
                return false;
            }
            // Check if flag matches expected value
            match story_flags.get(flag) {
                Some(current_value) => current_value == equals,
                None => false,
            }
        }
    }
}

fn event_matches_name(event: &LevelEvent, name: &str) -> bool {
    match event {
        LevelEvent::ProjectilesDone => name == "ProjectilesDone",
        LevelEvent::DialogueComplete => name == "DialogueComplete",
        LevelEvent::BossDefeated => name == "BossDefeated",
    }
}

pub fn process_reactions(
    mut events: MessageReader<LevelEvent>,
    loaded_data: Res<LoadedLevelData>,
    mut commands: Commands,
    mut story_flags: ResMut<StoryFlags>,
    mut attack_timer: ResMut<AttackTimer>,
    mut current_level: ResMut<CurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
    mut queued_dialogue: ResMut<QueuedDialogue>,
    arena_query: Query<Entity, With<PlayerArena>>,
    wall_query: Query<Entity, With<Wall>>,
) {
    for event in events.read() {
        info!("LevelEvent fired: {:?}", event);

        let Some(level_data) = &loaded_data.0 else { continue };

        // Only one reaction should fire per event - break after first match
        for reaction in &level_data.reactions {
            if trigger_matches(&reaction.trigger, event, &story_flags) {
                info!("Reaction triggered: {:?}", reaction.trigger);

                for action in &reaction.actions {
                    execute_action(
                        action,
                        &mut commands,
                        &mut story_flags,
                        &mut attack_timer,
                        &mut current_level,
                        &mut next_state,
                        &mut queued_dialogue,
                        &arena_query,
                        &wall_query,
                    );
                }
                break; // Only fire one reaction per event
            }
        }
    }
}

fn execute_action(
    action: &Action,
    commands: &mut Commands,
    story_flags: &mut ResMut<StoryFlags>,
    attack_timer: &mut ResMut<AttackTimer>,
    current_level: &mut ResMut<CurrentLevel>,
    next_state: &mut ResMut<NextState<GameState>>,
    queued_dialogue: &mut ResMut<QueuedDialogue>,
    arena_query: &Query<Entity, With<PlayerArena>>,
    wall_query: &Query<Entity, With<Wall>>,
) {
    match action {
        Action::DespawnArena => {
            info!("Executing: DespawnArena");
            // Despawn arena marker
            for entity in arena_query.iter() {
                commands.entity(entity).despawn();
            }
            // Despawn all walls (arena walls are the only walls in boss fight)
            for entity in wall_query.iter() {
                commands.entity(entity).despawn();
            }
        }

        Action::SetFlag { key, value } => {
            info!("Executing: SetFlag({} = {:?})", key, value);
            story_flags.set(key, value.clone());
        }

        Action::QueueDialogue { lines, then } => {
            info!("Executing: QueueDialogue ({} lines, then: {})", lines.len(), then);
            queued_dialogue.lines = lines.clone();
            queued_dialogue.then_state = then.clone();
            next_state.set(GameState::Dialogue);
        }

        Action::SpawnDoor { position, leads_to, label } => {
            info!("Executing: SpawnDoor at {:?} -> {}", position, leads_to);
            commands.spawn((
                Text2d::new(DOOR_ART),
                TextFont { font_size: 6.0, ..default() },
                TextColor(Color::WHITE),
                Transform::from_translation(Vec3::new(position.0, position.1, 1.0)),
                LevelDoor { leads_to: leads_to.clone() },
                HitBox { width: 80.0, height: 120.0 },
                LevelEntity,
            ));
            info!("Spawned door '{}' at ({}, {})", label, position.0, position.1);
        }

        Action::RestartProjectiles { count } => {
            info!("Executing: RestartProjectiles({})", count);
            attack_timer.projectiles_fired = 0;
            attack_timer.max_projectiles = *count;
            attack_timer.event_sent = false;
        }

        Action::TransitionToLevel { level_id } => {
            info!("Executing: TransitionToLevel({})", level_id);
            current_level.level_id = level_id.clone();
            current_level.loaded = false;
            next_state.set(GameState::LoadingNewLevel);
        }

        Action::SetNextLevel { level_id } => {
            info!("Executing: SetNextLevel({})", level_id);
            current_level.level_id = level_id.clone();
            current_level.loaded = false;
            // Don't transition - let QueueDialogue's then_state handle it
        }
    }
}
