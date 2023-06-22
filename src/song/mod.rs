use bevy::prelude::*;

mod systems; 
pub mod resources;

pub struct SongPlugin;

use systems::loader::load_song;
use systems::parser::parse_song_data;
use systems::player::{play_song, pause_song};


use crate::GameState;

use self::resources::*;

impl Plugin for SongPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Song::default())
      .insert_resource(SongChoice {
        artist: "system_of_a_down".to_string(),
        title: "lonely_day".to_string()
      })
      .add_systems((
        load_song,
        parse_song_data,
      ).in_schedule(OnEnter(GameState::Loading)))
      .add_system(play_song.in_schedule(OnEnter(GameState::Play)))
      .add_system(pause_song.in_schedule(OnExit(GameState::Play)));
    
  }
}