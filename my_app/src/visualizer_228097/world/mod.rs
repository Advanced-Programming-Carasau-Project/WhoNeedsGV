use bevy::prelude::*;

mod systems;
pub mod components;

use crate::visualizer_228097::world::systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_tile)
        ;
    }
}


