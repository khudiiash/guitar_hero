

use bevy::prelude::*;
use bevy_kira_audio::*;

use crate::{loader::resources::Loader, fretboard::resources::MissedSounds};

pub fn load_missed_sounds(
    server: Res<AssetServer>,
    mut loader: ResMut<Loader>,
    mut missed_sounds: ResMut<MissedSounds>,
) {
    for i in 0..=5 {
        let miss_handle: Handle<AudioSource> = server.load(format!("sfx/miss_{}.ogg", i));
        missed_sounds.handles.push(miss_handle.clone()); 
        loader.0.push(miss_handle.clone_untyped());
    }
}