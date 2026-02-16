use bevy::prelude::*;
use crate::hitbox::HitBox;
use crate::level_entity::LevelEntity;
use crate::player::PlayerHealth;
use crate::state::GameState;
use crate::hitbox::PlayerTouchedSomething;


#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
}

fn handle_projectile_touch_player(
    mut messages: MessageReader<PlayerTouchedSomething>,
    mut commands: Commands,
    projectiles: Query<(), With<Projectile>>,
    mut health: ResMut<PlayerHealth>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for message in messages.read() {
        // Try to get the LevelDoor component from the touched entity
        if projectiles.get(message.messaging_entity).is_ok() {
            info!("Projectile hit player!");
            commands.entity(message.messaging_entity).despawn();
            health.current -= 1;// TODO: HARD CODED
            if health.current == 0{
                next_state.set(GameState::Defeat)
            }
        }
    }
}

fn move_projectiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Projectile)>,
) {
    for (mut transform, projectile) in &mut query {
        transform.translation.x += projectile.velocity.x * time.delta_secs();
        transform.translation.y += projectile.velocity.y * time.delta_secs();
    }
}

pub fn spawn_projectile_at(
    commands: &mut Commands,
    pos: Vec3,
    velocity: Vec2,
    shape: &str,
) {
    commands.spawn((
        Text2d::new(shape),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        Transform::from_translation(pos),
        Projectile { velocity },
        LevelEntity,
        HitBox { width: 24.0, height: 24.0 },
    ));
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_projectiles,
            handle_projectile_touch_player,
        ).run_if(in_state(GameState::BossFight)));
    }
}
