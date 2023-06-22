use bevy::prelude::*;
use bevy_particle_systems::{ParticleSystemPlugin};
pub struct FretboardPlugin;

mod components;
mod resources;
mod systems;
mod styles;
mod events;

use crate::GameState;

use self::events::{ReleaseEvent, MissEvent, HitEvent};
use self::systems::particles::{show_particles, hide_particles};
use self::systems::{build::*, interactions::*, translate::*};
use self::resources::*;
use self::systems::miss_sounds::play_missed_sound;
use self::systems::loader::load_missed_sounds;
use self::systems::highlight::{highlight, dehighlight};

impl Plugin for FretboardPlugin {
    fn build(&self, app: &mut App) {

        app
        .insert_resource(MissedSounds { handles: Vec::new() })
        .add_plugin(ParticleSystemPlugin)
        .add_event::<MissEvent>()
        .add_event::<HitEvent>()
        .add_event::<ReleaseEvent>()

        .insert_resource(Base::default())
        .add_system(load_missed_sounds.in_schedule(OnEnter(GameState::Loading)))
        .add_systems((
            init_values,
            build_contacts.after(init_values),
            build_strings.after(build_contacts),
            build_notes.after(build_strings),
        ).in_schedule(OnEnter(GameState::Prepare)))
        .add_system(check_input.run_if(in_state(GameState::Play).or_else(in_state(GameState::Prepare))))
        .add_system(translate.run_if(in_state(GameState::Prepare).or_else(in_state(GameState::Play))))
        .add_system(play_missed_sound.run_if(in_state(GameState::Prepare).or_else(in_state(GameState::Play))))
        .add_systems((
            show_particles,
            hide_particles,
            highlight,
            dehighlight,
        ).in_set(OnUpdate(GameState::Play)).after(check_input));
    }
}