use bevy::prelude::*;
use bevy_ui::node_bundles::{ImageBundle, NodeBundle, TextBundle};
use bevy_ui::{AlignItems, AlignSelf, FlexDirection, PositionType, Style, Val};
use crate::visualizer_228097::components::GameInfo;
use crate::visualizer_228097::meteo::components::Meteo;
use crate::visualizer_228097::stats::components::{ActualDayTime, Hour};

pub fn spawn_environment_stats_menu(
    asset_server: &AssetServer,
    menu: &mut ChildBuilder,
    game_info: &ResMut<GameInfo>,
){
    menu.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(30.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Percent(1.0),
            ..default()
        },
        background_color: Color::AZURE.into(),
        ..default()
    }).with_children(|environment_stats| {

        //Style utilizzato per la barra Day e quella Time
        let style1 = Style {
            width: Val::Percent(100.0),
            height: Val::Percent(17.5),
            position_type: PositionType::Relative,
            align_self: AlignSelf::Center,
            ..default()
        };

        //DayTime
        environment_stats.spawn((TextBundle {
            style: style1.clone(),
            text: Text {
                sections: vec![TextSection::new(
                    format!("      {:?}", game_info.environmental_condition.get_time_of_day()),
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        font: asset_server.load("font/FiraSans-Bold.ttf").into(),
                    },
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            background_color: Color::MIDNIGHT_BLUE.into(),
            ..default()
        }, ActualDayTime{}
        ));

        //Hour
        environment_stats.spawn(
            (TextBundle {
                style: style1.clone(),
                text: Text {
                    sections: vec![TextSection::new(
                        format!("      {:?}", game_info.environmental_condition.get_time_of_day_string()),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            font: asset_server.load("font/FiraSans-Bold.ttf").into(),
                        },
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                background_color: Color::MIDNIGHT_BLUE.into(),
                ..default()
            }, Hour{}
            )
        );


        //Meteo
        environment_stats.spawn((ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(65.0),
                position_type: PositionType::Relative,
                align_self: AlignSelf::Center,
                ..default()
            },
            background_color: Color::AZURE.into(),
            ..default()
        }, Meteo{}
        ));



    });
}