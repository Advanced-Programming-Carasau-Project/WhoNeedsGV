use bevy::prelude::*;
use robotics_lib::world::tile::Content;


#[derive(Resource)]
pub struct CollectableItems {
    pub coin: usize,
    pub garbage: usize,
    pub rock: usize,
    pub tree: usize,
    pub water: usize,
    pub fish: usize,
    pub bush: usize,
    pub jolly_block: usize,
    pub scarecrow: usize
}

impl Default for CollectableItems {
    fn default() -> Self {
        CollectableItems { coin: 0, garbage: 0, rock: 0, tree: 0, water: 0, fish: 0, bush: 0, jolly_block: 0, scarecrow: 0 }
    }
}

#[derive(Component)]
pub struct ContentHub {
    pub tile_content: Content
}

