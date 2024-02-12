use bevy::prelude::*;
use robotics_lib::event::events::Event::*;
use robotics_lib::world::environmental_conditions::WeatherType::*;
use crate::game_data::{GameData, MySet};
use crate::world::ContentComponent;

pub struct InputPlugin;

impl Plugin for InputPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,go_stop)
            .add_systems(Update,back_pack_show_hide.in_set(MySet::First))
            .add_systems(Update,map_show_hide.in_set(MySet::First))
            .add_systems(Update,content_show_hide.in_set(MySet::First))
            .add_systems(Update,feed_show_hide.in_set(MySet::First));
    }
}

fn go_stop(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///

    if keyboard_input.pressed(KeyCode::Space) {
        game_data.autoplay = !game_data.autoplay;
    }
}
fn next(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///

    if keyboard_input.pressed(KeyCode::Right) && !game_data.autoplay{
        game_data.next += 1;
        if game_data.previous > 0 {
            game_data.previous -= 1;
        }

    }
}
fn previous(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///
    if keyboard_input.pressed(KeyCode::Left) && !game_data.autoplay{
        if game_data.next > 0 {
            game_data.next -= 1;
        }
        game_data.previous += 1;
    }
}
fn back_pack_show_hide(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///
    if keyboard_input.just_pressed(KeyCode::B) {
        if game_data.robot_data.back_pack_visibility == 0{
            game_data.robot_data.back_pack_visibility = 1;
        } else if game_data.robot_data.back_pack_visibility == 1 {
            game_data.robot_data.back_pack_visibility = 2;
        }else {
            game_data.robot_data.back_pack_visibility = 0;
        }
    }
}
fn feed_show_hide(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///
    if keyboard_input.just_pressed(KeyCode::F) {
        game_data.feed_visibility = !game_data.feed_visibility;
    }
}
fn map_show_hide(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///Pressing M the user can visualize the entire world known///
    if keyboard_input.just_pressed(KeyCode::M) {
        info!("MAPPAPAAAA");
        if game_data.camera_data.camera_mode != 3{
            game_data.camera_data.camera_mode_bu = game_data.camera_data.camera_mode;
            game_data.camera_data.camera_direction_bu = game_data.camera_data.camera_direction.clone();
            game_data.camera_data.camera_transform_bu = game_data.camera_data.camera_transform;
            game_data.camera_data.camera_velocity_bu = game_data.camera_data.camera_velocity;

            game_data.camera_data.camera_mode = 3;
            game_data.camera_data.camera_direction = crate::Direction::Up;
            game_data.camera_data.camera_transform = Transform::from_xyz(0.0,game_data.map_radius * 1.5,0.0).looking_at(Vec3::ZERO,Vec3::Z);
            game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x,game_data.map_radius * 1.5,game_data.robot_data.robot_translation.z).translation;
            game_data.camera_data.camera_velocity = Vec3::ZERO;
        }else {
            game_data.camera_data.camera_mode = game_data.camera_data.camera_mode_bu;
            game_data.camera_data.camera_direction = game_data.camera_data.camera_direction_bu.clone();
            game_data.camera_data.camera_transform = game_data.camera_data.camera_transform_bu;
            game_data.camera_data.camera_velocity = game_data.camera_data.camera_velocity_bu;
        }
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