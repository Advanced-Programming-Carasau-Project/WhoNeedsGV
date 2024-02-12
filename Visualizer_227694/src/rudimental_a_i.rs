use std::sync::Mutex;
use robotics_lib::event::events::Event;
use bevy::prelude::*;
use robotics_lib::energy::Energy;
use robotics_lib::interface::{get_score, robot_map};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use crate::game_data::*;
use crate::GameUpdate;
use lazy_static::lazy_static;
use robotics_lib::world::tile::Tile;

use crate::ai_226840_mirto_robot;
use crate::ai_226840_mirto_goal;
use crate::ai_226840_mirto_robot::MirtoRobot;
use crate::ai_226840_woodworker_goal;

// Static variables for data exchange between bevy and non bevy code
lazy_static! {
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
        app.add_systems(PostStartup, setup_artificial_intelligence)
            .add_systems(Update, update_game_update.in_set(MySet::First));
    }
}

fn setup_artificial_intelligence(mut game_update: ResMut<GameUpdate>,
                      mut game_data: ResMut<GameData>,
                      mut commands: Commands,
){

    if game_data.ai{ //here I initialize the runner resource with right AI robot
        const world_size: usize = 45;
        let robot = MirtoRobot::new(Robot::new(), true);
        let mut generator = rip_worldgenerator::MyWorldGen::new_param(world_size,2,5,0,true,false, 5, false, None);
        let mut run = Runner::new(Box::new(robot), &mut generator).unwrap();

        commands.insert_resource(RunnerTag(run));
    }else{
        println!("la funzione della libreria AI di MURRU");
    }
}
fn update_game_update(mut game_update: ResMut<GameUpdate>,
                      mut game_data: ResMut<GameData>,
){
    if game_data.next!=0{
        game_data.next -= 1;
        info!("next process_tick");
        let mut update = events.lock().unwrap();
    }else {
        let mut events_update = events.lock().unwrap();
        let mut points_update = points.lock().unwrap();
        let mut world_update = robot_view.lock().unwrap();
        for i in events_update.iter(){
            game_update.events.push(i.clone());
        }
        game_update.world = world_update.clone();
        game_update.points = points_update.clone();
    }
}
