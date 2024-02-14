use std::sync::TryLockResult;
use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use robotics_lib::world::tile::*;
use crate::game_data::{GameData,MySet};
use crate::assets_loader::SceneAssets;
use robotics_lib::world::tile::Content::*;
use robotics_lib::event::events::Event::*;
use robotics_lib::world::tile::TileType::*;
use crate::rudimental_a_i::{events, robot_view};

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
#[derive(Component)]
struct Ground;
pub struct WorldPlugin;

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_ground_and_light)
            .add_systems(Update, discover_and_update_tile.in_set(MySet::Fourth))
            .add_systems(Update, discover_tile_under_robot.in_set(MySet::Third))
            .add_systems(Update,update_content.in_set(MySet::Fifth))
            .add_systems(Update, hide_content_under_robot.in_set(MySet::Sixth))
            .add_systems(Update, next_event_action.in_set(MySet::Eighth));
    }
}

fn create_ground_and_light(
    game_data: Res<GameData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(game_data.world_size as f32 * 2.0).into()),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(game_data.world_size as f32 / 2.0,-2.5,game_data.world_size as f32 / 2.0)),
            ..default()
        },
        Ground,
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

}
pub(crate) fn create_tile_model(){

}
fn discover_tile_under_robot(mut commands: Commands,
                            scene_assets: Res<SceneAssets>,
                            mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }
    match events.try_lock() {
        Ok(mut events_guard) => {
            if events_guard.len() != 0 {
                match &events_guard[0] {
                    Moved(tile,(x,z)) =>{
                        match &game_data.world[*x][*z] {
                            Some(old_tile) =>{
                            }
                            Option::None => {
                                game_data.world[*x][*z] = Some(tile.clone());
                                let coordinates = (*x as f32,*z as f32);
                                let new_tile_radius = f32::sqrt(((coordinates.0 - game_data.world_size as f32) * (coordinates.0 - game_data.world_size as f32)) + ((coordinates.1 - game_data.world_size as f32) * (coordinates.1 - game_data.world_size as f32)));
                                if new_tile_radius > game_data.map_radius {
                                    game_data.map_radius = new_tile_radius;
                                }

                                let tile_scene;
                                let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                                let content_scene;
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
                                    Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
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
                                        content_scene = scene_assets.bin.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                        content_transform.translation.y += 0.45;
                                    }
                                    Crate(_) => {
                                        content_scene = scene_assets.crate_.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                        content_transform.translation.y += 0.15;
                                    }
                                    Bank(_) => {
                                        content_scene = scene_assets.bank.clone();
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
                                        if game_data.ai{
                                            content_scene = scene_assets.mirto.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(2.2,2.2,2.2)).scale;
                                        }else {
                                            content_scene = scene_assets.jolly_block.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                        }
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
                                                    translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.5 ,coordinates.1).translation,
                                                    rotation: Default::default(),
                                                    scale: tile_scale,
                                                } ,
                                                ..default()
                                            },
                                        },
                                        TileComponent
                                    )
                                );
                                let mut visibility = Visibility::Visible;
                                if !game_data.content_visibility {
                                    visibility = Visibility::Hidden;
                                }
                                commands.spawn(
                                    (
                                        ContentBundle{
                                            model: SceneBundle{
                                                scene: content_scene,
                                                transform: content_transform,
                                                visibility,
                                                ..default()
                                            }
                                        },
                                        ContentComponent
                                    )
                                );
                            }
                        }
                        let world_guard = robot_view.lock().unwrap();
                        if *x > 0{
                            let tile = world_guard[*x - 1][*z].clone().unwrap();
                            match &game_data.world[*x - 1][*z] {
                                Some(old_tile) =>{
                                }
                                Option::None => {
                                    game_data.world[*x - 1][*z] = Some(tile.clone());
                                    let coordinates = ((*x - 1) as f32,*z as f32);
                                    let new_tile_radius = f32::sqrt(((coordinates.0 - game_data.world_size as f32) * (coordinates.0 - game_data.world_size as f32)) + ((coordinates.1 - game_data.world_size as f32) * (coordinates.1 - game_data.world_size as f32)));
                                    if new_tile_radius > game_data.map_radius {
                                        game_data.map_radius = new_tile_radius;
                                    }

                                    let tile_scene;
                                    let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                                    let content_scene;
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
                                        Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
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
                                            content_scene = scene_assets.bin.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                            content_transform.translation.y += 0.45;
                                        }
                                        Crate(_) => {
                                            content_scene = scene_assets.crate_.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            content_transform.translation.y += 0.15;
                                        }
                                        Bank(_) => {
                                            content_scene = scene_assets.bank.clone();
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
                                            if game_data.ai{
                                                content_scene = scene_assets.mirto.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(2.2,2.2,2.2)).scale;
                                            }else {
                                                content_scene = scene_assets.jolly_block.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            }
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
                                                        translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.5 ,coordinates.1).translation,
                                                        rotation: Default::default(),
                                                        scale: tile_scale,
                                                    } ,
                                                    ..default()
                                                },
                                            },
                                            TileComponent
                                        )
                                    );
                                    let mut visibility = Visibility::Visible;
                                    if !game_data.content_visibility {
                                        visibility = Visibility::Hidden;
                                    }
                                    commands.spawn(
                                        (
                                            ContentBundle{
                                                model: SceneBundle{
                                                    scene: content_scene,
                                                    transform: content_transform,
                                                    visibility,
                                                    ..default()
                                                }
                                            },
                                            ContentComponent
                                        )
                                    );
                                }
                            }
                        }
                        if *z > 0{
                            let tile = world_guard[*x][*z - 1].clone().unwrap();
                            match &game_data.world[*x][*z - 1] {
                                Some(old_tile) =>{
                                }
                                Option::None => {
                                    game_data.world[*x][*z - 1] = Some(tile.clone());
                                    let coordinates = (*x as f32,(*z - 1) as f32);
                                    let new_tile_radius = f32::sqrt(((coordinates.0 - game_data.world_size as f32) * (coordinates.0 - game_data.world_size as f32)) + ((coordinates.1 - game_data.world_size as f32) * (coordinates.1 - game_data.world_size as f32)));
                                    if new_tile_radius > game_data.map_radius {
                                        game_data.map_radius = new_tile_radius;
                                    }

                                    let tile_scene;
                                    let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                                    let content_scene;
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
                                        Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
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
                                            content_scene = scene_assets.bin.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                            content_transform.translation.y += 0.45;
                                        }
                                        Crate(_) => {
                                            content_scene = scene_assets.crate_.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            content_transform.translation.y += 0.15;
                                        }
                                        Bank(_) => {
                                            content_scene = scene_assets.bank.clone();
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
                                            if game_data.ai{
                                                content_scene = scene_assets.mirto.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(2.2,2.2,2.2)).scale;
                                            }else {
                                                content_scene = scene_assets.jolly_block.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            }
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
                                                        translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.5 ,coordinates.1).translation,
                                                        rotation: Default::default(),
                                                        scale: tile_scale,
                                                    } ,
                                                    ..default()
                                                },
                                            },
                                            TileComponent
                                        )
                                    );
                                    let mut visibility = Visibility::Visible;
                                    if !game_data.content_visibility {
                                        visibility = Visibility::Hidden;
                                    }
                                    commands.spawn(
                                        (
                                            ContentBundle{
                                                model: SceneBundle{
                                                    scene: content_scene,
                                                    transform: content_transform,
                                                    visibility,
                                                    ..default()
                                                }
                                            },
                                            ContentComponent
                                        )
                                    );
                                }
                            }
                        }
                        if *z < game_data.world_size - 1{
                            let tile = world_guard[*x][*z + 1].clone().unwrap();
                            match &game_data.world[*x][*z + 1] {
                                Some(old_tile) =>{
                                }
                                Option::None => {
                                    game_data.world[*x][*z + 1] = Some(tile.clone());
                                    let coordinates = (*x as f32,(*z + 1) as f32);
                                    let new_tile_radius = f32::sqrt(((coordinates.0 - game_data.world_size as f32) * (coordinates.0 - game_data.world_size as f32)) + ((coordinates.1 - game_data.world_size as f32) * (coordinates.1 - game_data.world_size as f32)));
                                    if new_tile_radius > game_data.map_radius {
                                        game_data.map_radius = new_tile_radius;
                                    }

                                    let tile_scene;
                                    let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                                    let content_scene;
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
                                        Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
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
                                            content_scene = scene_assets.bin.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                            content_transform.translation.y += 0.45;
                                        }
                                        Crate(_) => {
                                            content_scene = scene_assets.crate_.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            content_transform.translation.y += 0.15;
                                        }
                                        Bank(_) => {
                                            content_scene = scene_assets.bank.clone();
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
                                            if game_data.ai{
                                                content_scene = scene_assets.mirto.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(2.2,2.2,2.2)).scale;
                                            }else {
                                                content_scene = scene_assets.jolly_block.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            }
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
                                                        translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.5 ,coordinates.1).translation,
                                                        rotation: Default::default(),
                                                        scale: tile_scale,
                                                    } ,
                                                    ..default()
                                                },
                                            },
                                            TileComponent
                                        )
                                    );
                                    let mut visibility = Visibility::Visible;
                                    if !game_data.content_visibility {
                                        visibility = Visibility::Hidden;
                                    }
                                    commands.spawn(
                                        (
                                            ContentBundle{
                                                model: SceneBundle{
                                                    scene: content_scene,
                                                    transform: content_transform,
                                                    visibility,
                                                    ..default()
                                                }
                                            },
                                            ContentComponent
                                        )
                                    );
                                }
                            }
                        }
                        if *x < game_data.world_size - 1{
                            let tile = world_guard[*x + 1][*z].clone().unwrap();
                            match &game_data.world[*x + 1][*z] {
                                Some(old_tile) =>{
                                }
                                Option::None => {
                                    game_data.world[*x + 1][*z] = Some(tile.clone());
                                    let coordinates = ((*x + 1) as f32,*z as f32);
                                    let new_tile_radius = f32::sqrt(((coordinates.0 - game_data.world_size as f32) * (coordinates.0 - game_data.world_size as f32)) + ((coordinates.1 - game_data.world_size as f32) * (coordinates.1 - game_data.world_size as f32)));
                                    if new_tile_radius > game_data.map_radius {
                                        game_data.map_radius = new_tile_radius;
                                    }

                                    let tile_scene;
                                    let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                                    let content_scene;
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
                                        Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
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
                                            content_scene = scene_assets.bin.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                            content_transform.translation.y += 0.45;
                                        }
                                        Crate(_) => {
                                            content_scene = scene_assets.crate_.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            content_transform.translation.y += 0.15;
                                        }
                                        Bank(_) => {
                                            content_scene = scene_assets.bank.clone();
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
                                            if game_data.ai{
                                                content_scene = scene_assets.mirto.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(2.2,2.2,2.2)).scale;
                                            }else {
                                                content_scene = scene_assets.jolly_block.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            }
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
                                                        translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.5 ,coordinates.1).translation,
                                                        rotation: Default::default(),
                                                        scale: tile_scale,
                                                    } ,
                                                    ..default()
                                                },
                                            },
                                            TileComponent
                                        )
                                    );
                                    let mut visibility = Visibility::Visible;
                                    if !game_data.content_visibility {
                                        visibility = Visibility::Hidden;
                                    }
                                    commands.spawn(
                                        (
                                            ContentBundle{
                                                model: SceneBundle{
                                                    scene: content_scene,
                                                    transform: content_transform,
                                                    visibility,
                                                    ..default()
                                                }
                                            },
                                            ContentComponent
                                        )
                                    );
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
        Err(_) => {
            return;
        }
        _ => {
            return;
        }
    }
}
fn discover_and_update_tile(mut commands: Commands,
                 scene_assets: Res<SceneAssets>,
                 mut game_data: ResMut<GameData>,
                 mut tile_query: Query<(&Transform,&mut Handle<Scene>),With<TileComponent>>,
){
    {
        if events.lock().unwrap().len() > 0 {
            //info!("Eventi");
            return;
        }
    }

    if !game_data.update_world{
        return;
    }
    game_data.update_world = false;

    match robot_view.try_lock() {
        Ok(world_guard) => {
            info!("Updating _ World");
            for i in 0..world_guard.len(){
                for j in 0..world_guard.len() {
                    match &world_guard[i][j] {
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
                                                    Teleport(_) => {
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
                                    let new_tile_radius = f32::sqrt(((coordinates.0 - game_data.world_size as f32) * (coordinates.0 - game_data.world_size as f32)) + ((coordinates.1 - game_data.world_size as f32) * (coordinates.1 - game_data.world_size as f32)));
                                    if new_tile_radius > game_data.map_radius {
                                        game_data.map_radius = new_tile_radius;
                                    }

                                    let tile_scene;
                                    let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                                    let content_scene;
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
                                        Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
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
                                            content_scene = scene_assets.bin.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                            content_transform.translation.y += 0.45;
                                        }
                                        Crate(_) => {
                                            content_scene = scene_assets.crate_.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            content_transform.translation.y += 0.15;
                                        }
                                        Bank(_) => {
                                            content_scene = scene_assets.bank.clone();
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
                                            if game_data.ai{
                                                content_scene = scene_assets.mirto.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(2.2,2.2,2.2)).scale;
                                            }else {
                                                content_scene = scene_assets.jolly_block.clone();
                                                content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                            }
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
                                                        translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.5 ,coordinates.1).translation,
                                                        rotation: Default::default(),
                                                        scale: tile_scale,
                                                    } ,
                                                    ..default()
                                                },
                                            },
                                            TileComponent
                                        )
                                    );
                                    let mut visibility = Visibility::Visible;
                                    if !game_data.content_visibility {
                                        visibility = Visibility::Hidden;
                                    }
                                    commands.spawn(
                                        (
                                            ContentBundle{
                                                model: SceneBundle{
                                                    scene: content_scene,
                                                    transform: content_transform,
                                                    visibility,
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
        Err(_) => {
            return;
        }
        _ => {
            return;
        }
    }
}
fn update_content(mut content_query: Query<(&mut Transform,&mut Handle<Scene>),With<ContentComponent>>,
                  scene_assets: Res<SceneAssets>,
                  mut game_data: ResMut<GameData>,
){
    match crate::rudimental_a_i::events.try_lock() {
        Ok(mut events_guard) => {
            if events_guard.len() != 0{
                match &events_guard[0] {
                    TileContentUpdated(new_tile, (x, z)) => {
                        let coordinates = (*x as f32, *z as f32);
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
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.01,0.01,0.01)).scale;
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
                                        if game_data.ai{
                                            *content_scene = scene_assets.mirto.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(2.2,2.2,2.2)).scale;
                                        }else {
                                            *content_scene = scene_assets.jolly_block.clone();
                                            content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                        }
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
                        game_data.feed.push(format!("Content updated on ({}, {})",x, z));
                        if game_data.feed.len() == 8{
                            game_data.feed.remove(7);
                        }
                        events_guard.remove(0);
                    }
                    _ => {
                        return;
                    }
                }
            }
        }
        Err(_) => {
            return;
        }
        _ => {
            return;
        }
    }
}
fn hide_content_under_robot(mut content_query: Query<(&mut Transform, &mut Visibility),With<ContentComponent>>,
                            mut game_data: ResMut<GameData>,
){
    if game_data.content_visibility{
        if (f32::floor(game_data.robot_data.robot_translation.x) != game_data.hided_content.0) || (f32::floor(game_data.robot_data.robot_translation.z) != game_data.hided_content.1) {
            let mut new_hidden_content= (0.0,0.0);
            for (transform,mut visibility) in content_query.iter_mut(){
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
fn next_event_action(mut game_data: ResMut<GameData>){
    if game_data.next_action{
        let mut events_guard = events.lock().unwrap();
        if events_guard.len() > 0{
            match &events_guard[0] {
                Ready => {
                    game_data.feed.push(format!("{}",Ready));
                    if game_data.feed.len() == 8{
                        game_data.feed.remove(7);
                    }
                    events_guard.remove(0);
                }
                _ => {}
            }

        }else if events_guard.len() == 0 && game_data.autoplay{
            game_data.next += 1;
        }

        game_data.next_action = false;
    }
}