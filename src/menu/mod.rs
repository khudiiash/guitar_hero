
mod components;
mod styles;
mod systems;

use systems::layout::*;
use bevy::prelude::*;
use crate::GameState;

use self::systems::interactions::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {

    fn build(&self, app: &mut App) {
      app
        .add_system(spawn_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
        .add_system(despawn_main_menu.in_schedule(OnExit(GameState::MainMenu)))
        .add_systems((
          interact_with_play_button,
          interact_with_quit_button
        )
        .in_set(OnUpdate(GameState::MainMenu)));
    }

}
