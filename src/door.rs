use bevy::prelude::*;
use crate::art::DOOR_ART;
use crate::state::GameState;
use crate::hitbox::{HitBox, PlayerTouchedSomething};

#[derive(Component)]
struct Door;

fn spawn_door(mut commands: Commands){
    commands.spawn((
        Text2d::new(DOOR_ART),
        TextFont{
            font_size: 6.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        Door,
        HitBox { width: 80.0, height: 120.0 },
    ));
}

fn handle_door_touch(
    mut messages: MessageReader<PlayerTouchedSomething>,
    doors: Query<(), With<Door>>,
) {
    for message in messages.read() {
        if doors.get(message.messaging_entity).is_ok() {
            info!("Door collision!");
        }
    }
}

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartGame), spawn_door)
           .add_systems(Update, handle_door_touch.run_if(in_state(GameState::Playing)));
    }
}