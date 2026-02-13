use bevy::prelude::*;
mod art;
mod player;
mod door;
mod roaming;
mod state;
mod hitbox;
use crate::door::DoorPlugin;
use crate::player::PlayerPlugin;
use crate::roaming::RoamingPlugin;
use crate::hitbox::HitBoxPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup_camera)
    .add_plugins(state::StatePlugin)
    .add_plugins(HitBoxPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(DoorPlugin)
    .add_plugins(RoamingPlugin)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
