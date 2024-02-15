
use robotics_lib::runner::Runner;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::Robot;
use bevy::ecs::system::Resource;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use bevy::ecs::system::Commands;
use rip_worldgenerator::MyWorldGen;
use crate::ai_226840::*;
use bevy::ecs::system::ResMut;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::get_score;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;
use robotics_lib::interface::robot_map;
use bevy::prelude::*;
use crate::components::{GameInfo, WORLD_SIZE};
use crate::energy::components::EnergyHub;
use crate::events::*;
use crate::states::AppState;
use crate::stats::components::N_EVENT_IN_LOG;
use crate::runner::RunnerTag;

// Static variables for data exchange between bevy and non bevy code
lazy_static! {
    // Store your variables here
    pub static ref EVENTS: Mutex<Vec<Event>> = Mutex::new(vec![]);
    pub static ref POINTS: Mutex<f32> = Mutex::new(0.00);
    pub static ref ROBOT_VIEW: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
}




pub fn update(
    mut remaining_energy: ResMut<EnergyHub>,
    mut game_info: ResMut<GameInfo>,
    mut runner: ResMut<RunnerTag>,

    mut ew_ready: EventWriter<Ready>,
    mut ew_terminated: EventWriter<Terminated>,
    mut ew_time_changed: EventWriter<TimeChanged>,
    mut ew_energy_update: EventWriter<EnergyUpdated>,
    mut ew_moved: EventWriter<Moved>,
    mut ew_tile_content_updated: EventWriter<TileContentUpdated>,
    mut ew_update_backpack: EventWriter<UpdateBackpack>,
)
{

    //thread::sleep(Duration::from_millis(5000));

        { // SERVE A LIBERARE I MUTEX SENZA ESPLICITARE UNLOCK

            let len;
            {
                let read_events = EVENTS.lock().unwrap();
                len = read_events.len();
            }

            if len == 0 { let _ = runner.0.game_tick(); }

            let mut read_events = EVENTS.lock().unwrap();
            //let read_points = POINTS.lock().unwrap();
            //let read_robot_view = ROBOT_VIEW.lock().unwrap();

            //println!("-------------------------------------------------");
            //println!("Events: {:?}", read_events);
            //thread::sleep(Duration::from_millis(3000));

            let mut continue_while = true;
            let mut tmp_for_energy = 0;
            let mut last_event = String::new();

            while continue_while {
                continue_while = false;



                let e = &read_events[0].clone();

                match e {
                    Event::Ready => {
                        //println!("Il robot è pronto -> {:?}", e);
                        //app_state_next_state.set(AppState::RobotIsReady);
                        ew_ready.send( Ready { } );

                        let mut str_builder = String::from("Robot is spawned in [");
                        str_builder.push_str(game_info.robot_position.0.to_string().as_str());
                        str_builder.push_str("][");
                        str_builder.push_str(game_info.robot_position.1.to_string().as_str());
                        str_builder.push_str("]\n");
                        last_event = str_builder;
                    }
                    Event::Terminated => {
                        //println!("Il robot è morto -> {:?}", e);
                        ew_terminated.send( Terminated { } );
                        last_event = String::from("Robot is dead\n");
                    }
                    Event::TimeChanged(ec) => {
                        //println!("Cambiato il tempo -> {:?}", "");

                        ew_time_changed.send( TimeChanged { new_environmental_conditions: ec.clone() });
                        continue_while = true;
                        let mut str_builder = String::from("Day changed -> [");
                        str_builder.push_str(game_info.environmental_condition.get_time_of_day_string().as_str());
                        str_builder.push_str("][");
                        str_builder.push_str(format!("{:?}", game_info.environmental_condition.get_weather_condition()).as_str());
                        str_builder.push_str("]\n");
                        last_event = str_builder;
                    }
                    Event::DayChanged(ec) => {
                        //println!("Cambiato il giorno -> {:?}", "");

                        ew_time_changed.send( TimeChanged { new_environmental_conditions: ec.clone() });
                        continue_while = true;
                        let mut str_builder = String::from("Day changed -> [");
                        str_builder.push_str(game_info.environmental_condition.get_time_of_day_string().as_str());
                        str_builder.push_str("][");
                        str_builder.push_str(format!("{:?}", game_info.environmental_condition.get_weather_condition()).as_str());
                        str_builder.push_str("]\n");
                        last_event = str_builder;
                    }
                    Event::EnergyRecharged(n) => {
                        if remaining_energy.energy + n > 1000 {
                            tmp_for_energy += 1000 - remaining_energy.energy;
                            remaining_energy.energy = 1000;
                        }
                        else {
                            tmp_for_energy += n;
                            remaining_energy.energy += n;
                        }

                        ew_energy_update.send( EnergyUpdated { total_energy: remaining_energy.energy } );

                        //println!("Recuperato {} energia, totale energia: {}", n, remaining_energy.energy);
                        continue_while = true;

                        let mut str_builder = String::from("Energy recharged (+");
                        str_builder.push_str(tmp_for_energy.to_string().as_str());
                        str_builder.push_str(") -> Remaining energy: ");
                        str_builder.push_str(remaining_energy.energy.to_string().as_str());
                        str_builder.push_str("\n");
                        last_event = str_builder;
                    }
                    Event::EnergyConsumed(n) => {
                        if remaining_energy.energy >= *n {
                            tmp_for_energy += n;
                            remaining_energy.energy -= n;
                        }
                        else {
                            tmp_for_energy += tmp_for_energy;   //giusto :)
                            remaining_energy.energy = 0;
                        }

                        ew_energy_update.send( EnergyUpdated { total_energy: remaining_energy.energy } );

                        //println!("Consumata {} energia, totale energia: {}", n, remaining_energy.energy);
                        continue_while = true;

                        let mut str_builder = String::from("Energy consumed (-");
                        str_builder.push_str(tmp_for_energy.to_string().as_str());
                        str_builder.push_str(") -> Remaining energy: ");
                        str_builder.push_str(remaining_energy.energy.to_string().as_str());
                        str_builder.push_str("\n");
                        last_event = str_builder;
                    }
                    Event::Moved(tile, position) => {
                        //println!("Letto evento Moved");

                        ew_moved.send( Moved { next_tile: tile.clone(), next_position: position.clone() });

                        let mut str_builder = String::new();
                        if game_info.first_interaction {
                            str_builder.push_str("Robot looks around\n");
                            game_info.first_interaction = false;
                        }
                        else {
                            str_builder.push_str("Moved from [");
                            str_builder.push_str(game_info.robot_position.0.to_string().as_str());
                            str_builder.push_str("][");
                            str_builder.push_str(game_info.robot_position.1.to_string().as_str());
                            str_builder.push_str("] to the tile [");
                            str_builder.push_str(position.0.to_string().as_str());
                            str_builder.push_str("][");
                            str_builder.push_str(position.1.to_string().as_str());
                            str_builder.push_str("] (");
                            str_builder.push_str(format!("{:?}", tile.tile_type).as_str());
                            str_builder.push_str(")\n");
                        }
                        last_event = str_builder;
                    }
                    Event::TileContentUpdated(t, p) =>  {
                        //println!("Casella aggiornata -> {:?}", e);

                        ew_tile_content_updated.send( TileContentUpdated{ new_tile: t.clone(), position: p.clone() });

                        let mut str_builder = String::from("Tile updated, now [");
                        str_builder.push_str(p.0.to_string().as_str());
                        str_builder.push_str("][");
                        str_builder.push_str(p.1.to_string().as_str());
                        str_builder.push_str("] became a ");
                        str_builder.push_str(format!("{:?}", t.tile_type.clone()).as_str());
                        str_builder.push_str(" with content: ");
                        str_builder.push_str(format!("{:?}", t.content.clone()).as_str());
                        str_builder.push_str(", and elevation: ");
                        str_builder.push_str(format!("{:?}", t.elevation.clone()).as_str());
                        str_builder.push_str("\n");
                        last_event = str_builder;
                    }
                    Event::AddedToBackpack(c, q) => {
                        //println!("Aggiunto allo zaino -> {:?}", e);
                        /*
                        item_backpack.item = Some(c.clone());
                        item_backpack.n = q.clone();
                        item_backpack.add = true;
                        */

                        ew_update_backpack.send( UpdateBackpack { content: c.clone(), n: q.clone(), add: true } );

                        let mut str_builder = String::from("Add to backpack ");
                        str_builder.push_str(q.to_string().as_str());
                        str_builder.push_str(" element of ");
                        str_builder.push_str(format!("{:?}", c).as_str());
                        str_builder.push_str("\n");
                        last_event = str_builder;
                    }
                    Event::RemovedFromBackpack(c, q) => {
                        //println!("Rimosso dallo zaino -> {:?}", e);

                        ew_update_backpack.send( UpdateBackpack { content: c.clone(), n: q.clone(), add: false } );

                        let mut str_builder = String::from("Removed from backpack ");
                        str_builder.push_str(q.to_string().as_str());
                        str_builder.push_str(" element of ");
                        str_builder.push_str(format!("{:?}", c).as_str());
                        str_builder.push_str("\n");
                        last_event = str_builder;
                    }
                }

                read_events.remove(0);

                if read_events.len() == 0 { continue_while = false; }
                else {
                    //println!("Evento appena concluso: {:?}", e);
                    //println!("Prossimo evento: {:?}", read_events[0]);
                    if read_events[0] != *e { continue_while = false; }
                }
            }

            if game_info.event_vec.len() >= N_EVENT_IN_LOG { //è già pieno
                game_info.event_vec.remove(0);
            }
            game_info.event_vec.push(last_event);

            //println!("event_vec: {:?}", game_info.event_vec);

            //println!("Points: {:?}", read_points);

            //println!("-------------------------------------------------");
            //read_events.clear();
            //thread::sleep(Duration::from_millis(500));
        }
}
