use bevy::{
    prelude::*, 
    window::{ PresentMode }, core_pipeline::bloom::BloomSettings 
};
use bevy_kira_audio::prelude::*;
use song::SongPlugin;

mod loader;
mod metronome;
mod menu;
mod song;
mod fretboard;

use loader::LoaderPlugin;
use menu::MainMenuPlugin;
use fretboard::FretboardPlugin;
use metronome::MetronomePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Prepare,
    Play,
    Paused,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Guitar Hero".into(),
                resolution: (1200., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LoaderPlugin)
        .add_plugin(MetronomePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(SongPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(FretboardPlugin)
        .add_startup_system(setup)
        .run();
}

pub fn setup(
    mut commands: Commands,
) {
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
}


