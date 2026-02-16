use bevy::prelude::*;
use crate::player::Player;
use crate::state::GameState;

#[derive(Component)]
pub struct Follow {
    pub speed: f32,   // Smoothness factor (0.1 = slow, 0.5 = snappy)
    pub distance: f32, // Stay this far from player
}

fn follow(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut followers: Query<(&mut Transform, &Follow), Without<Player>>,
) {
    let Ok(player_transform) = player_query.single() else { return };
    let player_pos = player_transform.translation;

    for (mut transform, follow) in &mut followers {
        let to_player = player_pos - transform.translation;
        let direction = to_player.normalize_or_zero();

        // Target position is `distance` away from player
        let target = player_pos - direction * follow.distance;

        // Lerp toward target
        transform.translation = transform.translation.lerp(target, follow.speed * time.delta_secs());
    }
}

pub struct FollowPlugin;

impl Plugin for FollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow.run_if(
            in_state(GameState::Playing).or(in_state(GameState::BossFight))
        ));
    }
}
