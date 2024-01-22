use bevy::prelude::*;
use crate::player::systems::spawn_player;
use crate::player::systems::tick_loop;
use crate::player::systems::move_player;
use crate::world::systems::update_map;
use crate::player::systems::change_tick_speed;
use crate::AppState;
use crate::player::systems::pause_tick;
use crate::player::systems::unpause_tick;

pub mod components;
pub mod systems;
pub mod resources;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems 
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(Update, 
                (
                    tick_loop.after(update_map),
                    move_player.after(tick_loop),
                    change_tick_speed,
                    pause_tick,
                    unpause_tick
                )
                .run_if(in_state(AppState::Game))
             );
    }
}
