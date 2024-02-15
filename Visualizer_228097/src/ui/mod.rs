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
use crate::states::AppState::GeneratingUi;

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GeneratingUi), generate_ui)
        ;
    }
}