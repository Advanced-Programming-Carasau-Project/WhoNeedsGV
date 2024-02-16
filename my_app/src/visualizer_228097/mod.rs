use bevy::prelude::*;
use bevy::app::App;
use bevy::DefaultPlugins;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use components::GameInfo;
use crate::visualizer_228097::robot::RobotPlugin;
use crate::visualizer_228097::connect_with_ai::*;
use crate::visualizer_228097::backpack::BackpackPlugin;
use crate::visualizer_228097::energy::EnergyPlugin;
use crate::visualizer_228097::events::*;
use crate::visualizer_228097::meteo::MeteoPlugin;
use crate::visualizer_228097::runner::run;
use crate::visualizer_228097::stats::StatsPlugin;
use crate::visualizer_228097::ui::MyUiPlugin;



mod world;
mod robot;
mod connect_with_ai;
mod energy;
mod backpack;
mod meteo;
pub mod components;
mod stats;
pub mod ui;
pub mod systems;
mod events;
pub mod runner;

use crate::visualizer_228097::world::WorldPlugin;
//pub const WINDOW_HEIGHT:f32 = (world_size * 10) as f32;
pub const WINDOW_HEIGHT:f32 = 500.0;
pub const WINDOW_WIDTH:f32 = WINDOW_HEIGHT + (WINDOW_HEIGHT * 2.0 / 3.0);

pub fn avvia_app(robot: bool){
    App::new()
        .add_plugins(DefaultPlugins)

        .insert_resource(GameInfo {
            environmental_condition: EnvironmentalConditions::new(&[WeatherType::Rainy], 0, 0).unwrap(),
            event_vec: Vec::new(),
            first_interaction: true,
            ai: robot
        })

        .add_event::<Ready>()
        .add_event::<Terminated>()
        .add_event::<TimeChanged>()
        .add_event::<EnergyUpdated>()
        .add_event::<Moved>()
        .add_event::<TileContentUpdated>()
        .add_event::<UpdateBackpack>()

        .add_plugins(BackpackPlugin)
        .add_plugins(EnergyPlugin)
        .add_plugins(MeteoPlugin)
        .add_plugins(RobotPlugin)
        .add_plugins(StatsPlugin)
        .add_plugins(MyUiPlugin)
        .add_plugins(WorldPlugin)

        .add_systems(PreStartup, set_window)
        .add_systems(PreStartup, run)
        .add_systems(Update, update)
        .run();
}


pub fn set_window(mut windows: Query<&mut Window>){
    //println!("Dentro set_window");
    let mut window = windows.single_mut();
    window.resolution.set(WINDOW_WIDTH, WINDOW_HEIGHT);
    window.resizable = false;
}


