use bevy::prelude::*;
use robotics_lib::world::tile::Content;
use crate::visualizer_228097::backpack::component::{CollectableItems};
use crate::visualizer_228097::backpack::component::ContentHub;
use crate::visualizer_228097::events::UpdateBackpack;

pub fn update_item_list(
    mut query: Query<(&mut Text, &ContentHub), With<ContentHub>>,
    mut collectable_items: ResMut<CollectableItems>,
    mut er_update_backpack: EventReader<UpdateBackpack>,
)
{
    for event in er_update_backpack.read() {

        let x = event.n as i32 * if event.add { 1 } else { -1 };
        match event.content {
            Content::Rock(_) => { collectable_items.rock = (collectable_items.rock as i32 + x) as usize }
            Content::Tree(_) => { collectable_items.tree = (collectable_items.tree as i32 + x) as usize }
            Content::Garbage(_) => { collectable_items.garbage = (collectable_items.garbage as i32 + x) as usize }
            Content::Coin(_) => { collectable_items.coin = (collectable_items.coin as i32 + x) as usize }
            Content::Water(_) => { collectable_items.water = (collectable_items.water as i32 + x) as usize }
            Content::Fish(_) => { collectable_items.fish = (collectable_items.fish as i32 + x) as usize }
            Content::Bush(_) => { collectable_items.bush = (collectable_items.bush as i32 + x) as usize }
            Content::JollyBlock(_) => { collectable_items.jolly_block = (collectable_items.jolly_block as i32 + x) as usize }
            Content::Scarecrow => { collectable_items.scarecrow = (collectable_items.scarecrow as i32 + x) as usize }
            _ => {}
        }

        for (mut t, c) in query.iter_mut(){
            if c.tile_content == event.content {
                let mut val = 0;
                match c.tile_content {
                    Content::Rock(_) => { val = collectable_items.rock; }
                    Content::Tree(_) => { val = collectable_items.tree; }
                    Content::Garbage(_) => { val = collectable_items.garbage; }
                    Content::Coin(_) => { val = collectable_items.coin; }
                    Content::Water(_) => { val = collectable_items.water; }
                    Content::Fish(_) => { val = collectable_items.fish; }
                    Content::Bush(_) => { val = collectable_items.bush; }
                    Content::JollyBlock(_) => { val = collectable_items.jolly_block; }
                    Content::Scarecrow => { val = collectable_items.scarecrow; }
                    _ => {}
                }
                t.sections[0].value = val.to_string();
            }
        }
    }
}
