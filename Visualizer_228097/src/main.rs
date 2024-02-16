use bevy::prelude::*;
use bevy::app::App;
use bevy::DefaultPlugins;
use components::GameInfo;
use crate::robot::RobotPlugin;
use crate::connect_with_ai::*;
use crate::backpack::BackpackPlugin;
use crate::energy::EnergyPlugin;
use crate::events::*;
use crate::meteo::MeteoPlugin;
use crate::runner::run;
use crate::stats::StatsPlugin;
use crate::ui::MyUiPlugin;


mod world;
mod robot;
mod connect_with_ai;
pub mod ai_226840;
pub mod ai_226930;
mod energy;
mod backpack;
mod meteo;
pub mod components;
mod stats;
pub mod ui;
pub mod systems;
mod events;
pub mod runner;

use crate::world::WorldPlugin;
pub const WINDOW_HEIGHT:f32 = 500.0;
pub const WINDOW_WIDTH:f32 = WINDOW_HEIGHT + (WINDOW_HEIGHT * 2.0 / 3.0);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .init_resource::<GameInfo>()

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
    println!("Dentro set_window");
    let mut window = windows.single_mut();
    window.resolution.set(WINDOW_WIDTH, WINDOW_HEIGHT);
    window.resizable = false;
}


