use std::collections::HashMap;
use bevy::prelude::*;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::tile::Content::*;

#[derive(Debug)]
pub(crate) struct RobotData{
    pub(crate) back_pack:HashMap<Content,i32>,
    pub(crate) back_pack_update:HashMap<Content,i32>,
    pub(crate) back_pack_visibility:u8,
    pub(crate) robot_velocity:Vec3,
    pub(crate) robot_direction:crate::visualizer_227694::Direction,
    pub(crate) robot_translation:Vec3,
    pub(crate) energy:i32,
    pub(crate) max_energy:i32,
    pub(crate) energy_update:i32,
    pub(crate) points:f32,
    pub(crate) max_points:f32,
    pub(crate) points_update:f32,
}
impl RobotData{
    pub(crate) fn new()->Self{
        let mut back_pack = HashMap::new();
        back_pack.insert(Water(0).to_default(),0);
        back_pack.insert(Tree(0).to_default(),0);
        back_pack.insert(Rock(0).to_default(),0);
        back_pack.insert(Fish(0).to_default(),0);
        back_pack.insert(Coin(0).to_default(),0);
        back_pack.insert(Bush(0).to_default(),0);
        back_pack.insert(JollyBlock(0).to_default(),0);
        back_pack.insert(Garbage(0).to_default(),0);
        back_pack.insert(Scarecrow.to_default(),0);
        let back_pack_update = back_pack.clone();
        RobotData{
            back_pack,
            back_pack_update,
            back_pack_visibility: 1,
            robot_velocity: Vec3::ZERO,
            robot_direction: crate::visualizer_227694::Direction::Up,
            robot_translation: Vec3::ZERO,
            energy: 1000,
            max_energy: 1000,
            energy_update: 0,
            points: 0.0,
            max_points: 100.0,
            points_update: 0.0,
        }
    }
}
#[derive(Debug)]
pub(crate) struct CameraData{
    pub(crate) camera_mode:u8,
    pub(crate) camera_velocity:Vec3,
    pub(crate) camera_direction:crate::visualizer_227694::Direction,
    pub(crate) camera_transform:Transform,
    pub(crate) camera_mode_bu:u8,
    pub(crate) camera_velocity_bu:Vec3,
    pub(crate) camera_direction_bu:crate::visualizer_227694::Direction,
    pub(crate) camera_transform_bu:Transform,
}
impl CameraData{
    pub(crate) fn new()->Self{
        CameraData{
            camera_mode: 0,
            camera_velocity: Vec3::ZERO,
            camera_direction: crate::visualizer_227694::Direction::Up,
            camera_transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO,Vec3::Z),
            camera_mode_bu: 0,
            camera_velocity_bu: Vec3::ZERO,
            camera_direction_bu: crate::visualizer_227694::Direction::Up,
            camera_transform_bu: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO,Vec3::Z),
        }
    }
}
#[derive(Resource,Debug)]
pub(crate) struct GameData{ // a resource used to store all data concerning the game status
    pub(crate) autoplay:bool, // if true the game-tick is automatically called after all the actions of the previous one have been showed
    pub(crate) next:usize,
    pub(crate) world_size:usize, // the size of the world
    pub(crate) world:Vec<Vec<Option<Tile>>>, // state of the displayed world
    pub(crate) update_world:bool,
    pub(crate) robot_data:RobotData, //data concerning robot status
    pub(crate) camera_data:CameraData, //data concerning robot status
    pub(crate) current_tile_elevation:f32,
    pub(crate) game_ticks:usize, // counter of total passed game ticks
    pub(crate) timer:Timer, // a Timer used to determine when to perform the next robot action
    pub(crate) next_action:bool,
    pub(crate) frames:usize, // used only for performance checks
    pub(crate) feed:Vec<String>, // a record of the last performed actions from the robot
    pub(crate) feed_visibility:bool,
    pub(crate) hided_content:(f32,f32), // used to hide the content under the robot
    pub(crate) content_visibility:bool,
    pub(crate) ai:bool, // True -> MirtoRobot - False -> LuaticRobot
}
#[derive(SystemSet,Debug,Hash,Eq, PartialEq,Clone)]
pub enum MySet{ // in this way I ensure that Systems executed every frame are executed in parallel only in a non-conflictual way
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (MySet::First,MySet::Second,MySet::Third,MySet::Fourth,MySet::Fifth,MySet::Sixth,MySet::Seventh,MySet::Eighth).chain(),
            )
            .insert_resource(ClearColor(Color::rgb(0.1,0.3,0.45)))
            .insert_resource(AmbientLight{
                color: Color::rgb(1.0, 1.0, 0.8),
                brightness: 1.0,
            })
            .add_systems(Update, update_game_data.in_set(MySet::Second));
    }
}

fn update_game_data(mut game_data: ResMut<GameData>,
                    time: Res<Time>,
){
    game_data.timer.tick(time.delta());
    game_data.frames += 1;
    if !game_data.timer.just_finished(){
        return;
    }else {
        //info!("frames{}",game_data.frames);
        game_data.frames = 0;
        game_data.next_action = true;
        game_data.robot_data.robot_velocity = Vec3::ZERO;
        if game_data.camera_data.camera_mode != 3{
            game_data.camera_data.camera_velocity = Vec3::ZERO;
        }else {
            game_data.camera_data.camera_velocity_bu = Vec3::ZERO;
        }
    }
}