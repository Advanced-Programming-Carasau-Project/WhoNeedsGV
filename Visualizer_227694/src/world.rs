use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use robotics_lib::world::tile::*;
use crate::GameUpdate;
use crate::game_data::{GameData,MySet};
use crate::assets_loader::SceneAssets;
use robotics_lib::world::tile::Content::*;
use robotics_lib::event::events::Event::*;
use robotics_lib::world::tile::TileType::*;

#[derive(Bundle)]
pub struct ContentBundle{
    model: SceneBundle,
}
#[derive(Component,Debug)]
pub struct TileComponent;
#[derive(Component,Debug)]
pub struct ContentComponent;
#[derive(Bundle)]
pub struct TileBundle{
    model: SceneBundle,
}
pub struct WorldPlugin;

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, discover_and_update_tile.in_set(MySet::Third))
            .add_systems(Update,update_content.in_set(MySet::Third))
            .add_systems(Update, hide_content_under_robot.in_set(MySet::Second))
            .add_systems(Update, remove_event.in_set(MySet::Fourth));
    }
}

/*
fn create_world(mut commands: Commands, //TODO non la uso, eliminare
                scene_assets: Res<SceneAssets>,
                mut game_update: ResMut<GameUpdate>,
                mut game_data: ResMut<GameData>,
){
    while game_update.events.len() != 0 {
        match &game_update.events[0] {
            DiscoverTile{tile,coordinates,energy,points} => {
                let new_tile_radius = f32::sqrt((coordinates.0*coordinates.0) + (coordinates.1*coordinates.1));
                if new_tile_radius > game_data.map_radius {
                    game_data.map_radius = new_tile_radius;
                }
                let mut tile_scene;
                let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                let mut content_scene;
                let mut content_transform = Transform{
                    translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 1.5 ,coordinates.1).translation,
                    rotation: Default::default(),
                    scale: Transform::from_scale(Vec3::new(0.1,0.1,0.1)).scale,
                };
                match tile.tile_type {
                    DeepWater => { tile_scene = scene_assets.deep_water.clone(); }
                    ShallowWater => { tile_scene = scene_assets.shallow_water.clone(); }
                    Sand => { tile_scene = scene_assets.sand.clone(); }
                    Grass => { tile_scene = scene_assets.grass.clone(); }
                    Street => { tile_scene = scene_assets.street.clone(); }
                    Hill => { tile_scene = scene_assets.hill.clone(); }
                    Mountain => { tile_scene = scene_assets.mountain.clone(); }
                    Snow => { tile_scene = scene_assets.snow.clone(); }
                    Lava => { tile_scene = scene_assets.lava.clone(); }
                    TileType::Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
                    Wall => { tile_scene = scene_assets.wall.clone();
                        tile_scale = Transform::from_scale(Vec3::new(0.5,1.5,0.5)).scale;}
                }
                match tile.content {
                    Rock(n) => {
                        if n < 2 {
                            content_scene = scene_assets.rock1.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                        }else if n < 3 {
                            content_scene = scene_assets.rock2.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.18,0.18,0.18)).scale;
                        }else {
                            content_scene = scene_assets.rock3.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.27,0.27,0.27)).scale;
                        }
                    }
                    Tree(n) => {
                        if n < 2 {
                            content_scene = scene_assets.tree1.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.09,0.09,0.09)).scale;
                        }else if n < 4 {
                            content_scene = scene_assets.tree2.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.08,0.08,0.08)).scale;
                        }else {
                            content_scene = scene_assets.tree3.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                        }
                    }
                    Garbage(_) => {
                        content_scene = scene_assets.garbage.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.005,0.005,0.005)).scale;
                    }
                    Fire => {
                        content_scene = scene_assets.fire.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.7,0.7,0.7)).scale;
                        content_transform.translation.y += 0.05;
                    }
                    Coin(_) => {
                        content_scene = scene_assets.coin.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(1.0,1.0,1.0)).scale;
                    }
                    Bin(_) => {
                        content_scene = scene_assets.bin.clone(); //TODO non mi piace troppo la resa
                        content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                        content_transform.translation.y += 0.45;
                    }
                    Crate(_) => {
                        content_scene = scene_assets.crate_.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                        content_transform.translation.y += 0.15;
                    }
                    Bank(_) => {
                        content_scene = scene_assets.bank.clone(); //TODO non funziona
                        content_transform.scale = Transform::from_scale(Vec3::new(0.01,0.01,0.01)).scale;
                        content_transform.rotate_y(f32::to_degrees(180.0));
                    }
                    Water(_) => {
                        content_scene = Default::default();
                    }
                    Market(_) => {
                        content_scene = scene_assets.market.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                    }
                    Fish(_) => {
                        content_scene = scene_assets.fish.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.02,0.02,0.02)).scale;
                    }
                    Building => {
                        content_scene = scene_assets.building.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.007,0.007,0.007)).scale;
                    }
                    Bush(_) => {
                        content_scene = scene_assets.bush.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                    }
                    JollyBlock(_) => {
                        content_scene = scene_assets.jolly_block.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                    }
                    Scarecrow => {
                        content_scene = scene_assets.scarecrow.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                    }
                    None => {
                        content_scene = Default::default();
                    }
                }
                use bevy::core_pipeline::{bloom::{BloomCompositeMode, BloomSettings}, tonemapping::Tonemapping,};
                commands.spawn(
                    (
                        TileBundle{
                            model: SceneBundle{
                                scene: tile_scene,
                                transform: Transform{
                                    translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.0 ,coordinates.1).translation,
                                    rotation: Default::default(),
                                    scale: tile_scale,
                                },
                                ..default()
                            },
                        },
                        TileComponent,
                    )
                );
                commands.spawn(
                    (
                        ContentBundle{
                            model: SceneBundle{
                                scene: content_scene,
                                transform: content_transform,
                                ..default()
                            }
                        },
                        ContentComponent
                    )
                );
                game_update.events.remove(0);
            }
            _ => {
                return;
            }
        }
    }
}*/
fn discover_and_update_tile(mut commands: Commands,
                 scene_assets: Res<SceneAssets>,
                 mut game_data: ResMut<GameData>,
                 mut game_update: ResMut<GameUpdate>,
                 mut tile_query: Query<(&Transform,&mut Handle<Scene>),With<TileComponent>>,
){
    if !game_data.next_action {
        return;
    }
    for i in 0..game_update.world.len(){
        for j in 0..game_update.world.len() {
            match &game_update.world[i][j] {
                Option::None => {
                    continue;
                },
                Some(tile) => {
                    let coordinates = (i as f32, j as f32);
                    match &game_data.world[i][j] {
                        Some(tile_vecchia) => {
                            if tile.tile_type == tile_vecchia.tile_type{
                                continue;
                            }else { // Update the tile_type (model) of the changed tile
                                for (transform, mut tile_scene) in tile_query.iter_mut(){
                                    if transform.translation.x == coordinates.0 && transform.translation.z == coordinates.1 {
                                        match tile.tile_type {
                                            DeepWater => {
                                                *tile_scene = scene_assets.deep_water.clone();
                                            }
                                            ShallowWater => {
                                                *tile_scene = scene_assets.shallow_water.clone();
                                            }
                                            Sand => {
                                                *tile_scene = scene_assets.sand.clone();
                                            }
                                            Grass => {
                                                *tile_scene = scene_assets.grass.clone();
                                            }
                                            Street => {
                                                *tile_scene = scene_assets.street.clone();
                                            }
                                            Hill => {
                                                *tile_scene = scene_assets.hill.clone();
                                            }
                                            Mountain => {
                                                *tile_scene = scene_assets.mountain.clone();
                                            }
                                            Snow => {
                                                *tile_scene = scene_assets.snow.clone();
                                            }
                                            Lava => {
                                                *tile_scene = scene_assets.lava.clone();
                                            }
                                            TileType::Teleport(_) => {
                                                *tile_scene = scene_assets.teleport.clone();
                                            }
                                            Wall => {
                                                *tile_scene = scene_assets.wall.clone();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Option::None => { /* I will insert a new tile because it was discovered*/
                            //info!("ho discoverato una nuova tile");
                            let new_tile_radius = f32::sqrt((coordinates.0*coordinates.0) + (coordinates.1*coordinates.1));
                            if new_tile_radius > game_data.map_radius {
                                game_data.map_radius = new_tile_radius;
                            }

                            let mut tile_scene;
                            let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                            let mut content_scene;
                            let mut content_transform = Transform{
                                translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.0 ,coordinates.1).translation,
                                rotation: Default::default(),
                                scale: Transform::from_scale(Vec3::new(0.1,0.1,0.1)).scale,
                            };
                            match tile.tile_type {
                                DeepWater => { tile_scene = scene_assets.deep_water.clone(); }
                                ShallowWater => { tile_scene = scene_assets.shallow_water.clone(); }
                                Sand => { tile_scene = scene_assets.sand.clone(); }
                                Grass => { tile_scene = scene_assets.grass.clone(); }
                                Street => { tile_scene = scene_assets.street.clone(); }
                                Hill => { tile_scene = scene_assets.hill.clone(); }
                                Mountain => { tile_scene = scene_assets.mountain.clone(); }
                                Snow => { tile_scene = scene_assets.snow.clone(); }
                                Lava => { tile_scene = scene_assets.lava.clone(); }
                                TileType::Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
                                Wall => { tile_scene = scene_assets.wall.clone();
                                    tile_scale = Transform::from_scale(Vec3::new(0.5,1.5,0.5)).scale;}
                            }
                            match tile.content {
                                Rock(n) => {
                                    if n < 2 {
                                        content_scene = scene_assets.rock1.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                                    }else if n < 3 {
                                        content_scene = scene_assets.rock2.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.18,0.18,0.18)).scale;
                                    }else {
                                        content_scene = scene_assets.rock3.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.27,0.27,0.27)).scale;
                                    }
                                }
                                Tree(n) => {
                                    if n < 2 {
                                        content_scene = scene_assets.tree1.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.09,0.09,0.09)).scale;
                                    }else if n < 4 {
                                        content_scene = scene_assets.tree2.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.08,0.08,0.08)).scale;
                                    }else {
                                        content_scene = scene_assets.tree3.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                                    }
                                }
                                Garbage(_) => {
                                    content_scene = scene_assets.garbage.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.005,0.005,0.005)).scale;
                                }
                                Fire => {
                                    content_scene = scene_assets.fire.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.7,0.7,0.7)).scale;
                                    content_transform.translation.y += 0.05;
                                }
                                Coin(_) => {
                                    content_scene = scene_assets.coin.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(1.0,1.0,1.0)).scale;
                                }
                                Bin(_) => {
                                    content_scene = scene_assets.bin.clone(); //TODO non mi piace troppo la resa
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                    content_transform.translation.y += 0.45;
                                }
                                Crate(_) => {
                                    content_scene = scene_assets.crate_.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                    content_transform.translation.y += 0.15;
                                }
                                Bank(_) => {
                                    content_scene = scene_assets.bank.clone(); //TODO non funziona
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.0001,0.0001,0.0001)).scale;
                                    content_transform.rotate_y(f32::to_degrees(180.0));
                                }
                                Water(_) => {
                                    content_scene = Default::default();
                                }
                                Market(_) => {
                                    content_scene = scene_assets.market.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                }
                                Fish(_) => {
                                    content_scene = scene_assets.fish.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.02,0.02,0.02)).scale;
                                }
                                Building => {
                                    content_scene = scene_assets.building.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.007,0.007,0.007)).scale;
                                }
                                Bush(_) => {
                                    content_scene = scene_assets.bush.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                                }
                                JollyBlock(_) => {
                                    content_scene = scene_assets.jolly_block.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                }
                                Scarecrow => {
                                    content_scene = scene_assets.scarecrow.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                                }
                                None => {
                                    content_scene = Default::default();
                                }
                            }
                            commands.spawn(
                                (
                                    TileBundle{
                                        model: SceneBundle{
                                            scene: tile_scene,
                                            transform: Transform{
                                                translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.0 ,coordinates.1).translation,
                                                rotation: Default::default(),
                                                scale: tile_scale,
                                            } ,
                                            ..default()
                                        },
                                    },
                                    TileComponent
                                )
                            );
                            commands.spawn(
                                (
                                    ContentBundle{
                                        model: SceneBundle{
                                            scene: content_scene,
                                            transform: content_transform,
                                            ..default()
                                        }
                                    },
                                    ContentComponent
                                )
                            );
                            game_data.next_action = true;
                        }
                    }
                }
            }
        }
    }
}
fn update_content(mut content_query: Query<(&mut Transform,&mut Handle<Scene>),With<ContentComponent>>,
                  scene_assets: Res<SceneAssets>,
                  mut aggiornamento: ResMut<GameUpdate>,
                  mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }else {
        if aggiornamento.events.len() != 0{
            match &aggiornamento.events[0] {
                TileContentUpdated(new_tile, (x, z)) => {
                    let mut coordinates = (*x as f32, *z as f32);
                    for (mut content_transform, mut content_scene) in content_query.iter_mut(){
                        if content_transform.translation.x == coordinates.0 && content_transform.translation.z == coordinates.1{
                            match new_tile.content {
                                Rock(n) => {
                                    if n < 2 {
                                        *content_scene = scene_assets.rock1.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                                    }else if n < 3 {
                                        *content_scene = scene_assets.rock2.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.18,0.18,0.18)).scale;
                                    }else {
                                        *content_scene = scene_assets.rock3.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.27,0.27,0.27)).scale;
                                    }
                                }
                                Tree(n) => {
                                    if n < 2 {
                                        *content_scene = scene_assets.tree1.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.09,0.09,0.09)).scale;
                                    }else if n < 4 {
                                        *content_scene = scene_assets.tree2.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.08,0.08,0.08)).scale;
                                    }else {
                                        *content_scene = scene_assets.tree3.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                                    }
                                }
                                Garbage(_) => {
                                    *content_scene = scene_assets.garbage.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.005,0.005,0.005)).scale;
                                }
                                Fire => {
                                    *content_scene = scene_assets.fire.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.7,0.7,0.7)).scale;
                                    content_transform.translation.y += 0.05;
                                }
                                Coin(_) => {
                                    *content_scene = scene_assets.coin.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(1.0,1.0,1.0)).scale;
                                }
                                Bin(_) => {
                                    *content_scene = scene_assets.bin.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                    content_transform.translation.y += 0.45;
                                }
                                Crate(_) => {
                                    *content_scene = scene_assets.crate_.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                    content_transform.translation.y += 0.15;
                                }
                                Bank(_) => {
                                    *content_scene = scene_assets.bank.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.1,0.1,0.1)).scale;
                                    content_transform.rotate_y(f32::to_degrees(180.0));
                                }
                                Water(_) => {
                                    *content_scene = Default::default();
                                }
                                Market(_) => {
                                    *content_scene = scene_assets.market.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                }
                                Fish(_) => {
                                    *content_scene = scene_assets.fish.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.02,0.02,0.02)).scale;
                                }
                                Building => {
                                    *content_scene = scene_assets.building.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.007,0.007,0.007)).scale;
                                }
                                Bush(_) => {
                                    *content_scene = scene_assets.bush.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                                }
                                JollyBlock(_) => {
                                    *content_scene = scene_assets.jolly_block.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                }
                                Scarecrow => {
                                    *content_scene = scene_assets.scarecrow.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                                }
                                None => {
                                    *content_scene = Default::default();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.1,0.1,0.1)).scale;
                                }
                            }
                        }
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }
}
fn hide_content_under_robot(mut content_query: Query<(&mut Transform, &mut Visibility),With<ContentComponent>>,
                            mut game_data: ResMut<GameData>,
){
    if game_data.content_visibility{
        if (f32::floor(game_data.robot_data.robot_translation.x) != game_data.hided_content.0) || (f32::floor(game_data.robot_data.robot_translation.z) != game_data.hided_content.1) {
            let mut new_hidden_content= (0.0,0.0);
            for (mut transform,mut visibility) in content_query.iter_mut(){
                if (f32::floor(transform.translation.x) == f32::floor(game_data.robot_data.robot_translation.x)) && (f32::floor(transform.translation.z) == f32::floor(game_data.robot_data.robot_translation.z)) {
                    new_hidden_content = (f32::floor(transform.translation.x),f32::floor(transform.translation.z));
                    *visibility = Visibility::Hidden;
                }else if (f32::floor(transform.translation.x) == game_data.hided_content.0) && (f32::floor(transform.translation.z) == game_data.hided_content.1) {
                    *visibility = Visibility::Visible;
                }
            }
            game_data.hided_content = new_hidden_content;
        }
    }
}
fn remove_event(mut game_update: ResMut<GameUpdate>,
                      mut game_data: ResMut<GameData>,
){
    if game_data.next_action{
        if game_update.events.len() > 0{
            game_data.feed.push(game_update.events[0].clone());
            game_update.events.remove(0);
        }
        game_data.next_action = false;
    }
}