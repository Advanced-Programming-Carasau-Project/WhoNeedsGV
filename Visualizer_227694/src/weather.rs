use bevy::prelude::*;
use robotics_lib::world::environmental_conditions::*;
use robotics_lib::event::events::Event::*;
use crate::assets_loader::ImageAssets;
use crate::game_data::{GameData, MySet};
use crate::gui_overlay::{ClockComponent, ClockImageComponent};
use crate::rudimental_a_i::{events};

pub struct WeatherPlugin;

impl Plugin for WeatherPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,update_weather.in_set(MySet::Seventh));
    }
}
fn update_weather(mut light: ResMut<AmbientLight>,      // TOLO NON MI BASTA DEVO TROVARE UN MODO MIGLIORE PER VISUALIZZARE IL WEATHER
                  mut clear_color: ResMut<ClearColor>,  // TODO NON MI BASTA DEVO TROVARE UN MODO MIGLIORE PER VISUALIZZARE IL WEATHER
                  game_data: Res<GameData>,             // TOQO NON MI BASTA DEVO TROVARE UN MODO MIGLIORE PER VISUALIZZARE IL WEATHER
                  mut clock_query: Query<&mut Text,With<ClockComponent>>,
                  mut image_query: Query<&mut UiImage,With<ClockImageComponent>>,
                  mut dir_light_query: Query<&mut DirectionalLight>, //TODO cambiare in base al tempo (e orario)
                  image_assets: Res<ImageAssets>,
){
    if !game_data.next_action{
        return;
    }
    match events.try_lock() {
        Ok(mut events_guard) => {
            if events_guard.len() != 0 {
                let mut new_brightness = 0.85;
                let mut new_color_light = Color::rgb(0.8, 0.8, 0.8); // color of the light
                let mut new_weather = WeatherType::Sunny;
                let mut time_of_the_day;

                let mut clock = clock_query.single_mut();

                let mut is_night = false;

                match &events_guard[0] {
                    TimeChanged(environmental_conditions) => {
                        new_weather = environmental_conditions.get_weather_condition();
                        match environmental_conditions.get_time_of_day() {
                            DayTime::Morning => {}
                            DayTime::Afternoon => {}
                            DayTime::Night => { is_night = true; }
                        }
                        time_of_the_day = environmental_conditions.get_time_of_day_string();
                        events_guard.remove(0);
                    },
                    DayChanged(environmental_conditions) => {
                        new_weather = environmental_conditions.get_weather_condition();
                        match environmental_conditions.get_time_of_day() {
                            DayTime::Morning => {}
                            DayTime::Afternoon => {}
                            DayTime::Night => { is_night = true; }
                        }
                        time_of_the_day = environmental_conditions.get_time_of_day_string();
                        events_guard.remove(0);
                    },
                    _ => {
                        return;
                    }
                }

                clock.sections[0].value = time_of_the_day;

                let mut image = image_query.single_mut();
                let mut dir_light = dir_light_query.single_mut();

                match new_weather {
                    WeatherType::Sunny => {
                        if is_night{
                            new_brightness = 0.50;
                            clear_color.0 = Color::rgb(0.1,0.3,0.45); // bg color
                            new_color_light = Color::rgb(1.0, 1.0, 0.8);
                            image.texture = image_assets.night.clone();
                        }else {
                            new_brightness = 1.00;
                            clear_color.0 = Color::rgb(0.1,0.3,0.45); // bg color
                            new_color_light = Color::rgb(1.0, 1.0, 0.8);
                            image.texture = image_assets.sunny.clone();
                        }
                    }
                    WeatherType::Rainy => {
                        if is_night{
                            new_brightness = 0.75;
                            clear_color.0 = Color::rgb(0.2,0.4,0.55);
                            new_color_light = Color::rgb(0.8, 0.8, 1.0);
                            image.texture = image_assets.rainy_night.clone();
                        }else {
                            new_brightness = 0.75;
                            clear_color.0 = Color::rgb(0.2,0.4,0.55);
                            new_color_light = Color::rgb(0.8, 0.8, 1.0);
                            image.texture = image_assets.rainy.clone();
                        }
                    }
                    WeatherType::Foggy => {
                        if is_night{
                            new_brightness = 0.85;
                            clear_color.0 = Color::rgb(0.7,0.7,0.7);
                            new_color_light = Color::rgb(1.0, 1.0, 1.0);
                            image.texture = image_assets.foggy_night.clone();
                        }else {
                            new_brightness = 0.85;
                            clear_color.0 = Color::rgb(0.7,0.7,0.7);
                            new_color_light = Color::rgb(1.0, 1.0, 1.0);
                            image.texture = image_assets.foggy.clone();
                        }
                    }
                    WeatherType::TropicalMonsoon => {
                        if is_night{
                            new_brightness = 0.70;
                            clear_color.0 = Color::rgb(0.4,0.4,0.5);
                            new_color_light = Color::rgb(1.0, 0.8, 0.8);
                            image.texture = image_assets.tropical_monson_night.clone();
                        }else {
                            new_brightness = 0.70;
                            clear_color.0 = Color::rgb(0.4,0.4,0.5);
                            new_color_light = Color::rgb(1.0, 0.8, 0.8);
                            image.texture = image_assets.tropical_monson.clone();
                        }
                    }
                    WeatherType::TrentinoSnow => {
                        if is_night{
                            new_brightness = 0.70;
                            new_color_light = Color::rgb(1.0, 1.0, 1.0);
                            clear_color.0 = Color::rgb(0.8,0.8,0.8);
                            image.texture = image_assets.trentino_snow_night.clone();
                        }else {
                            new_brightness = 0.70;
                            new_color_light = Color::rgb(1.0, 1.0, 1.0);
                            clear_color.0 = Color::rgb(0.8,0.8,0.8);
                            image.texture = image_assets.trentino_snow.clone();
                        }
                    }
                }
                light.as_mut().brightness = new_brightness;
                light.as_mut().color = new_color_light;
            }

        }
        Err(_) => {
            return;
        }
        _ => {
            return;
        }
    }
}