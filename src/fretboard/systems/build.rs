use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_particle_systems::*;

use crate::fretboard::{components::*, resources::*, styles::*};
use crate::song::resources::Song;

use crate::fretboard::components::Line;

pub fn init_values(
    mut base: ResMut<Base>,
    window_query: Query<&Window>,
) {
    // base line
    let window_height = window_query.single().height();
    base.y = -window_height / 2. + 100.; 
    
    base.x = [
        -3. * SPACING, 
        -2. * SPACING, 
        -1. * SPACING,
        0. * SPACING,
        1. * SPACING,
        2. * SPACING,
    ]; 
    
    for i in 0..FRETS {
        base.x[i] = base.x[i] + SPACING * 0.5;
    };
}

pub fn build_notes(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  base: Res<Base>,
  song: Res<Song>,
) {
      // notes
      for (i, string) in song.map.iter().enumerate() {
        let mut y = base.y - RADIUS * 2.0 + (song.spacing.1 * (START_TICKS + 1) as f32) + convert_time_to_y(song.delay, song.bpm); 
        let mut slide_start = 0.0;
        let mut slide_length = 0.0;
        let size_32: f32 = song.spacing.1 / (32.0 / 4.0);
        println!("32 - song.beats: {}", 32.0 - song.beats as f32);
        y -= (size_32 * (32.0 - song.beats as f32));
        
        for char in string.chars() {
            match char {
                'e' => {
                    commands.spawn((SpriteBundle {
                        sprite: Sprite {
                            color: Color::hex(COLORS[i].clone()).unwrap(),
                            custom_size: Some(Vec2::new(10.0, slide_length)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(base.x[i], slide_start + (slide_length * 0.5), 1.0)),
                        ..default()
                    }, 
                    Line {index: i, x: base.x[i], active: false },
                    Slide { length: slide_length, active: false },
                    Moving
                  ));
                    y += size_32; 
                    slide_start = 0.0;
                    slide_length = 0.0;
                }
                'o' => {
                    // Slide continuation
                    if slide_start == 0.0 {
                        slide_start = y - RADIUS;
                    }
                    slide_length += size_32;
                    y += size_32; 
                },
                '-' => {
                    // Skip
                    y += size_32; 
                },
                '|' => {
                    // Fret line 
                    commands.spawn((SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.45, 0.35, 0.25),
                            custom_size: Some(Vec2::new(song.spacing.0 as f32, 2.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(base.x[i], y, 0.0)),
                        ..default()
                        }, Moving));
                },
                'x' => {
                    // Note
                    let x = base.x[i];
                    commands.spawn((MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::hex(COLORS[i].clone()).unwrap())),
                        transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                        ..default()
                        },
                    Line { index: i, x, active: false },
                    Note { active: false },
                    Moving
                    ));
                    
                    y += size_32; 
                },
                _ => {}
            }
        }
    }

}


pub fn build_contacts(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
  base: Res<Base>,
) {

  for i in 0..FRETS {
    // Contact circles
    let color = Color::hex(COLORS[i].clone()).unwrap();
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
        material: materials.add(ColorMaterial::from(color)),
        transform: Transform::from_translation(Vec3::new(base.x[i], base.y, 1.2)),
        ..default()
        },
        Contact { index: i, x: base.x[i] }
    ));
    // Black inner circles
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(RADIUS - 5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        transform: Transform::from_translation(Vec3::new(base.x[i], base.y, 1.5)),
        ..default()
        },
    ));

    // Particles 
    commands.spawn((ParticleSystemBundle {
          particle_system: get_particles_system(&asset_server),
          transform: Transform::from_translation(Vec3::new(base.x[i], base.y, 3.0)),
          ..default()
        },
        Contact { index: i, x: base.x[i] },
    )); 
    }
}

pub fn build_strings(
  mut commands: Commands,
  window: Query<&Window>,
  base: Res<Base>,
) {
  for i in 0..FRETS {
      // Vertical strings
      commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.45, 0.35, 0.25),
            custom_size: Some(Vec2::new(2.0, window.single().height() as f32 * 2.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(base.x[i], base.y, 0.0)),
        ..default()
        });
  }
}


fn convert_time_to_y(time: f32, bpm: u32) -> f32 {
  let duration_value = 60.0 as f32 / bpm as f32;
  let y = (time / duration_value) * SPACING;
  y
}
