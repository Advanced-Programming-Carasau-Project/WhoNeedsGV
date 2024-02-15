use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct IsEnergy{}

#[derive(Resource)]
pub struct EnergyHub{
    pub energy: usize
}

impl Default for EnergyHub {
    fn default() -> Self {
        EnergyHub { energy: 1000 }
    }
}
