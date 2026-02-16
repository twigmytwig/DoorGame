use bevy::prelude::*;
use crate::hitbox::HitBox;
use crate::state::GameState;

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
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
        HitBox { width: 24.0, height: 24.0 },
    ));
}

// Test spawn - remove later
fn test_spawn_projectile(mut commands: Commands) {
    spawn_projectile_at(
        &mut commands,
        Vec3::new(300.0, 0.0, 5.0),
        Vec2::new(-150.0, 0.0),
        "{=}",
    );
}

pub struct ProjectilePlugin;
//TODO: ITS PLAYING STATE RN BUT WILL BE BOSSFIGHT STATE
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BossFight), test_spawn_projectile)
           .add_systems(Update, move_projectiles.run_if(in_state(GameState::BossFight)));
    }
}
