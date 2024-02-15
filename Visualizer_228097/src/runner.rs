use bevy::prelude::{Commands, NextState, ResMut, Resource};
use rip_worldgenerator::MyWorldGen;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{get_score, robot_map};
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use crate::ai_226840::MirtoRobot;
use crate::components::{GameInfo, WORLD_SIZE};
use crate::connect_with_ai::{EVENTS, POINTS, ROBOT_VIEW};
use crate::states::AppState;

#[derive(Resource)]
pub struct RunnerTag(pub(crate) Runner);


pub fn run(
    mut commands: Commands,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_info: ResMut<GameInfo>,
)
{
    //println!("Dentro run");

    let robot = MirtoRobot::new(Robot::new(), true);
    let mut generator = MyWorldGen::new_param(WORLD_SIZE,1,5,1,true,false, 10);//, false, None);
    let run = Runner::new(Box::new(robot), &mut generator).unwrap();

    game_info.robot_position.0 = run.get_robot().get_coordinate().get_row();
    game_info.robot_position.1 = run.get_robot().get_coordinate().get_col();

    commands.insert_resource(RunnerTag(run));
    app_state_next_state.set(AppState::GeneratingUi);
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


unsafe impl Sync for RunnerTag {}
unsafe impl Send for RunnerTag {}
