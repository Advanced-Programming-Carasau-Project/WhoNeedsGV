use colored::Colorize;
use robotics_lib::interface::{discover_tiles, get_score, go, put, robot_map, teleport};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::tile::Content::Fire;
use robotics_lib::world::tile::TileType::ShallowWater;
use robotics_lib::world::World;
use rust_and_furious_dynamo::dynamo::Dynamo;
use rustici_planner::tool::{Action, Destination, Planner, PlannerError, PlannerResult};
use spyglass::spyglass::{Spyglass, SpyglassResult};
use spyglass::spyglass::SpyglassResult::{Failed, Paused, Stopped};
use std::ops::Range;
use ohcrab_collection::collection::{CollectTool, LibErrorExtended};
use rustici_planner::tool::Destination::Content as OtherContent;
use crate::{MirtoRobot, RobotMode};
impl MirtoRobot{
    pub fn delivery_content_to(&mut self, world: &mut World, content: Content, dest_content: Content){
        let n_content = *self.get_backpack().get_contents().get(&content).unwrap();
        if n_content > 0{
            let destination = Destination::go_to_content(dest_content.clone());
            let result = Planner::planner(self, destination, world);
            match result {
                Ok(p) => {
                    match p {
                        PlannerResult::Path(mut v) => {
                            let last_move = v.0.pop().unwrap();
                            for i in 0..v.0.len(){
                                *self.get_energy_mut() = Dynamo::update_energy();
                                match &v.0[i] {
                                    Action::Move(d) => {
                                        go(self, world, d.clone());
                                    },
                                    Action::Teleport((i, j)) => {
                                        teleport(self, world, (*i, *j));
                                    },
                                }
                            }
                            match last_move {
                                Action::Move(d) => {
                                    *self.get_energy_mut() = Dynamo::update_energy();
                                    println!("{}, {:?} messo in {:?}: {:?}", n_content, content.clone(), dest_content.clone(), put(self, world, content.clone(), n_content, d));
                                }
                                Action::Teleport(_) => {}
                            }
                            self.mode = RobotMode::Delivery_Trees;
                        },
                        _ => {}
                    }
                }
                Err(e) => {
                    self.mode = RobotMode::Explore_Map;
                    println!("e: {:?}", e);
                }
            }
        }
        else {
            self.mode = RobotMode::Search_Trees;
        }
    }

    pub fn delivery_trees(&mut self, world: &mut World){
        self.delivery_content_to(world, Content::Tree(0), Content::Crate(0..20));
        if self.mode == RobotMode::Explore_Map{ //se non hai trovato casse in cui posizionare gli alberi
            self.delivery_content_to(world, Content::Tree(0), Content::Market(20)); //prova a posizionare gli alberi al market
            if self.mode == RobotMode::Delivery_Trees{ //se hai posizionato con successo gli alberi nel market hai ottenuto delle monete
                println!("\nmode: {:?} - energy: {} - score: {} - backpack_content: {:?}", self.mode, self.get_energy().get_energy_level(), get_score(world), self.robot.backpack.get_contents());
                self.delivery_content_to(world, Content::Coin(0), Content::Bank(0..50));
                self.empty_your_backpack(world);
            }
        }
    }
    pub fn search_tress_to_delivery(&mut self, world: &mut World){
        println!("\nmode: {:?} - energy: {} - score: {} - backpack_content: {:?}", self.mode, self.get_energy().get_energy_level(), get_score(world), self.robot.backpack.get_contents());
        let search_content = Content::Tree(1);
        let mut result = CollectTool::collect_content(self, world, &search_content, 20, self.robot.energy.get_energy_level());
        match result {
            Ok(n) => {
                println!("collected: {}", n);
            }
            Err(e) => {
                match e {
                    LibErrorExtended::CommonError(e) => {
                        println!("e: {:?}", e);
                        self.mode = RobotMode::Delivery_Trees;
                    }
                    LibErrorExtended::NoSolution => { self.mode = RobotMode::Explore_Map; }
                    LibErrorExtended::RobotMapEmpty => { self.mode = RobotMode::Explore_Map; }
                    LibErrorExtended::NoWalkableTile => { self.mode = RobotMode::Explore_Map; }
                    LibErrorExtended::EnergyOutOfLimit => {}
                }
            }
        }
    }

    pub fn explore_map_as_woodworker(&mut self, world: &mut World){
        if !self.used_spyglass {
            let map_size = robot_map(world).unwrap().len();
            let mut spyglass = Spyglass::new(
                self.get_coordinate().get_row(), // center_row
                self.get_coordinate().get_col(), // center_col
                map_size/3, // distance
                map_size, // world_dim
                Some(self.get_energy().get_energy_level()), // energy_budget
                true, // enable_cross
                1.0, // view_threshold
                |_| false,
            );
            loop {
                *self.get_energy_mut() = Dynamo::update_energy();
                spyglass.set_energy_budget(Some(self.get_energy().get_energy_level()));

                match spyglass.new_discover(self, world) {
                    SpyglassResult::Complete => { break; }
                    Stopped(_) => { break; }
                    SpyglassResult::Paused => {
                        loop {
                            *self.get_energy_mut() = Dynamo::update_energy();
                            match spyglass.resume_discover(self, world) {
                                SpyglassResult::Complete => { break; }
                                Stopped(_) => { break; }
                                Paused => {}
                                Failed(f) => { break; }
                            }
                        }
                    }
                    Failed(_) => { break; }
                }
            }
            self.used_spyglass = true;
        }
        else{
            let map_size = robot_map(world).unwrap().len();
            let destination = Destination::explore(self.robot.energy.get_energy_level(), map_size);
            let result = Planner::planner(self, destination, world);
        }
        self.mode = RobotMode::Search_Trees;
    }

    pub fn make_next_thing_for_woodworker_goal(&mut self, world: &mut World){
        *self.get_energy_mut() = Dynamo::update_energy();

        if self.mode == RobotMode::Delivery_Trees{
            self.delivery_trees(world);
        }
        else if self.mode == RobotMode::Search_Trees{
            self.search_tress_to_delivery(world);
        }

        if self.mode == RobotMode::Explore_Map{
            self.explore_map_as_woodworker(world);
        }
    }
}