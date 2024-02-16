use robotics_lib::world::environmental_conditions::{DayTime, EnvironmentalConditions, WeatherType};
use bevy::prelude::{EventReader, Query, Res, With};
use bevy_ui::UiImage;
use bevy::asset::AssetServer;
use crate::events::TimeChanged;
use crate::meteo::components::Meteo;

pub fn get_meteo_path(env_conditions: EnvironmentalConditions) -> String {
    match env_conditions.get_weather_condition() {
        WeatherType::Sunny => {
            match env_conditions.get_time_of_day() {
                DayTime::Morning => { return "sprites/weather/Sunny.png".to_string(); }
                DayTime::Afternoon => { return "sprites/weather/Afternoon.png".to_string(); }
                DayTime::Night => { return "sprites/weather/Night.png".to_string(); }
            }
        }
        WeatherType::Rainy => { return "sprites/weather/Rainy.png".to_string(); }
        WeatherType::Foggy => { return "sprites/weather/Foggy.png".to_string(); }
        WeatherType::TropicalMonsoon => { return "sprites/weather/TropicalMonsoon.png".to_string(); }
        WeatherType::TrentinoSnow => { return "sprites/weather/TrentinoSnow.png".to_string(); }
    }
}

pub fn update_meteo(
    mut query: Query<&mut UiImage, With<Meteo>>,
    asset_server: Res<AssetServer>,
    mut er_time_changed: EventReader<TimeChanged>,
)
{
    for event in er_time_changed.read() {
        if let Ok(mut i) = query.get_single_mut() {
            i.texture = asset_server.load(get_meteo_path(event.new_environmental_conditions.clone())).into();
            //println!("------------->Meteo: {:?}", event.new_environmental_conditions.get_weather_condition());
        }
    }
}
