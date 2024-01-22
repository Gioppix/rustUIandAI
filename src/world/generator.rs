// use rand::Rng;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{robot_map, Tools, where_am_i};
#[allow(unused_imports)]
use robotics_lib::interface::{craft, debug, destroy, go, look_at_sky, teleport, Direction, robot_view};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::{Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree};
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street, Teleport,
};
#[allow(unused_imports)]
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;
use std::collections::HashMap;
use oxagworldgenerator::world_generator::content_options::OxAgContentOptions;
use rand::Rng;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::tile_type_options::OxAgTileTypeOptions;
use worldgen_unwrap::public::WorldgeneratorUnwrap;
use crate::AI::training::generate_generator;

pub struct WorldGenerator {
    size: usize,
}

impl WorldGenerator {
    pub fn init(size: usize) -> Self {
        WorldGenerator { size }
    }
}

impl Generator for WorldGenerator {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {
        const SEED: u64 = 421;

        let temp = generate_generator(SEED).gen();

        return temp;
    }
}

