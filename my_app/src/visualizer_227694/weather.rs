use bevy::prelude::*;
use robotics_lib::world::environmental_conditions::*;
use robotics_lib::event::events::Event::*;
use crate::visualizer_227694::assets_loader::ImageAssets;
use crate::visualizer_227694::game_data::{GameData, MySet};
use crate::visualizer_227694::gui_overlay::{ClockComponent, ClockImageComponent};
use crate::{events};

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
                let mut new_weather = WeatherType::Sunny;
                let time_of_the_day;

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
                let diffused_light = light.as_mut();

                match new_weather {
                    WeatherType::Sunny => {
                        if is_night{
                            dir_light.illuminance = 800.0;
                            dir_light.color = Color::rgb(0.95, 0.95, 1.0);
                            diffused_light.brightness = 0.90;
                            diffused_light.color = Color::rgb(0.95, 0.95, 1.0);
                            clear_color.0 = Color::rgb(0.1,0.1,0.1); // bg color
                            image.texture = image_assets.night.clone();
                        }else {
                            dir_light.illuminance = 100000.0; //direct sunlight intensity in lux
                            dir_light.color = Color::rgb(232.0/255.0, 218.0/255.0, 88.0/255.0); //directional light color
                            diffused_light.brightness = 1.50;
                            diffused_light.color = Color::rgb(232.0/255.0, 218.0/255.0, 91.0/255.0); //diffused light color
                            clear_color.0 = Color::rgb(0.1,0.3,0.45); // bg color
                            image.texture = image_assets.sunny.clone();
                        }
                    }
                    WeatherType::Rainy => {
                        if is_night{
                            dir_light.illuminance = 500.0;
                            dir_light.color = Color::rgb(140.0/255.0, 145.0/255.0, 202.0/255.0); //directional light color
                            diffused_light.brightness = 0.80;
                            diffused_light.color = Color::rgb(148.0/255.0, 154.0/255.0, 232.0/255.0);
                            clear_color.0 = Color::rgb(0.25,0.25,0.55);
                            image.texture = image_assets.rainy_night.clone();
                        }else {
                            dir_light.illuminance = 45000.0;
                            dir_light.color = Color::rgb(178.0/255.0, 174.0/255.0, 232.0/255.0); //directional light color
                            diffused_light.brightness = 0.95;
                            diffused_light.color = Color::rgb(148.0/255.0, 154.0/255.0, 232.0/255.0);
                            clear_color.0 = Color::rgb(0.2,0.4,0.55);
                            image.texture = image_assets.rainy.clone();
                        }
                    }
                    WeatherType::Foggy => {
                        if is_night{
                            dir_light.illuminance = 150.0;
                            dir_light.color = Color::rgb(196.0/255.0, 234.0/255.0, 255.0/255.0); //directional light color
                            diffused_light.brightness = 0.65;
                            diffused_light.color = Color::rgb(196.0/255.0, 234.0/255.0, 255.0/255.0);
                            clear_color.0 = Color::rgb(0.2,0.2,0.2);
                            image.texture = image_assets.foggy_night.clone();
                        }else {
                            dir_light.illuminance = 40000.0;
                            dir_light.color = Color::rgb(196.0/255.0, 234.0/255.0, 255.0/255.0); //directional light color
                            diffused_light.brightness = 0.80;
                            diffused_light.color = Color::rgb(0.8, 0.9, 1.0);
                            clear_color.0 = Color::rgb(0.7,0.7,0.7);
                            image.texture = image_assets.foggy.clone();
                        }
                    }
                    WeatherType::TropicalMonsoon => {
                        if is_night{
                            dir_light.illuminance = 1000.0;
                            dir_light.color = Color::rgb(254.0/255.0, 255.0/255.0, 212.0/255.0); //directional light color
                            diffused_light.brightness = 0.05;
                            diffused_light.color = Color::rgb(1.0, 1.0, 0.8);
                            clear_color.0 = Color::rgb(0.1,0.1,0.1);
                            image.texture = image_assets.tropical_monson_night.clone();
                        }else {
                            dir_light.illuminance = 9200.0;
                            dir_light.color = Color::rgb(254.0/255.0, 255.0/255.0, 212.0/255.0); //directional light color
                            diffused_light.brightness = 0.1;
                            diffused_light.color = Color::rgb(1.0, 1.0, 0.8);
                            clear_color.0 = Color::rgb(0.3,0.3,0.3);
                            image.texture = image_assets.tropical_monson.clone();
                        }
                    }
                    WeatherType::TrentinoSnow => {
                        if is_night{
                            dir_light.illuminance = 180.0;
                            dir_light.color = Color::rgb(1.0, 1.0, 1.0); //directional light color
                            diffused_light.brightness = 0.70;
                            diffused_light.color = Color::rgb(1.0, 1.0, 1.0);
                            clear_color.0 = Color::rgb(0.7,0.7,0.7);
                            image.texture = image_assets.trentino_snow_night.clone();
                        }else {
                            dir_light.illuminance = 3200.0;
                            dir_light.color = Color::rgb(1.0, 1.0, 1.0); //directional light color
                            diffused_light.brightness = 0.70;
                            diffused_light.color = Color::rgb(1.0, 1.0, 1.0);
                            clear_color.0 = Color::rgb(0.9,0.9,0.9);
                            image.texture = image_assets.trentino_snow.clone();
                        }
                    }
                }
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