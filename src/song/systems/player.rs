
use bevy::prelude::*;
use bevy_kira_audio::*;
use crate::song::resources::{Song};

pub fn play_song(
    audio: Res<bevy_kira_audio::Audio>,
    song: Res<Song>,
) {
    println!("Playing song");
    audio.play(song.audio.clone());
}

pub fn pause_song(
    audio: Res<bevy_kira_audio::Audio>,
) {
    println!("Pausing song");
    audio.pause();
}