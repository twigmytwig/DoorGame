use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CurrentMusic {
    pub entity: Option<Entity>,
    pub track: Option<String>,
}

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

// Start looping music (stops previous if different track)
pub fn play_music(
    commands: &mut Commands,
    asset_server: &AssetServer,
    current: &mut CurrentMusic,
    name: &str,
) {
    // Skip if same track already playing
    if current.track.as_deref() == Some(name) {
        return;
    }

    // Stop previous music
    if let Some(entity) = current.entity {
        commands.entity(entity).despawn();
    }

    // Start new music
    let handle = asset_server.load(format!("sounds/music/{}.mp3", name));
    let entity = commands.spawn((
        AudioPlayer::new(handle),
        PlaybackSettings::LOOP,
    )).id();

    current.entity = Some(entity);
    current.track = Some(name.to_string());
}

// Stop music for silent levels
pub fn stop_music(commands: &mut Commands, current: &mut CurrentMusic) {
    if let Some(entity) = current.entity {
        commands.entity(entity).despawn();
    }
    current.entity = None;
    current.track = None;
}
