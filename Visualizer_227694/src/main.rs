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
mod ai_226840_mirto_robot;
mod ai_226840_mirto_goal;
mod ai_226840_woodworker_goal;
mod ai_226930_main;
mod ai_226930_coin_collection;
mod ai_226930_scare_crow_killing;
mod ai_226930_print;
use std::fmt::Debug;
use bevy::prelude::*;
use robotics_lib::world::tile::Tile;

use crate::assets_loader::AssetsLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::game_data::{CameraData, GameData, GameDataPlugin, RobotData};
use crate::gui_overlay::GUIPlugin;
use crate::movement::MovementPlugin;
use crate::robot::RobotPlugin;
use crate::rudimental_a_i::ArtificialIntelligencePlugin;
use crate::user_inputs::InputPlugin;
use crate::weather::WeatherPlugin;
use crate::world::WorldPlugin;
#[derive(Debug,Clone)]
pub enum Direction{
    Right,
    Left,
    Up,
    Down
}

pub const ACTIONS_VELOCITY:f32 = 0.15;

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
}

fn main() {
    VisualizerGLC::run(false,50);
}

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
