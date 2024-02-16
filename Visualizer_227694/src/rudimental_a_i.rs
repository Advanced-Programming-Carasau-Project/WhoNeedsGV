use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use robotics_lib::event::events::Event;
use bevy::prelude::*;
use colored::Colorize;
use robotics_lib::energy::Energy;
use robotics_lib::interface::{get_score, robot_map};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use crate::game_data::*;
use lazy_static::lazy_static;
use robotics_lib::world::tile::{Content, Tile, TileType};



use crate::ai_226840_mirto_robot::MirtoRobot;
use crate::ai_226930_main::LunaticRobot;


// Static variables for data exchange between bevy and non bevy code
lazy_static! { /// OGNi VOLTA CHE CAMBIA QUALCOSA L'IA MI AGGIORNA QUESTA RESOURCE E IO HO TUTTO LI PRONTO ///
    // Store your variables here
    pub static ref events: Mutex<Vec<Event>> = Mutex::new(vec![]);
    pub static ref points: Mutex<f32> = Mutex::new(0.00);
    pub static ref robot_view: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
}
impl Runnable for MirtoRobot {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
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

#[derive(Resource)]
pub struct RunnerTag(Runner);

unsafe impl Sync for RunnerTag {}
unsafe impl Send for RunnerTag {}

pub struct ArtificialIntelligencePlugin;

impl Plugin for ArtificialIntelligencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_artificial_intelligence)
            .add_systems(Update, robot_runner.in_set(MySet::Third));
    }
}

fn setup_artificial_intelligence(mut game_data: ResMut<GameData>, mut commands: Commands){

    //let mut generator = rip_worldgenerator::MyWorldGen::new_param(game_data.world_size,1,1,1,true,false, 5, false, None);
    let mut generator = who_needs_gv_world_generator::WorldGenerator::new(game_data.world_size);

    let mut run = Runner::new(Box::new(LunaticRobot::new()), &mut generator).unwrap();


    if game_data.ai{ //here I initialize the runner resource with right AI robot
        let robot = MirtoRobot::new(Robot::new(), false);
        run = Runner::new(Box::new(robot), &mut generator).unwrap();
    }else{
        let robot = LunaticRobot::new();
        run = Runner::new(Box::new(robot), &mut generator).unwrap();
    }
    let spawn_point = (run.get_robot().get_coordinate().get_row(),run.get_robot().get_coordinate().get_col());
    let robot_energy = run.get_robot().get_energy().get_energy_level() as i32;

    let mut runner = RunnerTag(run);
    let _ = runner.0.game_tick();

    let mondo = robot_view.lock().unwrap();

    match &mondo[spawn_point.0][spawn_point.1]{
        None => {
            panic!("spawn point unknown");
        }
        Some(tile) => {
            game_data.current_tile_elevation = tile.elevation as f32;
        }
    }

    game_data.robot_data.energy = robot_energy;
    game_data.robot_data.robot_translation = Transform::from_translation(Vec3::new(spawn_point.0 as f32,game_data.current_tile_elevation  / 10.0 - 0.95,spawn_point.1 as f32)).translation;

    game_data.camera_data.camera_transform = Transform::from_translation(Vec3::new(0.0,10.0,0.0)).looking_at(Vec3::ZERO,Vec3::Z);
    game_data.camera_data.camera_transform.translation = Transform::from_translation(Vec3::new(spawn_point.0 as f32, (game_data.current_tile_elevation / 10.0) + 9.05, spawn_point.1 as f32)).translation;

    commands.insert_resource(runner);
}
fn robot_runner(mut game_data: ResMut<GameData>, mut runner: ResMut<RunnerTag>){
    if game_data.next <= 0{
        return;
    }
    { // SERVE A LIBERARE I MUTEX SENZA ESPLICITARE UNLOCK
        let _ = runner.0.game_tick();
        game_data.next -= 1;
        game_data.update_world = true;
    }
}
