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

use std::fmt::Debug;

use bevy::prelude::*;

use robotics_lib::world::tile::Tile;


use crate::assets_loader::AssetsLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::game_data::{CameraData, GameData, GameDataPlugin, RobotData};
use crate::gui_overlay::GUIPlugin;
use crate::movement::MovementPlugin;
use crate::robot::RobotPlugin;
use crate::user_inputs::InputPlugin;
use crate::weather::WeatherPlugin;
use crate::world::WorldPlugin;
#[derive(Debug,Clone)]
pub enum Direction{ //TODO capire come usarle comunque per la direzzione in cui deve guardare il robot nonostante non abbia gia la pappa pronta
    Right,
    Left,
    Up,
    Down
}

pub struct VisualizerGLC;
impl VisualizerGLC{
    pub fn run(ai: bool,world_size: usize){
        let robot_data = RobotData::new();
        let camera_data= CameraData::new();
        App::new()
            .insert_resource(GameData{
                autoplay:true,
                next:0,
                previous:0,
                world: vec![vec![Option::None;world_size];world_size],
                robot_data,
                camera_data,
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                next_action: false,
                frames: 0,
                feed: vec![],
                feed_visibility: true,
                map_radius: 0.0,
                hided_content: (0.0, 0.0),
                content_visibility: true,
                max_points: 100.0,
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
            .run();
    }
}

fn main() {
    VisualizerGLC::run(true,50);
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
