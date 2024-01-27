use std::collections::HashMap;
use robotics_lib::interface::{craft, Direction, put, teleport, Tools};
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

struct MirtoRobot {
    robot: Robot,
    mode: char,
    audio_tool: OxAgAudioTool,
}

impl MirtoRobot {
    pub fn new(robot: Robot, mode: char, audio_tool: OxAgAudioTool) -> Self{
        MirtoRobot {
            robot,
            mode,
            audio_tool
        }
    }

    pub fn print_map_debug(&self, world: &World){
        let map = robot_map(world).unwrap();
        let i_robot = self.robot.coordinate.get_row();
        let j_robot = self.robot.coordinate.get_col();
        let size = map.len();

        println!("mode: {} - energy: {} - backpack_content: {:?}", self.mode, self.get_energy().get_energy_level(), self.robot.backpack.get_contents());
        for i in 0..size{
            for j in 0..size{
                if i == i_robot && j == j_robot{
                    print!("{}", "R".bright_yellow());
                }
                else if let Some(tile) = &map[i][j]{
                    if tile.tile_type == TileType::DeepWater || tile.tile_type == TileType::Lava || tile.tile_type == ShallowWater {
                        print!("{}", "~".blue());
                    }
                    else if tile.tile_type == TileType::Teleport(true){
                        print!("{}", "^".red());
                    }
                    else if tile.tile_type == TileType::Wall{
                        print!("{}", "#".black());
                    }
                    else{
                        match tile.content {
                            Content::Rock(_) => { print!("r"); }
                            Content::Tree(_) => { print!("t"); }
                            Content::Garbage(_) => { print!("g"); }
                            Content::Coin(_) => { print!("C"); }
                            Content::Crate(_) => { print!("c"); }
                            Content::Bush(_) => { print!("b"); }
                            Content::JollyBlock(_) => { print!("J"); }
                            Content::Scarecrow => { print!("s"); }
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

    pub fn finds_the_nearest_empty_cell(&self, world: &World) -> Option<(Direction, usize, usize)>{
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
        visited[i_robot][j_robot] = true;

        while queue.size() != 0 {
            let (i, j) = queue.remove().unwrap();
            if Self::is_point_inside_map((i as i32 -1) , j as i32, size as i32) && !visited[i-1][j] {
                visited[i-1][j] = true;
                if let Some(tile) = &map[i-1][j]{
                    if tile.content == Content::None {
                        return Some((Direction::Up, i, j));
                    }
                }
                queue.add((i-1, j));
            }
            if Self::is_point_inside_map((i as i32 + 1) , j as i32, size as i32) && !visited[i+1][j] {
                visited[i+1][j] = true;
                if let Some(tile) = &map[i+1][j]{
                    if tile.content == Content::None {
                        return Some((Direction::Down, i, j));
                    }
                }
                queue.add((i+1, j));
            }
            if Self::is_point_inside_map(i as i32, (j as i32 -1) , size as i32) && !visited[i][j-1] {
                visited[i][j-1] = true;
                if let Some(tile) = &map[i][j-1]{
                    if tile.content == Content::None {
                        return Some((Direction::Left, i, j));
                    }
                }
                queue.add((i, j-1));
            }
            if Self::is_point_inside_map(i as i32, (j as i32 +1) , size as i32) && !visited[i][j+1] {
                visited[i][j+1] = true;
                if let Some(tile) = &map[i][j+1]{
                    if tile.content == Content::None {
                        return Some((Direction::Right, i, j));
                    }
                }
                queue.add((i, j+1));
            }
        }

        None
    }

    pub fn make_next_thing(&mut self, world: &mut World){
        *self.get_energy_mut() = Dynamo::update_energy();

        if self.mode == 'i' { //piazza il mirto in giro per il mondo
            let n_jolly_block = self.get_backpack().get_contents().get(&Content::JollyBlock(0)).unwrap();
            if *n_jolly_block > 0{
                match self.finds_the_nearest_empty_cell(world) {
                    None => { self.mode = 'm' }
                    Some((d, i, j)) => {
                        println!("coordinate robot: {:?}, coordinate da raggiungere: {:?}", self.robot.coordinate, (i, j));
                        if !(self.robot.coordinate.get_row() == i && self.robot.coordinate.get_col() == j) {
                            let destination = Destination::go_to_coordinate((i, j));
                            let result = Planner::planner(self, destination, world);
                            println!("{:?}", result);
                            match result {
                                Ok(r) => {
                                    match &r {
                                        PlannerResult::Path(p) => {
                                            for i in 0..p.0.len(){
                                                match &p.0[i] {
                                                    Action::Move(d) => {
                                                        go(self, world, d.clone());
                                                    },
                                                    Action::Teleport((i, j)) => {
                                                        teleport(self, world, (*i, *j));
                                                    },
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                        println!("coordinate robot: {:?} - direction: {:?}", self.robot.coordinate, d);
                        println!("{:?}", put(self, world, Content::JollyBlock(0), 1, d));
                    }
                }
            }
            else {
                self.mode = 's';
            }
        }
        else if self.mode == 'c' { //crafta il mirto
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
                                self.mode = 'i';
                                cont_flag = false;
                            }
                            LibError::NotEnoughSpace(s) => {
                                self.mode = 'i';
                                cont_flag = false;
                            }
                            _ => {  }
                        }
                    }
                }
            }
        }
        else if self.mode == 's' { //cerca la roba per craftare il mirto, nella mappa che conosci
            let search_content = Content::Bush(1);
            let mut result = CollectTool::collect_content(self, world, &search_content, 10, self.robot.energy.get_energy_level());
            match result {
                Ok(n) => {
                    println!("collected: {}", n);
                }
                Err(e) => {
                    match e {
                        LibErrorExtended::CommonError(e) => {
                            println!("e: {:?}", e);
                            self.mode = 'c';
                        }
                        LibErrorExtended::NoSolution => { self.mode = 'm'; }
                        LibErrorExtended::RobotMapEmpty => { self.mode = 'm'; }
                        LibErrorExtended::NoWalkableTile => { self.mode = 'm'; }
                        LibErrorExtended::EnergyOutOfLimit => {}
                    }
                }
            }
        }
        if self.mode == 'm' { //vai in giro per la mappa in maniera casuale
            let destination = Destination::explore(self.robot.energy.get_energy_level(), 200);
            let result = Planner::planner(self, destination, world);
            self.mode = 's';
        }
    }
}

impl Runnable for MirtoRobot {
    fn process_tick(&mut self, world: &mut World) {
        self.make_next_thing(world);
        self.print_map_debug(world);
    }
    fn handle_event(&mut self, event: Event) {
        self.audio_tool.play_audio_based_on_event(&event);
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
    match OxAgAudioTool::new(HashMap::new(), HashMap::new(), HashMap::new()) {
        Ok(mut r) => {
            let robot = MirtoRobot::new(Robot::new(), 'm', r);

            let mut generator = MyWorldGen::new_param(100,2,5,0,true,true, 3);

            let mut run = Runner::new(Box::new(robot), &mut generator).unwrap(); //creo un runner (l'oggetto che gestisce i tick del mondo). Questa struc creerà il mondo grazie al world generator

            loop{
                run.game_tick(); //faccio avanzare un tick del mondo (un tick corrisponde all'unità elementare di tempo nel "gioco")
                thread::sleep(Duration::from_millis(100));
            }
        }
        Err(e) => {
            println!("err: {:?}", e);
        }
    }
}