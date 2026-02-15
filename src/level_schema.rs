use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub id: String,
    pub name: String,
    pub room_type: String,
    pub player_start: (f32, f32),

    #[serde(default)]
    pub dialogue: Vec<DialogueLine>,

    pub doors: Vec<DoorData>,

    #[serde(default)]
    pub boss: Option<String>,

    #[serde(default)]
    pub items: Vec<ItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorData {
    pub position: (f32, f32),
    pub leads_to: String,
    pub label: String,
    pub locked: bool,
    #[serde(default)]
    pub key_required: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub item_type: String,
    pub position: (f32, f32),
}