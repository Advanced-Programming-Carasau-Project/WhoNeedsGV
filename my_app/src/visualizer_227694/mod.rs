mod camera;
mod robot;
mod world;
mod weather;
mod user_inputs;
mod assets_loader;
mod gui_overlay;
mod movement;
mod game_data;
pub(crate) mod rudimental_a_i;
use std::fmt::Debug;
use bevy::prelude::*;
use robotics_lib::world::tile::Tile;

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