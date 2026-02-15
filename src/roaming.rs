use bevy::prelude::*;
use crate::state::GameState;

#[derive(Component)]
pub struct Roam{
    pub speed: f32
}

//When time passes, we choose random direction, and move in that direction

fn roam(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Roam)>,
){
    for (mut transform, roam_component) in &mut query{
        let mut direction = Vec2::ZERO;
        direction.x -= 1.0;
        if direction != Vec2::ZERO{
            let delta = direction.normalize() * roam_component.speed * time.delta_secs();
            transform.translation.x += delta.x;
        }
    }
}

pub struct RoamingPlugin;

impl Plugin for RoamingPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, roam.run_if(in_state(GameState::Playing)));
    }
}