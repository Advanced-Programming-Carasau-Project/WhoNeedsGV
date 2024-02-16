use bevy::asset::AssetServer;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::*;
use bevy_ui::{AlignItems, FlexDirection, PositionType, Style, Val};
use bevy_ui::node_bundles::{NodeBundle, TextBundle};
use crate::visualizer_228097::energy::components::IsEnergy;
use crate::visualizer_228097::stats::components::{Elevation, Score};

pub fn spawn_robot_stats(
    asset_server: &AssetServer,
    menu: &mut ChildBuilder,
)
{

    menu.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(70.0),
            height:  Val::Percent(100.0),
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Percent(1.0),
            ..default()
        },
        background_color: Color::AZURE.into(),
        ..default()
    }).with_children(|robot_stats| {
        //Elevation
        spawn_stats_menu(&asset_server,Elevation{}, robot_stats, "Elevation:".to_string());

        //Score
        spawn_stats_menu(&asset_server, Score{}, robot_stats, "Score:".to_string());

        //Energy
        spawn_energy_menu(&asset_server, robot_stats);

    });
}

fn spawn_stats_menu(asset_server: &AssetServer, component: impl Component, robot_stats: &mut ChildBuilder, text: String){

    let style1 = Style {
        width: Val::Percent(100.0),
        height: Val::Percent(25.0),
        position_type: PositionType::Relative,
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..default()
    };


    let style2 = Style {
        width: Val::Percent(50.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Relative,
        justify_self: JustifySelf::Center,
        align_self: AlignSelf::Center,
        ..default()
    };

    //Punteggio
    robot_stats.spawn(NodeBundle {
        style: style1.clone(),
        background_color: Color::AZURE.into(),
        ..default()
    }
    ).with_children(|punteggio| {
        //Etichetta
        punteggio.spawn(TextBundle {
            style: style2.clone(),
            text: Text {
                sections: vec![TextSection::new(
                    format!("      {}", text),
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        font: asset_server.load("font/FiraSans-Bold.ttf").into(),
                    },
                )],
                alignment: TextAlignment::Right,
                ..default()
            },
            background_color: Color::MIDNIGHT_BLUE.into(),
            ..default()
        });

        //Valore
        punteggio.spawn((TextBundle {
            style: style2.clone(),
            text: Text {
                sections: vec![TextSection::new(
                    "",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        font: asset_server.load("font/FiraSans-Bold.ttf").into(),
                    },
                )],
                alignment: TextAlignment::Right,
                ..default()
            },
            background_color: Color::MIDNIGHT_BLUE.into(),
            ..default()
        }, component
        ));
    });
}

fn spawn_energy_menu(asset_server: &AssetServer, robot_stats: &mut ChildBuilder){
    robot_stats.spawn((TextBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            position_type: PositionType::Relative,
            ..default()
        },
        text: Text {
            sections: vec![TextSection::new(
                "----------------------------------",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: asset_server.load("font/FiraSans-Bold.ttf").into(),
                },
            ), TextSection::new(
                "\nEnergy: 100% (1000)\n",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    font: asset_server.load("font/FiraSans-Bold.ttf").into(),
                },
            ), TextSection::new(
                "----------------------------------",
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
    }, IsEnergy {}
    ));
}