use bevy::prelude::*;
use crate::art::DOOR_ART;

#[derive(Component)]
struct Door;

fn spawn_door(mut commands: Commands){
    commands.spawn((
        Text2d::new(DOOR_ART),
        TextFont{
            font_size: 12.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
        Door
    ));
}

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_door);
    }
}