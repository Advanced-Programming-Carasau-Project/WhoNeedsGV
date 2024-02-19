use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::*;
use bevy_ui::node_bundles::NodeBundle;
use bevy_ui::{Display, Style, Val};
use crate::world_size as WORLD_SIZE;
use crate::visualizer_228097::ui::components::Grid;
//use crate::visualizer_228097::WINDOW_HEIGHT;
use crate::visualizer_228097::world::components::TileHub;

pub fn grid(asset_server: &AssetServer, commands: &mut ChildBuilder) {

    commands.spawn((NodeBundle {
        style: Style {
            display: Display::Grid,     //Layout di tipo griglia
            width: Val::Percent(65.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Relative,
            grid_template_columns: RepeatedGridTrack::flex(WORLD_SIZE as u16, 1.0), //(nElementi in ogni riga, dimensione di ogni cella)
            grid_template_rows: RepeatedGridTrack::flex(WORLD_SIZE as u16 , 1.0),
            ..default()
        },
        background_color: BackgroundColor(Color::RED),
        ..default()
    }, Grid{}
    )).with_children(|builder| {

        for r in 0..WORLD_SIZE {
            for c in 0..WORLD_SIZE {

                let color1 = Color::GRAY;

                let path = "".to_string();

                builder.spawn((
                    NodeBundle {
                        style: Style { display: Display::Grid, ..default() },
                        background_color: BackgroundColor(color1), ..default()
                    },
                    TileHub { r, c }
                ))
                    .with_children(|parent|{
                        parent.spawn(ImageBundle {
                            style: Style {
                                width: Val::Percent(30.0),
                                height: Val::Percent(30.0),
                                align_self: AlignSelf::Center,
                                justify_self: JustifySelf::Center,
                                ..default()
                            },
                            image: asset_server.load(path).into(),
                            ..default()
                        });
                    })
                ;
            }
        }
    });
}