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
use crate::{MirtoRobot};
use queues::IsQueue;
use robotics_lib::event::events::Event;
use robotics_lib::world::tile::Content::Bush;

impl MirtoRobot{
    pub fn place_mirto(&mut self, world: &mut World){
        while self.do_u_have_this_content(Content::JollyBlock(0)) {
            let (d, i, j) = self.finds_the_nearest_content_not_on_fluids(world, Content::None).unwrap();
            if !(i == self.robot.coordinate.get_row() && j == self.robot.coordinate.get_col()) {
                //println!("coordinate robot: {:?}, coordinate da raggiungere: {:?}", self.robot.coordinate, (i, j));
                let destination = Destination::go_to_coordinate((i, j));
                let result = Planner::planner(self, destination, world).unwrap();
                //println!("{:?}", result);
                match result {
                    PlannerResult::Path(p) => {
                        for i in 0..p.0.len() {
                            self.recharge_all_energy();
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
            }
            /*println!("coordinate robot: {:?} - direction: {:?}", self.robot.coordinate, d);
            println!("{:?}", put(self, world, Content::JollyBlock(0), 1, d));*/
        }
    }

    pub fn craft_mirto(&mut self){
        let mut result;
        let mut cont_flag = true;
        while cont_flag{
            result = craft(self, Content::JollyBlock(1));
            //println!("{:?}", result);
            match result {
                Ok(_) => {}
                Err(e) => {
                    match e {
                        LibError::NotEnoughEnergy => {
                            cont_flag = false;
                        }
                        LibError::NotCraftable => {
                            cont_flag = false;
                        }
                        LibError::NotEnoughSpace(s) => {
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
    }

    pub fn make_next_thing_for_mirto_goal(&mut self, world: &mut World){
        let exists_empty_cell = self.finds_the_nearest_content_not_on_fluids(world, Content::None).is_some();
        if self.do_u_have_this_content(Content::JollyBlock(0)) && exists_empty_cell{
            println!("placing mirto around...");
            self.place_mirto(world);
        }
        else if self.finds_the_nearest_content_not_on_fluids(world, Content::Bush(0)).is_some() && exists_empty_cell{
            println!("search of bushes...");
            self.search_bushes_for_mirto(world);
            if self.do_u_have_this_content(Bush(0)){
                println!("crafting mirto...");
                self.craft_mirto();
            }
            else {
                println!("it was impossible to collect: Bush(1)");
                println!("explore...");
                self.explore_map(world)
            }
        }
        else {
            println!("explore...");
            self.explore_map(world)
        }
    }
}