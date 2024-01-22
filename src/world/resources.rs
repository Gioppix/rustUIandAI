use bevy::prelude::*;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::Runner;
use robotics_lib::interface::Tools;
use robotics_lib::runner::Robot;
use std::sync::Arc;
use robotics_lib::world::tile::Tile;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;

#[derive(Resource)]
pub struct WorldRes {
    pub world: Option<Vec<Vec<Option<Tile>>>>,
    pub rx: Mutex<Receiver<((Vec<Vec<Option<Tile>>>, (usize, usize)), EnvironmentalConditions, f32)>>,
    pub world_size: usize,
    pub player_x: usize,
    pub player_y: usize,
    pub environmental_conditions: EnvironmentalConditions,
    pub score: f32,
    pub elevation: usize,
}

