use bevy::prelude::{Resource, Handle};
use bevy_kira_audio::AudioSource;

#[derive(Resource, Default)]
pub struct Base {
    pub y: f32,
    pub x: [f32; 6]
}

#[derive(Resource, Default)]
pub struct MissedSounds {
    pub handles: Vec<Handle<AudioSource>>
}