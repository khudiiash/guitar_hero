use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle, 
    window::{PresentMode}, core_pipeline::bloom::BloomSettings,
};
use bevy_kira_audio::prelude::*;
use bevy_particle_systems::*;
use lerp::Lerp;

mod plugins;
use plugins::{
    metronome::Metronome,
    score::Score,
    rockmeter::RockMeter,
};

use std::fs;
use std::hash::Hash;
use std::collections::HashMap;

const SPOTS: usize = 6;

const RADIUS: f32 = 20.0;
pub const KEYS: [KeyCode; SPOTS] = [KeyCode::A, KeyCode::S, KeyCode::D, KeyCode::F, KeyCode::J, KeyCode::K];
const SPACING: (f32, f32) = (60., 60.);
const START_TICKS: u32 = 8;

#[derive(Component)]
struct Moving;


#[derive(Component)]
struct Slide {
    spot: usize,
    length: f32,
    active: bool,
}


#[derive(Resource, Default)]
struct StartDelayTimer(Timer);

#[derive(Component)]
    
#[derive(Resource, Default)]
pub struct Song {
    map: Vec<String>,
    bpm: u32,
    delay: f32,
    title: String,
    speed: f32,
    spacing: (f32, f32),
    artist: String,
    audio: Handle<AudioSource>,
}

#[derive(Component)]
pub struct SongAudio(Handle<AudioSource>);

#[derive(Resource, Default)]
pub struct MissSounds {
    sounds: Vec<Handle<AudioSource>>,
}

#[derive(Component)]
pub struct Note;

#[derive(Component)]
pub struct MissedNote;

#[derive(Component)]
struct Spot {
    index: usize,
    active: bool,
}

#[derive(Resource, Default, Debug)]
struct SongChoice {
    artist: String,
    title: String,
}

#[derive(Resource, Default)]
pub struct Data {
    colors: [Color; SPOTS],
}


#[derive(Resource, Default)]
pub struct Base {
    y: f32,
    x: [f32; SPOTS],
} 

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
}


fn main() {
    // create 2d array of notes
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Guitar Hero".into(),
                resolution: (1200., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))

        .add_plugin(ParticleSystemPlugin::default()) 
        .add_plugin(AudioPlugin)
        .add_plugin(Score)
        .add_plugin(RockMeter)

        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Data::default())
        .insert_resource(StartDelayTimer::default())
        .insert_resource::<Base>( Default::default())
        .insert_resource::<Song>(Default::default())
        .insert_resource::<MissSounds>( Default::default() )
        .insert_resource(SongChoice { artist: "polyphia".to_string(), title: "amour".to_string() })

        .add_startup_systems((
            load_song_data, 
            setup
        ))
        .add_system(play_song)
        .add_system(move_notes)
        .add_system(check_input)
        .add_plugin(Metronome)
        .run();
}


