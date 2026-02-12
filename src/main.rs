use bevy::prelude::*;
mod art;
mod player;
mod door;
use crate::door::DoorPlugin;
use crate::player::PlayerPlugin;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup_camera)
    .add_plugins(PlayerPlugin)
    .add_plugins(DoorPlugin)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}