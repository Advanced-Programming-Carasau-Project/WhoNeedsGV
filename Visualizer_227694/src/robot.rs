use bevy::prelude::*;
use robotics_lib::event::events::Event::*;

use crate::assets_loader::SceneAssets;
use crate::game_data::{GameData, MySet};
use crate::rudimental_a_i::{events, points};

#[derive(Component,Debug)]
pub struct RobotComponent;
#[derive(Bundle)]
struct RobotBundle{
    model:SceneBundle,
}

pub struct RobotPlugin;

impl Plugin for RobotPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,spawn_robot)
            .add_systems(Update,move_robot.in_set(MySet::Third))
            .add_systems(Update,robot_energy.in_set(MySet::Third))
            .add_systems(Update,robot_points.in_set(MySet::Third))
            .add_systems(Update,fine_robot.in_set(MySet::Third));
    }
}
fn spawn_robot(mut commands: Commands,scene_assets: Res<SceneAssets>,
                game_data: Res<GameData>
){
    let mut transform = Transform::from_translation(Vec3::ZERO).looking_at(Vec3::Z,Vec3::ZERO);
    transform.translation = game_data.robot_data.robot_translation;
    commands.spawn((RobotBundle{
        model:SceneBundle{
            scene: scene_assets.robot.clone(),
            transform,
            ..default()
        },
    }, RobotComponent));
}
fn fine_robot(mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }
    match events.try_lock() {
        Ok(mut events_guard) => {
            if events_guard.len() != 0{
                match &events_guard[0] {
                    Terminated => {
                        //TODO schermo nero con scritta tipo "the robot terminated his task" e un bottone che cliccato fa terminare l'app (forse potrei anche mettere un bottone per riavviare)
                        game_data.feed.push(events_guard[0].clone());
                        events_guard.remove(0);
                    }
                    _ => { return; }
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
fn robot_energy(mut game_data: ResMut<GameData>){
    match events.try_lock() {
        Ok(mut events_guard) => {
            let mut energy_events = true;
            while energy_events {
                energy_events = false;
                if events_guard.len() != 0 {
                    match events_guard[0] {
                        EnergyRecharged(energy) => {
                            game_data.robot_data.energy_update = i32::min(game_data.robot_data.max_energy - game_data.robot_data.energy, energy as i32);
                            game_data.robot_data.energy += game_data.robot_data.energy_update;
                            game_data.feed.push(events_guard[0].clone());
                            events_guard.remove(0);
                            energy_events = true;
                        }
                        EnergyConsumed(energy) => {
                            game_data.robot_data.energy -= energy as i32;
                            game_data.robot_data.energy_update = energy as i32;
                            game_data.feed.push(events_guard[0].clone());
                            events_guard.remove(0);
                            energy_events = true;
                        }
                        _ => {return;}
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
fn robot_points(mut game_data: ResMut<GameData>){
    match points.try_lock() {
        Ok(points_guard) => {
            game_data.robot_data.points_update = *points_guard - game_data.robot_data.points;
            game_data.robot_data.points = *points_guard;
        }
        Err(_) => {
            return;
        }
        _ => {
            return;
        }
    }

}
fn robot_back_pack(mut game_data: ResMut<GameData>){
    if !game_data.next_action{
        return;
    }
    match events.try_lock() {
        Ok(mut events_guard) => {
            if events_guard.len() == 0 {
                return;
            }
            match &events_guard[0] {
                AddedToBackpack(content, n) => {
                    game_data.robot_data.back_pack_update.insert(content.to_default(),*n as i32);
                    game_data.feed.push(events_guard[0].clone());
                    events_guard.remove(0);
                },
                RemovedFromBackpack(content, n)=> {
                    game_data.robot_data.back_pack_update.insert(content.to_default(), - (*n as i32));
                    game_data.feed.push(events_guard[0].clone());
                    events_guard.remove(0);
                },
                _ => {
                    return;
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
fn move_robot(mut robot_query: Query<&mut Transform,With<RobotComponent>>,
              mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }

    match events.try_lock() {
        Ok(mut events_guard) => {
            let mut robot_transform = robot_query.single_mut();
            robot_transform.translation = game_data.robot_data.robot_translation;
            if events_guard.len() != 0 {
                match &events_guard[0] {
                    Moved(tile,(z,x)) =>{
                        let mut direction = game_data.robot_data.robot_direction.clone();
                        match (*x as f32 - f32::round(game_data.robot_data.robot_translation.x) , *z as f32 - f32::round(game_data.robot_data.robot_translation.z)) {
                            (0.0,1.0) => {
                                direction = crate::Direction::Right;
                            }
                            (0.0,-1.0) => {
                                direction = crate::Direction::Left;
                            }
                            (1.0,0.0) => {
                                direction = crate::Direction::Up;
                            }
                            (-1.0,0.0) => {
                                direction = crate::Direction::Down;
                            }
                            _ => { //Teleport only way the robot can move by more than 1 tile
                                let destination = (*x as f32,*z as f32);
                                let destination_elevation = tile.elevation as f32 - robot_transform.translation.y;

                                let mut robot_transform = robot_query.single_mut();
                                robot_transform.translation = Transform::from_xyz(destination.0, robot_transform.translation.y + destination_elevation/10.0, destination.1).translation;
                                game_data.robot_data.robot_translation = Transform::from_xyz(destination.0, robot_transform.translation.y + destination_elevation/10.0, destination.1).translation;
                                return;
                            }
                        }
                        let elevation = tile.elevation as f32;
                        match direction {
                            crate::Direction::Right => {
                                game_data.robot_data.robot_translation = Transform::from_xyz(robot_transform.translation.x - 1.0, robot_transform.translation.y + elevation/10.0, robot_transform.translation.z).looking_at(Vec3::ZERO, Vec3::Z).translation;
                                match game_data.robot_data.robot_direction {
                                    crate::Direction::Right => {}
                                    crate::Direction::Left => { robot_transform.rotate_y(f32::to_radians(180.0));}
                                    crate::Direction::Up => { robot_transform.rotate_y(f32::to_radians(-90.0));}
                                    crate::Direction::Down => { robot_transform.rotate_y(f32::to_radians(90.0));}
                                }
                                game_data.robot_data.robot_direction = crate::Direction::Right;
                                game_data.robot_data.robot_velocity = Vec3::new(-1.0,elevation/10.0,0.0);
                            }
                            crate::Direction::Left => {
                                game_data.robot_data.robot_translation = Transform::from_xyz(robot_transform.translation.x + 1.0, robot_transform.translation.y + elevation/10.0, robot_transform.translation.z).looking_at(Vec3::ZERO, Vec3::Z).translation;
                                match game_data.robot_data.robot_direction {
                                    crate::Direction::Right => { robot_transform.rotate_y(f32::to_radians(180.0));}
                                    crate::Direction::Left => {}
                                    crate::Direction::Up => { robot_transform.rotate_y(f32::to_radians(90.0));}
                                    crate::Direction::Down => { robot_transform.rotate_y(f32::to_radians(-90.0));}
                                }
                                game_data.robot_data.robot_direction = crate::Direction::Left;
                                game_data.robot_data.robot_velocity = Vec3::new(1.0,elevation/10.0,0.0);
                            }
                            crate::Direction::Up => {
                                game_data.robot_data.robot_translation = Transform::from_xyz(robot_transform.translation.x, robot_transform.translation.y + elevation/10.0, robot_transform.translation.z + 1.0).looking_at(Vec3::ZERO, Vec3::Z).translation;
                                match game_data.robot_data.robot_direction {
                                    crate::Direction::Right => { robot_transform.rotate_y(f32::to_radians(90.0));}
                                    crate::Direction::Left => { robot_transform.rotate_y(f32::to_radians(-90.0));}
                                    crate::Direction::Up => {}
                                    crate::Direction::Down => { robot_transform.rotate_y(f32::to_radians(180.0));}
                                }
                                game_data.robot_data.robot_direction = crate::Direction::Up;
                                game_data.robot_data.robot_velocity = Vec3::new(0.0,elevation/10.0,1.0);
                            }
                            crate::Direction::Down => {
                                game_data.robot_data.robot_translation = Transform::from_xyz(robot_transform.translation.x, robot_transform.translation.y + elevation/10.0, robot_transform.translation.z - 1.0).looking_at(Vec3::ZERO, Vec3::Z).translation;
                                match game_data.robot_data.robot_direction {
                                    crate::Direction::Right => { robot_transform.rotate_y(f32::to_radians(-90.0));}
                                    crate::Direction::Left => { robot_transform.rotate_y(f32::to_radians(90.0));}
                                    crate::Direction::Up => { robot_transform.rotate_y(f32::to_radians(180.0));}
                                    crate::Direction::Down => {}
                                }
                                game_data.robot_data.robot_direction = crate::Direction::Down;
                                game_data.robot_data.robot_velocity = Vec3::new(0.0,elevation/10.0,-1.0);
                            }
                        }
                        game_data.feed.push(events_guard[0].clone());
                        events_guard.remove(0);
                    }
                    _ => {
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