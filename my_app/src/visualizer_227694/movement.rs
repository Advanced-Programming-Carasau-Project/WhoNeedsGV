use bevy::prelude::*;
use crate::visualizer_227694::ACTIONS_VELOCITY;
use crate::visualizer_227694::game_data::{GameData, MySet};
use crate::visualizer_227694::robot::*;

pub struct MovementPlugin;

const MOVEMENT_VELOCITY:f32 = 1.0 / ACTIONS_VELOCITY; // a multiplier constant to make move robot and camera at the correct velocity

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position_robot.in_set(MySet::Seventh))
            .add_systems(Update, update_position_camera.in_set(MySet::Seventh));
    }
}

fn update_position_robot(mut robot_query: Query<&mut Transform,With<RobotComponent>>,
                         game_data: Res<GameData>,
                         time: Res<Time>){ // moves the robot the right amount to arrive at the next tile for the next action
    let mut robot_transform = robot_query.single_mut();
    robot_transform.translation.x += game_data.robot_data.robot_velocity.x * time.delta_seconds() * MOVEMENT_VELOCITY;
    robot_transform.translation.y += game_data.robot_data.robot_velocity.y * time.delta_seconds() * MOVEMENT_VELOCITY;
    robot_transform.translation.z += game_data.robot_data.robot_velocity.z * time.delta_seconds() * MOVEMENT_VELOCITY;
}
fn update_position_camera(mut camera_query: Query<&mut Transform,With<crate::visualizer_227694::camera::Camera3DComponent>>,
                          game_data: Res<GameData>,
                          time: Res<Time>){
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x += game_data.camera_data.camera_velocity.x * time.delta_seconds() * MOVEMENT_VELOCITY;
    camera_transform.translation.y += game_data.camera_data.camera_velocity.y * time.delta_seconds() * MOVEMENT_VELOCITY;
    camera_transform.translation.z += game_data.camera_data.camera_velocity.z * time.delta_seconds() * MOVEMENT_VELOCITY;
}