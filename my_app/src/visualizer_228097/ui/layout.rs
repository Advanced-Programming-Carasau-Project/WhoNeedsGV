use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Camera2dBundle, Commands, NodeBundle, Res, ResMut, Style, Val};
//use crate::visualizer_228097::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::visualizer_228097::components::GameInfo;
use crate::visualizer_228097::ui::grid::grid;
use crate::visualizer_228097::ui::backpack_menu::spawn_backpack;
use crate::visualizer_228097::ui::game_stats_menu::spawn_game_stats;
use crate::visualizer_228097::ui::log::spawn_log;


pub fn generate_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_info: ResMut<GameInfo>
)
{
    commands.spawn(Camera2dBundle::default());
    // Create a root node to hold all the other nodes
    commands.spawn(NodeBundle {
        style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        background_color: Color::RED.into(),
        ..default()
    }).with_children(|parent| {
        //-------------World-------------------------
        grid(&asset_server, parent);
        //-------------Menu-------------------------
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(35.0) ,
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Percent(1.0),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        }).with_children(|menu| {

            spawn_backpack(&asset_server, menu);

            spawn_log(&asset_server, menu);

            spawn_game_stats(&asset_server, menu, &game_info);
        });
    });

    //println!("------------------Generata UI----------------------");
}


