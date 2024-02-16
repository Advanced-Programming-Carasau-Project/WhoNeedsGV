use bevy::prelude::{Commands, Resource};
use rip_worldgenerator::MyWorldGen;
use who_needs_gv_world_generator::WorldGenerator;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{get_score, robot_map};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use crate::ai_226840::MirtoRobot;
use crate::ai_226930::LunaticRobot;
use crate::components::{WORLD_SIZE};
use crate::connect_with_ai::{BACKPACK_CONTENT, ENERGY, EVENTS, POINTS, POSITIONS, ROBOT_VIEW};


#[derive(Resource)]
pub struct RunnerTag(pub(crate) Runner);


pub fn run(
    mut commands: Commands,
)
{
    //println!("Dentro run");
    let robot = LunaticRobot::new();
    //let robot = MirtoRobot::new(Robot::new(), true);
    let mut generator = MyWorldGen::new_param(WORLD_SIZE,1,5,1,true,false, 10);//, false, None);
    //let mut generator = WorldGenerator::new(WORLD_SIZE);
    let run = Runner::new(Box::new(robot), &mut generator).unwrap();

    let mut update_positions = POSITIONS.lock().unwrap();
    *update_positions = (run.get_robot().get_coordinate().get_row(), run.get_robot().get_coordinate().get_col());

    commands.insert_resource(RunnerTag(run));
}

impl Runnable for MirtoRobot {
    fn process_tick(&mut self, world: &mut World) {
        self.make_next_thing(world);

        let mut update_points = POINTS.lock().unwrap();
        let mut update_robot_view = ROBOT_VIEW.lock().unwrap();

        *update_points = get_score(world);
        *update_robot_view = robot_map(world).unwrap();
    }
    fn handle_event(&mut self, event: Event) {
        let _ = self.audio_tool.play_audio_based_on_event(&event);
        self.weather_prediction_tool.process_event(&event);

        let mut update_events = EVENTS.lock().unwrap();
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

impl Runnable for LunaticRobot {
    fn process_tick(&mut self, world: &mut World) {
        self.routine(world);

        let mut update_points = POINTS.lock().unwrap();
        let mut update_robot_view = ROBOT_VIEW.lock().unwrap();
        let mut update_positions = POSITIONS.lock().unwrap();
        let mut update_energy = ENERGY.lock().unwrap();
        let mut update_backpack_content = BACKPACK_CONTENT.lock().unwrap();

        *update_positions = (self.robot.coordinate.get_row(), self.robot.coordinate.get_col());
        *update_points = get_score(world);
        *update_robot_view = robot_map(world).unwrap();
        *update_energy = self.robot.energy.get_energy_level();
        *update_backpack_content = self.get_backpack().get_contents().clone();
    }
    fn handle_event(&mut self, event: Event) {

        let mut update_events = EVENTS.lock().unwrap();
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

unsafe impl Sync for RunnerTag {}
unsafe impl Send for RunnerTag {}