fn convert_time_to_y(time: f32, bpm: u32) -> f32 {
    let duration_value = 60.0 as f32 / bpm as f32;
    let y = (time / duration_value) * SPACING.1;
    y
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
    mut base: ResMut<Base>,
    mut song: ResMut<Song>,
    mut missed_sounds: ResMut<MissSounds>,
    asset_server: Res<AssetServer>,
    song_choice: Res<SongChoice>,
    mut data: ResMut<Data>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // create colors
    data.colors[0] = Color::hex("ece0a6").unwrap();
    data.colors[1] = Color::hex("6b2327").unwrap();
    data.colors[2] = Color::hex("df503f").unwrap();
    data.colors[3] = Color::hex("e09f4f").unwrap();
    data.colors[4] = Color::hex("7a9163").unwrap();
    data.colors[5] = Color::hex("4f7b58").unwrap();
    
    let handle: Handle<AudioSource> = asset_server.load(format!("songs/{}/{}.ogg", song_choice.artist, song_choice.title).as_str());
    song.audio = handle.clone(); 
    //spawn audio entity
    commands.spawn((SongAudio(handle),));

    next_game_state.set(GameState::Playing);
    


    
    
    
    // missed notes sounds
    for i in 0..6 {
        missed_sounds.sounds.push(asset_server.load(format!("sfx/miss_{}.ogg", i).as_str()));
    }
     
    // camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings {
            intensity: 0.5,
            ..default()
        }
    ));
    
    // base line
    let window_height = window_query.single().height();
    let base_y: f32 = -window_height / 2. + 100.; 
    base.y = base_y;
    
    
    base.x = [
        -3. * SPACING.0, 
        -2. * SPACING.0, 
        -1. * SPACING.0, 
        0. * SPACING.0,
        1. * SPACING.0,
        2. * SPACING.0,
    ]; 
    
    for i in 0..SPOTS {
        base.x[i] = base.x[i] + SPACING.0 * 0.5;
    };
    for i in 0..SPOTS {
        let color = data.colors[i].clone();
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(base.x[i], base.y, 1.2)),
            ..default()
            },
        Spot { index: i, active: false }
        ));
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RADIUS - 5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_translation(Vec3::new(base.x[i], base.y, 1.5)),
            ..default()
            },
        ));
        
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.45, 0.35, 0.25),
                custom_size: Some(Vec2::new(2.0, window_height as f32 * 2.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(base.x[i], base.y, 0.0)),
            ..default()
            });
        // Add the playing component so it starts playing. This can be added later as well.
    }
    

    for i in 0..SPOTS {
        
        commands
        // Add the bundle specifying the particle system itself.
        .spawn((ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 4_000,
                texture: ParticleTexture::Sprite(asset_server.load("images/px.png")),
                spawn_rate_per_second: 4_000.0.into(),
                initial_speed: JitteredValue::jittered(100.0, -50.0..50.0),
                lifetime: JitteredValue::jittered(0.2, -0.1..0.1),
                z_value_override: Some(JitteredValue::from(100.0)),
                color: ColorOverTime::Gradient(Gradient::new(vec![
                    ColorPoint::new(Color::WHITE, 0.0),
                    ColorPoint::new(Color::rgba(2., 2., 0., 0.5), 0.25),
                    ColorPoint::new(Color::rgba(2., 0., 0., 0.5), 0.75),
                    ColorPoint::new(Color::rgba(0., 0., 0., 0.), 1.0),
                ])),
                scale: 2.0.into(),
                looping: true,
                system_duration_seconds: 0.2,
                ..ParticleSystem::default()
            },
            transform: Transform::from_translation(Vec3::new(base.x[i], base.y, 3.0)),
            ..default()
        }, Spot { index: i, active: false }));
    }

    // notes
    for (i, string) in song.map.iter().enumerate() {
        let mut y = base_y + (RADIUS) + (song.spacing.1 * START_TICKS as f32) + convert_time_to_y(song.delay, song.bpm); 
        let mut slide_start = 0.0;
        let mut slide_length = 0.0;
        let size_32: f32 = song.spacing.1 / (32.0 / 4.0);
        
        for char in string.chars() {
            match char {
                'e' => {
                    commands.spawn((SpriteBundle {
                        sprite: Sprite {
                            color: data.colors[i].clone(),
                            custom_size: Some(Vec2::new(10.0, slide_length)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(base.x[i], slide_start + (slide_length * 0.5), 1.0)),
                        ..default()
                    }, Slide { spot: i, length: slide_length, active: false }, Moving));
                    y += size_32; 
                    slide_start = 0.0;
                    slide_length = 0.0;
                }
                'o' => {
                    if slide_start == 0.0 {
                        slide_start = y;
                    }
                    slide_length += size_32;
                    y += size_32; 
                },
                '-' => {
                    y += size_32; 
                },
                '|' => {
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
                    let x = base.x[i];
                    commands.spawn((MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
                        material: materials.add(ColorMaterial::from(data.colors[i])),
                        transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                        ..default()
                        },
                    Note,
                    Moving
                    ));
                    
                    y += size_32; 
                },
                _ => {}
            }
        }
    }

}


fn load_song_data(
    mut song: ResMut<Song>,
    mut start_delay_timer: ResMut<StartDelayTimer>,
    song_choice: Res<SongChoice>,
) {
    let contents = fs::read_to_string(format!("assets/songs/{}/{}.txt", song_choice.artist, song_choice.title)).expect("Should have been able to read the file");
    //let contents = fs::read_to_string("assets/songs/test.txt").expect("Should have been able to read the file");
    let data: Vec<&str> = contents.lines().collect();

    let mut riffs: HashMap<String, Vec<String>> = HashMap::new(); 
    let mut parts: HashMap<String, Vec<String>> = HashMap::new();
    
    for (i, line) in data.iter().enumerate() {
        if line.starts_with("|") || line.is_empty() {
            continue;
        }
        let mut split = line.split(": ");
        let header = split.next().unwrap();
        let content = match split.next() {
            Some(content) => content.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>(), 
            None => "".to_string(),
        };

        //init song map
        for _ in 0..SPOTS {
            song.map.push("".to_string());
        }

        match header {
            "title" => song.title = content.to_string(), 
            "artist" => song.artist = content.to_string(),
            "bpm" => song.bpm = content.parse::<u32>().unwrap(), 
            "delay" => song.delay = content.parse::<f32>().unwrap(),
            "riff" => {
                let riff_name = content.to_string();
                let mut riff_lines: Vec<String> = Vec::new();
                for (_, line) in data[i+1..i+7].iter().enumerate() {
                    riff_lines.push(line.to_string());
                }
                riffs.insert(riff_name, riff_lines);
            } 
            "part" => {
                let part_name = content.to_string();
                let next_line = data[i+1];
                let part_riffs = next_line.split_whitespace().collect::<Vec<&str>>();
                let mut part_lines: Vec<String> = Vec::new();
                for _ in 0..6 {
                    part_lines.push("".to_string());
                }

                for riff_name in part_riffs.iter() {
                    let riff_lines = riffs.get(riff_name.clone()).unwrap();
                    for (j, line) in riff_lines.iter().enumerate() {
                        part_lines[j] += line;
                    }
                }
               parts.insert(part_name, part_lines);
            },
            "song" => {
               let next_line = data[i+1];
               let song_parts = next_line.split_whitespace().collect::<Vec<&str>>();
               for part_name in song_parts.iter() {
                   let part_lines = parts.get(part_name.clone()).unwrap();
                   for (j, line) in part_lines.iter().enumerate() {
                       song.map[j] += line;
                   }
               }  
            }
            _ => {}
        }
    }
    // calculate song speed based on bpm
    // the lower the bpm, the faster the song
    // this way, we get the same visual speed for every song
    song.speed = 260. / song.bpm as f32;
    song.spacing = (60., 60. * song.speed);
    
    start_delay_timer.0 = Timer::from_seconds((60.0 / song.bpm as f32) * START_TICKS as f32, TimerMode::Once);
}

