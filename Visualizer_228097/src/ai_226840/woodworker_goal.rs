use robotics_lib::interface::{go, teleport};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content};
use robotics_lib::world::tile::Content::{Bank, Coin, Crate, Market, Tree};
use robotics_lib::world::World;
use rustici_planner::tool::{Action, Destination, Planner, PlannerResult};
use ohcrab_collection::collection::{CollectTool};
use crate::ai_226840::MirtoRobot;

impl MirtoRobot{
    pub fn delivery_content_to(&mut self, world: &mut World, content: Content, dest_content: Content){
        let _n_content = *self.get_backpack().get_contents().get(&content).unwrap();
        let destination = Destination::go_to_content(dest_content.clone());
        let result = Planner::planner(self, destination, world);
        match result {
            Ok(p) => {
                match p {
                    PlannerResult::Path(mut v) => {
                        let last_move = v.0.pop().unwrap();
                        for i in 0..v.0.len(){
                            self.recharge_all_energy();
                            match &v.0[i] {
                                Action::Move(d) => {
                                    let _ = go(self, world, d.clone());
                                },
                                Action::Teleport((i, j)) => {
                                    let _ = teleport(self, world, (*i, *j));
                                },
                            }
                        }
                        match last_move {
                            Action::Move(_d) => {
                                self.recharge_all_energy();
                                //println!("{}, {:?} messo in {:?}: {:?}", n_content, content.clone(), dest_content.clone(), put(self, world, content.clone(), n_content, d));
                            }
                            Action::Teleport(_) => {}
                        }
                    },
                    _ => {}
                }
            },
            Err(_e) => { }
        }
    }

    pub fn collect_and_delivery_content(&mut self, world: &mut World, content: Content, quantity: usize, dest_content: Content){
        self.recharge_all_energy();
        let _result = CollectTool::collect_content(self, world, &content, quantity, self.robot.energy.get_energy_level());
        //println!("result: {:?}", result);
        let mut new_content = Content::None;
        match content {
            Tree(_) => { new_content = Content::Tree(0) }
            Coin(_) => { new_content = Content::Coin(0) }
            _ => {}
        }
        if self.do_u_have_this_content(new_content.clone()) {
            self.delivery_content_to(world, new_content, dest_content);
        }
        else {
            println!("it was impossible to collect: {:?}", content);
            println!("explore...");
            self.explore_map(world)
        }
    }

    pub fn make_next_thing_for_woodworker_goal(&mut self, world: &mut World){
        //println!("emptying the backpack...");
        self.empty_your_backpack(world);
        //println!("number of things inside the backpack: {}", self.get_backpack_objects_number());
        if self.get_backpack_objects_number() == 0 && self.finds_the_nearest_content_not_on_fluids(world, Content::Tree(0)).is_some() && self.found_content(world, Content::Crate(0..20)){
            //println!("delivery of trees in crates...");
            self.collect_and_delivery_content(world, Content::Tree(1), 20, Crate(0..20));
        }
        else if self.get_backpack_objects_number() == 0 && self.finds_the_nearest_content_not_on_fluids(world, Content::Tree(0)).is_some() && self.found_content(world, Content::Market(20)){
            //println!("delivery of trees to the market...");
            self.collect_and_delivery_content(world, Content::Tree(1), 10, Market(20));
            if self.do_u_have_this_content(Content::Coin(0)) && self.found_content(world, Content::Bank(0..50)) {
                //println!("delivery of coins to the bank...");
                self.collect_and_delivery_content(world, Content::Coin(1), 20, Bank(0..50));
            }
        }
        else if self.get_backpack_objects_number() == 0 && self.finds_the_nearest_content_not_on_fluids(world, Content::Coin(0)).is_some() && self.found_content(world, Content::Bank(0..50)){
            //println!("delivery of coins to the bank...");
            self.collect_and_delivery_content(world, Content::Coin(1), 20, Bank(0..50));
        }
        else {
            //println!("explore...");
            self.explore_map(world);
        }
    }
}