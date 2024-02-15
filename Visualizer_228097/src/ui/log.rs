use bevy::asset::AssetServer;
use bevy::prelude::{ChildBuilder, Color, default, Text, TextAlignment, TextSection, TextStyle};
use bevy_ui::node_bundles::TextBundle;
use bevy_ui::{PositionType, Style, Val};
use crate::stats::components::Log;
use crate::stats::components::N_EVENT_IN_LOG;

pub fn spawn_log(asset_server: &AssetServer, menu: &mut ChildBuilder){
    let style_log = TextStyle {
        font_size: 15.0,
        color: Color::BLACK,
        font: asset_server.load("font/FiraSans-Bold.ttf").into()
    };

    let mut v:Vec<TextSection> = Vec::new();
    for _i in 0..N_EVENT_IN_LOG { v.push(TextSection::new("\n", style_log.clone())); }

    //Log
    menu.spawn((TextBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(30.0),
            position_type: PositionType::Relative,
            ..default()
        },
        text: Text {
            sections: v,
            alignment: TextAlignment::Left,
            ..default()
        },
        background_color: Color::BEIGE.into(),
        ..default()
    }, Log{}
    ));
}