mod camera;
mod robot;
mod world;
mod weather;
mod user_inputs;
mod assets_loader;
mod gui_overlay;
mod movement;
mod game_data;
mod rudimental_a_i;
use std::fmt::Debug;
use bevy::prelude::*;
use rip_worldgenerator::MyWorldGen;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::world_generator::Generator;
use crate::robot_view;


use crate::visualizer_227694::assets_loader::AssetsLoaderPlugin;
use crate::visualizer_227694::camera::CameraPlugin;
use crate::visualizer_227694::game_data::{CameraData, GameData, GameDataPlugin, RobotData};
use crate::visualizer_227694::gui_overlay::GUIPlugin;
use crate::visualizer_227694::movement::MovementPlugin;
use crate::visualizer_227694::robot::RobotPlugin;
use crate::visualizer_227694::rudimental_a_i::ArtificialIntelligencePlugin;
use crate::visualizer_227694::user_inputs::InputPlugin;
use crate::visualizer_227694::weather::WeatherPlugin;
use crate::visualizer_227694::world::WorldPlugin;
#[derive(Debug,Clone)]
pub(crate) enum Direction{
    Right,
    Left,
    Up,
    Down
}

pub const ACTIONS_VELOCITY:f32 = 0.15;

fn from_map_to_option_world(map: &Vec<Vec<Tile>>)->Vec<Vec<Option<Tile>>>{ //Used to load the entire world for testing purpose
    let mut r = vec![];
    for i in 0..map.len(){
        let mut t = vec![];
        for j in 0..map.len(){
            t.push(Some(map[i][j].clone()));
        }
        r.push(t);

    }
    return r;
}

pub struct VisualizerGLC;
impl VisualizerGLC{
    pub fn run(ai: bool,world_size: usize){
        App::new()
            .insert_resource(GameData{
                autoplay:false,
                next:0,
                world_size,
                world: vec![vec![None; world_size]; world_size],
                update_world: true,
                robot_data: RobotData::new(),
                camera_data : CameraData::new(),
                current_tile_elevation: 0.0,
                game_ticks: 0,
                timer: Timer::from_seconds(ACTIONS_VELOCITY, TimerMode::Repeating),
                next_action: false,
                frames: 0,
                feed: vec![],
                feed_visibility: true,
                hided_content: (0.0, 0.0),
                content_visibility: true,
                ai,
            })
            .add_plugins(DefaultPlugins)
            //plugins developed by Giulio Lo Cigno
            .add_plugins(AssetsLoaderPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(WeatherPlugin)
            .add_plugins(GameDataPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(GUIPlugin)
            .add_plugins(RobotPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(ArtificialIntelligencePlugin)
            .run();
    }
    pub fn visualize_world(world_size: usize,world_bool: bool){
        let mut mondo;
        if world_bool{
            let mut generator = MyWorldGen::new_param(world_size, 2, 2, 2, true, false, 3, false, None);
            mondo = generator.gen();
        }else {
            let mut generator = who_needs_gv_world_generator::WorldGenerator::new(world_size);
            mondo = generator.gen();
        }

        *robot_view.lock().unwrap() = from_map_to_option_world(&mondo.0);


        App::new()
            .insert_resource(GameData{
                autoplay:false,
                next:0,
                world_size,
                world: vec![vec![None;world_size];world_size],
                update_world: true,
                robot_data: RobotData::new(),
                camera_data : CameraData::new(),
                current_tile_elevation: 0.0,
                game_ticks: 0,
                timer: Timer::from_seconds(ACTIONS_VELOCITY, TimerMode::Repeating),
                next_action: false,
                frames: 0,
                feed: vec![],
                feed_visibility: true,
                hided_content: (0.0, 0.0),
                content_visibility: true,
                ai:false,
            })
            .add_plugins(DefaultPlugins)
            //plugins developed by Giulio Lo Cigno
            .add_plugins(AssetsLoaderPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(WeatherPlugin)
            .add_plugins(GameDataPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(GUIPlugin)
            .add_plugins(RobotPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(InputPlugin)
            .run();
    }
}