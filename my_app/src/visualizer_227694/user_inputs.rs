use bevy::prelude::*;
use crate::visualizer_227694::camera::Camera3DComponent;


use crate::visualizer_227694::game_data::{GameData, MySet};
use crate::visualizer_227694::world::ContentComponent;

pub struct InputPlugin;

impl Plugin for InputPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,go_stop.in_set(MySet::First))
            .add_systems(Update,next.in_set(MySet::First))
            .add_systems(Update,back_pack_show_hide.in_set(MySet::First))
            .add_systems(Update,map_show_hide.in_set(MySet::First))
            .add_systems(Update,content_show_hide.in_set(MySet::First))
            .add_systems(Update,feed_show_hide.in_set(MySet::First));
    }
}

fn go_stop(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){
    if keyboard_input.just_pressed(KeyCode::Space){
        game_data.autoplay = !game_data.autoplay;
    }
}
fn next(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){
    if keyboard_input.just_pressed(KeyCode::Right){
        if game_data.next < 1{
            game_data.next += 1;
        }
    }
}
fn back_pack_show_hide(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){
    if keyboard_input.just_pressed(KeyCode::B) {
        if game_data.robot_data.back_pack_visibility == 0{
            game_data.robot_data.back_pack_visibility = 1;
        } else {
            game_data.robot_data.back_pack_visibility = 0;
        }
    }
}
fn feed_show_hide(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){
    if keyboard_input.just_pressed(KeyCode::F) {
        game_data.feed_visibility = !game_data.feed_visibility;
    }
}
fn map_show_hide(keyboard_input: Res<Input<KeyCode>>,
                 mut game_data: ResMut<GameData>,
                mut query: Query<&mut Transform,With<Camera3DComponent>>
){ ///Pressing M the user can visualize the entire world known///
    if keyboard_input.just_pressed(KeyCode::M) {
        if game_data.camera_data.camera_mode != 3{
            game_data.camera_data.camera_mode_bu = game_data.camera_data.camera_mode;
            game_data.camera_data.camera_direction_bu = game_data.camera_data.camera_direction.clone();
            game_data.camera_data.camera_transform_bu = game_data.camera_data.camera_transform;
            game_data.camera_data.camera_velocity_bu = game_data.camera_data.camera_velocity;

            game_data.camera_data.camera_mode = 3;
            game_data.camera_data.camera_direction = crate::visualizer_227694::Direction::Up;
            game_data.camera_data.camera_transform = Transform::from_xyz(0.0,0.0,0.0).looking_at(Vec3::ZERO,Vec3::Z);
            game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.world_size as f32/2.0,game_data.world_size as f32 * 1.3,game_data.world_size as f32/2.0).translation;
            game_data.camera_data.camera_velocity = Vec3::ZERO;
        }else {
            game_data.camera_data.camera_mode = game_data.camera_data.camera_mode_bu;
            game_data.camera_data.camera_direction = game_data.camera_data.camera_direction_bu.clone();
            game_data.camera_data.camera_transform = game_data.camera_data.camera_transform_bu;
            game_data.camera_data.camera_velocity = game_data.camera_data.camera_velocity_bu;
        }
        let mut camera_transform = query.single_mut();
        camera_transform.translation = game_data.camera_data.camera_transform.translation;
    }
}
fn content_show_hide(keyboard_input: Res<Input<KeyCode>>,
                     mut game_data: ResMut<GameData>,
                    mut query: Query<&mut Visibility,With<ContentComponent>>,
){ ///Pressing P the user can choose to hide or show all the contents///
    if keyboard_input.just_pressed(KeyCode::P) {
        if game_data.content_visibility{
            for mut i in query.iter_mut(){
                *i = Visibility::Hidden;
            }
            game_data.hided_content = (777777.0,777777.0); // a random number bigger than the biggest world size
        }else {
            for mut i in query.iter_mut(){
                *i = Visibility::Visible;
            }
        }
        game_data.content_visibility = !game_data.content_visibility;
    }
}