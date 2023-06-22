use bevy::prelude::*;
use crate::{menu::{components::*, styles::*}, GameState};

pub fn interact_with_play_button(
  mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  if let Ok((interaction, mut backround_color)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Clicked => {
        *backround_color = CLICKED_BUTTON_COLOR.into();
        next_state.set(GameState::Loading);
      },
      Interaction::Hovered => {
        *backround_color = HOVERED_BUTTON_COLOR.into();
      },
      Interaction::None => {
        *backround_color = NORMAL_BUTTON_COLOR.into();
      }

    }
  }
}


pub fn interact_with_quit_button(
  mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
) {
  if let Ok((interaction, mut backround_color)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Clicked => {
        *backround_color = CLICKED_BUTTON_COLOR.into();
      },
      Interaction::Hovered => {
        *backround_color = HOVERED_BUTTON_COLOR.into();
      },
      Interaction::None => {
        *backround_color = NORMAL_BUTTON_COLOR.into();
      }

    }
  }
}