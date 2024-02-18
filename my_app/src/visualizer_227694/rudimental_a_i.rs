use bevy::prelude::*;
use robotics_lib::runner::{Robot, Runnable, Runner};
use crate::visualizer_227694::game_data::*;
use crate::{robot_view, RunnerTag, world_size};
use crate::LunaticRobot;
use crate::MirtoRobot;

pub struct ArtificialIntelligencePlugin;

impl Plugin for ArtificialIntelligencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_artificial_intelligence)
            .add_systems(Update, robot_runner.in_set(MySet::Third));
    }
}

fn setup_artificial_intelligence(mut game_data: ResMut<GameData>, mut commands: Commands){
    let mut generator = rip_worldgenerator::MyWorldGen::new_param(world_size, 2, 2, 2, true, false, 3, false, None);

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

    game_data.game_ticks += 1;

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
    { // next game tick
        let _ = runner.0.game_tick();
        game_data.game_ticks += 1;
        game_data.next -= 1;
        game_data.update_world = true;
    }
}
