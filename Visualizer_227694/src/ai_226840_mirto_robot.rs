


use std::collections::HashMap;
use rand::thread_rng;
use robotics_lib::interface::{Direction, get_score, put};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::World;
use robotics_lib::interface::where_am_i;

use robotics_lib::interface::robot_map;
use robotics_lib::interface::go;


use robotics_lib::runner::{Robot};
use robotics_lib::event::events::Event;





use robotics_lib::world::environmental_conditions::WeatherType;

use rand::Rng;

use rust_and_furious_dynamo::dynamo::Dynamo;
use rustici_planner::tool::{Destination};
use rustici_planner::tool::Planner;





use robotics_lib::world::tile::TileType::{DeepWater, Lava, Teleport, Wall};
use queues::queue;
use queues::Queue;
use queues::IsQueue;
use colored::Colorize;

use oxagaudiotool::OxAgAudioTool;
use oxagaudiotool::sound_config::OxAgSoundConfig;
use ohcrab_weather::weather_tool::WeatherPredictionTool;

use spyglass::spyglass::{Spyglass, SpyglassResult};
use spyglass::spyglass::SpyglassResult::{Failed, Paused, Stopped};

pub struct MirtoRobot {
    pub robot: Robot,
    pub audio_tool: OxAgAudioTool,
    pub weather_prediction_tool: WeatherPredictionTool,
    pub tickets_to_wait: usize,
    pub tickets: usize,
    pub is_the_goal_woodworking: bool,
    pub used_spyglass: bool,
}

impl MirtoRobot {
    pub fn new(robot: Robot, is_the_goal_woodworking: bool) -> Self{
        MirtoRobot {
            robot,
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

        println!("woodworking: {} - energy: {} - score: {} - backpack_content: {:?}\n", self.is_the_goal_woodworking, self.get_energy().get_energy_level(), get_score(world), self.robot.backpack.get_contents());
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
                                    print!("{}", "B".bright_red());
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

    fn map_audio_with_event() -> HashMap<Event, OxAgSoundConfig>{
        let mut mapping = HashMap::new();

        mapping.insert(Event::Ready, OxAgSoundConfig::new("./sounds/ready.mp3"));
        mapping.insert(Event::Terminated, OxAgSoundConfig::new("./sounds/terminated.mp3"));
        for i in 1..=20{
            mapping.insert(Event::AddedToBackpack(Content::Coin(0), i), OxAgSoundConfig::new("./sounds/added_to_backpack.mp3"));
            mapping.insert(Event::AddedToBackpack(Content::Tree(0), i), OxAgSoundConfig::new("./sounds/added_to_backpack.mp3"));

            mapping.insert(Event::RemovedFromBackpack(Content::Coin(0), i), OxAgSoundConfig::new("./sounds/removed_from_backpack.mp3"));
            mapping.insert(Event::RemovedFromBackpack(Content::Tree(0), i), OxAgSoundConfig::new("./sounds/removed_from_backpack.mp3"));
            mapping.insert(Event::RemovedFromBackpack(Content::Bush(0), i), OxAgSoundConfig::new("./sounds/removed_from_backpack.mp3"));
            mapping.insert(Event::RemovedFromBackpack(Content::JollyBlock(0), i), OxAgSoundConfig::new("./sounds/removed_from_backpack.mp3"));
        }

        mapping.insert(Event::EnergyRecharged(10), OxAgSoundConfig::new("./sounds/energy_recharged.mp3"));


        mapping
    }

    fn map_audio_with_weather() -> HashMap<WeatherType, OxAgSoundConfig>{
        let mut mapping = HashMap::new();

        mapping.insert(WeatherType::Sunny, OxAgSoundConfig::new_looped("./sounds/weathertype_sunny.mp3"));
        mapping.insert(WeatherType::Rainy, OxAgSoundConfig::new_looped("./sounds/weathertype_rainy.mp3"));
        mapping.insert(WeatherType::Foggy, OxAgSoundConfig::new_looped_with_volume("./sounds/weathertype_foggy.mp3", 1.5));
        mapping.insert(WeatherType::TrentinoSnow, OxAgSoundConfig::new_looped("./sounds/weathertype_trentino_snow.mp3"));
        mapping.insert(WeatherType::TropicalMonsoon, OxAgSoundConfig::new_looped("./sounds/weathertype_tropical_monsoon.mp3"));

        mapping
    }

    fn map_audio_with_tile() -> HashMap<TileType, OxAgSoundConfig>{
        let mut mapping = HashMap::new();

        mapping.insert(TileType::Teleport(true), OxAgSoundConfig::new("./sounds/tile_teleport.mp3"));
        mapping.insert(TileType::Street, OxAgSoundConfig::new("./sounds/tile_street.mp3"));
        mapping.insert(TileType::Grass, OxAgSoundConfig::new("./sounds/tile_grass.mp3"));
        mapping.insert(TileType::Snow, OxAgSoundConfig::new("./sounds/tile_snow.mp3"));
        mapping.insert(TileType::Sand, OxAgSoundConfig::new("./sounds/tile_sand.mp3"));
        mapping.insert(TileType::Hill, OxAgSoundConfig::new("./sounds/tile_grass.mp3"));
        mapping.insert(TileType::Mountain, OxAgSoundConfig::new("./sounds/tile_mountain.mp3"));

        mapping
    }

    pub fn is_point_inside_map(i: i32, j: i32, size: i32) -> bool{
        if i >= 0 && i < size && j >= 0 && j < size{
            true
        }
        else {
            false
        }
    }

    pub fn found_content(&mut self, world: &mut World, content: Content) -> bool{
        let destination = Destination::go_to_content(content);
        let result = Planner::planner(self, destination, world);
        match result {
            Ok(_) => { true }
            Err(_) => { false }
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
                        if !Self::is_a_valid_tyle(t.tile_type.clone()) || t.tile_type == Teleport(true){
                            visited[i][j] = true;
                        }
                    }
                }
            }
        }

