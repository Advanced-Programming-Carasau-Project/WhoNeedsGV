use colored::Colorize;
use ohcrab_collection::collection::{CollectTool, LibErrorExtended};
use queues::{Queue, queue};
use robotics_lib::interface::{craft, Direction, get_score, go, put, robot_map, teleport};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::tile::TileType::{DeepWater, Lava, ShallowWater, Teleport, Wall};
use robotics_lib::world::World;
use rust_and_furious_dynamo::dynamo::Dynamo;
use rustici_planner::tool::{Action, Destination, Planner, PlannerResult};
use crate::{MirtoRobot, RobotMode};
use queues::IsQueue;

impl MirtoRobot{
    pub fn place_mirto(&mut self, world: &mut World){
        while self.mode == RobotMode::Place_Mirto {
            let n_jolly_block = self.get_backpack().get_contents().get(&Content::JollyBlock(0)).unwrap();
            if *n_jolly_block > 0 {
                match self.finds_the_nearest_content_not_on_fluids(world, Content::None) {
                    None => { self.mode = RobotMode::Explore_Map }
                    Some((d, i, j)) => {
                        if !(i == self.robot.coordinate.get_row() && j == self.robot.coordinate.get_col()) {
                            println!("coordinate robot: {:?}, coordinate da raggiungere: {:?}", self.robot.coordinate, (i, j));
                            let destination = Destination::go_to_coordinate((i, j));
                            let result = Planner::planner(self, destination, world);
                            println!("{:?}", result);
                            match result {
                                Ok(r) => {
                                    match &r {
                                        PlannerResult::Path(p) => {
                                            for i in 0..p.0.len() {
                                                *self.get_energy_mut() = Dynamo::update_energy();
                                                match &p.0[i] {
                                                    Action::Move(d) => {
                                                        go(self, world, d.clone());
                                                    },
                                                    Action::Teleport((i, j)) => {
                                                        teleport(self, world, (*i, *j));
                                                    },
                                                }
                                            }
                                        },
                                        _ => {}
                                    }
                                },
                                Err(e) => {
                                    self.mode = RobotMode::Explore_Map;
                                    println!("e: {:?}", e);
                                }
                            }
                        }
                        println!("coordinate robot: {:?} - direction: {:?}", self.robot.coordinate, d);
                        println!("{:?}", put(self, world, Content::JollyBlock(0), 1, d));
                    }
                }
            } else {
                self.mode = RobotMode::Search_Bushes;
            }
        }
    }

    pub fn craft_mirto(&mut self){
        let mut result;
        let mut cont_flag = true;
        while cont_flag{
            result = craft(self, Content::JollyBlock(1));
            println!("{:?}", result);
            match result {
                Ok(_) => {}
                Err(e) => {
                    match e {
                        LibError::NotEnoughEnergy => {
                            cont_flag = false;
                        }
                        LibError::NotCraftable => {
                            self.mode = RobotMode::Place_Mirto;
                            cont_flag = false;
                        }
                        LibError::NotEnoughSpace(s) => {
                            self.mode = RobotMode::Place_Mirto;
                            cont_flag = false;
                        }
                        _ => {  }
                    }
                }
            }
        }
    }
    pub fn search_bushes_for_mirto(&mut self,  world: &mut World){
        let search_content = Content::Bush(1);
        let mut result = CollectTool::collect_content(self, world, &search_content, 20, self.robot.energy.get_energy_level());
        match result {
            Ok(n) => {
                println!("collected: {}", n);
            }
            Err(e) => {
                match e {
                    LibErrorExtended::CommonError(e) => {
                        println!("e: {:?}", e);
                        self.mode = RobotMode::Craft_Mirto;
                    }
                    LibErrorExtended::NoSolution => { self.mode = RobotMode::Explore_Map; }
                    LibErrorExtended::RobotMapEmpty => { self.mode = RobotMode::Explore_Map; }
                    LibErrorExtended::NoWalkableTile => { self.mode = RobotMode::Explore_Map; }
                    LibErrorExtended::EnergyOutOfLimit => {}
                }
            }
        }
    }

    pub fn explore_map_for_mirto(&mut self, world: &mut World){
        let map_size = robot_map(world).unwrap().len();
        let destination = Destination::explore(self.robot.energy.get_energy_level(), map_size);
        let result = Planner::planner(self, destination, world);
        self.mode = RobotMode::Search_Bushes;
    }

    pub fn make_next_thing_for_mirto_goal(&mut self, world: &mut World){
        if self.mode == RobotMode::Place_Mirto { //piazza il mirto in giro per il mondo
            self.place_mirto(world);
        }
        else if self.mode == RobotMode::Craft_Mirto { //crafta il mirto
            self.craft_mirto();
        }
        else if self.mode == RobotMode::Search_Bushes { //cerca la roba per craftare il mirto, nella mappa che conosci
            self.search_bushes_for_mirto(world);
        }
        if self.mode == RobotMode::Explore_Map { //vai in giro per la mappa in maniera casuale
            self.explore_map_for_mirto(world);
        }
    }
}