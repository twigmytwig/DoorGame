use bevy::prelude::*;
mod art;
mod player;
mod door;
mod roaming;
mod state;
mod hitbox;
mod wall;
mod level;
mod camera;
use crate::camera::CameraPlugin;
use crate::door::DoorPlugin;
use crate::player::PlayerPlugin;
use crate::roaming::RoamingPlugin;
use crate::hitbox::HitBoxPlugin;
use crate::wall::WallPlugin;
use crate::level::LevelPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(state::StatePlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(HitBoxPlugin)
    .add_plugins(WallPlugin)
    .add_plugins(LevelPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(DoorPlugin)
    .add_plugins(RoamingPlugin)
    .run();
}
