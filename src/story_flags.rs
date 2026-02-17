use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct StoryFlags {
    flags: HashMap<String, FlagValue>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum FlagValue {
    Bool(bool),
    Text(String),
    Number(i32),
}


impl StoryFlags {
    pub fn set(&mut self, key: &str, value: FlagValue) {
        self.flags.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&FlagValue> {
        self.flags.get(key)
    }

    pub fn get_text(&self, key: &str) -> Option<&str> {
        match self.flags.get(key) {
            Some(FlagValue::Text(s)) => Some(s),
            _ => None,
        }
    }

    pub fn get_number(&self, key: &str) -> Option<i32> {
        match self.flags.get(key) {
            Some(FlagValue::Number(n)) => Some(*n),
            _ => None,
        }
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.flags.get(key) {
            Some(FlagValue::Bool(b)) => Some(*b),
            _ => None,
        }
    }

    /// Check if a speaker can speak (is present).
    /// Returns true if no flag exists (narrators, "---", etc. always speak).
    pub fn can_speaker_speak(&self, speaker: &str) -> bool {
        let present_key = format!("{}_present", speaker.to_lowercase());
        self.get_bool(&present_key).unwrap_or(true)
    }
}