mod coin_collection;
mod scare_crow_killing;
mod print;

use std::collections::{HashMap, VecDeque};


use ohcrab_collection::collection::LibErrorExtended;
use ohcrab_weather::weather_tool::WeatherPredictionTool;
use op_map::op_pathfinding::OpActionOutput;

use oxagaudiotool::OxAgAudioTool;

use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{Direction, get_score, go, look_at_sky, put, robot_map, teleport};
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::DayTime::Night;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::{Content, TileType};

use robotics_lib::world::World;
use rust_and_furious_dynamo::dynamo::Dynamo;
use rustici_planner::tool::{Action, Destination, Planner, PlannerError, PlannerResult};
use spyglass::spyglass::{Spyglass, SpyglassResult};
use crate::{backpack_content, energy, events, points, positions, robot_view};


//checks if a certain tuple of coordinates is inside the map
pub fn valid_coords(x:i32, y:i32, size:i32) -> bool{
    if x >= size || x < 0 || y >= size || y < 0 {
        false
    }
    else{
        true
    }
}


pub struct LunaticRobot {
    pub robot: Robot,
    pub audio: OxAgAudioTool,
    pub lava_coords: Option<(usize, usize, Direction)>,
    pub weather_tool: WeatherPredictionTool,
    pub ticks: usize,
    pub must_empty: bool

}
impl LunaticRobot{
    pub fn new() -> LunaticRobot{
        LunaticRobot{
            robot: Robot::new(),
            audio: OxAgAudioTool::new(HashMap::new(),HashMap::new(),HashMap::new()).unwrap(),
            lava_coords: None,
            weather_tool: WeatherPredictionTool::new(),
            ticks: 0,
            must_empty: false
        }
    }
    //function that uses the Dynamo tool to give the max energy to the robot
    pub fn replenish(&mut self){
        *self.get_energy_mut() = Dynamo::update_energy();
        self.handle_event(Event::EnergyRecharged(1000));
    }
    //makes the robot explore the world as long as he has energy
    pub fn exploration(&mut self, _content: Content, world: &mut World){
        println!("spyglass exploration");
        let map = robot_map(world).unwrap();
        let map_size = map.len();
        let distance;
        if map_size < 64{
            distance = map_size/4;
        }else{
            distance = 30;
        }
        let mut spy_glass = Spyglass::new(
            self.get_coordinate().get_row(),
            self.get_coordinate().get_col(),
            distance,
            map_size,
            Some(self.get_energy().get_energy_level()),
            true,
            1.0,
            |_| false,
        );
        let spy_return = spy_glass.new_discover(self, world);
        self.spy_glass_error_handler(&spy_return);
        //TO BE CONTINUED
    }
    pub fn move_to_coords(&mut self, coords: (usize, usize), world: &mut World){
        let destination = Destination::go_to_coordinate(coords);
        let result = Planner::planner(self, destination, world);
        match result {
            Ok(p) => {
                match p{
                    PlannerResult::Path((actions,_cost)) => {
                        //TODO check on cost, and break the action in multiple ticks
                        for i in 0..actions.len(){
                            self.replenish();
                            match &actions[i]{
                                Action::Move(d) => {
                                    go(self, world, d.clone());
                                }
                                Action::Teleport(tile) => {
                                    teleport(self, world, *tile);
                                }
                            }
                        }
                    }
                    _ => {println!("planner path not a path")}
                }
            }
            Err(e) => { self.planner_error_handler(e) }
        }
    }
    pub fn spy_glass_error_handler(&mut self, result: &SpyglassResult){
        //PROVVISORIO
        match result{
            SpyglassResult::Complete(_) => {println!("SPYGLASS: Complete")}
            SpyglassResult::Stopped(_) => {println!("SPYGLASS: Stopped")}
            SpyglassResult::Paused => {println!("SPYGLASS: Paused")}
            SpyglassResult::Failed(x) => {println!("SPYGLASS: Failed with {:?}",x)}
        }
        //PROVVISORIO
    }
    pub fn planner_error_handler(&mut self, _error: PlannerError){
        //todo!()
    }
    pub fn is_content_available(&self, _content: Content){
        //todo!()
    }
    pub fn explore(&mut self, world: &mut World){
        println!("Free exploring...");
        let map_size = robot_map(world).unwrap().len();
        //decide what kind of planner i am going to use
        let destination = Destination::explore(self.robot.energy.get_energy_level(), map_size);
        let result = Planner::planner(self, destination, world);
        if let Err(r) = result{
            self.planner_error_handler(r);
        }
    }
    //returns the free space in the backpack
    pub fn get_remaining_backpack_space(&mut self) -> usize{
        let backpack_size = self.robot.backpack.get_size();
        let mut space = backpack_size;
        let contents = self.robot.backpack.get_contents();
        for (_content,quantity) in contents{
            space -= quantity;
        }
        if space < backpack_size/5{
            self.must_empty = true;
        }
        return space;
    }
    pub fn get_content_quantity(&mut self, content: &Content) -> usize{
        let quantity = self.robot.backpack.get_contents().get(&content).unwrap();
        return *quantity;
    }
    // puts the decided content on the desired tile
    pub fn put_content(&mut self, content: Content, world: &mut World){
        let any_usize = 50;
        let destination = Destination::go_to_content(Content::Bank(0..any_usize));
        let result = Planner::planner(self, destination, world);
        match result {
            Ok(p) => {
                match p{
                    PlannerResult::Path((mut actions,_cost)) => {
                        let last_move = actions.pop();
                        //TODO check on cost, and break the action in multiple ticks
                        for i in 0..actions.len(){
                            self.replenish();
                            match &actions[i]{
                                Action::Move(d) => {
                                    go(self, world, d.clone());
                                }
                                Action::Teleport(tile) => {
                                    teleport(self, world, *tile);
                                }
                            }
                        }
                        match last_move.unwrap(){
                            Action::Move(d) => {
                                let quantity = self.get_content_quantity(&content);
                                put(self, world, content, quantity, d);
                            }
                            Action::Teleport(_) => {}
                        }

                    }
                    _ => {println!("planner path not a path")}
                }
            }
            Err(e) => { self.planner_error_handler(e) }
        }
    }
    //handles the Result from the Collection tool for every method who calls the tool
    pub fn collection_result_handler(&mut self, _res: Result<usize, LibErrorExtended>){
        //todo!()
    }
    //moves the robot to a tile close to other undiscovered tiles
    pub fn move_to_unexplored_land(&mut self, world: &mut World){
        let target = self.find_closest_undiscovered_tile(world);
        if target.is_none(){
            return;
        }else {
            if let Some(tile_target) = target {
                let destination = Destination::go_to_coordinate(tile_target);
                let result = Planner::planner(self, destination, world);
                match result {
                    Ok(p) => {
                        match p{
                            PlannerResult::Path((actions,_cost)) => {
                                //TODO check on cost, and break the action in multiple ticks
                                for i in 0..actions.len(){
                                    self.replenish();
                                    match &actions[i]{
                                        Action::Move(d) => {
                                            go(self, world, d.clone());
                                        }
                                        Action::Teleport(tile) => {
                                            teleport(self, world, *tile);
                                        }
                                    }
                                }
                            }
                            _ => {println!("planner path not a path")}
                        }
                    }
                    Err(e) => { self.planner_error_handler(e) }
                }
            }
        }
        //todo!()
    }
    pub fn find_closest_undiscovered_tile(&mut self, world: &mut World) -> Option<(usize, usize)>{
        //range where we are currently searching for the undiscovered tile
        let _range = 2usize;

        let robot_x = self.get_coordinate().get_row();
        let robot_y = self.get_coordinate().get_col();
        //map as seen as the robot
        let known_map = robot_map(world).unwrap();
        let map_size = known_map.len() as i32;
        let mut flag = vec![vec![false; map_size as usize]; map_size as usize];
        let mut queue = VecDeque::new();
        for i in 0..flag.len() {
            for j in 0..flag[i].len() {
                match &known_map[i][j] {
                    None => {}
                    Some(t) => {
                        match t.tile_type {
                            TileType::Lava => { flag[i][j] = true }
                            TileType::DeepWater => { flag[i][j] = true }
                            TileType::Wall => { flag[i][j] = true }
                            _ => {}
                        }
                    }
                }
            }
        }
        //coordinates of the tile
        queue.push_back((robot_x, robot_y));
        flag[robot_x][robot_y] = true;

        while !queue.is_empty(){
            let (x_u,y_u) = queue.pop_front().unwrap();
            let x_i = x_u as i32;
            let y_i = y_u as i32;
            if valid_coords(x_i -1, y_i, map_size) && !flag[x_u -1][y_u]{
                if known_map[x_u -1][y_u].is_none(){
                    return Some((x_u, y_u))
                }
                queue.push_back((x_u-1, y_u));
                flag[x_u -1][y_u] = true;
            }
            if valid_coords(x_i +1, y_i, map_size) && !flag[x_u +1][y_u]{
                if known_map[x_u +1][y_u].is_none(){
                    return Some((x_u, y_u))
                }
                queue.push_back((x_u+1, y_u));
                flag[x_u +1][y_u] = true
            }
            if valid_coords(x_i, y_i-1, map_size) && !flag[x_u][y_u-1]{
                if known_map[x_u][y_u-1].is_none(){
                    return Some((x_u, y_u))
                }
                queue.push_back((x_u, y_u-1));
                flag[x_u][y_u-1] = true;
            }
            if valid_coords(x_i, y_i+1, map_size) && !flag[x_u][y_u+1]{
                if known_map[x_u][y_u+1].is_none(){
                    return Some((x_u, y_u))
                }
                queue.push_back((x_u, y_u+1));
                flag[x_u][y_u+1] = true;
            }

        }
        return None;
    }
    pub fn op_map_handler(&mut self, _return_value: Option<OpActionOutput>){
        //todo!()
    }
    //methods that dictates the actions the robot is going to make
    pub fn routine(&mut self, world: &mut World){
        self.ticks += 1;
        //the robot gets the max energy every tick
        self.replenish();
        self.exploration(Content::None, world);
        let environment = look_at_sky(world);
        println!("current weather: {:?}", environment.get_weather_condition());
        //depending on the time of the day the robot will have different behaviour
        if environment.get_time_of_day() != Night{
            println!("in day");
            // if it's sunny the robot won't collect coins, but he will simply have a walk (explore
            // the world)
            if environment.get_weather_condition() == Sunny{
                println!("in sunny day routine");
                self.explore(world);
            }else{
                println!("in day routine");
                self.day(world);
            }
        }else{
            println!("in night");
            self.night(world);
        }
    }
}
impl Runnable for LunaticRobot {
    fn process_tick(&mut self, world: &mut World) {
        self.routine(world);

        let mut update_points = points.lock().unwrap();
        let mut update_robot_view = robot_view.lock().unwrap();
        let mut update_positions = positions.lock().unwrap();
        let mut update_energy = energy.lock().unwrap();
        let mut update_backpack_content = backpack_content.lock().unwrap();

        *update_positions = (self.robot.coordinate.get_row(), self.robot.coordinate.get_col());
        *update_points = get_score(world);
        *update_robot_view = robot_map(world).unwrap();
        *update_energy = self.robot.energy.get_energy_level();
        *update_backpack_content = self.get_backpack().get_contents().clone();
    }
    fn handle_event(&mut self, event: Event) {

        let mut update_events = events.lock().unwrap();
        update_events.push(event.clone());
    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate{
        &mut self.robot.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}

#[cfg(test)]
mod tests{

}