        queue.add((i_robot, j_robot));

        while queue.size() != 0 {
            let (i, j) = queue.remove().unwrap();
            if Self::is_point_inside_map(i as i32 -1 , j as i32, size as i32) && !visited[i-1][j] {
                visited[i-1][j] = true;
                if let Some(tile) = &map[i-1][j]{
                    if std::mem::discriminant(&tile.content) == std::mem::discriminant(&content) {
                        return Some((Direction::Up, i, j));
                    }
                }
                queue.add((i-1, j));
            }
            if Self::is_point_inside_map(i as i32 + 1 , j as i32, size as i32) && !visited[i+1][j] {
                visited[i+1][j] = true;
                if let Some(tile) = &map[i+1][j]{
                    if std::mem::discriminant(&tile.content) == std::mem::discriminant(&content) {
                        return Some((Direction::Down, i, j));
                    }
                }
                queue.add((i+1, j));
            }
            if Self::is_point_inside_map(i as i32, j as i32 -1 , size as i32) && !visited[i][j-1] {
                visited[i][j-1] = true;
                if let Some(tile) = &map[i][j-1]{
                    if std::mem::discriminant(&tile.content) == std::mem::discriminant(&content) {
                        return Some((Direction::Left, i, j));
                    }
                }
                queue.add((i, j-1));
            }
            if Self::is_point_inside_map(i as i32, j as i32 +1 , size as i32) && !visited[i][j+1] {
                visited[i][j+1] = true;
                if let Some(tile) = &map[i][j+1]{
                    if std::mem::discriminant(&tile.content) == std::mem::discriminant(&content) {
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
        for (_content, quantity) in back_pack_contents{
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
        self.handle_event(Event::EnergyRecharged(1000));

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
                        if !Self::is_a_valid_tyle(t.tile_type.clone()){
                            visited[i][j] = true;
                        }
                    }
                }
            }
        }
        self.empty_your_backpack_with_a_walk(world, &mut visited);
    }

    pub fn explore_map(&mut self, world: &mut World){
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
                self.handle_event(Event::EnergyRecharged(1000));
                spyglass.set_energy_budget(Some(self.get_energy().get_energy_level()));

                match spyglass.new_discover(self, world) {
                    SpyglassResult::Complete => { break; }
                    Stopped(_) => { break; }
                    SpyglassResult::Paused => {
                        loop {
                            *self.get_energy_mut() = Dynamo::update_energy();
                            self.handle_event(Event::EnergyRecharged(1000));
                            match spyglass.resume_discover(self, world) {
                                SpyglassResult::Complete => { break; }
                                Stopped(_) => { break; }
                                Paused => {}
                                Failed(_f) => { break; }
                            }
                        }
                    }
                    Failed(_) => { break; }
                }
            }
            self.used_spyglass = true;
        }
        else{
            *self.get_energy_mut() = Dynamo::update_energy();
            self.handle_event(Event::EnergyRecharged(1000));
            let map_size = robot_map(world).unwrap().len();
            let destination = Destination::explore(self.robot.energy.get_energy_level(), map_size);
            let _result = Planner::planner(self, destination, world);
        }
    }

    pub fn make_next_thing(&mut self, world: &mut World){
        self.tickets = self.tickets + 1;
        if self.tickets == self.tickets_to_wait{
            self.tickets_to_wait = thread_rng().gen_range(7..=12);
            self.tickets = 0;
            let is_the_new_goal_woodworking;
            match self.weather_prediction_tool.predict(self.tickets_to_wait){
                Ok(w ) => {
                    //println!("predicted_weather: {:?}", w);
                    match w {
                        WeatherType::Sunny => {
                            if thread_rng().gen_range(1..=100000) < 80000{
                                is_the_new_goal_woodworking = true;
                            }
                            else {
                                is_the_new_goal_woodworking = false;
                            }
                        }
                        WeatherType::Rainy => {
                            if thread_rng().gen_range(1..=100000) < 80000{
                                is_the_new_goal_woodworking = false;
                            }
                            else {
                                is_the_new_goal_woodworking = true;
                            }
                        }
                        WeatherType::Foggy => {
                            if thread_rng().gen_range(1..=100000) < 50000{
                                is_the_new_goal_woodworking = true;
                            }
                            else {
                                is_the_new_goal_woodworking = false;
                            }
                        }
                        WeatherType::TropicalMonsoon => { is_the_new_goal_woodworking = true; }
                        WeatherType::TrentinoSnow => { is_the_new_goal_woodworking = false; }
                    }
                },
                Err(_e) => { is_the_new_goal_woodworking = false; },
            }
            if is_the_new_goal_woodworking != self.is_the_goal_woodworking{
                //println!("modifica modalità robot...");
                self.is_the_goal_woodworking = is_the_new_goal_woodworking;
                //println!("svuotando lo zaino ...");
                self.empty_your_backpack(world);
            }
        }

        *self.get_energy_mut() = Dynamo::update_energy();
        self.handle_event(Event::EnergyRecharged(1000));
        if self.is_the_goal_woodworking {
            self.make_next_thing_for_woodworker_goal(world);
        }
        else {
            self.make_next_thing_for_mirto_goal(world);
        }
    }
}
