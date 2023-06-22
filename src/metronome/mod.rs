use bevy::prelude::*;
use crate::GameState;

pub struct MetronomePlugin;

mod resources;
mod systems;

use resources::*;
use systems::metronome::*;

impl Plugin for MetronomePlugin {

  fn build(&self, app: &mut App) {
    app
      .insert_resource(SongDelayTimer::default())
      .insert_resource(MetronomeAudio::default())
      .insert_resource(MetronomeTimer::default())
      .insert_resource(IsTimerSet(false))
      .insert_resource(Ticks { value: 0 })
      .add_system(load_audio.in_schedule(OnEnter(GameState::Loading)))
      .add_system(set_song_delay_timer.in_schedule(OnEnter(GameState::Prepare)))
      .add_systems((
        set_metronome_timer,
        play_metronome.after(set_metronome_timer)
      ).in_set(OnUpdate(GameState::Prepare)));
  } 

}