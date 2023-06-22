use bevy::prelude::*;
use bevy_kira_audio::*;

use crate::loader::resources::Loader;
use crate::song::resources::{Song, SongChoice};

pub fn load_song(
    server: Res<AssetServer>,
    mut loader: ResMut<Loader>,
    song_choice: Res<SongChoice>,
    mut song: ResMut<Song>,
) {
    println!("Loading song");
    let song_handle: Handle<AudioSource> = server.load(format!("songs/{}/{}.ogg", song_choice.artist, song_choice.title));
    song.audio = song_handle.clone();
    loader.0.push(song_handle.clone_untyped());
}