use bevy::prelude::{Commands, Res};
use rip_worldgenerator::MyWorldGen;




use robotics_lib::runner::{Robot, Runnable, Runner};



use crate::ai_226840::MirtoRobot;
use crate::ai_226930::LunaticRobot;
//use crate::visualizer_228097::components::WORLD_SIZE;
use crate::{positions as POSITIONS};
use crate::RunnerTag;
use crate::visualizer_228097::components::GameInfo;
use crate::world_size as WORLD_SIZE;

pub fn run(
    mut commands: Commands,
    game_info: Res<GameInfo>
)
{
    let robot: Box<dyn Runnable>;

    match game_info.ai {
        true => { robot = Box::new(MirtoRobot::new(false)) }
        false => { robot = Box::new(LunaticRobot::new()) }
    }

    let mut generator = MyWorldGen::new_param(WORLD_SIZE, 2, 2, 2, true, false, 3, false, None);

    let run = Runner::new(robot, &mut generator).unwrap();

    let mut update_positions = POSITIONS.lock().unwrap();
    *update_positions = (run.get_robot().get_coordinate().get_row(), run.get_robot().get_coordinate().get_col());

    commands.insert_resource(RunnerTag(run));
}
