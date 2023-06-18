use crate::{Song, START_TICKS};
use bevy::{prelude::*};
use bevy_kira_audio::prelude::*;


pub struct Metronome;

#[derive(Resource, Default)]
pub struct MetronomeTimer(Timer);

#[derive(Resource, Default)]
pub struct SongDelayTimer(Timer);

#[derive(Resource, Default)]
pub struct MetronomeAudio {
  audio: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct IsTimerSet(bool);

#[derive(Resource)]
pub struct Ticks {
  pub value: u32,
}

impl Metronome {
  
  pub fn set_song_delay_timer(
    mut timer: ResMut<SongDelayTimer>,
    song: Res<Song>,
  ) {
    // calculate 4 beats before starting
    timer.0 = Timer::from_seconds(song.delay, TimerMode::Once);
  }
  
  pub fn load_audio(
    asset_server: Res<AssetServer>,
    mut metronome_audio: ResMut<MetronomeAudio>,
  ) {
    metronome_audio.audio = asset_server.load("sfx/metronome.ogg");
  }

  pub fn set_metronome_timer(
    mut timer: ResMut<MetronomeTimer>,
    time: Res<Time>,
    song: Res<Song>,
    mut is_timer_set: ResMut<IsTimerSet>,
    mut song_delay_timer: ResMut<SongDelayTimer>,
  ) {
    if !song_delay_timer.0.finished() || is_timer_set.0 {
      song_delay_timer.0.tick(time.delta());
      return;
    }

    let duration_value = 60.0 as f32 / song.bpm as f32;
    timer.0 = Timer::from_seconds(duration_value, TimerMode::Repeating);
    is_timer_set.0 = true;
  }
  
  pub fn play_metronome(
    mut timer: ResMut<MetronomeTimer>,
    time: Res<Time>,
    audio: Res<Audio>,
    metronome_audio: Res<MetronomeAudio>,
    start_delay_timer: Res<SongDelayTimer>,
    mut ticks: ResMut<Ticks>,
  ) {
    if ticks.value < START_TICKS && timer.0.tick(time.delta()).just_finished() && start_delay_timer.0.finished() {
        ticks.value += 1;
        audio.play(metronome_audio.audio.clone());
    }
  }

}

impl Plugin for Metronome {

  fn build(&self, app: &mut App) {
    app
      .insert_resource(SongDelayTimer::default())
      .insert_resource(MetronomeAudio::default())
      .insert_resource(MetronomeTimer::default())
      .insert_resource(IsTimerSet(false))
      .insert_resource(Ticks { value: 0 })
      .add_startup_systems((
        Self::load_audio,
        Self::set_song_delay_timer,
      ))
      .add_system(Self::set_metronome_timer)
      .add_system(Self::play_metronome.after(Self::set_metronome_timer));
  } 

}
