use bevy::prelude::*;
use crate::game_data::{GameData, MySet};
use crate::robot::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position_robot.in_set(MySet::Seventh))
            .add_systems(Update, update_position_camera.in_set(MySet::Seventh));
    }
}

fn update_position_robot(mut robot_query: Query<&mut Transform,With<RobotComponent>>,
                         game_data: Res<GameData>,
                         time: Res<Time>){
    let mut robot_transform = robot_query.single_mut();
    robot_transform.translation.x += game_data.robot_data.robot_velocity.x * time.delta_seconds();
    robot_transform.translation.y += game_data.robot_data.robot_velocity.y * time.delta_seconds();
    robot_transform.translation.z += game_data.robot_data.robot_velocity.z * time.delta_seconds();
}
fn update_position_camera(mut camera_query: Query<&mut Transform,With<crate::camera::Camera3DComponent>>,
                          game_data: Res<GameData>,
                          time: Res<Time>){
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x += game_data.camera_data.camera_velocity.x * time.delta_seconds();
    camera_transform.translation.y += game_data.camera_data.camera_velocity.y * time.delta_seconds();
    camera_transform.translation.z += game_data.camera_data.camera_velocity.z * time.delta_seconds();
}