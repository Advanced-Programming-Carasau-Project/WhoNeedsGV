use bevy::app::{App, Update};
use bevy::prelude::Plugin;
use crate::stats::systems::*;

mod systems;
pub mod components;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (update_hour, update_day_time, update_score, update_elevation, update_log))
        ;
    }
}