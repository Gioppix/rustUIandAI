use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;
use std::{mem, thread};
use op_map::op_pathfinding::{get_best_action_to_element, OpActionInput, OpActionOutput, ShoppingList};
use oxagworldgenerator::world_generator::content_options::OxAgContentOptions;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use rand::{Rng, thread_rng};
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{destroy, Direction, get_score, go, put};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::utils::go_allowed;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content;
use robotics_lib::world::World;
use robotics_lib::world::world_generator::Generator;
use crate::AI::network::network::{ActivationFunction, LayerTopology, Network};
use crate::AI::training::BrainAction::{Explore, GetContent, PutContent};
use std::io::{self, Write};
use std::time::Instant;
use op_map::op_pathfinding::OpActionInput::Destroy;
use rand::distributions::WeightedIndex;
use worldgen_unwrap::public::WorldgeneratorUnwrap;


pub fn train(gen_size: u32) -> Network {
    println!("Started training");
    //return generator.gen();
    let mut brains: Vec<Brain> = (0..gen_size).map(|_| Brain::default()).collect();

    for _ in 0..1 {
        let scores = thread::scope(move |s| {
            let mut handles = Vec::new();

            let seed: u64 = thread_rng().gen_range(421..=421);

            for brain in brains {
                let h = s.spawn(move || {
                    let mut w = generate_generator(seed);

                    let my_robot = TrainingRobot::from_brain(brain);
                    let start = Instant::now(); // Start time
                    let maybe_runner = Runner::new(Box::new(my_robot), &mut w);
                    // println!("Time taken: {:?}", start.elapsed());
                    if let Err(error) = maybe_runner {
                        panic!("Runner has an error, {:?}", error);
                    }
                    let mut runner = maybe_runner.expect("Just checked");
                    // println!("Score {}: {}", i, runner.expect("").);

                    for _ in 0..500 {
                        let res = runner.game_tick();
                    }

                    let robot = runner.get_robot();
                    let training_robot: &Box<TrainingRobot> = unsafe {
                        mem::transmute::<&Box<dyn Runnable>, &Box<TrainingRobot>>(robot)
                    };
                    let score = training_robot.get_score();


                    return (score, training_robot.get_cloned_brain());
                    // return (-1.99983, Brain::default());

                });
                handles.push(h);
            }
            let mut scores = Vec::new();
            for handle in handles {
                scores.push(handle.join().unwrap());
            }
            return scores;
        });
        let start = Instant::now(); // Start time
        brains = end_and_reproduce(scores);
    }
    return brains[0].network.clone();
}

fn end_and_reproduce(mut scores_brains: Vec<(f32, Brain)>) -> Vec<Brain> {
    scores_brains.sort_by(|v1, v2| v2.0.total_cmp(&v1.0));

    let scores: Vec<f32> = scores_brains.iter().map(|s| s.0).collect();
    println!("{:?}, avg: {:.3}", scores, scores_brains.iter().map(|b| b.0).sum::<f32>() / scores_brains.len() as f32);

    let mut rng = rand::thread_rng();
    let maybe_weight_distribution = WeightedIndex::new(&scores);
    if let Ok(weight_distribution) = maybe_weight_distribution {
        let mut new_b = Vec::with_capacity(scores_brains.len());

        // Mutation parameters
        let mutation_rate = 0.2;
        let mutation = 0.1;

        while new_b.len() < scores_brains.len() {
            let parent1_index = rng.sample(&weight_distribution);
            let parent2_index = rng.sample(&weight_distribution);

            let parent1 = &scores_brains[parent1_index].1;
            let parent2 = &scores_brains[parent2_index].1;

            let new_network = parent1.network.mutate(&parent2.network, mutation_rate, mutation);
            new_b.push(Brain::from_network(new_network));
        }
        new_b
    } else {
        scores_brains.into_iter().map(|sb| sb.1).collect()
    }
}

struct TrainingRobot {
    brain: Brain,
    robot: Robot,
    score: f32,
}


