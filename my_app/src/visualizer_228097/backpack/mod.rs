use bevy::app::{App, Plugin, Update};
use crate::visualizer_228097::backpack::component::{CollectableItems};
use crate::visualizer_228097::backpack::systems::*;

mod systems;
pub mod component;


pub struct BackpackPlugin;

impl Plugin for BackpackPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CollectableItems>()
            .add_systems(Update, update_item_list)
        ;
    }
}