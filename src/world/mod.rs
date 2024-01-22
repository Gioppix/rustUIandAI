use bevy::prelude::*;
use crate::world::systems::spawn_map;
use crate::world::systems::update_map;
use crate::AppState;

pub mod components;
pub mod systems;
pub mod resources;
pub mod generator;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_map)
            .add_systems(Update, 
                (
                    update_map,
                )
                .run_if(in_state(AppState::Game))
            );
    }
}
