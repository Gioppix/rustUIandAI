use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::Robot;
use robotics_lib::world::World;
use robotics_lib::interface::{Direction, put};
use robotics_lib::interface::go;
use robotics_lib::utils::go_allowed;
use bevy::ecs::system::Resource;
use crate::{AI, Component};
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::interface::robot_map;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use rand::Rng;
use crate::Timer;
use robotics_lib::interface::where_am_i;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::interface::look_at_sky;
use robotics_lib::interface::get_score;
use robotics_lib::interface::destroy;
use std::collections::HashMap;
use robotics_lib::utils::LibError;
use robotics_lib::interface::discover_tiles;
use robotics_lib::interface::one_direction_view;
use std::time::Duration;
use std::thread;
use op_map::op_pathfinding::{get_best_action_to_element, OpActionInput, OpActionOutput, ShoppingList};
use op_map::op_pathfinding::OpActionInput::Destroy;
use rust_eze_spotlight::Spotlight;
use AI::training;
use crate::AI::training::{Brain, train};
use crate::AI::training::BrainAction;
use crate::AI::training::BrainAction::*;

#[derive(Resource)]
pub struct TickTimer {
    pub timer: Timer,
}


pub struct MyRobot(pub Robot, pub Mutex<Sender<((Vec<Vec<Option<Tile>>>, (usize, usize)), EnvironmentalConditions, f32)>>, Brain);

impl MyRobot {
    pub(crate) fn new(r: Robot, m: Mutex<Sender<((Vec<Vec<Option<Tile>>>, (usize, usize)), EnvironmentalConditions, f32)>>) -> Self {
        // Self(r, m, Brain::from_network(net))
        Self(r, m, Brain::default())
    }
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        // println!("Tick nel roboto vero");
        // let niugy = robot_map(world).expect("Errore nella mappa");

        let brain_action = self.2.think_action(world, self);

        match brain_action {
            None => {}
            Some(opa) => {
                match opa {
                    OpActionOutput::Move(dir) => {
                        self.go_ui(world, dir);
                    }
                    OpActionOutput::Destroy(dir) => {
                        destroy(self, world, dir);
                    }
                    OpActionOutput::Put(content, quantity, dir) => {
                        put(self, world, content, quantity, dir);
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        // println!("{:?}", event);
    }

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
}

impl MyRobot {
    fn go_ui(&mut self, world: &mut World, direction: Direction) {
        let _ = go(self, world, direction);
        let view = where_am_i(self, world);
        let condition = look_at_sky(world);
        let score = get_score(world);
        let _ = self.1.lock().unwrap().send((view, condition, score));
    }

    fn discover_tiles_ui(&mut self, world: &mut World, to_discover: &[(usize, usize)]) -> Result<HashMap<(usize, usize), Option<Tile>>, LibError> {
        let discovered_hash = discover_tiles(self, world, &to_discover)?;
        let mut empty_vec: Vec<Vec<Option<Tile>>> = vec![vec![None, None, None], vec![None, None, None], vec![None, None, None]];

        let condition = look_at_sky(world);
        let score = get_score(world);

        for (x, y) in to_discover {
            empty_vec[1][1] = discovered_hash.get(&(*x, *y)).unwrap().clone();
            let view = (empty_vec.clone(), (*x, *y));
            let _ = self.1.lock().unwrap().send((view, condition.clone(), score));
        }
        Ok(discovered_hash)
    }

    /*
    fn one_direction_view_ui(&mut self, world: &World, direction: Direction, distance: usize) -> Result<Vec<Vec<Tile>>, LibError> {
        
        if distance <= 0 {
            Err(LibError::OperationNotAllowed)
        }

        let view = one_direction_view(self, world, direction, distance)?;
     
        let condition = look_at_sky(world);
        let score = get_score(world);


        match direction {
            Direction::Up => {
                if distance == 1 {
                    let new_view = vec![view.0, vec![], vec![]];
                } 
                else if distance == 2 {

                }
                // Caso generale
                else {

                }
            },
            Direction::Down => {

            },
            Direction::Left => {

            },
            Direction::Right => {

            },
        }



        Ok(view)
    }
    */
}
