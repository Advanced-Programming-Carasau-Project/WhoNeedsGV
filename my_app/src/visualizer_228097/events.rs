use bevy::prelude::Event;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content, Tile};

#[derive(Event)]
pub struct Ready {
    /*?*/
}

#[derive(Event)]
pub struct Terminated {
    /*?*/
}

#[derive(Event)]
pub struct TimeChanged {    //Usato anche per DayChanged
    pub new_environmental_conditions: EnvironmentalConditions
}

#[derive(Event)]
pub struct EnergyUpdated {
    pub total_energy: usize
}


#[derive(Event)]
pub struct Moved {
    pub next_tile: Tile,
    pub next_position: (usize, usize)
}

#[derive(Event)]
pub struct TileContentUpdated {
    pub new_tile: Tile,
    pub position: (usize, usize),
}

#[derive(Event)]
pub struct UpdateBackpack {
    pub content: Content,
    pub n: usize,
    pub add: bool
}

