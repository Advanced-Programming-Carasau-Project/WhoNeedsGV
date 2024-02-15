use bevy::prelude::*;
use crate::components::{GameInfo, WORLD_SIZE};
use crate::connect_with_ai::ROBOT_VIEW;
use crate::events::{Moved, Ready};
use crate::states::AppState;
use crate::systems::{get_path_content, give_color};
use crate::world::components::TileHub;


pub fn is_intorno(r:usize, c: usize, position: (usize, usize)) -> bool {

    if position.0 == r {    //Left || Right
        if position.1 > 0 && position.1 - 1 == c { return true }    //Left
        if position.1 < WORLD_SIZE-1 && position.1 + 1 == c { return true } //Right
    }
    if position.1 == c {    //Up || Down
        if position.0 > 0 && position.0 - 1 == r { return true }    //Up
        if position.0 < WORLD_SIZE-1 && position.0 + 1 == r { return true}  //Down
    }

    return false;
}

pub fn move_robot(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BackgroundColor, &TileHub), With<TileHub>>,
    asset_server: Res<AssetServer>,
    mut game_info: ResMut<GameInfo>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut er_moved: EventReader<Moved>,
)
{
    for event in er_moved.read() {
        //println!("......................Dentro new_move_robot_3.........................");
        //println!("Il robot si trova in posizione [{}][{}]", game_info.robot_position.0, game_info.robot_position.1);
        //println!("E deve andare nel punto [{}][{}]", where_to_move.r, where_to_move.c);

        let mut new_position:(usize, usize) = game_info.robot_position;
        let read_robot_view = ROBOT_VIEW.lock().unwrap();

        if event.next_position.0 != game_info.robot_position.0 || event.next_position.1 != game_info.robot_position.1 {
            for (e, mut b, t) in query.iter_mut() {
                if  read_robot_view[t.r][t.c].is_some() {
                    if game_info.robot_position.0 == t.r && game_info.robot_position.1 == t.c { //Lascio la cella attuale
                        b.0 = give_color(&read_robot_view[t.r][t.c].clone().unwrap().tile_type);
                        //println!("Assegno a [{}][{}] un nuovo colore", t.r, t.c);
                    }
                    else {
                        if t.r == event.next_position.0 && t.c == event.next_position.1 {
                            b.0 = Color::GOLD;                                      //Occupo la nuova cella
                            commands.entity(e).clear_children();
                            new_position = (t.r, t.c);
                            //println!("Rendo [{}][{}] la nuova posizione del robot", t.r, t.c);
                        }
                    }
                    //println!("Prima di intorno: t[{}][{}], where_to_move[{}][{}]", t.r, t.c, where_to_move.r, where_to_move.c);
                    if is_intorno(t.r, t.c, event.next_position){
                        //println!("Intorno");
                        let tmp = read_robot_view[t.r][t.c].clone();
                        match tmp {
                            None => { }
                            Some(tile) => {
                                b.0 = give_color(&tile.tile_type);
                                //println!("Scoperta la tile [{}][{}]", t.r, t.c);
                                let child_entity = commands
                                    .spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Percent(90.0),
                                            height: Val::Percent(90.0),
                                            align_self: AlignSelf::Center,
                                            justify_self: JustifySelf::Center,
                                            ..default()
                                        },
                                        //background_color: BackgroundColor(give_color_content(&t.tile_content)),
                                        image: asset_server.load(get_path_content(&tile.content)).into(),
                                        ..default()
                                    })
                                    .id();
                                //commands.entity(e).replace_children(&[child_entity]); //non serve se uso discovered_map[t.r][t.c]
                                commands.entity(e).add_child(child_entity);
                            }
                        }

                    }
                }
            }
            game_info.robot_position = new_position;
            //println!("Scopro l'intorno di [{}][{}]", world.robot_position.0, world.robot_position.1);
            //mostra_intorno(&mut commands, &mut query, &mut world, &asset_server);
            //println!("Fatto");
            //println!("..........................Fine new_move_robot_3.............................");
            app_state_next_state.set(AppState::ReadingEvents);
        }
    }



}

pub fn spawn_robot(
    mut query: Query<(Entity, &mut BackgroundColor, &TileHub), With<TileHub>>,
    game_info: ResMut<GameInfo>,
    mut er_ready: EventReader<Ready>,
)
{
    for event in er_ready.read() {
        let read_robot_view = ROBOT_VIEW.lock().unwrap();

        for (e, mut b, t) in query.iter_mut() {
            if t.r == game_info.robot_position.0 && t.c == game_info.robot_position.1 {
                b.0 = Color::GOLD;
            }

            /*
            if (t.r == game_info.robot_position.0 && t.c == game_info.robot_position.1) || (new_is_intorno(t.r, t.c, game_info.robot_position)){
                if  read_robot_view[t.r][t.c].is_some() {
                    if t.r == game_info.robot_position.0 && t.c == game_info.robot_position.1 { b.0 = Color::GOLD; }
                    else { b.0 = give_color(&read_robot_view[t.r][t.c].clone().unwrap().tile_type); }

                    let child_entity = commands
                        .spawn(ImageBundle {
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(90.0),
                                align_self: AlignSelf::Center,
                                justify_self: JustifySelf::Center,
                                ..default()
                            },
                            //background_color: BackgroundColor(give_color_content(&t.tile_content)),
                            image: asset_server.load(get_path_content(&read_robot_view[t.r][t.c].clone().unwrap().content)).into(),
                            ..default()
                        })
                        .id();
                    commands.entity(e).replace_children(&[child_entity]);
                }
            }
            */
        }
    }



}
