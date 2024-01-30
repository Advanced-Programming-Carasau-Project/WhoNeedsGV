mod mirto_goal;
mod woodworker_goal;

use rand::thread_rng;
use std::collections::HashMap;
use robotics_lib::interface::{craft, Direction, get_score, put, teleport, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::World;
use robotics_lib::interface::where_am_i;
use robotics_lib::interface::robot_view;
use robotics_lib::interface::robot_map;
use robotics_lib::interface::go;
use robotics_lib::world::tile::Tile;
use robotics_lib::utils::LibError;
use robotics_lib::runner::{Robot};
use robotics_lib::event::events::Event;
use robotics_lib::energy::Energy;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::runner::Runner;
use rand::Rng;
use rip_worldgenerator::MyWorldGen;
use rust_and_furious_dynamo::dynamo::Dynamo;
use rustici_planner::tool::{Action, Destination, PlannerError, PlannerResult};
use rustici_planner::tool::Planner;
use std::thread;
use std::time::Duration;
use ohcrab_collection::collection::{CollectTool, LibErrorExtended};
use op_map::op_pathfinding::{get_best_action_to_element, ShoppingList};
use op_map::op_pathfinding::OpActionInput::Put;
use robotics_lib::world::tile::TileType::{DeepWater, Lava, ShallowWater, Teleport, Wall};
use queues::queue;
use queues::Queue;
use queues::IsQueue;
use colored::Colorize;
use oxagaudiotool::error::error::OxAgAudioToolError;
use oxagaudiotool::OxAgAudioTool;
use oxagaudiotool::sound_config::OxAgSoundConfig;
use ohcrab_weather::weather_tool::WeatherPredictionTool;
use robotics_lib::world::tile::Content::{Bush, Fire, JollyBlock, Tree, Water};
use spyglass::spyglass::Spyglass;

#[derive(Debug, PartialEq)]
enum RobotMode{
    // modalità condivisa tra le due missioni del robot
    Explore_Map,

    // modalità della missione del robot di distribuire mirto
    Place_Mirto,
    Craft_Mirto,
    Search_Bushes,

    // modalityà della missione del robot di svolgere il proprio lavoro
    Search_Trees,
    Delivery_Trees,
}

struct MirtoRobot {
    robot: Robot,
    mode: RobotMode,
    audio_tool: OxAgAudioTool,
    weather_prediction_tool: WeatherPredictionTool,
    tickets_to_wait: usize,
    tickets: usize,
    is_the_goal_woodworking: bool,
    used_spyglass: bool,
}

impl MirtoRobot {
    pub fn new(robot: Robot, mode: RobotMode, is_the_goal_woodworking: bool) -> Self{
        MirtoRobot {
            robot,
            mode,
            audio_tool: OxAgAudioTool::new(HashMap::new(), HashMap::new(), HashMap::new()).unwrap(),
            weather_prediction_tool:  WeatherPredictionTool::new(),
            tickets_to_wait: 8,
            tickets: 0,
            is_the_goal_woodworking,
            used_spyglass: false,
        }
    }

    pub fn print_robot_debug(&self, world: &World){
        let map = robot_map(world).unwrap();
        let i_robot = self.robot.coordinate.get_row();
        let j_robot = self.robot.coordinate.get_col();
        let size = map.len();

        println!("\nmode: {:?} - energy: {} - score: {} - backpack_content: {:?}", self.mode, self.get_energy().get_energy_level(), get_score(world), self.robot.backpack.get_contents());
        for i in 0..size{
            for j in 0..size{
                if i == i_robot && j == j_robot{
                    print!("{}", "R".bright_yellow());
                }
                else if let Some(tile) = &map[i][j]{
                    if tile.tile_type == TileType::DeepWater || tile.tile_type == TileType::Lava {
                        print!("{}", "~".blue());
                    }
                    else if tile.tile_type == TileType::Teleport(true){
                        print!("{}", "^".red());
                    }
                    else if tile.tile_type == TileType::Wall{
                        print!("{}", "#".black());
                    }
                    else{
                        match &tile.content {
                            Content::Tree(_) => { print!("t"); }
                            Content::Bush(_) => { print!("b"); }
                            Content::Coin(_) => { print!("c"); }
                            Content::Bank(r) => {
                                if r.start != r.end {
                                    print!("{}", "B".bright_green());
                                }
                                else {
                                    print!("{}", "-".green());
                                }
                            }
                            Content::JollyBlock(_) => { print!("{}", "J".bright_red()); }
                            Content::Crate(r) => {
                                if r.start != r.end {
                                    print!("{}", "C".bright_green());
                                }
                                else {
                                    print!("{}", "-".green());
                                }
                            }
                            Content::Market(s) => {
                                if *s > 0 {
                                    print!("{}", "M".bright_blue());
                                }
                                else {
                                    print!("{}", "-".green());
                                }
                            }
                            _ => { print!("{}", "-".green()); }
                        }
                    }
                }
                else {
                    print!("?");
                }
            }
            println!("");
        }
    }

    pub fn is_point_inside_map(i: i32, j: i32, size: i32) -> bool{
        if i >= 0 && i < size && j >= 0 && j < size{
            true
        }
        else {
            false
        }
    }

    pub fn finds_the_nearest_content_not_on_fluids(&self, world: &World, content: Content) -> Option<(Direction, usize, usize)>{
        let map = robot_map(world).unwrap();
        let size = map.len();
        let i_robot = self.robot.coordinate.get_row();
        let j_robot = self.robot.coordinate.get_col();
        let mut visited = vec![vec![false ; size]; size];
        let mut queue: Queue<(usize, usize)> = queue![];

        for i in 0..size{
            for j in 0..size{
                match &map[i][j] {
                    None => {
                        visited[i][j] = true;
                    }
                    Some(t) => {
                        if t.tile_type == DeepWater || t.tile_type == ShallowWater || t.tile_type == Lava || t.tile_type == Wall || t.tile_type == Teleport(true){
                            visited[i][j] = true;
                        }
                    }
                }
            }
        }

        queue.add((i_robot, j_robot));

        while queue.size() != 0 {
            let (i, j) = queue.remove().unwrap();
            if Self::is_point_inside_map((i as i32 -1) , j as i32, size as i32) && !visited[i-1][j] {
                visited[i-1][j] = true;
                if let Some(tile) = &map[i-1][j]{
                    if tile.content == content {
                        return Some((Direction::Up, i, j));
                    }
                }
                queue.add((i-1, j));
            }
            if Self::is_point_inside_map((i as i32 + 1) , j as i32, size as i32) && !visited[i+1][j] {
                visited[i+1][j] = true;
                if let Some(tile) = &map[i+1][j]{
                    if tile.content == content {
                        return Some((Direction::Down, i, j));
                    }
                }
                queue.add((i+1, j));
            }
            if Self::is_point_inside_map(i as i32, (j as i32 -1) , size as i32) && !visited[i][j-1] {
                visited[i][j-1] = true;
                if let Some(tile) = &map[i][j-1]{
                    if tile.content == content {
                        return Some((Direction::Left, i, j));
                    }
                }
                queue.add((i, j-1));
            }
            if Self::is_point_inside_map(i as i32, (j as i32 +1) , size as i32) && !visited[i][j+1] {
                visited[i][j+1] = true;
                if let Some(tile) = &map[i][j+1]{
                    if tile.content == content{
                        return Some((Direction::Right, i, j));
                    }
                }
                queue.add((i, j+1));
            }
        }

        None
    }

    pub fn get_backpack_objects_number(&mut self) -> usize{
        let mut size = 0;
        let back_pack_contents = self.robot.backpack.get_contents();
        for (content, quantity) in back_pack_contents{
            size = size + quantity;
        }
        size
    }

    pub fn empty_content_around(&mut self, world: &mut World) -> Option<Vec<Direction>>{
        let around = where_am_i(self, world).0;
        let mut vec = vec![];
        match &around[1][0]{
            None => {},
            Some(t) => {
                if t.content == Content::None && Self::is_a_valid_tyle(t.tile_type.clone()) && t.tile_type != Teleport(true){
                    vec.push(Direction::Left);
                }
            }
        }
        match &around[0][1] {
            None => {},
            Some(t) => {
                if t.content == Content::None && Self::is_a_valid_tyle(t.tile_type.clone()) && t.tile_type != Teleport(true){
                    vec.push(Direction::Up);
                }
            }
        }
        match &around[1][2] {
            None => {},
            Some(t) => {
                if t.content == Content::None && Self::is_a_valid_tyle(t.tile_type.clone()) && t.tile_type != Teleport(true) {
                    vec.push(Direction::Right);
                }
            }
        }
        match &around[2][1] {
            None => {},
            Some(t) => {
                if t.content == Content::None && Self::is_a_valid_tyle(t.tile_type.clone()) && t.tile_type != Teleport(true) {
                    vec.push(Direction::Down);
                }
            }
        }
        if vec.len() == 0{
            None
        }
        else {
            Some(vec)
        }
    }

    pub fn insert_objects_around(&mut self, world: &mut World){
        let back_pack_contents = self.robot.backpack.get_contents().clone();
        for (content, quantity) in back_pack_contents{
            if quantity > 0 {
                let direction_to_put = self.empty_content_around(world);
                match direction_to_put {
                    None => {}
                    Some(v) => {
                        println!("directions: {:?}", v);
                        for i in 0..v.len(){
                            match put(self, world, content.clone(), quantity, v[i].clone()) {
                                Err(_) => {},
                                Ok(_) => {},
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn is_a_valid_tyle(t: TileType) -> bool{
        if t == Wall || t == DeepWater || t == Lava{
            return false;
        }
        return true;
    }
    pub fn empty_your_backpack_with_a_walk(&mut self, world: &mut World, visited: &mut Vec<Vec<bool>>){
        *self.get_energy_mut() = Dynamo::update_energy();

        let map = robot_map(world).unwrap();
        let i_robot = self.robot.coordinate.get_row();
        let j_robot = self.robot.coordinate.get_col();

        self.insert_objects_around(world);

        if self.get_backpack_objects_number() > 0 && Self::is_point_inside_map(i_robot as i32 - 1, j_robot as i32, map.len() as i32)  && !visited[i_robot - 1][j_robot] {
            visited[i_robot - 1][j_robot] = true;
            go(self, world, Direction::Up);
            self.empty_your_backpack_with_a_walk(world, visited);
            go(self, world, Direction::Down);
        }
        if self.get_backpack_objects_number() > 0 && Self::is_point_inside_map(i_robot as i32 + 1, j_robot as i32, map.len() as i32)  && !visited[i_robot + 1][j_robot] {
            visited[i_robot + 1][j_robot] = true;
            go(self, world, Direction::Down);
            self.empty_your_backpack_with_a_walk(world, visited);
            go(self, world, Direction::Up);
        }
        if self.get_backpack_objects_number() > 0 && Self::is_point_inside_map(i_robot as i32, j_robot as i32 - 1 , map.len() as i32)  && !visited[i_robot][j_robot-1]{
            visited[i_robot][j_robot - 1] = true;
            go(self, world, Direction::Left);
            self.empty_your_backpack_with_a_walk(world, visited);
            go(self, world, Direction::Right);
        }
        if self.get_backpack_objects_number() > 0 && Self::is_point_inside_map(i_robot as i32, j_robot as i32 + 1 , map.len() as i32)  && !visited[i_robot][j_robot+1] {
            visited[i_robot][j_robot + 1] = true;
            go(self, world, Direction::Right);
            self.empty_your_backpack_with_a_walk(world, visited);
            go(self, world, Direction::Left);
        }
    }

    pub fn empty_your_backpack(&mut self, world: &mut World){
        let map = robot_map(world).unwrap();
        let map_size = map.len();
        let mut visited = vec![vec![false; map_size] ; map_size];
        visited[self.robot.coordinate.get_row()][self.robot.coordinate.get_col()] = true;
        for i in 0..map_size{
            for j in 0..map_size{
                match &map[i][j] {
                    None => {
                        visited[i][j] = true;
                    },
                    Some(t) => {
                        if !Self::is_a_valid_tyle(t.tile_type.clone()) || t.tile_type == ShallowWater{
                            visited[i][j] = true;
                        }
                    }
                }
            }
        }
        self.empty_your_backpack_with_a_walk(world, &mut visited);
    }

    pub fn make_next_thing(&mut self, world: &mut World){
        println!("TICKS CONT: {}", self.tickets);
        self.tickets = self.tickets + 1;
        if self.tickets == self.tickets_to_wait{
            self.tickets_to_wait = thread_rng().gen_range(7..=12);
            self.tickets = 0;
            let mut is_the_new_goal_woodworking;
            match self.weather_prediction_tool.predict(self.tickets_to_wait){
                Ok(w ) => {
                    println!("predicted_weather: {:?}", w);
                    match w {
                        WeatherType::Sunny => { is_the_new_goal_woodworking = true; }
                        WeatherType::Rainy => { is_the_new_goal_woodworking = false; }
                        WeatherType::Foggy => { is_the_new_goal_woodworking = true; }
                        WeatherType::TropicalMonsoon => { is_the_new_goal_woodworking = true; }
                        WeatherType::TrentinoSnow => { is_the_new_goal_woodworking = false; }
                    }
                },
                Err(e) => { is_the_new_goal_woodworking = false; },
            }
            if is_the_new_goal_woodworking != self.is_the_goal_woodworking{
                println!("SIUM");
                self.is_the_goal_woodworking = is_the_new_goal_woodworking;
                self.empty_your_backpack(world);
                self.mode = RobotMode::Explore_Map;
            }
        }

        *self.get_energy_mut() = Dynamo::update_energy();
        if self.is_the_goal_woodworking {
            self.make_next_thing_for_woodworker_goal(world);
        }
        else {
            self.make_next_thing_for_mirto_goal(world);
        }
    }
}

impl Runnable for MirtoRobot {
    fn process_tick(&mut self, world: &mut World) {
        self.make_next_thing(world);
        self.print_robot_debug(world);
    }
    fn handle_event(&mut self, event: Event) {
        self.audio_tool.play_audio_based_on_event(&event);
        self.weather_prediction_tool.process_event(&event);
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

fn main() {
        const world_size: usize = 100;

        let robot = MirtoRobot::new(Robot::new(), RobotMode::Explore_Map, true);

        let mut generator = MyWorldGen::new_param(world_size,2,5,0,true,false, 5);

        let mut run = Runner::new(Box::new(robot), &mut generator).unwrap(); //creo un runner (l'oggetto che gestisce i tick del mondo). Questa struc creerà il mondo grazie al world generator

        loop{
            run.game_tick(); //faccio avanzare un tick del mondo (un tick corrisponde all'unità elementare di tempo nel "gioco")
            thread::sleep(Duration::from_millis(5000));
        }
}