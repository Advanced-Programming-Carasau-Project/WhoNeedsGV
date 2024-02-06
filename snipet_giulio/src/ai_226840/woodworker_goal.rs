use colored::Colorize;
use robotics_lib::interface::{discover_tiles, get_score, go, put, robot_map, teleport};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::tile::Content::{Bank, Bin, Bush, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree, Water};
use robotics_lib::world::tile::TileType::ShallowWater;
use robotics_lib::world::World;
use rust_and_furious_dynamo::dynamo::Dynamo;
use rustici_planner::tool::{Action, Destination, Planner, PlannerError, PlannerResult};
use spyglass::spyglass::{Spyglass, SpyglassResult};
use spyglass::spyglass::SpyglassResult::{Failed, Paused, Stopped};
use std::ops::Range;
use ohcrab_collection::collection::{CollectTool, LibErrorExtended};
use rustici_planner::tool::Destination::Content as OtherContent;
use crate::{MirtoRobot};
impl MirtoRobot{
    pub fn delivery_content_to(&mut self, world: &mut World, content: Content, dest_content: Content){
        let n_content = *self.get_backpack().get_contents().get(&content).unwrap();
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
                                // println!("{}, {:?} messo in {:?}: {:?}", n_content, content.clone(), dest_content.clone(), put(self, world, content.clone(), n_content, d));
                            }
                            Action::Teleport(_) => {}
                        }
                    },
                    _ => {}
                }
            },
            Err(e) => { }
        }
    }

    pub fn collect_and_delivery_content(&mut self, world: &mut World, content: Content, quantity: usize, dest_content: Content){
        *self.get_energy_mut() = Dynamo::update_energy();
        let mut result = CollectTool::collect_content(self, world, &content, quantity, self.robot.energy.get_energy_level());
        // println!("result: {:?}", result);
        let mut new_content = Content::None;
        match content {
            Tree(_) => { new_content = Content::Tree(0) }
            Coin(_) => { new_content = Content::Coin(0) }
            _ => {}
        }
        // println!("new_content: {}", new_content);
        self.delivery_content_to(world, new_content, dest_content);
    }

    pub fn make_next_thing_for_woodworker_goal(&mut self, world: &mut World){
        // println!("svuotando lo zaino ...");
        self.empty_your_backpack(world); //svuota il tuo zaino per non avere problemi
        // println!("number: {}", self.get_backpack_objects_number());
        if self.get_backpack_objects_number() == 0 && self.finds_the_nearest_content_not_on_fluids(world, Content::Tree(0)).is_some() && self.found_content(world, Content::Crate(0..20)){
            // println!("CONSEGNA DI ALBERI IN CASSE");
            self.collect_and_delivery_content(world, Content::Tree(1), 20, Crate(0..20));
        }
        else if self.get_backpack_objects_number() == 0 && self.finds_the_nearest_content_not_on_fluids(world, Content::Tree(0)).is_some() && self.found_content(world, Content::Market(20)){
            // println!("CONSEGNA ALBERI AL MARKET");
            self.collect_and_delivery_content(world, Content::Tree(1), 10, Market(20));
            if self.found_content(world, Content::Bank(0..50)) {
                // println!("CONSEGNA MONETE IN BANCA");
                self.collect_and_delivery_content(world, Content::Coin(1), 20, Bank(0..50));
            }
        }
        else if self.get_backpack_objects_number() == 0 && self.finds_the_nearest_content_not_on_fluids(world, Content::Coin(0)).is_some() && self.found_content(world, Content::Bank(0..50)){
            // println!("CONSEGNA MONETE IN BANCA");
            self.collect_and_delivery_content(world, Content::Coin(1), 20, Bank(0..50));
        }
        else {
            // println!("ESPLORAZIONE");
            self.explore_map(world);
        }
    }
}