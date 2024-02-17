mod ai_226840;
mod ai_226930;
mod visualizer_227694;
mod visualizer_228097;
mod test_tool;

use robotics_lib::runner::Runner;
use robotics_lib::runner::Runnable;

use robotics_lib::runner::{Robot};
use lazy_static::lazy_static;
use std::sync::{Mutex};

use rocket::{State};

use std::collections::HashMap;


use rip_worldgenerator::MyWorldGen;

use robotics_lib::event::events::Event;


use robotics_lib::world::tile::{Content, Tile};


use colored::Colorize;
use std::io;
use crate::ai_226840::MirtoRobot;
use crate::ai_226930::LunaticRobot;


use rocket::launch;
use rocket::get;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::serde::Serialize;



use bevy::prelude::Resource;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;

use crate::test_tool::run_test_tool;


use rocket::yansi::Paint;


// Static variables for data exchange between bevy and non bevy code
lazy_static! {
    // Store your variables here
    pub static ref points: Mutex<f32> = Mutex::new(0.00);
    pub static ref energy: Mutex<usize> = Mutex::new(0);
    pub static ref robot_view: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
    pub static ref positions: Mutex<(usize, usize)> = Mutex::new((0, 0));
    pub static ref backpack_content: Mutex<HashMap<Content, usize>> = Mutex::new(HashMap::new());
    pub static ref events: Mutex<Vec<Event>> = Mutex::new(vec![]);
}
pub struct RunnerTagRocket(Mutex<Runner>);

unsafe impl Sync for RunnerTagRocket {}
unsafe impl Send for RunnerTagRocket {}

#[derive(Serialize)]
pub enum SerEvent {
    Ready,
    Terminated,
    TimeChanged(EnvironmentalConditions),
    DayChanged(EnvironmentalConditions),
    EnergyRecharged(usize),
    EnergyConsumed(usize),
    Moved(Tile, (usize, usize)),
    TileContentUpdated(Tile, (usize, usize)),
    AddedToBackpack(Content, usize),
    RemovedFromBackpack(Content, usize),
}
#[derive(Serialize)]
struct Robot_Data {
    world_size: usize,
    world: Vec<Vec<Option<Tile>>>,
    positions: (usize, usize),
    points: f32,
    energy: usize,
    backpack: HashMap<String, usize>,
    ser_events: Vec<SerEvent>,
}

fn convert_event_to_serevent(event: &Event) -> SerEvent{
    match event {
        Event::Ready => { SerEvent::Ready }
        Event::Terminated => { SerEvent::Ready }
        Event::TimeChanged(t) => { SerEvent::TimeChanged(t.clone()) }
        Event::DayChanged(d) => { SerEvent::DayChanged(d.clone()) }
        Event::EnergyRecharged(e) => { SerEvent::EnergyRecharged(*e) }
        Event::EnergyConsumed(e) => { SerEvent::EnergyConsumed(*e) }
        Event::Moved(t, coord) => { SerEvent::Moved(t.clone(), *coord) }
        Event::TileContentUpdated(t, coord) => { SerEvent::TileContentUpdated(t.clone(), (coord.0, coord.1)) }
        Event::AddedToBackpack(c, q) => { SerEvent::AddedToBackpack(c.clone(), *q) }
        Event::RemovedFromBackpack(c, q) => { SerEvent::RemovedFromBackpack(c.clone(), *q) }
    }
}

fn convert_content_to_string(content: &Content) -> String{
    match content {
        Content::Rock(_) => { "ROCK".to_string() }
        Content::Tree(_) => { "TREE".to_string() }
        Content::Garbage(_) => { "GARBAGE".to_string() }
        Content::Fire => { "FIRE".to_string() }
        Content::Coin(_) => { "COIN".to_string() }
        Content::Bin(_) => { "BIN".to_string() }
        Content::Crate(_) => { "CRATE".to_string() }
        Content::Bank(_) => { "BANK".to_string() }
        Content::Water(_) => { "WATER".to_string() }
        Content::Market(_) => { "MARKET".to_string() }
        Content::Fish(_) => { "FISH".to_string() }
        Content::Building => { "BUILDING".to_string() }
        Content::Bush(_) => { "BUSH".to_string() }
        Content::JollyBlock(_) => { "JOLLYBLOCK".to_string() }
        Content::Scarecrow => { "SCARECROW".to_string() }
        Content::None => { "NONE".to_string() }
    }
}

