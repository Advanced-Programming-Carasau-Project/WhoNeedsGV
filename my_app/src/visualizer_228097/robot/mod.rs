use bevy::prelude::*;
use crate::visualizer_228097::robot::systems::*;

pub mod components;
pub mod systems;

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(OnEnter(AppState::RobotIsReady), spawn_robot)
            .add_systems(Update, spawn_robot)
            .add_systems(Update, move_robot)
        ;
    }
}