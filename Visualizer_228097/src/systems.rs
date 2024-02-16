use robotics_lib::world::tile::{Content, TileType};
use bevy::prelude::Color;

pub fn get_path_content(content: Content) -> String{
    match content {
        Content::Rock(_) => { return "sprites/contents/rock.png".to_string(); }
        Content::Tree(_) => { return "sprites/contents/tree.png".to_string(); }
        Content::Garbage(_) => { return "sprites/contents/garbage.png".to_string(); }
        Content::Fire => { return "sprites/contents/fire.png".to_string(); }
        Content::Coin(_) => { return "sprites/contents/coin.png".to_string(); }
        Content::Bin(_) => { return "sprites/contents/bin.png".to_string(); }
        Content::Crate(_) => { return "sprites/contents/crate.png".to_string(); }
        Content::Bank(_) => { return "sprites/contents/bank.png".to_string(); }
        Content::Water(_) => { return "sprites/contents/water.png".to_string(); }
        Content::Market(_) => { return "sprites/contents/market.png".to_string(); }
        Content::Fish(_) => { return "sprites/contents/fish.png".to_string(); }
        Content::Building => { return "sprites/contents/building.png".to_string(); }
        Content::Bush(_) => { return "sprites/contents/bush.png".to_string(); }
        Content::JollyBlock(_) => { return "sprites/contents/jolly_block.png".to_string(); }
        Content::Scarecrow => { return "sprites/contents/scarecrow.png".to_string(); }
        Content::None => { return "".to_string(); }
    }
}

pub fn give_color(tile: TileType) -> Color {
    match tile {
        TileType::DeepWater => { Color::BLUE }
        TileType::ShallowWater => { Color::CYAN }
        TileType::Sand => { Color::Rgba { red: 0.761, green: 0.698, blue: 0.502, alpha: 1.0 } }
        TileType::Grass => { Color::GREEN }
        TileType::Street => { Color::BLACK }
        TileType::Hill => { Color::DARK_GREEN }
        TileType::Mountain => { Color::MAROON }
        TileType::Snow => { Color::WHITE }
        TileType::Lava => { Color::RED }
        TileType::Teleport(_) => { Color::PURPLE }
        TileType::Wall => { Color::GRAY }
    }
}

pub fn _give_color_content(content: Content) -> Color {
    match content {
        Content::Rock(_) => { Color::MAROON }
        Content::Tree(_) => { Color::GREEN }
        Content::Garbage(_) => { Color::BLACK }
        Content::Fire => { Color::RED }
        Content::Coin(_) => { Color::GOLD }
        Content::Bin(_) => { Color::ORANGE }
        Content::Crate(_) => { Color::BEIGE }
        Content::Bank(_) => { Color::SILVER }
        Content::Water(_) => { Color::CYAN }
        Content::Market(_) => { Color::YELLOW }
        Content::Fish(_) => { Color::SALMON }
        Content::Building => { Color::GRAY }
        Content::Bush(_) => { Color::DARK_GREEN }
        Content::JollyBlock(_) => { Color::PURPLE }
        Content::Scarecrow => { Color::NONE }
        Content::None => { Color::NONE }
    }
}
