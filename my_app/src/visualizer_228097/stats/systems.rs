use bevy::prelude::{EventReader, Query, ResMut, Text, TextStyle, With};
use bevy::utils::default;
use robotics_lib::world::environmental_conditions::DayTime;
use crate::visualizer_228097::components::{GameInfo};
use crate::points as POINTS;
use crate::visualizer_228097::events::{Moved, TimeChanged};
use crate::visualizer_228097::stats::components::{ActualDayTime, Elevation, Hour, Log, Score};

pub fn update_hour(
    mut query: Query<&mut Text, With<Hour>>,
    mut er_time_changed: EventReader<TimeChanged>,
)
{
    for event in er_time_changed.read() {
        if let Ok(mut t) = query.get_single_mut() {
            let mut str_builder = String::from("       ");
            str_builder.push_str(event.new_environmental_conditions.get_time_of_day_string().as_str());
            t.sections[0].value = str_builder;
            //println!("------------->Ora: {:?}", event.new_environmental_conditions.get_time_of_day_string());
        }
    }

}

pub fn update_day_time(
    mut query: Query<&mut Text, With<ActualDayTime>>,
    mut er_time_changed: EventReader<TimeChanged>,
)
{
    for event in er_time_changed.read() {
        if let Ok(mut t) = query.get_single_mut() {
            match event.new_environmental_conditions.get_time_of_day() {
                DayTime::Morning => { t.sections[0].value = format!("    {:?}", DayTime::Morning); }
                DayTime::Afternoon => { t.sections[0].value = format!(" {:?}", DayTime::Afternoon); }
                DayTime::Night => { t.sections[0].value = format!("       {:?}", DayTime::Night); }
            }
            //println!("------------->DayTime: {:?}", event.new_environmental_conditions.get_time_of_day());
        }
    }
}

pub fn update_score(
    mut query: Query<(&mut Text, &Score), With<Score>>,
)
{
    let read_points = POINTS.lock().unwrap();

    if let Ok((mut t, _m)) = query.get_single_mut() {
        t.sections[0].value = read_points.to_string();
        //println!("------------->Punti: {:?}", read_points);
    }
}

pub fn update_elevation(
    mut query: Query<&mut Text, With<Elevation>>,
    mut er_move: EventReader<Moved>,
)
{
    for event in er_move.read(){
        if let Ok(mut t) = query.get_single_mut() {
            t.sections[0].value = event.next_tile.elevation.to_string();
            //println!("------------->Elevation: {:?}", event.next_tile.elevation);
        }
    }

}

pub fn update_log(
    mut query: Query<(&mut Text, &Log), With<Log>>,
    game_info: ResMut<GameInfo>,
)
{
    //println!("Dentro update_log");
    if let Ok((mut t, _e)) = query.get_single_mut() {
        for i in 0..game_info.event_vec.len() {
            t.sections[i].value = game_info.event_vec[i].clone();
        }
    }
}
