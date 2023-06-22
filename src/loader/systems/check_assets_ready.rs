use bevy::prelude::*;

use crate::loader::resources::Loader;
use crate::GameState;

pub fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loader: Res<Loader>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loader.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            // one of our assets had an error
            println!("Failed to load assets");
        }
        LoadState::Loaded => {
            println!("Assets loaded");
            commands.remove_resource::<Loader>();
            next_game_state.set(GameState::Prepare);
        }
        _ => {
            println!("Loading")
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
