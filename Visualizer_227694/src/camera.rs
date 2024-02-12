use bevy::prelude::*;
use crate::game_data::{GameData, MySet};
use crate::Direction;
use robotics_lib::event::events::Event::*;

const CAMERA_0_VERTICAL_DISTANCE:f32 = 10.0;
const CAMERA_1_HORIZONTAL_DISTANCE:f32 = 10.0;
const CAMERA_1_VERTICAL_DISTANCE:f32 = 4.0;
const CAMERA_1_INCLINATION:f32 = 15.0;

const CAMERA_2_HORIZONTAL_DISTANCE:f32 = 10.0;
const CAMERA_2_VERTICAL_DISTANCE:f32 = 4.0;
const CAMERA_2_INCLINATION:f32 = 15.0;

#[derive(Component)]
pub struct Camera3DComponent;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_camera)
            .add_systems(Update,change_camera)
            .add_systems(Update,camera_follow_robot.in_set(MySet::Third));
            //.add_systems(Update,camera_follow_mouse.in_set(MySet::Third)); TODO farlo meglio
    }
}

fn spawn_camera(mut commands: Commands,
                game_data: Res<GameData>
){
    commands.spawn((Camera3dBundle{
        transform: game_data.camera_data.camera_transform,
        ..default()
    }, Camera3DComponent));
}

