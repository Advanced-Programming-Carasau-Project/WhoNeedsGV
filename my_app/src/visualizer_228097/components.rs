use bevy::prelude::Resource;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions};

#[derive(Resource)]
pub struct GameInfo {
    pub environmental_condition: EnvironmentalConditions,
    pub event_vec: Vec<String>,
    pub first_interaction: bool,
    pub ai: bool
}


