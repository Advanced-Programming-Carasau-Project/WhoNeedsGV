mod ai_226840;

use robotics_lib::runner::Runner;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot};
use bevy::ecs::system::Resource;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;
use bevy::app::App;
use bevy::app::PreStartup;
use bevy::ecs::system::Commands;
use core::time::Duration;
use rip_worldgenerator::MyWorldGen;
use ai_226840::MirtoRobot;
use bevy::ecs::system::ResMut;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{get_score};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;
use robotics_lib::interface::robot_map;
use bevy::prelude::*;
use colored::Colorize;

// Static variables for data exchange between bevy and non bevy code
lazy_static! {
    // Store your variables here
    pub static ref events: Mutex<Vec<Event>> = Mutex::new(vec![]);
    pub static ref points: Mutex<f32> = Mutex::new(0.00);
    pub static ref robot_view: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
}

impl Runnable for MirtoRobot {
    fn process_tick(&mut self, world: &mut World) {
        self.make_next_thing(world);

        let mut update_points = points.lock().unwrap();
        let mut update_robot_view = robot_view.lock().unwrap();

        *update_points = get_score(world);
        *update_robot_view = robot_map(world).unwrap();
    }
    fn handle_event(&mut self, event: Event) {
        self.audio_tool.play_audio_based_on_event(&event);
        self.weather_prediction_tool.process_event(&event);

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

fn main() {
    App::new()
        .add_systems(PreStartup, run)
        .add_systems(Update, update)
        .run();
}

#[derive(Resource)]
pub struct RunnerTag(Runner);

unsafe impl Sync for RunnerTag {}
unsafe impl Send for RunnerTag {}

fn run(mut commands: Commands) {
    const world_size: usize = 45;
    let robot = MirtoRobot::new(Robot::new(), true);
    let mut generator = MyWorldGen::new_param(world_size,2,5,0,true,false, 5, false, None);
    let mut run = Runner::new(Box::new(robot), &mut generator).unwrap();

    commands.insert_resource(RunnerTag(run));
}

fn update(mut runner: ResMut<RunnerTag>) {
    let mut robot_coordinates = (0, 0);
    loop {
        { // SERVE A LIBERARE I MUTEX SENZA ESPLICITARE UNLOCK
            let _ = runner.0.game_tick();
            //READ DATA
            let read_points = points.lock().unwrap();
            let read_robot_view = robot_view.lock().unwrap();
            let mut read_events = events.lock().unwrap();
            for i in 0..read_events.len(){
                match &read_events[i] {
                    Event::Moved(tile, (i,j)) => {
                        robot_coordinates.0 = *i;
                        robot_coordinates.1 = *j;
                    }
                    _ => {}
                }
            }

            //UN PO DI PRINT
            println!("Events: {:?}", read_events);
            println!("Points: {:?}", read_points);
            println!("Robot View:");
            for i in 0..read_robot_view.len() {
                for j in 0..read_robot_view.len() {
                    if i == robot_coordinates.0 && j == robot_coordinates.1 {
                        print!("{}", "R".bright_yellow());
                    }
                    else if let Some(tile) = &read_robot_view[i][j]{
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
            read_events.clear();
        }

        thread::sleep(Duration::from_millis(5000));
    }
}