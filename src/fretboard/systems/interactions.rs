use bevy::prelude::*;
use bevy_particle_systems::*;

use crate::fretboard::components::Note;
use crate::fretboard::{styles::*, components::*, resources::*, events::*};


const KEYS: [KeyCode; 6] = [
    KeyCode::A,
    KeyCode::S,
    KeyCode::D,
    KeyCode::F,
    KeyCode::J,
    KeyCode::K,
];


pub fn check_input(
    mut commands: Commands,
    mut notes: Query<(Entity, &mut Transform), With<Note>>,
    keys: Res<Input<KeyCode>>,
    base: ResMut<Base>,
    mut miss_event_writer: ResMut<Events<MissEvent>>,
    mut hit_event_writer: ResMut<Events<HitEvent>>,
    mut release_event_writer: ResMut<Events<ReleaseEvent>>,
) {
    for (i, key) in KEYS.iter().enumerate() {

        if keys.just_pressed(*key) {
          let mut hit = false;

          for (entity, transform) in notes.iter_mut() {
            let x = transform.translation.x;
            let y = transform.translation.y;

            if (y - base.y).abs() <= RADIUS * 1.5 && x == base.x[i] {
                commands.entity(entity).despawn();
                hit = true;
            }
          }

          if hit {
            hit_event_writer.send(HitEvent(i));
          } else {
            miss_event_writer.send(MissEvent(i));
          } 
        }
        
        if keys.just_released(*key) {
            release_event_writer.send(ReleaseEvent(i));
        }
    }

    // for (mut sprite, mut slide, transform) in slides.iter_mut() {
    //     if pressed_indices.contains(&slide) && transform.translation.y - slide.length * 0.5 <= base.y && transform.translation.y + slide.length * 0.5 > base.y && slide.active == false  { 
    //         sprite.color = sprite.color * 10.0; 
    //         slide.active = true;
    //     } else if slide.active {
    //         sprite.color = sprite.color * 0.1; 
    //         slide.active = false;
    //     } 
    // }
    

}
