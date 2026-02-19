use bevy::prelude::*;

#[derive(Message, Clone, Debug)]
pub enum LevelEvent {
    ProjectilesDone,
    DialogueComplete,
    BossDefeated,
}
