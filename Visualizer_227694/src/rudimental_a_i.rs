use std::sync::Mutex;
use std::thread;
use bevy::prelude::*;
use lazy_static::lazy_static;
use crate::game_data::*;
use crate::GameUpdate;


lazy_static! {
    static ref aggiornamenti: Mutex<Aggiornamento> = Mutex::new(Aggiornamento{ next:false,eventi: vec![], points:0.0, world:vec![] });
}

struct Aggiornamento {
    next:bool,
    eventi: Vec<robotics_lib::event::events::Event>,
    points: f32,
    world: Vec<Vec<Option<robotics_lib::world::tile::Tile>>>,
}
pub struct ArtificialIntelligencePlugin;

impl Plugin for ArtificialIntelligencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_artificial_intelligence)
            .add_systems(Update, update_game_update.in_set(MySet::First));
    }
}

fn setup_artificial_intelligence(mut game_update: ResMut<GameUpdate>,
                      mut game_data: ResMut<GameData>,
){

    if game_data.ai{
        thread::spawn(|| {
            //AIDIGOLDO::funzionechesitaomodicendo();
            println!("la funzione della libreria AI di Goldo");
        });
    }else{
        thread::spawn(|| {
            println!("la funzione della libreria AI di MURRU");
        });
    }
    //TODO creo un nuovo thread in cui chiamo la funzione dell'intelligenza artificiale corrispondente
}
fn update_game_update(mut game_update: ResMut<GameUpdate>,
                      mut game_data: ResMut<GameData>,
){
    if game_data.next!=0{
        game_data.next -= 1;
        info!("next process_tick");
        let mut update = aggiornamenti.lock().unwrap();
        update.next = true;
    }else {
        let mut update = aggiornamenti.lock().unwrap();
        for i in update.eventi.iter(){
            game_update.events.push(i.clone());
        }
        game_update.world = update.world.clone();
        game_update.points = update.points.clone();
    }
}
