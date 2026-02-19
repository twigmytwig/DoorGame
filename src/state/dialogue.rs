use bevy::prelude::*;
use crate::level::LoadedLevelData;
use crate::story_flags::StoryFlags;
use crate::reaction::QueuedDialogue;
use crate::level_schema::DialogueLine;

#[derive(Component)]
pub struct DialoguePanel;

#[derive(Component)]
pub struct DialogueSpeakerText;

#[derive(Component)]
pub struct DialogueBodyText;

// Tracks which line of dialogue we're on
#[derive(Resource, Default)]
pub struct DialogueState {
    pub current_line: usize,
}

pub fn spawn_dialogue_panel(
    mut commands: Commands,
    loaded_data: Res<LoadedLevelData>,
    story_flags: Res<StoryFlags>,
    mut dialogue_state: ResMut<DialogueState>,
    queued_dialogue: Res<QueuedDialogue>,
) {
    // Get dialogue lines - check QueuedDialogue first, fall back to LoadedLevelData
    let dialogue_lines: Vec<DialogueLine> = if !queued_dialogue.is_empty() {
        queued_dialogue.lines.clone()
    } else if let Some(level_data) = &loaded_data.0 {
        level_data.dialogue.clone()
    } else {
        Vec::new()
    };

    // Get first dialogue line that can be spoken, or placeholder if none
    let (speaker, text) = {
        // Find first line from a speaker who can speak
        loop {
            if dialogue_state.current_line >= dialogue_lines.len() {
                break ("".to_string(), "".to_string());
            }
            let line = &dialogue_lines[dialogue_state.current_line];
            if story_flags.can_speaker_speak(&line.speaker) {
                break (line.speaker.clone(), line.text.clone());
            }
            info!("Skipping initial dialogue from '{}' (not present)", line.speaker);
            dialogue_state.current_line += 1;
        }
    };

    commands.spawn((
        DialoguePanel,
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(25.0),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(5.0),
            left: Val::Percent(10.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
    )).with_children(|parent| {
        // Speaker name
        parent.spawn((
            DialogueSpeakerText,
            Text::new(speaker),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.2)),
        ));
        // Dialogue text
        parent.spawn((
            DialogueBodyText,
            Text::new(text),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

pub fn advance_dialogue(
    input: Res<ButtonInput<KeyCode>>,
    loaded_data: Res<LoadedLevelData>,
    mut dialogue_state: ResMut<DialogueState>,
    mut speaker_query: Query<&mut Text, (With<DialogueSpeakerText>, Without<DialogueBodyText>)>,
    mut body_query: Query<&mut Text, (With<DialogueBodyText>, Without<DialogueSpeakerText>)>,
    mut next_state: ResMut<NextState<crate::state::GameState>>,
    story_flags: Res<StoryFlags>,
    queued_dialogue: Res<QueuedDialogue>,
) {
    if !input.just_pressed(KeyCode::Space) && !input.just_pressed(KeyCode::Enter) {
        return;
    }

    // Get dialogue lines - check QueuedDialogue first, fall back to LoadedLevelData
    let dialogue_lines: Vec<DialogueLine> = if !queued_dialogue.is_empty() {
        queued_dialogue.lines.clone()
    } else if let Some(level_data) = &loaded_data.0 {
        level_data.dialogue.clone()
    } else {
        return;
    };

    // Determine next state based on queued dialogue or level data
    let get_next_state = || -> crate::state::GameState {
        if !queued_dialogue.is_empty() {
            // Use the then_state from queued dialogue
            match queued_dialogue.then_state.as_str() {
                "Playing" => crate::state::GameState::Playing,
                "BossFight" => crate::state::GameState::BossFight,
                "LoadingNewLevel" => crate::state::GameState::LoadingNewLevel,
                "Dialogue" => crate::state::GameState::Dialogue,
                _ => {
                    warn!("Unknown then_state: {}, defaulting to Playing", queued_dialogue.then_state);
                    crate::state::GameState::Playing
                }
            }
        } else if let Some(level_data) = &loaded_data.0 {
            if level_data.room_type == "boss" {
                crate::state::GameState::BossFight
            } else {
                crate::state::GameState::Playing
            }
        } else {
            crate::state::GameState::Playing
        }
    };

    dialogue_state.current_line += 1;

    // Check if we've exhausted all dialogue
    if dialogue_state.current_line >= dialogue_lines.len() {
        let target_state = get_next_state();
        info!("Dialogue finished, transitioning to {:?}", target_state);
        next_state.set(target_state);
        return;
    }

    // Skip dialogue from NPCs that can't speak (dead, traded, not present)
    loop {
        // Check if we've exhausted all dialogue while skipping
        if dialogue_state.current_line >= dialogue_lines.len() {
            let target_state = get_next_state();
            info!("Dialogue finished (after skipping), transitioning to {:?}", target_state);
            next_state.set(target_state);
            return;
        }

        let line = &dialogue_lines[dialogue_state.current_line];

        if story_flags.can_speaker_speak(&line.speaker) {
            break; // This speaker can speak, show their line
        }

        // Speaker can't speak, skip to next line
        info!("Skipping dialogue from '{}' (not present)", line.speaker);
        dialogue_state.current_line += 1;
    }

    // Update the text to show the valid line
    let line = &dialogue_lines[dialogue_state.current_line];

    for mut text in speaker_query.iter_mut() {
        **text = line.speaker.clone();
    }
    for mut text in body_query.iter_mut() {
        **text = line.text.clone();
    }
}

pub fn reset_dialogue_state(mut dialogue_state: ResMut<DialogueState>) {
    dialogue_state.current_line = 0;
}

pub fn despawn_dialogue_panel(
    mut commands: Commands,
    query: Query<Entity, With<DialoguePanel>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    info!("Dialogue panel despawned");
}

pub fn clear_queued_dialogue(mut queued_dialogue: ResMut<QueuedDialogue>) {
    if !queued_dialogue.is_empty() {
        info!("Clearing queued dialogue");
        queued_dialogue.clear();
    }
}