use bevy::app::{App, Plugin, Update};
use crate::meteo::systems::*;

mod systems;
pub mod components;

pub struct MeteoPlugin;

impl Plugin for MeteoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_meteo)
        ;
    }
}