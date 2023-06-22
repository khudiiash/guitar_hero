use bevy::{prelude::*};
use crate::{KEYS, RADIUS, Base, Note, };

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCORE_COLOR: Color =  Color::rgb(1., 1., 1.);

#[derive(Component)]
pub struct Score;

#[derive(Resource, Default)]
pub struct ScoreValue {
  pub value: u32,
}

impl Score {

   pub fn setup(
      mut commands: Commands,
      asset_server: Res<AssetServer>,
    ) {
      // score text
      commands.spawn((
          TextBundle::from_section(
              "",
              TextStyle {
                  font: asset_server.load("fonts/Eczar-Medium.ttf"),
                  font_size: SCOREBOARD_FONT_SIZE,
                  color: SCORE_COLOR
              }
          )
          .with_text_alignment(TextAlignment::Center)
          // Set the style of the TextBundle itself.
          .with_style(Style {
              position_type: PositionType::Absolute,
              position: UiRect {
                  bottom: Val::Px(85.),
                  left: Val::Px(90.),
                  ..default()
              },
              ..default()
          }),
          Score
      ));

   } 
 
  pub fn update(
    mut query: Query<&mut Text, With<Score>>,
    score: ResMut<ScoreValue>,
  ) {
    let mut text = query.single_mut();
    text.sections[0].value = score.value.to_string();
  }
  
  pub fn check_input(
    mut score: ResMut<ScoreValue>,
    base: ResMut<Base>,
    mut notes: Query<&Transform, With<Note>>,
    keyboard_input: ResMut<Input<KeyCode>>,
  ) {

    for (i, key) in KEYS.iter().enumerate() {
        if keyboard_input.just_pressed(*key) {
            for transform in notes.iter_mut() {
                if (transform.translation.y - base.y).abs() < RADIUS && transform.translation.x == base.x[i] {
                    score.value += 1;
                }
            }
        }
    }
  }  

}

impl Plugin for Score {
  
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ScoreValue { value: 0 })
      .add_startup_system(Self::setup)
      .add_system(Self::update)
      .add_system(Self::check_input);
  }
  
}