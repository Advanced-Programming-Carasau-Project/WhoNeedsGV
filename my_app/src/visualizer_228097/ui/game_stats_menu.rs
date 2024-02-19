use bevy::asset::AssetServer;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{Color, default, ResMut};
use bevy_ui::node_bundles::NodeBundle;
use bevy_ui::{FlexDirection, PositionType, Style, Val};
use crate::visualizer_228097::components::GameInfo;
use crate::visualizer_228097::ui::environment_stats_menu::spawn_environment_stats_menu;
use crate::visualizer_228097::ui::robot_stats_menu::spawn_robot_stats;

pub fn spawn_game_stats(asset_server: &AssetServer, menu: &mut ChildBuilder, game_info: &ResMut<GameInfo>){

    menu.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(22.0),
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Row, //dispongo i figli in un'unica grande riga
            column_gap: Val::Percent(1.0),
            ..default()
        },
        background_color: Color::AZURE.into(),
        ..default()
    }
    ).with_children(|game_stats| {

        spawn_environment_stats_menu(&asset_server, game_stats, &game_info);
        spawn_robot_stats(&asset_server, game_stats);

    });
}

