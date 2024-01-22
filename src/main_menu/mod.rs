use bevy::prelude::*;
use crate::AppState;
use systems::layout::*;
use crate::main_menu::systems::interactions::*;

mod components;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
           .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
           .add_systems(Update,
                (
                    interact_with_play_button,
                    interact_with_quit_button,
                ).run_if(in_state(AppState::MainMenu))
            );
    }
}
