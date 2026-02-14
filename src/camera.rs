use bevy::prelude::*;
use crate::player::Player;
use crate::state::GameState;

#[derive(Component)]
pub struct GameCamera;

fn spawn_camera(mut commands: Commands){
    commands.spawn((
        Camera2d,
        GameCamera,
    ));
}

fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
){
    let Ok(player_transform) = player_query.single() else { return; };
    let Ok(mut camera_transform) = camera_query.single_mut() else { return; };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
           .add_systems(Update, follow_player.run_if(in_state(GameState::Playing)));
    }
}
