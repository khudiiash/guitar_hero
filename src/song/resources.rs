use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource, Default)]
pub struct Song {
   pub map: Vec<String>,
   pub bpm: u32,
   pub data: String,
   pub delay: f32,
   pub title: String,
   pub artist: String,
   pub speed: f32,
   pub beats: u32,
   pub spacing: (f32, f32),
   pub audio: Handle<AudioSource>,
}

#[derive(Resource, Default, Debug)]
pub struct SongChoice {
    pub artist: String,
    pub title: String
}