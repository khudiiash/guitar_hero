use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource, Default)]
pub struct MetronomeTimer(pub Timer);

#[derive(Resource, Default)]
pub struct SongDelayTimer(pub Timer);

#[derive(Resource, Default)]
pub struct MetronomeAudio {
  pub audio: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct IsTimerSet(pub bool);

#[derive(Resource)]
pub struct Ticks {
  pub value: u32,
}