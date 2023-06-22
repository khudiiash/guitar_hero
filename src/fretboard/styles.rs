use bevy::prelude::*;
use bevy_particle_systems::*;

pub const COLORS: [&str; 6] = ["ece0a6", "6b2327", "df503f", "e09f4f", "7a9163", "619188"];
pub const SPACING: f32 = 60.;
pub const RADIUS: f32 = 20.;
pub const FRETS: usize = 6;
pub const START_TICKS: usize = 4;

pub fn get_particles_system(asset_server: &Res<AssetServer>) -> ParticleSystem {
  ParticleSystem {
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
    }
}