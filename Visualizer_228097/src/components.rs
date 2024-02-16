use bevy::prelude::Resource;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use crate::WINDOW_HEIGHT;

pub const WORLD_SIZE:usize = (WINDOW_HEIGHT / 10.0) as usize;

#[derive(Resource)]
pub struct GameInfo {
    pub environmental_condition: EnvironmentalConditions,
    pub event_vec: Vec<String>,
    pub first_interaction: bool,
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            environmental_condition: EnvironmentalConditions::new(&[WeatherType::Rainy], 0, 0).unwrap(),
            event_vec: Vec::new(),
            first_interaction: true,
        }
    }
}

