use bevy::prelude::*;
mod art;
mod helpers;
mod player;
mod door;
mod roaming;
mod state;
mod hitbox;
mod wall;
mod level;
mod level_entity;
mod level_schema;
mod camera;
mod projectile;
mod ui;
mod npc;
mod follow;
mod story_flags;
use crate::camera::CameraPlugin;
use crate::projectile::ProjectilePlugin;
use crate::door::DoorPlugin;
use crate::player::PlayerPlugin;
use crate::roaming::RoamingPlugin;
use crate::hitbox::HitBoxPlugin;
use crate::level::LevelPlugin;
use crate::follow::FollowPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window{
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(state::StatePlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(HitBoxPlugin)
    .add_plugins(LevelPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(DoorPlugin)
    .add_plugins(RoamingPlugin)
    .add_plugins(ProjectilePlugin)
    .add_plugins(FollowPlugin)
    .run();
}
