use bevy::prelude::*;
use crate::{song::resources::Song, fretboard::{components::Moving}};

pub fn translate(
    mut query: Query<&mut Transform, With<Moving>>,
    time: Res<Time>,
    song: Res<Song>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= song.bpm as f32 * (song.spacing.1 / 60.0) * time.delta_seconds();
    }
}