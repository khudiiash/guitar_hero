
use bevy::prelude::*;
use bevy_kira_audio::*;
use super::resources::*;
use crate::GameState;

pub struct LoaderPlugin;

pub fn load_metronome(
    server: Res<AssetServer>,
    mut loader: ResMut<Loader>,
    song_choice: Res<SongChoice>,
    mut song: ResMut<Song>,
) {
    println!("Loading metronome");
    let song_handle: Handle<AudioSource> = server.load(format!("songs/{}/{}.ogg", song_choice.artist, song_choice.title));
    song.audio = song_handle.clone();
    loading.0.push(song_handle.clone_untyped());
}


fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<SongLoader>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            // one of our assets had an error
            println!("Failed to load assets");
        }
        LoadState::Loaded => {
            println!("Assets loaded");
            commands.remove_resource::<SongLoader>();
            next_game_state.set(GameState::Play);
        }
        _ => {
            println!("Assets not loaded yet")
            // NotLoaded/Loading: not fully ready yet
        }
    }
}

pub fn play_song(
    audio: Res<bevy_kira_audio::Audio>,
    song: Res<Song>,
) {
    println!("Playing song");
    audio.play(song.audio.clone());
}

impl Plugin for LoaderPlugin {

    fn build(&self, app: &mut App) {
        app
            .insert_resource(SongLoader::default())
            .add_system(load_song.in_schedule(OnEnter(GameState::Loading)))
            .add_system(check_assets_ready.in_set(OnUpdate(GameState::Loading)))
            .add_system(play_song.in_schedule(OnEnter(GameState::Play)));

    }

}