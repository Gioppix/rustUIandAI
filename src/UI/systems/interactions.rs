use bevy::prelude::*;

use crate::UI::components::*;
use crate::UI::styles::*;
use crate::AppState;
use bevy::app::AppExit;
use std::time::Duration;
use crate::player::resources::TickTimer;

pub fn interact_with_play_tick_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayTickButton>)
        >,
    mut timer_res: ResMut<TickTimer>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                if timer_res.timer.paused() {
                    timer_res.timer.unpause();
                }
                timer_res.timer.set_duration(Duration::from_secs(1));
            },
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_pause_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PauseButton>)
        >,
    mut timer_res: ResMut<TickTimer>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                timer_res.timer.pause();
            },
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_double_speed_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DoubleSpeedButton>)
        >,
    mut timer_res: ResMut<TickTimer>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                timer_res.timer.set_duration(Duration::from_millis(100));
            },
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
