use bevy::prelude::*;
use crate::hitbox::HitBox;
use crate::level_entity::LevelEntity;
use crate::state::GameState;

#[derive(Component)]
pub struct Wall;

fn spawn_wall(mut commands: Commands){
    commands.spawn((
        Text2d::new("||"),
        TextFont{
            font_size:12.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3 { x: (110.0), y: (10.0), z: (0.0) }),
        HitBox{width: 12.0, height: 12.0},
        LevelEntity,
        Wall
    ));
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartGame), spawn_wall);
    }
}