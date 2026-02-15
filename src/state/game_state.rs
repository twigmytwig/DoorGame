use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    StartGame,
    Playing,
    LoadingNewLevel,
    Dialogue,
    Paused,
}