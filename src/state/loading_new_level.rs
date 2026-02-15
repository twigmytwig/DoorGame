use bevy::prelude::*;
use crate::level_entity::LevelEntity;

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
){
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
