use bevy::prelude::*;

pub struct LoaderPlugin;
pub mod resources;
mod systems;

use resources::*;
use systems::check_assets_ready::check_assets_ready;

use crate::GameState;

impl Plugin for LoaderPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Loader::default())
      .add_system(check_assets_ready.in_set(OnUpdate(GameState::Loading)));
  }
}
