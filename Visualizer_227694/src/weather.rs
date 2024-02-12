use bevy::prelude::*;
use robotics_lib::world::environmental_conditions::*;
use robotics_lib::event::events::Event::*;
use crate::GameUpdate;
use crate::game_data::{GameData, MySet};

pub struct WeatherPlugin;

impl Plugin for WeatherPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,update_weather.in_set(MySet::Third));
    }
}
fn update_weather(mut light: ResMut<AmbientLight>,      // TOLO NON MI BASTA DEVO TROVARE UN MODO MIGLIORE PER VISUALIZZARE IL WEATHER
                  mut clear_color: ResMut<ClearColor>,  // TODO NON MI BASTA DEVO TROVARE UN MODO MIGLIORE PER VISUALIZZARE IL WEATHER
                  mut game_update: ResMut<GameUpdate>,  // TOBO NON MI BASTA DEVO TROVARE UN MODO MIGLIORE PER VISUALIZZARE IL WEATHER
                  mut game_data: ResMut<GameData>,      // TOQO NON MI BASTA DEVO TROVARE UN MODO MIGLIORE PER VISUALIZZARE IL WEATHER
){
    if !game_data.next_action{
        return;
    }else {
        if game_update.events.len() != 0 {
            let mut new_brightness = 0.85;
            let mut new_color_light = Color::rgb(0.8, 0.8, 0.8); // color of the light
            let mut new_weather = WeatherType::Sunny;

            match &game_update.events[0] {
                TimeChanged(environmental_conditions) => {
                    todo!()
                    //new_weather = environmental_conditions;
                },
                DayChanged(environmental_conditions) => {
                    todo!()
                    //new_weather = environmental_conditions;
                },
                _ => {
                    return;
                }
            }
            match new_weather {
                WeatherType::Sunny => {
                    new_brightness = 1.00;
                    clear_color.0 = Color::rgb(0.1,0.3,0.45); // bg color
                    new_color_light = Color::rgb(1.0, 1.0, 0.8);
                }
                WeatherType::Rainy => {
                    new_brightness = 0.75;
                    clear_color.0 = Color::rgb(0.2,0.4,0.55);
                    new_color_light = Color::rgb(0.8, 0.8, 1.0);
                }
                WeatherType::Foggy => {
                    new_brightness = 0.85;
                    clear_color.0 = Color::rgb(0.7,0.7,0.7);
                    new_color_light = Color::rgb(1.0, 1.0, 1.0);
                }
                WeatherType::TropicalMonsoon => {
                    new_brightness = 0.70;
                    clear_color.0 = Color::rgb(0.4,0.4,0.5);
                    new_color_light = Color::rgb(1.0, 0.8, 0.8);
                }
                WeatherType::TrentinoSnow => {
                    new_brightness = 0.70;
                    new_color_light = Color::rgb(1.0, 1.0, 1.0);
                    clear_color.0 = Color::rgb(0.8,0.8,0.8);
                }
            }
            light.as_mut().brightness = new_brightness;
            light.as_mut().color = new_color_light;
        }

    }
}