#[get("/get_robot_data")]
fn get_robot_data(runner_tag: &State<RunnerTagRocket>) -> Json<Robot_Data> {
    let mut runner = runner_tag.0.lock().unwrap();

    println!("[.............................. ] processing game tick");
    runner.game_tick();

    let read_robot_view = robot_view.lock().unwrap();
    let read_points = points.lock().unwrap();
    let read_positions = positions.lock().unwrap();
    let read_energy = energy.lock().unwrap();
    let read_backpack_content = backpack_content.lock().unwrap();
    let mut read_events = events.lock().unwrap();

    let mut ser_events = vec![];
    ser_events.clear();
    for e in read_events.iter(){
        ser_events.push(convert_event_to_serevent(e));
    }
    for _i in 0..read_events.len(){
        read_events.pop();
    }

    let mut backpack = HashMap::new();
    for (key, value) in read_backpack_content.iter(){
        backpack.insert(convert_content_to_string(key), *value);
    }

    let mondo = Robot_Data {
        world_size: read_robot_view.len(),
        world: (*read_robot_view.clone()).to_vec(),
        positions: *read_positions,
        points: *read_points,
        energy: *read_energy,
        backpack,
        ser_events,
    };

    println!("[.............................. ] sending to JS");

    Json(mondo)
}

fn input_number() -> u32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Errore durante la lettura dell'input.");

    // Converto l'input in un numero intero unsigned a 32 bit
    let _number: u32 = match input.trim().parse() {
        Ok(num) => {return num;} ,
        Err(_) => {
            println!("Input non valido, inserisci un numero intero.");
            return 0;
        }
    };
}

const world_size: usize = 64;


#[derive(Resource)]
pub(crate) struct RunnerTag(pub(crate) Runner);
unsafe impl Sync for RunnerTag {}
unsafe impl Send for RunnerTag {}

#[launch]
fn rocket()->_{
    //compila il file typescript
    /*let status = Command::new("tsc")
        .arg("./static/visualizer.ts") // Percorso del tuo file TypeScript
        .arg("--lib")
        .arg("ES2015,DOM")
        .status()
        .expect("failed to execute TypeScript compiler");*/

    let mut choice;
    let mut robot:Box<dyn Runnable> = Box::new(LunaticRobot::new());
    //let mut whoneedsgv_wg = WorldGenerator::new(world_size);
    let mut rustinpeace_wg = MyWorldGen::new_param(world_size, 2, 2, 2, true, false, 3, false, None);
    //let mut whoneedsgv_runner;
    let mut rustinpeace_runner = Runner::new(Box::new(LunaticRobot::new()), &mut rustinpeace_wg);;
    let mut robot_bool_bevy = false;
    let mut test_tool = false;

    println!("Choose a mode: ");
    println!("1 - Start robot demonstration");
    println!("2 - visualize a world");
    println!("3 - tool"); //TODO

    let mut input_invalido = true;

    while input_invalido {
        choice = input_number();
        match choice {
            1 => { break; }
            2 => {
                println!("Choose a world: ");
                println!("1 - WhoNeedsGv");
                println!("2 - RustInPeace");

                let mut input_invalido = true;

                while input_invalido {
                    choice = input_number();
                    match choice {
                        1 => {
                            visualizer_227694::VisualizerGLC::visualize_world(100,false);

                        }
                        2 => {
                            visualizer_227694::VisualizerGLC::visualize_world(100,true);

                        }

                        _ => {
                            println!("invalid input");
                        }
                    }
                }
            }
            3 => {
                run_test_tool();
                test_tool = true;
                input_invalido = false;
            }
            _ => {
                println!("invalid input");
            }
        }
    }

    if !test_tool{
        println!("Choose a robot: ");
        println!("1 - MirtoRobot");
        println!("2 - LunaticRobot");

        let mut input_invalido = true;

        while input_invalido {
            choice = input_number();
            match choice {
                1 => {
                    robot_bool_bevy = true;
                    robot = Box::new(MirtoRobot::new(Robot::new(), true));
                    input_invalido = false;
                }
                2 => {
                    robot_bool_bevy = false;
                    robot = Box::new(LunaticRobot::new());
                    input_invalido = false;
                }
                _ => {
                    println!("invalid input");
                }
            }
        }

        rustinpeace_runner = Runner::new(robot, &mut rustinpeace_wg);

        println!("Choose a visualizer: ");
        println!("1 - Rocket");
        println!("2 - Bevy Giulio");
        println!("3 - Bevy Lorenzo");

        let mut input_invalido = true;

        while input_invalido {
            choice = input_number();
            match choice {
                1 => {
                    input_invalido = false;
                }
                2 => {
                    visualizer_227694::VisualizerGLC::run(robot_bool_bevy, world_size);
                    input_invalido = false;
                }
                3 => {
                    visualizer_228097::avvia_app(robot_bool_bevy);
                    input_invalido = false;
                }
                _ => {
                    println!("invalid input");
                }
            }
        }
    }
    rocket::build().manage(RunnerTagRocket(Mutex::new(rustinpeace_runner.expect("SIAMO PAZZI")))).mount("/", routes![get_robot_data]).mount("/", rocket::fs::FileServer::from("static"))
    //rocket::build().mount("/", routes![get_robot_data]).mount("/", rocket::fs::FileServer::from("static"))

}
