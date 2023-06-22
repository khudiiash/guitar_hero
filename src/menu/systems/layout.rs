use bevy::prelude::*;

use crate::menu::components::*;
use crate::menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
  println!("spawn_main_menu");
  let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
  mut commands: Commands,
  main_menu_query: Query<Entity, With<MainMenu>>
) {
  println!("despawn_main_menu");
  if let Ok(main_menu_entity) = main_menu_query.get_single() {
    commands.entity(main_menu_entity).despawn_recursive();
  }
}

pub fn build_main_menu(
  commands: &mut Commands,
  asset_server: &Res<AssetServer>,
) -> Entity {

  let main_menu_entity = commands
    .spawn((NodeBundle { 
      style: Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center, 
        gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        ..default()
      },
      background_color: Color::rgba(0., 0., 0., 0.9).into(), 
      ..default()
    }, 
    MainMenu
  ))
  .with_children(|parent| {
    // Title
    spawn_text(parent, asset_server, "Guitar Neon");
    // Play
    spawn_button(parent, asset_server, "Play", PlayButton);
    // Quit
    spawn_button(parent, asset_server, "Quit", QuitButton);
  }) 
  .id();
  main_menu_entity
}

pub fn spawn_button(
  parent: &mut ChildBuilder,
  asset_server: &Res<AssetServer>,
  text: &str,
  button_type: impl Component,
) {
  parent.spawn((
    ButtonBundle {
      style: BUTTON_STYLE,
      background_color: NORMAL_BUTTON_COLOR.into(),
      ..default()
    },
    button_type
  ))
  .with_children(|parent| {
    parent.spawn( TextBundle {
      text: Text {
        sections: vec![
          TextSection::new(
            text,
            TextStyle {
              font: asset_server.load("fonts/Eczar-Medium.ttf"),
              font_size: 40.0,
              color: Color::WHITE,
            }, 
          ) 
        ],
        alignment: TextAlignment::Center, 
        ..default()
      },
      ..default() 
    });
  });
}

pub fn spawn_text(
  parent: &mut ChildBuilder,
  asset_server: &Res<AssetServer>,
  text: &str,
) {
  parent.spawn( TextBundle {
    text: Text {
      sections: vec![
        TextSection::new(
          text,
          TextStyle {
            font: asset_server.load("fonts/Eczar-Medium.ttf"),
            font_size: 40.0,
            color: Color::WHITE,
          }, 
        ) 
      ],
      alignment: TextAlignment::Center, 
      ..default()
    },
    ..default() 
  });
}