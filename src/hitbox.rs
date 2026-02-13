use bevy::prelude::*;
use crate::player::Player;
use crate::state::GameState;

#[derive(Component)]
pub struct HitBox{
    pub width: f32,
    pub height: f32,
}

// Message fired when player touches something
#[derive(Message)]
pub struct PlayerTouchedSomething {
    pub messaging_entity: Entity,
}

fn detect_col_with_player(
    player_query: Single<(&Transform, &HitBox), With<Player>>,
    other_query: Query<(Entity, &Transform, &HitBox), Without<Player>>,
    mut messages: MessageWriter<PlayerTouchedSomething>,
){
    let (player_transform, player_hitbox) = *player_query;
    let player_pos = player_transform.translation;

    for (entity, transform, hitbox) in &other_query {
        let other_pos = transform.translation;

        // AABB overlap check
        let overlap_x = (player_pos.x - other_pos.x).abs() < (player_hitbox.width + hitbox.width) / 2.0;
        let overlap_y = (player_pos.y - other_pos.y).abs() < (player_hitbox.height + hitbox.height) / 2.0;

        if overlap_x && overlap_y {
            messages.write(PlayerTouchedSomething { messaging_entity: entity });
            info!("overlapping!");
        }
    }
}

pub struct HitBoxPlugin;

impl Plugin for HitBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerTouchedSomething>()
           .add_systems(Update, detect_col_with_player.run_if(in_state(GameState::Playing)));
    }
}