use bevy::prelude::*;
use crate::level::LoadedLevelData;
use crate::story_flags::StoryFlags;

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
) {
    // Get first dialogue line that can be spoken, or placeholder if none
    let (speaker, text) = if let Some(level_data) = &loaded_data.0 {
        // Find first line from a speaker who can speak
        loop {
            if dialogue_state.current_line >= level_data.dialogue.len() {
                break ("".to_string(), "".to_string());
            }
            let line = &level_data.dialogue[dialogue_state.current_line];
            if story_flags.can_speaker_speak(&line.speaker) {
                break (line.speaker.clone(), line.text.clone());
            }
            info!("Skipping initial dialogue from '{}' (not present)", line.speaker);
            dialogue_state.current_line += 1;
        }
    } else {
        ("".to_string(), "No dialogue loaded".to_string())
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
) {
    if !input.just_pressed(KeyCode::Space) && !input.just_pressed(KeyCode::Enter) {
        return;
    }

    let Some(level_data) = &loaded_data.0 else { return };

    dialogue_state.current_line += 1;

    // Check if we've exhausted all dialogue
    if dialogue_state.current_line >= level_data.dialogue.len() {
        if level_data.room_type == "boss" {
            info!("Dialogue finished, transitioning to BossFight");
            next_state.set(crate::state::GameState::BossFight);
        } else {
            info!("Dialogue finished, transitioning to Playing");
            next_state.set(crate::state::GameState::Playing);
        }
        return;
    }

    // Skip dialogue from NPCs that can't speak (dead, traded, not present)
    loop {
        // Check if we've exhausted all dialogue while skipping
        if dialogue_state.current_line >= level_data.dialogue.len() {
            if level_data.room_type == "boss" {
                info!("Dialogue finished (after skipping), transitioning to BossFight");
                next_state.set(crate::state::GameState::BossFight);
            } else {
                info!("Dialogue finished (after skipping), transitioning to Playing");
                next_state.set(crate::state::GameState::Playing);
            }
            return;
        }

        let line = &level_data.dialogue[dialogue_state.current_line];

        if story_flags.can_speaker_speak(&line.speaker) {
            break; // This speaker can speak, show their line
        }

        // Speaker can't speak, skip to next line
        info!("Skipping dialogue from '{}' (not present)", line.speaker);
        dialogue_state.current_line += 1;
    }

    // Update the text to show the valid line
    let line = &level_data.dialogue[dialogue_state.current_line];

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