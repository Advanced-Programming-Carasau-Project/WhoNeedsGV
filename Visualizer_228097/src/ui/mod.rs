pub mod layout;
mod backpack_menu;
mod log;
mod game_stats_menu;
mod environment_stats_menu;
mod grid;
mod robot_stats_menu;
mod components;

use bevy::prelude::*;

use layout::*;

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, generate_ui)
        ;
    }
}