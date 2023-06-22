use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::{KEYS, RADIUS, Base, Note, MissedNote, GameState};

pub struct RockMeter;

#[derive(Component)]
pub struct Rock;


#[derive(Resource, Default)]
pub struct RockValue {
  pub min: f32,
  pub max: f32,
  pub value: f32,
  pub color: Color,
}

#[derive(Component)]
pub struct Rotating;

#[derive(Component)]
pub struct RockMeterBackground;

impl RockMeter {

   pub fn setup(
      mut commands: Commands,
      asset_server: Res<AssetServer>,
      window: Query<&Window>,
      mut materials: ResMut<Assets<ColorMaterial>>,
      mut meshes: ResMut<Assets<Mesh>>,
    ) {
     
      let window = window.single();
      let window_width = window.width();
      let window_height = window.height();
      let radius = 99.;
      

      let transform = Transform {
          translation: Vec3::new(window_width / 2.0 - radius, -window_height / 2.0 + radius, 1.),
          scale: Vec3::new(0.5, 0.5, 0.),
          rotation: Quat::from_rotation_z(0.),
          ..Default::default()
      };
      
      

      commands.spawn((MaterialMesh2dBundle {
          mesh:     meshes.add(shape::Circle::new(radius).into()).into(),
          material: materials.add(ColorMaterial::from(Color::BLACK)),
          transform: Transform {
            translation: Vec3::new(window_width / 2.0 - radius, -window_height / 2.0 + radius, 0.0),
            ..transform
          }, 
          ..default()
          },
          RockMeterBackground
      ));
      commands.spawn((MaterialMesh2dBundle {
          mesh:     meshes.add(shape::Circle::new(radius - 10.).into()).into(),
          material: materials.add(ColorMaterial::from(Color::BLACK)),
          transform,
          ..default()
          },
      ));
      // rock meter sprite
      commands.spawn((SpriteBundle {
          texture: asset_server.load("images/rock_meter_arrow.png"),
          transform,
          ..default()
      }, Rotating));

      // rock meter sprite
      commands.spawn(SpriteBundle {
          texture: asset_server.load("images/rock_meter_glass.png"),
          transform: transform,
          ..default()
      });
   } 
   
   pub fn update_rotation(
    mut query: Query<&mut Transform, With<Rotating>>,
    rock_value: Res<RockValue>,

   ) {
    let mut transform = query.single_mut();
        let rotation = Quat::from_rotation_z(-(rock_value.value / rock_value.max) * std::f32::consts::PI * 0.75);
        transform.rotation = transform.rotation.lerp(rotation, 0.1);
   }
   

   pub fn update_color(
    mut query: Query<&mut Handle<ColorMaterial>, With<RockMeterBackground>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    rock_value: Res<RockValue>,
   ) {
    if let Ok(handle) = query.get_single_mut() {
      let color_mat = materials.get_mut(&handle).unwrap();
      let t = (rock_value.value - rock_value.min) / (rock_value.max - rock_value.min);
      let mut red = Color::hex("ab3130").unwrap();
      let mut green = Color::hex("7c9923").unwrap();
      let mult = 7.;
      red = Color::rgb(red.r() * mult, red.g() * mult, red.b() * mult);
      green = Color::rgb(green.r() * mult, green.g() * mult, green.b() * mult);
    
      let interpolated_color = Color::from(Vec4::from(red).lerp(Vec4::from(green), t));
      color_mat.color = interpolated_color; 
    }
  }
 
  
  pub fn check_input(
    mut rock_meter: ResMut<RockValue>,
    base: ResMut<Base>,
    mut notes: Query<&Transform, With<Note>>,
    keyboard_input: ResMut<Input<KeyCode>>,
  ) {

    let mut hit = false;

    for (i, key) in KEYS.iter().enumerate() {
        if keyboard_input.just_pressed(*key) {
            for transform in notes.iter_mut() {
                if (transform.translation.y - base.y).abs() < RADIUS && transform.translation.x == base.x[i] {
                   hit = true; 
                }                 
            }
            
            if hit {
              rock_meter.value = (rock_meter.value + 10.).min(rock_meter.max);
            } else {
              rock_meter.value = (rock_meter.value - 10.).max(rock_meter.min);
            }
        }
    }
    
  }  
  
  pub fn check_missed_notes(
    mut commands: Commands,
    mut rock_meter: ResMut<RockValue>,
    mut notes: Query<(Entity, &Transform), (With<Note>, Without<MissedNote>)>,
    base: Res<Base>,
    window: Query<&Window>,

  ) {
    for (entity, transform) in notes.iter_mut() {
      //check if note is below the lower window bound
      if transform.translation.y > base.y {
        continue;
      } 
      
      if transform.translation.y < -window.single().height() / 2.0 - RADIUS {
        continue;
      }

      
      if (transform.translation.y - base.y).abs() > RADIUS * 2.0 {
        rock_meter.value = (rock_meter.value - 10.).max(rock_meter.min);
        commands.entity(entity).insert(MissedNote);
      }
    }
  }

}

impl Plugin for RockMeter {
  
  fn build(&self, app: &mut App) {
    app
      .insert_resource(RockValue { value: 0., min: -100., max: 100., color: Color::rgb(1.0, 1.0, 0.0) })
      .add_startup_system(Self::setup)

      .add_systems((
          Self::check_input,
          Self::check_missed_notes,
          Self::update_rotation,
          Self::update_color,
      ).in_set(OnUpdate(GameState::Play)));
  }
  
}