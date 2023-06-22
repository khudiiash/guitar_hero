use bevy::prelude::*;
use bevy_particle_systems::{Playing, ParticleSystem};
use crate::fretboard::events::*;
use crate::fretboard::components::*;

pub fn show_particles(
  mut commands: Commands,
  mut particles: Query<(Entity, &Contact), With<ParticleSystem>>,
  mut hit_events: EventReader<HitEvent>,
) {
  if hit_events.is_empty() {
    return;
  }
  
  let index = hit_events.iter().next().unwrap().0;

  for (entity, contact) in particles.iter_mut() {
    if contact.index == index {
      commands.entity(entity).insert(Playing);
    } 
  }
  
  // hit_events.clear();
}

pub fn hide_particles(
  mut commands: Commands,
  mut particles: Query<(Entity, &Contact), With<ParticleSystem>>,
  mut release_events: EventReader<ReleaseEvent>,
) {
  if release_events.is_empty() { 
    return;
  } 
  let index = release_events.iter().next().unwrap().0;

  for (entity, contact) in particles.iter_mut() {
    if contact.index == index {
      commands.entity(entity).remove::<Playing>();
    } 
  }
  
  // release_events.clear();
}