fn check_input(
    mut commands: Commands,
    mut notes: Query<(Entity, &mut Transform), With<Note>>,
    mut spots: Query<(&mut Handle<ColorMaterial>, &mut Transform), (With<Spot>, Without<Note>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut slides: Query<(&mut Sprite, &mut Slide, &Transform), (With<Slide>, Without<Note>, Without<Spot>)>,
    missed_sounds: Res<MissSounds>,
    data: Res<Data>,
    audio: Res<Audio>,
    keys: Res<Input<KeyCode>>,
    base: ResMut<Base>,
    game_state: Res<State<GameState>>,
    mut particles: Query<(Entity, &mut Spot), With<ParticleSystem>>, 
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut pressed_indices: Vec<usize> = vec![];

    
    // pause 
    if keys.just_pressed(KeyCode::Escape) {
        match game_state.0 {
            GameState::Paused => {
                next_game_state.set(GameState::Playing);
                audio.resume();
            },
            GameState::Playing => {
                next_game_state.set(GameState::Paused);
                audio.pause();
                return;
            },
            _ => {}
        }
    }
    

    for (i, key) in KEYS.iter().enumerate() {

        if keys.just_pressed(*key) {
            let mut hit = false;
            pressed_indices.push(i);

            for (entity, transform) in notes.iter_mut() {
                if (transform.translation.y - base.y).abs() < RADIUS && transform.translation.x == base.x[i] {
                    commands.entity(entity).despawn();
                    hit = true;
                }
            }
            
            if hit == false {
                audio.play(missed_sounds.sounds[i].clone());
            } else {
                for (entity, mut spot) in particles.iter_mut() {
                    if  i == spot.index {
                        commands.entity(entity).insert(Playing);
                        spot.active = true;
                    }                 
                }
            
            } 
        }

        if keys.pressed(*key) && !pressed_indices.contains(&i) {
            pressed_indices.push(i);
        }
        
        if keys.just_released(*key) {
            pressed_indices.retain(|&x| x != i);
            for (entity, mut spot) in particles.iter_mut() {
                if  i == spot.index {
                    commands.entity(entity).remove::<Playing>();
                    spot.active = false;
                }
            }
        }
    }

    for (mut sprite, mut slide, transform) in slides.iter_mut() {

        if pressed_indices.contains(&slide.spot) && transform.translation.y - slide.length * 0.5 <= base.y && transform.translation.y + slide.length * 0.5 > base.y && slide.active == false  { 
            sprite.color = sprite.color * 10.0; 
            slide.active = true;
        } else if slide.active {
            sprite.color = sprite.color * 0.1; 
            slide.active = false;
        } 
    }
    
    
    for (i, (color, mut transform)) in spots.iter_mut().enumerate() {
        let material = materials.get_mut(&color).unwrap();
        if pressed_indices.contains(&i) {
           material.color = data.colors[i] * 2.0; 
           transform.scale.x = transform.scale.x.lerp(1.2, 0.1);
           transform.scale.y = transform.scale.y.lerp(1.2, 0.1); 
        } else {
           material.color = data.colors[i]; 
           transform.scale.x = transform.scale.x.lerp(1., 0.1); 
           transform.scale.y = transform.scale.y.lerp(1., 0.1);
        }
    }

}

fn play_song(
    time: Res<Time>,
    song: Res<Song>,
    mut start_delay_timer: ResMut<StartDelayTimer>,
    audio: Res<Audio>,
) {
    if start_delay_timer.0.tick(time.delta()).just_finished() {
        audio.play(song.audio.clone());
    }
}


fn move_notes(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Moving>>,
    song: Res<Song>,
    game_state: Res<State<GameState>>,
) {
    if game_state.0 != GameState::Playing {
        return;
    }
    for mut transform in query.iter_mut() {
        transform.translation.y -= song.bpm as f32 * (song.spacing.1 / 60.0) * time.delta_seconds();
    }
}