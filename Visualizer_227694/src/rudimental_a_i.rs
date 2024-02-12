use std::sync::Mutex;
use robotics_lib::event::events::Event;
use bevy::prelude::*;
use robotics_lib::energy::Energy;
use robotics_lib::interface::{get_score, robot_map};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use crate::game_data::*;
use lazy_static::lazy_static;
use robotics_lib::world::tile::Tile;



use crate::ai_226840_mirto_robot::MirtoRobot;


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
        app.add_systems(PostStartup, setup_artificial_intelligence)
            .add_systems(Update, update_game_update.in_set(MySet::First));
    }
}

fn setup_artificial_intelligence(game_data: ResMut<GameData>, mut commands: Commands){

    if game_data.ai{ //here I initialize the runner resource with right AI robot
        const world_size: usize = 45;
        let robot = MirtoRobot::new(Robot::new(), true);
        let mut generator = rip_worldgenerator::MyWorldGen::new_param(world_size,2,5,0,true,false, 5, false, None);
        let run = Runner::new(Box::new(robot), &mut generator).unwrap();

        commands.insert_resource(RunnerTag(run));
        /* TODO capire come ricevere queste info e aggiungerle a game_data subito
        robot_data.energy = energy as i32;
        robot_data.robot_translation = Transform::from_translation(Vec3::new(robot_spawn.0 as f32,robot_elevation as f32 / 10.0 - 0.45,robot_spawn.1 as f32)).translation;
        camera_data.camera_transform = Transform::from_translation(Vec3::new(0.0,10.0,0.0)).looking_at(Vec3::ZERO,Vec3::Z);
        camera_data.camera_transform.translation = Transform::from_translation(Vec3::new(robot_spawn.0 as f32,(robot_elevation as f32 /10.0) + 10.0,robot_spawn.1 as f32)).translation;
        */
    }else{
        println!("la funzione della libreria AI di MURRU");
    }
}
fn update_game_update(mut game_data: ResMut<GameData>, mut runner: ResMut<RunnerTag>){
    if game_data.next <= 0{
        return;
    }
    game_data.next -= 1;
    runner.0.game_tick();

}
