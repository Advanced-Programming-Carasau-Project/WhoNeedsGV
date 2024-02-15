use rocket::launch;
use rocket::get;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use robotics_lib::world::tile::Tile;
use who_needs_gv_world_generator::WorldGenerator;
use robotics_lib::world::world_generator::Generator;
use rip_worldgenerator::MyWorldGen;
#[derive(Serialize)]
struct World{
    size :usize,
    tiles: Vec<Vec<Tile>>,
}

#[get("/get_data")]
fn get_data() -> Json<World> {
    let size: usize = 128; //max: 1024 potenze di 2 per non avere problemi con la canvas

    let mut world_generator = MyWorldGen::new_param(size,2,5,0,true,false, 5, false, None);


    //world_generator.e_seed = 1688614679299147791;
    //world_generator.m_seed = 15581163141988959099;
    //world_generator.t_seed = 15151208778076085932;

    //world_generator.trees = false; //per generare mondi più velocemente se non ti interessano gli alberi
    //world_generator.teleports = false;

    let generato = world_generator.gen();

    let mondo = World{
        size: size,
        tiles: generato.0,
    };

    println!("[.............................. ] sending to JS");

    Json(mondo)
}
#[launch]
fn rocket()->_{
    rocket::build().mount("/", routes![get_data]).mount("/", rocket::fs::FileServer::from("static"))
}