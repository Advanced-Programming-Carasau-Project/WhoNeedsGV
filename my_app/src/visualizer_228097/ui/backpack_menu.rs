use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{ChildBuilder, Color, default, Text, TextAlignment, TextSection, TextStyle};
use bevy_ui::node_bundles::{ImageBundle, NodeBundle, TextBundle};
use bevy_ui::{AlignContent, AlignItems, AlignSelf, Display, FlexDirection, JustifySelf, PositionType, RepeatedGridTrack, Style, Val};
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Content::*;
use crate::visualizer_228097::backpack::component::ContentHub;

pub fn spawn_backpack(asset_server: &AssetServer, menu: &mut ChildBuilder){
    //Backpack
    menu.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(44.0),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::PURPLE.into(),
        ..default()
    }).with_children(|backpack| {

        //Title
        spawn_title(&asset_server, backpack);

        //Items menu
        spawn_items_menu(&asset_server, backpack);
    });
}
fn spawn_title(asset_server: &AssetServer, backpack: &mut ChildBuilder){
    //Titolo
    backpack.spawn(
        TextBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                display: Display::Flex,
                align_self: AlignSelf::Center,
                position_type: PositionType::Relative,
                ..default()
            },
            text: Text {
                sections: vec![TextSection::new(
                    "         Backpack",
                    TextStyle {
                        font_size: 58.0,
                        color: Color::WHITE,
                        font: asset_server.load("font/Country_Wedding.ttf").into(),
                    },
                )],
                ..default()
            },
            background_color: Color::DARK_GREEN.into(),
            ..default()
        }
    );

}
fn spawn_items_menu(asset_server: &AssetServer, backpack: &mut ChildBuilder){
//Menu con gli items
    backpack.spawn(NodeBundle {
        style: Style {
            display: Display::Grid,     //Layout di tipo griglia
            width: Val::Percent(100.0),
            height: Val::Percent(80.0),
            position_type: PositionType::Relative,
            grid_template_columns: RepeatedGridTrack::flex(3, 1.0), //(nElementi in ogni riga, dimensione di ogni cella)
            grid_template_rows: RepeatedGridTrack::flex(3 , 1.0),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        background_color: Color::LIME_GREEN.into(),
        ..default()
    }).with_children(|menu_items| {
        spawn_item(&asset_server, menu_items, Bush(0), "bush.png".to_string());
        spawn_item(&asset_server, menu_items, Coin(0), "coin.png".to_string());
        spawn_item(&asset_server, menu_items, Water(0), "water.png".to_string());
        spawn_item(&asset_server, menu_items, Fish(0), "fish.png".to_string());
        spawn_item(&asset_server, menu_items, Garbage(0), "garbage.png".to_string());
        spawn_item(&asset_server, menu_items, JollyBlock(0), "jolly_block.png".to_string());
        spawn_item(&asset_server, menu_items, Scarecrow, "scarecrow.png".to_string());
        spawn_item(&asset_server, menu_items, Rock(0), "rock.png".to_string());
        spawn_item(&asset_server, menu_items, Tree(0), "tree.png".to_string());
    });
}
fn spawn_item(asset_server: &AssetServer, menu_items: &mut ChildBuilder, content: Content, path: String, ) {

    let mut complete_path = String::from("sprites/contents/");
    complete_path.push_str(path.as_str());

    let style_item: Style = Style {
        width: Val::Percent(70.0),
        height: Val::Percent(70.0),
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        align_items: AlignItems::Center,
        align_content: AlignContent::Center,
        ..default()
    };

    let style_children: Style = Style {
        width: Val::Percent(50.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Relative,
        align_self: AlignSelf::Center,
        ..default()
    };

    let text_children: Text = Text {
        sections: vec![TextSection::new(
            "0",
            TextStyle{
                font_size: 50.0,
                color: Color::WHITE,
                font: asset_server.load("font/Country_Wedding.ttf").into(),
            },
        )],
        alignment: TextAlignment::Center,
        ..default()
    };

    menu_items.spawn(NodeBundle {
        style: style_item.clone(),
        background_color: Color::DARK_GREEN.into(),
        ..default()
    }).with_children(|menu_bush| {
        menu_bush.spawn(ImageBundle {
            style: style_children.clone(),
            image: asset_server.load(complete_path).into(),
            ..default()
        });

        menu_bush.spawn((TextBundle {
            style: style_children.clone(),
            text: text_children.clone(),
            ..default()
        }, ContentHub{ tile_content: content }
        ));
    });
}
