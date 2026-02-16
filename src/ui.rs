use bevy::prelude::*;
use crate::player::PlayerHealth;
use crate::art::{FULL_HEART, EMPTY_HEART};

#[derive(Component)]
pub struct HealthContainer;

#[derive(Component)]
pub struct HeartDisplay {
    pub index: usize,
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