fn change_camera(
    mut camera_query: Query<&mut Transform,With<Camera3DComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_data: ResMut<GameData>,
    time: Res<Time>,
){
    let mut camera_transform = camera_query.single_mut();
    if keyboard_input.just_pressed(KeyCode::C){
        if game_data.camera_data.camera_mode == 0 { //CAMERA 1 (fixed third person)
            game_data.camera_data.camera_transform = Transform::from_xyz(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + CAMERA_1_VERTICAL_DISTANCE, game_data.robot_data.robot_translation.z - CAMERA_1_HORIZONTAL_DISTANCE).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + 4.0, game_data.robot_data.robot_translation.z),Vec3::Y);
            game_data.camera_data.camera_transform.rotate_x(f32::to_radians(CAMERA_1_INCLINATION));
            *camera_transform = game_data.camera_data.camera_transform;
            game_data.camera_data.camera_mode = 1;
        }else if game_data.camera_data.camera_mode == 1{ //CAMERA 2 (robot third person)
            game_data.camera_data.camera_mode = 2;
            game_data.camera_data.camera_transform = Transform::from_xyz(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + CAMERA_2_VERTICAL_DISTANCE, game_data.robot_data.robot_translation.z - CAMERA_2_HORIZONTAL_DISTANCE).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + 4.0, game_data.robot_data.robot_translation.z), Vec3::Y);
            game_data.camera_data.camera_transform.rotate_x(f32::to_radians(CAMERA_2_INCLINATION));
            match game_data.camera_data.camera_direction {
                Direction::Right => {
                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x + CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y, game_data.robot_data.robot_translation.z).translation;
                    game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                }
                Direction::Left => {
                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x - CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y, game_data.robot_data.robot_translation.z).translation;
                    game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                }
                Direction::Up => {}
                Direction::Down => {
                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y, game_data.robot_data.robot_translation.z + CAMERA_2_HORIZONTAL_DISTANCE).translation;
                    game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                }
            }
            *camera_transform = game_data.camera_data.camera_transform;
        }else if game_data.camera_data.camera_mode == 2{//CAMERA 0 (free cam)
            game_data.camera_data.camera_transform =  Transform::from_xyz(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + CAMERA_0_VERTICAL_DISTANCE, game_data.robot_data.robot_translation.z).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, 0.0, game_data.robot_data.robot_translation.z),Vec3::Z);
            *camera_transform = game_data.camera_data.camera_transform;
            game_data.camera_data.camera_mode = 0;
        }
    }
    if keyboard_input.pressed(KeyCode::V) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.y += 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::X) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.y -= 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::K) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.x -= 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::H) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.x += 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::U) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.z += 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::J) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.z -= 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Y) && game_data.camera_data.camera_mode == 0 {
        camera_transform.rotate_y(1.0 * time.delta_seconds());
    }
    if keyboard_input.pressed(KeyCode::I) && game_data.camera_data.camera_mode == 0{
        camera_transform.rotate_y(-1.0 * time.delta_seconds());
    }
    if keyboard_input.pressed(KeyCode::Key7) && game_data.camera_data.camera_mode == 0 {
        camera_transform.rotate_x(1.0 * time.delta_seconds());
    }
    if keyboard_input.pressed(KeyCode::Key8) && game_data.camera_data.camera_mode == 0{
        camera_transform.rotate_x(-1.0 * time.delta_seconds());
    }
    game_data.camera_data.camera_transform = *camera_transform;

}
fn camera_follow_robot(
    mut camera_query: Query<&mut Transform,With<Camera3DComponent>>,
    mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }
    match crate::rudimental_a_i::events.try_lock() {
        Ok(events_guard) => {
            let mut camera_transform = camera_query.single_mut();
            *camera_transform = game_data.camera_data.camera_transform;
            if game_data.camera_data.camera_mode == 0{
                return;
            }
            if events_guard.len() > 0{
                match &events_guard[0]{
                    Moved(tile,(x,y)) =>{
                        let mut direction = game_data.robot_data.robot_direction.clone();
                        match (*x as f32 - f32::round(game_data.robot_data.robot_translation.x) , *y as f32 - f32::round(game_data.robot_data.robot_translation.z)) {
                            (0.0,1.0) => {
                                direction = Direction::Right;
                            }
                            (0.0,-1.0) => {
                                direction = Direction::Left;
                            }
                            (1.0,0.0) => {
                                direction = Direction::Up;
                            }
                            (-1.0,0.0) => {
                                direction = Direction::Down;
                            }
                            _ => { //Teleport only way the robot can move by more than 1 tile
                                let destination = (*x as f32,*y as f32);
                                let destination_elevation = tile.elevation as f32;

                                if game_data.camera_data.camera_mode == 1{
                                    camera_transform.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation/10.0, destination.1 - 5.0).translation;
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation/10.0, destination.1 - 5.0).translation;
                                }else if game_data.camera_data.camera_mode == 2 {
                                    match game_data.camera_data.camera_direction {
                                        Direction::Right => {
                                            game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0 + CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y + destination_elevation / 10.0, destination.1).translation;
                                        }
                                        Direction::Left => {
                                            game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0 - CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y + destination_elevation / 10.0, destination.1).translation;
                                        }
                                        Direction::Up => {
                                            game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 - CAMERA_2_HORIZONTAL_DISTANCE).translation;
                                        }
                                        Direction::Down => {
                                            game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 + CAMERA_2_HORIZONTAL_DISTANCE).translation;
                                        }
                                    }
                                    *camera_transform = game_data.camera_data.camera_transform;
                                }else if game_data.camera_data.camera_mode == 3 && game_data.camera_data.camera_mode != 0 {
                                    if game_data.camera_data.camera_mode_bu == 1{
                                        game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 - 5.0).translation;
                                    }else if game_data.camera_data.camera_mode_bu == 2 {
                                        match game_data.camera_data.camera_direction {
                                            Direction::Right => {
                                                game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0 + CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y + destination_elevation / 10.0, destination.1).translation;
                                            }
                                            Direction::Left => {
                                                game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0 - CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y + destination_elevation / 10.0, destination.1).translation;
                                            }
                                            Direction::Up => {
                                                game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 - CAMERA_2_HORIZONTAL_DISTANCE).translation;
                                            }
                                            Direction::Down => {
                                                game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 + CAMERA_2_HORIZONTAL_DISTANCE).translation;
                                            }
                                        }
                                    }
                                }
                                return;
                            }
                        }
                        let elevation = tile.elevation as f32;
                        match direction {
                            Direction::Right => {
                                if game_data.camera_data.camera_mode != 3{
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(camera_transform.translation.x - 1.0, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z).translation;
                                    if game_data.camera_data.camera_mode == 2{
                                        game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x + CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z).translation;
                                        match game_data.camera_data.camera_direction {
                                            Direction::Right => {}
                                            Direction::Left => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                                            }
                                            Direction::Up => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                                            }
                                            Direction::Down => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                                            }
                                        }
                                        *camera_transform = game_data.camera_data.camera_transform;
                                    }
                                    game_data.camera_data.camera_direction = Direction::Right;
                                    game_data.camera_data.camera_velocity = Vec3::new(-1.0,elevation/10.0,0.0);
                                }else {
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(camera_transform.translation.x - 1.0, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z).translation;
                                    if game_data.camera_data.camera_mode_bu == 2{
                                        game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x + CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z).translation;
                                        match game_data.camera_data.camera_direction_bu {
                                            Direction::Right => {}
                                            Direction::Left => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(180.0));
                                            }
                                            Direction::Up => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(-90.0));
                                            }
                                            Direction::Down => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(90.0));
                                            }
                                        }
                                    }
                                    game_data.camera_data.camera_direction_bu = Direction::Right;
                                    game_data.camera_data.camera_velocity_bu = Vec3::new(-1.0,elevation/10.0,0.0);
                                }
                            }
                            Direction::Left => {
                                if game_data.camera_data.camera_mode != 3 {
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(camera_transform.translation.x + 1.0, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z).translation;
                                    if game_data.camera_data.camera_mode == 2{
                                        game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x - CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z).translation;
                                        match game_data.camera_data.camera_direction {
                                            Direction::Right => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                                            }
                                            Direction::Left => {}
                                            Direction::Up => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                                            }
                                            Direction::Down => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                                            }
                                        }
                                        *camera_transform = game_data.camera_data.camera_transform;
                                    }
                                    game_data.camera_data.camera_direction = Direction::Left;
                                    game_data.camera_data.camera_velocity = Vec3::new(1.0, elevation / 10.0, 0.0);
                                }else {
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(camera_transform.translation.x + 1.0, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z).translation;
                                    if game_data.camera_data.camera_mode_bu == 2{
                                        game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x - CAMERA_2_HORIZONTAL_DISTANCE, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z).translation;
                                        match game_data.camera_data.camera_direction_bu {
                                            Direction::Right => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(180.0));
                                            }
                                            Direction::Left => {}
                                            Direction::Up => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(90.0));
                                            }
                                            Direction::Down => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(-90.0));
                                            }
                                        }
                                    }
                                    game_data.camera_data.camera_direction_bu = Direction::Left;
                                    game_data.camera_data.camera_velocity_bu = Vec3::new(1.0,elevation/10.0,0.0);
                                }
                            }
                            Direction::Up => {
                                if game_data.camera_data.camera_mode != 3 {
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(camera_transform.translation.x, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z + 1.0).translation;
                                    if game_data.camera_data.camera_mode == 2{
                                        game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z - CAMERA_2_HORIZONTAL_DISTANCE).translation;
                                        match game_data.camera_data.camera_direction {
                                            Direction::Right => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                                            }
                                            Direction::Left => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                                            }
                                            Direction::Up => {}
                                            Direction::Down => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                                            }
                                        }
                                        *camera_transform = game_data.camera_data.camera_transform;
                                    }
                                    game_data.camera_data.camera_direction = Direction::Up;
                                    game_data.camera_data.camera_velocity = Vec3::new(0.0, elevation / 10.0, 1.0);
                                }else {
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(camera_transform.translation.x, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z + 1.0).translation;
                                    if game_data.camera_data.camera_mode_bu == 2{
                                        game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z - CAMERA_2_HORIZONTAL_DISTANCE).translation;
                                        match game_data.camera_data.camera_direction_bu {
                                            Direction::Right => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(90.0));
                                            }
                                            Direction::Left => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(-90.0));
                                            }
                                            Direction::Up => {}
                                            Direction::Down => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(180.0));
                                            }
                                        }
                                    }
                                    game_data.camera_data.camera_direction_bu = Direction::Up;
                                    game_data.camera_data.camera_velocity_bu = Vec3::new(0.0,elevation/10.0,1.0);
                                }
                            }
                            Direction::Down => {
                                if game_data.camera_data.camera_mode != 3{
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(camera_transform.translation.x, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z - 1.0).translation;
                                    if game_data.camera_data.camera_mode == 2{
                                        game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z + CAMERA_2_HORIZONTAL_DISTANCE).translation;
                                        match game_data.camera_data.camera_direction {
                                            Direction::Right => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                                            }
                                            Direction::Left => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                                            }
                                            Direction::Up => {
                                                game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                                            }
                                            Direction::Down => {}
                                        }
                                        *camera_transform = game_data.camera_data.camera_transform;
                                    }
                                    game_data.camera_data.camera_direction = Direction::Down;
                                    game_data.camera_data.camera_velocity = Vec3::new(0.0,elevation/10.0,-1.0);
                                }else {
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(camera_transform.translation.x, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z - 1.0).translation;
                                    if game_data.camera_data.camera_mode_bu == 2{
                                        game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z + CAMERA_2_HORIZONTAL_DISTANCE).translation;
                                        match game_data.camera_data.camera_direction_bu {
                                            Direction::Right => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(-90.0));
                                            }
                                            Direction::Left => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(90.0));
                                            }
                                            Direction::Up => {
                                                game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(180.0));
                                            }
                                            Direction::Down => {}
                                        }
                                    }
                                    game_data.camera_data.camera_direction_bu = Direction::Down;
                                    game_data.camera_data.camera_velocity_bu = Vec3::new(0.0,elevation/10.0,-1.0);
                                }
                            }
                        }
                    }
                    _ => {}
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