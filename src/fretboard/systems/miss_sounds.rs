use bevy::prelude::*;
use bevy_kira_audio::*;

use crate::fretboard::{events::MissEvent, resources::MissedSounds};


pub fn play_missed_sound(
    audio: ResMut<Audio>,
    mut missed_event: EventReader<MissEvent>,
    missed_sounds: Res<MissedSounds>,
) {
    if missed_event.is_empty() {
        return;
    }
    let missed_id = missed_event.iter().next().unwrap().0;
    audio.play(missed_sounds.handles[missed_id].clone());
}