impl Runnable for TrainingRobot {
    fn process_tick(&mut self, world: &mut World) {
        let brain_action = self.brain.think_action(world, self);
        match brain_action {
            None => {}
            Some(opa) => {
                match opa {
                    OpActionOutput::Move(dir) => {
                        go(self, world, dir);
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

        self.score = get_score(world);
    }

    fn handle_event(&mut self, event: Event) {}

    fn get_energy(&self) -> &Energy { &self.robot.energy }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate { &mut self.robot.coordinate }
}

impl TrainingRobot {
    fn new() -> Self {
        Self {
            brain: Brain::default(),
            robot: Robot::new(),
            score: 0f32,
        }
    }
    fn from_brain(brain: Brain) -> Self {
        Self {
            brain,
            robot: Robot::new(),
            score: 0f32,
        }
    }
    fn get_score(&self) -> f32 {
        self.score
    }

    fn get_cloned_brain(&self) -> Brain {
        self.brain.clone()
    }
}

#[derive(Clone)]
pub struct Brain {
    pub network: Network,
}

impl Default for Brain {
    fn default() -> Self {
        Self {
            // Input: energy, number of Garbage in the inventory, garbage known, bin known, is_inv_full
            // Output:  GetGarbage, PutGarbage,  Explore
            network: Network::random(&[
                LayerTopology { neurons: 3, activation_function: ActivationFunction::ReLU },
                LayerTopology { neurons: 4, activation_function: ActivationFunction::ReLU },
                LayerTopology { neurons: 3, activation_function: ActivationFunction::TANH },
            ]),
        }
    }
}

impl Brain {
    pub fn from_network(network: Network) -> Self {
        Self {
            network
        }
    }
    pub(crate) fn think_action(&self, world: &World, robot: &impl Runnable) -> Option<OpActionOutput> {
        let maybe_garbage_in_invetory = robot.get_backpack().get_contents().get(&Content::Garbage(0).to_default());
        let garbage_in_invetory = *maybe_garbage_in_invetory.unwrap_or(&0);
        let energy = robot.get_energy().get_energy_level();

        let mut s = ShoppingList::new(vec![(Content::Garbage(1), Some(OpActionInput::Destroy()))]);
        let a = get_best_action_to_element(robot, world, &mut s);
        let is_there_garbage = if a.is_none() { 1 } else { 0 } as f32;

        let mut s = ShoppingList::new(vec![(Content::Bin(0..0), Some(OpActionInput::Put(Content::Garbage(1), 1)))]);
        let a = get_best_action_to_element(robot, world, &mut s);
        let is_there_bin = if a.is_none() { 1 } else { 0 } as f32;

        let is_inv_full = if robot.get_backpack().get_size() - robot.get_backpack().get_contents().values().sum::<usize>() == 0 { 1 } else { 0 } as f32;


        let res = self.network.propagate(vec![is_inv_full, is_there_garbage, is_there_bin]);

        let shifted_values: Vec<f32> = res.iter().map(|&x| x + 1.0).collect();
        let dist = WeightedIndex::new(&shifted_values).unwrap();
        let mut rng = rand::thread_rng();
        let mut random_index = rng.sample(dist);

        // Return action based on the random index
        if is_inv_full > 0.1 && is_there_bin < 0.1 { random_index = 2 }
        random_index = 2;
        let brain_action = match random_index {
            0 => GetContent(Content::Garbage(1)),
            1 => PutContent(Content::Bin(0..0), Content::Garbage(1)),
            2 => Explore(),
            _ => panic!("Action not implemented"),
        };

        let a: Option<OpActionOutput>;
        match brain_action {
            GetContent(c) => {
                a = get_best_action_to_element(robot, world, &mut ShoppingList::new(vec![(c.clone(), Some(Destroy()))]));
                println!("GetContent, {:?}, {:?}", a, c);
            }
            PutContent(wheree, whatt) => {
                a = get_best_action_to_element(robot, world, &mut ShoppingList::new(vec![(wheree.clone(), Some(OpActionInput::Put(whatt.clone(), 100)))]));
                println!("PutContent, {:?}, {:?}, {:?}", a, wheree, whatt);
            }
            Explore() => {
                a = get_best_action_to_element(robot, world, &mut ShoppingList::new(vec![(Content::None, None)]));
                println!("Explore, {:?}", a);
            }
        }
        a
    }
}

pub enum BrainAction {
    GetContent(Content),
    PutContent(Content, Content),
    Explore(),
}

pub fn generate_generator(seed: u64) -> impl Generator {
    // println!("generated: ");

    let generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .alter_content_option(Content::Garbage(0), OxAgContentOptions {
            in_batches: true,
            is_present: true,
            max_radius: 4,
            max_spawn_number: 200,
            min_spawn_number: 50,
            percentage: 0.1,
            with_max_spawn_number: false,
        }).unwrap()
        .alter_content_option(Content::Bin(0..0), OxAgContentOptions {
            in_batches: false,
            is_present: true,
            max_radius: 4,
            max_spawn_number: 50,
            min_spawn_number: 10,
            percentage: 0.05,
            with_max_spawn_number: false,
        }).unwrap()
        .set_size(40)
        .set_with_info(false)
        .build()
        .unwrap();
    return generator;
}