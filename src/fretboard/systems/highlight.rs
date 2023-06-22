use bevy::prelude::*;
use bevy_particle_systems::ParticleSystem;
use lerp::Lerp;
use crate::fretboard::styles::RADIUS;
use crate::fretboard::{styles::COLORS, components::*, resources::Base, events::*};
use crate::fretboard::components::Line;

const HIGHLIGHT_INTENSITIES: [f32; 6] = [3., 8., 6., 6., 6., 6.];

pub fn highlight(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut contacts: Query<(&mut Handle<ColorMaterial>, &mut Transform), (With<Contact>, Without<ParticleSystem>)>,
    mut slides: Query<(&mut Sprite, &Transform, &Line, &mut Slide), (With<Slide>, Without<Contact>)>,
    mut hit_events: EventReader<HitEvent>,
    base: Res<Base>, 
) {
    if hit_events.is_empty() {
        return;
    }
    
    // Get the indices of the lines that were hit
    let indices = hit_events.iter().map(|e| e.0).collect::<Vec<usize>>();

    for (index, (color, mut transform)) in contacts.iter_mut().enumerate() {
        let material = materials.get_mut(&color).unwrap();
        if indices.contains(&index) {
           material.color = Color::hex(COLORS[index]).unwrap() * HIGHLIGHT_INTENSITIES[index]; 
           transform.scale.x = transform.scale.x.lerp(1.2, 0.1);
           transform.scale.y = transform.scale.y.lerp(1.2, 0.1); 
        }
    }
    for (mut sprite, transform, line, slide) in slides.iter_mut() {
        if !overlap_y(transform.translation.y - RADIUS, base.y, slide.length) { 
            continue; 
        }
        if indices.contains(&line.index) {
            sprite.color = Color::hex(COLORS[line.index]).unwrap() * HIGHLIGHT_INTENSITIES[line.index];
        }
    }
}

fn overlap_y(y1: f32, y2: f32, length: f32) -> bool {
    y1 - length * 0.5 <= y2 && y1 + length * 0.5 > y2
}
pub fn dehighlight(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut contacts: Query<(&mut Handle<ColorMaterial>, &mut Transform), (With<Contact>, Without<ParticleSystem>)>,
    mut release_events: EventReader<ReleaseEvent>,
    mut slides: Query<(&mut Sprite, &Transform, &Line, &mut Slide), (With<Slide>, Without<Contact>)>,
    base: Res<Base>,
) {
    if release_events.is_empty() {
        return;
    }
    
    let indices = release_events.iter().map(|e| e.0).collect::<Vec<usize>>();

    for (i, (color, mut transform)) in contacts.iter_mut().enumerate() {
        let material = materials.get_mut(&color).unwrap();
        if indices.contains(&i) {
           material.color = Color::hex(COLORS[i]).unwrap(); 
           transform.scale.x = transform.scale.x.lerp(1., 0.1); 
           transform.scale.y = transform.scale.y.lerp(1., 0.1);
        } 
    }

    for (mut sprite, transform, line, mut slide) in slides.iter_mut() {
        let slide_start = transform.translation.y - slide.length * 0.5;
        if slide_start > base.y {
            continue;
        }
        if indices.contains(&line.index) {
            sprite.color = Color::hex(COLORS[line.index]).unwrap();
            slide.active = true;
        }
    }
}