use bevy::app::{App, Plugin, Update};
use crate::energy::systems::*;

mod systems;
pub mod components;

pub struct EnergyPlugin;

impl Plugin for EnergyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_energy)
        ;
    }
}