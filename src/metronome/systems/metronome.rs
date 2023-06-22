use crate::GameState;
use crate::song::resources::Song;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;


const START_TICKS: u32 = 4;

use crate::metronome::resources::*; 
  
pub fn set_song_delay_timer(
  mut timer: ResMut<SongDelayTimer>,
  song: Res<Song>,
) {
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
  audio: Res<Audio>,
  metronome_audio: Res<MetronomeAudio>,
  mut is_timer_set: ResMut<IsTimerSet>,
  mut song_delay_timer: ResMut<SongDelayTimer>,
) {
  if !song_delay_timer.0.finished() || is_timer_set.0 {
    song_delay_timer.0.tick(time.delta());
    return;
  }
  audio.play(metronome_audio.audio.clone());
  println!("Tick: {}", 1);
  let duration_value = 60.0 as f32 / song.bpm as f32;
  timer.0 = Timer::from_seconds(duration_value, TimerMode::Repeating);

  is_timer_set.0 = true;
}
  
pub fn play_metronome(
    mut timer: ResMut<MetronomeTimer>,
    time: Res<Time>,
    audio: Res<Audio>,
    is_timer_set: ResMut<IsTimerSet>,
    metronome_audio: Res<MetronomeAudio>,
    start_delay_timer: Res<SongDelayTimer>,
    mut ticks: ResMut<Ticks>,
    mut game_next_state: ResMut<NextState<GameState>>,
  ) {
    if !is_timer_set.0 {
      return;
    }
    let just_finished = timer.0.tick(time.delta()).just_finished(); 
    let delay_finished = start_delay_timer.0.finished();
    if !just_finished || !delay_finished { 
        return;
    }
    if ticks.value < START_TICKS - 1 {
        ticks.value += 1;
        println!("Tick: {}", ticks.value + 1);
        audio.play(metronome_audio.audio.clone());
    } else if ticks.value == START_TICKS - 1 {
        ticks.value = 0;
        game_next_state.set(GameState::Play);      
        println!("Done");
    }     
  }



