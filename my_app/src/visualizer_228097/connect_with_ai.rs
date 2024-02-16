use std::collections::HashMap;
use std::sync::Mutex;
use bevy::ecs::system::ResMut;
use robotics_lib::event::events::Event;
use robotics_lib::world::tile::{Content, Tile};
use bevy::prelude::*;
use crate::visualizer_228097::components::{GameInfo};
use crate::visualizer_228097::events::*;
use crate::visualizer_228097::stats::components::N_EVENT_IN_LOG;
use crate::visualizer_227694::rudimental_a_i::RunnerTag;
use crate::events as EVENTS;
use crate::positions as POSITIONS;
use crate::energy as ENERGY;

pub fn update(
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
            let mut update_positions = POSITIONS.lock().unwrap();
            let mut update_energy = ENERGY.lock().unwrap();
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
                        str_builder.push_str(update_positions.0.to_string().as_str());
                        str_builder.push_str("][");
                        str_builder.push_str(update_positions.1.to_string().as_str());
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
                        if *update_energy + n > 1000 {
                            tmp_for_energy += 1000 - *update_energy;
                            *update_energy = 1000;
                        }
                        else {
                            tmp_for_energy += n;
                            *update_energy += n;
                        }

                        ew_energy_update.send( EnergyUpdated { total_energy: *update_energy } );

                        //println!("Recuperato {} energia, totale energia: {}", n, remaining_energy.energy);
                        continue_while = true;

                        let mut str_builder = String::from("Energy recharged (+");
                        str_builder.push_str(tmp_for_energy.to_string().as_str());
                        str_builder.push_str(") -> Remaining energy: ");
                        str_builder.push_str(update_energy.to_string().as_str());
                        str_builder.push_str("\n");
                        last_event = str_builder;
                    }
                    Event::EnergyConsumed(n) => {
                        if *update_energy >= *n {
                            tmp_for_energy += n;
                            *update_energy -= n;
                        }
                        else {
                            tmp_for_energy += tmp_for_energy;   //giusto :)
                            *update_energy = 0;
                        }

                        ew_energy_update.send( EnergyUpdated { total_energy: *update_energy } );

                        //println!("Consumata {} energia, totale energia: {}", n, remaining_energy.energy);
                        continue_while = true;

                        let mut str_builder = String::from("Energy consumed (-");
                        str_builder.push_str(tmp_for_energy.to_string().as_str());
                        str_builder.push_str(") -> Remaining energy: ");
                        str_builder.push_str(update_energy.to_string().as_str());
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

                            str_builder.push_str(update_positions.0.to_string().as_str());
                            str_builder.push_str("][");
                            str_builder.push_str(update_positions.1.to_string().as_str());
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
