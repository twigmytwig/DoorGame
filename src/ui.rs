use bevy::prelude::*;
use crate::follow::Follow;
use crate::npc::Npc;
use crate::player::PlayerHealth;
use crate::art::{FULL_HEART, EMPTY_HEART};
use crate::story_flags::StoryFlags;

#[derive(Component)]
pub struct HealthContainer;

#[derive(Component)]
pub struct HeartDisplay {
    pub index: usize,
}

#[derive(Component)]
pub struct FollowerHealthContainer {
    pub npc_name: String,
}

#[derive(Component)]
pub struct FollowerHeartDisplay {
    pub npc_name: String,
    pub index: usize,
}

pub fn spawn_follower_health_ui(
    mut commands: Commands,
    story_flags: Res<StoryFlags>,
    followers: Query<&Npc, With<Follow>>,
){
    let starting_pos_value = 40.0;
    let mut num_in_party = 2; // Start after player's row

    for npc in &followers {
        let name_lower = npc.name.to_lowercase();
        let health = story_flags.get_number(&format!("{}_health", name_lower)).unwrap_or(0);
        let max_health = story_flags.get_number(&format!("{}_max_health", name_lower)).unwrap_or(0);

        commands.spawn((
            FollowerHealthContainer { npc_name: name_lower.clone() },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(starting_pos_value * num_in_party as f32),
                left: Val::Px(20.0), //hardcoded in players. Should fix
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|parent| {
            for i in 0..max_health as usize {
                let heart_art = if (i as i32) < health {
                    FULL_HEART
                } else {
                    EMPTY_HEART
                };

                parent.spawn((
                    FollowerHeartDisplay {
                        npc_name: name_lower.clone(),
                        index: i,
                    },
                    Text::new(heart_art),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            }
        });
        num_in_party += 1;
    }
}

pub fn update_follower_health_ui(
    story_flags: Res<StoryFlags>,
    mut hearts: Query<(&FollowerHeartDisplay, &mut Text)>,
) {
    for (heart, mut text) in &mut hearts {
        let health_key = format!("{}_health", heart.npc_name);
        let current_health = story_flags.get_number(&health_key).unwrap_or(0);

        if (heart.index as i32) < current_health {
            **text = FULL_HEART.to_string();
        } else {
            **text = EMPTY_HEART.to_string();
        }
    }
}

pub fn despawn_follower_health_ui(
    mut commands: Commands,
    query: Query<Entity, With<FollowerHealthContainer>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_health_ui(mut commands: Commands, player_health: Res<PlayerHealth>) {
    // Container in top-left corner
    commands.spawn((
        HealthContainer,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        },
    )).with_children(|parent| {
        // Spawn hearts based on max health
        for i in 0..player_health.max as usize {
            let heart_art = if (i as i8) < player_health.current {
                FULL_HEART
            } else {
                EMPTY_HEART
            };

            parent.spawn((
                HeartDisplay { index: i },
                Text::new(heart_art),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
            ));
        }
    });
}

pub fn update_health_ui(
    player_health: Res<PlayerHealth>,
    mut hearts: Query<(&HeartDisplay, &mut Text)>,
) {
    for (heart, mut text) in &mut hearts {
        if (heart.index as i8) < player_health.current {
            **text = FULL_HEART.to_string();
        } else {
            **text = EMPTY_HEART.to_string();
        }
    }
}

pub fn despawn_health_ui(mut commands: Commands, query: Query<Entity, With<HealthContainer>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}