use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CurrentMusic(pub Option<Entity>);

pub fn play_sfx(
    commands: &mut Commands,
    asset_server: &AssetServer,
    name: &str,
    file_type: &str,
){
    let handle = asset_server
        .load(format!("sounds/sfx/{}.{}",name,file_type));
    commands.spawn(AudioPlayer::new(handle));
}

// Start looping music (stops previous)
pub fn play_music(
    commands: &mut Commands,
    asset_server: &AssetServer,
    current: &mut CurrentMusic,
    name: &str,
) {
    if let Some(entity) = current.0 {
        commands.entity(entity).despawn();
    }
    let handle = asset_server.load(format!("sounds/music/{}.ogg", name));
    let entity = commands.spawn((
        AudioPlayer::new(handle),
        PlaybackSettings::LOOP,
    )).id();
    current.0 = Some(entity);
}