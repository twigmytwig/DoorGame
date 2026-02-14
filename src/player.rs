use bevy::prelude::*;
use crate::state::GameState;
use crate::hitbox::HitBox;
use crate::wall::Wall;

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands){
    commands.spawn((
        Text2d::new("@"),
        TextFont{
            font_size: 24.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        Player,
        HitBox{width: 24.0,height: 24.0}
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Single<(&mut Transform, &HitBox), With<Player>>,
    walls: Query<(&Transform, &HitBox), (With<Wall>, Without<Player>)>,
){
    let (ref mut player_transform, player_hitbox) = *player_query;

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft){
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight){
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp){
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown){
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO{
        let speed = 300.0;
        let delta = direction.normalize() * speed * time.delta_secs();
        let desired_pos = Vec2::new(
            player_transform.translation.x + delta.x,
            player_transform.translation.y + delta.y,
        );

        let mut blocked = false;
        for (wall_transform, wall_hitbox) in &walls {
            let overlap_x = (desired_pos.x - wall_transform.translation.x).abs()
                < (player_hitbox.width + wall_hitbox.width) / 2.0;
            let overlap_y = (desired_pos.y - wall_transform.translation.y).abs()
                < (player_hitbox.height + wall_hitbox.height) / 2.0;

            if overlap_x && overlap_y {
                blocked = true;
                break;
            }
        }

        if !blocked {
            player_transform.translation.x = desired_pos.x;
            player_transform.translation.y = desired_pos.y;
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartGame), spawn_player)
            .add_systems(Update, move_player);
    }
}
